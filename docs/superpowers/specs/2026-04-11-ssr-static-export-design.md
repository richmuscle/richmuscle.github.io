# Phase 1: Leptos SSR with Static Export — Design Spec

Date: 2026-04-11  
Status: Approved  
Branch: revamp  

---

## Goal

Migrate from Leptos CSR (`csr` feature) to true SSR with client-side hydration (`hydrate` + `ssr` features). Pre-render each route to static HTML at CI build time. WASM loads and hydrates the server-rendered DOM — no flash, no layout shift on content, measurably faster time-to-interactive. Remains deployable to GitHub Pages as static files.

---

## Approach

Approach A: Effect-safe gating + resource suspension.

- `create_effect`, event handlers, and `fn main()` are already safe in `hydrate` mode — the SSR renderer never calls them. These are not touched.
- Browser API gating is applied only where the compiler requires it (WASM-only imports) and where synchronous component-body calls would execute during SSR render.
- `create_resource` fetchers return `None` on the server — Suspense boundaries render `"Loading..."` in static HTML, real content loads post-hydration.
- A native `ssr_gen` binary renders each route to HTML, reads Trunk's `dist/index.html` as the shell, and writes `dist/<route>/index.html`.
- No `leptos_axum` dependency. No `cargo-leptos`. Trunk remains the WASM build tool.

---

## Section 1: Dependency Architecture

### Cargo.toml restructure

**Feature flags (new):**
```toml
[features]
hydrate = ["leptos/hydrate", "leptos_meta/hydrate", "leptos_router/hydrate"]
ssr     = ["leptos/ssr",     "leptos_meta/ssr",     "leptos_router/ssr"]
```

**Universal dependencies (feature-stripped):**
```toml
[dependencies]
leptos       = { version = "0.6" }
leptos_meta  = { version = "0.6" }
leptos_router = { version = "0.6" }
serde        = { version = "1", features = ["derive"] }
```

**WASM-only dependencies** (moved from `[dependencies]`):
```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys      = { version = "0.3", features = [...] }
js-sys       = "0.3"
gloo-net     = { version = "0.5", features = ["http"] }
gloo-timers  = { version = "0.3", features = ["futures"] }
getrandom    = { version = "0.2", features = ["js"] }
```

### Trunk directive change

`index.html` Rust link directive:
```html
<link data-trunk rel="rust" data-wasm-opt="0" data-cargo-features="hydrate"/>
```

`csr` → `hydrate`. No other Trunk changes.

---

## Section 2: Browser API Gating Strategy

### Category 1 — Already safe, no changes

All browser API calls inside `create_effect`, event handlers (`on:click`, `on:keydown`, etc.), scroll listeners (`Closure::forget()`), and `fn main()` are not called by the SSR renderer. Do not add `#[cfg]` here.

Affected inventory entries: all of §7 under effects, event handlers, `BackToTop` scroll listener, `KeyboardNav` global listener, `ReadingProgress` scroll listener, clipboard calls in `on:click` handlers, `window.print()` calls, `window.history().back()` calls.

### Category 2 — Import-level gating

Files with top-level WASM-only imports that will fail to compile on native target once the dep split lands. Fix: `#[cfg(not(feature = "ssr"))]` on the `use` statements, not scattered at call sites.

Affected files (9):
- `src/main.rs` — `use web_sys::*` (only needed for `fn main()` which is also gated)
- `src/utils.rs` — `web_sys::window()` in `perf_now_ms()`
- `src/components/nav.rs` — `web_sys`, `gloo_timers` imports
- `src/components/layout.rs` — `web_sys`, `wasm_bindgen::closure::Closure`
- `src/components/palette.rs` — `web_sys` import
- `src/pages/home.rs` — `web_sys`, `js_sys`, `gloo_timers`
- `src/pages/resume.rs` — `js_sys`
- `src/pages/one_pager.rs` — `js_sys`
- `src/pages/telemetry.rs` — entire file is browser-only (skipped in SSR, but imports still compile)

Pattern:
```rust
#[cfg(not(feature = "ssr"))]
use web_sys::Window;
```

### Category 3 — Synchronous component-body calls

Browser API calls that execute during SSR render pass (outside effects/handlers). Exact sites:

- `src/utils.rs` — `perf_now_ms()`: needs `#[cfg]` stub returning `0.0_f64` on SSR
- `src/utils.rs` — `WASM_START_TIME_MS: OnceLock<f64>`: entire static + accessor gated `#[cfg(not(feature = "ssr"))]`; SSR gets `fn wasm_start_time_ms() -> f64 { 0.0 }`
- `src/main.rs` — `fn main()`: entire body is `#[cfg(not(feature = "ssr"))]`; `capture_wasm_start_time()` call is inside it, inherently gated

**Hard rule:** `#[cfg]` gates go only on imperative browser calls and imports. Never around `view!` elements, signal initializations, or any component tree structure. Structural differences between SSR and hydrate render paths break Leptos's hydration ID sequence.

### Category 4 — `create_resource` / `gloo_net` fetchers

`gloo_net` is WASM-only — its HTTP client does not exist in the SSR binary. Resources with `gloo_net` fetchers need dual implementations:

```rust
#[cfg(not(feature = "ssr"))]
let detail = create_resource(move || slug(), |s| async move {
    gloo_net::http::Request::get(&format!("/writeups/{s}.json"))
        .send().await.ok()
        .and_then(|r| r.json::<WriteUpDetail>().await.ok())  // simplified
});
#[cfg(feature = "ssr")]
let detail = create_resource(move || slug(), |_| async move {
    None::<WriteUpDetail>
});
```

Affected: `src/pages/writing.rs:180`, `src/pages/project.rs:12`, `src/pages/project.rs:161`.

Suspense fallbacks (`"Loading..."`) render in static HTML. Post-hydration, the client resource resolves and re-renders with real content. Expected, correct behavior.

`ProjectDocsPage` (`src/pages/project.rs:157`) currently has no `<Suspense>` wrapper — add one as part of this migration for consistency.

---

## Section 3: `ssr_gen` Binary

**File:** `src/bin/ssr_gen.rs`  
**Compiled with:** `cargo build --release --features ssr --bin ssr_gen`  
**Target:** native host (`x86_64-unknown-linux-gnu` in CI)

### Rendering loop

For each route:
1. Provide `RouterIntegrationContext::new(ServerIntegration { path: route.to_string() })` — gives the router a URL to resolve
2. Call `leptos::ssr::render_to_string(|| { let meta = MetaContext::new(); provide_context(meta.clone()); view! { <App/> } })` — returns rendered body HTML
3. Call `meta.render_tags()` to collect `<title>`/`<meta>`/`<link>` tags emitted by `leptos_meta` during the render
4. Read `dist/index.html` (Trunk output) as HTML shell — content-hashed asset tags (`<script>`, `<link rel="stylesheet">`) are preserved verbatim
5. Replace everything between `<body` `>` and `</body>` with SSR body HTML (preserve `<body>` tag attributes)
6. Append collected meta tags into `<head>` before `</head>`
7. Write to `dist/<route>/index.html` (create directory as needed)

Per-route render failures are logged and cause a non-zero exit. No `catch_unwind` — see profile note below.

### Routes rendered

**Static routes** (10):
`/`, `/about`, `/writing`, `/resume`, `/contact`, `/one-pager`

**Dynamic routes — enumerated from static data** (21):
- `/writing/<slug>` × 17 (from `WRITEUPS` `LazyLock`)
- `/project/<slug>` × 4 (from `PROJECTS` `LazyLock`)

**Routes skipped:**
- `/telemetry` — entirely browser-only, falls back to SPA `404.html`
- `/project/:slug/docs` — thin wrapper, no static content
- `/project/:slug/demo` — placeholder pages
- `/*any` (404 page) — not pre-rendered

Total: 31 HTML files written to `dist/`.

### CLI interface

```
ssr_gen --dist <path>   # path to Trunk dist/ output, default: dist/
```

---

## Section 4: CI Pipeline

The release profile has `panic = "abort"` (optimized for WASM). The `ssr_gen` binary is a native build-time tool that must be able to handle render errors gracefully. Add a custom profile to `Cargo.toml`:

```toml
[profile.ssr-gen]
inherits = "release"
panic = "unwind"
strip = false   # native binary, debug info useful in CI logs
```

Additions to `.github/workflows/deploy.yml` after `trunk build --release`:

```yaml
- name: Build SSR generator
  run: cargo build --profile ssr-gen --features ssr --bin ssr_gen

- name: Generate static HTML
  run: ./target/ssr-gen/ssr_gen --dist dist/
```

Cargo build cache keyed on `Cargo.lock` covers both Trunk and `ssr_gen` builds — no separate cache step needed.

`dist/404.html` (SPA fallback, copied from `index.html` in existing CI) remains unchanged. Covers `/telemetry` and any dynamic slugs not enumerated.

`JamesIves/github-pages-deploy-action` step is unchanged.

---

## Risk Register

| Risk | Mitigation |
|---|---|
| Hydration ID mismatch (component tree differs between SSR and hydrate) | Hard rule: `#[cfg]` only on imperative calls, never view structure. Verify with Leptos's hydration debug output in dev. |
| `#[cfg]` import gate missed, SSR binary fails to compile | The WASM-only dep split makes this a compile error, not a runtime error. CI catches it. |
| Resource re-fetch flash on hydration (Loading... → content) | Expected. Acceptable for detail pages. Mitigable in Phase 3 (SQLite) or a future content-server phase. |
| `ssr_gen` panics on a route, silently skips HTML | `catch_unwind` per route + non-zero exit code + CI failure. |
| Trunk content-hash changes between builds invalidate ssr_gen HTML | `ssr_gen` always reads the current `dist/index.html` — it uses whatever Trunk just wrote. |
| `panic = "abort"` in release profile aborts ssr_gen on route panic | Custom `[profile.ssr-gen]` inherits release but sets `panic = "unwind"`. `ssr_gen` is built with `--profile ssr-gen`. |

---

## Out of Scope for This Phase

- Server-side data loading for resources (Suspense shows "Loading..." on detail pages — Phase 3 SQLite addresses this)
- `leptos_axum` integration
- Streaming SSR
- Per-route build caching
