# 00 — Pre-flight Conflict Inventory

Before the restructure can proceed, these conflicts must be resolved by the
owner. Each conflict lists the competing sources verbatim with file+line
references, then proposes a resolution. The owner decides; the restructure
implements.

---

## Conflict 1: Target Identity — Three Competing Professional Titles

The repo contains three distinct professional identities that cannot coexist
in a restructured site.

### Source A: CLAUDE.md (committed, canonical project instructions)

> `CLAUDE.md:3` — "The site's purpose is to effectively market the owner to
> technical recruiters and hiring managers for **senior platform and
> infrastructure engineering roles**."

### Source B: CLAUDE.sysadmin-portfolio.md (UNTRACKED, ambiguous authority)

> `CLAUDE.sysadmin-portfolio.md:9` — "**Identity being projected:** Linux
> Systems Administrator with modern infrastructure toolkit (IaC, containers,
> GitOps, observability). NOT 'DevOps engineer.' That positioning choice
> matters throughout the site copy."

### Source C: Live site (every rendered page)

The following files render "Systems Administrator & DevOps Engineer" as the
primary identity:

| File | Line | Verbatim string |
|---|---|---|
| `src/data/mod.rs` | 20 | `PROFESSIONAL_TITLE: &str = "Systems Administrator & DevOps Engineer"` |
| `index.html` | 18 | `<title>Richard J. Mussell \| Systems Administrator & DevOps Engineer</title>` |
| `index.html` | 19 | `<meta name="description" content="...Systems Administrator & DevOps Engineer...">` |
| `index.html` | 20 | `<meta property="og:title" content="...Systems Administrator & DevOps Engineer">` |
| `index.html` | 21 | `<meta property="og:description" content="Systems Administrator & DevOps Engineer...">` |
| `index.html` | 28 | `<meta name="twitter:title" content="...Systems Administrator & DevOps Engineer">` |
| `index.html` | 29 | `<meta name="twitter:description" content="Systems Administrator & DevOps Engineer...">` |
| `index.html` | 97 | JSON-LD: `"jobTitle": "Systems Administrator & DevOps Engineer"` |
| `index.html` | 98 | JSON-LD: `"description": "...Systems Administrator & DevOps Engineer..."` |
| `index.html` | 153 | `<noscript>`: "Systems Administrator & DevOps Engineer" |
| `README.md` | 3 | "**Systems Administrator & DevOps Engineer** — Portfolio site built with Rust..." |
| `README.md` | 175 | "**Richard J. Mussell** — Systems Administrator & DevOps Engineer..." |
| `src/pages/home.rs` | 87 | Meta description: "Systems Administrator & DevOps Engineer..." |
| `src/pages/home.rs` | 96 | Hero subtitle: `"Systems Administrator & DevOps Engineer"` |
| `src/pages/about.rs` | 9 | Meta description: "Systems Administrator & DevOps Engineer..." |
| `src/pages/about.rs` | 24 | Body text: "Systems Administrator and DevOps Engineer..." |
| `src/pages/resume.rs` | 15 | Meta description: "Systems Administrator & DevOps Engineer..." |
| `src/pages/resume.rs` | 25 | Resume header: `"Systems Administrator & DevOps Engineer"` |
| `src/pages/resume.rs` | 57 | Summary paragraph: "Systems Administrator and DevOps Engineer..." |
| `src/pages/contact.rs` | 15 | Meta description: "Systems Administrator & DevOps Engineer..." |
| `src/pages/contact.rs` | 22 | Subtext: "Systems Administrator and DevOps Engineer..." |
| `src/pages/one_pager.rs` | 20 | Title: `"Systems Administrator & DevOps Engineer"` |
| `src/pages/not_found.rs` | 11 | Meta description: "Systems Administrator & DevOps Engineer..." |

### Drifted variants (not "Systems Administrator & DevOps Engineer")

| File | Line | Verbatim string |
|---|---|---|
| `src/components/site_footer.rs` | 21 | `"Systems Administrator · DevOps · Platform Engineer · Oklahoma City"` |
| `src/pages/home.rs` | 98 | `"SOC-trained infrastructure engineer..."` |
| `src/pages/home.rs` | 106 | `"Systems Engineering & Lab Projects"` |
| `src/pages/writing.rs` | 63 | `"Writing · Richard Mussell · Systems Engineering"` |
| `manifest.json` | 4 | `"Systems Administrator & Platform Operations"` |
| `~/.claude/projects/portfolio/CONTEXT.md` | 6 | `"Systems Administrator & DevOps Engineer"` |

### Proposed resolution

Source B (`CLAUDE.sysadmin-portfolio.md`) represents the owner's most recent
strategic intent and aligns with the external handoff context. If the owner
confirms Source B wins:

- **Primary title:** "Linux Systems Administrator"
- **Modifier:** "with modern infrastructure toolkit" (IaC, automation,
  observability)
- **Suppress:** "DevOps Engineer" as a co-equal title; "Platform Engineer"
  as a title
- `PROFESSIONAL_TITLE` in `src/data/mod.rs` becomes the single source of
  truth; all 24+ hardcoded strings above get replaced with references to it
  or updated to match

If the owner resolves differently (e.g., keeps the dual title), adjust
accordingly.

**Decision required:** Which identity wins?

---

## Conflict 2: Target Level — Entry-Level vs Senior vs Staff

Four sources state four different target levels.

| Source | File:Line | Target level |
|---|---|---|
| CLAUDE.md | line 3 | "senior platform and infrastructure engineering roles" |
| CLAUDE.sysadmin-portfolio.md | lines 144-146 | "Entry-level to mid Linux Systems Administrator, Infrastructure Engineer, or Junior DevOps roles" |
| User prompt (this session) | — | "staff-engineer-lens Linux Systems Administrator" |
| RECRUITER_FIT.md | lines 11-16 | "Associate Platform / FDE I" as realistic floor; "None of the six personas advance this candidate for senior" |

### Proposed resolution

`CLAUDE.sysadmin-portfolio.md` line 146 appears to predate the current
ambition. The user's prompt for this session explicitly targets
"staff-engineer-lens." RECRUITER_FIT.md provides the grounded assessment
that senior is not yet realistic this cycle.

Recommendation: the restructure should **build the site for senior-level
positioning** (the structure, depth, and rigor a senior would exhibit) while
the owner works the Tier 1 unblocks from RECRUITER_FIT.md to close the
credibility gap. The site's architecture should not cap itself at entry-level
when the content can be upgraded incrementally.

**Decision required:** What level should the site target? The restructure
adapts to the answer.

---

## Conflict 3: Footer Label

| Source | String |
|---|---|
| `src/components/site_footer.rs:21` | `"Systems Administrator · DevOps · Platform Engineer · Oklahoma City"` |
| CLAUDE.sysadmin-portfolio.md (implied by identity) | "Linux Systems Administrator" (single role) |
| User prompt | `"Systems Administrator · Oklahoma City"` (dropping DevOps and Platform Engineer) |

### Proposed resolution

If Conflict 1 resolves in favor of Source B: footer becomes
`"Systems Administrator · Oklahoma City"`. One role, one location. The
three-label footer dilutes positioning — a hiring manager should immediately
know what role the candidate fills.

**Decision required:** Confirm the footer string after Conflict 1 is resolved.

---

## Conflict 4: Untracked Status of CLAUDE.sysadmin-portfolio.md — BLOCKER

```
$ git status
?? CLAUDE.sysadmin-portfolio.md
```

This file:
- Line 1 says: `> Place at ~/dev/projects/richardmussell.github.io/CLAUDE.md (merge with existing).`
- Contains 206 lines of detailed instructions: hero pitch, case study template,
  Platform page spec, about page copy, acceptance criteria, build sequence,
  voice guidelines, and the word "engineered" ban
- Is NOT imported by `CLAUDE.md` (which imports only `~/.claude/CLAUDE.md`,
  `~/.claude/projects/portfolio/CONTEXT.md`, `~/.claude/projects/portfolio/DECISIONS.md`)
- Is NOT in `.gitignore`

**Current status is ambiguous.** It could be:

1. **A draft** — written during a planning session, never finalized. In that
   case, its instructions should not drive the restructure.
2. **The intended source of truth** — meant to be merged into `CLAUDE.md` but
   the merge never happened. In that case, its instructions supersede `CLAUDE.md`
   on identity, template, and voice.
3. **Dead** — superseded by later decisions. In that case, it should be deleted
   or moved to `docs/rfcs/`.

The restructure cannot proceed until the owner resolves this. Every subsequent
step depends on whether this file's instructions are authoritative.

**Decision required:** Is `CLAUDE.sysadmin-portfolio.md` the source of truth,
a draft, or dead? Then either:
- (a) Merge its directives into `CLAUDE.md` and commit, or
- (b) Move it to `docs/rfcs/` as a historical planning artifact, or
- (c) Delete it

---

## Conflict 5: "Engineered" Word Ban vs Current Usage

`CLAUDE.sysadmin-portfolio.md:199` states:

> "Word 'engineered' must not appear more than once on the entire site"

The word currently appears **10 times** in source code:

| File | Line | Context |
|---|---|---|
| `src/data/projects.rs` | 436 | `"Engineered a data-driven performance telemetry pipeline..."` |
| `src/data/projects.rs` | 483 | `"Engineered an idempotent framework for a deterministic GCP Landing Zone..."` |
| `src/data/projects.rs` | 484 | `"Engineered an idempotent framework enforcing deterministic Linux state..."` |
| `src/data/projects.rs` | 485 | `"Engineered an idempotent framework for data-driven performance telemetry..."` |
| `src/data/projects.rs` | 486 | `"Engineered an idempotent framework for a ZTNA administrative fabric..."` |
| `src/data/projects.rs` | 487 | `"Engineered an idempotent framework for IT operations"` (fallback) |
| `src/pages/resume.rs` | 189 | `"Summary: Engineered a secure, modular landing zone..."` |
| `src/pages/resume.rs` | 207 | `"Summary: Engineered an identity-based SASE administrative fabric..."` |
| `src/pages/one_pager.rs` | 52 | `"Engineered a secure, version-controlled GCP Landing Zone..."` |
| `src/pages/one_pager.rs` | 70 | `"Engineered a ZTNA administrative fabric..."` |

### Proposed resolution

If `CLAUDE.sysadmin-portfolio.md` is the source of truth (Conflict 4), all
10 instances must be rewritten. Replace with active verbs that match sysadmin
voice: "Built", "Deployed", "Configured", "Automated", "Hardened." This is a
content pass, not a structural pass — but the ban must be acknowledged before
restructure proceeds so the new template enforces it going forward.

**Decision required:** Is the "engineered" ban in effect?

---

## Conflict 6: About Page — Prescribed Copy vs Actual

`CLAUDE.sysadmin-portfolio.md:137-161` prescribes specific about page copy:

> "I'm richmuscle — a Linux systems administrator based in Edmond, Oklahoma."
> ...
> "Entry-level to mid Linux Systems Administrator..."

`src/pages/about.rs:24` currently renders:

> "Richard Mussell is a Systems Administrator and DevOps Engineer based in
> Oklahoma City."

Differences: name form ("richmuscle" vs "Richard Mussell"), city ("Edmond"
vs "Oklahoma City"), identity (sysadmin-only vs dual DevOps), level
(entry-level vs unspecified), voice (first-person vs third-person).

### Proposed resolution

Depends on Conflicts 1, 2, and 4. If `CLAUDE.sysadmin-portfolio.md` is
source of truth, the about page needs a full rewrite. If it's dead, the
current copy stays but still needs identity alignment per Conflict 1.

---

## Conflict 7: Missing /platform Route

`CLAUDE.sysadmin-portfolio.md:44` declares:

> "/platform — the big 'how 6 projects connect' page"

`CLAUDE.sysadmin-portfolio.md:109`:

> "**The Platform page is the single highest-leverage page** — it's where 6
> projects become one story."

No `/platform` route exists in `src/lib.rs` or any routing table. The current
site has no equivalent — no dependency graph, no cross-project connection
diagram, no "one platform" narrative page.

### Proposed resolution

If the sysadmin framing wins and the file is authoritative, the `/platform`
route is a core structural addition for Step 2. If the file is dead, this
route is not required but should be evaluated on its own merit as a
high-leverage page for any restructure.

---

## Conflict 8: Case Study Template — Prescribed vs Actual

`CLAUDE.sysadmin-portfolio.md:73-107` prescribes a 7-section case study
template: The problem, What I built, Architecture, Key decisions, Tech stack,
What I'd do differently, Links.

The current project detail pages (`src/pages/project/detail.rs`,
`docs.rs`, `demo.rs`) use a 3-tab template (Case Study / Docs / Demo) that
renders content from JSON files. The 7-section structure does not appear in
the JSON schema.

### Proposed resolution

The 3-tab template is more ambitious than the prescribed 7-section single
page. The restructure should evaluate whether the existing template can
accommodate the prescribed sections as subsections within the Case Study tab,
preserving the Docs and Demo tabs. This is a Step 2 decision.

---

## Conflict 9: CONTEXT.md Staleness

`~/.claude/projects/portfolio/CONTEXT.md:6`:

> "Owner: Richard J. Mussell — Systems Administrator & DevOps Engineer"

`~/.claude/projects/portfolio/CONTEXT.md:9`:

> "Composite audit score: 7.4/10"

Both are stale. The identity must align with Conflict 1 resolution. The
composite score is 7.6 as of the 2026-04-20 audit.

### Proposed resolution

Update CONTEXT.md after Conflict 1 is resolved.

---

## Conflict 10: Loom Video Embed — Prescribed but Absent

`CLAUDE.sysadmin-portfolio.md:23` lists as in-scope:

> "Loom video embed (5-min walkthrough)"

`CLAUDE.sysadmin-portfolio.md:67`:

> Hero CTA: `[View the platform] [View my GitHub] [Download resume]`
> Below: Loom video embed (5 min).

No Loom video exists on the site. No iframe or embed code is present in any
source file. No video URL is referenced anywhere.

### Proposed resolution

If the file is authoritative, a Loom video is a required deliverable. If
dead, this is informational only. Either way, producing a 5-minute walkthrough
video is owner-dependent — the restructure can add the embed slot but cannot
produce the video.

---

---

# Project-List Reconciliation

Three project inventories exist. They partially overlap but are NOT the same
list. The restructure cannot proceed until the owner commits to one canonical
list.

## Inventory A — CLAUDE.sysadmin-portfolio.md (6 projects)

From `CLAUDE.sysadmin-portfolio.md:51-56`:

| # | Name | Route |
|---|---|---|
| 1 | Terraform Landing Zone | `/projects/terraform-landing-zone` |
| 2 | Ansible Baseline | `/projects/ansible-baseline` |
| 3 | Zero-Trust Admin Fabric | `/projects/zerotrust-admin-fabric` |
| 4 | GitOps Platform | `/projects/gitops-platform` |
| 5 | Observability Stack | `/projects/observability-stack` |
| 6 | CI/CD Showcase | `/projects/cicd-showcase` |

## Inventory B — External handoff (6 projects)

From the owner's external planning thread:

| # | Name |
|---|---|
| 1 | Identity & Access Lifecycle Platform |
| 2 | Endpoint Management & Compliance System |
| 3 | Security Baseline & Continuous Audit Toolkit |
| 4 | Backup, Recovery & Business Continuity System |
| 5 | Observability & Operational Intelligence Platform |
| 6 | Operational Foundation (Wiki/Runbooks/Change Mgmt/IR) |

## Inventory C — Currently shipping (4 projects)

From `public/projects/*.json` and `src/data/projects.rs`:

| # | Slug | Title |
|---|---|---|
| 1 | `linux-admin-scripting` | Systems Lifecycle Automation Framework |
| 2 | `monitoring-observability` | Multi-Tier Strategic Observability Pipeline |
| 3 | `terraform-gcp` | Hardened Cloud Landing Zone (IaC) |
| 4 | `zero-trust-networking` | Zero-Trust Administrative Fabric |

## Overlap Map

| Inventory A | Inventory B | Inventory C |
|---|---|---|
| Terraform Landing Zone | — | `terraform-gcp` (same concept, different name) |
| Ansible Baseline | Endpoint Management & Compliance | `linux-admin-scripting` (partial — scripting, not Ansible) |
| Zero-Trust Admin Fabric | Identity & Access Lifecycle Platform (partial) | `zero-trust-networking` (same concept) |
| GitOps Platform | — | — (absent) |
| Observability Stack | Observability & Operational Intelligence Platform | `monitoring-observability` (same concept) |
| CI/CD Showcase | — | — (absent) |
| — | Security Baseline & Continuous Audit Toolkit | — (absent from both A and C) |
| — | Backup, Recovery & Business Continuity System | — (absent from both A and C) |
| — | Operational Foundation (Wiki/Runbooks/Change Mgmt/IR) | — (absent from both A and C) |

## Gap Analysis

**Present in A but absent from C:**
- Ansible Baseline (C has bash scripting, not Ansible)
- GitOps Platform (no equivalent on site)
- CI/CD Showcase (no equivalent on site)

**Present in B but absent from both A and C:**
- Security Baseline & Continuous Audit Toolkit
- Backup, Recovery & Business Continuity System
- Operational Foundation (Wiki/Runbooks/Change Mgmt/IR)

**Present in C but not named in A or B:**
- `linux-admin-scripting` exists on site but A names it "Ansible Baseline"
  (different tool) and B names it "Endpoint Management & Compliance System"
  (different scope)

**Key question:** Does the owner intend to:
1. Keep the current 4 projects and restructure around them?
2. Expand to 6 projects from Inventory A (requires building 2 new projects)?
3. Expand to 6 projects from Inventory B (requires building 4 new projects)?
4. Merge A and B into a hybrid 6-project list?

Per portfolio ADR-005 ("content before new features") and
`CLAUDE.sysadmin-portfolio.md:36` ("Case study exists only when the project
repo is merged, CI green"), adding projects that don't exist as shipped repos
would violate both constraints.

**Decision required:** Which project list is canonical? The restructure will
build the information architecture around that list and only that list.

---

# Summary of Decisions Required

| # | Decision | Blocks |
|---|---|---|
| 1 | Which professional identity? (sysadmin-only, dual DevOps, or platform) | Everything |
| 2 | Which target level? (entry, senior, or staff-lens) | Template depth, voice |
| 3 | Is `CLAUDE.sysadmin-portfolio.md` source of truth, draft, or dead? | Identity, template, voice, word bans, /platform route, Loom |
| 4 | Which project list is canonical? (A, B, C, or hybrid) | Information architecture, routes, sitemap |
| 5 | Is the "engineered" word ban in effect? | All content strings |
| 6 | Footer: single-label or multi-label? | `site_footer.rs` |

**No step of the restructure should proceed until all six decisions are
resolved.** I will implement whatever the owner decides.
