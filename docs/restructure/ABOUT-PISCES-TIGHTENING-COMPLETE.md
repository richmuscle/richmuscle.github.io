# About-Page PISCES Clause Tightening Complete

Single-commit stage. One-line edit to `src/pages/about.rs` line 27. Closes the follow-up flag raised in Stage 2.3M's closeout: the about page's PISCES clause understated first-line triage work by framing triage as analyst-only. Tightened to match actual scope without tipping into overclaim.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `3680146` | `src/pages/about.rs` line 27 | Two phrase replacements inside one sentence; 1 insertion, 1 deletion |

## What changed

Before:
> "…building visualizations on the ELK stack, working the ticketing system, and **watching analysts triage alerts** across thirteen municipal networks."

After:
> "…building visualizations on the ELK stack, working the ticketing system **as first-line triage**, and **watching analysts handle escalation** across thirteen municipal networks."

Two surgical changes:
- "working the ticketing system" → "working the ticketing system as first-line triage"
- "watching analysts triage alerts" → "watching analysts handle escalation"

Everything else in the paragraph byte-identical. The load-bearing "I did not own detections or run shifts at volume" and "stance, not a skill" clauses untouched.

## Why this tightening, and why minimal

Stage 2.3M rewrote the PISCES writeup body from ~400 words of uncalibrated LLM voice to ~180 words matching the about-page anchor. During that rewrite, the owner's actual scope surfaced more clearly than the about page had captured: the ticketing work wasn't passive observation, it was first-line triage (writing tickets on irregular signals). The analysts above that layer owned escalation and detection.

The about-page clause as written ("watching analysts triage alerts") collapsed both layers into a single observer-framed verb. This understated the first-line work — writing tickets on irregular signals *is* triage — and overstated the analysts' triage role (they weren't triaging alerts, they were handling escalations from the first-line layer).

2.3M flagged this as "direction of error is toward understatement (acceptable), but a future honesty pass should tighten it." This stage is that pass.

Three wording options were drafted before the edit. Option A (minimal tightening, selected) was chosen over Option B (plainer, used the writeup body's exact "irregular signals" phrasing) and Option C (structural sentence rewrite). Rationale: the about page is the anchor for six downstream PISCES surfaces. A structural rewrite risked invalidating calibration work that had already shipped across resume.rs, the three resume variants, home.rs one-liner, one_pager.rs one-liner, and the writeup body. Option A preserved sentence rhythm and kept the diff to exactly the two clauses that needed correction.

"First-line triage" is a term of art — noted as a con during the options review. Accepted because staff-level readers will understand it immediately and the phrase is more precise than any plain-language alternative. A reader who doesn't know the term can still parse the sentence ("working the ticketing system as something called first-line triage, while analysts did the escalation"), and the rest of the paragraph provides the context that makes the layering clear.

## Cross-surface consistency after this stage

| Surface | PISCES framing | Status |
|---|---|---|
| `about.rs` | "first-line triage…analysts handle escalation" | **Tightened anchor** (this stage) |
| `resume.rs` | "Academic SOC rotation — exposure…not operator tenure" | Unchanged, still aligned (Stage 2.3J) |
| `home.rs` | "Monitored 13 municipal entities" (neutral) | Unchanged, still consistent |
| `one_pager.rs` | "SOC internship monitoring 13 municipal entities" (neutral) | Unchanged, still consistent |
| `resume-devops.md` | Observer-framed three-bullet structure (2.3K) | Unchanged, still aligned |
| `resume-platform.md` | Observer-framed three-bullet structure (2.3K) | Unchanged, still aligned |
| `resume-sysadmin.md` | Observer-framed three-bullet structure (2.3K) | Unchanged, still aligned |
| `soc-observability-pisces-elk-kql.json` | "Built a handful of Kibana visualizations…wrote MantisBT tickets on whatever looked irregular…no escalation authority past ticket creation" | Unchanged, still aligned (Stage 2.3M) |

Post-apply grep confirmed:
- "first-line triage" and "handle escalation" appear only in `about.rs` after edit
- Zero residual "watching analysts triage" or "triage alerts" hits anywhere in src/, public/, or docs/resumes/
- "academic SOC rotation" and "operator tenure" phrases remain byte-identical across all seven PISCES-bearing surfaces

The anchor tightened without propagating inconsistency. The downstream surfaces already used framing that was compatible with the new anchor — the 2.3J/2.3K/2.3M calibration work had been slightly more scope-honest than the about page itself, which is why this tightening brought the anchor into line with the downstream surfaces rather than the other way around.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 15/15 passing
- trunk build --release: success

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. This stage closes the 2.3M follow-up flag and contributes to the `voice_consistency` lens (weight 18) by bringing the anchor into tighter alignment with the downstream surfaces it seeds. No new audit findings closed — this was hygiene on a flag raised by the series itself, not on an external audit finding.

## Discipline carrying forward

When a closeout flags a follow-up, the flag is a contract. 2.3M's note said "a future honesty pass should tighten it." Honoring that flag three stages later — same session, even — is the staff-engineer pattern of landing a change, flagging its follow-ups, and then closing them once the primary work stabilizes. Same pattern as 2.3H flagging the dead CSS and this session closing it two stages later.

Anchor surfaces get smaller edits, not larger ones. The about-page PISCES paragraph is referenced or paraphrased across eight surfaces. A one-sentence structural rewrite would have been a 50+ line downstream propagation stage. A two-phrase tightening within one sentence was a 1-insertion, 1-deletion diff with zero downstream surfaces needing follow-up, because the downstream surfaces had already calibrated against the direction this tightening moved toward. Smaller edits to anchor surfaces compound better than structural rewrites.

"Direction of error" framing holds up. 2.3M said understatement was acceptable residue, overclaim was not — and that a future pass should tighten toward accuracy from the understated side. That direction was respected here: the tightening moved closer to the actual scope without crossing into overclaim. The three wording options reviewed before drafting all stayed on the understatement-to-accuracy spectrum; no option that risked overclaim was entertained. That's the calibration rule the whole 2.3 series has been built on.
