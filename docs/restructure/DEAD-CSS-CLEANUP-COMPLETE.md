# Dead CSS Cleanup Complete — cert-* Classes and .about-pills-row Removed

Single-commit stage. Removed 22 orphaned CSS rules across two files — all cert-* classes left behind by Stage 2.3H (CertificationsSection deletion) and `.about-pills-row` left behind by Stage 2.3A (Section 4 removal). Pure deletion commit: 131 lines removed, zero added.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `2ae6302` | `style/pages/home.css`, `style/pages/about.css` | Delete 22 orphaned CSS rules; 131 deletions total |

## Scope expansion named before drafting

The original 2.3H closeout predicted fourteen unused cert-* classes flagged for a future cleanup pass. Recon surfaced twenty-one rules in home.css (the fourteen named classes appeared in three contexts: a light-theme override at line 2, the main definition block at lines 116–201, and a responsive media query at lines 309–336). Plus `.about-pills-row` in about.css. Total: 22 rules across two files.

Scope named before drafting. Same pattern as 2.3C (six numbers → eight), 2.3K (one file → three), and the ROADMAP regen (content update → structural rewrite). The briefing number set direction; reading the source set scope.

## What changed

### `style/pages/home.css` — three deletion sites

1. **Top-of-file light-theme override (lines 1–3):** `html.light .cert-row` — the only override for this class. Deleted cleanly; the file now opens with `html.light .category-desc` which is unrelated and survives.

2. **Main cert block (lines 114–203):** the entire `.certifications-section` wrapper plus its thirteen child rules:
   - `.home-page-wrap .certifications-section` (section wrapper)
   - `.cert-section-header`, `.cert-section-name`, `.cert-section-line`
   - `.cert-list { }` (empty rule — dead is dead, empty rules are noise)
   - `.home-page-wrap .certifications-section .cert-row` (scoped row)
   - `.cert-name`, `.cert-issuer`, `.cert-status`
   - Four status variants: `.cert-status-pursuing`, `.cert-status-studying`, `.cert-status-interested`, `.cert-status-completed`, `.cert-status-default`
   - Second `.cert-row` block + `.cert-row:last-child` (unscoped duplicate)

   The "Legacy `.home-footer` rules removed" comment was preserved because it's unrelated documentation about prior cleanup.

3. **Responsive media query (lines 308–337):** the entire `@media (max-width: 640px)` block labeled "Certifications: stack on narrow viewports" — four scoped child rules (`.cert-row`, `.cert-name`, `.cert-status`, `.cert-issuer`). The block was cert-only, so the wrapper and comment were removed with the contents.

### `style/pages/about.css` — one deletion site

4. **`.about-pills-row` rule (5-line block):** inside a larger about-page media query. Surgical delete of just the rule + its descriptive comment. Sibling rules (`.about-quote`, `.about-wrap > section + section`) survive because they're unrelated.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 15/15 passing
- trunk build --release: success

Post-apply greps confirmed:
- `grep -rn 'cert-\|about-pills-row' style/`: zero hits
- `grep -rn 'class=.*cert-\|class=.*about-pills-row' src/ public/`: zero hits (unchanged from pre-edit state; these classes were never referenced in src/ or public/)
- home.css: 337 lines → 216 lines (121 lines removed)
- about.css: 71 lines → 66 lines (5 lines removed)

Trunk build success confirms no CSS compilation errors introduced by the deletion. No visual regression possible because the classes were never rendered — zero references in Rust source or public assets.

## Cross-surface consistency

No cross-surface matrix required. CSS classes are either referenced or they aren't; grep settles the question. The post-apply grep confirming zero references across both source surfaces is the consistency check.

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. Dead CSS cleanup doesn't map to a specific numbered finding — it's hygiene follow-through on 2.3H and 2.3A deletions. Contributes to code-quality lens (weight 8) via reduced dead-code surface area.

## Discipline carrying forward

Orphan detection is a grep problem, not a judgment problem. The entire stage's scope determination reduced to one command — `grep -rn 'cert-\|about-pills-row' style/` for definitions, `grep -rn 'class=.*cert-\|"cert-' src/ public/` for usages, delete anything in the first set not found in the second. When a stage reduces cleanly to "confirmed orphaned by grep, delete it," the stage should be fast; the only cognition is in the wrapper-vs-surgical delete calls.

Follow-through on prior stages is a real category of work. 2.3H flagged the cert CSS as dead but left it out of scope for cross-surface alignment work. 2.3A flagged `.about-pills-row` as harmless. Both flags were correct at the time. Closing them three weeks later is not "cleanup debt" — it's the staff-engineer pattern of landing a change, flagging its follow-ups, and then closing those follow-ups once the primary work stabilizes. The flag is the contract. This stage honored both.

Deletion-only commits compress well. 131 deletions, zero insertions, two files changed. Diffs like this are the cheapest possible code review: the reviewer verifies that nothing deleted was referenced elsewhere (which the post-apply grep already proved) and approves. When a stage can be shaped as deletion-only, it should be.
