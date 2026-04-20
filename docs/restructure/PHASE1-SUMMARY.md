# Phase 1 Summary — Structural Foundation

Completed: 2026-04-20
Branch: `revamp`
Commits: `7a78f83` (CLAUDE.md merge) → `389823b` (registry + fold + redirects)

---

## Initial state

- 4 shipped projects under old slugs, misaligned with canonical frame
- Positioning drift across 28+ locations ("Systems Administrator & DevOps Engineer" vs target "Linux Systems Administrator")
- 10 banned-word violations ("engineered")
- Scalability contract violated: adding a project required editing 8 files
- 6 conflicting identity sources across CLAUDE.md, sysadmin-portfolio.md, and site copy
- 17 writeup stubs, 9 with LLM-pretension titles

## Final state

- **3-project registry** with canonical slugs:
  - `security-baseline-audit` — **Shipped** (CIS compliance, Terraform gates, drift detection — absorbed terraform-gcp content)
  - `observability-operational-intelligence` — Operational (Prometheus/ELK/Grafana pipeline)
  - `identity-access-lifecycle` — Operational (WireGuard/AD zero-trust fabric)
- **4-entry redirect table** (LEGACY_REDIRECTS) handles all old slugs:
  - linux-admin-scripting → security-baseline-audit
  - zero-trust-networking → identity-access-lifecycle
  - monitoring-observability → observability-operational-intelligence
  - terraform-gcp → security-baseline-audit
- **Client-side redirects** with replace:true on all three project tabs (detail, docs, demo)
- **Compile-time redirect tests** verify array well-formedness (3 tests)
- **All four `just check` gates green**, 13 tests passing
- **CLAUDE.md consolidated**: identity, voice, word bans, project registry contract, case-study rules merged from the deleted sysadmin-portfolio.md
- **Sitemap** emits only canonical slugs
- **404.html → index.html** deep-link compatibility confirmed

## What changed in src/

| File | Change |
|---|---|
| `src/data/projects.rs` | 3 slug renames, 1 entry removed (terraform-gcp folded), redirect table + resolver + 3 tests, one-liners rewritten (banned words removed) |
| `src/pages/project/detail.rs` | Redirect check on legacy slugs |
| `src/pages/project/docs.rs` | Redirect check on legacy slugs |
| `src/pages/project/demo.rs` | Redirect check + removed dead terraform-gcp V1 fallback + updated slug matches |
| `src/pages/telemetry.rs` | Probe slugs updated |
| `src/pages/one_pager.rs` | Project entries updated to canonical slugs, banned words removed |

## Deferred to Phase 2

- **security-baseline-audit voice reframing** (HIGHEST PRIORITY): case study has strong content in landing-zone frame; needs retoning to continuous-audit frame. The registry subtitle/description were updated structurally; the JSON case-study prose still reads as GCP IaC, not security baseline.
- **Whole-site positioning sweep**: 28 locations still say "Systems Administrator & DevOps Engineer" — needs `PROFESSIONAL_TITLE` centralization and manual updates in index.html, README, manifest.json.
- **Banned-word sweep**: 10 instances of "engineered" identified in 01-current-state.md. 8 removed in this pass (projects.rs one-liners + one_pager entries). 2 remain in resume.rs hardcoded project summaries.
- **Writeup verdict execution**: 04-writeup-verdicts.md proposes 4 keep / 5 retitle / 3 demote / 5 retire. Owner must approve before execution.
- **Resume public/internal split**: 3 variants exist, proposal to surface sysadmin only.
- **Registry struct refactor**: ProjectStatus enum (Shipped/InArchitecture/Planned), produces/consumes arrays, ProjectDomain enum — designed in 02-target-structure.md, not yet implemented.
- **5 canonical projects not yet in registry**: endpoint-management-compliance, security-baseline (planned), backup-recovery-continuity, observability-operational-intelligence (already InArchitecture-equivalent), operational-foundation — all Planned status.
- **/platform page**: designed in 02-target-structure.md, requires registry struct refactor first.
- **Tech-stack mismatch flag**: security-baseline-audit registry entry shows Bash/Linux/Cron tech stack, but its JSON content describes Terraform/GCP/tfsec. Needs alignment in Phase 2.

## Documents produced

| File | Purpose |
|---|---|
| `docs/restructure/00-conflicts.md` | Pre-flight conflict inventory (10 conflicts, 6 decisions required) |
| `docs/restructure/01-current-state.md` | Information architecture, positioning drift, scalability-contract audit |
| `docs/restructure/02-target-structure.md` | Target route map, registry design, template taxonomy, /platform page |
| `docs/restructure/03-diff-plan.md` | File manifest, dependency ordering, change batches, kill-switch analysis |
| `docs/restructure/04-writeup-verdicts.md` | Per-writeup keep/retitle/retire/demote verdicts |
| `docs/restructure/PHASE1-SUMMARY.md` | This file |
