# Stage 2.3F-Cisco Complete — Cisco IOS Writeup Voice Rewrite + CCNA Exam Framing Cross-Surface Correction

Single-commit stage. Wholesale rewrite of `public/writeups/cisco-ios-fundamentals.json` body from ~340 words of uncalibrated LLM voice ("Universal Control Plane," "Father, Creator, and Builder," "Architect of the Full Stack," "Lead Architect performs manual Status Propagation") to ~180 words across three sections matching the about-page voice anchor. Registry summary in `src/data/writeups.rs` replaced with calibrated one-liner. Cross-surface CCNA exam framing corrected across `src/pages/resume.rs` and `src/pages/one_pager.rs` Training lines.

This is the first half of 2.3F. The remaining half (windows-server-lab-powershell-automatedlab.json) is the next and final stage of this session.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `6cba0ce` | `public/writeups/cisco-ios-fundamentals.json`, `src/data/writeups.rs`, `src/pages/resume.rs`, `src/pages/one_pager.rs` | Body rewrite + registry summary + cross-surface exam framing correction; 4 files, 4 insertions, 4 deletions |

## Scope expansion named before drafting

The original briefing scoped 2.3F-Cisco as "writeup body rewrite (~30-60 min, same shape as 2.3M)." Recon + scope conversation surfaced two findings that expanded the stage:

1. **Owner sat the CCNA exam and did not pass.** Stage 2.3H closed cert framing across the site with "No exam taken" language on `resume.rs` and `one_pager.rs` Training sections. That framing was inaccurate — the exam was attempted, just unsuccessfully. The writeup rewrite would have created a cross-surface contradiction (writeup says "did not pass," resume says "no exam taken") if the resume + one-pager weren't updated in the same commit.

2. **The fictional original body could not be salvaged sentence-by-sentence.** Same conclusion 2.3M reached for PISCES — when every sentence claims something the owner didn't do, sentence-level rewriting preserves the inflation's skeleton. Wholesale rewrite from owner's actual scope statement is the honest path.

Scope expansion named before draft, per established discipline (2.3C six→eight, 2.3K one→three files, ROADMAP regen content→structural, identity reframing five→six edits). Same pattern: briefing sets direction, scope conversation sets actual scope.

## What changed

### Writeup body (public/writeups/cisco-ios-fundamentals.json)

Before (~340 words, 3 sections + 4 subsections):
- OVERVIEW: "architect a high-level Internal Developer Platform (IDP) or orchestrate multi-cloud compositions...My journey into systems architecture was forged in the physical trenches...building a Universal Control Plane from the ground up"
- FOUNDATIONAL PROTOCOLS: 4 subsections inflating CCNA basics into Kubernetes/platform-engineering analogies — "Cisco IOS as Declarative Intent," "Subnetting as Admission Policy," "Port Security at the Edge" with "rejecting invalid intent at the moment it hits the interface," "Lead Architect performs manual Status Propagation"
- THE ARCHITECT'S FOUNDATION: "As a Father, Creator, and Builder...masterclass in the Basics of Responsibility...Whether I am defining a Crossplane Composition or an AI Agent...Architect of the Full Stack...architect the Golden Path of the digital age"

After (~180 words, 3 sections, no subsections):
- Overview: "Cisco Networking Academy CCNA-track coursework, four courses, 2018–2019, high-school program. Hybrid lab environment — physical rack-mounted Cisco gear at school (2960 switches, 1941/2901 routers), Packet Tracer at home for between-class exercises. I sat the CCNA exam and didn't pass; the academy coursework was completed."
- What the Coursework Covered: standard four-semester CCNA-track sequence; hands-on (VLAN config, OSPF setup, port security, VLSM math, OSI troubleshooting in Packet Tracer + real gear); studied (protocol theory)
- What the Foundation Carries Forward: networking literacy, VLSM math → cloud VPC CIDR planning, OSI bottom-up debugging discipline, Terraform networking module comprehension. Closes with "Foundational coursework, not Cisco-engineer credentials. The resume credit is what it is."

The four-subsection structure was deliberately collapsed. The original used `pd-challenges`/`pd-challenge` machinery to enumerate four inflated framings; honest scope didn't need that machinery. Three pd-section blocks with paragraph bodies — same structural simplification as 2.3M's PISCES rewrite.

### Registry summary (src/data/writeups.rs:156)

Before: "Foundational networking via Cisco IOS: VLSM subnetting for environment isolation, MAC-based port security, and bottom-up OSI troubleshooting as the bedrock of infrastructure operations."

After: "High-school CCNA-track coursework — what foundational networking literacy gives you in 2026, and what it doesn't."

The "and what it doesn't" half is the calibration signature. The original summary read as marketing-grade credential framing ("the bedrock of infrastructure operations"); the rewrite signals scope-honesty before the reader clicks through.

### Cross-surface CCNA exam framing correction

Before (both `src/pages/resume.rs` and `src/pages/one_pager.rs`):
> "Cisco Networking Academy — CCNA track, four courses completed (2018-2019). No exam taken."

After:
> "Cisco Networking Academy — CCNA track, four courses completed (2018-2019). Sat the CCNA exam, did not pass."

Direction of correction: more honest, not less. "No exam taken" is a softer framing than reality. "Sat the exam, did not pass" is the actual fact, and on a portfolio explicitly positioned as "Honest about scope" (per the OG image regen this session), the actual fact is the right framing.

Stage 2.3H closed cert language across the site believing "no exam taken" was accurate. That belief turned out to be inaccurate. This stage corrects the record. No fault on 2.3H — the calibration discipline is good information now revealed by deeper scope conversation.

## Scope conversation method

Five questions were asked before drafting (same shape as 2.3M's scope conversation for PISCES):

- Q1: When and where? → High school program, 2018-2019, four courses (already established by 2.3H)
- Q2: Real hardware or simulator? → Hybrid: rack-mounted Cisco gear at school (2960 switches, 1941/2901 routers), Packet Tracer at home
- Q3: What topics covered? → Standard four-semester sequence
- Q4: What carries forward? → All four candidate framings (networking literacy, VLSM → VPC CIDR math, OSI bottom-up debugging, Terraform networking module reading) — owner confirmed all real
- Q5: Quiet or loud honest tone? → Quiet honest, with explicit exam-failure disclosure as load-bearing fact

The Q4 multi-select answer ("yes all of it") was the deciding factor between Option A (rewrite to honest scope) and Option C (delete the writeup entirely). Recommendation before scope conversation was Option C — high school networking class is the lowest-tier credential on the portfolio and the resume already credits it. But Q4 confirmed substantive present-tense relevance: the work demonstrably gives the owner four defensible carry-forwards in 2026 work. Option A became the right call.

The Q5 answer surfaced the exam-failure fact, which expanded scope to the cross-surface fix.

## Cross-surface consistency after this stage

| Surface | CCNA framing | Status |
|---|---|---|
| `cisco-ios-fundamentals.json` body | "I sat the CCNA exam and didn't pass; the academy coursework was completed" | **Calibrated anchor** (this stage) |
| `cisco-ios-fundamentals.json` registry summary | "what foundational networking literacy gives you in 2026, and what it doesn't" | Aligned (this stage) |
| `resume.rs` Training | "Sat the CCNA exam, did not pass" | **Aligned** (this stage) |
| `one_pager.rs` Training | "Sat the CCNA exam, did not pass" | **Aligned** (this stage) |
| `about.rs` | Silent on CCNA — references academic coursework generally without claiming credentials | Consistent |
| Resume variants (`docs/resumes/*.md`) | Did not contain "No exam taken" language pre-stage; checked via grep | Consistent (no edit needed) |

Post-apply grep confirmed zero residual "No exam taken" hits anywhere in src/, public/, or docs/resumes/. The cross-surface contradiction risk is closed.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 15/15 passing
- trunk build --release: success

JSON validity confirmed via `python3 -c "import json; json.load(...)"` before gate suite.

Post-apply greps confirmed:
- Zero hits on the ten pre-edit grandeur phrases ("Father, Creator, and Builder", "Architect of the Full Stack", "Universal Control Plane", "Internal Developer Platform", "Lead Architect performs manual Status Propagation", "Cisco IOS as Declarative Intent", "Subnetting as Admission Policy", "architect the Golden Path", "forged in the physical trenches", "orchestrate multi-cloud")
- All four post-edit phrases present across the relevant files ("CCNA-track coursework", "Sat the CCNA exam", "did not pass", "Cisco Networking Academy")
- Zero residual "No exam taken" hits anywhere in src/, public/, or docs/resumes/

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. This stage closes the first half of P1-2 (KEEP-writeup voice pass). Contributes to `voice_consistency` lens (weight 18) and `honesty_discipline` lens (weight 22) by replacing the most LLM-voice-saturated coursework writeup with calibrated scope statement and propagating the corrected exam framing across four surfaces.

The cross-surface fix on resume.rs and one_pager.rs also retroactively strengthens 2.3H's cert honesty work — the "Training" sections now reflect actual scope rather than a softer-than-reality framing.

## Discipline carrying forward

Scope conversations surface facts that change the stage. The exam-failure disclosure was not in the briefing, was not in the prior closeouts, and was not in any prior recon. It surfaced because Q5 ("does the writeup need to mention the gap?") created space for the owner to name the fact directly. Stage discipline includes asking the questions that let load-bearing facts surface, not just the questions that confirm the briefing's scope.

The "stop short of asking" failure mode — never asking Q5 because the briefing didn't flag exam status — would have shipped a writeup that contradicted resume and one-pager surfaces the moment any audit ran a cross-surface grep. The cost of a 5-question scope conversation per stage is much lower than the cost of cross-surface contradictions discovered three stages later.

When carry-forward is real, the writeup earns its place. The Option A-vs-C decision pivoted entirely on Q4. If Q4 had returned "(e) None — resume credit is enough," Option C deletion would have been the right call, the writeup would have been removed, and the portfolio would be one item lighter. Q4 returned all four carry-forwards confirmed real, which made the writeup a substantive piece despite its low-tier credential origin. The discipline is letting Q4 decide, not deciding before asking.

Closing lines do calibration work. The body's last sentence — "Foundational coursework, not Cisco-engineer credentials. The resume credit is what it is." — is the same kind of load-bearing closer as 2.3M's "stance, not a skill." Both signal "I know the scope of this work and I'm not going to oversell it." A staff reader who reaches the closing line of a writeup and finds the writer naming the limit themselves trusts the writer more than one who finds inflation. The closer is part of the calibration, not separable from it.
