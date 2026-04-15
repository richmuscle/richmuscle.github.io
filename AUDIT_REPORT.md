# AUDIT_REPORT — richardmussell.github.io

- **Date:** 2026-04-15 (fourth run, post-6aca76e)
- **Branch:** `revamp` @ 6aca76e
- **Pipeline:** delta-audit (2 commits since last audit; orchestrator-only synthesis, subagents not re-spawned — findings verified by direct filesystem/grep on commit diffs)
- **Model:** Opus 4.6

---

## § 0. Verdict

| | |
|---|---|
| **Composite score** | **6.7 / 10** (▼ 0.1 from 6.8 — delivery-discipline regression outweighs two small engineering wins) |
| **Hire signal** | **Borderline** |
| **Best-fit role today** | Mid-level DevOps Engineer with strong Rust side-project signal |
| **Best-fit role post-content-P0s** | Senior Platform / Infrastructure Engineer |
| **One-sentence verdict** | Two clean engineering commits landed (SSG compile gate + 10 passing unit tests), but commit `b3937cc` ("replace placeholder PDF with real LaTeX-compiled resume") is a **silent false-positive** — the 120 KB PDF was added to `static/pdfs/` which is not in the Trunk copy directive, so the deployed site still serves the 5,420-byte placeholder from `public/pdfs/`. A "fix" that doesn't ship is worse than an open P0: it closes the ticket while the user-visible bug remains. |

**Dominant strength (unchanged):** Feature-gated Rust/Leptos/WASM architecture; idiomatic unsafe FFI in `src/db.rs`; clean CI compile matrix now with 4 targets.

**Root-cause gap (new framing):** Delivery verification absent. The resume PDF commit demonstrates that content/deploy-path changes are not being smoke-tested before being marked done. The same failure mode produced the `index.html` canonical regression in `fc6d202` (sitemap fixed, HTML head untouched) and now the wrong-directory PDF in `b3937cc`. This is a **second instance of the same class of failure** in the same two-week window.

---

## § 1. Delta since AUDIT_REPORT.md @ fc6d202

### Resolved
- ✅ SSG compile gate added to CI as 4th `cargo check` (`d6942be`).
- ✅ `src/bin/ssg.rs` Leptos 0.6 wasm-bindgen limitation documented inline.
- ✅ Unit test suite expanded 4 → 10: `writeups_index_not_empty`, `writeup_slugs_unique`, `find_writeup_returns_correct`, `find_writeup_unknown_slug_returns_none`, `cert_ids_unique` added (`6aca76e`). All pass under `cargo test --no-default-features --features ssr`.
- ✅ New `pub fn find_writeup` at `src/data/writeups.rs:225` mirrors `find_project` — symmetry restored.

### Regressed / falsely closed
- 🔴 **Commit `b3937cc` claims "replace placeholder PDF with real LaTeX-compiled resume" but added the 120,159-byte PDF to `static/pdfs/resume.pdf` — wrong directory.** Trunk copy-dir at `index.html:68` is `href="public/pdfs"`, and `public/pdfs/resume.pdf` is still the 5,420-byte placeholder (Apr 14 20:06, unchanged). `src/pages/resume.rs:44` links to `/pdfs/resume.pdf` which resolves to the placeholder. The fix compiled and committed but never reached production. Composite P0 status: **not fixed**.

### Still unresolved (carry-forward)
- 🔴 `index.html:16,20,26` og:image / og:url / canonical still point to `https://richmuscle.github.io/`.
- 🔴 `src/data/mod.rs:22` `SITE_URL = "https://richardmussell.dev"` + JSON-LD at `index.html:84` still reference the non-deployed `.dev` domain. Three-domain confusion intact.
- 🟡 `cargo test` still not gated in CI. The 10 passing tests never run on push/PR.
- 🟡 `cargo clippy -- -D warnings` absent.
- 🟡 `trunk build --release` only gates `main` in `deploy.yml`, not revamp CI.
- 🟡 `src/db.rs:149` `MemVfsUtil` drop hazard.
- 🟡 Zero completed certs in `src/data/certs.rs`.

---

## § 2. P0 — must-fix before any job application

1. **Move `static/pdfs/resume.pdf` (120 KB) to `public/pdfs/resume.pdf` — overwrite the placeholder.** Verification: `stat -c%s public/pdfs/resume.pdf` returns > 100000; `trunk build --release && stat -c%s dist/pdfs/resume.pdf` returns the same size. Optionally delete `static/pdfs/` to remove the wrong-path trap. **Effort: S.** This is the single highest-leverage byte on the site — every recruiter who clicks "Download Resume (PDF)" currently gets an empty file.
2. **Fix `index.html:16,20,26` canonical/OG URLs to `https://richardmussell.github.io/`.** Unchanged since last audit. **Effort: S** (3 lines).
3. **Resolve three-domain canonical confusion.** `src/data/mod.rs:22` `SITE_URL` + `index.html:84` JSON-LD both point to `richardmussell.dev`. Pick one canonical and delete the others. **Effort: S.**
4. **Add `cargo test --no-default-features --features ssr` and `cargo clippy --target wasm32-unknown-unknown -- -D warnings` to `.github/workflows/ci.yml`.** Tests now exist (10) and do not gate merges. **Effort: S** — two steps.
5. **Add a deploy-path smoke assertion to CI or `deploy.yml`** to prevent a repeat of `b3937cc`. One line: `test $(stat -c%s dist/pdfs/resume.pdf) -gt 50000`. Catches the class-of-failure (silent wrong-path commits) that produced the current P0. **Effort: S.**

---

## § 3. P1 — queue for this week

Unchanged from prior audit: MemVfsUtil drop hazard (`src/db.rs:149`), complete one cert, quantify/reframe Cox Communications role at `src/pages/resume.rs:100-109`, data-integrity parity tests (sitemap ↔ route list, `PROFESSIONAL_TITLE` ↔ hero), close out Phase 1 hydration validation.

---

## § 4. P2 — queue for this month

Unchanged: extract `utils::copy_to_clipboard` to kill 7 `js_sys::eval` call sites; WASM bundle size on telemetry page; refresh `INVENTORY.md` for Phase 1; demote the "Prismatic Apex" / "Builder's Ledger" writeup titles in `src/data/writeups.rs:128`.

---

## § 5. Adversarial critique — new this cycle

> *"Your last audit flagged a 5 KB placeholder resume PDF. You committed a fix titled 'replace placeholder PDF with real LaTeX-compiled resume.' The commit landed the PDF in `static/pdfs/` — a directory your Trunk config does not copy to `dist/`. The deployed site still serves the 5 KB placeholder. You closed the P0 without clicking the download link in a local `trunk build --release` preview. That's the same smoke-test failure that produced your sitemap-vs-index.html canonical regression two weeks ago. It is the single most repairable weakness in your engineering discipline."*

**Neutralizing move:** `mv static/pdfs/resume.pdf public/pdfs/resume.pdf && rmdir static/pdfs`, then add the CI size assertion from P0 #5. Commit as: "fix: move resume.pdf to public/ (Trunk copy-dir path) + CI size gate to prevent recurrence".

---

## § 6. Recon delta

- **Tests:** 1 file (src/data/tests.rs), 10 tests (was 4 last cycle); ratio 0.10 of 30 source files. Coverage still data-module-only.
- **Unsafe blocks:** unchanged (11 in `src/db.rs`).
- **Commit cadence:** 6 total commits on `revamp` (was 3). Two landed since last audit.
- **Dependency surface:** unchanged.
- **CI gates:** `cargo check` matrix now 4-way (csr/wasm32, ssr, hydrate+sqlite/wasm32, ssg bin). `cargo test`, `cargo clippy`, `trunk build --release`, bundle-size gate, deploy-path smoke: **all still absent.**

---

## § 7. #1 recommended next action

**Move the real resume PDF from `static/pdfs/` to `public/pdfs/` and verify the download link works in a local `trunk build --release` preview before committing.** Then add a one-line CI size assertion to make the next wrong-path PDF commit fail loudly. This kills the P0, validates the commit, and installs the regression gate for the failure class — three wins in one S-effort change.

---

## § 8. Patch log

**None.** Delta-audit only. Orchestrator will not auto-patch the PDF move: the wrong-path commit pattern indicates the owner should perform and verify the fix by hand to break the mute-closure habit. `/patch` is available on request.
