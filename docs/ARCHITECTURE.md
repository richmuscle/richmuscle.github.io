# Architecture — richardmussell.github.io

Current state as of 2026-04-16. Source of truth: the commit this file lives in.
Historical journal removed — see `docs/audits/` for audit history and
`docs/DECISIONS.md` + `~/.claude/projects/portfolio/DECISIONS.md` for ADRs.

## 1. Stack

- Rust edition 2021, toolchain pinned to `stable` via `rust-toolchain.toml`
  with `wasm32-unknown-unknown` target and `rustfmt` + `clippy` components.
- Leptos 0.6 (`leptos`, `leptos_meta`, `leptos_router`) as the UI framework,
  CSR-mounted to the DOM.
- Compile target `wasm32-unknown-unknown`, bundled via Trunk.
- `serde` + `serde_json` for data-index deserialization; `thiserror` for
  the `AppError` taxonomy.
- Browser-only deps (`web-sys`, `js-sys`, `gloo-net`, `gloo-timers`,
  `getrandom` with the `js` feature) are gated under
  `cfg(target_arch = "wasm32")`.
- Optional in-browser SQLite search via `sqlite-wasm-rs 0.5.2`, gated on the
  `sqlite` cargo feature (on by default).
- Native host-only dep `tokio` is gated under the `ssg` cargo feature and
  only pulled in when building the static-site-generator binary.

See `Cargo.toml` and `rust-toolchain.toml`.

## 2. Build pipeline

- `Trunk.toml` declares `target = "index.html"`, dev server on port 8002,
  and a single `[[assets]] source = "public"` block that copies the
  `public/` tree (including `robots.txt`, `sitemap.xml`, `og-image.png`,
  PDFs and diagrams) into `dist/` on every build.
- `index.html` drives the rest of the bundle via Trunk link directives:
  `rel="rust"` (WASM compile, `data-wasm-opt="0"`), `rel="scss"` for
  `style/style.scss`, and `copy-dir` / `copy-file` entries for fonts,
  writeup/project/doc/demo JSON, PDFs, diagrams, manifest, favicons and OG
  image.
- `.cargo/config.toml` sets `wasm32-unknown-unknown` link flags:
  `--no-entry`, `-z stack-size=1048576`, `--import-memory`, and
  `target-feature=+bulk-memory`.
- GitHub Actions workflows in `.github/workflows/`:
  - `ci.yml` — branch check: `cargo check` across feature gates,
    `cargo fmt --check`, clippy, cargo-deny.
  - `deploy.yml` — build-and-publish to GitHub Pages on push to `main`:
    `trunk build --release`, copy fonts into `dist/fonts/`, duplicate
    `index.html` to `404.html` for SPA refresh support, touch `.nojekyll`,
    and deploy via `actions/upload-pages-artifact` +
    `actions/deploy-pages`.
  - `weekly-audit.yml` — scheduled audit snapshot committed on a cadence.
- `flake.nix` provides a Nix dev shell with stable Rust +
  `wasm32-unknown-unknown` target, `trunk`, `binaryen`, and `openssl`.
- `justfile` exposes local task shortcuts matching the CI verification
  matrix.

## 3. Feature gates

Cargo features, defined in `Cargo.toml`:

- `csr` — client-side render. Enables `leptos/csr`, `leptos_meta/csr`,
  `leptos_router/csr`.
- `hydrate` — hydration path for pre-rendered HTML.
- `ssr` — server-side render (non-WASM target).
- `ssg` — superset of `ssr` plus `tokio`; gates `src/bin/ssg.rs`.
- `sqlite` — pulls in `sqlite-wasm-rs` for the in-browser search index.

`default = ["csr", "sqlite"]` — the shipped bundle is CSR with SQLite
search. See portfolio ADR-001 (CSR deploy) and portfolio ADR-007
(feature gating).

## 4. Module topology

Crate root is `src/lib.rs`; WASM entry point is `src/main.rs`.

```
src/
├── lib.rs            — App component, router, ErrorBoundary, context wiring
├── main.rs           — WASM entry: panic hook, mount_to_body, init indicator
├── state.rs          — GlobalAppState struct (single-context consolidation)
├── error.rs          — AppError enum (Fetch / Parse / Logic) via thiserror
├── db.rs             — SQLite WASM index + pure-Rust search fallback
├── utils.rs          — perf_now, scroll-lock, wasm-start-time capture
├── data/
│   ├── mod.rs        — shared types (ProjectCategory, Certification, …)
│   ├── projects.rs   — LazyLock<Vec<ProjectIndex>> compile-time index
│   ├── writeups.rs   — LazyLock<Vec<WriteUpIndex>> compile-time index
│   ├── certs.rs      — LazyLock<Vec<Certification>> compile-time index
│   └── tests.rs      — data-integrity unit tests
├── components/
│   ├── mod.rs        — re-exports
│   ├── nav.rs        — NavBar, ThemeToggle, BackToTop, KeyboardNav
│   ├── layout.rs     — ReadingProgress scroll listener
│   ├── palette.rs    — CommandPalette (⌘K)
│   ├── project.rs    — ProjectCard + related view helpers
│   ├── site_footer.rs — unified SiteFooter
│   └── error_fallback.rs — ErrorBoundary fallback view helpers
├── pages/
│   ├── mod.rs        — re-exports
│   ├── home.rs       — HomePage + Terminal + CertificationsSection
│   ├── about.rs      — AboutPage
│   ├── writing.rs    — WritingPage + WriteupDetailPage
│   ├── project/
│   │   ├── mod.rs    — re-exports (keeps `pages::Project…` paths stable)
│   │   ├── detail.rs — ProjectDetailPage
│   │   ├── docs.rs   — ProjectDocsPage
│   │   └── demo.rs   — ProjectDemoPage
│   ├── resume.rs     — ResumePage
│   ├── contact.rs    — ContactPage
│   ├── telemetry.rs  — TelemetryPage
│   ├── one_pager.rs  — OnePageSummary
│   └── not_found.rs  — NotFoundPage
└── bin/
    └── ssg.rs        — static-site-generator (ssg feature only)
```

The `src/pages/project/` directory replaces an earlier monolithic
`project.rs`; the `mod.rs` re-exports preserve `pages::ProjectDetailPage`
et al. as import targets.

## 5. Routing

Eleven client-side routes declared in `src/lib.rs`, rendered via
`leptos_router::Router` with a single `<Routes>` block inside the app-wide
`ErrorBoundary`:

| Path                    | Component            |
|-------------------------|----------------------|
| `/`                     | `HomePage`           |
| `/about`                | `AboutPage`          |
| `/writing`              | `WritingPage`        |
| `/writing/:slug`        | `WriteupDetailPage`  |
| `/project/:slug`        | `ProjectDetailPage`  |
| `/project/:slug/docs`   | `ProjectDocsPage`    |
| `/project/:slug/demo`   | `ProjectDemoPage`    |
| `/resume`               | `ResumePage`         |
| `/contact`              | `ContactPage`        |
| `/telemetry`            | `TelemetryPage`      |
| `/one-pager`            | `OnePageSummary`     |
| `/*any`                 | `NotFoundPage`       |

Routing is hash-free path-based, served via CSR. GitHub Pages SPA-refresh
support comes from the deploy workflow copying `index.html` to
`404.html` in `dist/`. See portfolio ADR-001.

## 6. State management

One `provide_context(GlobalAppState)` call in `App` replaces five
previously-separate contexts. `GlobalAppState` (defined in `src/state.rs`)
exposes named fields for dark-mode signals, shortcuts / palette open
flags, reading-progress signals, project-card drag signals, and
portfolio-index filter/search/tick signals. Named fields avoid the
`RwSignal<bool>` type-collision that affected the earlier five-context
layout. See portfolio ADR-003.

Per-page local state lives inside each page component as
`create_signal` / `create_rw_signal` / `store_value` / `create_memo`.

## 7. Data layer

Compile-time indices live in `src/data/*.rs` as
`LazyLock<Vec<T>>` singletons: `ALL_PROJECTS`, `ALL_WRITEUPS`,
`ALL_CERTS`. These carry card-level metadata (title, summary, slug,
category, tags, status).

Detail bodies are fetched at runtime from the static asset tree:

- `/writeups/{slug}.json` — writing article bodies.
- `/projects/{slug}.json` — project detail bodies.
- `/docs/{slug}.json` — project docs sub-pages.

Fetches use `gloo_net::http::Request` inside `create_resource`, deserialized
via `serde_json`. The JSON trees live under `static/writeups/`,
`static/projects/`, `static/docs/` and are copied into `dist/` by Trunk
`copy-dir` directives.

## 8. Error model

`AppError` (defined in `src/error.rs`) is a three-variant `thiserror`
enum: `Fetch(String)`, `Parse(String)`, `Logic(String)`, implementing
`Serialize` + `Deserialize` so resources can cache typed errors.

A root `ErrorBoundary` in `App` renders a `error[E0308]` panic-style
fallback. Page-level `ErrorBoundary`s in `pages/project/` and
`pages/writing.rs` catch section-scoped fetch/parse failures so a single
broken writeup or project does not take down the surrounding page.
See portfolio ADR-006.

## 9. Styling

Layered SCSS import order in `style/style.scss`:

1. `tokens` — design tokens (colors, spacing, fonts).
2. `base` — resets and global element defaults.
3. `components/*` — layout, nav, buttons, cards, palette, site-footer.
4. `pages/*` — per-page overrides (home, about, resume, project, writing,
   telemetry, contact).

Trunk bundles the sheet via `rel="scss"`. See portfolio ADR-002.

## 10. Browser API gating

Every browser API call site is wrapped in `#[cfg(not(feature = "ssr"))]`
or equivalent target-arch gate so `cargo check --features ssr` compiles
cleanly on native without `web-sys` stubs. The 2026-04-16 security audit
(`docs/audits/2026-04-16.md`) confirms zero ungated browser-API
references across the current tree.

## 11. SQLite search (optional)

Under `cfg(feature = "sqlite")` (default), `src/db.rs` opens an in-memory
SQLite database via `sqlite-wasm-rs` using the memory VFS and seeds it
from `static/projects/*.json` at startup. `search_projects()` and
`search_portfolio_projects()` are the query entry points.

When the `sqlite` feature is disabled, `search_fallback()` delivers
pure-Rust substring search over the compile-time `ALL_PROJECTS` index so
search never silently breaks.

The `unsafe` blocks in `src/db.rs` handle the FFI lifecycle for the
SQLite C handle with scoped lifetimes. See portfolio ADR-004.

## 12. Deploy

GitHub Actions (`deploy.yml`) → `trunk build --release` on
`ubuntu-latest` → `dist/` uploaded via `actions/upload-pages-artifact`
→ published by `actions/deploy-pages` to GitHub Pages static hosting
under `richmuscle.github.io`. Static hosting means no server runtime and
no response-header control — CSP and security headers are inlined as
`<meta http-equiv>` in `index.html`.

Current open issue: `wasm-opt` post-processing is absent from the deploy
pipeline. `index.html` sets `data-wasm-opt="0"` (Trunk's built-in pass
off), and the explicit post-build `wasm-opt` step was removed in commit
`6814214`. See `docs/audits/2026-04-16.md` P0-1 for the restoration
plan and bundle-size evidence.
