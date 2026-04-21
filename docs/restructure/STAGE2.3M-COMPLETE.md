# Stage 2.3M Complete — PISCES Writeup Body Voice Rewrite

Single-commit stage. The PISCES writeup body (`public/writeups/soc-observability-pisces-elk-kql.json`) was the loudest remaining voice break in the PISCES story after Stages 2.3A (about page), 2.3J (resume), and 2.3B (writing intro). ~400 words of uncalibrated LLM voice claiming "Principal Architect of Observability", detection authorship, institutional ownership of 30+ public institutions, and a pathos section ("Father, Creator, and Builder") were replaced with ~180 words across three sections matching the about-page framing established in Stage 2.3A.

Also updated the registry summary in `src/data/writeups.rs` for this slug — the old summary claimed "KQL semantic search for APT detection" without scoping it as rotation work.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `284da8f` | public/writeups/soc-observability-pisces-elk-kql.json | Replace entire content body: 3 sections (Overview, What the Work Actually Was, What the Rotation Taught). THE BUILDER'S RESPONSIBILITY section deleted, not rewritten. |
| `284da8f` | src/data/writeups.rs:145 | Replace summary field for this slug with scope-honest one-liner |

## What changed

### Writeup body (public/writeups/soc-observability-pisces-elk-kql.json)

Before (~400 words, 3 sections + 4 subsections):
- OVERVIEW: "I acted as a Principal Architect of Observability", "90% of the value was created through the mastery of the ELK Stack"
- OPERATIONAL ARCHITECTURE: "I architected filters using KQL and Lucene", "I architected correlative rules", four pd-challenge subsections with inflated framing
- THE BUILDER'S RESPONSIBILITY: "Father, Creator, and Builder", "30+ public institutions we secured", "Senior Principal Lead", "Systems Sovereignty"

After (~180 words, 3 sections, no subsections):
- OVERVIEW: "Academic SOC rotation...My hands-on work was dashboard building and first-line ticketing...not detection authoring, not shift ownership, not stakeholder briefing."
- WHAT THE WORK ACTUALLY WAS: "Built a handful of Kibana visualizations...wrote MantisBT tickets on whatever looked irregular...no escalation authority past ticket creation."
- WHAT THE ROTATION TAUGHT: "I don't own detections now — I own making the signal visible...That stance is what I took from PISCES, not a skill."

### Registry summary (src/data/writeups.rs)

Before: "Operational SOC observability at PISCES International: ELK Stack log aggregation, KQL semantic search for APT detection, temporal correlation of IDS alerts with outbound traffic, and MantisBT for incident state tracking."

After: "Dashboard work and first-line ticketing during an academic SOC rotation — what observability looks like from the upstream side of detection."

## Cross-surface consistency after this stage

| Surface | PISCES framing | Status |
|---|---|---|
| about.rs | "academic rotation...did not own detections or run shifts at volume" | Anchor (calibrated in 2.3A) |
| resume.rs | "Academic SOC rotation — exposure...not operator tenure" | Aligned (Stage 2.3J) |
| home.rs | "Monitored 13 municipal entities" (neutral) | Consistent |
| one_pager.rs | "SOC internship monitoring 13 municipal entities" (neutral) | Consistent |
| writeup body | "Academic SOC rotation...dashboard building and first-line ticketing" | **Aligned** (this stage) |
| writeup summary (registry) | "Dashboard work and first-line ticketing during an academic SOC rotation" | **Aligned** (this stage) |

Note: the owner's actual scope per this rewrite is slightly broader than the about page's "watching analysts triage" framing. The writeup accurately captures that dashboards-to-tickets work happened, which is lightweight analyst work. The about page understates this slightly. Worth flagging for a future minor pass on the about page but not blocking — the direction of error (understating rather than overclaiming) is the right one.

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. Stage 2.3J closed P0-1 (PISCES resume contradiction). This stage closes the deepest remaining contradiction in the PISCES story, which the audit flagged indirectly via the voice_consistency lens (weight 18). P0-2 and P0-3 (unsupported metrics in InDevelopment project pages) remain open for Stage 2.3C.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 18/18 passing
- trunk build --release: success
- Banned-word grep on writeup body: zero hits
- Scope-contradicting phrase grep on writeup body: zero hits

## Remaining stages

| Stage | Priority | Title | Status |
|---|---|---|---|
| 2.3C | P0 | Unsupported metrics resolution (identity + observability) | Open |
| 2.3H | P1 | Cert honesty sweep | Open |
| 2.3D | P1 | InDevelopment project content voice | Open |
| 2.3F | P1 | KEEP-writeup voice pass | Open |
| 2.3L | P1 | Writeup body author-line sweep (dead code cleanup) | Open |
| 2.3K | P1 | Internal resume variants | Open |
| 2.3G | P3 | OG image regeneration | Open |
| 2.3E | P3 | Platform page build | Blocked on 2.3D |
| 2.3I | Low | Senior-tier residue sweep | Pending confirmation grep (audit indicated likely resolved) |

## Discipline carrying forward

When the fabricated body is unsalvageable, writing fresh from the owner's own scope statement is the honest path — no attempt to "rescue" invented claims. The old body's four pd-challenge subsections and pathos section were structurally intact but every sentence claimed something the owner didn't do. Rewriting sentence-by-sentence would have preserved the inflation's skeleton. Starting from "what did I actually do during this rotation?" produced a shorter, truer piece.

Writeup rewrites need to touch both the content body AND the registry summary, since both are user-visible surfaces. The registry summary appears on the /writing index page; a calibrated body with an uncalibrated summary would create the same cross-surface contradiction this series is designed to close.

When a correction moves a surface from overclaim toward scope-honesty, mild understatement elsewhere is acceptable residue. The direction of error matters: overclaiming is a calibration failure, understating is a calibration artifact. Future audits should flag understatement for correction, not treat it as equivalent to overclaim.
