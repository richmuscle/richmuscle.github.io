# The Professional Substrate
🏛️ RICHARD J. MUSSELL | Infrastructure & Platform Operations

Designation: Systems Administrator | Oklahoma City, OK

Thesis: "Bridging high-stakes physical operational discipline with elite infrastructure automation. Engineering the convergence of legacy hardware reliability and modern cloud-native orchestration."

## System Architecture (The "Correctness" Layer)
Framework: Rust-based Single Page Application (SPA) via Leptos.

Runtime: WebAssembly (WASM32-unknown-unknown) target for high-performance client-side hydration.

Build Strategy: Trunk v0.21+ utilizing LTO Fat and opt-level `z` for minimal binary footprint (<50ms initialization).

Reproducibility: Hermetic development environment provisioned via Nix Flake (flake.nix).

CI/CD: Automated GitHub Actions pipeline with bulk-memory support and state-safe deployment to GitHub Pages.

## The Fleet: Engineering Specifications
Operational Modules:
- Hardened Cloud Landing Zone (IaC): Deterministic GCP provisioning via Terraform. Implements GCS state-locking and NIST 800-53 compliant private-first networking.
- Systems Lifecycle Automation Framework: Idempotent POSIX-compliant Bash framework. Automated RBAC provisioning and CIS-standard system hardening to eliminate configuration drift.
- Multi-Tier Strategic Observability Pipeline: Unified telemetry via Prometheus and ELK Stack. Maps technical heuristics to operational SLO/SLI targets with alert cardinality management.
- Zero-Trust Administrative Fabric (ZTNA): Identity-governed SASE architecture via WireGuard and AWS. Features MSS clamping for packet stability and AD-integrated access revocation.

## Operational Directives (Local Development)
```bash
# Enter the hermetic build environment
nix develop

# Initialize development server
trunk serve

# Execute production build
trunk build --release
```

## Deploy
Deploy: Push to main -> GitHub Actions builds and deploys to GitHub Pages.

## Engineering Fidelity Specs:
Hydration Latency: <50ms.
Console Hygiene: Zero-Warning / Zero-Error Production State.
Typography: Inter & JetBrains Mono (Mobile Optimized @ 44px hit areas).
Fidelity: Print-ready CSS for LaTeX-standard document generation.
