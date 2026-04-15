# PROJECT_LENS — AI memory map (richardmussell.github.io)

This file is the **high-level mental model** for how the app runs: entry → shell → router → pages → data. For file-by-file fact tables, use **`INVENTORY.md`**. For human process and constraints, use **`CLAUDE.md`** and **`.cursorrules`**.

---

## 1. What this application is

A **single-page portfolio** built as a **Leptos 0.6 CSR** app compiled to **WASM**, served as **static files** (GitHub Pages). There is **no Rust server** in production. All “routes” are **client-side** URL paths handled by `leptos_router`.

---

## 2. Engineering standards & definition of done

**Baseline:** Follow the **Pinnacle Engineering Tenets** in root **`.cursorrules`** (zero panic policy, typed JSON with `serde(deny_unknown_fields)`, WASM performance habits, semantic a11y, wasm32-safe dependencies).

Treat the items below as **definition of done** for merge-ready changes, alongside those tenets.

### Quality gates

1. **Memory safety:** Signals are read via **`.with()`** or **`.get_untracked()`** when reactive tracking is not required, so the work does not subscribe unnecessarily and avoids extra re-renders.
2. **Error handling:** Every **`create_resource`** is wrapped in **`<Suspense />`** (pending state) and **`<ErrorBoundary />`** (or sits under a parent boundary that covers the resource’s rendered subtree with equivalent error UI).
3. **Build integrity:** **`trunk build`** must not emit warnings in the **dev** profile; treat warnings as failures for production-quality work.
4. **CSR invariant:** No code shall be added that **requires Node.js** or a **specialized Rust server** at runtime; the app must remain deployable as static files (e.g. GitHub Pages).

---

## 3. Boot sequence (HTML → WASM → UI)

1. **Browser loads `index.html`** (Trunk’s build target). The body is almost empty except:
   - `#wasm-init-indicator` (loading hint; removed after mount).
   - `<noscript>` fallback text.
   - Inline scripts: WASM preload hint, 10s “failed to load” timeout.

2. **Trunk-injected assets:** hashed JS loader + WASM, plus bundled `style/style.css`, copied fonts under `/fonts/`, and JSON trees under `/writeups/`, `/projects/`, etc. (see `index.html` `data-trunk` links).

3. **`src/main.rs` (`fn main`)**
   - Records WASM start time (`utils::capture_wasm_start_time`).
   - Installs a **panic hook** that writes a red full-screen `div` + `console.error` (helps debug WASM panics).
   - **`mount_to_body(|| view! { <App /> })`** — **full client mount** (not hydration).
   - Removes `#wasm-init-indicator` from the DOM on success.

4. **`App`** (in `src/lib.rs`) becomes the root of the Leptos tree.

---

## 4. `App` shell: context, theme, and chrome

`#[component] pub fn App()` does the following **before** route matching:

| Concern | Mechanism |
|---------|-----------|
| Meta / head | `provide_meta_context()` (`leptos_meta`) |
| Dark mode | `is_dark` / `set_is_dark` signal + `provide_context`; `create_effect` toggles `html` classes and `body` background (**`web_sys`**, behind `#[cfg(not(feature = "ssr"))]`) |
| Command palette | `palette_open` `RwSignal`, `provide_context` |
| Keyboard shortcuts modal | `shortcuts_open` `RwSignal`, `provide_context` |
| Reading progress | `ReadProgressSignals { progress, set_progress }`, `provide_context` |
| Project card expand / drag | `ProjectCardSignals { expanded_slug, set_expanded_slug, did_drag }`, `provide_context` |

**Structural order inside `view!`:**

1. Skip link (`#main-content`).
2. **`<Router>`** wrapping:
   - **`KeyboardNav`** — global key handling.
   - **`NavBar`** — site nav + theme toggle (receives `is_dark` / `set_is_dark`).
   - **`CommandPalette`** — command UI.
   - **`ErrorBoundary`** — styled fallback with back/home actions (uses `web_sys` history under cfg).
   - **`Routes`** — table of **`Route`**s (see §5).
   - **`ReadingProgress`** — progress UI tied to read position.
   - **`BackToTop`**.
   - **`Show`** — keyboard shortcuts modal when `shortcuts_open` is true.

---

## 5. Router → pages (URL to view mapping)

All routes live in **`src/lib.rs`** inside `<Routes>`:

| Path | View component | Source file (typical) |
|------|----------------|------------------------|
| `/` | `HomePage` | `pages/home.rs` |
| `/about` | `AboutPage` | `pages/about.rs` |
| `/writing` | `WritingPage` | `pages/writing.rs` |
| `/writing/:slug` | `WriteupDetailPage` | `pages/writing.rs` |
| `/project/:slug` | `ProjectDetailPage` | `pages/project.rs` |
| `/project/:slug/docs` | `ProjectDocsPage` | `pages/project.rs` |
| `/project/:slug/demo` | `ProjectDemoPage` | `pages/project.rs` |
| `/resume` | `ResumePage` | `pages/resume.rs` |
| `/contact` | `ContactPage` | `pages/contact.rs` |
| `/telemetry` | `TelemetryPage` | `pages/telemetry.rs` |
| `/one-pager` | `OnePageSummary` | `pages/one_pager.rs` |
| `/*any` | `NotFoundPage` | `pages/not_found.rs` |

**Module wiring:** `src/pages/mod.rs` declares submodules and **`pub use`** exports the page components so `lib.rs` can import them in one block.

---

## 6. Shared components (not full pages)

**`src/components/mod.rs`** re-exports:

- **`layout`:** `ReadingProgress`
- **`nav`:** `NavBar`, `KeyboardNav`, `BackToTop`
- **`palette`:** `CommandPalette`
- **`project`:** `ProjectCard` (and any other project UI in that file)
- **`writeup`:** placeholder module; much writeup UI is inlined in `WritingPage` per module comment

These sit **beside** `<Routes>` inside `<Router>` so they see the same router context and can navigate or react globally.

---

## 7. Data and runtime I/O

- **`src/data.rs`:** constants (`EMAIL`, URLs, title), enums/structs for projects, timelines, categories, and **static project/writeup listings** used by pages.
- **JSON on disk:** under `static/writeups/` and `static/projects/` (copied to `dist/` by Trunk). Pages fetch these at runtime for detail views and lists (typical pattern: `gloo_net::Request::get` + `serde` parsing).
- **`src/utils.rs`:** WASM timing, clipboard helpers, and other small utilities; some items are cfg-gated for SSR vs browser.

---

## 8. Build & deploy lens (hydrate + assembly)

- **Trunk** reads **`data-cargo-features`** from **`index.html`** (production: **`hydrate sqlite`**). **`Cargo.toml`** defaults to **`hydrate` + `sqlite`** so wasm builds never enable **`csr`** and **`hydrate`** together; use **`--no-default-features --features csr`** for CSR-only experiments.
- **GitHub Actions** runs **SSG fragments** (`cargo … --no-default-features --features ssg --bin ssg` with `SSG_FRAGMENTS_ONLY=1`), then **`trunk build --release`**, then **`scripts/assemble_dist.py`** to splice each route’s SSR body into the Trunk shell (hashed WASM/JS/CSS unchanged). Fonts and **`.nojekyll`** are verified before deploy.
- **404:** SSG exports `this-route-should-404` into **`dist/404.html`** (GitHub Pages / static hosts).

**Hydration note (Leptos 0.6):** there is no `hydrate_to_body` symbol — **`mount_to_body`** with **`leptos/hydrate`** reconciles against the pre-rendered DOM from SSG. CSR vs hydrate is mutually exclusive via features.

---

## 9. Quick dependency graph (mental, not Cargo)

```text
index.html + Trunk (hydrate + sqlite)
    → main.rs (mount_to_body, hydrate feature)
        → App (lib.rs)
            → Router
                → KeyboardNav, NavBar, CommandPalette
                → ErrorBoundary → Routes → [Page components]
                → ReadingProgress, BackToTop, shortcuts modal
            ← contexts: theme, palette, shortcuts, read progress, project cards
    → pages/* → data.rs + HTTP fetch to /writeups, /projects, …
    → components/* (shared UI)
```

---

## 10. Architectural ascension — SSG hybrid, edge headers, WASM SQLite (Phase 3)

**Goal:** Keep GitHub Pages / static hosting viable while adding an **optional** path toward **pre-rendered HTML**, **Cloudflare Pages** hardening (COOP/COEP for future `SharedArrayBuffer` / worker SQLite), and a **real in-browser SQLite** index for portfolio search.

### SSG + hydration (Leptos 0.6)

- **CSR** remains the **Cargo default** for host-only checks; **production** uses **hydrate** via Trunk + **`mount_to_body`** (see §8).
- **Native binary** `ssg`: **`SSG_OUT`**, optional **`SSG_FRAGMENTS_ONLY=1`** for **`index.body.html`** per route (input to **`scripts/assemble_dist.py`**). CI uses **`CARGO_TARGET_DIR=${{ github.workspace }}/target`** so build scripts run reliably.
- **DOM / `web_sys`:** shared browser crates (`web-sys`, `js-sys`, `gloo-*`, `wasm-bindgen`) live in top-level `[dependencies]` so the library compiles under **`ssr`** for the `ssg` binary. Runtime DOM work stays behind **`#[cfg(not(feature = "ssr"))]`** in `App` where needed.

### Edge (Cloudflare Pages)

- **`public/_headers`**: copied into **`dist/`** via Trunk `[[assets]] source = "public"`. Sets **`Cross-Origin-Opener-Policy: same-origin`** and **`Cross-Origin-Embedder-Policy: require-corp`** (required for advanced WASM threading / some SQLite VFS paths). **Caveat:** `require-corp` breaks cross-origin embeds without CORP; keep WASM/JS/CSS/fonts on the same origin or fix `crossorigin` on third parties.
- **`public/_redirects`**: **`/* /index.html 200`** so unmatched paths get the SPA shell while static SSG files still win when present.

### WASM SQLite (portfolio)

- **`sqlite-wasm-rs`** (optional, **`sqlite`** feature): in-memory VFS; **`src/db.rs`** creates `projects`, bulk-inserts from **`get_infrastructure_fleet`**, and runs **parameterized `SELECT`** for home search. **Fallback** string matching remains if SQLite init fails.
- **Telemetry** surfaces **last query duration (µs)** and **linear memory buffer size** vs a **soft uncompressed budget** constant.

### Invariants

- **Zero panic** policy unchanged (no `.unwrap()` on new hot paths; SQLite errors map to fallback + logging).
- **No-JS:** expanded **`<noscript>`** navigation; **SSG** adds crawlable shells when you run the `ssg` binary.

---

## 11. Assembly line — SSG + Trunk + hydrate (Phase 3 close-out)

**Goal:** One production **`dist/`**: every marketing route is **pre-rendered HTML** (readers without JS see real content), and the same shell **hydrates** to the full WASM app (SQLite-backed search after JSON enrichment).

### CI sequence

1. **`cargo clippy`** — `ssg` binary (`--no-default-features --features ssg`) and WASM (`--no-default-features --features hydrate,sqlite`), **`-D warnings`**.
2. **`cargo build/run --no-default-features --features ssg --bin ssg`** with **`SSG_FRAGMENTS_ONLY=1`** → **`ssg-workspace/**/index.body.html`**. The binary runs export inside a Tokio **`LocalSet`** (for `create_resource` / `spawn_local`) and ends with **`std::process::exit(0)`** to avoid `RuntimeDisposed` races during teardown.
3. **`trunk build --release`** → **`dist/`** with hashed `.wasm` / `.js` / `.css` (from **`index.html`** `data-cargo-features="hydrate sqlite"`).
4. **`python3 scripts/assemble_dist.py`** — merges each fragment into the Trunk template (insert before **`<noscript>`** in **`<body>`**). **`this-route-should-404`** → **`dist/404.html`**.
5. Font verification + **`.nojekyll`**; deploy **`dist/`** to **`gh-pages`**.

### SQLite indexer

- Boot: rows from **`get_infrastructure_fleet`**, **`body_text`** empty; async **`enrich_index_from_static_json`** loads **`/projects/{slug}.json`** into **`body_text`** and bumps **`GlobalAppState::portfolio_index_tick`** so the home grid re-queries.
- Search: parameterized SQL with **`LIKE '%' || lower(?1) || '%'`** over title, subtitle, description, slug, tech stack, and **`body_text`**.

*Update this file when routing, entry mounting, or deployment steps change materially.*

---

## 12. Governance & maintenance (Phase 4)

**Goal:** Make the pipeline auditable for recruiters and future you: runtime evidence on `/telemetry`, supply-chain gates in CI, and performance budgets enforced as code.

### Telemetry (`src/pages/telemetry.rs`)

- **WASM linear memory** from `wasm_bindgen::memory()` (buffer byte length), shown in MB with a **soft budget** progress bar (constant in code; not a hard browser limit).
- **SQLite:** last **JSON→SQLite enrichment** time (µs), **sample search** time after indexing (µs), and last **home search** query time (µs) from `src/db.rs` atomics.
- **Hydration vs static:** on the client, a short **post-tick** message distinguishes **`hydrate`** (reconciled with SSG DOM) vs CSR-only builds; SSR/SSG snapshots show a static HTML note.
- **Build metadata:** `PORTFOLIO_BUILD_GIT_SHA` and `PORTFOLIO_BUILD_TIME` are injected at compile time via **`build.rs`** from environment variables; **GitHub Actions** sets them on the **Trunk** step (`github.sha`, `github.event.head_commit.timestamp`).

### Supply chain (`deny.toml` + CI)

- **`cargo-deny`** runs in **`deploy.yml`** (`EmbarkStudios/cargo-deny-action`): **advisories** (with **ignores** only for known transitive unmaintained crates Leptos 0.6 cannot yet replace), **licenses** (allow-list including **Unicode-3.0**, **BSL-1.0** where required), **bans** (**wildcard `*` denied**; **multiple versions warned** so duplicate bloat is visible without failing on unavoidable Leptos transitive pairs).

### Performance guardrails (Lighthouse CI)

- After **`dist/`** is assembled and fonts verified, **`lhci autorun`** runs against **`lighthouserc.json`**: **Performance ≥ 0.9**, **Accessibility = 1**, **SEO = 1** (desktop preset; categories-only collection to keep runs fast).

### Search UX

- SQLite path uses **weighted relevancy** in SQL (`ORDER BY` scoring title/slug above body); fallback matching uses a parallel score sort. Empty results on the home portfolio filter show a **SQLite-aware** status line when the index is ready.

### Invariants

- **Zero panic** on hot paths; **no new heavy deps** without wasm size justification; site remains **static-host deployable** (GitHub Pages).
