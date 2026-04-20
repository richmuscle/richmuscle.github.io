# Stage 2.2B Complete — security-baseline-audit voice reframe

Four commits, one section per commit, owner-approved before each apply.

| Commit | Section | Change |
|---|---|---|
| `34c4e0a` | Problem | Reframed from provisioning-speed lead to compliance-gap lead. First-person voice. "Provisioning was the starting state, not the point." |
| `5f890fc` | Approach | Reframed from module-list topology to gate-apply-detect compliance loop. Infrastructure positioned as substrate. |
| `ef04345` | Hero metric #1 | Replaced "Environment provisioning time (~4h→5min)" with "Drift detection latency (≤24h nightly cron)". Provisioning metric retained in Outcomes. |
| `47f90f4` | Outcome #1 label | "Environment provisioning time" → "Time to compliant baseline". Same data, compliance framing. |

Sections verified as frame-neutral (no changes needed): Decisions (4 ADRs), Highlights (3 code snippets), Lessons (3 items), Constraints in/out, Artifact links, Demo walkthrough, Docs/Runbooks/Threat model.

All four gates green. 17/17 tests pass.

Deferred: body content of the retitled writeups still contains manifesto-voice prose ("Father, Creator, and Builder", "Golden Path", "Systems Sovereignty"). That is Stage 2.3 voice work on writeup bodies, not case-study reframing.
