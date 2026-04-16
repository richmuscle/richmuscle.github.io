# AUDIT REPORT — 2026-04-16 (three-agent parallel run)

Three-agent parallel swarm (`audit-agent` + `recon-agent` + `security-agent`) running on Opus 4.6 Fast mode. Read-only. Portfolio-scoped per `~/.claude/projects/portfolio/agents/*.md` and `~/.claude/projects/portfolio/DECISIONS.md`. Supersedes `AUDIT_REPORT.md` dated 2026-04-15 (second pass / 16-agent swarm).

---

## § Verdict

| Dimension | Value |
|---|---|
| **Composite score** | **7.6 / 10** (+0.8 vs 2026-04-15 / 6.8) — recomputed from lens weights (20/15/20/15/30); audit-agent's self-reported 7.8 does not match its own weights, corrected here |
| **Hire signal** | **Associate Platform / FDE I pass; senior platform still a deferred conversation.** Recruiter-persona battery from 2026-04-15 remains unsuperseded — today's three-agent run surfaces engineering regressions, not a narrative shift |
| **Dominant strength** | Engineering substrate held or improved across 4 of 5 lenses: SQLite FFI (12 `unsafe`, all `src/db.rs`, all ADR-004-justified), correct feature gating (0 ungated browser APIs — SSR/hydrate gates hold), consolidated `GlobalAppState`, `AppError` taxonomy, 3 working CI workflows |
| **Root-cause gap** | **WASM bundle regressed catastrophically** after `wasm-opt` was removed (commit `6814214`, 2026-04-14). Measured today: **2.4 MB gzipped vs. 618 KB on 2026-04-15** — 4× regression, ~5× over the 500 KB target. Content P0s (sitemap slugs, cert honesty, demo "Coming Soon" pages, domain stitching) remain mostly unchanged per ADR-005 |

**One-sentence verdict:** Engineering floor lifted (composite 6.8 → 7.6) but the bundle silently regressed 4× when `wasm-opt` was pulled to unstick CI — the single finding that actually matters in this audit supersedes everything else on the list.

---

## § Five-lens scores (weighted composite 7.6)

| Lens | Score | Weight | Δ vs 2026-04-15 | Evidence |
|---|---:|---:|---:|---|
| architecture | **8** | 20 | 0 | Module split clean (29 files / 7,132 LOC), `GlobalAppState` consolidation holds, no ungated browser APIs, feature gates honored across csr/hydrate/ssr/ssg/sqlite |
| cs_depth | **8** | 15 | +1 | 12 `unsafe` blocks all in `src/db.rs` with scoped lifetime handling, `AtomicU64` perf counters, four hand-rolled syntax highlighters still shipping |
| rust_idiom | **8** | 20 | +1 | Unwraps down 9 → 7 overall (5 in `src/components/layout.rs` scroll closures, 2 in `src/data/tests.rs`). 0 TODO/FIXME/HACK/XXX anywhere |
| testing | **5** | 15 | 0 | 11 test markers across 2 files (10 in `src/data/tests.rs`, 1 in `src/data/mod.rs`). No hydration smoke, no integration/E2E. CI gate is present but test set is thin |
| ops_product | **8** | 30 | +2 | 3 workflows green (`ci.yml`, `deploy.yml`, `weekly-audit.yml`). Recent CI fixes land (fmt, trunk URL, pull-rebase). **wasm-opt removed** — the structural regression |

Composite = (8·20 + 8·15 + 8·20 + 5·15 + 8·30) / 100 = **7.55 ≈ 7.6**

---

## § Delta vs 2026-04-15 baseline

### What resolved / closed (no longer findings)

- **`inner_html` sanitization P0 (4 sites)** — security-agent refutes. All 4 sinks (`writing.rs:262`, `project.rs:480`, `project.rs:940`, `components/project.rs:148`) render same-origin static JSON the owner authors, never user input. Downgrade: accepted architectural pattern, not a vulnerability.
- **GitHub handle wrong account** — `src/data/mod.rs` now correctly points to `github.com/richmuscle`. Residue persists in comments + anchor text (see P1).
- **LinkedIn URL wrong** — fixed (commit `9ffbbca`).
- **CI workflow breakage** — fixed across 3 workflows (commits `42faebc`, `c8b7bc3`).
- **Rust unwrap cluster in `src/components/nav.rs`** — resolved; unwraps moved out or cleaned up (baseline said 9, now 0 in nav.rs).

### What regressed (new P0 territory)

- **WASM bundle 4× larger**. 2026-04-15 measured 618 KB gzipped. 2026-04-16 measures **2,442,387 bytes gzipped (10,122,467 bytes raw)** from `dist/richardmussell-5d79fa1f55561945_bg.wasm`. The commit that caused it (`6814214`) says *"remove wasm-opt step — apt binaryen 108 corrupts wasm-bindgen output"* — the mitigation killed post-processing entirely instead of swapping to a working toolchain. `index.html:68` still has `data-wasm-opt="0"` so Trunk's built-in opt is also disabled. **This is now the single most damaging finding in the audit.**

### What is unchanged from baseline (ADR-005 content P0s)

Per portfolio ADR-005, content work is deferred by owner choice. These remain open and the 2026-04-15 audit still names them. Not re-detailed here — consult the prior report's § Content P0s block.

---

## § P0 Actions

### P0-1 — Restore `wasm-opt` post-build with a working binaryen source  `[ENG]`

**Evidence:** `dist/…_bg.wasm` measures 10.1 MB raw / 2.4 MB gzipped. 2026-04-15 measured 618 KB gzipped with wasm-opt active. Delta +1.8 MB gzipped. `index.html:68` `data-wasm-opt="0"`. `.github/workflows/deploy.yml` has no `wasm-opt` step after commit `6814214`. Commit `6814214` rationale (apt `binaryen 108` corrupts wasm-bindgen output) was correct — but the response removed the step entirely instead of sourcing a working `wasm-opt`.

**Fix (preferred — use Nix flake):**
`flake.nix` already declares `binaryen`. Add a Nix step to the deploy workflow, or install `binaryen` from a release tarball pinned to a known-good version (≥116) and run `wasm-opt -Oz -o dist/<name>_bg.wasm.opt dist/<name>_bg.wasm && mv dist/<name>_bg.wasm.opt dist/<name>_bg.wasm`.

**Fix (alternative):** Re-enable Trunk's built-in opt by setting `data-wasm-opt="z"` in `index.html:68`; Trunk pulls its own vendored `wasm-opt`, sidestepping the apt package version.

**Verify:** `stat -c%s dist/*.wasm` should return ≤ ~600,000 (raw) or verify `gzip -c dist/*.wasm | wc -c` ≤ 512,000.

### P0-2 — Resolve dual resume-PDF source-of-truth  `[CONTENT + ENG]`

**Evidence:** `public/pdfs/resume.pdf` and `static/pdfs/` both present; `index.html:79` has `copy-dir public/pdfs` so Trunk uses `public/`. The stale `static/pdfs/` path still exists and invites drift. `.github/workflows/deploy.yml:42` gates on size > 100 KB so current state does not break deploy — but leaves the orphan.

**Fix:** Delete `static/pdfs/` entirely. Add `ls static/pdfs/ 2>/dev/null && exit 1` to deploy verification so the directory cannot silently re-appear. Confirm `curl -sI https://richmuscle.github.io/pdfs/resume.pdf | grep content-length` returns the real file post-deploy.

---

## § P1 Actions

### P1-1 — `contact.rs:64` anchor text reads "richardmussell" while link target is `richmuscle`  `[CONTENT]`

**Evidence:** `src/pages/contact.rs:64` — `<a href=GITHUB_URL …>"richardmussell"</a>`. `GITHUB_URL` resolves correctly to `github.com/richmuscle`, but the visible anchor label shows the old handle. User sees "richardmussell" → lands on "richmuscle"'s profile. Same issue in comment form at `src/components/nav.rs:99`, `src/pages/resume.rs:41`, `src/pages/home.rs:111` (comments only; not user-visible but signal drift).

**Fix:** Update anchor text in `contact.rs:64` to `"richmuscle"` or `"@richmuscle"`. Update the three VERIFY comments to match the actual URL. One-line edits, no behavior change.

### P1-2 — Weekly audit workflow collects metrics but does not enforce thresholds  `[OPS]`

**Evidence:** `.github/workflows/weekly-audit.yml` runs `cargo audit`, `cargo deny check`, `cargo outdated` — all with `continue-on-error: true`. Appends to `docs/AUDIT_SCORE_HISTORY.md` (currently header-only, no rows). No gate on bundle-size creep, no gate on new advisories.

**Fix:** Two steps. (a) Remove `continue-on-error` from `cargo audit` and `cargo deny check` for direct-dep vulns only (keep transitive-warning tolerance). (b) Add a bundle gate: `gzip -c dist/*_bg.wasm | wc -c` and fail if > 550,000 bytes. Surface all metrics in a job summary. P0-1 must land first or this gate will immediately fail.

### P1-3 — Test surface thin, not zero (contra audit-agent finding)  `[TEST]`

**Evidence:** `grep '#\[test\]\|#\[cfg(test)\]' src/` returns 11 matches across 2 files: 10 in `src/data/tests.rs`, 1 in `src/data/mod.rs`. audit-agent reported "zero tests found" — a false negative; recon-agent's `test_ratio: 0.3448` is correct. INVENTORY.md §18 corroborates (5 tests in `data/tests.rs`). But: tests concentrate in the data layer only. Zero hydration / integration / route-render coverage. Phase 1 close-out roadmap item "headless smoke test on 3+ routes" still open.

**Fix:** Add one `wasm-bindgen-test` target that mounts `<App/>` and asserts at least one route renders without panic. Keeps CI honest about what's actually verified. Do not block on this for P0-1.

---

## § P2 Actions

| # | Tag | Title | Citation |
|---|---|---|---|
| P2-1 | ENG | Reserved but unused `GlobalAppState` fields (`portfolio_category`, `portfolio_search`, `portfolio_index_tick`) — forward-compat scaffolding w/ no consumer | `src/lib.rs:53–55`, `src/state.rs` |
| P2-2 | ENG | Command palette allocates `Vec<PaletteItem>` per keystroke (~88 Strings/keypress) — perf not user-visible today | `src/components/palette.rs` |
| P2-3 | SECURITY | CSP `unsafe-inline` retained — documented exception per portfolio ADR-006, not a finding, included only for completeness | `index.html:16`, portfolio ADR-006 |
| P2-4 | ENG | Geist font-weight 800 requested by CSS but not bundled → synthetic bold fallback | `style/base.css` (ratio-hero rules), `index.html:36–37` |
| P2-5 | ENG | `.pd-stat-bar` CSS selector declared 3× in `style/components/cards.css` — cascade masks, still technical debt | `style/components/cards.css` |
| P2-6 | ENG | 5 `unwrap()` calls in `src/components/layout.rs` scroll closures — `panic=abort` kills the WASM module on reentry | `src/components/layout.rs` |
| P2-7 | SECURITY | Missing `Referrer-Policy` and `Permissions-Policy` meta headers (X-Frame-Options + CSP present) | `index.html` |
| P2-8 | OPS | 4 `cargo audit` warnings on unmaintained transitives (`instant 0.1.13`, `paste 1.0.15`, `proc-macro-error 1.0.4`, `lru 0.11.1`) — all via Leptos 0.6.15 / rstml 0.11.2; resolves when Leptos 0.7 lands | `Cargo.lock`, `cargo audit` |

---

## § Recon snapshot (appendix)

```json
{
  "timestamp": "2026-04-16T23:11:05Z",
  "file_count": 29,
  "total_lines": 7132,
  "over_400_lines": [
    {"file": "src/pages/project.rs",     "lines": 1278},
    {"file": "src/utils.rs",              "lines": 1006},
    {"file": "src/pages/telemetry.rs",    "lines": 500},
    {"file": "src/data/projects.rs",      "lines": 496},
    {"file": "src/components/nav.rs",     "lines": 469},
    {"file": "src/db.rs",                 "lines": 414}
  ],
  "unwrap_count": 7,
  "todo_count": 0,
  "test_ratio": 0.3448,
  "git_hygiene": {"uncommitted": 0, "untracked": 0}
}
```

**Extras:** dist bundle 12 MB total (incl. fonts/PDFs), raw WASM 10,122,467 bytes, **gzipped WASM 2,500,605 bytes** (measured 2026-04-16 post-baseline). 3 CI workflows present. 37 direct Cargo deps.

---

## § Security scan (appendix)

| Check | Result |
|---|---|
| CSP / X-Frame-Options | **Present** in `index.html` meta (X-Frame-Options `SAMEORIGIN`) |
| Referrer-Policy / Permissions-Policy | Missing (P2-7) |
| `cargo audit` | **0 vulnerabilities**; 4 unmaintained transitive warnings (P2-8) |
| Ungated browser APIs under SSR feature | **0** — Phase 1 gating holds |
| Secrets grep | **0 hits** |
| Dynamic sinks | 7 `js_sys::eval` (all compile-time literal clipboard payloads, safe), 4 `inner_html` (all same-origin static JSON, safe) |
| `unsafe` blocks | **12 total, all in `src/db.rs`** — every block is ADR-004-justified SQLite FFI |
| External origin loads | **0** — fully self-hosted |

---

## § Methodology notes (orchestrator record)

1. **Agent artifacts.** Each subagent was instructed to write `/tmp/{audit,recon,security}.json`. All three returned valid JSON in their final message; none successfully wrote to `/tmp/` (tool permissions). Orchestrator used the returned JSON directly — findings are citable via the JSONL transcripts under `/tmp/claude-1000/.../tasks/`.
2. **audit-agent's composite of 7.8 does not satisfy its own declared weights** (20/15/20/15/30). Recomputed to 7.55 (rounded 7.6). Authoritative value here.
3. **audit-agent's "zero tests" P1 is a false negative.** recon-agent's `test_ratio: 0.3448` and direct `grep` of `#[test]|#[cfg(test)]` return 11 matches across `src/data/tests.rs` (10) and `src/data/mod.rs` (1). Reframed as P1-3 "test surface thin, not zero".
4. **security-agent correctly respects portfolio ADR-004 and ADR-006** — all 12 `unsafe` blocks and the CSP `unsafe-inline` are tagged as documented exceptions, not P0/P1 findings.
5. **No patch-agent run.** Per portfolio `skills/audit.md`: "patch-agent on standby — not activated unless P0s found and confirmed." Use `/orchestrate` for the patch gate, or `/patch` on an explicitly approved P0 list.

---

## § Next-phase recommendation

**Single next action: fix P0-1 (`wasm-opt` restoration) before any Phase 2+ work.** Phase 2 in `docs/ROADMAP.md` is literally "WASM code-splitting + size budget" — it starts from an honest bundle measurement, and today's measurement is 5× the target. Re-enabling `wasm-opt` via the Nix flake or a pinned binaryen release likely recovers 1.5–2 MB of the regression immediately, restoring the 2026-04-15 baseline (~618 KB gzipped) as the starting point for Phase 2's budget work.

P0-2 (PDF source unification) and P1-1 (stale handle in `contact.rs:64`) are both single-commit fixes that can ride behind P0-1 in the same push.
