# richardmussell.github.io

> **Systems Administrator & DevOps Engineer** — Portfolio site built with Rust, Leptos 0.6, and WASM.  
> Live at **[richmuscle.github.io](https://richmuscle.github.io)**

![CI](https://github.com/richmuscle/richmuscle.github.io/actions/workflows/deploy.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.77%2B-orange?logo=rust)
![WASM](https://img.shields.io/badge/target-wasm32--unknown--unknown-blue?logo=webassembly)
![License](https://img.shields.io/badge/license-MIT-green)

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
│   ├── terminal.rs      # Animated terminal component (HomePage)
│   └── nav.rs           # Navigation with resume download
├── pages/               # Route-level page components
│   ├── home.rs
│   ├── projects.rs      # Error boundary wired
│   ├── writing.rs       # Error boundary wired
│   ├── resume.rs
│   └── contact.rs
├── data/                # Content module — split by domain
│   ├── projects.rs
│   ├── writeups.rs
│   ├── certs.rs
│   └── mod.rs
├── state.rs             # GlobalAppState — unified provide_context
├── error.rs             # AppError (thiserror)
└── ssg.rs               # SSG pipeline (opt-in)

styles/
├── tokens/              # 85-token CSS variable system
├── base/                # Reset, typography, layout
├── components/          # Component-scoped styles
└── pages/               # Page-scoped styles
```

**Key design decisions** (full ADR log in `CLAUDE.md`):

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
# → http://localhost:8080
```

**Run checks (both gates must pass before any deploy):**

```bash
# CSR/WASM gate
cargo check --target wasm32-unknown-unknown

# SSR gate
cargo check --features ssr
```

**Run tests:**

```bash
cargo test
```

---

## Deployment

Deploys automatically on push to `revamp` via GitHub Actions. The workflow:

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
| Resume PDF | ✅ Live at `/pdfs/resume.pdf` |
| Mobile (P0) | ✅ Fixed |
| Security headers | ✅ CSP meta tag, frame-ancestors |
| Unit tests | ✅ 5 passing (`src/data/tests.rs`) |
| SSG pipeline | 🔄 Wired, end-to-end validation pending |

---

## About

**Richard J. Mussell** — Systems Administrator & DevOps Engineer based in Oklahoma City, OK.

BS in IT & Administrative Management (Cybersecurity specialization) from Central Washington University. Hands-on experience in SOC operations, enterprise identity infrastructure, and platform engineering. Pursuing GCP Associate Cloud Engineer and CKA.

→ [richmuscle.github.io](https://richmuscle.github.io) · [richard.mussell@yahoo.com](mailto:richard.mussell@yahoo.com)
