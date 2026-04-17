# richardmussell.github.io

> **Systems Administrator & DevOps Engineer** — Portfolio site built with Rust, Leptos 0.6, and WASM.  
> Live at **[richmuscle.github.io](https://richmuscle.github.io)**

![CI](https://github.com/richmuscle/richmuscle.github.io/actions/workflows/deploy.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.77%2B-orange?logo=rust)
![WASM](https://img.shields.io/badge/target-wasm32--unknown--unknown-blue?logo=webassembly)

---

## What This Is

A production portfolio site written entirely in Rust — no JavaScript framework, no Node toolchain. Compiled to WebAssembly and deployed to GitHub Pages via CI/CD. Built to demonstrate the same rigor I apply to infrastructure: layered architecture, reproducible builds, and zero-compromise deployment pipelines.

---

## Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (stable) |
| UI Framework | Leptos 0.6 (fine-grained reactivity) |
| Compile Target | `wasm32-unknown-unknown` |
| Bundler | Trunk |
| Styling | SCSS → layered CSS (tokens / base / components / pages) |
| Search | sqlite-wasm-rs 0.5.2 (pure-Rust WASM SQLite) |
| CI/CD | GitHub Actions → GitHub Pages |
| Error Handling | `thiserror` + per-section error boundaries |

---

## Architecture

```
src/
├── components/          # Reusable UI primitives
│   ├── nav.rs           # Navigation + keyboard chords
│   ├── palette.rs       # Command palette (⌘/Ctrl+K)
│   ├── project.rs       # ProjectCard + CodeBlock
│   ├── layout.rs        # Reading-progress signals
│   ├── site_footer.rs
│   ├── error_fallback.rs
│   └── mod.rs
├── pages/               # Route-level page components
│   ├── home.rs
│   ├── about.rs
│   ├── writing.rs       # Error boundary wired
│   ├── project/         # Case Study / Docs / Demo surfaces
│   │   ├── detail.rs
│   │   ├── docs.rs
│   │   ├── demo.rs
│   │   └── (module root in src/pages/project.rs — shared tabs + meta strip)
│   ├── resume.rs
│   ├── contact.rs
│   ├── telemetry.rs
│   ├── one_pager.rs
│   ├── not_found.rs
│   └── mod.rs
├── data/                # Content module — split by domain
│   ├── projects.rs
│   ├── writeups.rs
│   ├── certs.rs
│   ├── tests.rs         # 10 unit tests covering data integrity
│   └── mod.rs
├── state.rs             # GlobalAppState — unified provide_context
├── error.rs             # AppError (thiserror)
├── db.rs                # sqlite-wasm-rs portfolio index + pure-Rust fallback
├── utils.rs             # Browser helpers, syntax highlighters, HTML escape
├── lib.rs               # Crate root + `App` component
├── main.rs              # WASM entry point
└── bin/
    └── ssg.rs           # Static-site-generator binary (feature = "ssg")

style/
├── tokens.css           # Design token CSS variable system
├── base.css             # Reset, typography, global rules
├── style.scss           # Entry point — imports the layered files below
├── components/          # Component-scoped styles
└── pages/               # Page-scoped styles
```

**Key design decisions** (full ADR log in `docs/DECISIONS.md`):

- **CSR deploy** — WASM hydration via Trunk, no SSR runtime required on GitHub Pages
- **Single context** — `GlobalAppState` replaces five scattered `provide_context` calls
- **Content-first** — features are gated behind content completeness, not the other way around
- **Opt-in SSR/SSG** — `default = ["csr", "sqlite"]`, hydrate/ssr/ssg are feature flags

---

## Local Development

**Prerequisites:** Rust stable, `wasm32-unknown-unknown` target, Trunk, `sass`

```bash
# Install target and bundler
rustup target add wasm32-unknown-unknown
cargo install trunk

# Clone and run
git clone https://github.com/richmuscle/richmuscle.github.io.git
cd richmuscle.github.io
trunk serve
# → http://localhost:8002
```

**Run checks (all four gates must pass before any deploy — mirrors `.github/workflows/ci.yml`):**

```bash
# 1. CSR default, wasm32
cargo check --target wasm32-unknown-unknown

# 2. SSR host-only
cargo check --no-default-features --features ssr

# 3. Hydrate + sqlite, wasm32
cargo check --no-default-features --features "hydrate sqlite" --target wasm32-unknown-unknown

# 4. SSG binary compile gate
cargo check --features ssg --bin ssg
```

The `justfile` wraps all four into `just check`; `just lint` runs `cargo fmt --check` + clippy; `just test` runs the unit suite.

**Run tests:**

```bash
cargo test --no-default-features --features ssr
```

---

## Deployment

Deploys automatically on push to `main` via GitHub Actions. The workflow:

1. Installs Rust stable + `wasm32-unknown-unknown` target
2. Runs both `cargo check` gates
3. Builds with `trunk build --release`
4. Pushes `dist/` to `gh-pages` branch

Manual deploy:

```bash
trunk build --release
# dist/ is ready for static hosting
```

---

## Project Status

| Area | Status |
|------|--------|
| CI/CD pipeline | ✅ Green |
| WASM build | ✅ Passing |
| SSR gate | ✅ Passing (0 errors) |
| SSG compile gate | ✅ Passing (runtime blocked by Leptos 0.6 upstream) |
| Resume PDF | ✅ Live at `/pdfs/resume.pdf` (120KB, LaTeX-compiled) |
| Mobile (P0) | ✅ Fixed |
| Security headers | ✅ CSP meta tag, frame-ancestors |
| Unit tests | ✅ 10 passing (`src/data/tests.rs`) |
| rust-toolchain.toml | ✅ Pinned stable + wasm32 target |
| justfile | ✅ 7 recipes (serve, check, test, build, deploy, lint) |
| SECURITY.md | ✅ 0 vulnerabilities (cargo audit clean) |
| Telemetry dashboard | ✅ Threshold-based status + LCP metric |
| cargo fmt | ✅ Clean |
| SSG pipeline | 🔄 Wired, end-to-end validation pending |

---

## About

**Richard J. Mussell** — Systems Administrator & DevOps Engineer based in Oklahoma City, OK.

BS in IT & Administrative Management (Cybersecurity specialization) from Central Washington University. Hands-on experience in SOC operations, enterprise identity infrastructure, and platform engineering. Pursuing GCP Associate Cloud Engineer and CKA.

→ [richmuscle.github.io](https://richmuscle.github.io) · [richard.mussell@yahoo.com](mailto:richard.mussell@yahoo.com)
