# Engineering Upgrade Roadmap

## Phase 0: Baseline and pre-existing bug fixes — COMPLETE
Goal: establish a clean base for future work.
- [x] Create `revamp` branch off main
- [x] Full codebase inventory (INVENTORY.md, 987 lines, 17 sections)
- [x] Set up staging remote (revamp-origin → github.com/richmuscle/portfolio-revamp)
- [x] Bug 1: Mount ReadingProgress component with scroll listener cleanup (5110319)
- [x] Bug 2: Add Trunk copy-dir directive for static/docs (1c193bc)
- [x] Bug 3: Unify rustflags in .cargo/config.toml instead of CI env override (867bd38)
- [x] Project persistence layer (CLAUDE.md, ROADMAP.md, DECISIONS.md)

## Phase 1: Leptos SSR with static export — COMPLETE — 2026-04-14
Goal: migrate from CSR to SSR with prerendered HTML + hydration, still deployable to GitHub Pages as static output.

Done:
- [x] Restructure Cargo.toml: hydrate/ssr/ssg feature split, WASM-only deps gated to wasm32, tokio gated behind ssg (e942f30)
- [x] Split App out of main.rs into src/lib.rs; main.rs slimmed to WASM hydrate entry point (9c9b096)
- [x] Switch Trunk directive from csr to hydrate feature (dd0c9e9)
- [x] Wrap all browser API call sites — `cargo check --no-default-features --features ssr` now produces 0 errors, 7 warnings (d593871)
- [x] src/bin/ssg.rs static export binary (Leptos SSR render_to_string + Tokio LocalSet, 27 routes) (e9bdc7b)
- [x] GlobalAppState context consolidation — 5 provide_context calls unified (d9e8463)
- [x] SQLite WASM search scaffolded behind `sqlite` feature (4a31d1e)
- [x] SSR CI gate — `cargo check --features ssr` + `cargo check --features hydrate --target wasm32-unknown-unknown` on push/PR to revamp (.github/workflows/ci.yml)

Outstanding (carry to Phase 1 close-out):
- [ ] Validate hydration end-to-end on a deployed URL (no console errors, hydration markers present)
- [ ] Headless smoke test on 3+ routes

## Phase 2: WASM code-splitting + size budget — NOT STARTED
Goal: route-level WASM chunks, aggressive wasm-opt, real measured bundle size surfaced on the telemetry page as evidence.

## Phase 3: In-browser SQLite search — NOT STARTED
Goal: rusqlite compiled to WASM, powering writeups/projects search instead of in-memory JSON. Memorable technical signal tied to real functionality.

## Phase 4: Rust service worker — NOT STARTED
Goal: offline support, instant repeat loads. Must come after Phase 2 (code-splitting) or it caches stale chunks.

## Phase 5: WebGPU systems visualization — NOT STARTED
Goal: ambient background visualization tied to real engineering work (e.g., Raft leader election animation). Visitor "whoa" moment. Heavy dep, must be budgeted against Phase 2's size budget.

## Phase 6: Cloudflare Pages migration + preview deploys per PR — NOT STARTED
Goal: unlock real HTTP headers, Brotli, and per-PR preview URLs.

## Phase 7: Lighthouse CI gate + WASM size budget gate — NOT STARTED
Goal: green checks on PRs, enforced perf/a11y/SEO thresholds.

## Phase 8: Brotli + content-addressed hashing + long cache headers — NOT STARTED
Depends on Phase 6.

## Phase 9: cargo audit + cargo deny + pinned toolchain + release-please — NOT STARTED
Goal: supply chain and release polish.

## Deferred: content and substance work
The initial audit surfaced that the site has an "identity crisis" — title drift across pages, aspirational language mismatched with documented experience, no completed certs, no PDF resume, no impact metrics on work history, broken sitemap referencing nonexistent project slugs, demo pages all say "Coming Soon". This work is deferred by owner choice, but it is the highest-leverage work for the stated goal of landing roles. Any session should remind the owner this exists when appropriate. Do not let the engineering phases become a permanent distraction from it.
