# INVENTORY.md — Codebase Factual Inventory

Generated: 2026-04-11  
Commit: `118b657` (branch: `revamp`)

---

## 1. Stack & Dependencies

| Crate | Version (Cargo.toml) | Resolved (Cargo.lock) | Feature flags |
|---|---|---|---|
| leptos | 0.6 | 0.6.15 | `csr` |
| leptos_meta | 0.6 | 0.6.15 | `csr` |
| leptos_router | 0.6 | 0.6.15 | `csr` |
| serde | 1 | 1.x | `derive` |
| gloo-net | 0.5 | 0.5.0 | `http` |
| gloo-timers | 0.3 | 0.3.0 | `futures` |
| js-sys | 0.3 | 0.3.91 | — |
| web-sys | 0.3 | 0.3.91 | `Clipboard`, `Document`, `Element`, `History`, `HtmlIFrameElement`, `IntersectionObserver`, `IntersectionObserverEntry`, `IntersectionObserverInit`, `MediaQueryList`, `Navigator`, `Window` |
| getrandom | 0.2 | 0.2.17 | `js` |

**Rust edition:** 2021  
**No MSRV pinned.** No `rust-toolchain.toml`.  
**Compile target:** `wasm32-unknown-unknown` (CSR only, no SSR feature anywhere in the codebase)  
**wasm-bindgen resolved:** 0.2.114

### Release profile (`Cargo.toml`)

```toml
[profile.release]
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
overflow-checks = false
```

---

## 2. Build Pipeline

### Trunk (`Trunk.toml`)

```toml
[build]
target = "index.html"

[serve]
port = 8080
open = false

[[assets]]
source = "public"
```

### `index.html` Trunk directives

| Directive | Source | Destination |
|---|---|---|
| `rel="rust" data-wasm-opt="0"` | — | Compiles Rust to WASM, emits JS + WASM |
| `rel="css" href="style/style.css"` | `style/style.css` | Bundled CSS in `dist/` |
| `rel="copy-dir" href="static/fonts"` | `static/fonts/` | `dist/fonts/` |
| `rel="copy-dir" href="static/writeups"` | `static/writeups/` | `dist/writeups/` |
| `rel="copy-dir" href="static/projects"` | `static/projects/` | `dist/projects/` |
| `rel="copy-file" href="public/sitemap.xml"` | `public/sitemap.xml` | `dist/sitemap.xml` |
| `rel="copy-file" href="robots.txt"` | `robots.txt` | `dist/robots.txt` |
| `rel="copy-file" href="manifest.json"` | `manifest.json` | `dist/manifest.json` |
| `rel="copy-file" href="public/og-image.png"` | `public/og-image.png` | `dist/og-image.png` |
| `rel="copy-file" href="favicon.svg"` | `favicon.svg` | `dist/favicon.svg` |
| `rel="copy-file" href="favicon-32.png"` | `favicon-32.png` | `dist/favicon-32.png` |

**Note:** `static/docs/` is NOT listed as a Trunk copy directive. The `docs/*.json` files are not copied to `dist/` by Trunk. They would need a `copy-dir` directive added.

### `.cargo/config.toml`

```toml
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "link-arg=--no-entry",
  "-C", "link-arg=-zstack-size=1048576",
  "-C", "link-arg=--import-memory",
]
```

### CI/CD (`.github/workflows/deploy.yml`)

- Trigger: push to `main`
- Runner: `ubuntu-latest`
- Steps:
  1. `actions/checkout@v4`
  2. Cache: `~/.cargo/registry`, `~/.cargo/git`, `target/` keyed on `Cargo.lock` hash
  3. Install Rust stable via `dtolnay/rust-toolchain@stable` with `wasm32-unknown-unknown` target
  4. Install Trunk from latest release tarball
  5. `trunk build --release` with `RUSTFLAGS="-C target-feature=+bulk-memory"`
  6. Manual asset verification: copies `static/fonts/*.woff2` to `dist/fonts/`, creates `dist/404.html` (copy of `index.html`), touches `dist/.nojekyll`
  7. Deploy via `JamesIves/github-pages-deploy-action@v4` to `gh-pages` branch

**Note:** CI sets `RUSTFLAGS="-C target-feature=+bulk-memory"` which overrides the flags in `.cargo/config.toml` (Cargo only uses one source of rustflags). The `--no-entry`, `--import-memory`, and `stack-size` linker args from `.cargo/config.toml` are therefore not applied during CI builds.

### Dev environment

- `flake.nix`: Nix dev shell with `rust-bin.stable.latest.default` + `wasm32-unknown-unknown` target, `trunk`, `binaryen`, `openssl`
- `.envrc`: `use flake`
- `deny.toml`: cargo-deny config — allowed licenses: MIT, Apache-2.0, ISC, BSD-2-Clause, BSD-3-Clause, Zlib, Unicode-3.0

---

## 3. Entry Points

### `index.html`

- `<!DOCTYPE html>`, `<html lang="en" class="dark" style="background-color:#0f172a">`
- OG/Twitter meta tags pointing to `https://richardmussell.github.io/`
- JSON-LD structured data: `@type: Person`
- Font preloads: `inter-400.woff2`, `inter-500.woff2`, `jbmono-400.woff2`
- Inline critical CSS: box-sizing reset, dark/light background, font stack
- `<noscript>` fallback with name/email text
- `<div id="wasm-init-indicator">` loading spinner
- Inline JS #1: WASM preload hint — derives `.wasm` URL from modulepreload/script `src`
- Inline JS #2: 10-second timeout that replaces loading indicator with "App failed to load"

### `src/main.rs:143` — `fn main()`

1. `capture_wasm_start_time()` — records `performance.now()` into `OnceLock<f64>`
2. `std::panic::set_hook()` — custom panic hook renders red overlay via DOM manipulation, logs to `web_sys::console::error_1`
3. `mount_to_body(|| view! { <App /> })`
4. Removes `#wasm-init-indicator` element from DOM

### `src/main.rs:34` — `fn App()`

1. `provide_meta_context()`
2. Creates and provides 5 context signals: `is_dark`, `shortcuts_open`, `palette_open`, `ReadProgressSignals`, `ProjectCardSignals`
3. Dark/light mode effect: manipulates `html.classList` and `body.style.backgroundColor`
4. Renders: skip-to-content link, Router, KeyboardNav, NavBar, CommandPalette, ErrorBoundary, Routes (11 routes), BackToTop, keyboard shortcuts modal (Show)

---

## 4. Routing

All routes defined in `src/main.rs:94-107`:

| Path | Component | File |
|---|---|---|
| `/` | `HomePage` | `src/pages/home.rs:263` |
| `/about` | `AboutPage` | `src/pages/about.rs:6` |
| `/writing` | `WritingPage` | `src/pages/writing.rs:8` |
| `/writing/:slug` | `WriteupDetailPage` | `src/pages/writing.rs:173` |
| `/project/:slug` | `ProjectDetailPage` | `src/pages/project.rs:7` |
| `/project/:slug/docs` | `ProjectDocsPage` | `src/pages/project.rs:157` |
| `/project/:slug/demo` | `ProjectDemoPage` | `src/pages/project.rs:232` |
| `/resume` | `ResumePage` | `src/pages/resume.rs:7` |
| `/contact` | `ContactPage` | `src/pages/contact.rs:8` |
| `/telemetry` | `TelemetryPage` | `src/pages/telemetry.rs:101` |
| `/one-pager` | `OnePageSummary` | `src/pages/one_pager.rs:8` |
| `/*any` | `NotFoundPage` | `src/pages/not_found.rs:6` |

**Router type:** `leptos_router::Router` (hash-based client-side routing, CSR mode)  
**SPA 404 fallback:** CI copies `index.html` to `404.html` so GitHub Pages serves the SPA shell for all paths.

---

## 5. Component Tree

```
App
├── KeyboardNav                     (src/components/nav.rs:207)
├── NavBar                          (src/components/nav.rs:9)
│   └── ThemeToggle                 (src/components/nav.rs:115)
├── CommandPalette                  (src/components/palette.rs:82)
├── ErrorBoundary
│   └── Routes
│       ├── HomePage                (src/pages/home.rs:263)
│       │   ├── Terminal            (src/pages/home.rs:22)
│       │   ├── CertificationsSection (src/pages/home.rs:232)
│       │   └── ProjectCard[]       (src/components/project.rs:8)
│       ├── AboutPage               (src/pages/about.rs:6)
│       ├── WritingPage             (src/pages/writing.rs:8)
│       ├── WriteupDetailPage       (src/pages/writing.rs:173)
│       ├── ProjectDetailPage       (src/pages/project.rs:7)
│       │   └── RelatedProjects     (inline via related_row memo)
│       ├── ProjectDocsPage         (src/pages/project.rs:157)
│       ├── ProjectDemoPage         (src/pages/project.rs:232)
│       ├── ResumePage              (src/pages/resume.rs:7)
│       ├── ContactPage             (src/pages/contact.rs:8)
│       ├── TelemetryPage           (src/pages/telemetry.rs:101)
│       ├── OnePageSummary          (src/pages/one_pager.rs:8)
│       └── NotFoundPage            (src/pages/not_found.rs:6)
├── BackToTop                       (src/components/nav.rs:166)
│   └── ReadingProgress             (src/components/layout.rs:10) [via context, not tree]
└── Show (keyboard shortcuts modal) (src/main.rs:110-138)
```

Components defined but not mounted in the current tree:
- `StatCard` (`src/components/project.rs:83`) — defined, not referenced from any view
- `CodeBlock` (`src/components/project.rs:99`) — defined, not referenced from any view
- `TimelineSection` (`src/components/project.rs:143`) — defined, not referenced
- `BeforeAfterSection` (`src/components/project.rs:159`) — defined, not referenced
- `ReadingProgress` (`src/components/layout.rs:10`) — defined, updates `ReadProgressSignals` context via scroll listener, but is not mounted anywhere in the component tree; its effect runs via `BackToTop` reading the context

**Note on ReadingProgress:** It is not mounted in the view tree. The `ReadProgressSignals` context is provided by `App` and consumed by `BackToTop`, but the `ReadingProgress` component that creates the scroll listener is never rendered. The reading progress ring in `BackToTop` will always show 0% unless `ReadingProgress` is mounted somewhere.

---

## 6. State Management

### Signals (`create_signal`)

| Signal | Type | File:Line | Scope |
|---|---|---|---|
| `is_dark` / `set_is_dark` | `bool` | `src/main.rs:37` | App (provided as context) |
| `read_progress` / `set_read_progress` | `f64` | `src/main.rs:45` | App (provided as ReadProgressSignals context) |
| `expanded_slug` / `set_expanded_slug` | `Option<String>` | `src/main.rs:48` | App (provided as ProjectCardSignals context) |
| `email_copied` / `set_email_copied` | `bool` | `src/pages/home.rs:265` | HomePage |
| `active_filter` / `set_filter` | `Option<ProjectCategory>` | `src/pages/home.rs:266` | HomePage |
| `log_output` / `set_log_output` | `String` | `src/pages/home.rs:25` | Terminal |
| `user_input` / `set_user_input` | `String` | `src/pages/home.rs:26` | Terminal |
| `email_copied` / `set_email_copied` | `bool` | `src/pages/about.rs:7` | AboutPage |
| `email_copied` / `set_email_copied` | `bool` | `src/pages/resume.rs:8` | ResumePage |
| `email_copied` / `set_email_copied` | `bool` | `src/pages/contact.rs:9` | ContactPage |
| `email_copied` / `set_email_copied` | `bool` | `src/pages/one_pager.rs:9` | OnePageSummary |
| `search_query` / `set_search_query` | `String` | `src/pages/writing.rs:9` | WritingPage |
| `active_category` / `set_active_category` | `Option<&'static str>` | `src/pages/writing.rs:10` | WritingPage |
| `heap_used` / `set_heap_used` | `Option<f64>` | `src/pages/telemetry.rs:102` | TelemetryPage |
| `heap_total` / `set_heap_total` | `Option<f64>` | `src/pages/telemetry.rs:103` | TelemetryPage |
| `wasm_init_ms` / `set_wasm_init_ms` | `f64` | `src/pages/telemetry.rs:104` | TelemetryPage |
| `ttfb_ms` / `set_ttfb_ms` | `Option<f64>` | `src/pages/telemetry.rs:105` | TelemetryPage |
| `ua` / `set_ua` | `String` | `src/pages/telemetry.rs:106` | TelemetryPage |
| `network_rows` / `set_network_rows` | `Vec<String>` | `src/pages/telemetry.rs:107` | TelemetryPage |
| `logs` / `set_logs` | `VecDeque<String>` | `src/pages/telemetry.rs:108` | TelemetryPage |
| `copied` / `set_copied` | `bool` | `src/components/project.rs:100` | CodeBlock |
| `nav_open` / `set_nav_open` | `bool` | `src/components/nav.rs:16` | NavBar |
| `visible` / `set_visible` | `bool` | `src/components/nav.rs:167` | BackToTop |
| `query` / `set_query` | `String` | `src/components/palette.rs:85` | CommandPalette |
| `selected_idx` / `set_selected_idx` | `usize` | `src/components/palette.rs:86` | CommandPalette |

### RwSignals (`create_rw_signal`)

| Signal | Type | File:Line | Scope |
|---|---|---|---|
| `shortcuts_open` | `bool` | `src/main.rs:40` | App (provided as context) |
| `palette_open` | `bool` | `src/main.rs:42` | App (provided as context) |
| `did_drag` | `bool` | `src/main.rs:49` | App (provided as ProjectCardSignals context) |
| `drag_start_x` | `f64` | `src/pages/home.rs:281` | HomePage |
| `drag_start_y` | `f64` | `src/pages/home.rs:282` | HomePage |
| `last_heap_used` | `Option<f64>` | `src/pages/telemetry.rs:109` | TelemetryPage |

### Context (`provide_context` / `use_context`)

| Context type | Provided at | Consumed at |
|---|---|---|
| `ReadSignal<bool>` (is_dark) | `src/main.rs:38` | Not consumed via `use_context` — passed as prop to `NavBar` |
| `RwSignal<bool>` (shortcuts_open) | `src/main.rs:41` | `src/components/nav.rs:15`, `src/components/nav.rs:212` |
| `RwSignal<bool>` (palette_open) | `src/main.rs:43` | `src/components/palette.rs:83`, `src/components/nav.rs:213` |
| `ReadProgressSignals` | `src/main.rs:46` | `src/components/layout.rs:11`, `src/components/nav.rs:168` |
| `ProjectCardSignals` | `src/main.rs:50` | `src/pages/home.rs:279`, `src/components/nav.rs:214` |

### `store_value`

| Value | File:Line |
|---|---|
| `navigate` (NavigateFn) | `src/pages/home.rs:23` |
| `boot_lines` (Vec<String>) | `src/pages/home.rs:24` |
| `initialized` (bool) | `src/pages/home.rs:27` |
| `certs` (Vec<Certification>) | `src/pages/home.rs:233` |
| `projects` (Vec<ProjectIndex>) | `src/pages/home.rs:264` |
| `navigate` (NavigateFn) | `src/components/palette.rs:84` |
| `index` (Vec<PaletteItem>) | `src/components/palette.rs:87` |

### Resources (`create_resource`)

| Resource | Trigger | Fetches | File:Line |
|---|---|---|---|
| `detail` | `slug()` | `/writeups/{slug}.json` via `gloo_net::http::Request` | `src/pages/writing.rs:180` |
| `detail` | `slug()` | `/projects/{slug}.json` via `gloo_net::http::Request` | `src/pages/project.rs:12` |
| `docs` | `slug()` | `/docs/{slug}.json` via `gloo_net::http::Request` | `src/pages/project.rs:161` |

### Memos (`create_memo`)

| Memo | Returns | File:Line |
|---|---|---|
| `counts` | `(usize, usize, usize, usize, usize)` project counts per category | `src/pages/home.rs:268` |
| `filtered` | `Vec<WriteUpIndex>` sorted/filtered writeup list | `src/pages/writing.rs:29` |
| `index` | `Option<WriteUpIndex>` matching writeup for current slug | `src/pages/writing.rs:179` |
| `project` | `Option<ProjectIndex>` matching project for current slug | `src/pages/project.rs:10` |
| `related_row` | `Vec<(String, String, String)>` related projects | `src/pages/project.rs:29` |
| `project` (docs) | `Option<ProjectIndex>` | `src/pages/project.rs:160` |
| `project` (demo) | `Option<ProjectIndex>` | `src/pages/project.rs:235` |
| `results` | `Vec<PaletteItem>` filtered/ranked command palette results | `src/components/palette.rs:89` |

### Effects (`create_effect`)

| Effect purpose | File:Line |
|---|---|
| Dark/light mode: toggle `html.classList`, set `body.style.backgroundColor` | `src/main.rs:52` |
| Terminal boot animation: schedule `set_timeout` for each boot line, auto-focus input | `src/pages/home.rs:29` |
| Track `resume_view` event | `src/pages/resume.rs:9` |
| Track `writeup_view` event | `src/pages/writing.rs:197` |
| Telemetry init: read UA, WASM init time, TTFB, heap | `src/pages/telemetry.rs:111` |
| Scroll listener: reading progress computation | `src/components/layout.rs:12` |
| NavBar: close drawer on route change | `src/components/nav.rs:19` |
| BackToTop: scroll listener for visibility toggle | `src/components/nav.rs:169` |
| KeyboardNav: global keydown listener with chord handling | `src/components/nav.rs:215` |
| CommandPalette: auto-focus input on open | `src/components/palette.rs:103` |

### Cleanup (`on_cleanup`)

| Cleanup | File:Line |
|---|---|
| Drop `heap_interval` and `network_interval` (Interval handles) | `src/pages/telemetry.rs:197` |

**Note:** No other `on_cleanup` calls exist. The `Closure::forget()` calls in `BackToTop`, `ReadingProgress`, and `KeyboardNav` intentionally leak the closures (they are meant to live for the app lifetime).

---

## 7. Browser API Surface (Exhaustive)

This section lists every place the code touches browser-only APIs. Each entry would need `cfg(feature = "ssr")` gating or a hydration guard for an SSR migration.

### `web_sys::window()`

| File:Line | Description |
|---|---|
| `src/utils.rs:13` | Get `performance` object for `perf_now_ms()` |
| `src/main.rs:53` | Get `window` for dark/light mode classList/style manipulation |
| `src/main.rs:83` | Get `window.history().back()` in ErrorBoundary fallback click |
| `src/main.rs:149` | Get `document` for panic hook DOM overlay |
| `src/main.rs:160` | `web_sys::console::error_1()` in panic hook |
| `src/main.rs:166` | Get `document` to remove `#wasm-init-indicator` |
| `src/pages/home.rs:49` | Get `document` for terminal auto-focus |
| `src/pages/resume.rs:25` | `window.print()` for resume PDF |
| `src/pages/not_found.rs:26` | `window.history().back()` for back button |
| `src/pages/one_pager.rs:160` | `window.print()` for one-pager PDF |
| `src/pages/telemetry.rs:10` | Get `performance` for `perf_now_ms()` |
| `src/pages/telemetry.rs:18` | Get `performance.memory` for heap bytes |
| `src/pages/telemetry.rs:27` | Get `performance.getEntriesByType("navigation")` for TTFB |
| `src/pages/telemetry.rs:112` | Get `window.navigator()` for user agent |
| `src/components/layout.rs:13` | Get `window` for scroll event listener |
| `src/components/layout.rs:16` | Get `window` inside scroll callback for scroll_y, scrollHeight, innerHeight |
| `src/components/nav.rs:170` | Get `window` for scroll event listener (BackToTop) |
| `src/components/nav.rs:172` | Get `window.scroll_y()` inside scroll callback |
| `src/components/nav.rs:198` | `window.scroll_to_with_x_and_y(0.0, 0.0)` for back-to-top button |
| `src/components/nav.rs:216` | Get `window` + `document` for KeyboardNav global keydown listener |
| `src/components/palette.rs:106` | Get `document` for auto-focus on palette open |

### `window.document()` / DOM manipulation

| File:Line | Description |
|---|---|
| `src/main.rs:54` | `window.document()` for html and body references |
| `src/main.rs:55` | `document.document_element()` for classList |
| `src/main.rs:56` | `document.body()` for style |
| `src/main.rs:58-59` | `html.class_list().add_1("dark")`, `remove_1("light")` |
| `src/main.rs:60` | `body.style().set_property("background-color", ...)` |
| `src/main.rs:62-64` | Light mode: same classList + style operations |
| `src/main.rs:149-158` | Panic hook: `document.body()`, `create_element("div")`, `set_attribute("style", ...)`, `set_text_content()`, `prepend_with_node_1()` |
| `src/main.rs:166` | `document.get_element_by_id("wasm-init-indicator").remove()` |
| `src/pages/home.rs:49-52` | `document.get_element_by_id("terminal-input")`, `dyn_ref::<HtmlInputElement>`, `.focus()` |
| `src/components/nav.rs:217` | `window.document()` for active element checks |
| `src/components/nav.rs:252-253` | `document.active_element()`, `dyn_ref::<HtmlElement>`, `.blur()` |
| `src/components/nav.rs:259-264` | `document.active_element()` — check tag name, contenteditable attribute |
| `src/components/nav.rs:278-280` | `document.get_element_by_id("keyboard-shortcuts-modal")`, `dyn_ref::<HtmlElement>`, `.focus()` |
| `src/components/nav.rs:318-323` | `document.get_element_by_id("terminal-input")`, `dyn_ref::<HtmlInputElement>`, `.focus()` |
| `src/components/palette.rs:109-112` | `document.get_element_by_id("command-palette-input")`, `dyn_ref::<HtmlInputElement>`, `.focus()`, `.select()` |

### `window.navigator()`

| File:Line | Description |
|---|---|
| `src/pages/telemetry.rs:113` | `window.navigator()` |
| `src/pages/telemetry.rs:114` | `navigator.user_agent()` |

### `window.history()` / `history.back()`

| File:Line | Description |
|---|---|
| `src/main.rs:84-85` | `window.history().back()` in ErrorBoundary fallback |
| `src/pages/not_found.rs:26` | `window.history().back()` in 404 page back button |

### `window.print()`

| File:Line | Description |
|---|---|
| `src/pages/resume.rs:25` | Print resume page |
| `src/pages/one_pager.rs:160` | Print one-pager page |

### `window.scroll_y()` / `window.scroll_to()`

| File:Line | Description |
|---|---|
| `src/components/layout.rs:18` | `win.scroll_y()` for reading progress calculation |
| `src/components/nav.rs:172` | `window.scroll_y()` for BackToTop visibility |
| `src/components/nav.rs:198` | `window.scroll_to_with_x_and_y(0.0, 0.0)` |

### `window.inner_height()`

| File:Line | Description |
|---|---|
| `src/components/layout.rs:20` | `win.inner_height()` for reading progress denominator |

### `document.body().scroll_height()`

| File:Line | Description |
|---|---|
| `src/components/layout.rs:19` | `doc.body().scroll_height()` for reading progress denominator |

### `performance` API (via `js_sys::Reflect`)

| File:Line | Description |
|---|---|
| `src/utils.rs:14-17` | `performance.now()` via Reflect for WASM start time |
| `src/pages/telemetry.rs:11-14` | `performance.now()` via Reflect (local perf_now_ms) |
| `src/pages/telemetry.rs:19-22` | `performance.memory.usedJSHeapSize`, `totalJSHeapSize` |
| `src/pages/telemetry.rs:28-42` | `performance.getEntriesByType("navigation")` for TTFB (requestStart, responseStart) |

### `js_sys` usage for clipboard and tracking

| File:Line | Description |
|---|---|
| `src/utils.rs:45-48` | `js_sys` call dispatches CustomEvent for analytics tracking (release only) |
| `src/pages/home.rs:474` | `js_sys` call for `navigator.clipboard.writeText()` email copy |
| `src/pages/about.rs:119` | `js_sys` call for `navigator.clipboard.writeText()` email copy |
| `src/pages/resume.rs:39` | `js_sys` call for `navigator.clipboard.writeText()` email copy (header) |
| `src/pages/resume.rs:217` | `js_sys` call for `navigator.clipboard.writeText()` email copy (footer) |
| `src/pages/one_pager.rs:25` | `js_sys` call for `navigator.clipboard.writeText()` email copy |
| `src/pages/contact.rs:36` | `js_sys` call for `navigator.clipboard.writeText()` email copy |
| `src/components/project.rs:127` | `js_sys` call for `navigator.clipboard.writeText()` code copy |

### `web_sys::console`

| File:Line | Description |
|---|---|
| `src/main.rs:160` | `console::error_1()` in panic hook |

### `gloo_net::http::Request`

| File:Line | Description |
|---|---|
| `src/pages/writing.rs:187` | `Request::get("/writeups/{slug}.json")` in create_resource |
| `src/pages/project.rs:19` | `Request::get("/projects/{slug}.json")` in create_resource |
| `src/pages/project.rs:168` | `Request::get("/docs/{slug}.json")` in create_resource |
| `src/pages/telemetry.rs:71` | `Request::get("/projects/{slug}.json")` in network probe |

### `gloo_timers`

| File:Line | Type | Description |
|---|---|---|
| `src/pages/telemetry.rs:156` | `Interval::new(2000, ...)` | Heap metrics refresh |
| `src/pages/telemetry.rs:189` | `Interval::new(15000, ...)` | Network probe refresh |
| `src/pages/contact.rs:39` | `Timeout::new(2000, ...)` | Email copied reset |
| `src/pages/one_pager.rs:28` | `Timeout::new(2000, ...)` | Email copied reset |
| `src/components/nav.rs:220` | `Rc<RefCell<Option<Timeout>>>` | Keyboard chord timeout handle |
| `src/components/nav.rs:277` | `Timeout::new(0, ...)` | Deferred focus after shortcuts modal open |
| `src/components/nav.rs:296` | `Timeout::new(1000, ...)` | G-chord expiry |
| `src/components/palette.rs:108` | `Timeout::new(0, ...)` | Deferred focus on palette open |

### `leptos::set_timeout` (re-export of gloo_timers)

| File:Line | Duration | Description |
|---|---|---|
| `src/pages/home.rs:35` | 350ms x i | Boot line animation delays |
| `src/pages/home.rs:47` | 120ms | Terminal auto-focus delay |
| `src/pages/home.rs:140` | 300ms | Navigate after terminal command |
| `src/pages/home.rs:476` | 2000ms | Email copied reset |
| `src/pages/about.rs:121` | 2000ms | Email copied reset |
| `src/pages/resume.rs:41` | 2000ms | Email copied reset (header) |
| `src/pages/resume.rs:219` | 2000ms | Email copied reset (footer) |
| `src/components/project.rs:129` | 2000ms | Code copy reset |

### `wasm_bindgen::closure::Closure` + `closure.forget()`

| File:Line | Event | Description |
|---|---|---|
| `src/components/layout.rs:15,26` | `scroll` | Reading progress listener — leaked |
| `src/components/nav.rs:171,176` | `scroll` | BackToTop visibility listener — leaked |
| `src/components/nav.rs:227,329` | `keydown` | KeyboardNav global listener — leaked |

### `leptos::spawn_local`

| File:Line | Description |
|---|---|
| `src/pages/telemetry.rs:182` | Initial network probe |
| `src/pages/telemetry.rs:192` | Network probe inside interval callback |

### `leptos_router` navigation APIs

| API | File:Line |
|---|---|
| `use_navigate()` | `src/pages/home.rs:23`, `src/components/nav.rs:210`, `src/components/palette.rs:84`, `src/components/project.rs:18` |
| `use_location()` | `src/components/nav.rs:13`, `src/pages/not_found.rs:7` |
| `use_params_map()` | `src/pages/writing.rs:174`, `src/pages/project.rs:8`, `src/pages/project.rs:158`, `src/pages/project.rs:233` |

### `web_sys::KeyboardEvent` (typed event handlers)

| File:Line | Description |
|---|---|
| `src/pages/home.rs:61` | Terminal handle_keydown — Enter key dispatch |
| `src/components/project.rs:22` | ProjectCard keydown — Enter/Space navigate |
| `src/components/nav.rs:227` | KeyboardNav global keydown — Cmd+K, Escape, ?, g-chord, / |
| `src/components/palette.rs:137` | CommandPalette keydown — ArrowDown/Up, Enter, Escape |

### `inner_html` usage (XSS surface)

| File:Line | Source | Description |
|---|---|---|
| `src/pages/writing.rs:281` | `d.content` from JSON | Writeup detail body rendered as raw HTML |
| `src/pages/project.rs:103` | `d.content` from JSON | Project detail body rendered as raw HTML |
| `src/pages/project.rs:215` | `d.content` from JSON | Project docs body rendered as raw HTML |
| `src/components/project.rs:136` | `highlighted` from `highlight_code()` | Syntax-highlighted code rendered as raw HTML |

### `OnceLock` (global mutable state)

| File:Line | Description |
|---|---|
| `src/utils.rs:10` | `WASM_START_TIME_MS: OnceLock<f64>` — set once at startup, read-only after |

### `LazyLock` (global static data)

| File:Line | Description |
|---|---|
| `src/pages/home.rs:19` | `BOOT_LINES: LazyLock<Vec<String>>` |
| `src/data/writeups.rs` | `WRITEUPS: LazyLock<Vec<WriteUpIndex>>` |
| `src/data/projects.rs` | `PROJECTS: LazyLock<Vec<ProjectIndex>>` |
| `src/data/certs.rs` | `CERTIFICATIONS: LazyLock<Vec<Certification>>` |

---

## 8. Async & Data Loading

### Runtime fetch pattern

All data fetching uses `gloo_net::http::Request::get()` inside `create_resource`. The resource trigger is a reactive `slug()` closure. Fetches are relative-path (`/writeups/`, `/projects/`, `/docs/`).

| Resource | URL pattern | Deserializes to | File:Line |
|---|---|---|---|
| Writeup detail | `/writeups/{slug}.json` | `WriteUpDetail` | `src/pages/writing.rs:180` |
| Project detail | `/projects/{slug}.json` | `ProjectDetail` | `src/pages/project.rs:12` |
| Project docs | `/docs/{slug}.json` | `ProjectDetail` | `src/pages/project.rs:161` |
| Network probe | `/projects/{slug}.json` (4 slugs) | Response status only | `src/pages/telemetry.rs:71` |

### JSON schema

**WriteUpDetail** (`src/data/writeups.rs`): `{ slug: String, content: String }`  
**ProjectDetail** (`src/data/projects.rs`): `{ slug: String, content: String, demo_url: Option<String> }`

### Static data (compiled in)

All index-level data is compiled into the WASM binary via `LazyLock<Vec<T>>`:
- `WRITEUPS`: 17 entries (title, slug, summary, date, read_time, tags, category, is_core)
- `PROJECTS`: 4 entries (title, slug, subtitle, description, tech_stack, category, status)
- `CERTIFICATIONS`: 5 entries (name, issuer, status)

Detail/body content is fetched at runtime from JSON files, not compiled in.

### Suspense boundaries

| Location | Fallback text | File:Line |
|---|---|---|
| `WriteupDetailPage` | "Loading..." | `src/pages/writing.rs:260` |
| `ProjectDetailPage` | "Loading..." | `src/pages/project.rs:94` |

`ProjectDocsPage` does not use `<Suspense>` — it renders "Loading..." as the `None` match arm.

---

## 9. Keyboard & Event Systems

### Global keyboard handler (`KeyboardNav`, `src/components/nav.rs:207-332`)

Installed via `create_effect` then `window.add_event_listener_with_callback("keydown", ...)` then `Closure::forget()`.

| Key | Condition | Action |
|---|---|---|
| `Cmd/Ctrl + K` | Any context | Toggle command palette, preventDefault |
| `Escape` | Palette open | Close palette |
| `Escape` | Not in palette | Close shortcuts modal, clear g-chord, close expanded project card, blur active element |
| `?` | Not in input/textarea/contenteditable | Toggle keyboard shortcuts modal, focus modal |
| `g` then `h` | Within 1000ms, not in input | Navigate to `/` |
| `g` then `p` | Within 1000ms, not in input | Navigate to first project detail |
| `g` then `r` | Within 1000ms, not in input | Navigate to `/resume` |
| `/` | Not in input, no modifier keys | Focus `#terminal-input`, preventDefault |

Guard: checks `document.active_element()` tag name and `contenteditable` attribute before processing non-Escape shortcuts.

### Component-level keyboard handlers

| Component | Element | Keys | File:Line |
|---|---|---|---|
| Terminal | `#terminal-input` | `Enter` — parse and dispatch command | `src/pages/home.rs:61` |
| ProjectCard | `article[tabindex=0]` | `Enter`, `Space` — navigate to project | `src/components/project.rs:22` |
| CommandPalette | `#command-palette-input` | `ArrowDown/Up`, `Enter`, `Escape` | `src/components/palette.rs:137` |

### Mouse/pointer events

| Component | Event | File:Line | Description |
|---|---|---|---|
| HomePage projects section | `mousedown` | `src/pages/home.rs:342` | Record drag start coordinates |
| HomePage projects section | `mousemove` | `src/pages/home.rs:347` | Detect drag (>5px) to suppress card click |
| ProjectCard | `click` (on `<a>`) | `src/components/project.rs:34` | preventDefault, expand card overlay if not dragging |

### Scroll listeners

| Listener | File:Line | Mechanism | Cleanup |
|---|---|---|---|
| Reading progress | `src/components/layout.rs:12-26` | `Closure::forget()` on "scroll" | None (leaked) |
| BackToTop visibility | `src/components/nav.rs:169-177` | `Closure::forget()` on "scroll" | None (leaked) |

---

## 10. Telemetry Page

**Component:** `TelemetryPage` (`src/pages/telemetry.rs:101`)

### Metrics collected

| Metric | Source | File:Line |
|---|---|---|
| Heap used (bytes) | `performance.memory.usedJSHeapSize` | `src/pages/telemetry.rs:21` |
| Heap total (bytes) | `performance.memory.totalJSHeapSize` | `src/pages/telemetry.rs:22` |
| WASM init time (ms) | `performance.now()` delta from `wasm_start_time_ms()` | `src/pages/telemetry.rs:117-119` |
| TTFB (ms) | Navigation Timing API: `responseStart - requestStart` | `src/pages/telemetry.rs:26-43` |
| User agent | `navigator.userAgent` | `src/pages/telemetry.rs:114` |
| Network probe latency | `gloo_net::http::Request::get()` timing 4 project JSON files | `src/pages/telemetry.rs:60-98` |

### Intervals

| Interval | Period | Purpose | Cleanup |
|---|---|---|---|
| `heap_interval` | 2000ms | Re-read heap metrics, log deltas >64KB | `on_cleanup` drop at `src/pages/telemetry.rs:198` |
| `network_interval` | 15000ms | Re-run network probe against 4 project JSONs | `on_cleanup` drop at `src/pages/telemetry.rs:199` |

### Log buffer

`VecDeque<String>` capped at 10 entries (`src/pages/telemetry.rs:54`). New entries pushed to front, oldest popped from back.

### Probed endpoints

`/projects/linux-admin-scripting.json`, `/projects/monitoring-observability.json`, `/projects/terraform-gcp.json`, `/projects/zero-trust-networking.json`

---

## 11. Terminal Emulator

**Component:** `Terminal` (`src/pages/home.rs:22-224`)

### Boot sequence

5 hardcoded boot lines (`src/pages/home.rs:10-16`) displayed sequentially via `set_timeout` at 350ms intervals.

### Command dispatch table

| Command | Response | File:Line |
|---|---|---|
| `help` | Lists available commands | `src/pages/home.rs:74` |
| `status` | Fleet status one-liner | `src/pages/home.rs:75` |
| `projects` | Lists all project slugs with category and status | `src/pages/home.rs:76` |
| `contact` | Email + LinkedIn + location | `src/pages/home.rs:84` |
| `ls` | Projects list + routes list | `src/pages/home.rs:85` |
| `cat resume` | Navigates to `/resume` | `src/pages/home.rs:95` |
| `cat telemetry` | Navigates to `/telemetry` | `src/pages/home.rs:99` |
| `cd <target>` | Navigates to project or route if found | `src/pages/home.rs:103` |
| `clear` | Clears terminal output | `src/pages/home.rs:125` |
| (anything else) | "Command not found" error | `src/pages/home.rs:130` |

### Input handling

- Max 100 chars (HTML `maxlength` + Rust `.take(100)`)
- Control characters stripped (`.filter(|c| !c.is_control())`)
- Navigation via `use_navigate()` with 300ms delay after command

### Auto-focus

Terminal input (`#terminal-input`) is auto-focused 120ms after mount (`src/pages/home.rs:47-58`).

---

## 12. Syntax Highlighters

**File:** `src/utils.rs:72-315`

### Dispatcher

`highlight_code(lang, code)` at `src/utils.rs:72-79` matches on uppercase lang string:

| Input | Handler |
|---|---|
| `POWERSHELL`, `PWSH`, `PS1`, `PS` | `hl_ps()` |
| `BASH`, `SHELL`, `SH` | `hl_sh()` |
| `YAML`, `YML` | `hl_yaml()` |
| `RUST`, `RS` | `hl_rust()` |
| (anything else) | `esc()` (plain HTML escaping) |

### Token CSS classes emitted

`tok-kw` (keyword), `tok-str` (string), `tok-cmt` (comment), `tok-fn` (function/command), `tok-ty` (type), `tok-num` (number), `tok-op` (operator), `tok-var` (variable), `tok-param` (parameter), `tok-attr` (attribute), `tok-const` (constant), `tok-yk` (YAML key), `tok-yv` (YAML value)

### Implementation pattern

All four highlighters are hand-rolled character-by-character parsers. Each produces `<span class="tok-*">...</span>` HTML strings. They process line-by-line. No external highlighting crate is used.

### Keyword lists

- **Rust:** 39 keywords at `src/utils.rs:83-89`
- **PowerShell:** 39 keywords + 20 comparison operators at `src/utils.rs:178-187`
- **Bash:** 28 keywords at `src/utils.rs:231`, 80+ command names at `src/utils.rs:232`
- **YAML:** 8 boolean/null keywords at `src/utils.rs:266`

### HTML escaping

`esc()` at `src/utils.rs:51-64` escapes `&`, `<`, `>`, `"`, `'`.

---

## 13. Styling

**Entry point:** `style/style.css` — imports all split files below. Previously a 6310-line monolith; split into design tokens, base, per-component, and per-page files (commit `5664605`). No CSS preprocessor. No Tailwind build step.

**`style/tokens.css`** — CSS custom properties for the full design token set (colors, spacing, typography, shadows).  
**`style/base.css`** — reset, global element styles, font declarations, utility classes, print and a11y blocks.  
**`style/components/buttons.css`** — button variants and states.  
**`style/components/cards.css`** — card, stat-card, project-card, and related variants.  
**`style/components/layout.css`** — grid helpers, section wrappers, skip-link, back-to-top, reading progress.  
**`style/components/nav.css`** — navbar, drawer, theme toggle, command palette, keyboard shortcuts modal.  
**`style/components/palette.css`** — command palette overlay and result list.  
**`style/components/terminal.css`** — terminal emulator chrome, boot lines, prompt, input.  
**`style/pages/contact.css`** — contact page layout and form elements.  
**`style/pages/home.css`** — hero, certifications section, project grid, filter bar.  
**`style/pages/project.css`** — project detail, docs, demo page layouts.  
**`style/pages/resume.css`** — resume page layout and print overrides.  
**`style/pages/telemetry.css`** — telemetry metrics grid and log output.  
**`style/pages/writing.css`** — writing index and writeup detail layouts.

### Font declarations

| Font family | Weights | Format | Source |
|---|---|---|---|
| Inter | 400, 500, 600, 700, 800 | woff2 | `/fonts/inter-{weight}.woff2` |
| JetBrains Mono | 400, 500, 600 | woff2 | `/fonts/jbmono-{weight}.woff2` |

All declarations use `font-display: swap`.

### Theming

- CSS custom properties defined on `:root` / `html` for dark mode (default)
- `html.light` selector overrides all custom properties for light mode
- Properties include: `--bg-base`, `--bg-surface`, `--bg-elevated`, `--text-primary`, `--text-secondary`, `--text-muted`, `--border-subtle`, `--border-active`, `--accent-cyan`, etc.

### Responsive breakpoints

| Breakpoint | Usage |
|---|---|
| `max-width: 480px` | Mobile-specific adjustments |
| `max-width: 768px` | Tablet/mobile layouts |
| `max-width: 1024px` | Compact desktop |

### Inline Tailwind-style utilities

The CSS file includes hand-written utility classes that match Tailwind naming conventions (e.g., `flex`, `grid`, `gap-*`, `text-*`, `px-*`, `py-*`, `rounded-*`). These are used directly in component markup alongside custom component classes. No Tailwind CDN or build process is involved.

### Print CSS

Two `@media print` blocks:
1. **One-pager print** (around line 1064): Minimal rules for one-pager layout
2. **Resume print** (around line 3771): Comprehensive rules with `@page` margins, font sizing, element hiding, grid-to-block conversion, link URL printing via `::after`

### Accessibility CSS

- `.skip-to-content`: visually hidden until focused
- `.sr-only`: screen-reader-only utility
- `focus-visible` outlines on interactive elements
- `@media (forced-colors: active)`: forced-colors media query adjustments
- Two `@media (prefers-reduced-motion: reduce)` blocks disabling animations and transitions
- Minimum touch target: 44px on mobile

### Performance CSS

- `content-visibility: auto` on `.project-grid` and `.certifications-section`
- Scrollbar custom styling (`::-webkit-scrollbar`)
- `::selection` highlight styling

### Syntax highlighting token colors

Token classes (`tok-kw`, `tok-str`, `tok-cmt`, etc.) have both dark and light variants defined.

---

## 14. Static Content

### JSON content files

| Directory | Count | Schema | Loaded by |
|---|---|---|---|
| `static/writeups/` | 17 files | `{ slug: string, content: string }` | WriteupDetailPage via create_resource |
| `static/projects/` | 4 files | `{ slug: string, content: string, demo_url: string|null }` | ProjectDetailPage via create_resource |
| `static/docs/` | 4 files | `{ slug: string, content: string, demo_url: string|null }` | ProjectDocsPage via create_resource |

**Note on `static/docs/`:** This directory exists and contains 4 JSON files, but there is no `copy-dir` Trunk directive for it in `index.html`. The docs JSON files will not be included in `dist/` builds unless a `<link data-trunk rel="copy-dir" href="static/docs"/>` directive is added.

### Content processing

`WriteupDetailPage` applies string replacements to `d.content` before rendering (`src/pages/writing.rs:269-279`):

| Original string | Replacement |
|---|---|
| `SECTION_01: THE TACTICAL DASHBOARD` | `TECHNICAL FOUNDATIONS` |
| `EXECUTIVE_SUMMARY_//_TECHNICAL_PILLARS` | `Executive Summary` |
| `//_BEGIN_STRATEGIC_NARRATIVE` | `FULL NARRATIVE` |
| `SECTION_02: THE STRATEGIC NARRATIVE (LaTeX View)` | `FULL NARRATIVE` |
| `SECTION_02: THE STRATEGIC NARRATIVE` | `FULL NARRATIVE` |
| `[DOWNLOAD_STRATEGIC_WHITE_PAPER_//_PDF]` | `Download Strategic Brief (PDF)` |
| `[DOWNLOAD_WHITE_PAPER_PDF]` | `Download Strategic Brief (PDF)` |
| `AUTHOR: Senior Principal Platform Architect` | `AUTHOR: Richard Mussell -- Principal Platform Architect` |
| Two variants of `ARCHITECT'S SEAL` lines | `Richard Mussell -- Principal Platform Architect` |

### PDFs

| File | Path |
|---|---|
| `builders-ledger.pdf` | `public/pdfs/builders-ledger.pdf` |
| `orchestrated-landscape.pdf` | `public/pdfs/orchestrated-landscape.pdf` |
| `platform-architecture-blueprint.pdf` | `public/pdfs/platform-architecture-blueprint.pdf` |
| `sustainable-architect.pdf` | `public/pdfs/sustainable-architect.pdf` |
| `universal-dialects.pdf` | `public/pdfs/universal-dialects.pdf` |

**Note:** The `public/pdfs/` directory is served via the `[[assets]] source = "public"` directive in `Trunk.toml`, so these PDFs end up at `/pdfs/*.pdf` in the deployed site.

### Images

| File | Path | Dimensions |
|---|---|---|
| `og-image.png` | `public/og-image.png` | 1200x630 |
| `og-image-template.html` | `public/og-image-template.html` | Generator file, not served as asset |

### Favicons

| File | Path |
|---|---|
| `favicon.svg` | Root |
| `favicon-32.png` | Root |

### Other static files

| File | Path | Notes |
|---|---|---|
| `manifest.json` | Root | PWA manifest, single icon entry (favicon.svg) |
| `robots.txt` | Root | `Allow: /` |
| `public/robots.txt` | `public/` | `Allow: /`, `Disallow: /one-pager`, Sitemap reference |
| `public/sitemap.xml` | `public/` | Contains project URLs — unclear if slug paths are current |

**Note:** Two `robots.txt` files exist: one in root (copied to dist/ by Trunk) and one in public/ (served via [[assets]]). The root one takes precedence in the Trunk build since it is explicitly copied via copy-file.

---

## 15. Fonts & Media

### Self-hosted fonts

| File | Family | Weight | Format |
|---|---|---|---|
| `static/fonts/inter-400.woff2` | Inter | 400 (Regular) | woff2 |
| `static/fonts/inter-500.woff2` | Inter | 500 (Medium) | woff2 |
| `static/fonts/inter-600.woff2` | Inter | 600 (SemiBold) | woff2 |
| `static/fonts/inter-700.woff2` | Inter | 700 (Bold) | woff2 |
| `static/fonts/inter-800.woff2` | Inter | 800 (ExtraBold) | woff2 |
| `static/fonts/jbmono-400.woff2` | JetBrains Mono | 400 (Regular) | woff2 |
| `static/fonts/jbmono-500.woff2` | JetBrains Mono | 500 (Medium) | woff2 |
| `static/fonts/jbmono-600.woff2` | JetBrains Mono | 600 (SemiBold) | woff2 |

### Font download script

`scripts/fetch-fonts.sh` — downloads from Google Fonts API.

### Preloaded fonts (index.html)

Only 3 of 8 fonts are preloaded: `inter-400`, `inter-500`, `jbmono-400`.

### Media assets

No images are referenced from Rust code. No SVG sprite sheet. All SVGs are inline in component markup (theme toggle icons, back-to-top arrow, breadcrumb chevrons, reading progress ring).

---

## 16. Print & Accessibility

### Print support

| Page | Mechanism | File:Line |
|---|---|---|
| Resume | `web_sys::window().print()` via button click | `src/pages/resume.rs:25` |
| One-Pager | `web_sys::window().print()` via button click | `src/pages/one_pager.rs:160` |

Dedicated `@media print` CSS blocks exist for both pages in `style/style.css`.

### ARIA attributes in markup

| Attribute | Occurrences | Files |
|---|---|---|
| `aria-label` | ~30+ | All page and component files |
| `aria-hidden="true"` | ~15+ | Decorative elements, separators, SVGs |
| `aria-modal="true"` | 2 | Keyboard shortcuts modal (`src/main.rs:119`), command palette (`src/components/palette.rs:127`) |
| `aria-live="polite"` | 1 | Terminal output region (`src/pages/home.rs:157`) |
| `aria-expanded` | 1 | Mobile nav toggle (`src/components/nav.rs:65`) |
| `aria-labelledby` | 1 | Hero section (`src/pages/home.rs:291`) |
| `role="navigation"` | 1 | NavBar (`src/components/nav.rs:27`) |
| `role="dialog"` | 2 | Shortcuts modal, command palette |
| `role="list"` / `role="listitem"` | 3 | Certifications, project card articles, timeline |
| `role="search"` | 1 | Writing page search (`src/pages/writing.rs:96`) |
| `role="region"` | 2 | Terminal output, code block |
| `role="table"` / `role="cell"` | 1 | Before/after comparison |
| `tabindex="0"` | 2 | Skip-to-content link, project card articles |
| `tabindex="-1"` | 1 | Shortcuts modal |

### Skip-to-content link

`<a href="#main-content" class="skip-to-content" tabindex="0">"Skip to main content"</a>` in App component (`src/main.rs:69`). All main pages have `id="main-content"` on their `<main>` element.

### Screen reader utilities

`.sr-only` class used on external link indicators ("(opens in new tab)" text).

### Reduced motion

Two `@media (prefers-reduced-motion: reduce)` blocks in `style/style.css` disable animations and transitions.

### Forced colors

One `@media (forced-colors: active)` block in `style/style.css`.

### Focus indicators

`focus-visible` outlines defined for interactive elements in CSS.

### Touch targets

44px minimum touch target enforced on mobile via CSS.

### Color contrast

Unclear from code alone. Would require visual testing. Dark mode uses `#e8edf5` text on `#0f172a` background (high contrast). Light mode uses `#0a1628` on `#f0f4f8` (high contrast). Muted text colors (`#64748b`, `#475569`) against dark backgrounds would need verification.

---

## 17. Anything Unusual

### No `unsafe` usage

The word "unsafe" appears only as a string literal in the Rust keyword list for the syntax highlighter (`src/utils.rs:87`).

### Leaked closures

Three `Closure::forget()` calls create permanent event listeners that are never cleaned up:
- `src/components/layout.rs:26` — scroll listener for reading progress
- `src/components/nav.rs:176` — scroll listener for BackToTop
- `src/components/nav.rs:329` — keydown listener for KeyboardNav

### `ReadingProgress` component is defined but not mounted

`ReadingProgress` (`src/components/layout.rs:10`) creates a scroll listener and updates `ReadProgressSignals`, but it is never rendered in the component tree. `BackToTop` reads `ReadProgressSignals.progress` but this signal will never be updated unless `ReadingProgress` is rendered somewhere.

### CI rustflags override

The CI workflow sets `RUSTFLAGS="-C target-feature=+bulk-memory"` as an environment variable. Because Cargo uses only one source of rustflags (environment takes precedence over config file), the linker args in `.cargo/config.toml` (`--no-entry`, `--import-memory`, `stack-size=1048576`) are not applied during CI builds. Local builds (without the env var) do apply the config.toml flags.

### Missing Trunk `copy-dir` for docs

`static/docs/` (4 JSON files) has no corresponding `<link data-trunk rel="copy-dir" href="static/docs"/>` in `index.html`. The `ProjectDocsPage` fetches `/docs/{slug}.json` at runtime, but these files will not be present in the Trunk build output.

### Two `robots.txt` files

`robots.txt` (root) and `public/robots.txt` both exist. The root file is explicitly copied by Trunk's copy-file directive. The public/ version is served via the [[assets]] directive. The root version should win in the final build.

### Duplicate `gloo-net` in Cargo.lock

Both `gloo-net` 0.5.0 and 0.6.0 appear in Cargo.lock. The 0.6.0 version is pulled in as a transitive dependency; the project directly depends on 0.5.

### Clipboard access pattern repeated in 7+ files

The `js_sys` call to `navigator.clipboard.writeText(...)` is repeated verbatim in `home.rs`, `about.rs`, `resume.rs` (x2), `one_pager.rs`, `contact.rs`, and `project.rs`. Each wraps the call in the same catch-all error suppression.

### `HtmlIFrameElement` and `IntersectionObserver` features enabled but unused

`web-sys` features list in `Cargo.toml` includes `HtmlIFrameElement`, `IntersectionObserver`, `IntersectionObserverEntry`, `IntersectionObserverInit`, and `MediaQueryList`. None of these types appear in any Rust source file. They may have been used previously or planned for future use.

### `getrandom` version mismatch

Both `getrandom` 0.2.17 and 0.4.2 appear in Cargo.lock. The project directly depends on 0.2 with the `js` feature. The 0.4 version is a transitive dependency.

### Content string replacements in writeup rendering

`WriteupDetailPage` (`src/pages/writing.rs:269-279`) applies 10 string replacements to writeup content at render time. These transform internal markup conventions into display-friendly text.

### `data-wasm-opt="0"` disables WASM optimization

The Trunk directive `<link data-trunk rel="rust" data-wasm-opt="0" />` sets wasm-opt level to 0 (no optimization). The WASM binary ships without wasm-opt optimization despite the Cargo.toml comment mentioning wasm-opt.

### No tests

No `#[test]`, `#[cfg(test)]`, or test files exist anywhere in the source tree. No `tests/` directory.

### Module structure

The `mod` declarations are in `src/lib.rs` (`mod data; mod utils; mod components; mod pages; mod error; mod state; mod db;`). The `pages`, `components`, and `data` modules use directory-based module structure with `mod.rs` files. `src/bin/ssg.rs` is a separate binary target gated behind the `ssg` feature.

---

## 18. Post-Inventory Additions (since commit `118b657`)

Files created after INVENTORY.md was generated. Inventory base commit: `118b657`.

### New source modules

| File | Description |
|---|---|
| `src/error.rs` | `AppError` enum — recoverable error taxonomy for fetch/parse boundaries and `ErrorBoundary` integration; `thiserror`-derived, `serde`-serializable for use in `create_resource` |
| `src/state.rs` | `GlobalAppState` — root reactive context struct consolidating all 5 `provide_context` calls previously scattered across `src/main.rs`; theme, global UI toggles, project filtering, and shared layout signals |
| `src/db.rs` | In-browser portfolio index backed by SQLite via `sqlite-wasm-rs` memory VFS; feature-gated behind `sqlite`; ~422 lines; structured fallback path for non-WASM or non-`sqlite` builds; `AtomicU64` perf counters for query and enrichment latency |
| `src/bin/ssg.rs` | Static-site-generator binary: pre-renders 27 route shells via `leptos::ssr::render_to_string` + `tokio::task::LocalSet`; feature-gated behind `ssg`; run as `cargo run --features ssg --bin ssg` |
| `src/components/terminal.rs` | `Terminal` component extracted from `src/pages/home.rs`; interactive boot-and-command terminal with 5-line boot sequence, command dispatch table, and `use_navigate` routing |

### Data layer split (`src/data/`)

`src/data.rs` (previously monolithic) has been replaced by a directory module. The individual files are:

| File | Description |
|---|---|
| `src/data/mod.rs` | Re-exports all public items from submodules; defines shared types `ProjectCardSignals` and `ReadProgressSignals` |
| `src/data/projects.rs` | `PROJECTS: LazyLock<Vec<ProjectIndex>>` — 4 compiled-in project index entries; `ProjectIndex`, `ProjectDetail`, `ProjectCategory` types; `find_project`, `get_infrastructure_fleet` helpers |
| `src/data/writeups.rs` | `WRITEUPS: LazyLock<Vec<WriteUpIndex>>` — 17 compiled-in writeup index entries; `WriteUpIndex`, `WriteUpDetail` types |
| `src/data/certs.rs` | `CERTIFICATIONS: LazyLock<Vec<Certification>>` — 5 compiled-in certification entries; `Certification` type |
| `src/data/tests.rs` | `#[cfg(test)]` unit tests for `find_project`, project index correctness, and certification data; 5 tests total |
