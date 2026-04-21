# Stage 2.3J Complete — Resume PISCES Framing Alignment

Single-commit stage. The four PISCES timeline bullets on the resume page overclaimed detection authorship, shift ownership, and stakeholder brief production that the about page explicitly disclaimed. This commit replaces all four with scope-honest versions calibrated against the about-page voice anchor.

Closes audit P0-1 from `docs/audits/2026-04-20-staff-lens.md` (cross-surface contradiction between about and resume).

## Commit

| Commit | Scope | Change |
|---|---|---|
| `85e5a06` | resume.rs:97-100 | Replace 4 PISCES bullets with scope-honest versions matching about-page framing |

## What changed

Before:
- "Triaged ~50 alerts/day...escalated ~3-5 incidents/week to senior analysts"
- "Authored KQL detection logic for anomalous endpoint behavior"
- "Produced weekly incident briefs...for SOC leadership and municipal stakeholders"
- "Maintained ELK Stack dashboards and correlation searches during shift rotations"

After:
- "Academic SOC rotation — exposure to production SOC operations, not operator tenure"
- "Built visualizations in Kibana...as the rotation's hands-on deliverable"
- "Worked the ticketing system alongside the analyst shift...tracking how cases moved from alert to resolution"
- "Did not author detections, run shifts, or produce stakeholder briefs. What the rotation taught was how a SOC functions in practice"

## Cross-surface consistency after this stage

| Surface | PISCES framing | Status |
|---|---|---|
| about.rs | "academic rotation...did not own detections or run shifts at volume" | Anchor (calibrated in 2.3A) |
| resume.rs | "Academic SOC rotation — exposure...not operator tenure" | **Aligned** (this stage) |
| home.rs | "Monitored 13 municipal entities" (neutral) | Consistent |
| one_pager.rs | "SOC internship monitoring 13 municipal entities" (neutral) | Consistent |

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. P0-1 (PISCES contradiction) is closed by this stage. P0-2 and P0-3 (both unsupported-metrics findings for identity and observability projects) remain open for Stage 2.3C. A re-audit after 2.3C should move the composite meaningfully — that gives the next session a score-delta baseline to measure against.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 18/18 passing
- trunk build --release: success

## Remaining stages

| Stage | Priority | Title | Status |
|---|---|---|---|
| 2.3C | P0 | Unsupported metrics resolution (identity + observability) | Open |
| 2.3H | P1 | Cert honesty sweep | Open |
| 2.3D | P1 | InDevelopment project content voice | Open |
| 2.3F | P1 | KEEP-writeup voice pass | Open |
| 2.3L | P1 | Writeup body author-line sweep | Open |
| 2.3G | P3 | OG image regeneration | Open |
| 2.3E | P3 | Platform page build | Blocked on 2.3D |
| 2.3I | Low | Senior-tier residue sweep | Open (may already be resolved) |
| 2.3K | P1 | Internal resume variants | Open — promoted from Low for consistency with 2.3L; both are banned-word residue on different surfaces |

## Note on test invocation

The user's request listed `cargo test` as a gate. The correct invocation for this repo is `cargo test --no-default-features --features ssr` (per the justfile `test` recipe). WASM-only deps (`gloo-net`, `gloo-timers`, `js-sys`, `web-sys`) are gated behind `cfg(target_arch = "wasm32")` in Cargo.toml and unavailable on the native test target without the SSR feature providing stub paths.

## Discipline carrying forward

Honest rewrites can contract content, not just reframe it. The resume PISCES entry is shorter and more explicit after this stage than before. Stages 2.3H (cert sweep) and 2.3I (senior-tier sweep) may produce similar contractions — "the page got shorter and more honest" is a success condition for this series, not a regression.

Cross-surface matrices belong in closeouts that resolve cross-surface findings. The audit's matrix format carried through; reusing it here makes the next audit's diff mechanical.
