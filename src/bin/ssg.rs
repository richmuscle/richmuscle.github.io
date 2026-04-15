//! Static site export: pre-render route shells with `leptos::ssr::render_to_string`.
//!
//! Run (native):
//! `SSG_OUT=target/ssg-html cargo run --features ssg --bin ssg`
//!
//! Merge output with Trunk’s `dist/` (WASM/ CSS) for a full hydrate-capable deploy, or serve HTML for crawlers / no-JS baseline.
//! COOP/COEP + SharedArrayBuffer workflows are documented in `public/_headers`.

// KNOWN LIMITATION (Leptos 0.6): `leptos_reactive` pulls `wasm-bindgen`
// unconditionally. Running this binary on native panics at runtime with
// "cannot access imported statics on non-wasm targets". Compilation passes.
// Tracked: upgrade path is Leptos 0.7+ which decouples the SSR/WASM boundary.
// Validation gate: `cargo check --features ssg --bin ssg` (CI step).

use leptos::ssr::render_to_string;
use leptos::{create_runtime, provide_context, view};
use leptos_router::{generate_route_list_inner, RouterIntegrationContext, ServerIntegration};
use richardmussell::data::{all_writeups, get_infrastructure_fleet};
use richardmussell::App;
use std::fs;
use std::path::Path;

fn collect_paths() -> Vec<String> {
    let mut paths = vec![
        "/".to_string(),
        "/about".to_string(),
        "/writing".to_string(),
        "/resume".to_string(),
        "/contact".to_string(),
        "/telemetry".to_string(),
        "/one-pager".to_string(),
    ];
    for p in get_infrastructure_fleet() {
        paths.push(format!("/project/{}", p.slug));
        paths.push(format!("/project/{}/docs", p.slug));
        paths.push(format!("/project/{}/demo", p.slug));
    }
    for w in all_writeups() {
        paths.push(format!("/writing/{}", w.slug));
    }
    paths.push("/this-route-should-404".to_string());
    paths
}

fn render_path(path: &str) -> String {
    let url = format!("http://ssg.local{path}");
    render_to_string(|| {
        provide_context(RouterIntegrationContext::new(ServerIntegration {
            path: url,
        }));
        view! { <App/> }
    })
    .to_string()
}

fn wrap_document(body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>Richard Mussell — static shell</title>
</head>
<body>
{body}
<noscript><p>Static snapshot. Enable JavaScript for the full WASM experience.</p></noscript>
</body>
</html>
"#
    )
}

fn out_paths(out: &str, route: &str) -> (String, String) {
    let trimmed = route.trim_start_matches('/');
    if trimmed.is_empty() {
        (
            format!("{out}/index.html"),
            format!("{out}/index.body.html"),
        )
    } else {
        (
            format!("{out}/{trimmed}/index.html"),
            format!("{out}/{trimmed}/index.body.html"),
        )
    }
}

fn write_page(out: &str, route: &str, body_fragment: &str, write_full: bool, write_fragment: bool) {
    let (full_path, frag_path) = out_paths(out, route);
    if let Some(parent) = Path::new(&full_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if write_full {
        let doc = wrap_document(body_fragment);
        let _ = fs::write(&full_path, doc);
    }
    if write_fragment {
        let _ = fs::write(frag_path, body_fragment);
    }
}

fn main() {
    let (route_list, _) = generate_route_list_inner(|| view! { <App/> });
    eprintln!(
        "ssg: leptos_router discovered {} static route listing(s) (manual path list still used for export)",
        route_list.len()
    );

    // `generate_route_list_inner` creates and disposes its own runtime; create a fresh one for export.
    let _export_rt = create_runtime();

    let out = std::env::var("SSG_OUT").unwrap_or_else(|_| "target/ssg-html".to_string());
    let fragments_only = std::env::var("SSG_FRAGMENTS_ONLY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let write_full = !fragments_only;
    let write_fragment = fragments_only
        || std::env::var("SSG_WRITE_FRAGMENTS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);

    let paths = collect_paths();
    // `create_resource` (project / writing pages) uses `spawn_local`; host SSG must run inside a
    // Tokio `LocalSet` with a Leptos `Runtime` scope (see leptos_reactive tests).
    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime for SSG");
    let local = tokio::task::LocalSet::new();
    local.block_on(&tokio_runtime, async {
        tokio::task::spawn_local(async move {
            let _ = fs::create_dir_all(&out);
            for path in paths {
                let body = render_path(&path);
                write_page(&out, &path, &body, write_full, write_fragment);
                println!("wrote {path}");
            }
            // Hard exit: Tokio / Leptos teardown can race with in-flight `create_resource` HTTP completions
            // and panic with `RuntimeDisposed`; this process has no further work.
            std::process::exit(0);
        })
        .await
        .expect("ssg export task");
    });
}
