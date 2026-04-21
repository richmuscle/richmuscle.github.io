# Stage 2.3C Complete — Unsupported Metrics Resolution

Single-commit stage. Both InDevelopment project JSONs made numeric claims without methodology. The audit flagged six; reading the full files found eight. Reframed all eight to either design targets (performance claims) or by-design structural claims (configuration properties). Closes audit P0-2 and P0-3 from `docs/audits/2026-04-20-staff-lens.md`.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `4afcbdf` | public/projects/identity-access-lifecycle.json | 5 string replacements qualifying metrics |
| `4afcbdf` | public/projects/observability-operational-intelligence.json | 3 string replacements qualifying metrics |

## What changed

### Performance claims reframed as design targets (5)

| File | Claim | Before | After |
|---|---|---|---|
| identity | 80% handshake latency | "reduce handshake latency by 80% vs. OpenVPN" | "design target of ~80% handshake latency reduction vs. OpenVPN (unbenchmarked; target based on protocol characteristics, not measured on this deployment)" |
| identity | <15ms session overhead | "Administrative session overhead" | "Administrative session overhead (design target, unbenchmarked)" |
| observability | Minutes → Seconds MTTR | "MTTR Reduction" | "MTTR Reduction (design target, unbenchmarked)" |
| observability | -60% alert noise | "Non-actionable Alert Noise" | "Non-actionable Alert Noise (design target, unbenchmarked)" |
| observability | 100% coverage | "Full-stack Observability Coverage" | "Full-stack Observability Coverage (design target)" |

### Structural claims qualified with "by design" (3)

| File | Claim | Before | After |
|---|---|---|---|
| identity | 1 → 0 public entry points | "Public administrative entry points (Bastion -> Dark Node)" | "Public administrative entry points by design (Bastion -> Dark Node)" |
| identity | 100% compliance audit | "Compliance audit trail of peer-to-peer data transfer via centralized logging" | "Compliance audit trail by design (every peer-to-peer transfer routed through centralized logging)" |
| identity | 100% stability | "ensuring 100% stability for high-bandwidth telemetry data" | "with the design intent of eliminating MTU-induced fragmentation on high-bandwidth telemetry paths" |

### Why the split matters

Performance claims verify against measurement (benchmarks, load tests, production metrics). Structural claims verify against configuration (Terraform state, Security Group rules, logging pipeline topology). Uniformly labeling everything "design target" would conflate two different evidence standards. A future verification pass on the identity project can confirm "1 → 0 public entry points" by grepping Terraform for zero-public-listener enforcement — that's a config check, not a benchmark. The 80% latency claim requires a WireGuard-vs-OpenVPN benchmark on the actual deployment, which is a different class of work.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 18/18 passing
- trunk build --release: success
- Numeric claims grep: all 8 hits appear with "design target", "by design", "design intent", or "unbenchmarked" qualifier
- JSON validity: both files parse as valid JSON

## Remaining stages

| Stage | Priority | Title | Status |
|---|---|---|---|
| 2.3H | P1 | Cert honesty sweep | Open |
| 2.3D | P1 | InDevelopment project content voice | Open |
| 2.3F | P1 | KEEP-writeup voice pass | Open |
| 2.3K | P1 | Internal resume variants | Open |
| 2.3G | P3 | OG image regeneration | Open |
| 2.3E | P3 | Platform page build | Blocked on 2.3D |
| 2.3I | Low | Senior-tier residue sweep | Pending confirmation grep (audit indicated likely resolved) |

Stage 2.3D is the next logical step for these two files — it will rewrite the pd-challenge bodies wholesale to match the security-baseline-audit template. This stage's metric qualifiers may be partially superseded by 2.3D's broader rewrite. That's expected: honesty calibration now, full template structure later.

Note: the "1 → 0 by design" claim is structurally true only if the deployment config matches. A future verification pass should grep the project's Terraform/config for zero-public-listener enforcement. Flagged as follow-up, not in 2.3C scope.

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. Stage 2.3J closed P0-1 (PISCES resume contradiction). Stage 2.3M closed the deepest P0-adjacent voice break (PISCES writeup body). This stage closes P0-2 and P0-3 (unsupported metrics in both InDevelopment project pages). All three staff-lens audit P0s are now resolved. A re-audit should show meaningful movement on the honesty_discipline lens (weight 22).

## Discipline carrying forward

Reading the full file before scoping — the audit caught six claims, the file contained eight. Audit findings set direction; reading the source sets scope.

Not all unverified claims are equal — the performance-vs-structural distinction saved one claim ("1 → 0 public entry points") from being over-qualified as "unbenchmarked" when it is actually verifiable against config.
