# Phase 2 Pre-flight Report

Read-only inspection on `revamp` @ `a571120`, 2026-04-20.

---

## 1. Status Enum Current State

### A. Enum definition

```rust
// src/data/projects.rs:4-11
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
#[allow(dead_code)] // Degraded, Maintenance used for project status in data and UI
pub enum SystemStatus {
    Operational,
    Degraded,
    Maintenance,
}
```

Three variants. The enum is named `SystemStatus`, not `ProjectStatus`.

### B. Has the enum changed?

No. It is identical to its pre-Phase-1 definition. The Phase 1 target
structure (02-target-structure.md) proposed renaming to `ProjectStatus`
with variants `Shipped | InArchitecture | Planned`. That refactor was
never executed — it was deferred to Phase 2.

### C. Per-project status values

| Slug | Registry value (src/data/projects.rs) | Line | Intended status |
|---|---|---|---|
| security-baseline-audit | `SystemStatus::Operational` | 403 | Shipped |
| observability-operational-intelligence | `SystemStatus::Operational` | 420 | InArchitecture (has V1 content, no V2 metrics) |
| identity-access-lifecycle | `SystemStatus::Operational` | 437 | InArchitecture (same) |
| endpoint-management-compliance | `SystemStatus::Operational` | 454 | Planned |
| backup-recovery-continuity | `SystemStatus::Operational` | 463 | Planned |
| operational-foundation | `SystemStatus::Operational` | 472 | Planned |

**All six are `SystemStatus::Operational`.** The Phase 1.5 report's
language ("Shipped", "Operational", "Planned") described intended
status, not actual enum values. Every project in the registry has the
same variant.

### D. How the template renders per status

**The project-detail template (`detail.rs`) does not read `SystemStatus`
at all.** Its render path is:

1. `find_project(slug)` → Some or None
2. If Some: render header (title, subtitle, tech pills, category)
3. Fetch `/projects/{slug}.json` → parse as `ProjectDetail`
4. If `ProjectDetail.is_structured()` (i.e., `problem.is_some()`): V2 render
5. Else: V1 render (`inner_html` of `content` field)

The status badge shown in V2 renders reads `ProjectDetail.status_label`
— a **JSON field**, not the registry's `SystemStatus`. It is set to
`"Homelab"` in the security-baseline-audit JSON. The other projects
have no V2 JSON, so no badge renders.

**The `ProjectCard` component (`src/components/project.rs:65-75`) DOES
read `SystemStatus`:**

```rust
// src/components/project.rs:65-75
{match project.status {
    SystemStatus::Operational => view! {
        <span class="project-card-status-chip" style="color:#10b981;">"● LIVE"</span>
    }.into_view(),
    SystemStatus::Degraded => view! {
        <span class="project-card-status-chip" style="color:#f59e0b;">"◐ PARTIAL"</span>
    }.into_view(),
    SystemStatus::Maintenance => view! {
        <span class="project-card-status-chip" style="color:var(--text-muted);">"○ WIP"</span>
    }.into_view(),
}}
```

**Consequence: all six project cards on the home page show "● LIVE".**
The three planned projects that have no content, no demos, and no
case studies are labeled "LIVE" to any visitor. This is actively
dishonest.

---

## 2. Domain Taxonomy Current State

### A. Per-project category values

| Slug | Category | Line |
|---|---|---|
| security-baseline-audit | `ProjectCategory::SystemsAdmin` | 402 |
| observability-operational-intelligence | `ProjectCategory::Networking` | 419 |
| identity-access-lifecycle | `ProjectCategory::CyberSecurity` | 436 |
| endpoint-management-compliance | `ProjectCategory::SystemsAdmin` | 453 |
| backup-recovery-continuity | `ProjectCategory::SystemsAdmin` | 462 |
| operational-foundation | `ProjectCategory::SystemsAdmin` | 471 |

### B. The "3 Disciplines" on the home page

The home page (home.rs:71-74) counts distinct categories with >0
projects:

| Category | Label | Project count | Projects |
|---|---|---|---|
| CyberSecurity | "Cyber Security" | 1 | identity-access-lifecycle |
| SystemsAdmin | "Systems Admin" | 4 | security-baseline-audit, endpoint-management-compliance, backup-recovery-continuity, operational-foundation |
| Networking | "Network Operations" | 1 | observability-operational-intelligence |
| CloudInfrastructure | "Cloud Infrastructure" | 0 | — (terraform-gcp was folded) |

Disciplines with >0 projects = **3**. The heading reads "6 Projects ·
3 Disciplines."

The **CloudInfrastructure** filter tab still renders on the home page
with count 0. Clicking it hides all project cards. This is a cosmetic
issue — not broken, but confusing.

### C. Is the domain field typed?

Yes. `ProjectCategory` is a Rust enum (projects.rs:13-19). No
free-string drift is possible — the compiler enforces the taxonomy.
However, the taxonomy was designed for the original four projects
(one per category). With four of six now in `SystemsAdmin`, the
grouping provides almost no discriminatory value.

### D. Defense of the three-domain choice — and why it's wrong

The current three-domain distribution was not a deliberate taxonomy
decision. It is a residue of:
- The original four projects each had their own category (1:1)
- terraform-gcp was folded into security-baseline-audit (losing
  `CloudInfrastructure`)
- Three new `SystemsAdmin` projects were added without
  recategorization

**The result is lopsided:** SystemsAdmin contains 67% of projects.
The category grouping on the home page shows one section with 4
projects and two sections with 1 each. This is not useful
information architecture.

**Alternative 1 — Four-domain split by subject:**

| Domain | Projects |
|---|---|
| Identity & Security | identity-access-lifecycle, security-baseline-audit |
| Endpoints & Administration | endpoint-management-compliance |
| Observability | observability-operational-intelligence |
| Governance & Resilience | backup-recovery-continuity, operational-foundation |

More balanced (2-1-1-2). Requires a new enum or rename of existing
variants.

**Alternative 2 — Six-domain split (one per project):**

| Domain | Project |
|---|---|
| Identity | identity-access-lifecycle |
| Security | security-baseline-audit |
| Endpoints | endpoint-management-compliance |
| Resilience | backup-recovery-continuity |
| Observability | observability-operational-intelligence |
| Operations | operational-foundation |

This is what 02-target-structure.md proposed as `ProjectDomain`. Each
project is its own domain. The /platform page groups by domain, and
since each project IS a domain, the grouping is the project list
itself. This makes domain useful as a label, not as a grouping
mechanism — which is correct for the /platform page where projects
are grouped visually but each domain is a single node.

**Alternative 3 — Lifecycle-stage grouping:**

| Stage | Projects |
|---|---|
| Provision | security-baseline-audit, endpoint-management-compliance |
| Operate | observability-operational-intelligence, identity-access-lifecycle |
| Govern | backup-recovery-continuity, operational-foundation |

Balanced (2-2-2). Tells a story ("how infrastructure moves through
its lifecycle") rather than labeling subjects. Better for the
/platform page narrative. Worse for filtering (a sysadmin looking
for "security" projects won't find them under "Provision").

**Recommendation:** Alternative 2 (six-domain, one per project) for
the /platform page. Keep the current `ProjectCategory` enum for home
page filtering until the /platform page is built, then evaluate
whether the home page filters should use domain instead. The two
taxonomies can coexist: `category` for the home page filter tabs,
`domain` for the /platform page grouping.

---

## 3. Readiness Verdicts

### Status: NEEDS ADJUSTMENT

**What should change:**
1. Rename `SystemStatus` → `ProjectStatus` with variants
   `Shipped | InArchitecture | Planned`
2. Update all six registry entries to honest status values
3. Update `ProjectCard` status chip rendering:
   - `Shipped` → "● SHIPPED" (green)
   - `InArchitecture` → "◐ IN PROGRESS" (amber)
   - `Planned` → "○ PLANNED" (muted)
4. Add status-gated section rendering in `detail.rs`:
   - `Planned`: suppress case study body, show scope statement only
   - `InArchitecture`: render available content, note missing sections
   - `Shipped`: render all sections

**Scope:** Moderate. The enum rename and match-arm updates are
mechanical. The status-gated rendering in detail.rs is the
significant work — it requires conditional view logic for each case
study section.

**Urgency:** The "● LIVE" badge on planned projects is the most
visible dishonesty on the site. This should be the first Phase 2
change, before content rewrites.

### Domains: NEEDS ADJUSTMENT (but lower priority than status)

**What should change:**
1. Add `ProjectDomain` enum with six variants (one per project)
2. Add `domain: ProjectDomain` field to `ProjectIndex`
3. Recategorize projects (or keep `category` for home filtering
   and add `domain` as a parallel field for the /platform page)

**Scope:** Trivial if done as a parallel field. Moderate if
`category` is replaced entirely (requires updating home page
filter logic).

**Urgency:** Low. The current category grouping is lopsided but not
broken. Domain becomes critical only when the /platform page is
built. Can be deferred to the /platform-page batch of Phase 2.

---

## 4. Adjacent Risks

### Risk 1: "● LIVE" on planned projects (CRITICAL)

All six home page cards show "● LIVE" including the three planned
projects. A hiring manager sees six green-badged "live" projects,
clicks `endpoint-management-compliance`, and gets a placeholder.
This is worse than showing no badge — it sets an expectation the
page then violates.

**Recommendation:** First Phase 2 change. Block all other Phase 2
content work until the status enum is honest and the badge renders
correctly.

### Risk 2: CloudInfrastructure filter tab with 0 projects

The home page renders four filter tabs: All / Cyber Security / Cloud
Infrastructure / Systems Admin / Network Operations. The "Cloud
Infrastructure" tab shows count 0. Clicking it hides all project
cards. Not broken, but confusing — a visitor wonders why an empty
category exists.

**Recommendation:** Either suppress tabs with count 0 in the filter
rendering, or defer until the category/domain refactor. Low priority.

### Risk 3: `one_liner_for_project()` is a separate function, not a field

Adding a project currently requires editing `init_projects_index()`
AND the `one_liner_for_project()` match statement — two edits in
the same file. The Phase 1 scalability contract promises "append to
the registry" as a single operation. Having a parallel match
statement violates the spirit of that contract.

**Recommendation:** Move `one_liner` into the `ProjectIndex` struct
as a field during the struct refactor (when `domain`, `produces`,
`consumes` are added). Not a separate task — bundle with the struct
expansion.

### Risk 4: Phase 1.5 report's status language is aspirational

`PHASE1.5-COMPLETE.md` describes projects as "Shipped," "Operational,"
and "Planned." The actual code has all six as `SystemStatus::Operational`.
A future session reading the report without checking the code would
assume the status taxonomy is implemented. It is not.

**Recommendation:** No action needed — this preflight report
corrects the record. Phase 2 implements the real taxonomy.

### Risk 5: `subtitle` field does double duty

For shipped projects, `subtitle` is a technical tagline ("CIS-Aligned
Hardening, Terraform Compliance Gates..."). For planned projects,
`subtitle` is a scope statement ("The wiki, runbook library,
change-management process..."). The field serves different purposes
per status. This isn't a bug — the template renders `subtitle` the
same way regardless — but Phase 2 content work should be aware that
planned-project subtitles are scope statements, not taglines, and
will need rewriting when those projects move to InArchitecture.
