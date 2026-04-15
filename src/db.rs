//! In-browser portfolio index backed by SQLite (WASM + `sqlite` feature) with a structured fallback.
//!
//! Memory VFS from `sqlite-wasm-rs` does **not** require COOP/COEP; OPFS / SharedArrayBuffer paths do.
#[cfg(all(target_arch = "wasm32", feature = "sqlite"))]
use crate::data::ProjectDetail;
use crate::data::{get_infrastructure_fleet, ProjectCategory, ProjectIndex};
use std::sync::atomic::{AtomicU64, Ordering};

static LAST_PORTFOLIO_QUERY_US: AtomicU64 = AtomicU64::new(0);
static LAST_INDEX_ENRICH_US: AtomicU64 = AtomicU64::new(0);
static LAST_SAMPLE_SEARCH_US: AtomicU64 = AtomicU64::new(0);

/// Wall-clock duration of the last portfolio search (microseconds), for telemetry.
pub fn last_portfolio_query_micros() -> u64 {
    LAST_PORTFOLIO_QUERY_US.load(Ordering::Relaxed)
}

/// Wall-clock duration of the last JSON→SQLite enrichment pass (microseconds).
pub fn last_index_enrich_micros() -> u64 {
    LAST_INDEX_ENRICH_US.load(Ordering::Relaxed)
}

/// Wall-clock duration of a fixed sample query after enrichment (microseconds).
pub fn last_sample_search_micros() -> u64 {
    LAST_SAMPLE_SEARCH_US.load(Ordering::Relaxed)
}

#[cfg_attr(not(all(target_arch = "wasm32", feature = "sqlite")), allow(dead_code))]
fn record_query_duration_us(start_ms: f64) {
    #[cfg(target_arch = "wasm32")]
    {
        let end = js_sys::Date::new_0().get_time();
        let us = ((end - start_ms).max(0.0) * 1000.0).round() as u64;
        LAST_PORTFOLIO_QUERY_US.store(us, Ordering::Relaxed);
    }
    let _ = start_ms;
}

fn fallback_match_score(p: &ProjectIndex, q_lower: &str) -> i32 {
    if q_lower.is_empty() {
        return 0;
    }
    let mut s = 0_i32;
    let tl = p.title.to_lowercase();
    let sub = p.subtitle.to_lowercase();
    let desc = p.description.to_lowercase();
    let sl = p.slug.to_lowercase();
    if tl.contains(q_lower) {
        s += 100;
    }
    if sl == q_lower {
        s += 130;
    } else if sl.contains(q_lower) {
        s += 75;
    }
    if sub.contains(q_lower) {
        s += 65;
    }
    if p.tech_stack
        .iter()
        .any(|t| t.to_lowercase().contains(q_lower))
    {
        s += 50;
    }
    if desc.contains(q_lower) {
        s += 40;
    }
    s
}

fn search_fallback(q_lower: &str, category: Option<ProjectCategory>) -> Vec<ProjectIndex> {
    let mut rows: Vec<ProjectIndex> = get_infrastructure_fleet()
        .iter()
        .filter(|p| {
            category.as_ref().map_or(true, |c| p.category == *c)
                && (q_lower.is_empty()
                    || p.title.to_lowercase().contains(q_lower)
                    || p.subtitle.to_lowercase().contains(q_lower)
                    || p.description.to_lowercase().contains(q_lower)
                    || p.slug.to_lowercase().contains(q_lower)
                    || p.tech_stack
                        .iter()
                        .any(|t| t.to_lowercase().contains(q_lower)))
        })
        .cloned()
        .collect();
    if !q_lower.is_empty() {
        rows.sort_by(|a, b| {
            let sa = fallback_match_score(a, q_lower);
            let sb = fallback_match_score(b, q_lower);
            sb.cmp(&sa).then_with(|| a.slug.cmp(b.slug))
        });
    }
    rows
}

#[cfg(all(target_arch = "wasm32", feature = "sqlite"))]
mod sqlite_engine {
    use super::*;
    use core::ffi::{c_char, CStr};
    use sqlite_wasm_rs::{
        sqlite3, sqlite3_bind_int, sqlite3_bind_text, sqlite3_close, sqlite3_column_text,
        sqlite3_exec, sqlite3_finalize, sqlite3_open_v2, sqlite3_prepare_v2, sqlite3_step,
        sqlite3_stmt, MemVfsUtil, WasmOsCallback, SQLITE_DONE, SQLITE_OK, SQLITE_OPEN_CREATE,
        SQLITE_OPEN_READWRITE, SQLITE_ROW,
    };
    use std::ffi::CString;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn category_id(c: crate::data::ProjectCategory) -> i32 {
        match c {
            crate::data::ProjectCategory::CyberSecurity => 0,
            crate::data::ProjectCategory::CloudInfrastructure => 1,
            crate::data::ProjectCategory::SystemsAdmin => 2,
            crate::data::ProjectCategory::Networking => 3,
        }
    }

    static DB_PTR: AtomicUsize = AtomicUsize::new(0);

    pub fn index_ready() -> bool {
        db_handle().is_some()
    }

    fn db_handle() -> Option<*mut sqlite3> {
        let p = DB_PTR.load(Ordering::SeqCst);
        if p == 0 {
            None
        } else {
            Some(p as *mut sqlite3)
        }
    }

    fn set_db_handle(db: *mut sqlite3) {
        DB_PTR.store(db as usize, Ordering::SeqCst);
    }

    unsafe fn run_ddl(db: *mut sqlite3, sql: &str) -> Result<(), String> {
        let c = CString::new(sql).map_err(|_| "invalid SQL CString".to_string())?;
        let rc = sqlite3_exec(
            db,
            c.as_ptr(),
            None,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        if rc == SQLITE_OK {
            Ok(())
        } else {
            Err(format!("sqlite3_exec failed: {rc}"))
        }
    }

    fn open_and_index() -> Result<(), String> {
        let _mem = MemVfsUtil::<WasmOsCallback>::new();
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let path = CString::new("portfolio_mem.db").map_err(|_| "path".to_string())?;
        let rc = unsafe {
            sqlite3_open_v2(
                path.as_ptr(),
                &mut db,
                SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
                core::ptr::null(),
            )
        };
        if rc != SQLITE_OK || db.is_null() {
            return Err(format!("sqlite3_open_v2: {rc}"));
        }

        unsafe {
            run_ddl(
                db,
                "CREATE TABLE IF NOT EXISTS projects (
                    slug TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    subtitle TEXT NOT NULL,
                    description TEXT NOT NULL,
                    category INTEGER NOT NULL,
                    tech_stack TEXT NOT NULL,
                    body_text TEXT NOT NULL DEFAULT ''
                );",
            )?;
            run_ddl(db, "DELETE FROM projects;")?;

            let insert_sql = CString::new(
                "INSERT INTO projects (slug, title, subtitle, description, category, tech_stack, body_text) VALUES (?,?,?,?,?,?,?);",
            )
            .map_err(|_| "insert sql".to_string())?;

            for p in get_infrastructure_fleet().iter() {
                let mut stmt: *mut sqlite3_stmt = core::ptr::null_mut();
                let tail: *mut *const c_char = core::ptr::null_mut();
                let pr = sqlite3_prepare_v2(db, insert_sql.as_ptr(), -1, &mut stmt, tail);
                if pr != SQLITE_OK || stmt.is_null() {
                    let _ = sqlite3_close(db);
                    return Err(format!("prepare insert: {pr}"));
                }
                let slug = CString::new(p.slug).map_err(|_| "slug nul")?;
                let title = CString::new(p.title).map_err(|_| "title nul")?;
                let subtitle = CString::new(p.subtitle).map_err(|_| "subtitle nul")?;
                let description = CString::new(p.description).map_err(|_| "desc nul")?;
                let tech = CString::new(p.tech_stack.join(",")).map_err(|_| "tech nul")?;
                let _ = sqlite3_bind_text(stmt, 1, slug.as_ptr(), -1, None);
                let _ = sqlite3_bind_text(stmt, 2, title.as_ptr(), -1, None);
                let _ = sqlite3_bind_text(stmt, 3, subtitle.as_ptr(), -1, None);
                let _ = sqlite3_bind_text(stmt, 4, description.as_ptr(), -1, None);
                let _ = sqlite3_bind_int(stmt, 5, category_id(p.category.clone()));
                let _ = sqlite3_bind_text(stmt, 6, tech.as_ptr(), -1, None);
                let empty = CString::new("").map_err(|_| "empty body".to_string())?;
                let _ = sqlite3_bind_text(stmt, 7, empty.as_ptr(), -1, None);
                let step = sqlite3_step(stmt);
                let _ = sqlite3_finalize(stmt);
                if step != SQLITE_DONE {
                    let _ = sqlite3_close(db);
                    return Err(format!("insert step: {step}"));
                }
            }
        }

        set_db_handle(db);
        Ok(())
    }

    pub fn ensure_sqlite() -> Result<(), String> {
        if db_handle().is_some() {
            return Ok(());
        }
        open_and_index()
    }

    pub fn search_sqlite(q: &str, category: Option<ProjectCategory>) -> Option<Vec<ProjectIndex>> {
        let start = js_sys::Date::new_0().get_time();
        let db = db_handle()?;
        let q_lower = q.trim().to_lowercase();
        let cat_bind = category.map(category_id).unwrap_or(-1);

        let sql = CString::new(
            "SELECT slug FROM projects WHERE \
             (length(?1) = 0 OR \
              lower(title) LIKE ('%' || ?1 || '%') OR \
              lower(subtitle) LIKE ('%' || ?1 || '%') OR \
              lower(description) LIKE ('%' || ?1 || '%') OR \
              lower(slug) LIKE ('%' || ?1 || '%') OR \
              lower(tech_stack) LIKE ('%' || ?1 || '%') OR \
              lower(body_text) LIKE ('%' || ?1 || '%')) \
             AND (?2 = -1 OR category = ?2) \
             ORDER BY \
             (CASE WHEN length(?1) = 0 THEN 0 ELSE \
               (CASE WHEN lower(title) LIKE ('%' || ?1 || '%') THEN 100 ELSE 0 END) + \
               (CASE WHEN lower(slug) = ?1 THEN 130 ELSE 0 END) + \
               (CASE WHEN length(?1) > 0 AND lower(slug) LIKE ('%' || ?1 || '%') AND lower(slug) <> ?1 THEN 75 ELSE 0 END) + \
               (CASE WHEN lower(subtitle) LIKE ('%' || ?1 || '%') THEN 65 ELSE 0 END) + \
               (CASE WHEN lower(tech_stack) LIKE ('%' || ?1 || '%') THEN 50 ELSE 0 END) + \
               (CASE WHEN lower(description) LIKE ('%' || ?1 || '%') THEN 40 ELSE 0 END) + \
               (CASE WHEN lower(body_text) LIKE ('%' || ?1 || '%') THEN 25 ELSE 0 END) \
             END) DESC, \
             lower(title) ASC;",
        )
        .ok()?;

        let mut stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let rc =
            unsafe { sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, core::ptr::null_mut()) };
        if rc != SQLITE_OK || stmt.is_null() {
            record_query_duration_us(start);
            return None;
        }

        let q_c = CString::new(q_lower.as_str()).ok()?;
        unsafe {
            let _ = sqlite3_bind_text(stmt, 1, q_c.as_ptr(), -1, None);
            let _ = sqlite3_bind_int(stmt, 2, cat_bind);
        }

        let mut slugs: Vec<String> = Vec::new();
        loop {
            let step = unsafe { sqlite3_step(stmt) };
            if step == SQLITE_ROW {
                let ptr = unsafe { sqlite3_column_text(stmt, 0) };
                if ptr.is_null() {
                    continue;
                }
                let s = unsafe { CStr::from_ptr(ptr.cast::<c_char>()) };
                if let Ok(u) = s.to_str() {
                    slugs.push(u.to_string());
                }
            } else if step == SQLITE_DONE {
                break;
            } else {
                let _ = unsafe { sqlite3_finalize(stmt) };
                record_query_duration_us(start);
                return None;
            }
        }
        let _ = unsafe { sqlite3_finalize(stmt) };

        let mut out = Vec::new();
        let fleet = get_infrastructure_fleet();
        for slug in slugs {
            if let Some(p) = fleet.iter().find(|x| x.slug == slug.as_str()) {
                out.push(p.clone());
            }
        }
        record_query_duration_us(start);
        Some(out)
    }

    pub fn update_body_text(slug: &str, body: &str) -> Result<(), String> {
        let Some(db) = db_handle() else {
            return Err("db not open".to_string());
        };
        let sql = CString::new("UPDATE projects SET body_text = ?1 WHERE slug = ?2;")
            .map_err(|_| "update sql".to_string())?;
        let mut stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let rc =
            unsafe { sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, core::ptr::null_mut()) };
        if rc != SQLITE_OK || stmt.is_null() {
            return Err(format!("prepare update: {rc}"));
        }
        let body_c = CString::new(body).map_err(|_| "body nul".to_string())?;
        let slug_c = CString::new(slug).map_err(|_| "slug nul".to_string())?;
        unsafe {
            let _ = sqlite3_bind_text(stmt, 1, body_c.as_ptr(), -1, None);
            let _ = sqlite3_bind_text(stmt, 2, slug_c.as_ptr(), -1, None);
            let step = sqlite3_step(stmt);
            let _ = sqlite3_finalize(stmt);
            if step != SQLITE_DONE {
                return Err(format!("update step: {step}"));
            }
        }
        Ok(())
    }
}

/// `true` when the in-memory SQLite handle is open (WASM + `sqlite` feature only).
pub fn sqlite_index_ready() -> bool {
    #[cfg(all(target_arch = "wasm32", feature = "sqlite"))]
    {
        sqlite_engine::index_ready()
    }
    #[cfg(not(all(target_arch = "wasm32", feature = "sqlite")))]
    {
        false
    }
}

pub fn init_portfolio_index() {
    #[cfg(all(target_arch = "wasm32", feature = "sqlite"))]
    {
        if let Err(e) = sqlite_engine::ensure_sqlite() {
            leptos::logging::warn!("portfolio SQLite init skipped: {e}");
        }
    }
}

/// Home-page search: all categories, backed by SQLite when available.
pub fn search_projects(query: &str) -> Vec<ProjectIndex> {
    search_portfolio_projects(query, None)
}

pub fn search_portfolio_projects(q: &str, category: Option<ProjectCategory>) -> Vec<ProjectIndex> {
    let q_lower = q.trim().to_lowercase();
    #[cfg(all(target_arch = "wasm32", feature = "sqlite"))]
    let result = {
        let _ = sqlite_engine::ensure_sqlite();
        if let Some(v) = sqlite_engine::search_sqlite(q, category.clone()) {
            v
        } else {
            let start = js_sys::Date::new_0().get_time();
            let v = search_fallback(&q_lower, category);
            record_query_duration_us(start);
            v
        }
    };
    #[cfg(not(all(target_arch = "wasm32", feature = "sqlite")))]
    let result = search_fallback(&q_lower, category);
    result
}

/// Fetch `/projects/{slug}.json` and merge long-form `content` into SQLite `body_text` for search.
#[cfg(all(target_arch = "wasm32", feature = "sqlite"))]
pub async fn enrich_index_from_static_json() {
    use gloo_net::http::Request;
    let enrich_start = js_sys::Date::new_0().get_time();
    for p in get_infrastructure_fleet().iter() {
        let url = format!("/projects/{}.json", p.slug);
        let Ok(resp) = Request::get(&url).send().await else {
            continue;
        };
        if !resp.ok() {
            continue;
        }
        let Ok(text) = resp.text().await else {
            continue;
        };
        let Ok(detail) = serde_json::from_str::<ProjectDetail>(&text) else {
            continue;
        };
        let _ = sqlite_engine::update_body_text(p.slug, &detail.content);
    }
    let enrich_end = js_sys::Date::new_0().get_time();
    let enrich_us = ((enrich_end - enrich_start).max(0.0) * 1000.0).round() as u64;
    LAST_INDEX_ENRICH_US.store(enrich_us, Ordering::Relaxed);

    let sample_start = js_sys::Date::new_0().get_time();
    let _ = search_portfolio_projects("terraform", None);
    let sample_end = js_sys::Date::new_0().get_time();
    let sample_us = ((sample_end - sample_start).max(0.0) * 1000.0).round() as u64;
    LAST_SAMPLE_SEARCH_US.store(sample_us, Ordering::Relaxed);
}

#[cfg(not(all(target_arch = "wasm32", feature = "sqlite")))]
#[allow(clippy::unused_async)]
pub async fn enrich_index_from_static_json() {}
