# 02 — Target Structure

Proposed information architecture for the restructured portfolio.
Depends on the resolved identity (Linux Systems Administrator, senior)
and the six-project canonical frame from the owner.

---

## A. Route Map

```
ROUTE                              VIEW              STATUS
──────────────────────────────────────────────────────────────
/                                  HomePage           MODIFY — registry-driven cards, computed count
/platform                          PlatformPage       NEW — connection graph, project groups
/about                             AboutPage          MODIFY — identity + voice rewrite
/writing                           WritingPage        MODIFY — voice filter, audit results
/writing/:slug                     WriteupDetailPage  KEEP
/project/:slug                     ProjectDetailPage  MODIFY — status-gated rendering
/project/:slug/docs                ProjectDocsPage    MODIFY — status-gated rendering
/project/:slug/demo                ProjectDemoPage    MODIFY — status-gated rendering
/resume                            ResumePage         MODIFY — identity + registry-driven projects
/contact                           ContactPage        MODIFY — identity rewrite
/telemetry                         TelemetryPage      MODIFY — registry-driven probe slugs
/one-pager                         OnePageSummary     MODIFY — identity + registry-driven projects
/*any                              NotFoundPage       MODIFY — identity in meta
```

New routes: 1 (`/platform`).
Deleted routes: 0.
Modified routes: 11 (identity alignment + registry centralization).
Unchanged routes: 1 (`/writing/:slug`).

---

## B. Project Registry Design

### ProjectIndex struct (src/data/projects.rs)

```rust
#[derive(Clone, PartialEq)]
pub struct ProjectIndex {
    pub slug: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,           // was: subtitle
    pub summary: &'static str,           // was: description (shortened)
    pub one_liner: &'static str,         // NEW — absorbs one_liner_for_project()
    pub category: ProjectCategory,
    pub domain: ProjectDomain,           // NEW — grouping for /platform page
    pub status: ProjectStatus,           // RENAMED + NEW VALUES
    pub tech_stack: &'static [&'static str],
    pub produces: &'static [&'static str],  // NEW — slug refs
    pub consumes: &'static [&'static str],  // NEW — slug refs
}
```

### ProjectStatus enum (replaces SystemStatus)

```rust
#[derive(Clone, PartialEq)]
pub enum ProjectStatus {
    Shipped,          // all three tabs populated, repo exists
    InArchitecture,   // design docs + ADRs exist, no demo
    Planned,          // title + scope only
}
```

### ProjectDomain enum (NEW — grouping for /platform page)

```rust
#[derive(Clone, PartialEq)]
pub enum ProjectDomain {
    Identity,         // IAM, AD, zero-trust, lifecycle
    Endpoints,        // compliance, hardening, patch management
    Security,         // baselines, audit, CIS, NIST
    Resilience,       // backup, DR, business continuity
    Observability,    // monitoring, SIEM, alerting, SLOs
    Operations,       // wiki, runbooks, change mgmt, IR
}
```

### Initial registry entries (six canonical + terraform-gcp)

| Slug | Title | Domain | Status | Mapped from |
|---|---|---|---|---|
| `identity-access-lifecycle` | Identity & Access Lifecycle Platform | Identity | `InArchitecture` | zero-trust-networking (expanded) |
| `endpoint-compliance` | Endpoint Management & Compliance System | Endpoints | `InArchitecture` | linux-admin-scripting (expanded) |
| `security-baseline` | Security Baseline & Continuous Audit Toolkit | Security | `Planned` | — (new) |
| `backup-recovery` | Backup, Recovery & Business Continuity System | Resilience | `Planned` | — (new) |
| `observability-platform` | Observability & Operational Intelligence Platform | Observability | `InArchitecture` | monitoring-observability (expanded) |
| `operational-foundation` | Operational Foundation | Operations | `Planned` | — (new) |
| `terraform-landing-zone` | Hardened Cloud Landing Zone (IaC) | Security | `Shipped` | terraform-gcp |

**Note on terraform-gcp:** This is the only project with substantive
demo, documentation, and case-study content. It maps most naturally
to the Security domain (CIS-aligned, NIST 800-53 controls). It stays
as the seventh project rather than being forced into the six canonical.
The registry is designed to scale — seven projects is fine.

**Decision point for owner:** The old slugs (linux-admin-scripting,
monitoring-observability, zero-trust-networking, terraform-gcp)
currently have routes at `/project/<old-slug>`. Renaming slugs breaks
existing bookmarks and search-engine indexes. Options:
- (a) Keep old slugs, update titles/metadata in registry
- (b) New slugs with 301 redirects from old routes
- (c) New slugs, accept the SEO reset (low traffic site)

### JSON content files reference the registry

Each `public/projects/<slug>.json` file contains a `"slug"` field
matching the registry entry. The page template looks up the registry
entry by slug to get metadata (title, status, connections), then
fetches the JSON for long-form content. This means:
- Registry owns: title, tagline, status, tags, connections
- JSON owns: prose content, diagrams, code highlights
- They sync on slug. If a JSON file exists for a slug not in the
  registry, it's orphaned. If a registry entry has no JSON, the
  template renders the status-appropriate fallback.

---

## C. Project-Page Template Taxonomy

### Case Study tab — the argument (5-min scan for a hiring manager)

| Section | Purpose | Content source | Status-gated? |
|---|---|---|---|
| Context | One paragraph: what operational pain drove this? | JSON `problem` field | Shipped + InArchitecture |
| Architecture at a Glance | Diagram + 3-5 sentences | JSON `approach` + `approach_diagram_src` | Shipped + InArchitecture |
| Decisions | 5-8 ADR summaries with rejected alternatives named | JSON `decisions` array | Shipped + InArchitecture |
| Failure Modes & Operational Reliability | Table of failure scenarios + mitigations | **NEW** JSON `failure_modes` array | Shipped only |
| Measurement | Numbers with methodology attached | JSON `outcomes` array | Shipped only |
| Scope Boundaries | What was deliberately not built | JSON `constraints_out` array | Shipped + InArchitecture |
| Connections | Auto-rendered from registry `produces`/`consumes` | Rust registry (computed) | All statuses |
| What I'd Do Differently | Honest limitations — non-negotiable | JSON `lessons` array | Shipped only |
| Before/After | Closing comparison | **NEW** JSON `before_after` field | Shipped only |

For `Planned` status: only title, tagline, status badge, and
Connections render. All other sections are suppressed.

For `InArchitecture` status: Context, Architecture, Decisions,
Scope Boundaries, and Connections render. Measurement, Failure Modes,
What I'd Do Differently, and Before/After are suppressed with a
"This project is in architecture — these sections will appear when
it ships" note.

### Documentation tab — the specification (exhaustive)

| Section | Purpose | Content source | Status-gated? |
|---|---|---|---|
| Design Document | Full design narrative | `docs/projects/<slug>/design.md` → rendered via JSON | Shipped + InArchitecture |
| ADR Set | Individual ADRs, Nygard template | `docs/projects/<slug>/decisions/` | Shipped + InArchitecture |
| Threat Model | STRIDE per trust boundary | `docs/projects/<slug>/threat-model.md` | Shipped only |
| Runbook Index | Operational procedures | `docs/projects/<slug>/runbooks/` | Shipped only |
| Measurement Plan | Methodology for claimed metrics | `docs/projects/<slug>/measurement.md` | Shipped only |
| Enterprise Comparison | Named tools, named features, honest delta | `docs/projects/<slug>/comparison.md` | Shipped + InArchitecture |

For `Planned` status: Documentation tab renders a single line:
"Design documentation will be published when this project enters
architecture."

### Demo tab — the proof (reproducible)

| Section | Purpose | Content source | Status-gated? |
|---|---|---|---|
| Operational Walkthrough | Real commands, real output | JSON `demo_steps` field | Shipped only |
| Screenshots | From actual runs, not mockups | JSON `demo_screenshots` array | Shipped only |
| Repository Link | Link to the actual project repo | JSON `repo_url` field | Shipped only |

For `InArchitecture` and `Planned` status: Demo tab renders a single
line: "Demo will be available when this project ships."

---

## D. The /platform Page

### Purpose

The single scan-target page that shows the whole portfolio as a
connected system. A hiring manager who opens one page should open
this one. It answers: "How does this person think about running
infrastructure?"

### Structure

**Section 1 — Thesis (one sentence):**
> Seven projects. One operational platform. This is how I think about
> running infrastructure.

(The count is computed from the registry. At 16 projects, it says
"Sixteen projects.")

**Section 2 — The graph:**
An SVG (or canvas-rendered) dependency graph generated from the
registry's `produces`/`consumes` arrays. Each project is a node
labeled with title and status badge. Edges are labeled with what
flows between them (e.g., "IAM policies", "hardened baselines",
"metrics pipeline").

The graph is rendered client-side from data — NOT a hand-drawn SVG.
When a project is added to the registry with `produces`/`consumes`,
it appears in the graph automatically.

**Section 3 — Domain groups:**
Below the graph, projects are grouped by `ProjectDomain`. Each group
shows its projects as cards with status badges. Click-through to
per-project pages.

**Section 4 — Story (sysadmin voice):**
One paragraph per domain group explaining how the projects in that
domain compose. This prose lives in a dedicated constant or small
data structure, not in per-project JSON — it describes the group, not
the individual project.

**Scaling behavior:** At 6-7 projects, the graph is readable as-is.
At 16+ projects, the domain groups provide visual chunking. At 30+,
the graph may need a zoom/filter mechanism — but that is a future
concern, not a restructure deliverable.

### Rejected alternatives for the connection mechanism

| Alternative | Why rejected |
|---|---|
| Hand-drawn SVG diagram | Doesn't scale. Adding a project requires redrawing. Violates the registry contract. |
| Dedicated Connections section per project page only | Loses the one-scan overview. The /platform page IS the overview. |
| Markdown table listing connections | Flat, not spatial. Loses the "this is a connected system" signal. |

The chosen approach (registry arrays + client-side graph rendering)
satisfies the scalability contract: add a project, declare its
connections, the graph updates.

---

## E. Status-Gated Rendering

### Summary table

| Element | Shipped | InArchitecture | Planned |
|---|---|---|---|
| Home page card | Full card + accent color | Card with "IN ARCHITECTURE" badge | Card with "PLANNED" badge, muted |
| Platform page node | Solid node | Dashed outline | Dotted outline, muted |
| Case Study tab | All 9 sections | Context + Architecture + Decisions + Scope + Connections | Title + tagline + Connections only |
| Documentation tab | All 6 sections | Design + ADRs + Comparison | "Coming when architecture begins" |
| Demo tab | All 3 sections | "Demo available when shipped" | "Demo available when shipped" |
| One-pager | Featured with full summary | Listed with status note | Not listed |
| Resume projects | Full entry | Not listed | Not listed |
| Sitemap | Full priority (0.9) | Reduced priority (0.5) | Not listed |
| JSON-LD | Full entry | Not listed | Not listed |

### Template behavior in detail.rs

```
if project.status == Planned {
    render title + tagline + status badge + connections
    return
}
if project.status == InArchitecture {
    render context + architecture + decisions + scope + connections
    render "Shipped sections will appear when this project is complete"
    return
}
// Shipped: render all sections
```

---

## F. Writeups Handling

### Audit criteria

Each writeup is evaluated against three questions:
1. Does the title align with sysadmin voice? (Direct, operational, no metaphysical framing)
2. Does the topic serve the "Linux Systems Administrator" positioning?
3. Does content exist? (All JSON files are currently 1-line stubs)

### Per-writeup verdict

**Keep (4) — technical, aligned with sysadmin positioning:**
| Slug | Verdict | Notes |
|---|---|---|
| hardening-linux-municipal-environments | Keep | Core sysadmin topic, direct title |
| automating-nist-800-53-compliance-with-terraform | Keep | Compliance + IaC, direct title |
| zero-trust-moving-beyond-bastion-hosts | Keep | Networking + security, direct title |
| siem-alert-hygiene-reducing-noise-in-the-soc | Keep | SOC ops, direct title |

**Retitle (5) — usable content topic, title is overwrought:**
| Slug | Current title | Proposed retitle direction | Notes |
|---|---|---|---|
| the-orchestrator-of-intent-... | The Orchestrator of Intent: Reflections on Service Provisioning | → "What Running a Telecom Taught Me About Service Provisioning" | Telecom ops experience, relevant |
| the-architect-of-oceanic-visibility-... | The Architect of High-Fidelity Observability: SOC Operations at Universal Scale | → "Building a SOC Observability Stack That Actually Reduces Noise" | SOC-relevant, needs grounding |
| the-connectivity-fabric-... | The Connectivity Fabric: Mastering the Bedrock... | → "Cisco IOS Fundamentals: What Network Admins Actually Need" | Networking fundamentals, retitle to operational |
| the-mirror-universe-... | The Mirror Universe: Architecting Deterministic Enterprise Simulations | → "Testing AD Group Policies Without Breaking Production" | AD/PowerShell content, retitle to operational |
| universal-dialects-... | Universal Dialects: The Role of Linux and Shell... | → "Linux and PowerShell: The Two Languages Every Sysadmin Needs" | Shell scripting, retitle to practical |

**Demote or retire (5) — topic doesn't serve sysadmin positioning:**
| Slug | Current title | Verdict | Reason |
|---|---|---|---|
| kubernetes-controller-reconciliation-deep-dive | Why Your Kubernetes Controller Is Lying to You | Demote (move below fold) | Advanced K8s, not core sysadmin |
| otel-ebpf-tracing-without-instrumentation | Distributed Tracing Without Touching Your App Code | Demote | eBPF is adjacent, not core |
| rust-wasm-edge-runtime-internals | Building a Zero-Copy Wasm Edge Runtime in Rust | Retire or move to blog | Rust systems dev, not sysadmin |
| ebpf-from-zero-to-prod | eBPF From Zero to Production | Demote | Niche, not core sysadmin |
| the-architect-of-the-prismatic-apex-... | The Architect of the Prismatic Apex: Orchestrating Equilibrium in a Holographic Landscape | **Retire** | Metaphysical essay, voice mismatch |

**Retire (3) — abstract/philosophical, voice mismatch with sysadmin:**
| Slug | Current title | Reason |
|---|---|---|
| the-orchestrated-landscape-... | The Orchestrated Landscape: Building a High-Integrity Ecosystem | Abstract systems philosophy, no operational content |
| the-sustainable-architect-... | The Sustainable Architect: Engineering Low-Entropy Landscapes... | Uses banned word "Engineering" in title, abstract |
| the-builders-ledger-... | The Builder's Ledger: Orchestrating Technical Outcomes... | Project governance framing, not sysadmin |

**Owner decides final fate.** These verdicts are recommendations.
The PDFs paired with retired writeups (builders-ledger.pdf,
orchestrated-landscape.pdf, sustainable-architect.pdf,
universal-dialects.pdf) should follow the same fate as their writeups.

---

## G. PDFs Handling

| File | Paired writeup | Verdict |
|---|---|---|
| resume.pdf | — | **Keep** — regenerate from sysadmin variant at senior level |
| platform-architecture-blueprint.pdf | — | **Keep if content aligns** — review for positioning |
| builders-ledger.pdf | the-builders-ledger-... | Follows writeup verdict (retire) |
| orchestrated-landscape.pdf | the-orchestrated-landscape-... | Follows writeup verdict (retire) |
| sustainable-architect.pdf | the-sustainable-architect-... | Follows writeup verdict (retire) |
| universal-dialects.pdf | universal-dialects-... | Follows writeup verdict (retitle) |

---

## H. Resume Handling

**Proposal:** Single public variant, two internal variants.

| Variant | Location | Visibility |
|---|---|---|
| Sysadmin (canonical) | `public/pdfs/resume.pdf` + `/resume` page | Public — linked from hero, footer, one-pager |
| DevOps | `docs/resumes/resume-devops.md` | Internal — application-time tailoring |
| Platform | `docs/resumes/resume-platform.md` | Internal — application-time tailoring |

The live resume page renders the sysadmin variant's content. Target
level updated from "Junior-to-Mid" to "Senior." The `/resume` route
does not expose variant selection — one resume, one identity.

---

## I. Footer + PROFESSIONAL_TITLE Centralization

### Target footer string

```
© 2026 Richard J. Mussell · Linux Systems Administrator · Oklahoma City
```

### PROFESSIONAL_TITLE refactor

`src/data/mod.rs:20` changes from:
```rust
pub const PROFESSIONAL_TITLE: &str = "Systems Administrator & DevOps Engineer";
```
to:
```rust
pub const PROFESSIONAL_TITLE: &str = "Linux Systems Administrator";
```

### Files that currently hardcode the title (must reference the constant)

| File | Line | Current approach | Required change |
|---|---|---|---|
| `src/components/site_footer.rs` | 21 | Hardcoded `"Systems Administrator · DevOps · Platform Engineer · Oklahoma City"` | Interpolate `PROFESSIONAL_TITLE` + `" · Oklahoma City"` |
| `src/pages/home.rs` | 96 | Hardcoded `"Systems Administrator & DevOps Engineer"` | Use `PROFESSIONAL_TITLE` |
| `src/pages/home.rs` | 87 | Hardcoded in Meta description | Interpolate `PROFESSIONAL_TITLE` |
| `src/pages/resume.rs` | 25 | Hardcoded `"Systems Administrator & DevOps Engineer"` | Use `PROFESSIONAL_TITLE` |
| `src/pages/resume.rs` | 15 | Hardcoded in Meta description | Interpolate `PROFESSIONAL_TITLE` |
| `src/pages/about.rs` | 9 | Hardcoded in Meta description | Interpolate `PROFESSIONAL_TITLE` |
| `src/pages/contact.rs` | 15 | Hardcoded in Meta description | Interpolate `PROFESSIONAL_TITLE` |
| `src/pages/contact.rs` | 22 | Hardcoded in subtext | Interpolate `PROFESSIONAL_TITLE` |
| `src/pages/one_pager.rs` | 20 | Hardcoded `"Systems Administrator & DevOps Engineer"` | Use `PROFESSIONAL_TITLE` |
| `src/pages/not_found.rs` | 11 | Hardcoded in Meta description | Interpolate `PROFESSIONAL_TITLE` |
| `src/pages/writing.rs` | 63 | Hardcoded `"Systems Engineering"` | Use `PROFESSIONAL_TITLE` |
| `index.html` | 18 | Hardcoded in `<title>` | Manual update (not Rust) |
| `index.html` | 19,20,21,28,29 | Hardcoded in OG/Twitter meta | Manual update |
| `index.html` | 97,98 | Hardcoded in JSON-LD | Manual update |
| `index.html` | 153 | Hardcoded in noscript | Manual update |
| `README.md` | 3, 175 | Hardcoded | Manual update |
| `manifest.json` | 4 | Hardcoded | Manual update |

**Total: 11 Rust files can reference the constant.** 6 non-Rust files
(index.html, README.md, manifest.json) require manual string updates
but should match the constant's value.
