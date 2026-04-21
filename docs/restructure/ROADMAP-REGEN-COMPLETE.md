# ROADMAP Regen Complete — Two-Track Structural Rewrite

Single-commit stage. `docs/ROADMAP.md` was last meaningfully updated pre-Stage 2.1. The doc tracked the Engineering Track (Phase 0–9) in detail and compressed the Content & Positioning work into a single "Deferred" paragraph at the bottom noting that content work "is deferred by owner choice." That framing went stale on 2026-04-14 when Stage 2.1 opened, and it stayed stale through thirteen content-track commits.

This stage rewrites the ROADMAP structurally — not as additive catch-up, but as a two-track model that reflects the actual shape of the project.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `a76654a` | `docs/ROADMAP.md` | Structural rewrite: Content & Positioning Track + Engineering Track as co-equal workstreams, with active-status indicator and resumption gating |

## Scope expansion named before drafting

The original briefing described ROADMAP regen as "documentation hygiene, no decisions. Medium session." Recon surfaced that the existing doc had a structural problem, not a staleness problem: it tracked one workstream at phase-level granularity and compressed the other into a footnote. Catching the doc up without restructuring it would have preserved the split-identity — the Engineering Track keeps its phase numbering, the Content Track would have gotten bolted on as "Phase 2 (Content)" creating a numbering collision, and the "Deferred" paragraph would still be lying about work that's been the entire project output for three weeks.

Two options were named before drafting:

- **Option A (minimal regen):** keep existing Phase 0–9 structure, promote the Deferred paragraph into a named section, minimize diff.
- **Option B (structural rewrite):** two explicit tracks at the top level, each with its own status table, plus a "Currently Active" indicator. Honest about what the doc is for.

Option B was selected. Rationale: the staff-engineer move is to make the document reflect reality. Preserving a structure that was already broken (lying by omission for three weeks) because the diff is smaller is the kind of documentation debt that compounds. Option A was also offered as a scope-split path (A now, B later) — declined in favor of doing the rewrite once.

## Structural decisions

**Phase numbering preserved.** The Engineering Track keeps Phase 0, Phase 1, Phase 2–9 labels intact because existing closeout filenames (`PHASE1-SUMMARY.md`, `PHASE1-VERIFICATION.md`, `PHASE1.5-COMPLETE.md`, `PHASE1.6-COMPLETE.md`, `PHASE2-PREFLIGHT.md`) and commit messages reference those numbers. Renumbering would invalidate existing artifacts for zero gain. The Content Track uses Stage 2.x labels, already established across thirteen commits and closeouts. Two numbering systems — both already load-bearing in the repo — coexist without collision because they live under clearly separated tracks.

**Active-status indicator added.** The prior doc had no mechanism to signal which track was loaded. This one does: "Currently active: Content & Positioning Track. The Engineering Track has been paused since 2026-04-14." Future audits don't need to cross-reference commit timestamps to figure out what's in flight.

**Engineering Track resumption gated explicitly.** The new doc names the entry condition (close of Stage 2.3F) rather than leaving the transition implicit. The reasoning — that everything on the Content Track after 2.3F is hygiene-level and can interleave with engineering work without cross-contamination — is written down, not inferred.

**Phase 1 outstanding items preserved as carrying.** Hydration validation and headless smoke tests were listed as Phase 1 close-out items in the prior doc. They stayed listed because they haven't been done. Closing them silently would have been dishonest; moving them to a "Done" column would have been worse.

## Verification

- Line count: 144 (within expected 140–180 range)
- Both track headers present (`## Content & Positioning Track`, `## Engineering Track`)
- Top-level section list: Currently active preamble, Content & Positioning Track, Engineering Track, Active status, Note on prior Deferred framing
- Diff: 125 insertions, 36 deletions

No source edits — no gates required.

## Discipline carrying forward

Hygiene-level stages get closeouts too. The ROADMAP regen wasn't a numbered Stage 2.x item and could have shipped without a closeout doc. Skipping the closeout because "it's just docs" is how small discipline erosion compounds. A future auditor looking at why the ROADMAP changed structure deserves the same artifact trail that STAGE2.3K-COMPLETE.md provides — the reasoning, the options considered, the call made, the scope-expansion moment. Five minutes of closeout discipline is cheap insurance against reconstructing the reasoning six months from now.

When a document's structure is the problem, the fix is structural. Iterative doc updates can fix content drift; they can't fix shape drift. The ROADMAP's shape was wrong (one track tracked at phase granularity, one track compressed into a paragraph) and no amount of additive updates would have corrected that — each update would have reinforced the imbalance. The rewrite was the correct cost.
