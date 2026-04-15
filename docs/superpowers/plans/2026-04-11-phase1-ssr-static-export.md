# Phase 1: Leptos SSR with Static Export — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate from Leptos CSR to true SSR with client-side hydration, pre-rendering all static and enumerable dynamic routes to HTML at CI build time.

**Architecture:** Add `hydrate`/`ssr` feature flags; move WASM-only deps behind `target_arch = "wasm32"` gating; gate all inline browser API references with `#[cfg(not(feature = "ssr"))]`; add a native `ssr_gen` binary that renders each route to HTML using Trunk's output as the shell; extend CI with two steps after `trunk build`.

**Tech Stack:** Rust 1.x, Leptos 0.6 (`hydrate` + `ssr` features), `leptos_router::RouterIntegrationContext` + `ServerIntegration` for SSR routing, Trunk (WASM build, unchanged), GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-04-11-ssr-static-export-design.md`

---

## File Map

| Action | Path | Responsibility |
|---|---|---|
| **Create** | `src/lib.rs` | `App` component, all `pub mod` declarations — the SSR-compilable library root |
| **Create** | `src/bin/ssr_gen.rs` | Native binary: render each route to HTML, inject into Trunk shell |
| **Modify** | `Cargo.toml` | Feature flags, WASM-only dep split, `[lib]` target, `[profile.ssr-gen]` |
| **Modify** | `index.html` | Trunk `rel="rust"` directive: `csr` → `hydrate` |
| **Modify** | `src/main.rs` | Thin WASM entry point only; imports App from lib |
| **Modify** | `src/utils.rs` | Gate `web_sys`/`js_sys`/`wasm_bindgen` imports and all WASM-only function bodies |
| **Modify** | `src/lib.rs` | Gate `create_effect` body + `ErrorBoundary` `on:click` in `App` |
| **Modify** | `src/components/nav.rs` | Gate all inline `web_sys`/`gloo_timers` references |
| **Modify** | `src/components/layout.rs` | Gate `web_sys`/`wasm_bindgen::closure::Closure` references |
| **Modify** | `src/components/palette.rs` | Gate `web_sys` reference |
| **Modify** | `src/pages/home.rs` | Gate `web_sys`/`js_sys`/`gloo_timers` references |
| **Modify** | `src/pages/resume.rs` | Gate `js_sys` reference |
| **Modify** | `src/pages/one_pager.rs` | Gate `js_sys` reference |
| **Modify** | `src/pages/telemetry.rs` | Gate all WASM-only imports (entire page is browser-only) |
| **Modify** | `src/pages/writing.rs` | Dual `create_resource` implementation |
| **Modify** | `src/pages/project.rs` | Dual `create_resource` × 2, add `<Suspense>` to `ProjectDocsPage` |
| **Modify** | `.github/workflows/deploy.yml` | Add ssr_gen build + run; reorder 404.html copy before ssr_gen |

---

## Task 1: Restructure Cargo.toml

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Replace `Cargo.toml` with the feature-split layout**

Replace the entire file content:

```toml
[package]
name = "richardmussell"
version = "0.1.0"
edition = "2021"

[lib]
name = "richardmussell"
path = "src/lib.rs"

[features]
hydrate = ["leptos/hydrate", "leptos_meta/hydrate", "leptos_router/hydrate"]
ssr     = ["leptos/ssr",     "leptos_meta/ssr",     "leptos_router/ssr"]

[dependencies]
leptos        = { version = "0.6" }
leptos_meta   = { version = "0.6" }
leptos_router = { version = "0.6" }
serde         = { version = "1", features = ["derive"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys     = { version = "0.3", features = [
    "Clipboard", "Document", "Element", "History",
    "HtmlIFrameElement", "HtmlInputElement",
    "IntersectionObserver", "IntersectionObserverEntry", "IntersectionObserverInit",
    "MediaQueryList", "Navigator", "Window"
] }
js-sys      = "0.3"
gloo-net    = { version = "0.5", features = ["http"] }
gloo-timers = { version = "0.3", features = ["futures"] }
getrandom   = { version = "0.2", features = ["js"] }

[profile.release]
opt-level        = "z"
lto              = "fat"
codegen-units    = 1
panic            = "abort"
strip            = true
overflow-checks  = false

[profile.ssr-gen]
inherits = "release"
panic    = "unwind"
strip    = false
```

- [ ] **Step 2: Verify Cargo.toml parses**

```bash
cargo metadata --no-deps --format-version 1 > /dev/null && echo "OK"
```

Expected: `OK` with no errors.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "build: restructure Cargo.toml for hydrate/ssr feature split and WASM-only deps"
```

---

## Task 2: Create `src/lib.rs` — App component and module exports

**Files:**
- Create: `src/lib.rs`

Moves `fn App()` and all `pub mod` declarations out of `src/main.rs` into the library crate root that both the WASM binary and `ssr_gen` can import from.

- [ ] **Step 1: Create `src/lib.rs`**

```rust
pub mod data;
pub mod utils;
pub mod components;
pub mod pages;

use crate::data::{ProjectCardSignals, ReadProgressSignals};
use crate::components::{BackToTop, CommandPalette, KeyboardNav, NavBar, ReadingProgress};
use crate::pages::{
    AboutPage, ContactPage, HomePage, NotFoundPage, OnePageSummary,
    ProjectDemoPage, ProjectDetailPage, ProjectDocsPage, ResumePage,
    TelemetryPage, WriteupDetailPage, WritingPage,
};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_dark, set_is_dark) = create_signal(true);
    provide_context(is_dark);

    let shortcuts_open = create_rw_signal(false);
    provide_context(shortcuts_open);
    let palette_open = create_rw_signal(false);
    provide_context(palette_open);

    let (read_progress, set_read_progress) = create_signal(0.0_f64);
    provide_context(ReadProgressSignals { progress: read_progress, set_progress: set_read_progress });

    let (expanded_slug, set_expanded_slug) = create_signal(None::<String>);
    let did_drag = create_rw_signal(false);
    provide_context(ProjectCardSignals { expanded_slug, set_expanded_slug, did_drag });

    // Dark/light mode — browser only. web_sys is a WASM-only dep; the cfg guard
    // prevents a compile error on the native SSR target even though create_effect
    // never runs during SSR rendering.
    create_effect(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            let Some(window) = web_sys::window() else { return };
            let Some(document) = window.document() else { return };
            let Some(html) = document.document_element() else { return };
            let Some(body) = document.body() else { return };
            if is_dark.get() {
                html.class_list().add_1("dark").ok();
                html.class_list().remove_1("light").ok();
                body.style().set_property("background-color", "#080d14").ok();
            } else {
                html.class_list().add_1("light").ok();
                html.class_list().remove_1("dark").ok();
                body.style().set_property("background-color", "#f0f4f8").ok();
            }
        }
    });

    view! {
        <a href="#main-content" class="skip-to-content" tabindex="0">"Skip to main content"</a>
        <Router>
            <KeyboardNav />
            <NavBar is_dark set_is_dark />
            <CommandPalette />
            <ErrorBoundary fallback=|_errors| {
                view! {
                    <main class="min-h-screen pt-28 flex flex-col items-center justify-center font-mono text-center px-4">
                        <p class="text-[96px] text-[var(--text-muted)] mb-4">"error[E0308]"</p>
                        <p class="text-[14px] text-[var(--text-secondary)] mb-2">"runtime error in component tree"</p>
                        <p class="text-[13px] text-[var(--text-muted)] mb-6">"expected: rendered view"</p>
                        <p class="text-[13px] text-[var(--text-muted)] mb-8">"found: panic"</p>
                        <div class="flex gap-4">
                            <button type="button" class="hero-btn" on:click=move |_| {
                                // window.history().back() — WASM-only dep guard required
                                #[cfg(not(feature = "ssr"))]
                                if let Some(w) = web_sys::window() {
                                    if let Ok(h) = w.history() { let _ = h.back(); }
                                }
                            }>"← Back"</button>
                            <a href="/" class="hero-btn">"→ Home"</a>
                        </div>
                    </main>
                }
            }>
            <Routes>
                <Route path="/"                    view=HomePage />
                <Route path="/about"               view=AboutPage />
                <Route path="/writing"             view=WritingPage />
                <Route path="/writing/:slug"       view=WriteupDetailPage />
                <Route path="/project/:slug"       view=ProjectDetailPage />
                <Route path="/project/:slug/docs"  view=ProjectDocsPage />
                <Route path="/project/:slug/demo"  view=ProjectDemoPage />
                <Route path="/resume"              view=ResumePage />
                <Route path="/contact"             view=ContactPage />
                <Route path="/telemetry"           view=TelemetryPage />
                <Route path="/one-pager"           view=OnePageSummary />
                <Route path="/*any"                view=NotFoundPage />
            </Routes>
            </ErrorBoundary>
            <ReadingProgress />
            <BackToTop />
            <Show when=move || shortcuts_open.get() fallback=|| ()>
                <div
                    class="shortcuts-scrim"
                    role="presentation"
                    on:click=move |_| shortcuts_open.set(false)
                >
                    <div
                        class="shortcuts-modal"
                        role="dialog"
                        aria-modal="true"
                        id="keyboard-shortcuts-modal"
                        tabindex="-1"
                        aria-label="Keyboard shortcuts"
                        on:click=move |ev| ev.stop_propagation()
                    >
                        <h2 class="shortcuts-modal-title">"Keyboard shortcuts"</h2>
                        <table class="shortcuts-table">
                            <tbody>
                                <tr><td class="shortcuts-key">"g then h"</td><td class="shortcuts-desc">"Go to Home"</td></tr>
                                <tr><td class="shortcuts-key">"g then p"</td><td class="shortcuts-desc">"Go to first project"</td></tr>
                                <tr><td class="shortcuts-key">"g then r"</td><td class="shortcuts-desc">"Go to Resume"</td></tr>
                                <tr><td class="shortcuts-key">"/"</td><td class="shortcuts-desc">"Focus terminal search"</td></tr>
                                <tr><td class="shortcuts-key">"Esc"</td><td class="shortcuts-desc">"Close / blur"</td></tr>
                                <tr><td class="shortcuts-key">"?"</td><td class="shortcuts-desc">"Toggle this help modal"</td></tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </Show>
        </Router>
    }
}
```

- [ ] **Step 2: Confirm lib.rs exists and Cargo recognizes it**

```bash
cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; d=json.load(sys.stdin); [print(t['kind'],t['src_path']) for p in d['packages'] for t in p['targets']]"
```

Expected: Lines for `lib` pointing to `src/lib.rs` and `bin` pointing to `src/main.rs` and `src/bin/ssr_gen.rs`.

---

## Task 3: Slim `src/main.rs` to WASM entry point only

**Files:**
- Modify: `src/main.rs`

All module declarations and `App` move to `src/lib.rs`. `src/main.rs` becomes the thin WASM binary entry point, always compiled with `--features hydrate`. `web_sys` is always available here — no `#[cfg]` guards needed in this file.

- [ ] **Step 1: Replace `src/main.rs` entirely**

```rust
//! WASM entry point. Compiled with `--features hydrate` by Trunk.
//! All app logic lives in src/lib.rs.

use leptos::*;
use richardmussell::App;

fn main() {
    richardmussell::utils::capture_wasm_start_time();

    std::panic::set_hook(Box::new(|info| {
        let msg = format!("WASM panic: {}", info);
        let _ = (|| -> Option<()> {
            let doc = web_sys::window()?.document()?;
            let body = doc.body()?;
            let div = doc.create_element("div").ok()?;
            div.set_attribute(
                "style",
                "position:fixed;inset:0;background:#7f1d1d;color:#fca5a5;\
                 font-family:monospace;font-size:13px;padding:32px;\
                 z-index:999999;overflow:auto;white-space:pre-wrap;",
            ).ok()?;
            div.set_text_content(Some(&msg));
            body.prepend_with_node_1(&div).ok()
        })();
        web_sys::console::error_1(&msg.into());
    }));

    mount_to_body(|| view! { <App /> });

    let _ = (|| -> Option<()> {
        web_sys::window()?.document()?.get_element_by_id("wasm-init-indicator")?.remove();
        Some(())
    })();
}
```

- [ ] **Step 2: Commit the lib + main restructure**

```bash
git add src/lib.rs src/main.rs
git commit -m "refactor: split App into src/lib.rs; slim main.rs to WASM entry point"
```

---

## Task 4: Update Trunk directive in `index.html`

**Files:**
- Modify: `index.html` line 51

- [ ] **Step 1: Change the Trunk `rel="rust"` directive**

Line 51 currently reads:
```html
<link data-trunk rel="rust" data-wasm-opt="0" />
```

Change to:
```html
<link data-trunk rel="rust" data-wasm-opt="0" data-cargo-features="hydrate" />
```

- [ ] **Step 2: Commit**

```bash
git add index.html
git commit -m "build: switch Trunk directive from csr to hydrate feature"
```

---

## Task 5: Gate `src/utils.rs`

**Files:**
- Modify: `src/utils.rs`

`utils.rs` imports `leptos::wasm_bindgen`, `js_sys` (for analytics tracking), and uses `web_sys::window()` in `perf_now_ms`. All three are WASM-only. The `WASM_START_TIME_MS` `OnceLock` and its accessors also need gating.

- [ ] **Step 1: Replace the top of `src/utils.rs`** (everything through `wasm_start_time_ms`, lines 1–28)

```rust
//! Sanitization, tracking, and code highlighting utilities.

#[cfg(debug_assertions)]
use leptos::logging::log;

pub fn sanitize_slug(slug: &str) -> String {
    let out: String = slug.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .take(100)
        .collect();
    #[cfg(debug_assertions)]
    if out != slug {
        log!("sanitize_slug: {:?} -> {:?}", slug, out);
    }
    out
}

// ── WASM-only: performance timing ────────────────────────────────────────────

#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::{JsCast, JsValue};
#[cfg(not(feature = "ssr"))]
use std::sync::OnceLock;

#[cfg(not(feature = "ssr"))]
static WASM_START_TIME_MS: OnceLock<f64> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
fn perf_now_ms() -> Option<f64> {
    let window = web_sys::window()?;
    let perf = js_sys::Reflect::get(&window, &JsValue::from_str("performance")).ok()?;
    let now_fn = js_sys::Reflect::get(&perf, &JsValue::from_str("now")).ok()?;
    let now_fn = now_fn.dyn_into::<js_sys::Function>().ok()?;
    now_fn.call0(&perf).ok()?.as_f64()
}

pub fn capture_wasm_start_time() {
    #[cfg(not(feature = "ssr"))]
    if let Some(now) = perf_now_ms() {
        let _ = WASM_START_TIME_MS.set(now);
    }
}

pub fn wasm_start_time_ms() -> Option<f64> {
    #[cfg(not(feature = "ssr"))]
    return WASM_START_TIME_MS.get().copied();
    #[cfg(feature = "ssr")]
    None
}
```

- [ ] **Step 2: Gate the `track` function body** (currently around line 42–49)

The `track` function uses `js_sys` for the analytics event dispatch (release-only). Wrap the release-mode body:

```rust
#[allow(unused_variables)]
pub fn track(event: &str, props: &str) {
    // Analytics dispatch: WASM-only, release-only.
    // The existing js_sys call is gated by both cfg flags.
    #[cfg(all(not(debug_assertions), not(feature = "ssr")))]
    {
        // existing js_sys dispatch body — unchanged, just wrapped in the cfg block
        let _ = js_sys::eval(&format!(
            "window.dispatchEvent(new CustomEvent('portfolio:{}', {{ detail: JSON.parse({:?}) }}))",
            event, props
        ));
    }
}
```

Note: `js_sys` resolves because the outer `#[cfg(not(feature = "ssr"))]` wrapping the import at the top means `js_sys` is in scope only on WASM. Since this body is also gated `#[cfg(not(feature = "ssr"))]`, it won't compile on the SSR target.

- [ ] **Step 3: Verify**

```bash
cargo check --features ssr 2>&1 | grep "utils" | head -10
```

Expected: no errors from `utils.rs`.

- [ ] **Step 4: Commit**

```bash
git add src/utils.rs
git commit -m "fix(ssr): gate WASM-only APIs in utils.rs"
```

---

## Task 6: Gate `src/components/layout.rs`

**Files:**
- Modify: `src/components/layout.rs`

`ReadingProgress` uses `web_sys::window()` and `wasm_bindgen::closure::Closure` inside a `create_effect`. The effect won't run in SSR, but the types must compile on native.

- [ ] **Step 1: Read the current file**

```bash
cat -n src/components/layout.rs
```

- [ ] **Step 2: Gate WASM-only imports at the top**

Add before any existing imports that reference WASM-only types:

```rust
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::closure::Closure;
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::JsCast;
```

If `web_sys` is used via full path (`web_sys::window()`) rather than an import, no additional import gating is needed — the type simply won't resolve on native, and the `#[cfg]` on the call site (Step 3) prevents compilation.

- [ ] **Step 3: Gate the `create_effect` body in `ReadingProgress`**

Wrap the entire body of the scroll listener `create_effect` (lines 12–26 per inventory):

```rust
create_effect(move |_| {
    #[cfg(not(feature = "ssr"))]
    {
        let Some(win) = web_sys::window() else { return };
        // ... all existing scroll listener + Closure::forget() code — unchanged ...
    }
});
```

- [ ] **Step 4: Verify**

```bash
cargo check --features ssr 2>&1 | grep "layout" | head -10
```

Expected: no errors from `layout.rs`.

- [ ] **Step 5: Commit**

```bash
git add src/components/layout.rs
git commit -m "fix(ssr): gate WASM-only APIs in components/layout.rs"
```

---

## Task 7: Gate `src/components/palette.rs`

**Files:**
- Modify: `src/components/palette.rs`

`CommandPalette` auto-focuses its input via `web_sys::window()` / `document.get_element_by_id()` inside a `create_effect` (inventory §7, lines 106–112).

- [ ] **Step 1: Read the current file**

```bash
cat -n src/components/palette.rs
```

- [ ] **Step 2: Gate the auto-focus `create_effect` body**

Wrap all `web_sys` and `JsCast` usage inside the effect with `#[cfg(not(feature = "ssr"))]`:

```rust
create_effect(move |_| {
    if palette_open.get() {
        #[cfg(not(feature = "ssr"))]
        {
            use leptos::wasm_bindgen::JsCast;
            // existing: gloo_timers Timeout::new(0, || { document.get_element_by_id... })
            // wrap entire Timeout block — unchanged
        }
    }
});
```

If `gloo_timers` is imported at the top of the file, gate that import too:
```rust
#[cfg(not(feature = "ssr"))]
use gloo_timers::callback::Timeout;
```

- [ ] **Step 3: Verify**

```bash
cargo check --features ssr 2>&1 | grep "palette" | head -10
```

Expected: no errors from `palette.rs`.

- [ ] **Step 4: Commit**

```bash
git add src/components/palette.rs
git commit -m "fix(ssr): gate WASM-only APIs in components/palette.rs"
```

---

## Task 8: Gate `src/components/nav.rs`

**Files:**
- Modify: `src/components/nav.rs`

The largest gating task. `nav.rs` contains: `BackToTop` scroll listener, `KeyboardNav` global keydown listener, `window.scroll_to_with_x_and_y`, `document.active_element`, `document.get_element_by_id`, `gloo_timers::callback::Timeout`, chord-handling state.

- [ ] **Step 1: Read the current file in full**

```bash
cat -n src/components/nav.rs
```

- [ ] **Step 2: Gate WASM-only imports at the top**

```rust
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::closure::Closure;
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use gloo_timers::callback::Timeout;
```

Leave `use leptos::*`, `use leptos_router::*`, `use std::*` ungated.

- [ ] **Step 3: Gate `BackToTop` scroll listener `create_effect` body** (inventory §7, lines 169–177)

```rust
create_effect(move |_| {
    #[cfg(not(feature = "ssr"))]
    {
        let Some(win) = web_sys::window() else { return };
        // ... existing scroll listener + Closure::forget() — unchanged ...
    }
});
```

Also gate the `on:click` scroll-to-top handler:
```rust
on:click=move |_| {
    #[cfg(not(feature = "ssr"))]
    if let Some(win) = web_sys::window() {
        win.scroll_to_with_x_and_y(0.0, 0.0);
    }
}
```

- [ ] **Step 4: Gate `KeyboardNav` global keydown `create_effect` body** (inventory §7, lines 215–329)

Wrap the entire effect body (everything that touches `web_sys::window()`, `document.active_element()`, `Timeout::new()`, `Closure::forget()`):

```rust
create_effect(move |_| {
    #[cfg(not(feature = "ssr"))]
    {
        let Some(win) = web_sys::window() else { return };
        // ... all existing chord-handling + global listener + Closure::forget() — unchanged ...
    }
});
```

- [ ] **Step 5: Gate `NavBar` route-change effect**

Read the effect at `src/components/nav.rs:19`. If it only calls `set_nav_open.set(false)` (a pure Leptos signal write), no gating is needed — signal writes compile everywhere. If it calls `web_sys` APIs, wrap those lines with `#[cfg(not(feature = "ssr"))]`.

- [ ] **Step 6: Verify**

```bash
cargo check --features ssr 2>&1 | grep "nav" | head -20
```

Expected: no errors from `nav.rs`.

- [ ] **Step 7: Commit**

```bash
git add src/components/nav.rs
git commit -m "fix(ssr): gate WASM-only APIs in components/nav.rs"
```

---

## Task 9: Gate `src/pages/telemetry.rs`

**Files:**
- Modify: `src/pages/telemetry.rs`

The telemetry page is skipped by `ssr_gen` but must compile on native since all source files in the library are compiled. Gate every WASM-only import and the entire component body.

- [ ] **Step 1: Read the current file**

```bash
cat -n src/pages/telemetry.rs
```

- [ ] **Step 2: Gate WASM-only imports at the top**

```rust
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::{JsCast, JsValue};
#[cfg(not(feature = "ssr"))]
use gloo_net::http::Request;
#[cfg(not(feature = "ssr"))]
use gloo_timers::callback::Interval;
```

- [ ] **Step 3: Gate all private helper functions**

The local `perf_now_ms`, `heap_bytes`, `ttfb_ms`, `probe_network` functions (lines 10–98) all use WASM-only types. Gate each entire function:

```rust
#[cfg(not(feature = "ssr"))]
fn perf_now_ms() -> Option<f64> { /* unchanged */ }

#[cfg(not(feature = "ssr"))]
fn heap_bytes() -> Option<(f64, f64)> { /* unchanged */ }

#[cfg(not(feature = "ssr"))]
fn ttfb_ms() -> Option<f64> { /* unchanged */ }

#[cfg(not(feature = "ssr"))]
async fn probe_network() -> Vec<String> { /* unchanged */ }
```

- [ ] **Step 4: Gate the `TelemetryPage` component internals**

The `#[component]` and its signature must remain (it's registered in `App`'s route list). Wrap only the body:

```rust
#[component]
pub fn TelemetryPage() -> impl IntoView {
    #[cfg(not(feature = "ssr"))]
    {
        // entire existing function body — signals, intervals, effects, view! — unchanged
    }
    #[cfg(feature = "ssr")]
    view! { <></> }
}
```

- [ ] **Step 5: Verify**

```bash
cargo check --features ssr 2>&1 | grep "telemetry" | head -10
```

Expected: no errors from `telemetry.rs`.

- [ ] **Step 6: Commit**

```bash
git add src/pages/telemetry.rs
git commit -m "fix(ssr): gate entire browser-only body in pages/telemetry.rs"
```

---

## Task 10: Gate `src/pages/home.rs`, `resume.rs`, `one_pager.rs`

**Files:**
- Modify: `src/pages/home.rs`
- Modify: `src/pages/resume.rs`
- Modify: `src/pages/one_pager.rs`

These pages use `js_sys` (clipboard, analytics), `web_sys::window()` (terminal focus, `window.print()`), and `gloo_timers` (email-copy reset timeout). All inside event handlers and effects — safe from running, but the types must compile.

- [ ] **Step 1: Read each file**

```bash
cat -n src/pages/home.rs | head -80
cat -n src/pages/resume.rs
cat -n src/pages/one_pager.rs | head -60
```

- [ ] **Step 2: Gate WASM-only clipboard calls in all three files**

The clipboard pattern appears 7 times across the codebase (inventory §7, `js_sys` usage table). In each `on:click` handler that calls the clipboard or dispatches a custom event, wrap the body:

```rust
on:click=move |_| {
    // Clipboard write — WASM-only dep guard
    #[cfg(not(feature = "ssr"))]
    {
        // existing js_sys clipboard call + Timeout/set_timeout for reset — unchanged
    }
}
```

- [ ] **Step 3: Gate `window.print()` calls in `resume.rs` and `one_pager.rs`**

```rust
on:click=move |_| {
    #[cfg(not(feature = "ssr"))]
    if let Some(w) = web_sys::window() { let _ = w.print(); }
}
```

- [ ] **Step 4: Gate terminal auto-focus in `home.rs`**

The terminal auto-focus `create_effect` (lines 47–58) calls `web_sys::window()` and `document.get_element_by_id("terminal-input")`. Wrap the `web_sys` portion:

```rust
create_effect(move |_| {
    // Boot animation set_timeout calls (leptos::set_timeout) are safe — keep as-is
    // web_sys focus call needs gating:
    #[cfg(not(feature = "ssr"))]
    leptos::set_timeout(move || {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            // existing focus logic — unchanged
        }
    }, std::time::Duration::from_millis(120));
});
```

- [ ] **Step 5: Verify all three files**

```bash
cargo check --features ssr 2>&1 | grep -E "(home|resume|one_pager)" | head -20
```

Expected: no errors from these files.

- [ ] **Step 6: Commit**

```bash
git add src/pages/home.rs src/pages/resume.rs src/pages/one_pager.rs
git commit -m "fix(ssr): gate WASM-only APIs in home, resume, one_pager pages"
```

---

## Task 11: Dual `create_resource` in `writing.rs` and `project.rs`

**Files:**
- Modify: `src/pages/writing.rs`
- Modify: `src/pages/project.rs`

`gloo_net::http::Request` is WASM-only. Resources that use it need a real fetch on WASM and a `None`-returning stub on SSR. Suspense boundaries render `"Loading..."` in static HTML; real content loads post-hydration.

- [ ] **Step 1: Replace `create_resource` in `WriteupDetailPage` (`src/pages/writing.rs:180–195`)**

Replace the existing `let detail = create_resource(...)` block with:

```rust
#[cfg(not(feature = "ssr"))]
let detail = create_resource(
    slug,
    |s| async move {
        if s.is_empty() { return None; }
        let url = format!("/writeups/{}.json", s);
        gloo_net::http::Request::get(&url)
            .send().await.ok()?
            .json::<WriteUpDetail>().await.ok()
    },
);
#[cfg(feature = "ssr")]
let detail = create_resource(move || slug(), |_| async move { None::<WriteUpDetail> });
```

- [ ] **Step 2: Replace `create_resource` in `ProjectDetailPage` (`src/pages/project.rs:12–27`)**

```rust
#[cfg(not(feature = "ssr"))]
let detail = create_resource(
    slug,
    |s| async move {
        if s.is_empty() { return None; }
        let url = format!("/projects/{}.json", s);
        gloo_net::http::Request::get(&url)
            .send().await.ok()?
            .json::<crate::data::ProjectDetail>().await.ok()
    },
);
#[cfg(feature = "ssr")]
let detail = create_resource(move || slug(), |_| async move { None::<crate::data::ProjectDetail> });
```

- [ ] **Step 3: Replace `create_resource` in `ProjectDocsPage` (`src/pages/project.rs:161–176`)**

```rust
#[cfg(not(feature = "ssr"))]
let docs = create_resource(
    slug,
    |s| async move {
        if s.is_empty() { return None; }
        let url = format!("/docs/{}.json", s);
        gloo_net::http::Request::get(&url)
            .send().await.ok()?
            .json::<crate::data::ProjectDetail>().await.ok()
    },
);
#[cfg(feature = "ssr")]
let docs = create_resource(move || slug(), |_| async move { None::<crate::data::ProjectDetail> });
```

- [ ] **Step 4: Add `<Suspense>` wrapper to `ProjectDocsPage`**

`ProjectDocsPage` has no `<Suspense>` around `docs` usage (inventory §8: "ProjectDocsPage does not use Suspense"). Reading a resource outside a Suspense boundary in hydration mode can panic. Wrap the docs-reading portion:

```rust
view! {
    {move || match project.get() {
        None => view! { /* 404 view unchanged */ }.into_view(),
        Some(p) => view! {
            <Title text=format!("Docs: {} | Richard Mussell", p.title)/>
            <main id="main-content" class="min-h-screen pt-16 pb-24">
                // nav breadcrumb and header unchanged above Suspense
                <Suspense fallback=|| view! {
                    <p class="font-mono text-[var(--text-muted)] p-8">"Loading..."</p>
                }>
                    {move || docs.get().flatten().map(|d| view! {
                        // existing docs content view — unchanged, just moved inside Suspense
                    })}
                </Suspense>
            </main>
        }.into_view()
    }}
}
```

Read `src/pages/project.rs:178` onwards carefully before making this edit to preserve all existing markup.

- [ ] **Step 5: Verify**

```bash
cargo check --features ssr 2>&1 | grep -E "(writing|project)" | head -20
```

Expected: no errors.

- [ ] **Step 6: Commit**

```bash
git add src/pages/writing.rs src/pages/project.rs
git commit -m "fix(ssr): dual create_resource for gloo_net fetches; add Suspense to ProjectDocsPage"
```

---

## Task 12: Full compile verification

**Files:** None (verification only)

- [ ] **Step 1: Verify SSR feature compiles cleanly**

```bash
cargo check --features ssr 2>&1
```

Expected: `Finished` with 0 errors. Unused-import warnings under SSR are acceptable.

- [ ] **Step 2: Verify WASM hydrate feature compiles**

```bash
cargo check --features hydrate --target wasm32-unknown-unknown 2>&1
```

Expected: `Finished` with 0 errors.

- [ ] **Step 3: Full Trunk build**

```bash
trunk build 2>&1 | tail -5
```

Expected: completes successfully, `dist/` contains WASM and JS assets.

- [ ] **Step 4: Commit any fixes**

```bash
git add -p
git commit -m "fix(ssr): resolve remaining compile errors from full SSR/hydrate verification"
```

Skip this step if Steps 1–3 passed cleanly.

---

## Task 13: Write `src/bin/ssr_gen.rs`

**Files:**
- Create: `src/bin/ssr_gen.rs`

Native binary compiled with `--profile ssr-gen --features ssr`. Reads Trunk's `dist/index.html` as HTML shell, renders each route via Leptos SSR, writes pre-rendered HTML to `dist/<route>/index.html`.

- [ ] **Step 1: Create `src/bin/ssr_gen.rs`**

```rust
//! Static HTML generator — Phase 1 SSR.
//!
//! Build:  cargo build --profile ssr-gen --features ssr --bin ssr_gen
//! Run:    ./target/ssr-gen/ssr_gen [--dist <path>]

use leptos::*;
use leptos_router::{RouterIntegrationContext, ServerIntegration};
use richardmussell::App;
use richardmussell::data::{WRITEUPS, PROJECTS};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dist_str = args.windows(2)
        .find(|w| w[0] == "--dist")
        .map(|w| w[1].clone())
        .unwrap_or_else(|| "dist".to_string());
    let dist = PathBuf::from(&dist_str);

    let shell = fs::read_to_string(dist.join("index.html"))
        .unwrap_or_else(|e| {
            panic!("Cannot read {}/index.html: {}. Run `trunk build` first.", dist_str, e)
        });

    let mut routes: Vec<String> = vec![
        "/".into(), "/about".into(), "/writing".into(),
        "/resume".into(), "/contact".into(), "/one-pager".into(),
    ];
    for w in WRITEUPS.iter() {
        routes.push(format!("/writing/{}", w.slug));
    }
    for p in PROJECTS.iter() {
        routes.push(format!("/project/{}", p.slug));
    }

    let total = routes.len();
    let mut errors = 0;

    for route in &routes {
        match render_route(route, &shell) {
            Ok(html) => {
                let out_dir = route_to_dir(&dist, route);
                if let Err(e) = fs::create_dir_all(&out_dir) {
                    eprintln!("FAIL  {} — could not create dir: {}", route, e);
                    errors += 1;
                    continue;
                }
                if let Err(e) = fs::write(out_dir.join("index.html"), html) {
                    eprintln!("FAIL  {} — could not write index.html: {}", route, e);
                    errors += 1;
                    continue;
                }
                println!("OK    {}", route);
            }
            Err(e) => {
                eprintln!("FAIL  {} — render error: {}", route, e);
                errors += 1;
            }
        }
    }

    println!("\n{}/{} routes rendered successfully.", total - errors, total);
    if errors > 0 {
        eprintln!("{} route(s) failed.", errors);
        std::process::exit(1);
    }
}

fn render_route(route: &str, shell: &str) -> Result<String, String> {
    let route = route.to_string();
    let body_html = leptos::ssr::render_to_string(move || {
        provide_context(RouterIntegrationContext::new(ServerIntegration {
            path: route.clone(),
        }));
        view! { <App /> }
    });
    inject_into_shell(shell, &body_html)
}

fn inject_into_shell(shell: &str, body_html: &str) -> Result<String, String> {
    let body_open = shell.find("<body")
        .ok_or_else(|| "shell missing <body".to_string())?;
    let body_tag_end = shell[body_open..]
        .find('>')
        .ok_or_else(|| "<body has no closing >".to_string())?
        + body_open + 1;
    let body_close = shell.rfind("</body>")
        .ok_or_else(|| "shell missing </body>".to_string())?;

    Ok(format!("{}\n{}\n{}", &shell[..body_tag_end], body_html, &shell[body_close..]))
}

fn route_to_dir(dist: &Path, route: &str) -> PathBuf {
    if route == "/" {
        dist.to_path_buf()
    } else {
        dist.join(&route[1..])
    }
}
```

- [ ] **Step 2: Build the binary**

```bash
cargo build --profile ssr-gen --features ssr --bin ssr_gen 2>&1
```

Expected: `Finished` with 0 errors. Binary at `target/ssr-gen/ssr_gen`.

- [ ] **Step 3: Run ssr_gen against a fresh Trunk output**

```bash
trunk build 2>&1 | tail -3
./target/ssr-gen/ssr_gen --dist dist/ 2>&1
```

Expected:
```
OK    /
OK    /about
...
27/27 routes rendered successfully.
```

- [ ] **Step 4: Spot-check a pre-rendered file**

```bash
grep -c "data-hk" dist/about/index.html
head -30 dist/about/index.html
```

Expected: `data-hk` hydration markers present. Visible nav and page content in the HTML.

- [ ] **Step 5: Commit**

```bash
git add src/bin/ssr_gen.rs
git commit -m "feat: add ssr_gen static HTML generator binary"
```

---

## Task 14: Update CI pipeline

**Files:**
- Modify: `.github/workflows/deploy.yml`

Critical ordering: `404.html` copy must happen BEFORE `ssr_gen` runs. `ssr_gen` overwrites `dist/index.html` with the pre-rendered home page. The `404.html` must remain the clean SPA shell so GitHub Pages can serve it for unrecognized paths.

- [ ] **Step 1: Replace `.github/workflows/deploy.yml`**

```yaml
name: Deploy to GitHub Pages
on:
  push:
    branches: ["main"]
permissions:
  contents: write
concurrency:
  group: "pages"
  cancel-in-progress: true
jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Code
        uses: actions/checkout@v4
      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install Trunk
        run: |
          wget -qO- https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
          sudo mv trunk /usr/local/bin/
      - name: Build WASM with Trunk (Release)
        run: trunk build --release
      - name: Prepare static assets and SPA fallback
        run: |
          mkdir -p dist/fonts
          rm -rf dist/fonts && mkdir -p dist/fonts
          cp -v static/fonts/*.woff2 dist/fonts/
          test -f dist/fonts/inter-400.woff2
          test -f dist/fonts/inter-500.woff2
          test -f dist/fonts/jbmono-400.woff2
          # Copy SPA shell to 404.html BEFORE ssr_gen overwrites dist/index.html
          cp dist/index.html dist/404.html
          touch dist/.nojekyll
      - name: Build SSR generator
        run: cargo build --profile ssr-gen --features ssr --bin ssr_gen
      - name: Generate pre-rendered HTML
        run: ./target/ssr-gen/ssr_gen --dist dist/
      - name: Deploy
        uses: JamesIves/github-pages-deploy-action@v4
        with:
          folder: dist
          branch: gh-pages
          clean: true
          commit-message: "Deployment: Production build updated via CI/CD pipeline"
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/deploy.yml
git commit -m "ci: add ssr_gen pre-render step; reorder 404.html copy before ssr_gen"
```

---

## Task 15: End-to-end verification and INVENTORY.md update

**Files:**
- Modify: `INVENTORY.md` (sections 1, 2, 7)

- [ ] **Step 1: Full local build simulation**

```bash
trunk build --release 2>&1 | tail -5
mkdir -p dist/fonts && rm -rf dist/fonts && mkdir -p dist/fonts
cp static/fonts/*.woff2 dist/fonts/
cp dist/index.html dist/404.html
./target/ssr-gen/ssr_gen --dist dist/ 2>&1
echo "Exit: $?"
```

Expected: 31/31 routes, exit 0.

- [ ] **Step 2: Verify hydration markers in pre-rendered files**

```bash
for route in "" "about" "writing" "resume" "contact" "one-pager"; do
  file="dist/${route:+$route/}index.html"
  count=$(grep -c "data-hk" "$file" 2>/dev/null || echo 0)
  echo "$file: $count hydration markers"
done
```

Expected: each file shows > 0 markers.

- [ ] **Step 3: Verify 404.html is the clean SPA shell**

```bash
grep -c "data-hk" dist/404.html
```

Expected: `0`.

- [ ] **Step 4: Update INVENTORY.md**

Update the following sections to reflect the new state:

**Section 1 (Stack & Dependencies):** Add `[features]` table: `hydrate` / `ssr`. Note WASM-only deps moved to `[target.'cfg(target_arch = "wasm32")'.dependencies]`. Remove `csr` feature from leptos entries. Add `[profile.ssr-gen]` note.

**Section 2 (Build Pipeline):** Add: "Trunk directive now `data-cargo-features="hydrate"`. After Trunk build, CI runs `cargo build --profile ssr-gen --features ssr --bin ssr_gen` then `ssr_gen --dist dist/` to emit 31 pre-rendered HTML files." Add `src/bin/ssr_gen.rs` to binary targets.

**Section 7 (Browser API Surface):** Add note: "All entries in this section that reference `web_sys`, `js_sys`, `gloo_net`, or `gloo_timers` are gated with `#[cfg(not(feature = \"ssr\"))]` at import and/or call-site level. Effects and event handlers are gated at body level. `create_resource` fetchers have dual `#[cfg]` implementations."

- [ ] **Step 5: Push to revamp-origin**

```bash
git push revamp-origin revamp
```

Watch CI at `github.com/richmuscle/portfolio-revamp/actions`. All steps must be green.

- [ ] **Step 6: Final commit (INVENTORY.md + any last fixups)**

```bash
git add INVENTORY.md
git commit -m "docs: update INVENTORY.md for Phase 1 SSR migration — feature flags, build pipeline, gating notes"
```

---

## Completion Checklist

- [ ] `cargo check --features ssr` — 0 errors
- [ ] `cargo check --features hydrate --target wasm32-unknown-unknown` — 0 errors
- [ ] `trunk build --release` — completes successfully
- [ ] `ssr_gen` — 31/31 routes, exit 0
- [ ] Pre-rendered HTML files contain `data-hk` hydration markers
- [ ] `dist/404.html` — 0 `data-hk` markers (clean SPA shell)
- [ ] CI pipeline — all steps green
- [ ] `INVENTORY.md` — sections 1, 2, 7 updated
