# Staff-Lens Audit — 2026-04-20

Ran on: `3db6279` (2026-04-20, post-Stage 2.3B)
Rubric: docs/audits/STAFF-LENS-RUBRIC.md
Previous audit: docs/audits/2026-04-20.md (pre-Stage 2.1, composite 7.6 on engineering rubric)

## Verdict

**Composite: 6.2 / 10** on the staff-lens rubric. Not directly comparable to the 7.6 engineering-lens baseline — this rubric measures honesty, voice, and coherence rather than code quality and CI gates. The site is partially calibrated (about + home + writing + contact are voice-consistent) but two InDevelopment project pages and three core writeup bodies remain uncalibrated, with unscoped metric claims across both.

## Seven-Lens Scores

| Lens | Score | Weight | Notes |
|---|---:|---:|---|
| honesty_discipline | 6 | 22 | security-baseline-audit fully methodical; identity + observability JSONs have 6 unscoped claims; resume PISCES overclaims vs about scope |
| voice_consistency | 6 | 18 | Primary surfaces calibrated post-2.3A/B; writeup bodies (cisco-ios, windows-server-lab) remain LLM-voiced; resume PISCES bullets assertive beyond stated scope |
| staff_lens_coherence | 6 | 15 | 1/6 projects has full template (security-baseline-audit); 2 InDevelopment have V1 HTML-in-JSON without decisions/methodology/lessons |
| positioning_alignment | 8 | 12 | No "senior" or "staff" self-declaration. One residual: writing.rs:280 renders "Principal Platform Architect" in demoted writeup author lines (Stage 2.3L) |
| cross_surface_consistency | 5 | 13 | PISCES: about says "did not own detections"; resume says "Authored KQL detection logic." Certs: about silent, three other surfaces claim "Pursuing/In Progress" |
| engineering_hygiene | 7 | 10 | All gates green (check/test/lint, 18 tests). Banned vocab in writeup JSONs. Dead CSS `.about-pills-row`. Untracked `.claudedoc/` |
| content_density | 6 | 10 | security-baseline-audit dense (7-min read, full template). Other projects thin: V1 HTML, no structured ADRs or methodology |

## Composite Calculation

```
(6×22 + 6×18 + 6×15 + 8×12 + 5×13 + 7×10 + 6×10) / 100
= (132 + 108 + 90 + 96 + 65 + 70 + 60) / 100
= 621 / 100
= 6.2
```

## Cross-Surface Consistency Matrix

| Claim | about.rs | home.rs | resume.rs | one_pager.rs | certs.rs | project JSON |
|---|---|---|---|---|---|---|
| PISCES role scope | "academic rotation...did not own detections or run shifts at volume" | "Monitored 13 municipal entities" (neutral) | "Triaged ~50 alerts/day...Authored KQL detection logic" — **CONTRADICTS about** | "SOC internship monitoring 13 municipal entities" (neutral) | — | — |
| Cert status | SILENT (dropped in 2.3A) | Renders certs.rs data ("In Progress") | "Pursuing (target Q3/Q4 2026)" | "Currently Studying...Target Q3/Q4 2026" | "In Progress" | — |
| Target tier | "not what title I'm claiming" | No tier claim | No tier claim | No tier claim | — | — |
| Terraform 87/92 | "passes 87 of 92 CIS GCP controls" | — | — | "CIS GCP 87/92 controls passing" | — | "87 of 92 applicable controls passing (94.5%)" + methodology ✓ |
| Stack self-reference | Silent (no Rust/WASM mention) | — | "Rust + Leptos + WASM32 portfolio" (lab section) | — | — | — |

## Findings

### P0 (ship-blocking)

**P0-1: PISCES scope contradiction between about and resume**
- File: `src/pages/resume.rs:97-100`
- Current: "Triaged ~50 alerts/day...escalated ~3–5 incidents/week to senior analysts" and "Authored KQL detection logic"
- About page: "I did not own detections or run shifts at volume"
- Desired: Resume PISCES bullets scoped to match about-page honesty calibration — activity described within rotation context, "authored" softened to "contributed to" or scoped as lab exercise
- Complexity: moderate (requires voice-consistent rewrite of 4 resume bullets)
- Stage: **2.3J** (Resume PISCES framing alignment)

**P0-2: InDevelopment project metrics lack methodology**
- File: `public/projects/identity-access-lifecycle.json`
- Current: "reduce handshake latency by 80% vs. OpenVPN", "100% stability for high-bandwidth telemetry data", "100% Compliance audit trail"
- Desired: Each metric has methodology (measurement tool, sample size, conditions) or is removed. Numbers without method are inflated claims.
- Complexity: substantial (requires re-running benchmarks or qualifying claims as design-target vs. measured)
- Stage: **2.3C** (Unsupported metrics resolution)

**P0-3: InDevelopment project metrics lack methodology (observability)**
- File: `public/projects/observability-operational-intelligence.json`
- Current: "Minutes → Seconds MTTR Reduction", "-60% Non-actionable Alert Noise", "100% Full-stack Observability Coverage"
- Desired: Methodology attached to each metric or metrics reframed as design targets
- Complexity: substantial
- Stage: **2.3C** (Unsupported metrics resolution)

### P1 (visible gap)

**P1-1: Cert status inconsistency across surfaces**
- Files: `src/data/certs.rs:13-14`, `src/pages/resume.rs:135-140`, `src/pages/one_pager.rs:144-149`, `src/pages/home.rs` (renders CertificationsSection)
- Current: About page is silent on certs; three other surfaces claim "In Progress" / "Pursuing" / "Currently Studying"
- Desired: All surfaces aligned — either all state cert intent consistently, or all drop until enrollment evidence exists
- Complexity: moderate
- Stage: **2.3H** (Cert honesty sweep)

**P1-2: Writeup bodies contain banned vocabulary and LLM voice**
- Files: `public/writeups/cisco-ios-fundamentals.json`, `public/writeups/windows-server-lab-powershell-automatedlab.json`
- Current: cisco-ios: "orchestrate multi-cloud compositions", "forged in the physical trenches", "architect a high-level Internal Developer Platform". windows-server-lab: "orchestration and OS bootstrap so the architect can focus"
- Desired: Writeup bodies rewritten in the same first-person operational voice as the about page
- Complexity: substantial (full rewrite of 2 writeup bodies)
- Stage: **2.3F** (KEEP-writeup voice pass)

**P1-3: InDevelopment project pages lack staff-lens template structure**
- Files: `public/projects/identity-access-lifecycle.json`, `public/projects/observability-operational-intelligence.json`
- Current: V1 HTML-in-JSON format with overview/architecture/outcomes sections but no structured `decisions[]`, `constraints_in[]`, `highlights[]`, `outcomes[]` with methodology, or `lessons[]`
- Desired: Full template matching security-baseline-audit.json structure
- Complexity: substantial (full case-study rewrite for each)
- Stage: **2.3D** (InDevelopment project content voice)

**P1-4: Writing.rs renders "Principal Platform Architect" in demoted writeup author line**
- File: `src/pages/writing.rs:280`
- Current: `.replace("AUTHOR: Senior Principal Platform Architect", "AUTHOR: Richard Mussell — Principal Platform Architect")`
- Desired: Author line becomes tier-neutral (e.g., "AUTHOR: Richard Mussell")
- Complexity: trivial
- Stage: **2.3L** (Writeup body author-line sweep)

### P2 (residue)

**P2-1: Dead CSS rule `.about-pills-row`**
- File: `style/pages/about.css`
- Current: Rule exists for Section 4 pills row removed in Stage 2.3A
- Desired: Rule deleted
- Complexity: trivial
- Stage: unlisted (trivial CSS cleanup, attach to next about-page stage)

**P2-2: Sixteen-agent-orchestrated-audit writeup uses banned vocabulary**
- File: `public/writeups/sixteen-agent-orchestrated-audit.md:1,19`
- Current: Title "Orchestrating a 22-Agent Audit", body "pipeline was orchestrated by Opus 4.6"
- Desired: Retitle to avoid banned word (e.g., "Running a 22-Agent Audit Pipeline") or document as meta-writeup where "orchestration" is technically accurate
- Complexity: trivial
- Stage: **2.3F** (KEEP-writeup voice pass)

**P2-3: `src/data/writeups.rs:189` — slug contains "orchestrator"**
- File: `src/data/writeups.rs:189`
- Current: slug `"the-orchestrator-of-intent-reflections-on-service-provisioning"`
- Desired: This is a URL slug, not user-visible rendered text. The display title was retitled in Stage 2.2A. Slug preserved for URL continuity — acceptable as-is.
- Complexity: none (documented exception)
- Stage: N/A

**P2-4: Untracked `.claudedoc/` directory at repo root**
- Current: Working directory from Claude sessions; untracked
- Desired: Either `.gitignore` the directory or remove it
- Complexity: trivial
- Stage: N/A (housekeeping)

### P3 (long-horizon)

**P3-1: Planned projects need case-study content (3 projects)**
- Files: `public/projects/endpoint-management-compliance.json`, `public/projects/backup-recovery-continuity.json`, `public/projects/operational-foundation.json`
- Current: Empty content (correctly — status is Planned)
- Desired: As projects ship, full staff-lens template populated
- Stage: per-project build stages (post-2.3 series)

**P3-2: OG image stale**
- File: `scripts/og-image-template.html`, `public/og-image.png`
- Current: Still says "Systems Engineer" and stale tech list
- Desired: Regenerated with current PROFESSIONAL_TITLE and positioning
- Stage: **2.3G** (OG image regeneration)

**P3-3: Demo pages serve "Coming Soon" placeholders**
- File: `src/pages/project/demo.rs`
- Current: All four project demos render hardcoded placeholder
- Desired: Real demos or removal of demo tab for projects without them
- Stage: Post-2.3 series (content per ADR-005)

**P3-4: Resume PDF quality unverified**
- File: `public/pdfs/resume.pdf`
- Current: CI validates size only; content currency and formatting unknown
- Desired: Content verified against site narrative; "Last updated" date added
- Stage: Content P0 per ADR-005 (resume PDF refresh)

## Delta from 2026-04-20 baseline

### Closed since baseline (pre-Stage 2.1)

| Baseline finding | Status |
|---|---|
| P2-6: Title consistency / positioning drift | **Partially closed.** PROFESSIONAL_TITLE constant is now "Linux Systems Administrator" consistently across all page titles and headers. About page opens with no-tier framing. However, some V1 project blurb language in resume.rs still uses older project framing. |
| P1-2: CLAUDE.sysadmin-portfolio.md untracked | **Closed.** File no longer exists in working tree. |

### Still open from baseline

| Baseline finding | Status |
|---|---|
| P2-1: Cert honesty | **Open.** Same issue, now worse: about page is silent (2.3A removed cert language) while three other surfaces still claim "In Progress." Now a cross-surface inconsistency (P1-1 in this audit). |
| P2-2: Demo placeholders | **Open.** Unchanged. Now P3-3. |
| P2-3: Work history unquantified | **Partially addressed.** Resume PISCES bullets now have activity counts (~50 alerts/day, ~3-5 incidents/week), but these contradict the about-page scope framing. Quantification landed but honesty calibration didn't. Escalated to P0-1. |
| P2-5: Resume PDF quality | **Open.** Now P3-4. |
| P1-1: Regenerate ARCHITECTURE.md | **Not measured by this rubric** (engineering documentation, not user-visible content). |

### Newly visible (post-Stage 2.3A voice calibration)

These findings exist BECAUSE the about page is now calibrated — the calibration creates contradictions against surfaces that haven't been updated:

1. **PISCES scope contradiction** (P0-1) — about page honesty creates tension with resume's assertive bullets
2. **Cert silence vs. claims** (P1-1) — about page dropped certs; other surfaces didn't follow
3. **Writeup voice gap now stark** (P1-2) — reading calibrated about page then clicking into an LLM-voiced writeup body produces a jarring disconnect that was less noticeable when everything was uncalibrated

## Recommended stage ordering

Based on audit evidence, the priority order should be:

1. **2.3C — Unsupported metrics resolution** (P0-2, P0-3). Ship-blocking. Unscoped claims in InDevelopment project pages. Highest honesty-discipline impact.
2. **2.3J — Resume PISCES framing alignment** (P0-1). Ship-blocking. Cross-surface contradiction with the voice anchor.
3. **2.3H — Cert honesty sweep** (P1-1). Visible gap. Quick alignment pass across four surfaces.
4. **2.3D — InDevelopment project content voice** (P1-3). Visible gap. Brings identity + observability to the same template as security-baseline-audit.
5. **2.3F — KEEP-writeup voice pass** (P1-2, P2-2). Voice consistency. Rewrites 2-3 writeup bodies to match the anchor.
6. **2.3L — Writeup body author-line sweep** (P1-4). Trivial positioning fix.
7. **2.3G — OG image regeneration** (P3-2). External-facing but not on-site.
8. **2.3E — Platform page build**. Long-horizon strategic value but blocked until InDevelopment content is dense.
9. **2.3I — Senior-tier residue sweep**. Low priority — grep confirms no self-declarations currently.
10. **2.3K — Internal resume variants**. Internal docs, lowest priority.

**Rationale for reordering:** The original closeout listed stages alphabetically (C through L). This audit reorders by impact on composite score. 2.3C addresses the heaviest-weighted lens (honesty_discipline, weight 22). 2.3J addresses the only cross-surface P0. Stages 2.3I and 2.3K drop because the audit found no "senior" self-declarations — the work Stage 2.3A anticipated may already be resolved.

## What this audit did not measure

- Visual design quality (layout, typography, spacing, color)
- Engineering code quality beyond gate pass/fail (architecture patterns, idiomatic Rust)
- Dependency freshness or security vulnerability scan
- Accessibility compliance (ARIA roles present but not functionally tested)
- SEO effectiveness (meta tags present but not validated against crawlers)
- Mobile responsiveness
- Load performance / Lighthouse scores

## Appendix A — Banned-word sweep output

### User-visible (P1)

| File | Line | Word | Context | Severity |
|---|---|---|---|---|
| `public/writeups/cisco-ios-fundamentals.json` | 1 | orchestrate | "orchestrate multi-cloud compositions" | P1 (writeup body) |
| `public/writeups/cisco-ios-fundamentals.json` | 1 | architect | "one can architect a high-level Internal Developer Platform" | P1 (writeup body) |
| `public/writeups/windows-server-lab-powershell-automatedlab.json` | 1 | orchestration | "Hyper-V orchestration and OS bootstrap" | P1 (writeup body) |
| `public/writeups/sixteen-agent-orchestrated-audit.md` | 1 | Orchestrating | "Orchestrating a 22-Agent Audit" (title) | P2 (meta-writeup) |
| `public/writeups/sixteen-agent-orchestrated-audit.md` | 19 | orchestrated | "pipeline was orchestrated by Opus 4.6" | P2 (meta-writeup) |

### Acceptable / grandfathered

| File | Line | Word | Reason |
|---|---|---|---|
| `src/pages/resume.rs` | 149 | Orchestration | "Cloud & Orchestration" — legitimate technical category per Stage 2.1 decision |
| `src/data/writeups.rs` | 189 | orchestrator | URL slug only, not rendered display text |
| `docs/restructure/*` | various | various | Build documentation, not user-visible |

## Appendix B — Voice-anchor deviation matrix

| Surface | Classification | Representative quote |
|---|---|---|
| about.rs (anchor) | CALIBRATED | "I spent a term at PISCES...I did not own detections or run shifts at volume. What I took from it was a stance, not a skill." |
| home.rs hero | SAME_VOICE | "SOC-trained Linux administrator. I build the automation, identity, and observability layers that keep hybrid-cloud environments running when nobody's looking." |
| writing.rs intro | SAME_VOICE | "Technical notes from lab work and prior experience — Linux hardening, compliance automation, SOC observability, zero-trust networking." |
| contact.rs | SAME_VOICE | "Linux Systems Administrator with a BS in IT & Administrative Management. I build repeatable infrastructure with Terraform..." |
| resume.rs summary | MINOR_DRIFT | "Linux Systems Administrator with a BS in IT & Administrative Management...Hands-on experience in SOC monitoring, ELK Stack log analysis..." (compressed resume format, acceptable) |
| resume.rs PISCES | MAJOR_DRIFT | "Triaged ~50 alerts/day...Authored KQL detection logic for anomalous endpoint behavior and lateral movement indicators." (asserts ownership the about page explicitly disclaims) |
| one_pager.rs | MINOR_DRIFT | "I build repeatable infrastructure with Terraform, automate system administration with PowerShell and Bash..." (direct but less scope-aware than anchor) |
| cisco-ios writeup body | UNCALIBRATED_RESIDUE | "my lens was built in a high-school Cisco lab environment...This was my first experience building a Universal Control Plane from the ground up, ensuring the connectivity fabric was resilient, secure, and ready to scale." |
| windows-server-lab writeup body | UNCALIBRATED_RESIDUE | "Utilizing PowerShell and AutomatedLab to transform raw ISOs into a fully-formed enterprise 'meal.' This is the pinnacle of Declarative Intent..." |
| identity-access-lifecycle JSON | MINOR_DRIFT | "Legacy 'Castle-and-Moat' VPNs grant excessive lateral trust once a perimeter is breached." (V1 generic technical voice — not wrong, just not the anchor's first-person specificity) |
| observability JSON | MINOR_DRIFT | "Legacy monitoring often results in 'data silos' and high alert cardinality, leading to operator fatigue." (same V1 pattern) |

## Appendix C — Gate status

### `just check` (4 cargo check targets)

```
cargo check --target wasm32-unknown-unknown
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```
All four gates: **PASS**

### `just test`

```
running 17 tests
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
running 1 test (integration)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
18 total tests: **PASS**

### `just lint`

```
cargo fmt --check: clean (exit 0)
cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean (exit 0)
```
**PASS**
