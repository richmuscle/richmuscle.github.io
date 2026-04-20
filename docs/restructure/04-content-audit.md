# 04 — Whole-Site Content Audit (Stage 2.0)

Produced: 2026-04-20, branch `revamp` @ `e912b41`.
Target identity: **Linux Systems Administrator with modern infrastructure toolkit.**
Target voice: direct, operational, first-person, honest about limits.

---

## Section 1 — Executive Summary

| Metric | Count |
|---|---|
| Total items audited | 68 |
| ALIGNED (keep as-is) | 12 |
| DRIFT-MINOR (mechanical fix) | 34 |
| DRIFT-MAJOR (voice rewrite) | 14 |
| RETIRE | 5 |
| VOICE-CRITICAL (owner review needed) | 3 |

| Priority | Count |
|---|---|
| P0 (credibility-breaking) | 16 |
| P1 (visible-but-survivable) | 22 |
| P2 (quality-of-polish) | 18 |
| P3 (nice-to-have) | 12 |

**Top 5 P0 items:**
1. `PROFESSIONAL_TITLE` constant says "Systems Administrator & DevOps Engineer" — every page that reads it renders the wrong identity
2. `index.html` — 9 hardcoded identity strings in title/meta/OG/JSON-LD
3. `site_footer.rs:21` — three-role footer label
4. `resume.rs:189,201,207` — three banned-word violations in project summaries visible to every recruiter
5. `identity-access-lifecycle.json` + `observability-operational-intelligence.json` — banned-word hits in V1 HTML content

---

## Section 2 — P0 Queue (Credibility-Breaking)

### P0-01: PROFESSIONAL_TITLE constant
**File:** `src/data/mod.rs:20`
**Current:** `"Systems Administrator & DevOps Engineer"`
**Classification:** DRIFT-MINOR
**Fix:** Change to `"Linux Systems Administrator"`
**Effort:** Trivial
**Impact:** Every page using the constant updates automatically (home, about, resume, contact, one-pager title tags)

### P0-02: index.html identity strings (9 locations)
**File:** `index.html:18,19,20,21,28,29,97,98,153`
**Current:** `"Systems Administrator & DevOps Engineer"` in title, meta, OG, Twitter, JSON-LD, noscript
**Classification:** DRIFT-MINOR
**Fix:** Replace all 9 with "Linux Systems Administrator" and adjust surrounding copy
**Effort:** Trivial (string swap)
**SEO impact:** Yes — OG cards, JSON-LD jobTitle, page title all change

### P0-03: Footer three-role label
**File:** `src/components/site_footer.rs:21`
**Current:** `"Systems Administrator · DevOps · Platform Engineer · Oklahoma City"`
**Classification:** DRIFT-MINOR
**Fix:** `"Linux Systems Administrator · Oklahoma City"` (per resolved CLAUDE.md)
**Effort:** Trivial

### P0-04: Hero subtitle
**File:** `src/pages/home.rs:96`
**Current:** `"Systems Administrator & DevOps Engineer"`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE` constant (after P0-01 fix)
**Effort:** Trivial

### P0-05: Resume header
**File:** `src/pages/resume.rs:25`
**Current:** `"Systems Administrator & DevOps Engineer"`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P0-06: Resume banned words — "Engineered" x2
**File:** `src/pages/resume.rs:189,207`
**Current:** `"Summary: Engineered a secure, modular landing zone..."` and `"Summary: Engineered an identity-based SASE administrative fabric..."`
**Classification:** DRIFT-MINOR
**Fix:** Replace "Engineered" → "Built" (189), "Deployed" (207)
**Effort:** Trivial

### P0-07: Resume banned word — "Architected" x2
**File:** `src/pages/resume.rs:189,201`
**Current:** `"...architected 'Private-First' VPCs..."` and `"Architected a multi-tier observability pipeline..."`
**Classification:** DRIFT-MINOR
**Fix:** Replace "architected" → "designed" (189), "Architected" → "Built" (201)
**Effort:** Trivial

### P0-08: One-pager title
**File:** `src/pages/one_pager.rs:20`
**Current:** `"Systems Administrator & DevOps Engineer"`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P0-09: One-pager meta description
**File:** `src/pages/one_pager.rs:15`
**Current:** `"...platform engineering, DevOps, and systems engineering targets..."`
**Classification:** DRIFT-MINOR
**Fix:** Rewrite to reference "Linux systems administration, IaC, and infrastructure automation"
**Effort:** Trivial

### P0-10: identity-access-lifecycle.json banned words
**File:** `public/projects/identity-access-lifecycle.json` (single-line HTML content)
**Current:** `"engineers a Zero-Trust..."`, `"Architected WireGuard Peer identities..."`
**Classification:** DRIFT-MINOR
**Fix:** Replace "engineers" → "deploys", "Architected" → "Configured"
**Effort:** Trivial (in-place string edits in HTML content)

### P0-11: observability-operational-intelligence.json banned words
**File:** `public/projects/observability-operational-intelligence.json` (single-line HTML content)
**Current:** `"Orchestrated a Prometheus-based..."`, `"Engineered Logstash filters..."`, `"Architected Grafana dashboards..."`
**Classification:** DRIFT-MINOR
**Fix:** "Orchestrated" → "Configured", "Engineered" → "Built", "Architected" → "Designed"
**Effort:** Trivial

### P0-12: identity-access-lifecycle.json unsupported claims
**File:** `public/projects/identity-access-lifecycle.json`
**Current:** `"reduce handshake latency by 80% vs. OpenVPN"` — no methodology. `"100% stability for high-bandwidth telemetry data"` — no methodology.
**Classification:** DRIFT-MAJOR
**Fix:** Either add methodology (what was measured, how, when) or remove the unsupported claim. "80% vs. OpenVPN" needs a benchmark description.
**Effort:** Moderate (requires honest assessment of whether the measurement exists)
**Voice-critical:** Yes — owner must confirm whether the 80% claim is real

### P0-13: observability-operational-intelligence.json unsupported claims
**File:** `public/projects/observability-operational-intelligence.json`
**Current:** `"Minutes → Seconds MTTR Reduction"` — no baseline, no method. `"-60% Non-actionable Alert Noise"` — no baseline, no method. `"100% Full-stack Observability Coverage"` — undefined metric.
**Classification:** DRIFT-MAJOR
**Fix:** Add methodology or replace with honest qualitative statements
**Effort:** Moderate
**Voice-critical:** Yes — owner must provide the actual measurement data or acknowledge these are estimates

### P0-14: About page banned word
**File:** `src/pages/about.rs:98`
**Current:** `"...infrastructure remains a seamless service for the business."`
**Classification:** DRIFT-MINOR
**Fix:** Replace "seamless" → "reliable" or "transparent"
**Effort:** Trivial

### P0-15: README identity strings
**File:** `README.md:3,175`
**Current:** `"Systems Administrator & DevOps Engineer"` in both locations
**Classification:** DRIFT-MINOR
**Fix:** Update to match `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P0-16: manifest.json identity variant
**File:** `manifest.json:4`
**Current:** `"Systems Administrator & Platform Operations"` — unique variant, inconsistent with everything
**Classification:** DRIFT-MINOR
**Fix:** Update to "Linux Systems Administrator — Portfolio"
**Effort:** Trivial

---

## Section 3 — P1 Queue (Visible-but-Survivable)

### P1-01: About page — third person, not first
**File:** `src/pages/about.rs:24`
**Current:** `"Richard Mussell is a Systems Administrator and DevOps Engineer..."`
**Classification:** DRIFT-MAJOR (voice rewrite)
**Fix:** Rewrite to first-person sysadmin voice. Owner must review.
**Effort:** Moderate

### P1-02: About page — aspirational language
**File:** `src/pages/about.rs:40`
**Current:** `"Where I have hands-on experience and where I am actively building depth."`
**Classification:** DRIFT-MINOR
**Fix:** Replace "actively building depth" with specific current work
**Effort:** Trivial

### P1-03: Hero body — "infrastructure engineer" framing
**File:** `src/pages/home.rs:98`
**Current:** `"SOC-trained infrastructure engineer. I build the automation..."`
**Classification:** DRIFT-MINOR
**Fix:** Replace "infrastructure engineer" → "systems administrator" or remove the label and let the work speak
**Effort:** Trivial

### P1-04: Hero meta — "Systems Engineering"
**File:** `src/pages/home.rs:106`
**Current:** `"Systems Engineering & Lab Projects"`
**Classification:** DRIFT-MINOR
**Fix:** "Systems Administration & Lab Projects"
**Effort:** Trivial

### P1-05: Writing page title — "Systems Engineering"
**File:** `src/pages/writing.rs:63`
**Current:** `"Writing · Richard Mussell · Systems Engineering"`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P1-06: Writing page intro — LLM voice
**File:** `src/pages/writing.rs:70`
**Current:** `"Architectural manifestos and operational deep-dives focused on orchestrating equilibrium within high-integrity ecosystems..."`
**Classification:** DRIFT-MAJOR (voice rewrite)
**Fix:** Rewrite to direct sysadmin voice: "Technical notes on Linux hardening, compliance automation, and operational tooling."
**Effort:** Trivial (one line)

### P1-07: Contact page meta — DevOps mention
**File:** `src/pages/contact.rs:15`
**Current:** `"...available for sysadmin, DevOps, and infrastructure roles."`
**Classification:** DRIFT-MINOR
**Fix:** "...available for Linux systems administration and infrastructure roles."
**Effort:** Trivial

### P1-08: Contact subtext — DevOps identity
**File:** `src/pages/contact.rs:22`
**Current:** `"Systems Administrator and DevOps Engineer with a BS..."`
**Classification:** DRIFT-MINOR
**Fix:** Use identity-aligned copy
**Effort:** Trivial

### P1-09: Resume summary — DevOps identity
**File:** `src/pages/resume.rs:57`
**Current:** `"Systems Administrator and DevOps Engineer with a BS..."`
**Classification:** DRIFT-MINOR
**Fix:** "Linux Systems Administrator with a BS..."
**Effort:** Trivial

### P1-10: Resume meta — DevOps identity
**File:** `src/pages/resume.rs:15`
**Current:** `"Resume of Richard Mussell — Systems Administrator & DevOps Engineer..."`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P1-11: Resume — "Platform Engineering" label
**File:** `src/pages/resume.rs:117`
**Current:** `"Self-directed Platform Engineering / Homelab"`
**Classification:** DRIFT-MINOR
**Fix:** "Self-directed Systems Administration & Infrastructure Lab"
**Effort:** Trivial

### P1-12: About page meta — DevOps identity
**File:** `src/pages/about.rs:9`
**Current:** `"About Richard Mussell — Systems Administrator & DevOps Engineer..."`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P1-13: Not-found page meta — DevOps identity
**File:** `src/pages/not_found.rs:11`
**Current:** `"...Richard Mussell Systems Administrator & DevOps Engineer Portfolio."`
**Classification:** DRIFT-MINOR
**Fix:** Use `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P1-14: security-baseline-audit JSON — landing-zone voice
**File:** `public/projects/security-baseline-audit.json` (problem field, line 11)
**Current:** `"In a homelab GCP environment, onboarding a new environment...was a ~4-hour console click-through"`
**Classification:** DRIFT-MAJOR (voice reframe)
**Fix:** Retone from "I was provisioning environments" to "I needed continuous compliance enforcement and drift detection"
**Effort:** Moderate
**Voice-critical:** Yes

### P1-15: security-baseline-audit JSON — approach field
**File:** `public/projects/security-baseline-audit.json` (approach field, line 27)
**Current:** `"Modular Terraform registry...producing a Private-First VPC with Cloud NAT..."`
**Classification:** DRIFT-MAJOR (voice reframe)
**Fix:** Retone emphasis from VPC topology to compliance enforcement mechanism
**Effort:** Moderate
**Voice-critical:** Yes

### P1-16: About page — "How I Think About Systems" section voice
**File:** `src/pages/about.rs:92-98`
**Current:** Academic/essay voice discussing monitoring philosophy in third-person abstractions
**Classification:** DRIFT-MAJOR
**Fix:** Rewrite to first-person operational voice with specific tool references
**Effort:** Substantial
**Voice-critical:** Yes

### P1-17: Home page project description line
**File:** `src/pages/home.rs:148`
**Current:** `"Terraform IaC for reliability · PowerShell/GPO & Windows Server 2022/Linux automation..."`
**Classification:** DRIFT-MINOR
**Fix:** Update to reflect current project mix (no Windows Server 2022 project exists; this is aspirational)
**Effort:** Trivial

### P1-18: Telemetry page title — no PROFESSIONAL_TITLE
**File:** `src/pages/telemetry.rs` (Title tag)
**Current:** `"Telemetry | Richard J. Mussell"` — omits professional title
**Classification:** DRIFT-MINOR
**Fix:** Include `PROFESSIONAL_TITLE`
**Effort:** Trivial

### P1-19: Resume projects — hardcoded list still present
**File:** `src/pages/resume.rs:185-209`
**Current:** 4 hardcoded projects with titles/summaries not sourced from registry
**Classification:** DRIFT-MAJOR (structural + content)
**Fix:** Source from registry; only show Shipped + InDevelopment projects
**Effort:** Moderate

### P1-20: Writing page content replacements — inflated title
**File:** `src/pages/writing.rs:269` (string replacement logic)
**Current:** Replaces `"AUTHOR: Senior Principal Platform Architect"` with `"AUTHOR: Richard Mussell -- Principal Platform Architect"`
**Classification:** DRIFT-MINOR
**Fix:** Replace with "AUTHOR: Richard Mussell — Linux Systems Administrator"
**Effort:** Trivial

### P1-21: JSON-LD knowsAbout array
**File:** `index.html:105-111`
**Current:** `"Systems Administration", "Kubernetes", "Terraform", "Linux Hardening", "Active Directory", "NIST Framework"`
**Classification:** ALIGNED (mostly)
**Fix:** Consider replacing "Systems Administration" with "Linux Systems Administration" for specificity
**Effort:** Trivial

### P1-22: Resume three-variant strategy
**File:** `docs/resumes/resume-sysadmin.md`, `resume-devops.md`, `resume-platform.md`
**Current:** Three variants targeting Junior-to-Mid at different roles ($70K-$120K)
**Classification:** DRIFT-MAJOR
**Fix:** Update sysadmin variant to Senior target ($100K+); keep devops/platform as internal application-time aids. Do not surface variant selection on site.
**Effort:** Moderate
**Voice-critical:** Yes — owner must confirm target comp and title

---

## Section 4 — P2 Queue (Polish)

### P2-01 through P2-08: Meta description consistency sweep
Every `<Meta name="description">` currently hardcodes the full identity string instead of interpolating `PROFESSIONAL_TITLE`. Files: `home.rs:87`, `about.rs:9`, `resume.rs:15`, `contact.rs:15`, `one_pager.rs:15`, `not_found.rs:11`, `writing.rs:64`. Each needs the identity portion updated to match P0-01.
**Effort:** Trivial per file, batch as one task

### P2-09: Home page hero — second paragraph claims
**File:** `src/pages/home.rs:101`
**Current:** `"Shipped zero-touch Windows deployment with Intune/Autopilot, WSUS patch automation, and 3-2-1 DR in lab."`
**Classification:** Status-honesty check — are these shipped or planned?
**Fix:** Verify each claim against actual project state; downgrade to "building" if not shipped
**Effort:** Trivial once owner confirms

### P2-10: About page — "Adjacent Technologies" pills
**File:** `src/pages/about.rs:108-113`
**Current:** 12 tech pills including "Crossplane", "Pulumi", "Helm Charts", "SPIRE/SPIFFE", "Cilium", "Talos Linux"
**Classification:** Status-honesty — these are aspirational, not demonstrated
**Fix:** Label section honestly ("Technologies I'm studying" or remove items not yet demonstrated)
**Effort:** Trivial

### P2-11: Home page — project description subtitle
**File:** `src/pages/home.rs:140-148`
**Current:** Static subtitle describing "4 disciplines" — stale after category changes
**Fix:** Update to match current project mix
**Effort:** Trivial

### P2-12 through P2-18: Category descriptions stale
**File:** `src/data/projects.rs` — `ProjectCategory::description()` method
Several descriptions are generic: `"Linux administration automation via scripting and repeatable runbooks."` — doesn't cover the security-baseline-audit project's Terraform/CIS focus.
**Fix:** Update category descriptions to match current project mix, or suppress them if not surfaced anywhere meaningful.
**Effort:** Trivial per description

---

## Section 5 — P3 Queue (Nice-to-Have)

### P3-01 through P3-05: Writeup retitles
Five writeups flagged for retitle in 04-writeup-verdicts.md. Each needs a new title in sysadmin voice. Low urgency — all writeup JSONs are stubs or essay-style content that won't be the first thing a hiring manager reads.

### P3-06 through P3-10: Writeup retirements
Five writeups flagged for retirement. Execution is: remove from writeups.rs, delete JSON files, update sitemap.

### P3-11: Writeup demotions
Three writeups flagged for demotion (keep URL, remove from index listing).

### P3-12: PDF retirements
Four PDFs paired with retired writeups should follow the same fate.

---

## Section 6 — Writeup Verdict Reconfirmation

**CORRECTION:** The earlier 04-writeup-verdicts.md said all 17 writeup JSONs were "stub — no content to quote." This was based on file line count (1 line = minified JSON). Several actually contain substantial HTML content in their `content` field:

| Slug | Actual content | Prior verdict | Updated verdict | Reason |
|---|---|---|---|---|
| hardening-linux-municipal-environments | ~200 words | KEEP | **KEEP** | Confirmed — core sysadmin topic |
| automating-nist-800-53-compliance-with-terraform | ~400 words | KEEP | **KEEP** | Confirmed — IaC compliance |
| zero-trust-moving-beyond-bastion-hosts | ~250 words | KEEP | **KEEP** | Confirmed — operational networking |
| siem-alert-hygiene-reducing-noise-in-the-soc | ~250 words | KEEP | **KEEP** | Confirmed — SOC ops |
| kubernetes-controller-reconciliation-deep-dive | ~1500 words | DEMOTE | **DEMOTE** | Confirmed — K8s dev, not core sysadmin |
| otel-ebpf-tracing-without-instrumentation | ~1200 words | DEMOTE | **DEMOTE** | Confirmed — adjacent observability |
| rust-wasm-edge-runtime-internals | ~1200 words | RETIRE | **RETIRE** | Confirmed — Rust dev, not sysadmin |
| ebpf-from-zero-to-prod | ~1500 words | DEMOTE | **KEEP** (flip) | Has substantial Linux kernel content; more aligned than initially assessed |
| the-orchestrator-of-intent-... | ~600 words | RETITLE | **RETITLE** | Confirmed — telecom experience, retitle to operational voice |
| the-architect-of-oceanic-visibility-... | ~500 words | RETITLE | **RETITLE** | Confirmed — SOC content, overwrought title |
| the-connectivity-fabric-... | ~600 words | RETITLE | **RETITLE** | Confirmed — networking fundamentals |
| the-orchestrated-landscape-... | ~1000 words | RETIRE | **RETIRE** | Confirmed — metaphysical manifesto, banned words in body |
| the-mirror-universe-... | ~500 words | RETITLE | **RETITLE** | Confirmed — AD/PowerShell, operational beneath the title |
| the-sustainable-architect-... | ~1000 words | RETIRE | **RETIRE** | Confirmed — abstract, banned word in title |
| the-builders-ledger-... | ~800 words | RETIRE | **RETIRE** | Confirmed — governance manifesto, "orchestrating" in title |
| universal-dialects-... | ~900 words | RETITLE | **RETITLE** | Confirmed — Linux/Shell content, overwrought title |
| the-architect-of-the-prismatic-apex-... | ~1000 words | RETIRE | **RETIRE** | Confirmed — metaphysical essay, multiple banned words |

**Verdict summary (updated):** 5 KEEP, 5 RETITLE, 2 DEMOTE, 5 RETIRE.

One flip: `ebpf-from-zero-to-prod` moved from DEMOTE to KEEP — the content covers kernel-level verifier constraints, CO-RE portability, and ring-buffer tuning, which is legitimately Linux systems depth.

---

## Section 7 — Recommended Phase 2.1 Sequence

### Proposed first five items

| Order | Item | Rationale |
|---|---|---|
| 1 | **P0-01 + P0-02 + P0-03 + P0-04 + P0-05 + P0-08 + P0-15 + P0-16** (identity sweep) | Single pass updates PROFESSIONAL_TITLE, index.html, footer, README, manifest. Every page fixes itself. This is the highest-leverage single commit — after it lands, the site says "Linux Systems Administrator" everywhere. |
| 2 | **P0-06 + P0-07 + P0-10 + P0-11 + P0-14** (banned-word sweep) | Mechanical string replacements. No judgment calls. Can be verified by grep. |
| 3 | **P1-14 + P1-15** (security-baseline-audit voice reframe) | The anchor work: retone problem + approach from "landing zone provisioning" to "continuous compliance enforcement." Two paragraphs, same facts, different emphasis. Voice-critical — owner reviews draft. |
| 4 | **P0-12 + P0-13** (unsupported claims) | Owner provides methodology for the 80% latency claim, MTTR claim, 60% noise reduction, or acknowledges they're estimates. Cannot be done without owner input. |
| 5 | **P1-01 + P1-06 + P1-16** (about page + writing page voice rewrites) | The largest voice-rewrite batch. About page moves to first-person. Writing page intro drops the LLM manifesto language. Voice-critical. |

### Defense of this sequence

The identity sweep (item 1) is first because it is the highest-leverage, lowest-risk change. Every subsequent item builds on correct identity. The banned-word sweep (item 2) follows because it is mechanical and removes the most visible P0 violations.

The alternative: starting with the security-baseline-audit reframe (item 3) because it is the deepest content work and anchors the rest. I rejected this because the reframe requires owner voice review and blocks on input — better to ship the mechanical fixes while the owner reviews the reframe draft.

Items 4 and 5 require owner input (methodology confirmation, voice review). They are sequenced last in the first batch to allow parallel progress: mechanical fixes ship while voice work is drafted and reviewed.

---

## Section 8 — Structural Residue

Issues discovered during the audit that are structural, not content:

### S-01: resume.rs project list still hardcoded
**File:** `src/pages/resume.rs:185-209`
Four projects hardcoded with titles, subtitles, summaries, and tech stacks duplicated from the registry. Should iterate the registry and filter by `ProjectStatus::Shipped` or `InDevelopment`.
**Severity:** P1 — violates scalability contract
**Fix scope:** Moderate refactor of the resume projects section

### S-02: Sitemap missing three planned projects
**File:** `public/sitemap.xml`
Only 3 project URLs listed. The three planned projects have no sitemap entries. Per the status-gated design, planned projects should either be in the sitemap at low priority or omitted entirely (the design doc says "not listed" for Planned).
**Severity:** P2 — the current state (omitted) matches the design intent

### S-03: Category descriptions stale
**File:** `src/data/projects.rs` — `ProjectCategory::description()` method
The `SystemsAdmin` description says "Linux administration automation via scripting and repeatable runbooks" but 4 of 5 projects in that category have nothing to do with that description (security-baseline-audit is Terraform/CIS, the three Planned projects have no content).
**Severity:** P2 — the descriptions render on the home page category section headers

### S-04: Writing page core_order array references retired slugs
**File:** `src/pages/writing.rs:15-25`
The `core_order` array determines the display order of "core" writeups. It contains 9 slugs, 5 of which are flagged for RETIRE in the writeup verdicts. After retirement, the ordering logic should be updated.
**Severity:** P3 — only matters after writeup retirements execute
