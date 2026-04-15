# AUDIT REPORT — 2026-04-15

Orchestrated agentic audit. Two parallel read-only subagents (audit + recon). Patch-agent held pending human approval per portfolio ADR-005 (content before new features) and session-scope authorization.

---

## § Verdict

| Dimension | Value |
|---|---|
| **Composite score** | **7.1 / 10** (+0.4 vs. prior audit) |
| **Hire signal** | **Borderline** |
| **Best-fit role** | Mid-level DevOps / Infrastructure Engineer with strong Rust side-project signal. Upgrades to Senior Platform/Infra once content P0s land. |
| **Dominant strength** | Feature-gated Rust/Leptos/WASM architecture with justified raw FFI (`src/db.rs`: 11 `unsafe` blocks wrapping sqlite3 C API, AtomicU64 perf counters, pure-Rust fallback). Pinned toolchain, 4-way `cargo check` CI matrix, `justfile` encoding the full pre-deploy pipeline. |
| **Root-cause gap** | Delivery verification. Six engineering commits since prior audit landed real improvements, but every content P0 flagged in prior cycles is unchanged — resume PDF pipeline, canonical URL triple-domain confusion, no `cargo test` / `cargo clippy` CI gate, unquantified recent roles, zero completed certs. Pattern: fixing the pipeline around broken content rather than fixing the content. |

**One-sentence verdict:** Engineering hygiene improved materially (+0.4) but the hiring-outcome bottleneck is still content, and the resume-PDF pipeline has now been "fixed" twice without confirming the live artifact — that is the tell.

---

## § Scores (five-lens, 1–10)

| Lens | Score | Notes |
|---|---|---|
| Architecture & code organization | 8 | Clean module split, `GlobalAppState` single context (portfolio ADR-003), layered SCSS (ADR-002), feature gating (ADR-007). |
| CS depth signals | 7 | Real `unsafe` FFI in `src/db.rs` with a correctness hazard (see P1 drop-order). Telemetry page w/ LCP observer. |
| Rust idioms & type discipline | 7 | `thiserror` `AppError` with per-section `ErrorBoundary` (ADR-006). 16 `unwrap`/`expect` hits, mostly web-sys-safe or in tests. Zero `panic!`/`todo!`/`unimplemented!`. |
| Deploy & build rigor | 6 | 4-way `cargo check` matrix in CI. **No `cargo test`, no `clippy -D warnings`, no `cargo audit`, no `cargo deny` in any workflow.** Bundle size unoptimized (`data-wasm-opt='0'`). |
| Content/marketing fit | 4 | Resume PDF pipeline fragile; OG/canonical URLs disagree with `SITE_URL`; About page still says "CSR" post-SSR migration; current job line on site is "Product Brand Ambassador"; no completed certs. |

---

## § P0 — Block recruiter outcomes or prod. Fix this week.

1. **Verify the live resume PDF, then harden the pipeline** (`public/pdfs/resume.pdf`, `static/pdfs/resume.pdf`, `.github/workflows/deploy.yml`, ~0.5 h)
   - **Unresolved verification:** audit-agent claims `public/pdfs/resume.pdf` is still a 5,420-byte placeholder and the live site serves it. Recon confirms local `dist/pdfs/resume.pdf` is 5,427 bytes *but* `deploy.yml` runs `cp static/pdfs/*.pdf dist/pdfs/` after `trunk build --release`, which should overwrite with the real 120,832-byte file. **Before touching anything, run `curl -sI https://richmuscle.github.io/pdfs/resume.pdf` and `curl -s https://richmuscle.github.io/pdfs/resume.pdf | wc -c`.** If the live file is >100k, the P0 is actually "eliminate the ambiguity so this stops recurring." If <10k, the CI step is not running as expected.
   - **Root-cause fix regardless:** pick one source directory. `public/pdfs/` (Trunk-copied automatically, no CI step needed) or `static/pdfs/` (requires the cp workaround). Delete the other. The duplicate-source pattern has now produced two commits in two weeks (b3937cc, efb6d00) and the audit-agent's uncertainty about live state is itself a signal that the pipeline is too indirect.
   - **Verification gate:** add `test $(stat -c%s dist/pdfs/resume.pdf) -gt 100000 || exit 1` to `deploy.yml` after the cp step.

2. **Unify canonical/OG/SITE_URL to one domain** (`index.html`, `src/data/mod.rs`, ~0.5 h)
   - `index.html:16` (`og:image`), `:20` (`og:url`), `:26` (canonical) → `https://richmuscle.github.io/`
   - `src/data/mod.rs:22` → `SITE_URL = "https://richardmussell.dev"`
   - JSON-LD references yet a third domain.
   - Three conflicting canonicals = duplicate-content signal to crawlers + broken OG previews. Pick one. The live URL per `CONTEXT.md` is `https://richmuscle.github.io/` — use that everywhere unless `richardmussell.dev` is actually provisioned (it isn't, per ADR context).

3. **Add `cargo test` + `cargo clippy` gates to `.github/workflows/ci.yml`** (~0.5 h)
   - 10 unit tests pass locally via `just test`; none run on merge. A slug-mismatch between `PROJECTS` and `static/projects/*.json` would land silently.
   - `clippy --target wasm32-unknown-unknown -- -D warnings` is in `justfile` but not CI.
   - Two new steps, each one line.

4. **Reframe or trim the recent non-engineering roles on the resume page** (`src/pages/resume.rs:100-120`, ~2 h — PROMOTED TO P0 from P1)
   - The audit-agent placed this in P1. **I'm escalating it.** The portfolio's stated goal is landing senior platform/infra roles at $180k+. The first thing a recruiter sees on the live resume is a "Product Brand Ambassador at Club Demonstration Services (Sept 2025–Present)" entry. That is a full-stop for that recruiter, regardless of the Rust engineering beneath it. No P1 hygiene fix outranks this.
   - Options: (a) omit from the on-site resume and disclose only in the PDF; (b) reframe as self-directed platform engineering with income flexibility + link to homelab. Quantify the Cox role (N customers, Y% retention, ICOMS/Salesforce toolchain) so it reads as enterprise toolchain exposure.

---

## § P1 — This week. After P0s.

1. **Fix `MemVfsUtil` drop hazard** (`src/db.rs:155`, ~2 h) — `open_and_index()` creates `_mem = MemVfsUtil::new()` on the stack, stores the raw `db` pointer in `DB_PTR`, then returns. If `MemVfsUtil` owns the VFS registration, dropping it may tear down the VFS while `DB_PTR` is still live → use-after-free on subsequent queries. Store `_mem` in a `OnceLock` or `mem::forget` it intentionally with a safety comment explaining the leak is load-bearing.
2. **Update `src/pages/about.rs:27`** — still says "CSR application" despite Phase 1 SSR+hydrate migration. A recruiter who reads that and then opens `Cargo.toml` concludes the owner doesn't understand their own stack.
3. **Refresh `INVENTORY.md` section 15** — font table still lists Inter even though the repo migrated to Geist (`static/fonts/geist-*.woff2`, `tokens.css` updated). Stale inventory misleads future sessions.
4. **Close Phase 1 smoke-test items in `ROADMAP.md`** — hydration console-error check and headless-browser smoke test were deferred; a silent hydration mismatch is exactly the regression class that escapes 4 `cargo check` gates.
5. **Extract `navigator.clipboard.writeText` to `utils::copy_to_clipboard`** — identical `js_sys::eval` pattern repeated at 7 sites (`home.rs:474`, `about.rs:119`, `resume.rs:31,216`, `one_pager.rs:25`, `contact.rs:36`, `components/project.rs:127`).

---

## § P2 — This month.

1. Add at least one completed certification to `src/data/certs.rs` — or remove the Professional Development section entirely. Two "In Progress" + one 7-year-old coursework entry broadcasts the gap.
2. Surface WASM bundle size on the telemetry page and add a CI size budget (e.g., fail at >2 MB gz). `data-wasm-opt='0'` means the 1.5 MB bundle is unoptimized; fixing this is Phase 2 work but the measurement is free now.
3. Add `cargo audit` + `cargo deny check` to CI. `deny.toml` exists but is not enforced. Resolve the `gloo-net 0.5/0.6` and `getrandom 0.2/0.4` version splits noted in `INVENTORY.md §17`.
4. Demote aspirational/speculative writeups (Prismatic Apex, Builder's Ledger, Orchestrated Landscape) below the technical ones (`automating-nist-800-53`, `zero-trust-moving-beyond-bastion-hosts`, `siem-alert-hygiene`) in `src/data/writeups.rs` and `public/sitemap.xml`. A platform-hiring-manager's first impression of the writing index determines whether they read the technical pieces at all.

---

## § Adversarial critiques

1. **Symptom-patching in CI.** Five audit cycles in, resume PDF P0 persists. Commit `b3937cc` "replaced placeholder PDF." Commit `efb6d00` then added a CI `cp` workaround to copy from `static/pdfs/` (where the real file is) to `dist/pdfs/` (where Trunk copied the stub from `public/pdfs/`). The fix should have been to move the real PDF to `public/pdfs/` once. A senior platform engineer who patches symptoms in CI instead of collapsing the source directories is not demonstrating the discipline being claimed.
   *Neutralizer:* delete either `public/pdfs/` or `static/pdfs/`, keep the one Trunk handles natively, verify with `curl` against the live URL, add the `stat -c%s > 100000` gate.
2. **Current-role optics.** "Product Brand Ambassador (Sept 2025–Present)" above the fold of a site marketed to senior platform recruiters is a near-instant disqualifier regardless of the Rust depth beneath it.
   *Neutralizer:* reframe or omit; see P0 §4.
3. **About page vs. `Cargo.toml` mismatch.** `about.rs:27` still says CSR; `Cargo.toml` has `hydrate` and `ssr` features. A recruiter who notices this concludes the owner can't describe their own architecture accurately.
4. **Tests that never run.** "10 unit tests" is a hollow signal if the merge path can't fail on them.
5. **Verification ambiguity on the #1 P0.** The fact that two read-only agents can't confidently tell you whether the live resume PDF is correct *is itself the defect*. The pipeline is too indirect for a deployed-artifact question to be answerable from the repo state.

---

## § Delta vs. prior `AUDIT_REPORT.md` (commit 6aca76e)

**Resolved since prior audit (6 commits):**
- `rust-toolchain.toml` pinned (`4363e31`)
- `justfile` with `check/test/build/deploy/lint` recipes (`8d23fe1`)
- LCP metric on telemetry page with threshold indicators (`1fcf677`)
- Font migration Inter → Geist (+ CI font verification updated)
- `SECURITY.md` added (`f2e05ca`) — and corrected this session for the `richardmussell.github.io` → `richmuscle.github.io` URL
- `static/pdfs/` → `dist/pdfs/` CI copy step (`efb6d00`) — though this is the symptom-patch flagged above

**Unresolved (carries forward):**
- Resume PDF pipeline ambiguity (verification gap)
- Triple-domain canonical/OG/SITE_URL confusion
- No `cargo test` / `cargo clippy` / `cargo audit` in CI
- `MemVfsUtil` drop hazard (`src/db.rs:155`)
- Zero completed certs
- Recent-roles unquantified and unreframed
- `about.rs:27` CSR self-description post-SSR migration
- `INVENTORY.md` stale (fonts, stack description)

**Net:** engineering hygiene +0.4 composite; every content P0 unchanged.

---

## § Appendix A — Dependencies

Direct deps (pinned where specified, caret-range otherwise):

| Package | Version | Notes |
|---|---|---|
| leptos | 0.6 | 0.7 exists; 0.6 stable-enough |
| leptos_meta, leptos_router | 0.6 | matched |
| serde, serde_json | 1 | unpinned minor/patch |
| thiserror | 1 | unpinned |
| web-sys | 0.3 | features: Clipboard, IntersectionObserver, MediaQueryList, etc. |
| js-sys, gloo-net (0.5), gloo-timers (0.3), getrandom (0.2) | — | unpinned; `gloo-net 0.5/0.6` + `getrandom 0.2/0.4` dual-major splits flagged in `INVENTORY.md §17` |
| sqlite-wasm-rs | 0.5.2 | optional, gate: `sqlite` |
| tokio | 1 | optional, gate: `ssg`, host-only |

Feature flags: `default = ["csr","sqlite"]` · `hydrate` · `ssr` · `ssg` (superset of ssr + tokio).

---

## § Appendix B — Code size & smells

**Largest Rust sources (top 10):** `src/utils.rs` 987 · `src/pages/telemetry.rs` 513 · `src/pages/home.rs` 428 · `src/db.rs` 414 · `src/pages/project.rs` 397 · `src/components/nav.rs` 380 · `src/pages/writing.rs` 322 · `src/components/palette.rs` 241 · `src/components/terminal.rs` 232 · `src/pages/resume.rs` 230.

**Largest styles (top 5):** `style/base.css` 1260 · `style/pages/project.css` 731 · `style/components/layout.css` 726 · `style/components/cards.css` 505 · `style/pages/writing.css` 470.

**Smells:** `TODO|FIXME|HACK|XXX` = 0. `unwrap|expect` = 16 (mostly web-sys-safe or tests). `panic!|todo!|unimplemented!` = 0. `unsafe` = 13 (all in `src/db.rs` SQLite FFI — justified, but see P1 drop-order hazard).

**Test ratio:** 1 `#[cfg(test)]` module (`src/data/tests.rs`, 10 assertions) over 29 src files = ~3.4%. No `tests/` directory.

**Build artifacts:** `dist/*.wasm` 1.5 MB · `dist/*.js` 62 KB · `dist/style-*.css` 144 KB. `wasm-opt` disabled (`data-wasm-opt='0'`).

---

## § Appendix C — CI surface

`ci.yml`: push/PR on `revamp`+`main`. 4 gates — `cargo check` × (CSR wasm32, SSR host, hydrate+sqlite wasm32, ssg bin). **No** `cargo test`, `clippy`, `audit`, `deny`.
`deploy.yml`: push to `main`. `trunk build --release` → cp fonts → cp index→404 → `.nojekyll` → cp `static/pdfs/*.pdf` → Pages deploy. No size assertions on output artifacts.

---

## § Appendix D — Env surface

All `std::env::var` calls are tool config for the SSG binary (`SSG_OUT`, `SSG_FRAGMENTS_ONLY`, `SSG_WRITE_FRAGMENTS`). No hardcoded secrets, API keys, or analytics IDs detected.

---

## § Patch log

**No patches applied this run.** Patch-agent deliberately not spawned:
1. Portfolio ADR-005 — content before new engineering features. The top P0s are content decisions (which directory hosts the PDF, which domain is canonical, how to reframe recent roles) and require human judgment, not code generation.
2. No session-scope authorization for writes.
3. P0 §1 has a *verification* gap before any fix is safe.

**Recommended sequence when authorized:**
- Human: run the `curl` checks on the live site to resolve P0 §1's verification gap.
- Human: decide canonical domain + how to reframe the recent-roles entry.
- Then spawn patch-agent with a narrowed brief covering P0 §§2–3 (URL unification + CI gates) which are mechanical.

---

## § Next action

**`curl -sI https://richmuscle.github.io/pdfs/resume.pdf && curl -s https://richmuscle.github.io/pdfs/resume.pdf | wc -c`** — the entire P0 pipeline question collapses to this one check.
