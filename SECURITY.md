# Security Policy

## Supported Versions

This repository builds a static WASM bundle deployed to GitHub Pages. There is **no server-side execution surface** — no backend, no database, no authenticated endpoints, no user session state. The only actively supported version is the current deploy of the `main` branch at https://richmuscle.github.io/.

| Version | Supported |
|---|---|
| `main` (live) | ✅ |
| `revamp` (staging) | ✅ |
| Historical tags | ❌ |

## Reporting a Vulnerability

If you believe you have found a security issue:

- **Email:** Richard.Mussell@yahoo.com
- **Response window:** within 72 hours of report
- **Do not** open a public GitHub issue or pull request describing the vulnerability before it is acknowledged and resolved.

Please include reproduction steps, affected route(s), the deployed commit SHA if observable, and any suggested remediation.

## Security Practices

- **Content Security Policy** — enforced via `<meta http-equiv="Content-Security-Policy">` in `index.html`. Inline scripts are limited to a small Trunk/WASM preload shim; no third-party script origins are permitted by default.
- **Zero user data collection** — the site does not collect, store, or transmit user data. No cookies for tracking, no analytics beacons, no forms that POST to any backend. The only "state" persisted client-side is a theme preference in `localStorage`.
- **Memory safety** — the runtime is written in Rust, compiled to `wasm32-unknown-unknown`. Buffer overflows, use-after-free, and data races in safe Rust are prevented by the compiler. The single `unsafe` region is `src/db.rs` (sqlite-wasm-rs FFI for in-browser full-text search), narrowly scoped to the documented prepare→bind→step→finalize lifecycle with `sqlite3_close` on every error path.
- **Supply chain** — dependency tree is audited via [`cargo audit`](https://rustsec.org/) against the RustSec advisory database.

## Dependency Audit

Last run: 2026-04-15 on `revamp` @ `8d23fe1`.

**Result: PASS** — 0 vulnerabilities, 4 informational warnings (all transitive dependencies of the Leptos 0.6 ecosystem; no direct action available until the Leptos 0.7+ upgrade).

| Advisory | Crate | Type | Path |
|---|---|---|---|
| [RUSTSEC-2024-0384](https://rustsec.org/advisories/RUSTSEC-2024-0384) | `instant` | unmaintained | transitive via Leptos |
| [RUSTSEC-2024-0436](https://rustsec.org/advisories/RUSTSEC-2024-0436) | `paste` | unmaintained | transitive via Leptos macros |
| [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370) | `proc-macro-error` | unmaintained | transitive via Leptos macros |
| [RUSTSEC-2026-0002](https://rustsec.org/advisories/RUSTSEC-2026-0002) | `lru` | unsoundness (IterMut) | transitive via `leptos_router` |

Reproduce locally:

```bash
cargo install cargo-audit  # one-time
cargo audit
```
