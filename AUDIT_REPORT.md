# AUDIT_REPORT — richardmussell.github.io

- **Date:** 2026-04-14
- **Branch:** `revamp` @ e9bdc7b (clean working tree)
- **Pipeline:** 3 subagents (audit · recon · patch-skipped-by-design)
- **Model:** Opus 4.6 (orchestrator) · Haiku (audit, recon)

---

## § 0. Verdict

| | |
|---|---|
| **Composite score** | **6.4 / 10** |
| **Hire signal** | **Borderline** (Strong Pass on engineering, No on recruiter framing) |
| **Best-fit role as framed today** | Mid-level DevOps / SysAdmin with a Rust side-interest |
| **Best-fit role after P0 content fixes** | Senior Platform / Infrastructure Engineer (the stated goal) |
| **One-sentence verdict** | Strong technical foundation undermined by deferred content work: the engineering signal is real, but every recruiter landing on the site sees zero completed certs, a mid-level title, four null demo URLs, and a 5 KB 1-page PDF resume — the gap is framing, not code. |

**Dominant strength:** Disciplined Rust/Leptos architecture — feature-gated `hydrate`/`ssr`/`ssg`/`sqlite` compile cleanly, consolidated `GlobalAppState`, 80+ browser API call sites gated with `#[cfg(not(feature = "ssr"))]`, clean conventional-commit git history, real tests on the data layer.

**Root cause gap:** ADR-005 is now the bottleneck. Phase 1 SSR is technically landed (SSR build passes), but the content layer flagged at kickoff — certs, title, demo URLs, resume PDF, quantified impact — is still in the exact state ADR-005 deferred it in. Engineering phases are compounding on top of an unresolved positioning problem.

---

## § 1. Five-Lens Scores

| Lens | Score | Notes |
|------|-------|-------|
| Architecture & design | 7 | Feature gating is textbook. `src/lib.rs` clean. 11 routes, well-scoped components. Module split of `data/` (projects, writeups, certs, mod) is a sensible refactor. |
| CS depth | 7 | Hydration/SSR/SSG separation understood at the compiler level. SQLite-WASM search is a genuine technical signal. Tokio correctly isolated to the SSG binary. |
| Rust/Leptos idioms | 7 | Signals/Memos/Effects used correctly. Error enum (`AppError`) in place. 9 `web_sys::window().unwrap()` calls in `layout.rs`/`nav.rs` are at DOM boundaries where failure is catastrophic anyway — acceptable. `Closure::forget()` ×3 for app-lifetime listeners is documented-intentional. |
| Hygiene | 7 | Git commits excellent. 5 unit tests on the data layer. SSR **and** hydrate builds pass (verified by orchestrator). No `todo!`/`panic!`/`dbg!` in source. **Missing:** `cargo check --features ssr` CI gate, `rust-toolchain.toml`, `cargo deny`/`cargo audit`. |
| **Recruiter-readiness** | **3** | **The crisis lens.** See § 3. |

---

## § 2. Agent Cross-Check & Ground-Truth Corrections

Both subagents produced useful output with correctable inaccuracies. The orchestrator verified the conflicts directly before writing this report.

| Claim | Source | Ground truth | Resolution |
|---|---|---|---|
| "109 SSR compile errors blocking Phase 1" | audit-agent | `cargo check --no-default-features --features ssr` → **0 errors, 7 warnings** | Audit-agent read `ROADMAP.md` without noticing commit `d593871` ("Session D — 109→0 SSR errors") already closed it. Audit-agent's P0 around SSR gating is stale. |
| "Hydrate build broken, 54 errors" | recon-agent | `cargo check --features hydrate --target wasm32-unknown-unknown` → **clean** | Recon-agent ran hydrate check on the host target; `web-sys`/`js-sys` are `cfg(target_arch = "wasm32")` gated, so host-target hydrate check will always fail. Not a real bug. |
| "WASM bundle 2.2 MB gzipped (4.5× over target)" | recon-agent | Not independently verified — no `dist/*.wasm` in tree at audit time | **Plausible; worth measuring.** `sqlite-wasm-rs` is in `default` features — it's the likely driver. Flagged P1. |
| "All 5 certs 'Studying'/'Planned'" | audit-agent | Verified `src/data/certs.rs:13-17` — **confirmed** | Carried into P0. |
| "Resume PDF missing" | recon-agent | `public/pdfs/resume.pdf` exists, **5,420 bytes, 1 page, PDF 1.4** | Corrected: it exists — but 5 KB / 1 page reads as a stub, not a senior resume. Reframed as P0 "resume freshness," not "resume missing." |
| "All 4 demo_url null" | audit-agent | Verified `static/projects/*.json` — **confirmed, all four null** | Carried into P0. |
| "No sitemap" | recon-agent | `public/sitemap.xml` exists, 27 URLs, **references 4 project slugs that match** `src/data/projects.rs` | Corrected: sitemap is present and consistent with current routes. Earlier roadmap claim of "broken sitemap referencing nonexistent project slugs" no longer applies. |

**Takeaway:** The codebase is in better engineering shape than `docs/ROADMAP.md` currently reflects — Session D closed the 109-error gating work; hydrate compiles; sitemap is correct. ROADMAP.md § Phase 1 "Outstanding" list is stale. The content-layer problems, however, are all real and unchanged since April 11.

---

## § 3. P0 — Must land before recruiter handoff

### P0-1. Ship at least one completed certification (or reframe the panel).
**Files:** `src/data/certs.rs:11-19`
**Why:** All five certs are `"Studying"` or `"Planned"`. For senior platform/infra roles, a credentials panel full of aspirational statuses is worse than no panel. Either (a) complete RHCSA or GCP-PCA and mark it `"Completed"` with issuer, date, credential URL, or (b) replace the "Certifications" panel with an explicit "Active Study Plan" panel so the framing is honest.
**Effort:** L if completing a cert, S if reframing.

### P0-2. Fix `PROFESSIONAL_TITLE` and meta/OG titles to target senior roles.
**Files:** `src/data/mod.rs:19`, `index.html` meta tags, per-page `<title>` drift.
**Why:** Current title "Systems Administrator & DevOps Engineer" is mid-level signal for a portfolio whose stated goal is senior platform/infrastructure hiring. Recruiters source by title; generic titles suppress outreach.
**Effort:** S.

### P0-3. Populate `demo_url` for ≥2 of 4 projects, or remove the demo route.
**Files:** `static/projects/*.json`, `src/pages/project.rs:232-308` (`ProjectDemoPage`), `src/lib.rs:115`.
**Why:** All four `demo_url` fields are `None`. Demo route exists but renders empty/placeholder. A visible-but-empty demo is a recruiter red flag. Either host 2 live demos (Terraform plan viewer, zero-trust network diagram playground) or drop `/project/:slug/demo` from the router until demos exist.
**Effort:** L (real demos) or S (remove route).

### P0-4. Replace the 5 KB 1-page resume PDF with a real senior-level resume.
**Files:** `public/pdfs/resume.pdf` (5,420 bytes, 1 page), `src/pages/resume.rs`.
**Why:** The artifact ships but reads as a stub. A senior platform-engineering resume is typically 2 pages with quantified outcomes. Also add an explicit `<a download>` link on `/resume` if the page currently only offers `window.print()` (verify in `resume.rs`).
**Effort:** M (rewrite) + S (wire download link).

### P0-5. Quantify work history with impact metrics.
**Files:** `src/pages/about.rs`, `src/data/projects.rs` (description strings), `src/pages/resume.rs`.
**Why:** No numbers. "Implemented", "managed", "engineered" without "reduced X by N%", "Y nodes", "Z runbooks". Platform/infra hiring filters on concrete outcomes. This is the single highest-leverage edit for the "land roles" goal.
**Effort:** M.

---

## § 4. P1 — This week

1. **Add `cargo check --no-default-features --features ssr` CI gate** in `.github/workflows/deploy.yml`. Phase 1 gating work is done; lock it in before regression.
2. **Measure and surface real WASM bundle size** on `/telemetry`. Recon-agent suspects ~2.2 MB gzipped vs. 500 KB target, driven by `sqlite-wasm-rs` in default features. Either lazy-load SQLite, move `sqlite` out of default features, or publish the measured size as an honest metric.
3. **Update `docs/ROADMAP.md` § Phase 1** to reflect reality: SSR builds clean, 109→0 is done, hydrate compiles. Don't carry stale "outstanding" items.
4. **Update `INVENTORY.md` § 7** (Browser API Surface) to note gates are now in place (67 `#[cfg(...)]`, 44 `cfg(not(feature = "ssr"))`).
5. **Pin Rust toolchain** via `rust-toolchain.toml`.

## § 5. P2 — This month

1. `cargo deny` + `cargo audit` in CI (Phase 9 prep).
2. Lighthouse CI gate + WASM size budget gate (Phase 7).
3. Headless smoke test on 3+ routes after SSG build (Phase 1 verification tail).
4. Cloudflare Pages migration (Phase 6) — unblocks Brotli, real headers, preview URLs per PR.
5. Route-level WASM code-splitting (Phase 2).

---

## § 6. Adversarial critiques

| Critique | Neutralizing move |
|---|---|
| "You've shipped 20 engineering commits on `revamp` but the live site on `main` still has the same content problems the audit flagged in April." | True. Phase 1 landed without moving the recruiter needle. Ship P0-1 through P0-5 as a single content PR **before** starting Phase 2, and cherry-pick that content PR onto `main` so the live site reflects the work. |
| "Your portfolio claims senior platform/infra but shows 0 completed certs, a 1-page 5 KB resume, and four null demo URLs. Why would a recruiter trust the deeper claims?" | They wouldn't. This is the framing gap. P0-1/2/4/5 close it directly — no code changes needed for most of it, just honest content. |
| "You spent a session wiring SQLite WASM search before validating anyone will see past the cert panel." | SQLite-WASM is a real differentiator and keeps. But P0 content work must ship in parallel — engineering flash does not compensate for a stub resume. |
| "`revamp-origin` is on a staging account. Nothing you're building is visible at `richardmussell.github.io` until you merge to `main`. How long is the staging runway?" | ADR-002 is a deliberate trade-off. Set a merge-back date: after P0 lands, merge `revamp` → `main` as one coherent release. Don't let staging drift indefinitely. |

---

## § 7. Patch log

**Patch agent was not run.** Per the plan confirmed at start-of-session: P0 actions require explicit owner approval before any writes, because several P0 items (completing a cert, rewriting the resume, building real demos) are content decisions the owner must make — they cannot be safely automated.

Items a patch agent *could* safely execute after approval:
- **P0-2** `PROFESSIONAL_TITLE` edit (owner picks the new title)
- **P0-3** remove `/project/:slug/demo` route from `src/lib.rs` (if owner prefers removal over building demos)
- **P1-1** add SSR CI gate (mechanical)
- **P1-3 / P1-4** doc updates (mechanical)

Items that **must not** be patch-agent work:
- **P0-1** cert completion, **P0-4** resume rewrite, **P0-5** quantification — require the owner's real achievements.

Say the word and I'll run the patch agent on the mechanical subset only.

---

## § 8. Appendix A — Recon snapshot (selected)

- **Rust source files:** 29
- **Test attributes:** 6 (`#[test]` ×5 in `data/tests.rs`, 1 `cfg(test)`)
- **Test ratio:** 0.21
- **`#[cfg(...)]` gates:** 67 total · 44 `cfg(not(feature = "ssr"))` · 8 `cfg(feature = "ssr")`
- **Browser API refs:** 59 across `web_sys::`/`js_sys::`
- **Largest files:** `src/db.rs` 422 · `src/pages/home.rs` 407 · `src/pages/project.rs` 359 · `src/components/nav.rs` 343 · `src/pages/telemetry.rs` 325 · `src/utils.rs` 319 · `src/pages/writing.rs` 315
- **`.unwrap()` hotspots:** 9 in `src/components/layout.rs` / `src/components/nav.rs` — all at DOM boundaries, acceptable
- **Compile status:** SSR ✅ (7 warnings) · hydrate-wasm32 ✅
- **Bundle measurement:** not run in this audit (no `dist/*.wasm` on disk at audit time) — flagged P1

## § 9. Appendix B — Content-layer snapshot

- **Certifications (`src/data/certs.rs`):** 5 entries, **0 completed** (3× Studying, 2× Planned)
- **Resume PDF:** `public/pdfs/resume.pdf`, **5,420 bytes, 1 page, PDF 1.4** (reads as stub)
- **Essay PDFs (strong signal):** 5 PDFs in `public/pdfs/` — `builders-ledger`, `orchestrated-landscape`, `platform-architecture-blueprint`, `sustainable-architect`, `universal-dialects`
- **Writeup JSONs:** 4 in `static/docs/` — linux-admin-scripting, monitoring-observability, terraform-gcp, zero-trust-networking
- **Project detail JSONs:** 4 in `static/projects/` — **all 4 with `demo_url: null`**
- **Sitemap:** `public/sitemap.xml`, 27 URLs, consistent with `src/lib.rs` route table — earlier "broken sitemap" claim no longer true
- **Routes:** 11, including `/project/:slug/demo` which currently renders against null demo URLs

## § 10. Appendix C — Dependency flags

| Crate | Version | Risk | Note |
|---|---|---|---|
| `sqlite-wasm-rs` | 0.5.2 | **High (bundle size)** | In `default` features. Suspected primary driver of WASM bloat. Consider moving out of default or lazy-loading. |
| `leptos` | 0.6 | Medium | Framework choice, inevitable. |
| `web-sys` | 0.3 | Low | Carefully feature-scoped in `Cargo.toml`. |
| `gloo-net`, `gloo-timers` | 0.5 / 0.3 | Low | Standard WASM utility crates. |
| `tokio` | 1 | Low | Gated behind `ssg` feature, host-only, does not ship to WASM. |
| `getrandom` | 0.2 (`js`) | Low | Required for WASM entropy. |

---
*End of report. No files were committed. Report lives at `AUDIT_REPORT.md` (untracked).*
