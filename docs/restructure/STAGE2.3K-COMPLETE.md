# Stage 2.3K Complete — Internal Resume Variants PISCES Alignment + Banned-Verb Scrub

Single-commit stage. Three resume variant files under `docs/resumes/` had two problems: banned-verb residue inherited from pre-2.3 drafts, and PISCES sections that overclaimed detection authorship, shift ownership, and stakeholder brief production — the same pattern 2.3J closed on the live resume page. This stage brings all three variants into alignment with the about-page anchor and the live `resume.rs` PISCES framing.

Closes audit P1-x (internal resume variants) from `docs/audits/2026-04-20-staff-lens.md`.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `8851d1d` | `docs/resumes/{resume-devops,resume-platform,resume-sysadmin}.md` | Banned-verb edits on devops (L42, L44); PISCES section rewrite across all three variants to match 2.3J anchor |

## Scope expansion named before drafting

The original briefing described 2.3K as a "ten-minute mechanical edit" — two `Engineered` hits on `resume-devops.md` L42 and L44. Recon showed the actual scope was wider:

1. Three resume variant files exist, not one (`resume-devops.md`, `resume-platform.md`, `resume-sysadmin.md`).
2. All three PISCES sections overclaimed against the 2.3J anchor. Severity varied: devops claimed "Owned ELK Stack monitoring" (one overclaim); platform claimed triaged/escalated/authored detections (three overclaims); sysadmin claimed triage volume metric, 15-minute escalation SLA, RBAC hygiene rounds, and stakeholder brief authorship (four overclaims).
3. Two of the three files were drafted before 2.3J closed P0-1 and had never been swept.

Scope expanded to "banned-verb scrub + PISCES framing alignment across all three variants" with owner approval before draft. Named-before-drafted is the 2.3C pattern (six numbers → eight).

## What changed

### `resume-devops.md` — banned verbs

Before:
- L42: "engineered PromQL alerting rules"
- L44: "**Engineered a zero-trust administrative fabric**"

After:
- L42: "wrote PromQL alerting rules"
- L44: "**Built a zero-trust administrative fabric**"

### All three variants — PISCES section rewrite

Anchor (from `resume.rs`, calibrated in 2.3J against `about.rs`):
> "Academic SOC rotation — exposure to production SOC operations, not operator tenure. Observed analysts triaging alerts across 13 municipal network feeds on the ELK stack."

**`resume-devops.md`** — removed "Owned ELK Stack monitoring", removed stakeholder-brief authorship claim, removed "Applied KQL-based log analysis" self-attribution. Replaced with observer-framed scope mirroring the live page: observed triage, built Kibana visualizations, worked the ticketing system, explicit non-claims on detections/shifts/briefs.

**`resume-platform.md`** — removed "Triaged and escalated alerts", "authored KQL-based detection queries", and "Produced incident summary briefs". Replaced with the same observer-framed three-bullet structure.

**`resume-sysadmin.md`** — removed triage volume metric (~30–50 alerts/shift), 15-minute escalation SLA claim, "Enforced RBAC review workflows" / "flagged stale service accounts" (confirmed overclaim by owner during scope review), and stakeholder brief authorship. Replaced with the same observer-framed structure.

## Cross-surface consistency after this stage

| Surface | PISCES framing | Status |
|---|---|---|
| `about.rs` | "academic rotation…did not own detections or run shifts at volume" | Anchor (calibrated in 2.3A) |
| `resume.rs` | "Academic SOC rotation — exposure…not operator tenure" | Aligned (2.3J) |
| `home.rs` | "Monitored 13 municipal entities" (neutral) | Consistent |
| `one_pager.rs` | "SOC internship monitoring 13 municipal entities" (neutral) | Consistent |
| `resume-devops.md` | "Academic SOC rotation — exposure…not operator tenure" | **Aligned** (this stage) |
| `resume-platform.md` | "Academic SOC rotation — exposure…not operator tenure" | **Aligned** (this stage) |
| `resume-sysadmin.md` | "Academic SOC rotation — exposure…not operator tenure" | **Aligned** (this stage) |

PISCES framing is now consistent across seven surfaces. Any future PISCES edit must treat `about.rs` as the anchor and propagate the same scope-honest language.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 15/15 passing
- trunk build --release: success

Post-apply greps confirmed:
- `resume-devops.md`: zero "engineered" or "owned" hits in resume prose
- `resume-platform.md`: zero "triaged"/"escalated"/"authored" hits in PISCES section
- `resume-sysadmin.md`: zero "triaged"/"escalated"/"authored"/"enforced rbac" hits in PISCES section

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. 2.3K extends P0-1's cross-surface PISCES alignment into the internal resume variants that were drafted before P0-1 closed. Contributes to `positioning_alignment` and `cross_surface_consistency` lenses.

## Remaining stages

| Stage | Priority | Title | Status |
|---|---|---|---|
| 2.3K | P1 | Internal resume variants | **Done** (this stage) |
| ROADMAP regen | — | Documentation hygiene | Open |
| Dead CSS cleanup | — | Unused cert-* classes + `.about-pills-row` | Open |
| About-page dashboards-to-tickets tightening | — | Small content edit | Open |
| Terraform "1 → 0 by design" verification | — | Config evidence trace | Open |
| 2.3G | P3 | OG image regeneration | Open |
| 2.3F | P1 | KEEP-writeup voice pass | Open |
| 2.3D | P1 | InDevelopment project content voice | Deferred — blocked on eight-repo triage |
| 2.3E | P3 | Platform page build | Blocked on 2.3D |

## Discipline carrying forward

When a briefing describes a stage as "mechanical" and "ten minutes," recon still runs first. The 2.3K scope was wider than the briefing said by a factor of three (three files, not one) and deeper (framing alignment, not just verb swap). The honest move is to name the expansion before drafting, not to either silently expand or silently contract. Same discipline that held on 2.3C when "six numbers" became "eight."

Cross-surface anchors compound. Once `about.rs` was calibrated (2.3A) and `resume.rs` followed (2.3J), every subsequent PISCES edit has a two-surface anchor to triangulate against. The resume variants in this stage didn't require fresh calibration — they inherited the framing that was already load-bearing on the live site.
