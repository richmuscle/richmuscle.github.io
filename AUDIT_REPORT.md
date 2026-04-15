# AUDIT_REPORT — richardmussell.github.io

- **Date:** 2026-04-14 (second run, post-ebee7ec)
- **Branch:** `revamp` @ ebee7ec (clean working tree)
- **Pipeline:** 3-agent orchestration (audit · recon · inline-patch)
- **Model:** Opus 4.6 (orchestrator) · Haiku (audit, recon) · Opus (inline patch after scope collapse)

---

## § 0. Verdict

| | |
|---|---|
| **Composite score** | **7.4 / 10** |
| **Hire signal** | **Borderline → Pass** on engineering; **No** until content P0s land |
| **Best-fit role as framed today** | Systems Administrator / DevOps Engineer with Rust depth |
| **Best-fit role after content P0s** | Senior Platform / Infrastructure Engineer (the stated goal) |
| **One-sentence verdict** | Technically legitimate — SSR/CSR gating, consolidated state, idiomatic Rust, clean CI — but the recruiter-visible surface (title drift, zero completed certs, 5 KB placeholder resume PDF, sitemap pointing at the staging domain) is still dragging the hire signal down. |

**Dominant strength:** Disciplined Rust/Leptos architecture. `hydrate`/`ssr`/`ssg`/`sqlite` feature gating is clean, `GlobalAppState` consolidated, 80+ browser API call sites gated with `#[cfg(not(feature = "ssr"))]`, conventional-commit history, three build targets (CSR / SSR / hydrate wasm32) all green in CI.

**Root-cause gap:** Content substance lags engineering polish. The site reads as a platform engineer's portfolio but the profile content reads as a junior sysadmin's profile. That mismatch is the bottleneck to the stated "land senior platform/infra roles" goal.

---

## § 1. Five-lens scores

| Lens | Score | Note |
|---|---|---|
| Architecture | 8 | Feature gating correct; GlobalAppState consolidation clean; SSG binary cleanly separates from lib via `ssg` feature |
| CS depth | 7 | Fuzzy search scoring + SQLite WASM scaffolding show systems thinking; no novel algorithms |
| Rust idiom | 9 | No `panic!` / `todo!` / `unimplemented!`; 16 `unwrap`/`expect` confined to DOM-access or unreachable-on-ssr paths; cfg gating idiomatic |
| Content / marketing | 4 | Identity crisis: Systems-Admin title + aspirational Principal-Architect copy; zero completed certs; sitemap wrong domain (fixed, see §6); placeholder 5.4 KB resume PDF |
| Engineering ops | 7 | CI gates three targets; deploy workflow sound; gaps: no live hydration validation, no WASM size budget, no pre-deploy checklist |

**Composite (weighted 25/25/20/15/15):** **7.4 / 10**

---

## § 2. P0 — must-fix before any job application

Tag legend: **[ENG]** patched this session · **[CONTENT]** flagged, owner to write.

| # | Tag | Action | Files | Status |
|---|---|---|---|---|
| 1 | ENG | Fix sitemap domain: replace `richmuscle.github.io` → `richardmussell.github.io` across all 27 `<loc>` entries | `public/sitemap.xml` | **Patched** (see §6) |
| 2 | ENG | Clean 7 SSR-build warnings (unused imports/vars, dead `push_log`) via `#[cfg(not(feature = "ssr"))]` gating | `src/pages/contact.rs`, `src/pages/one_pager.rs`, `src/components/nav.rs`, `src/pages/telemetry.rs` | **Patched** (see §6) |
| 3 | CONTENT | Replace placeholder 5.4 KB `public/pdfs/resume.pdf` with real, current résumé | `public/pdfs/resume.pdf` | **Owner** |
| 4 | CONTENT | Resolve title drift: either commit to "Systems Administrator & DevOps Engineer" everywhere and remove aspirational Principal-Architect framing, OR earn and document the senior framing with quantified wins | `src/data/mod.rs`, `src/pages/home.rs`, `src/pages/about.rs`, `src/pages/resume.rs`, `index.html` | **Owner** |
| 5 | CONTENT | Certifications section: either ship one completed credential or restructure to "Completed / Active Study" so all five entries aren't "In Progress" | `src/data/certs.rs` | **Owner** |

**Audit claims that did not survive verification** (flagged by audit-agent as P0, rejected):
- "Missing Trunk `copy-dir` for `static/docs`" → already present at `index.html:67`.
- "No resume PDF exists" → `public/pdfs/resume.pdf` exists. Separate concern: content quality of that PDF (5.4 KB, likely placeholder) — promoted to P0 #3.

---

## § 3. P1 — this week

| Tag | Action |
|---|---|
| CONTENT | Quantify work experience on `resume.rs:50-100` and `about.rs:23-29` — dates, team sizes, measurable impact (the "13 municipal entities" line needs company name + date span). |
| CONTENT | Ship one working demo per project, or remove the "demo" framing and call them "case studies" so "Coming Soon" text disappears. |
| ENG | Validate hydration end-to-end on deployed `revamp-origin` URL. No hydration-mismatch console errors, markers present, interactive elements wire. ROADMAP Phase 1 close-out. |
| ENG | Headless smoke test on ≥3 routes (home, resume, /project/terraform-gcp) as a CI job. |

## § 4. P2 — this month

| Tag | Action |
|---|---|
| ENG | Instrument real WASM gzip/brotli size on telemetry page; gate in CI (feeds ROADMAP Phase 2). |
| ENG | Extract duplicated clipboard-copy pattern (7 call sites) into `utils::copy_to_clipboard` helper. |
| ENG | Add `cargo test` pass: slug uniqueness, sitemap ↔ route parity, PROFESSIONAL_TITLE string consistency. |
| ENG | Integration test for SQLite-search → in-memory-search fallback path. |
| CONTENT | Tighten OG/meta descriptions toward concrete technical accomplishments ("ELK-based SOC, NIST 800-53") over generic phrasing. |

---

## § 5. Adversarial critiques

1. **"Senior/principal claims without quantified wins."** Hiring manager will ask why a Systems Administrator title appears next to Principal-Architect-tier writeup copy. *Neutralizing move:* pick one framing and commit; for the senior/staff framing, quantify the SOC work (scale, tools, findings) and ship one real outcome metric.

2. **"Every cert is aspirational."** Five "In Progress" certs reads as incomplete follow-through. *Neutralizing move:* finish one (Sec+, GCP ACE, or AZ-900 are 2–3 week commitments), or restructure so "Active Study" is a distinct section from "Completed."

3. **"Broken canonical URL + placeholder resume."** First impression = poor QA. *Neutralizing move:* sitemap fix shipped this session (see §6). Resume PDF replacement is P0 for the owner.

---

## § 6. Patch Log — this session

All changes are [ENG] only. [CONTENT] items listed in §2 are flagged, not touched.

### 6.1 `public/sitemap.xml` — canonical domain fix
- **Before:** 27 × `https://richmuscle.github.io/...` (staging repo domain)
- **After:** 27 × `https://richardmussell.github.io/...` (live site domain)
- **Method:** `sed -i 's|https://richmuscle\.github\.io|https://richardmussell.github.io|g'`
- **Verification:** `grep -c "richardmussell.github.io"` → 27; `grep -c "richmuscle.github.io"` (as non-substring) → 0.

### 6.2 SSR-build warning cleanup (7 → 0)
All four edits use `#[cfg(not(feature = "ssr"))]` to gate items only used on wasm32 paths, or `#[cfg(feature = "ssr")] let _ = x;` to consume a signal setter moved into non-SSR closures.

| File | Change |
|---|---|
| `src/pages/contact.rs:5` | Gate `use crate::utils::track;` with `cfg(not(feature = "ssr"))`. |
| `src/pages/contact.rs:9` | Add `#[cfg(feature = "ssr")] let _ = set_email_copied;` to consume the signal setter on SSR build. |
| `src/pages/one_pager.rs:9` | Same `let _ = set_email_copied;` shim. (Left `use crate::utils::track;` ungated — `track("print", ...)` at line 161 is called unconditionally inside an `on:click` closure and needs the import on both builds.) |
| `src/components/nav.rs:4` | Gate `use crate::utils::sanitize_slug;` with `cfg(not(feature = "ssr"))`. |
| `src/components/nav.rs:216-217` | Gate `use std::cell::RefCell;` and `use std::rc::Rc;` with `cfg(not(feature = "ssr"))`. |
| `src/pages/telemetry.rs:58` | Gate `fn push_log` with `cfg(not(feature = "ssr"))` — all 11 callers are inside non-SSR blocks. |

### 6.3 Verification
```
cargo check --no-default-features --features ssr               → 0 errors, 0 warnings
cargo check --features hydrate --target wasm32-unknown-unknown → 0 errors, 0 warnings
cargo check                   --target wasm32-unknown-unknown  → 0 errors, 0 warnings
```

---

## § 7. Recon appendix

- **Deps:** Leptos 0.6 pinned, `sqlite-wasm-rs 0.5.2`, no loose pins. WASM-only deps correctly `cfg(target_arch = "wasm32")` gated; `tokio` gated behind `ssg` feature so SSR check doesn't pull it.
- **Code smells:** 11 `unwrap` + 5 `expect`, all confined to DOM-access paths where `window()` / `document()` are infallibly present on wasm32, or unreachable-on-SSR. No `panic!`, `todo!`, `unimplemented!`, `dbg!`. No `TODO`/`FIXME` in source tree.
- **cfg gating:** 8 `cfg(feature = ...)` uses, 1 `cfg(target_arch = ...)` use. Idiomatic.
- **File sizes:** Largest Rust files all under 500 lines — healthy modularity.
- **CI:** `ci.yml` checks CSR + SSR + hydrate wasm32 on push/PR to `revamp`. `deploy.yml` deploys `main` → GitHub Pages.
- **Build status:** All three targets green after this session's patches.
- **Git hygiene:** Conventional-commit style, descriptive messages, two meaningful commits (`7b88343` initial release, `ebee7ec` Phase 1 pinnacle). Good cadence, no churn.
- **CSS:** 6,309 lines across modular design-tokens + components.
- **WASM artifact:** 1.5 MB `dist/*.wasm` — reasonable for CSR + SQLite + design system. Phase 2 code-splitting will measure and budget this.

---

## § 8. Next action

**#1 remaining action for the human owner:**
Replace `public/pdfs/resume.pdf` (currently a 5.4 KB placeholder) with the real, current résumé PDF, and in the same pass decide on one framing — "Systems Administrator & DevOps Engineer" vs senior/staff — and make it consistent across `src/data/mod.rs`, home/about/resume pages, and `index.html` OG tags.
