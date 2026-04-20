# Phase 1 Verification Report

Verified on `revamp` @ `2b32fdd` via `trunk serve --port 8002`.
Trunk dev server serves the SPA shell (index.html) for all paths;
rendering happens client-side in WASM. JSON content files verified
via direct HTTP fetch to confirm real data vs HTML fallback.

---

## 1. Runtime Verification

### Routes in the registry (3 canonical projects)

| Route | Status | What the user sees |
|---|---|---|
| `/` | **Renders — stale count** | Home page with 3 project cards. Heading says "4 Projects · 4 Disciplines" (hardcoded at `home.rs:139` — stale, should be 3). "Cloud Infrastructure" filter tab shows count 0 (terraform-gcp was removed from registry). Filter tabs functional otherwise. |
| `/about` | **Renders — identity drifted** | About page with "Systems Administrator and DevOps Engineer" in body text and meta description. Deferred to Phase 2 identity sweep. Functional. |
| `/writing` | **Renders correctly** | Writing index listing all 17 writeups. Search and category filter functional. |
| `/platform` | **404** | No route registered in `src/lib.rs`. Catches on `/*any` → NotFoundPage. User sees: `error[E0425]: cannot find route /platform in scope`. |
| `/project/security-baseline-audit` | **Renders — dissonant** | Full V2 case study: hero metrics strip (87/92 CIS, 4h→5min, 0 public IPs), problem section, 4 architecture decisions, 3 code highlights, 4 outcome metrics with methodology, 3 lessons. Title and subtitle align ("Security Baseline & Continuous Audit Toolkit"). **Tech pills show old linux-admin-scripting stack** (Bash, Linux, Cron, RBAC, Systems Hardening, GPO for Linux, Sysctl) — not the Terraform/GCP/tfsec tech in the case study body. See Section 3 for full dissonance assessment. |
| `/project/identity-access-lifecycle` | **Renders correctly** | V1 case study (inner_html). Title: "Identity & Access Lifecycle Platform". Content covers WireGuard ZTNA, AD integration, micro-segmentation. Stats strip shows 1→0 public entry points, <15ms overhead, 100% audit trail. Tabs (Case Study / Docs / Demo) functional. |
| `/project/observability-operational-intelligence` | **Renders correctly** | V1 case study (inner_html). Title: "Observability & Operational Intelligence Platform". Content covers Prometheus scraping, ELK enrichment, Grafana SLO dashboards, delta alerting. Stats: MTTR minutes→seconds, -60% alert noise. Tabs functional. |
| `/resume` | **Renders — identity drifted** | Resume page with "Systems Administrator & DevOps Engineer" in header and summary. Hardcoded project summaries use old titles (unchanged). Functional. |
| `/contact` | **Renders — identity drifted** | Contact page with "Systems Administrator and DevOps Engineer" in subtext. Functional. |
| `/one-pager` | **Renders correctly** | One-pager with 3 featured projects under canonical slugs. Title still says "Systems Administrator & DevOps Engineer" (deferred). |
| `/telemetry` | **Renders correctly** | Telemetry dashboard. Network probes now target 3 canonical slugs. |

### Routes for planned projects (NOT in registry)

| Route | Status | What the user sees |
|---|---|---|
| `/project/endpoint-management-compliance` | **404** | "404 / Project not found. ← Return to base" |
| `/project/backup-recovery-continuity` | **404** | Same 404 fallback |
| `/project/operational-foundation` | **404** | Same 404 fallback |

### Legacy redirect routes

| Route | Status | Redirects to |
|---|---|---|
| `/project/linux-admin-scripting` | **Redirects correctly** | → `/project/security-baseline-audit` (replace:true, URL bar updates) |
| `/project/zero-trust-networking` | **Redirects correctly** | → `/project/identity-access-lifecycle` |
| `/project/monitoring-observability` | **Redirects correctly** | → `/project/observability-operational-intelligence` |
| `/project/terraform-gcp` | **Redirects correctly** | → `/project/security-baseline-audit` |

### JSON content file verification

| Slug | /projects/ | /docs/ | /demos/ |
|---|---|---|---|
| security-baseline-audit | **Real JSON** (V2 structured, 117 lines) | **Real JSON** (V2 docs, 245 lines) | **Real JSON** (V2 demo, 110 lines) |
| identity-access-lifecycle | **Real JSON** (V1 HTML content) | **Real JSON** (V1 HTML content) | No file (HTML fallback → fetch error → V1 demo steps render from Rust) |
| observability-operational-intelligence | **Real JSON** (V1 HTML content) | **Real JSON** (V1 HTML content) | No file (same pattern) |
| endpoint-management-compliance | **No file** (HTML fallback) | **No file** | **No file** |
| backup-recovery-continuity | **No file** | **No file** | **No file** |
| operational-foundation | **No file** | **No file** | **No file** |

---

## 2. Planned-Project Rendering State

**Three planned projects are not in the registry.** They render as generic
project-level 404 pages ("Project not found. ← Return to base"). This is
a **Phase 1.5 gap** that must be resolved before Phase 2 content work.

### Why this is a gap

The six-project canonical frame was established in Phase 1, but only
three entries were added to the registry (the three with existing
content). The other three — endpoint-management-compliance,
backup-recovery-continuity, operational-foundation — were deferred.

If a hiring manager arrives via the (future) /platform page and clicks a
"Planned" project, they get a 404. This is worse than a placeholder — it
signals a broken site, not an honest in-progress project.

### Proposed Phase 1.5 fix

Add three `Planned`-status entries to `init_projects_index()` in
`src/data/projects.rs` with:
- slug, title, subtitle, description (scope statement only)
- category and tech_stack appropriate to each
- status: `SystemStatus::Operational` (closest to "Planned" in current enum — the enum refactor to `Shipped`/`InArchitecture`/`Planned` is Phase 2)

Create stub JSON files:
- `public/projects/endpoint-management-compliance.json` → `{"slug":"endpoint-management-compliance","content":""}`
- `public/projects/backup-recovery-continuity.json` → same pattern
- `public/projects/operational-foundation.json` → same pattern

The detail page will:
1. Find the project in the registry → render header, title, tech pills
2. Fetch the JSON → get empty content
3. Render V1 fallback with empty content → blank body area

This is not ideal (blank body), but it is strictly better than a 404.
The status-gated rendering (suppressing sections for non-Shipped projects)
is a Phase 2 template refactor.

### Additional home-page gap

`src/pages/home.rs:139` hardcodes `"4 Projects · 4 Disciplines"`. The
registry now has 3 projects (will be 6 after Phase 1.5). This string must
be computed from the registry. Classified as Phase 1.5 fix — trivial
(replace string literal with formatted count).

---

## 3. Security-Baseline-Audit Dissonance Assessment

### Overall dissonance level: NOTICEABLY OFF

Not actively misleading — the case study content is real and honest. But
the framing mismatch between registry metadata and JSON content is visible
to anyone who reads both the header and the body.

### Specific dissonance points

| Element | Source | What it says | What it should say | Severity |
|---|---|---|---|---|
| **Tech pills** | Registry `tech_stack` | Bash, Linux, Cron, RBAC, Systems Hardening, GPO for Linux, Sysctl | Terraform, GCP, tfsec, Checkov, CIS Benchmark, Workload Identity | **High** — first thing a reader sees under the title; immediately contradicts the case study |
| **Title** | Registry `title` | "Security Baseline & Continuous Audit Toolkit" | Aligned — this is the correct canonical name | Fine |
| **Subtitle** | Registry `subtitle` | "CIS-Aligned Hardening, Terraform Compliance Gates & Continuous Drift Detection" | Aligned — updated in Phase 1 fold | Fine |
| **Problem section** | JSON `problem` | "In a homelab GCP environment, onboarding a new environment..." | Should read as "enforcing continuous compliance" not "provisioning environments" | **Moderate** — the operational pain is real but framed as provisioning, not audit |
| **Approach section** | JSON `approach` | "Modular Terraform registry...producing a Private-First VPC..." | Reframe emphasis from "what the VPC looks like" to "how compliance is continuously enforced" | **Moderate** |
| **Decisions** | JSON `decisions` | 4 ADRs about state backend, WIF, Private-First VPC, module registry | Real decisions with real tradeoffs. Usable as-is. The decisions ARE security-baseline decisions — GCS state locking is audit-trail integrity, WIF is credential hygiene, Private-First is attack surface reduction | **Low** — content is sound, just needs an intro sentence framing them as audit decisions |
| **Outcomes** | JSON `outcomes` | 4h→5min provisioning, drift detection ≤24h, 87/92 CIS, zero plaintext creds | Directly relevant to a continuous audit narrative. Metrics with methodology — strongest content on the site | **Fine as-is** — these are audit outcomes |
| **Lessons** | JSON `lessons` | Module blast radius, tfsec vs Checkov disagreement, IAP setup cost | Real operational lessons. Reusable without reframing | **Fine as-is** |
| **Hero metrics** | JSON `hero_metrics` | 4h→5min, 87/92, 0 public IPs | Directly relevant. 87/92 CIS is the money metric for a security baseline project | **Fine as-is** |
| **Diagram** | SVG | GCP architecture diagram | The diagram is factual infrastructure — it doesn't claim a frame. Usable as-is | **Fine as-is** |
| **Demo** | JSON demo | Drift-detect → reconciliation cycle | This IS a continuous audit demo — injecting drift and watching the pipeline catch and revert it. Perfect for Project 3 | **Fine as-is** |
| **Category** | Registry `category` | `ProjectCategory::SystemsAdmin` | Should arguably be a security/compliance category, but the current enum doesn't have one. Leave for Phase 2 enum refactor | **Low** |

### Reusable without reframing (strong content)

- Hero metrics (87/92 CIS, 0 public IPs, 4h→5min)
- All 4 outcome metrics with methodology
- All 3 lessons (honest, specific, operational)
- All 4 ADR decisions (real tradeoffs, rejected alternatives named)
- The drift-detection demo (this is literally a continuous audit demo)
- All code highlights (backend.tf, vpc-network, workload-identity, CI gate)
- The entire documentation tab (threat model, runbooks, incident playbooks)

### Needs voice reframing

- **Tech pills**: swap to Terraform/GCP/tfsec/CIS (registry field update, trivial)
- **Problem section**: reframe from "provisioning was slow" to "compliance posture was unaudited and drift was invisible" — the facts are the same, the emphasis shifts
- **Approach section**: reframe from "modular Terraform registry" to "continuous compliance enforcement via IaC" — again, same facts, different lens

### Estimated scope: MODERATE

The content does not need rewriting — it needs retoning. The facts,
metrics, decisions, and demos are all genuinely strong and stand on their
own. The reframe is about changing the opening sentences of Problem and
Approach (2 paragraphs) and updating the tech pills (1 array). Everything
else is usable as-is.

**Recommendation:** Keep the reframe in Phase 2 (planned), not Phase 1.5
(urgent). Reason: the site is not live on the canonical URL yet (still on
`revamp` branch). The dissonance does not face the public until a deploy
to `main`. The tech-pill fix is trivial and could be done as a Phase 1.5
quick-fix alongside the planned-project registry additions without
touching the case-study prose.

---

## Summary of gaps

| Gap | Severity | Proposed resolution | Phase |
|---|---|---|---|
| 3 planned projects 404 instead of placeholder | **Blocking** | Add to registry + stub JSON | Phase 1.5 |
| Home page says "4 Projects · 4 Disciplines" | **Visible** | Compute from registry | Phase 1.5 |
| Tech pills on security-baseline-audit mismatch body | **Visible** | Update `tech_stack` array in registry | Phase 1.5 |
| /platform route does not exist | **Expected** | New page, Phase 2 per diff plan | Phase 2 |
| Problem/Approach voice in security-baseline-audit JSON | **Moderate** | Retone 2 paragraphs | Phase 2 |
| Identity strings across 28 locations | **Deferred** | PROFESSIONAL_TITLE centralization | Phase 2 |
