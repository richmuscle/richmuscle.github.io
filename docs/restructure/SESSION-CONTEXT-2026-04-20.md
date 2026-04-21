# Session Context — 2026-04-20

Context that doesn't live in other closeout docs but future sessions need.

## PISCES scope — owner's own description

Stage 2.3A's about page rewrite recalibrated PISCES from "SOC analyst experience monitoring 13 municipal government networks" to "academic SOC rotation, not operator tenure." This was based on the owner's own statement:

> "i honestly didnt do much in the soc cwu exprience, i made some visulizaTIONS, used the ticketing sytstem, didnt really do much work, jsut exposure to how it all works, elastic stack"

The about page now accurately reflects this scope. Future stages that touch PISCES content (2.3J resume alignment, any one-pager or home-page edit) must match the about page's framing, not the resume's current inflated bullets.

Specifically the owner did NOT do:
- Author detection logic (KQL or otherwise)
- Triage at volume
- Produce stakeholder briefs
- Run shifts
- Own incident response

The owner DID do:
- Build visualizations on the ELK stack
- Use the ticketing system
- Observe analysts triaging alerts across municipal networks
- Get exposure to how a SOC functions in practice

## Cert status — owner's decision

Owner is not pursuing certs right now. Stage 2.3A dropped cert language from the about page entirely. Stages that touch cert content (2.3H sweep across certs.rs, home.rs, resume.rs, one_pager.rs) should either:

1. Remove cert claims site-wide (match the about page's move), or
2. Keep only completed coursework (Cisco CCNA 2018–2019).

"Pursuing GCP ACE" and "CKA target Q3/Q4 2026" are not honest current-state and should not survive 2.3H.

## Positioning — no-tier declaration

CLAUDE.md was updated in Stage 2.3A: "Target level is not declared on the site." The owner is open to senior, mid, or entry placements. The staff-engineer lens applies to how work is documented; the role tier is the hiring team's decision.

Stage 2.3I sweeps "senior" residue across home, resume, one-pager, and contact. Any surface that declares a tier needs tier-neutral framing.

## Voice anchor

The about page (`src/pages/about.rs`) and the writing page intro (`src/pages/writing.rs:68`) are the voice calibration reference. Stages that touch other user-visible surfaces should measure against these, not against the older security-baseline-audit case study JSON (which was the anchor during Stage 2.2B but has since been superseded).

## Audit state — 2026-04-20 staff-lens audit

Composite: 6.2/10. Findings in `docs/audits/2026-04-20-staff-lens.md`.

Top three findings from audit:
1. **P0** — PISCES contradiction between about page and resume. Addressed by Stage 2.3J.
2. **P0-adjacent** — Unsupported metrics in two InDevelopment project JSONs. Addressed by Stage 2.3C.
3. **P1** — Cert inconsistency across four surfaces. Addressed by Stage 2.3H.

Audit-recommended stage ordering, reconciled with closeout-declared ordering:
1. 2.3J first (only P0, closes the contradiction)
2. 2.3C second (heaviest-weighted rubric lens, score-mover)
3. 2.3H third (P1, mechanical once direction is decided)
4. 2.3I, 2.3D, 2.3G, 2.3L, 2.3F, 2.3K in rough order
5. 2.3E and 2.4 are longer-horizon builds
