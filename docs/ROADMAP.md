# Portfolio Roadmap

This document tracks two parallel workstreams: a **Content & Positioning Track** (the Stage 2.x series calibrating the site's voice, scope claims, and cross-surface consistency) and an **Engineering Track** (the Phase 0–9 technical upgrade plan that established the platform). Both tracks are real work. Neither is "deferred" in the sense of "ignored" — each pauses when the other is loaded, and only one is active at a time.

**Currently active:** Content & Positioning Track. The Engineering Track has been paused since 2026-04-14 when the Stage 2.1 series opened. Resumption is gated on the Content Track reaching a stopping point — most likely after Stage 2.3F closes. See "Active status" at the bottom.

---

## Content & Positioning Track

Driven by the 2026-04-20 staff-lens audit (composite 6.2/10, three P0s, several P1s) at `docs/audits/2026-04-20-staff-lens.md`. The audit surfaced that the site's voice and scope claims had drifted from the owner's actual experience — grandeur language without evidence, tier self-declarations, cross-surface contradictions, unsupported metrics, and aspirational cert language. This track is the systematic honesty pass.

Working discipline is captured in `CLAUDE.md` and the per-stage closeouts: single-commit scope, prose drafted in markdown and owner-approved before Rust edits, evidence-backed pushback when scope needs expansion, separate closeout commit per stage.

### Done

| Stage | Scope | Commit |
|---|---|---|
| 2.1 | Identity sweep — title and tier normalization across user-visible surfaces | See `STAGE2.1-COMPLETE.md` |
| 2.2A | Case study structure pass | See `STAGE2.2A-COMPLETE.md` |
| 2.2B | Case study voice reframe | See `STAGE2.2B-COMPLETE.md` |
| 2.3A | About page voice rewrite (anchor for all downstream calibration) | See `STAGE2.3A-COMPLETE.md` |
| 2.3B | Writing page intro rewrite | See `STAGE2.3B-COMPLETE.md` |
| 2.3C | Unsupported metrics reframed as design targets | See `STAGE2.3C-COMPLETE.md` |
| 2.3H | Cert honesty sweep — drop aspirational claims | See `STAGE2.3H-COMPLETE.md` |
| 2.3I | Senior-tier residue sweep — verified clean | See `STAGE2.3I-COMPLETE.md` |
| 2.3I-verify | Re-sweep after 2.3J/L/M — confirmed no regression | See `STAGE2.3I-VERIFY-COMPLETE.md` |
| 2.3J | Resume PISCES framing alignment | See `STAGE2.3J-COMPLETE.md` |
| 2.3K | Internal resume variants — PISCES alignment + banned-verb scrub | See `STAGE2.3K-COMPLETE.md` |
| 2.3L | Dead author-line rewriter cleanup | See `STAGE2.3L-COMPLETE.md` |
| 2.3M | PISCES writeup body rewrite | See `STAGE2.3M-COMPLETE.md` |

### Audit findings resolved

- P0-1 (cross-surface PISCES contradiction) — closed by 2.3J, extended by 2.3K
- P0-2 (unsupported identity metrics) — closed by 2.3C
- P0-3 (unsupported observability metrics) — closed by 2.3C
- P1-1 (cert aspirational language) — closed by 2.3H
- P1-4 (senior-tier self-declaration residue) — closed by 2.3I, reconfirmed by 2.3I-verify

### Open

Ordered cheapest-first per established working cadence.

| Stage | Priority | Scope | Est. cost |
|---|---|---|---|
| ROADMAP regen | — | Documentation hygiene — this document | In progress |
| Dead CSS cleanup | — | Unused cert-* classes from 2.3H + `.about-pills-row` from 2.3A | 20 min |
| About-page dashboards-to-tickets tightening | — | Small content edit to `src/pages/about.rs` Section 1 para 2 per 2.3M note | 30 min |
| Terraform "1 → 0 by design" verification | — | Grep private Terraform repo for zero-public-listener enforcement; add config evidence reference to `identity-access-lifecycle.json` | 30 min |
| 2.3G | P3 | OG image regeneration — template edit + screenshot | 1 session |
| 2.3F | P1 | KEEP-writeup voice pass — `cisco-ios-fundamentals.json` + `windows-server-lab-powershell-automatedlab.json` wholesale rewrites | 1–2 sessions |

### Deferred

| Stage | Priority | Why deferred |
|---|---|---|
| 2.3D | P1 | InDevelopment project content voice. Both projects confirmed `planned` not `in-development` (no deployed code, private notes only). Staff-engineer move is status downgrade in `src/data/projects.rs`. But this stage opens a larger question: eight public GitHub repos (`gitops-platform`, `cicd-showcase`, `observability-stack`, `zerotrust-admin-fabric`, `ansible-baseline`, `terraform-landing-zone`, `threat-intel-aggregator`, `richmuscle.github.io`) all carry boilerplate "Portfolio project" descriptions. Whether those repos contain real code or are shells is unknown. If shells, that's a P0 larger than anything this series has closed — fabrication at the repo level. Needs fresh session with owner triage of all eight repos before 2.3D opens. |
| 2.3E | P3 | Platform page build — blocked on 2.3D resolution. |
| 2.4 | — | Planned project builds. Long-horizon work, out of scope for current cleanup pass. |

### Entry conditions for Engineering Track resumption

Content Track is stopping at the close of 2.3F (KEEP-writeup voice pass). At that point: all P0s resolved, all P1s except 2.3D resolved, the largest remaining voice-calibration work complete. Everything after 2.3F on the open list is hygiene-level (dead CSS, OG image, ROADMAP itself, Terraform verification) and can be interleaved with Engineering Track work without cross-contamination. 2.3D stays deferred until owner triages the eight-repo question — that's a scope decision, not an execution one, and it shouldn't block engineering work.

---

## Engineering Track

Multi-phase technical upgrade plan establishing the platform's foundation. Phase 0 and Phase 1 are complete. Phase 2 onward is paused.

### Phase 0 — Baseline and pre-existing bug fixes — COMPLETE

Goal: establish a clean base for future work.

- Create `revamp` branch off main
- Full codebase inventory (`INVENTORY.md`, 987 lines, 17 sections)
- Staging remote setup (`revamp-origin` → github.com/richmuscle/portfolio-revamp)
- Bug 1: Mount ReadingProgress component with scroll listener cleanup (5110319)
- Bug 2: Trunk copy-dir directive for static/docs (1c193bc)
- Bug 3: Unify rustflags in `.cargo/config.toml` instead of CI env override (867bd38)
- Project persistence layer (`CLAUDE.md`, `ROADMAP.md`, `DECISIONS.md`)

### Phase 1 — Leptos SSR with static export — COMPLETE — 2026-04-14

Goal: migrate from CSR to SSR with prerendered HTML + hydration, still deployable to GitHub Pages as static output.

- Cargo.toml restructure: hydrate/ssr/ssg feature split, WASM-only deps gated to wasm32, tokio gated behind ssg (e942f30)
- App split out of main.rs into `src/lib.rs`; main.rs slimmed to WASM hydrate entry point (9c9b096)
- Trunk directive switched from csr to hydrate feature (dd0c9e9)
- All browser API call sites wrapped — `cargo check --no-default-features --features ssr` produces 0 errors, 7 warnings (d593871)
- `src/bin/ssg.rs` static export binary (Leptos SSR render_to_string + Tokio LocalSet, 27 routes) (e9bdc7b)
- GlobalAppState context consolidation — 5 provide_context calls unified (d9e8463)
- SQLite WASM search scaffolded behind `sqlite` feature (4a31d1e)
- SSR CI gate — `cargo check --features ssr` + `cargo check --features hydrate --target wasm32-unknown-unknown` on push/PR to revamp (`.github/workflows/ci.yml`)

Outstanding (Phase 1 close-out, carrying):

- Validate hydration end-to-end on a deployed URL (no console errors, hydration markers present)
- Headless smoke test on 3+ routes

### Phase 2 — WASM code-splitting + size budget — NOT STARTED

Goal: route-level WASM chunks, aggressive wasm-opt, real measured bundle size surfaced on the telemetry page as evidence.

### Phase 3 — In-browser SQLite search — NOT STARTED

Goal: rusqlite compiled to WASM, powering writeups/projects search instead of in-memory JSON. Memorable technical signal tied to real functionality.

### Phase 4 — Rust service worker — NOT STARTED

Goal: offline support, instant repeat loads. Must come after Phase 2 (code-splitting) or it caches stale chunks.

### Phase 5 — WebGPU systems visualization — NOT STARTED

Goal: ambient background visualization tied to real engineering work (e.g., Raft leader election animation). Heavy dep, must be budgeted against Phase 2's size budget.

### Phase 6 — Cloudflare Pages migration + per-PR preview deploys — NOT STARTED

Goal: unlock real HTTP headers, Brotli, and per-PR preview URLs.

### Phase 7 — Lighthouse CI gate + WASM size budget gate — NOT STARTED

Goal: green checks on PRs, enforced perf/a11y/SEO thresholds.

### Phase 8 — Brotli + content-addressed hashing + long cache headers — NOT STARTED

Depends on Phase 6.

### Phase 9 — cargo audit + cargo deny + pinned toolchain + release-please — NOT STARTED

Goal: supply chain and release polish.

---

## Active status

**As of 2026-04-21:** Content & Positioning Track is active. Engineering Track is paused at end of Phase 1. Next Content Track stage is ROADMAP regen (this document), then dead CSS cleanup per the backlog ordering above.

Phase 1 has two Phase-1-closeout items still outstanding (hydration validation, headless smoke test). Those remain Engineering Track work and will open when Phase 2 starts, not before.

## Note on prior "Deferred" framing

Prior versions of this document contained a single "Deferred: content and substance work" paragraph at the bottom noting that content work was "deferred by owner choice" but was "the highest-leverage work for the stated goal of landing roles." That framing was accurate at the time it was written. It became stale once the Stage 2.1 series opened on 2026-04-14 and has been stale since. The Content & Positioning Track section above replaces it.
