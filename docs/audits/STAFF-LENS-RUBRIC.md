# Staff-Lens Audit Rubric

This rubric measures the portfolio against staff-engineer-level documentation discipline applied to a sysadmin target role. It is not an engineering code-quality audit — that work is covered by `just check`, `cargo clippy`, and `cargo audit`. This rubric measures what the code-quality gates cannot: honesty, voice, positioning, and coherence across surfaces.

## Scoring

Seven lenses, each 0–10. Composite is the weighted average.

| Lens | Weight | What it measures |
|---|---:|---|
| honesty_discipline | 22 | Every user-visible claim has backing, qualification, or explicit scope boundary |
| voice_consistency | 18 | All surfaces read as one author; voice breaks are named |
| staff_lens_coherence | 15 | Work is documented with ADRs, tradeoffs, failure modes, and measurement methodology — everywhere, not just in calibrated pages |
| positioning_alignment | 12 | Site does not self-declare a tier it shouldn't; it lets the work speak |
| cross_surface_consistency | 13 | When multiple surfaces describe the same experience, they agree |
| engineering_hygiene | 10 | Build passes, gates green, no banned vocabulary in user-visible copy, no orphan files |
| content_density | 10 | The pages that claim work have enough detail to defend the claim |

## Per-Lens Scoring Criteria

### honesty_discipline (weight 22)

**10:** Every number has methodology. Every "hands-on" claim is scoped (lab vs. production). Every in-progress item is labeled. Every limitation is surfaced before the reader has to ask.

**8:** One or two unqualified claims exist, but the discipline is the dominant pattern. Scope statements are present on most surfaces. The reader can trust what they read.

**6:** Some surfaces are calibrated, others are not. The reader encounters honest claims and inflated claims on the same visit. The about page (if calibrated) may contradict the resume page (if not).

**4:** Inflated claims are common. Duty-description bullets masquerade as impact claims. Numbers appear without methodology. Scope is rarely stated.

**2:** The site reads as a résumé-generator default. Most claims overstate what was done. Limitations are hidden.

### voice_consistency (weight 18)

**10:** Every surface reads as the same author. No surface has banned vocabulary. No surface is more flowery or more terse than the others without reason. A reader can't tell which surface was written when.

**8:** One or two minor voice variations exist (e.g., a page uses slightly different sentence length), but the reader perceives a single author.

**6:** A clear divide exists between calibrated and uncalibrated surfaces. Reading the about page then the resume produces an "is this the same person?" reaction.

**4:** Multiple voices are visible. Some surfaces read as LLM-assisted; others read as human-edited. Banned vocabulary appears on at least one surface.

**2:** Voice is inconsistent page-to-page. The reader concludes the site was assembled from mismatched sources.

### staff_lens_coherence (weight 15)

**10:** Every project page has ADRs with rejected alternatives, failure modes, measurement methodology, scope boundaries, and a "what I'd do differently" section. Writeups are technically grounded, not essays about discipline.

**8:** The shipped projects have the full template. InDevelopment projects have partial templates with notice bars. Planned projects correctly show scope only.

**6:** One project has the full template (the anchor case study). Others have V1 content that lacks decisions/failure modes/method.

**4:** No project has a full template. Case studies describe what was built without the decisions behind it.

**2:** The site lists projects without documenting reasoning.

### positioning_alignment (weight 12)

**10:** The site does not self-declare a tier. The work documents the capability; the hiring team places the candidate. No "senior" self-label. No "staff" self-label. No hidden tier claim in metadata.

**8:** Positioning is consistent with the declared intent in CLAUDE.md across all surfaces. One or two legacy tier mentions exist but are documented as known residue.

**6:** CLAUDE.md says one thing; some surfaces contradict it. "Senior" may be self-declared on the home page but removed from the about page, for instance.

**4:** Multiple self-declared tiers exist across surfaces. The reader can't tell what tier the candidate targets.

**2:** The site either overclaims tier systematically or underclaims it relative to the evidence.

### cross_surface_consistency (weight 13)

**10:** The same experience described on two surfaces uses the same scope language. Numbers match. Tool lists match. Cert status matches across the about page, home page, resume, one-pager, and certs.rs.

**8:** Minor phrasing differences across surfaces, but no contradictions. A careful reader notices no inconsistency.

**6:** One or two cross-surface contradictions exist (e.g., about page says "academic rotation," resume claims active triage at volume).

**4:** Multiple contradictions. Cert language differs between surfaces. Role framing differs. Numbers differ.

**2:** The surfaces appear to describe different candidates.

### engineering_hygiene (weight 10)

**10:** All four `just check` gates green. Tests passing. `cargo fmt --check` clean. `cargo clippy -- -D warnings` clean on wasm32. Zero banned-word hits in user-visible copy via grep. No orphan files (no untracked docs, no dead CSS rules documented as live, no unused imports). Scalability contract per CLAUDE.md satisfied.

**8:** Gates green, tests passing, clippy clean. One or two cosmetic hygiene issues (e.g., a dead CSS rule documented as unused).

**6:** Gates green but hygiene has slipped — banned words appear in a grep of `src/` or `docs/`, untracked files exist at the repo root.

**4:** Gates pass but tests are thin. Banned vocabulary has leaked into source.

**2:** A gate fails.

### content_density (weight 10)

**10:** Every project page has enough detail that a hiring manager reading it for five minutes knows what was built, why, what was hard, and what the candidate would change. Every writeup has enough content to demonstrate depth, not pad a list.

**8:** The shipped project is fully dense. InDevelopment projects are clearly marked as V1 with notice bars. Writeups average 500+ words of real content.

**6:** The shipped project is dense; other project pages are thin. Writeup content is uneven.

**4:** Most pages are thin. A reader finishes a project page with open questions about what was actually built.

**2:** Pages are placeholders or single-paragraph summaries.

## Composite Calculation

```
composite = (
  honesty_discipline * 22 +
  voice_consistency * 18 +
  staff_lens_coherence * 15 +
  positioning_alignment * 12 +
  cross_surface_consistency * 13 +
  engineering_hygiene * 10 +
  content_density * 10
) / 100
```

## Severity Taxonomy for Findings

**P0 — ship-blocking.** Evidence the site cannot be honestly sent to a recruiter in its current state. Examples: contradictory claims across surfaces that a careful reader would notice. Banned vocabulary in the hero text. A measurement claim that is demonstrably wrong.

**P1 — visible gap.** Known inconsistency that degrades signal. Examples: two surfaces using different phrasings for the same experience, where the less honest phrasing is the inflated one. An uncalibrated surface that sits adjacent to a calibrated one.

**P2 — residue.** Small, isolated inconsistencies. CSS rules no longer used. Documentation files that haven't been updated. Minor phrasing drift.

**P3 — long-horizon.** Work that won't land this cycle. New project builds, template completions, secondary content passes.

## What the Audit Does NOT Measure

- Whether the code would pass a senior code review. `cargo clippy` does that.
- Whether the dependencies are current. `cargo audit` does that.
- Whether the engineering decisions in the codebase are right. ADRs do that.
- Whether the site looks good visually. Design review does that.

This rubric measures whether the portfolio is a **trustworthy artifact for a hiring decision** — whether everything it says about the candidate is defensible, whether it reads as one coherent voice, and whether the rigor it claims is evenly distributed.
