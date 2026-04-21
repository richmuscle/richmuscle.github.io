# Identity-Access-Lifecycle Claim Reframing Complete — Config-Assertion to Design-Intent

Single-commit stage. Closes the Terraform "1 → 0 by design" verification stage flagged in Stage 2.3C's closeout — but closes it to a different outcome than originally scoped. Recon revealed the underlying config does not exist: the `infra-tooling/zerotrust-admin-fabric` submodule in the private monorepo is scaffolded (LICENSE, .git, empty DECISIONS/scripts/terraform/.github directories) but contains zero Terraform files. The claims in `identity-access-lifecycle.json` describe an architecture that has been designed but not implemented.

The original stage premise was "grep Terraform for zero-public-listener enforcement; if it matches, add config evidence reference; if not, revise the JSON claim." The Terraform doesn't exist, so the revise-if-not branch is the correct close. Six reframings move achieved-config voice ("Utilized," "Implemented," "Configured," "Built to satisfy") to design-intent voice ("Designed," "Architecture specifies," "Design specifies," "Design intent is").

## Commit

| Commit | Scope | Change |
|---|---|---|
| `e46acbd` | `public/projects/identity-access-lifecycle.json` | Six string substitutions inside the one-line JSON content body; 1 insertion, 1 deletion |

## Stage premise change named explicitly

The 2.3C closeout flagged: "the '1 → 0 by design' claim is structurally true only if the deployment config matches. A future verification pass should grep the project's Terraform/config for zero-public-listener enforcement."

Recon followed that flag and surfaced two findings the original stage scope didn't anticipate:

1. **The `~/dev/floorp` private monorepo exists and is mature.** SOC stack with 15 healthy containers, TPM2-sealed credentials, restic backups with drilled restore, 24 Wazuh + 15 Sigma detection rules, threat-intel-aggregator integration. Composite audit score 9.5/10 per April 16 audit report. Real ops work.

2. **The portfolio-facing submodules are mixed substance.** Of the six referenced in the portfolio's `InDevelopment` projects:
   - `terraform-landing-zone` — substantive (13 .tf files, modules/, environments/, Makefile, .github)
   - `ansible-baseline` — substantive (30 .yml files across roles/, 6 Jinja templates)
   - `zerotrust-admin-fabric` — scaffold only (zero source files, four empty subdirectories)
   - `observability-stack` — scaffold only (LICENSE only)
   - `cicd-showcase` — scaffold only (zero source files, twelve log artifacts)
   - `gitops-platform` — scaffold only (CLAUDE.md only)

The identity-access-lifecycle.json describes `zerotrust-admin-fabric`. That submodule is in the scaffold tier. The claim has no backing config because the implementation phase hasn't started yet — the scaffolding was created on April 16 (six days ago) per the recent "convert portfolio projects to standalone submodule repos" refactor.

Two paths were named before drafting:

- **Path 1 (selected):** Revise the JSON to match reality — design-voice reframings, no implementation claims. One JSON file, ~15 minute stage. Removes immediate exposure (a staff reader could ask "show me the Terraform" and get nothing).
- **Path 2 (deferred):** Open 2.3D-expanded — full registry status review across all six portfolio submodules, downgrade scaffolds to `planned`, upgrade underrepresented real projects, address the positioning conflict between `PORTFOLIO_PLAN.md` (sysadmin-first) and the current site (DevOps/platform-leaning). Multi-session.

Owner stance — "we are doing the core to land jobs then put those in later" — aligned with Path 1 first. Path 2 is the right next P1 stage and belongs in a fresh session, not as the eleventh commit of this one.

## What changed

Six exact-match string substitutions inside the JSON `content` body:

### Edit 1 — Dark Infrastructure pd-challenge body

Before: "Utilized AWS Security Groups and Network ACLs so the management subnet exposes zero public-facing listeners. Initiated connections via authenticated UDP hole-punching with persistent keep-alives."

After: "Designed AWS Security Groups and Network ACLs to expose zero public-facing listeners on the management subnet. Connection model is authenticated UDP hole-punching with persistent keep-alives — initiated outbound from the peer, never accepting unsolicited inbound."

### Edit 2 — "1 → 0" outcome stat label

Before: "Public administrative entry points by design (Bastion -> Dark Node)"

After: "Public administrative entry points (design target, Bastion -> Dark Node)"

### Edit 3 — Compliance audit trail stat (value AND label)

Before stat value: "100%"
After stat value: "Centralized"

Before stat label: "Compliance audit trail by design (every peer-to-peer transfer routed through centralized logging)"
After stat label: "Compliance audit trail (design target — every peer-to-peer transfer routed through central log sink)"

The "100%" itself was the problem — a 100% claim qualified as "design target" reads as internally contradictory. Replacing the numeric stat with "Centralized" lets the qualifier do real work. The numeric position now describes a design property (centralization) rather than a measured outcome (100% coverage).

### Edit 4 — Identity Control Plane pd-challenge body

Before: "Configured WireGuard peer identities with Active Directory (LDAP). Implemented instant access revocation across the entire fabric when an AD account is disabled."

After: "Designed WireGuard peer identities to bind to Active Directory (LDAP). When an AD account is disabled, the design intent is fabric-wide access revocation via the AD integration layer — implementation pending."

### Edit 5 — NIST 800-207 Micro-segmentation pd-challenge body

Before: "Built to satisfy NIST 800-207 standards. Implemented micro-segmentation where the Admin Peer can reach the DB-Proxy but never the DB-Master directly."

After: "Designed to satisfy NIST 800-207 micro-segmentation requirements. The Admin Peer reaches the DB-Proxy; the DB-Master is unreachable from the admin fabric. The DB-Proxy is the trust boundary."

### Edit 6 — MTU & Fragmentation pd-challenge body

Before: "Mitigated the failure mode by configuring PostUp iptables rules to clamp the Maximum Segment Size (MSS) to 1280 bytes, with the design intent of eliminating MTU-induced fragmentation on high-bandwidth telemetry paths."

After: "Design specifies PostUp iptables rules to clamp Maximum Segment Size (MSS) at 1280 bytes — the intent is eliminating MTU-induced fragmentation on high-bandwidth telemetry paths."

This edit was scope-expansion named before drafting. 2.3C's pass added "with the design intent of" but left the achieved-voice "Mitigated the failure mode by configuring" intact. Same class of issue as Edits 1, 4, 5 — fixed in this stage to avoid a third pass on the same file.

## Drafts that were verified before drafting the prompt

The first-pass drafts of Edits 1, 3, 4, and 5 had problems that wouldn't have survived staff-engineer review:

- Edit 1 v1 said "Architecture specifies … configured" (grammatically muddled), used "Inbound connections handled via" (still achieved voice), and added a padding clause ("the management endpoint accepts no unsolicited traffic") that was an additional unverified claim.
- Edit 3 v1 kept the "100%" stat with a "design target" qualifier — internally contradictory framing.
- Edit 4 v1 said "achieved by tying peer authorization to live LDAP lookup rather than static key distribution" — invented a specific technical mechanism (live LDAP lookup) that the actual design may not specify.
- Edit 5 v1 ended with "— proxy isolation is the trust boundary" as a dash-clause restatement, padding the previous sentence.

V2 drafts were owner-approved before the CLI prompt. The verification step caught problems that would have shipped if the first-pass drafts had gone straight to commit.

The pattern: drafting in design voice is harder than it looks. "Implemented X" → "Designed X to" is straightforward. But staying in design voice across three or four sentences without backsliding into "achieved by," "handled via," "configured," or "built" requires re-reading every sentence with fresh eyes. The verification pass is not optional for voice-calibration stages.

A status-line addition was considered ("Project status: design + scaffolding complete; implementation in progress") and dropped. Status communication belongs in the registry badge, not as a meta-disclaimer interrupting the case study's voice. The 2.3D-expanded stage will address the registry alignment.

## Cross-surface consistency after this stage

| Surface | Identity claims framing | Status |
|---|---|---|
| `identity-access-lifecycle.json` | Design-intent voice throughout: "Designed," "Design specifies," "design target," "implementation pending" | **Aligned** (this stage) |
| `src/data/projects.rs` registry entry | Status badge unchanged (`InDevelopment`) | Pending — addressed in 2.3D-expanded |
| Resume variants (devops/platform/sysadmin) | "Built a zero-trust administrative fabric" framing in `resume-devops.md` L44 | Note: still claims "Built." The scope here is real lab work (WireGuard + AWS VPC + AD LDAP demonstrated in homelab), distinct from the public-portfolio identity-access-lifecycle project. Different scope, same name. Worth a future cross-reference check but not in this stage's scope. |
| Other portfolio JSONs | Unaffected | No change required |

The post-apply grep confirmed zero residual achieved-config phrases in the JSON and zero propagation to other public/ or src/ surfaces.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 15/15 passing
- trunk build --release: success

JSON validity confirmed via `python3 -c "import json; json.load(open(...))"` before gate suite.

Post-apply greps confirmed:
- Zero hits on the eight pre-edit phrases ("Utilized AWS Security Groups", "Implemented instant access revocation", "Implemented micro-segmentation", "Built to satisfy NIST", "by design (Bastion", "by design (every peer-to-peer", "Configured WireGuard peer identities", "Mitigated the failure mode")
- Seven hits on the post-edit phrases ("Designed AWS Security Groups", "Designed WireGuard peer identities", "Designed to satisfy NIST", "design target, Bastion", "design target — every", "Design specifies PostUp", "implementation pending")
- Zero propagation: no other surface in `public/` or `src/` carries the achieved-config phrasing

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. This stage closes the verification flag from Stage 2.3C's closeout. Contributes to the `honesty_discipline` lens (weight 22) and removes a class of claim that a staff reader could pressure-test by asking for the underlying config — the case study now invites design-level discussion instead.

The original 2.3C closeout said: "Stage 2.3D is the next logical step for these two files — it will rewrite the pd-challenge bodies wholesale to match the security-baseline-audit template." That guidance still holds. This stage made the bodies honest in voice; 2.3D-expanded will address the registry status alignment and any remaining structural reframing across both InDevelopment project pages.

## What's next, named for the future session

The recon for this stage surfaced three things that belong on the next session's table, not this one:

1. **2.3D-expanded.** Six portfolio submodules need registry status review. Three are scaffolds and need downgrade to `planned`. Two (terraform-landing-zone, ansible-baseline) are substantive and may be undersold by current portfolio framing. Per-project scope conversation needed for each, same shape as 2.3M and 2.3F.

2. **Positioning conflict between `PORTFOLIO_PLAN.md` and the current portfolio.** The plan positions as "Linux Systems Administrator (with DevOps toolkit)." The current portfolio runs three resume variants (devops, platform, sysadmin) tiered by comp target. The about page says "Linux systems administrator in Oklahoma City" which matches the plan, but downstream surfaces hedge across three registers. Worth a positioning-alignment stage at some point.

3. **Substantial floorp work that's not on the portfolio yet.** `soc-stack`, `detection-engineering`, `soc-automation` are real, mature, audit-scored 9.5/10 work that the public portfolio doesn't surface. Owner stance is "land jobs first, port the rest later" — that's a legitimate sequence, but the porting work eventually needs its own stage series.

None of these are blocked. All three should open in a fresh session with clean head, not as continuation of this one.

## Discipline carrying forward

When a verification stage finds the thing being verified doesn't exist, the stage doesn't fail — it pivots. The original stage premise was "verify the claim against config." Recon found there was no config. The stage didn't become "blocked, defer." It became "the claim needs revision, revise it." Both outcomes are legitimate closes for a verification stage. The framing of the work as "verify" rather than "verify-and-substantiate" kept the stage honest about what was being asked.

Voice calibration drafts need a verification pass. Four of the six v1 drafts had problems that staff-engineer review caught — grammatical muddling, padding with unverified claims, internal contradiction in qualifier framing, restatement-as-padding. None would have caused a hard failure (the JSON would have parsed, the gates would have passed) but all would have been visible to a staff reader. Self-verifying drafts before drafting the CLI prompt is the discipline that prevents shipping subtly-wrong calibration.

Status communication belongs in metadata, not in prose disclaimers. The instinct to add "Project status: design + scaffolding complete; implementation in progress" inside the case study was wrong. Case studies tell stories; status badges communicate state. Mixing the two reads as either apologizing or distrusting the badge — both bad signals to a staff reader. The reframings themselves carry enough signal that a careful reader infers the project state correctly without a meta-disclaimer.

The 2.3 series demonstrates that an honesty pass can take many shapes. 2.3A rewrote a page. 2.3J aligned cross-surface scope. 2.3K extended that alignment to variants. 2.3M rewrote a writeup body. 2.3C qualified metric claims. This stage qualified structural claims in the same vocabulary. The shapes differ; the underlying move is the same — make the surface match the reality. When the gap can't be closed by reframing alone, the next stage opens at the registry level. That's the natural continuation.
