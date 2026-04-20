# 01 — Current State Map

Produced as Step 1 of the staff-lens portfolio restructure. All claims
are file+line grounded from the `revamp` branch at `7a78f83`.

---

## A. Information Architecture

### Route map (src/lib.rs:177-189)

```
/                          → HomePage        (src/pages/home.rs:48)
/about                     → AboutPage       (src/pages/about.rs:6)
/writing                   → WritingPage     (src/pages/writing.rs:10)
/writing/:slug             → WriteupDetailPage (src/pages/writing.rs)
/project/:slug             → ProjectDetailPage (src/pages/project/detail.rs)
/project/:slug/docs        → ProjectDocsPage   (src/pages/project/docs.rs)
/project/:slug/demo        → ProjectDemoPage   (src/pages/project/demo.rs)
/resume                    → ResumePage      (src/pages/resume.rs:7)
/contact                   → ContactPage     (src/pages/contact.rs:9)
/telemetry                 → TelemetryPage   (src/pages/telemetry.rs)
/one-pager                 → OnePageSummary  (src/pages/one_pager.rs:8)
/*any                      → NotFoundPage    (src/pages/not_found.rs:6)
```

12 routes total. No `/platform` route exists.

### Content-source mapping

| Page | Rust consts (compiled in) | JSON (runtime fetch) |
|---|---|---|
| HomePage | `get_infrastructure_fleet()`, `get_certifications()`, `PROFESSIONAL_TITLE`, `GITHUB_URL` | — |
| ProjectDetailPage | `find_project(slug)` for index | `/projects/{slug}.json` → `ProjectDetail` |
| ProjectDocsPage | `find_project(slug)` for index | `/docs/{slug}.json` → `ProjectDetail` |
| ProjectDemoPage | `find_project(slug)` for index | `/demos/{slug}.json` (only terraform-gcp populated) |
| WritingPage | `WRITEUPS` LazyLock (17 entries) | — |
| WriteupDetailPage | `WRITEUPS` for index | `/writeups/{slug}.json` → `WriteUpDetail` |
| ResumePage | `EMAIL`, `GITHUB_URL`, `LINKEDIN_URL`, `PROFESSIONAL_TITLE` | — |
| AboutPage | `PROFESSIONAL_TITLE` | — |
| ContactPage | `EMAIL`, `GITHUB_URL`, `LINKEDIN_URL`, `PROFESSIONAL_TITLE` | — |
| OnePageSummary | `get_infrastructure_fleet()`, `EMAIL`, `PROFESSIONAL_TITLE` | — |
| TelemetryPage | Hardcoded slug list for network probes | — |

### Three-tab project template

The project detail system uses three route suffixes per slug:
- `/project/:slug` → Case Study (detail.rs)
- `/project/:slug/docs` → Documentation (docs.rs)
- `/project/:slug/demo` → Demo (demo.rs)

All three use `find_project(slug)` for the index entry and fetch
per-project JSON at runtime for long-form content. The template
already supports a V2 structured format (`ProjectDetail.is_structured()`)
with fields for problem, constraints, approach, decisions, outcomes,
and lessons (src/data/projects.rs:87-124).

---

## B. Positioning Drift Inventory

Target identity (per resolved CLAUDE.md): **"Linux Systems Administrator"**

Every location below deviates from that target.

### Central constant

| File | Line | Current value | Required change |
|---|---|---|---|
| `src/data/mod.rs` | 20 | `"Systems Administrator & DevOps Engineer"` | → `"Linux Systems Administrator"` |

### index.html (9 locations)

| Line | Element | Current string |
|---|---|---|
| 18 | `<title>` | `"Richard J. Mussell \| Systems Administrator & DevOps Engineer"` |
| 19 | `<meta name="description">` | `"...Systems Administrator & DevOps Engineer specializing in..."` |
| 20 | `<meta property="og:title">` | `"...Systems Administrator & DevOps Engineer"` |
| 21 | `<meta property="og:description">` | `"Systems Administrator & DevOps Engineer..."` |
| 28 | `<meta name="twitter:title">` | `"...Systems Administrator & DevOps Engineer"` |
| 29 | `<meta name="twitter:description">` | `"Systems Administrator & DevOps Engineer..."` |
| 97 | JSON-LD `jobTitle` | `"Systems Administrator & DevOps Engineer"` |
| 98 | JSON-LD `description` | `"...Systems Administrator & DevOps Engineer..."` |
| 153 | `<noscript>` | `"Systems Administrator & DevOps Engineer"` |

### src/pages/ (14 locations)

| File | Line | Element | Current string |
|---|---|---|---|
| `home.rs` | 87 | Meta description | `"Systems Administrator & DevOps Engineer..."` |
| `home.rs` | 96 | Hero subtitle | `"Systems Administrator & DevOps Engineer"` |
| `home.rs` | 98 | Hero body | `"SOC-trained infrastructure engineer..."` |
| `home.rs` | 106 | Hero meta | `"Systems Engineering & Lab Projects"` |
| `about.rs` | 9 | Meta description | `"...Systems Administrator & DevOps Engineer..."` |
| `about.rs` | 24 | Body text | `"Systems Administrator and DevOps Engineer..."` |
| `resume.rs` | 15 | Meta description | `"...Systems Administrator & DevOps Engineer..."` |
| `resume.rs` | 25 | Resume header | `"Systems Administrator & DevOps Engineer"` |
| `resume.rs` | 57 | Summary paragraph | `"Systems Administrator and DevOps Engineer..."` |
| `contact.rs` | 15 | Meta description | `"...Systems Administrator & DevOps Engineer..."` |
| `contact.rs` | 22 | Subtext | `"Systems Administrator and DevOps Engineer..."` |
| `one_pager.rs` | 20 | Title line | `"Systems Administrator & DevOps Engineer"` |
| `not_found.rs` | 11 | Meta description | `"...Systems Administrator & DevOps Engineer..."` |
| `writing.rs` | 63 | Title tag | `"...Systems Engineering"` |

### src/components/ (1 location)

| File | Line | Current string |
|---|---|---|
| `site_footer.rs` | 21 | `"Systems Administrator · DevOps · Platform Engineer · Oklahoma City"` |

### Other files (3 locations)

| File | Line | Current string |
|---|---|---|
| `README.md` | 3 | `"**Systems Administrator & DevOps Engineer**..."` |
| `README.md` | 175 | `"**Richard J. Mussell** — Systems Administrator & DevOps Engineer..."` |
| `manifest.json` | 4 | `"Systems Administrator & Platform Operations"` |

### Import chain file (1 location — outside repo)

| File | Line | Current string |
|---|---|---|
| `~/.claude/projects/portfolio/CONTEXT.md` | 6 | `"Systems Administrator & DevOps Engineer"` |

**Total: 28 locations** requiring string update.

Of these, only `src/data/mod.rs:20` (`PROFESSIONAL_TITLE`) is read
programmatically. The other 27 are hardcoded strings in HTML, Rust
view macros, Markdown, JSON, and the Claude import chain. The
restructure must centralize them: pages that render
`<Title>` and `<Meta>` should interpolate `PROFESSIONAL_TITLE`.

### Banned-word violations (10 instances of "engineered")

| File | Line | Context |
|---|---|---|
| `src/data/projects.rs` | 436 | Description: `"Engineered a data-driven..."` |
| `src/data/projects.rs` | 483 | One-liner: `"Engineered an idempotent framework..."` |
| `src/data/projects.rs` | 484 | One-liner: `"Engineered an idempotent framework..."` |
| `src/data/projects.rs` | 485 | One-liner: `"Engineered an idempotent framework..."` |
| `src/data/projects.rs` | 486 | One-liner: `"Engineered an idempotent framework..."` |
| `src/data/projects.rs` | 487 | One-liner fallback: `"Engineered an idempotent..."` |
| `src/pages/resume.rs` | 189 | Project summary: `"Engineered a secure..."` |
| `src/pages/resume.rs` | 207 | Project summary: `"Engineered an identity-based..."` |
| `src/pages/one_pager.rs` | 52 | Featured project: `"Engineered a secure..."` |
| `src/pages/one_pager.rs` | 70 | Featured project: `"Engineered a ZTNA..."` |

---

## C. Scalability-Contract Audit

Testing each of the six contract requirements against the current code.

### Requirement 1: Single canonical project registry

**Status: PARTIAL.** `src/data/projects.rs` has `PROJECTS: LazyLock<Vec<ProjectIndex>>`
with `get_infrastructure_fleet()` accessor. This is the registry. But it is
missing fields required by the contract:

| Required field | Present? | Notes |
|---|---|---|
| slug | Yes | `&'static str` |
| title | Yes | `&'static str` |
| tagline | Partial | `subtitle` field serves this role |
| status | **Wrong values** | `SystemStatus::Operational/Degraded/Maintenance` — not `shipped/in-architecture/planned` |
| tags | **No** | `tech_stack` is close but is a tech list, not a tag taxonomy |
| produces | **No** | No connection fields exist |
| consumes | **No** | No connection fields exist |
| summary | Partial | `description` field serves this role but is verbose |
| one-liner | **Separate** | `one_liner_for_project()` (line 481) is a match statement, not a field |

### Requirement 2: Every surface reads from the registry

**Status: VIOLATED.** These surfaces hardcode project data instead of
reading from the registry:

| Surface | File:Line | What's hardcoded |
|---|---|---|
| Resume project section | `resume.rs:185-209` | 4 projects with title, subtitle, summary, and tech stack — all duplicated from registry |
| One-pager featured projects | `one_pager.rs:48-72` | 4 projects with slug, display_title, summary, and tags — duplicated |
| Home page count string | `home.rs:139` | `"4 Projects · 4 Disciplines"` — should be computed |
| One-liner function | `projects.rs:481-489` | Per-project match with hardcoded strings instead of a field |
| Telemetry network probes | `telemetry.rs:130-135` | 4 hardcoded slugs |
| Sitemap | `public/sitemap.xml:14-21` | 4 hardcoded `<url>` entries |
| SSG route list | `src/bin/ssg.rs:33-37` | **Reads from registry** (correct) |
| Command palette | `src/components/palette.rs:43-50` | **Reads from registry** (correct) |
| Home page cards | `home.rs:49` | **Reads from registry** (correct) |
| SQLite search fallback | `db.rs:39-95` | **Reads from registry** (correct) |

### Requirement 3: Per-project content lives in per-project files

**Status: MOSTLY MET.** Content JSON files exist per-project under
`public/projects/`, `public/docs/`, `public/demos/`. But:
- No `docs/projects/<slug>/` directory structure exists for design
  docs, ADRs, threat models, or runbooks
- Writeup JSON files in `public/writeups/` are ALL 1-line stubs
  (17 files, each 1 line)

### Requirement 4: Connections declared as data

**Status: NOT MET.** No `produces`/`consumes` fields on `ProjectIndex`.
No connection mechanism exists anywhere in the codebase. Cross-references
appear only in prose (project descriptions and case study bodies).

### Requirement 5: Status is a first-class field

**Status: WRONG VALUES.** `SystemStatus` enum (projects.rs:7-11) has:
`Operational`, `Degraded`, `Maintenance`. All 4 projects are set to
`Operational`. The contract requires: `shipped`, `in-architecture`,
`planned` — and template rendering gated on these values.

### Requirement 6: Adding a project is a three-file change

**Status: VIOLATED.** Adding a seventh project currently requires
editing **8 files**:

1. `src/data/projects.rs` — append to `init_projects_index()` vec
2. `src/data/projects.rs` — add case to `one_liner_for_project()` match
3. `src/pages/resume.rs:185-209` — add to hardcoded project vec
4. `src/pages/one_pager.rs:48-72` — add to hardcoded project array
5. `src/pages/telemetry.rs:130-135` — add slug to network probe vec
6. `public/sitemap.xml` — add `<url>` entry
7. `public/projects/<slug>.json` — create content file
8. Optionally: `public/docs/<slug>.json`, `public/demos/<slug>.json`

The contract requires 3 files maximum. The delta is 5 extra files
caused by hardcoded lists in resume.rs, one_pager.rs, telemetry.rs,
projects.rs (one-liner match), and sitemap.xml.

---

## D. Content Inventory

### D1. Four shipped projects mapped to six canonical

| Canonical name (resolved) | Shipped slug | Status | Notes |
|---|---|---|---|
| Identity & Access Lifecycle Platform | `zero-trust-networking` | Partial overlap | Current project covers WireGuard/AD zero-trust but not full identity lifecycle (provisioning, MFA, lifecycle automation). Needs scope expansion or rename. |
| Endpoint Management & Compliance System | `linux-admin-scripting` | Partial overlap | Current project covers POSIX bash scripting and hardening but not endpoint management (Intune, Autopilot, GPO fleet). Needs scope expansion or rename. |
| Security Baseline & Continuous Audit Toolkit | — | **Not built** | No shipped equivalent. No slug, no content, no code. `planned` status in registry. |
| Backup, Recovery & Business Continuity System | — | **Not built** | No shipped equivalent. `planned` status in registry. |
| Observability & Operational Intelligence Platform | `monitoring-observability` | Good overlap | Current project covers Prometheus/Grafana/ELK pipeline. Rename and expand to match canonical. |
| Operational Foundation (Wiki/Runbooks/Change Mgmt/IR) | — | **Not built** | No shipped equivalent. `planned` status in registry. But this one is unusual — it could reference the portfolio site itself + the SOC homelab as operational artifacts. |

Current project `terraform-gcp` does not map cleanly to any of the
six canonical names. It could live as a seventh project or be folded
into the Security Baseline or Operational Foundation scope.

**Decision point:** The owner must confirm the mapping. Three of six
canonical projects have no shipped code. Per the content rules, they
can exist in the registry as `planned` status but must NOT have case
study content or demos.

### D2. Writeups (17 entries in src/data/writeups.rs)

**Technical writeups (8) — date 2026, is_core: false:**

| Slug | Title | Category | Voice alignment |
|---|---|---|---|
| hardening-linux-municipal-environments | Hardening Linux for Municipal Environments | CYBERSECURITY | **Aligned** — direct, operational |
| automating-nist-800-53-compliance-with-terraform | Automating NIST 800-53 Compliance with Terraform | CYBERSECURITY | **Aligned** — direct, operational |
| zero-trust-moving-beyond-bastion-hosts | Zero-Trust: Moving Beyond Bastion Hosts | CYBERSECURITY | **Aligned** — direct, operational |
| siem-alert-hygiene-reducing-noise-in-the-soc | SIEM Alert Hygiene: Reducing Noise in the SOC | OBSERVABILITY | **Aligned** — direct, operational |
| kubernetes-controller-reconciliation-deep-dive | Why Your Kubernetes Controller Is Lying to You | PLATFORM | **Marginal** — advanced topic, may not align with sysadmin framing |
| otel-ebpf-tracing-without-instrumentation | Distributed Tracing Without Touching Your App Code | OBSERVABILITY | **Marginal** — eBPF is adjacent, not core sysadmin |
| rust-wasm-edge-runtime-internals | Building a Zero-Copy Wasm Edge Runtime in Rust | PLATFORM | **Flag** — Rust systems programming, not sysadmin |
| ebpf-from-zero-to-prod | eBPF From Zero to Production | OBSERVABILITY | **Marginal** — valuable but niche |

**Essay-style writeups (9) — date 2024, is_core: true:**

| Slug | Title | Category | Voice alignment |
|---|---|---|---|
| the-architect-of-the-prismatic-apex-... | The Architect of the Prismatic Apex: Orchestrating Equilibrium in a Holographic Landscape | STRATEGY | **HIGH RISK** — metaphysical, LLM-essay voice |
| the-orchestrated-landscape-... | The Orchestrated Landscape: Building a High-Integrity Ecosystem | PLATFORM | **HIGH RISK** — abstract, marketing-adjacent |
| the-sustainable-architect-... | The Sustainable Architect: Engineering Low-Entropy Landscapes for the 50-Year Lookout | STRATEGY | **HIGH RISK** — abstract, uses banned "Engineering" in title |
| the-builders-ledger-... | The Builder's Ledger: Orchestrating Technical Outcomes through Project Governance | STRATEGY | **MODERATE RISK** — governance angle could align if retitled |
| the-orchestrator-of-intent-... | The Orchestrator of Intent: Reflections on Service Provisioning | PLATFORM | **MODERATE RISK** — telecom experience, could align if retitled |
| the-architect-of-oceanic-visibility-... | The Architect of High-Fidelity Observability: SOC Operations at Universal Scale | OBSERVABILITY | **MODERATE RISK** — SOC-relevant, title is overwrought |
| the-connectivity-fabric-... | The Connectivity Fabric: Mastering the Bedrock of the Universal Control Plane | NETWORKING | **MODERATE RISK** — networking fundamentals, title is overwrought |
| the-mirror-universe-... | The Mirror Universe: Architecting Deterministic Enterprise Simulations | PLATFORM | **MODERATE RISK** — AD/PowerShell content, title is misleading |
| universal-dialects-... | Universal Dialects: The Role of Linux and Shell in the Unified Control Plane | PLATFORM | **MODERATE RISK** — Linux/Shell content, title is overwrought |

**Critical finding:** All 17 writeup JSON files in `public/writeups/`
are **1-line stubs**. None contain actual article content. The writing
page currently renders empty article bodies for every writeup.

### D3. PDFs (6 files in public/pdfs/)

| Filename | Size | Alignment |
|---|---|---|
| resume.pdf | 118 KB | **Keep** — primary deliverable |
| builders-ledger.pdf | 81 KB | **Flag** — paired with essay-style writeup |
| orchestrated-landscape.pdf | 68 KB | **Flag** — paired with essay-style writeup |
| sustainable-architect.pdf | 81 KB | **Flag** — paired with essay-style writeup |
| universal-dialects.pdf | 84 KB | **Flag** — paired with essay-style writeup |
| platform-architecture-blueprint.pdf | 15 KB | **Flag** — generic title, small size |

### D4. Resume variants (3 files in docs/resumes/)

| File | Target tier | Target comp | Generated |
|---|---|---|---|
| resume-sysadmin.md | Junior-to-Mid IT Admin / Sys Admin | $70K-$95K | 2026-04-15 |
| resume-devops.md | Junior-to-Mid DevOps / Cloud Engineer / SRE | $80K-$110K | 2026-04-15 |
| resume-platform.md | Junior-to-Mid Platform Eng / Infra Eng | $90K-$120K | 2026-04-15 |

**Proposal:** With the identity resolved to "Linux Systems Administrator,"
the sysadmin variant becomes the canonical public resume. The devops and
platform variants are kept in `docs/resumes/` as internal application-
time tailoring aids but are not linked from the site. The live
`public/pdfs/resume.pdf` should be regenerated from the sysadmin variant
with the target level updated from "Junior-to-Mid" to "Senior."

### D5. Demo and doc JSON files

| Directory | Files | Populated | Stubs |
|---|---|---|---|
| `public/demos/` | 1 | terraform-gcp.json (110 lines, full narrative) | — |
| `public/docs/` | 4 | terraform-gcp.json (245 lines, comprehensive) | linux-admin-scripting.json (6 lines), monitoring-observability.json (6 lines), zero-trust-networking.json (6 lines) |

Only terraform-gcp has substantive demo and documentation content.
The other three have minimal stubs.

---

## E. Build/CI Constraints

These constraints are hard — the restructure must not break any.

| Constraint | Gate | Notes |
|---|---|---|
| CSR default build | `cargo check --target wasm32-unknown-unknown` | Must pass after any Rust change |
| SSR host-only | `cargo check --no-default-features --features ssr` | Must pass |
| Hydrate+sqlite wasm32 | `cargo check --no-default-features --features "hydrate sqlite" --target wasm32-unknown-unknown` | Must pass |
| SSG binary compile | `cargo check --features ssg --bin ssg` | Must pass |
| MSRV | 1.82 | `Cargo.toml` rust-version field |
| Trunk release build | `trunk build --release` | Must pass before any deploy |
| Lighthouse | ≥ 90 | Informational — no CI gate currently |
| Dependencies | No new crates without bundle-size justification | WASM binary is 1.6 MB raw |
| Commit trailers | No `Co-Authored-By: Claude` | Per working agreements |
| Clippy | `cargo clippy -- -D warnings` | CI gate on `--target wasm32-unknown-unknown` |
| Format | `cargo fmt --check` | CI gate |

---

## F. Existing Structure to Preserve

### Three-tab project template

The `detail.rs` / `docs.rs` / `demo.rs` split under `src/pages/project/`
is the correct information architecture for the restructure. The V2
`ProjectDetail` struct (projects.rs:87-124) already has structured
fields for:

- `problem` — maps to the Case Study tab "Context" section
- `constraints_in` / `constraints_out` — maps to "Scope Boundaries"
- `approach` + `approach_diagram_src` — maps to "Architecture at a Glance"
- `decisions` — maps to "Decisions" section (Vec<Decision>)
- `outcomes` — maps to "Measurement" section (Vec<Outcome>)
- `lessons` — maps to "What I'd do differently"
- `artifact_links` — maps to repo/doc links
- `highlights` — maps to code snippets

This V2 schema is 80% of what the restructured Case Study tab needs.
Missing: failure-mode table, explicit connections section,
enterprise-comparison section.

### Content separation

Rust consts in `src/data/` hold index metadata (compiled into WASM).
JSON in `public/` holds long-form content (fetched at runtime). This
split is architecturally correct — index is fast (no network), detail
is lazy (loaded on navigation). Preserve this.

### PROFESSIONAL_TITLE constant

`src/data/mod.rs:20` defines `PROFESSIONAL_TITLE`. Five pages already
import it for `<Title>` tags (home, about, resume, contact, one_pager).
But all five also hardcode the full string in `<Meta name="description">`
tags instead of interpolating the constant. The restructure should
complete the centralization.

### GlobalAppState consolidation

`src/state.rs` provides `GlobalAppState` as a single `provide_context`.
This is stable and correct. The restructure adds no new global state.

### CSS layered architecture

`style/style.scss` → tokens → base → components → pages. Adding the
`/platform` page means adding `style/pages/platform.css` following
the existing convention. No structural CSS changes needed.
