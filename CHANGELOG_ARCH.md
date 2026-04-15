# Architecture changelog

## Phase 1 — SSR migration IN PROGRESS (2026-04-14)

Supersedes the "CSR only" invariant previously asserted in the Pinnacle foundation entry below. The project is mid-migration to Leptos SSR with static export + client-side hydration; CSR is no longer the deployment mode.

### Flipped

- **`Cargo.toml`** — feature split: `default = ["hydrate", "sqlite"]`; `csr` / `hydrate` / `ssr` are mutually exclusive on `wasm32`; `ssg = ["ssr", "dep:tokio"]` for the static export binary; `tokio` and `sqlite-wasm-rs` are gated behind their respective features (e942f30).
- **`src/lib.rs`** — `App` extracted from `main.rs`; provides `GlobalAppState` once and composes `AppShell` + routes (9c9b096).
- **`src/main.rs`** — slimmed to the WASM hydrate entry point; `mount_to_body` replaced with hydrate-mode mounting under `#[cfg(feature = "hydrate")]`.
- **`index.html`** — `data-cargo-features` switched from `csr` to `hydrate sqlite` (dd0c9e9).
- **`src/bin/ssg.rs`** — Leptos SSR `render_to_string` over 27 routes via Tokio `LocalSet`; output assembled into Trunk `dist/` by `scripts/assemble_dist.py` and integrated into `.github/workflows/deploy.yml`.

### Outstanding

- ~80 browser API call sites (INVENTORY.md §7) still need `#[cfg(not(feature = "ssr"))]` or `create_effect` wrapping. Only 5 sites are gated today (`utils.rs:47`, `lib.rs:69`, `main.rs:45/48`, `telemetry.rs:226`).
- No `cargo check --features ssr` CI gate yet — `ssr` is exercised only via the SSG binary build.
- Hydration not yet validated end-to-end on a deployed URL (no console-error / hydration-marker smoke test).

---

# Architecture changelog — Pinnacle foundation (2026-04-11)

Summary of the three-pillar infrastructure pass aligned with `.cursor/PROJECT_LENS.md` §2 (quality gates) and Pinnacle Engineering Tenets.

## Pillar 1 — Safety & resilience

- Replaced panicking `.unwrap()` / `.expect()` on hot paths with `Option` handling or graceful no-ops (`nav`, `layout`, `resume`, `BackToTop`, `KeyboardNav`).
- All `create_resource` loads for project case studies, docs JSON, and write-up bodies are wrapped in **`<Suspense>`** (loading UI + `role="status"`) and **`<ErrorBoundary>`** with **`ComponentErrorFallback`**, which logs render errors to `console.error` on WASM while preserving the rest of the tree.
- Root routes remain under a top-level **`ErrorBoundary`** that composes **`ComponentErrorFallback`** with the existing monospace error screen.

## Pillar 2 — Type-safe data layer

- **`#[serde(deny_unknown_fields)]`** on **`ProjectDetail`** and **`WriteUpDetail`** so static JSON cannot drift silently.
- Central **`fetch_site_data<T>(path)`** in **`src/data.rs`** maps **`gloo_net`** failures and JSON parse errors into **`AppError`** (fetch / parse / logic). Host `cargo check` uses a stub that returns a clear fetch error (no `gloo_net` on non-wasm32).
- **`AppError`** in **`src/error.rs`** uses **`thiserror`** and **`serde`** so it satisfies Leptos **`create_resource`** serialization bounds and integrates with **`ErrorBoundary`** via **`Result` → `IntoView`**.

## Pillar 3 — Scalable UI shell

- **`GlobalAppState`** in **`src/state.rs`** consolidates theme, shortcuts modal, command palette, reading progress, and project-card interaction signals; **`App`** provides it once with **`provide_context`**.
- **`AppShell`** in **`src/components/shell.rs`** owns persistent chrome (**`KeyboardNav`**, **`NavBar`**, **`CommandPalette`**, outlet, **`ReadingProgress`**, **`BackToTop`**); routed pages stay in **`src/pages/`** without re-declaring that structure.
- **`NavBar`** / **`CommandPalette`** / chrome read from **`GlobalAppState`**; **`HomePage`** uses the same for **`ProjectCardSignals`** with a defensive empty state if context is missing.

## Invariants preserved (as of 2026-04-11; superseded by the Phase 1 entry above)

- ~~**CSR only:** `index.html` still uses **`data-cargo-features="csr"`**; **`main.rs`** still uses **`mount_to_body`**.~~ — no longer true as of dd0c9e9 (2026-04-14); see Phase 1 entry.
- ~~**No SSR/hydration** introduced for the GitHub Pages deployment path.~~ — no longer true; hydrate is now the default deploy mode.

## Verification

- `cargo check --features csr --target wasm32-unknown-unknown` and `cargo build` (same) complete **without rustc warnings** in this environment.
- **`trunk build`** may be affected by local Trunk/CLI flags; use a supported Trunk release if `--no-color` errors appear in CI.

## Dependencies

- **`thiserror`** added for structured **`AppError`** (small, WASM-safe).

---

## Phase 2 — Polish & performance (2026-04-11)

### Idiomatic Rust / Clippy

- **`cargo clippy --target wasm32-unknown-unknown --features csr`** run clean (no warnings) in this environment (use `CARGO_TARGET_DIR=target` if the default cargo cache cannot execute build scripts).
- Continued preference for **`leptos::logging`** over direct **`web_sys::console`** in error paths (**`ComponentErrorFallback`**, **`main`** panic hook).

### WASM release profile

- **`[profile.release]`** in **`Cargo.toml`**: **`lto = true`**, **`opt-level = "z"`**, **`codegen-units = 1`**, **`panic = "abort"`**, **`strip = true`** for smaller Wasm output.

### Trunk / compression

- **`Trunk.toml`** documents fingerprinted Wasm output and that **brotli/gzip** precompression is a **host/CDN** concern (GitHub Pages does not serve `.br`/`.gz` sidecars without extra configuration).

### Content engine (home portfolio)

- **`GlobalAppState`** drives **`portfolio_category`** and **`portfolio_search`**; **`HomePage`** search field (`role="search"`) and category tabs filter the grid and tab counts consistently.
- **`ProjectDetailBodySkeleton`** provides a stable layout inside **`<Suspense>`** on project detail and docs routes.

### A11y / command palette

- **Command palette**: dialog **`tabindex="-1"`**, combobox/listbox roles, **`aria-hidden`** on emoji icons, result rows as **`role="option"`** with **`aria-selected`**.
- **`/`** focuses **`#terminal-input`** on the home route when present; otherwise toggles the palette (same signal as **Cmd/Ctrl+K**). Shortcuts help table documents **⌘/Ctrl+K**.

### Error UX

- **`ComponentErrorFallback`** includes a **Return home** link to **`/`** alongside logging via **`leptos::logging::error!`**.

### Docs

- Public **`AppError`** and several **`data`** / **`error`** types now have **`///`** summaries.

### Verification

- **`cargo check --target wasm32-unknown-unknown --features csr`** succeeds (project-local target dir as above).
