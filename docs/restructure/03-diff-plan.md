# 03 — Diff Plan

Every file that needs to change to execute the target structure from
02-target-structure.md. Organized into deployable batches with
dependency ordering, classification, and kill-switch analysis.

---

## A. Complete File Manifest

### Files to modify (Rust source)

| File | Change | Classification |
|---|---|---|
| `src/data/mod.rs` | Update `PROFESSIONAL_TITLE` constant | Trivial |
| `src/data/projects.rs` | Restructure `ProjectIndex` struct (rename fields, add `one_liner`, `domain`, `produces`, `consumes`; replace `SystemStatus` with `ProjectStatus`; add `ProjectDomain` enum; remove `one_liner_for_project()` function; add 3 new `Planned` entries + update 4 existing) | Substantial |
| `src/components/site_footer.rs` | Replace hardcoded footer string with `PROFESSIONAL_TITLE` interpolation | Trivial |
| `src/pages/home.rs` | Replace hardcoded title strings with `PROFESSIONAL_TITLE`; replace `"4 Projects · 4 Disciplines"` with computed count; rewrite hero copy for sysadmin voice | Moderate |
| `src/pages/about.rs` | Replace Meta description + body text with sysadmin identity | Moderate |
| `src/pages/resume.rs` | Replace hardcoded title strings; replace hardcoded project list with registry-driven iteration; rewrite summary for sysadmin identity | Moderate |
| `src/pages/contact.rs` | Replace hardcoded title strings + subtext | Moderate |
| `src/pages/one_pager.rs` | Replace hardcoded title + project list with registry-driven iteration | Moderate |
| `src/pages/not_found.rs` | Replace Meta description | Trivial |
| `src/pages/writing.rs` | Replace `"Systems Engineering"` in Title tag | Trivial |
| `src/pages/telemetry.rs` | Replace hardcoded slug vec with `get_infrastructure_fleet()` iteration | Trivial |
| `src/pages/project/detail.rs` | Add status-gated section rendering; add failure-modes section; add connections section (reads `produces`/`consumes` from registry) | Substantial |
| `src/pages/project/docs.rs` | Add status-gated rendering | Moderate |
| `src/pages/project/demo.rs` | Add status-gated rendering | Moderate |
| `src/lib.rs` | Add `/platform` route pointing to new `PlatformPage` | Trivial |
| `src/pages/mod.rs` | Add `pub mod platform;` | Trivial |
| `src/bin/ssg.rs` | May need update if slug names change; already reads from registry | Trivial |
| `src/db.rs` | May need update if fields on `ProjectIndex` change; already reads from registry | Trivial |

### Files to create

| File | Purpose | Classification |
|---|---|---|
| `src/pages/platform.rs` | New PlatformPage component: connection graph + domain groups | Substantial |
| `style/pages/platform.css` | Styles for the platform page | Moderate |
| `docs/projects/` (directory) | Per-project document directories (one per slug) | Trivial (mkdir) |

### Files to modify (non-Rust)

| File | Change | Classification |
|---|---|---|
| `index.html` | Update 9 title/meta/OG/JSON-LD strings | Trivial |
| `README.md` | Update 2 identity strings | Trivial |
| `manifest.json` | Update description | Trivial |
| `public/sitemap.xml` | Generate from registry data or update manually; add /platform route | Moderate |
| `style/style.scss` | Add `@import 'pages/platform'` | Trivial |

### Files to potentially rename (if slug rename is chosen)

| Old | New | Notes |
|---|---|---|
| `public/projects/terraform-gcp.json` | `public/projects/terraform-landing-zone.json` | If slug changes |
| `public/projects/linux-admin-scripting.json` | `public/projects/endpoint-compliance.json` | If slug changes |
| `public/projects/monitoring-observability.json` | `public/projects/observability-platform.json` | If slug changes |
| `public/projects/zero-trust-networking.json` | `public/projects/identity-access-lifecycle.json` | If slug changes |
| Corresponding `public/docs/` and `public/demos/` files | Same pattern | If slug changes |

### Files to create (per-project content for new entries)

| File | Status | Notes |
|---|---|---|
| `public/projects/security-baseline.json` | Planned — stub only | `{"slug":"security-baseline","content":""}` |
| `public/projects/backup-recovery.json` | Planned — stub only | `{"slug":"backup-recovery","content":""}` |
| `public/projects/operational-foundation.json` | Planned — stub only | `{"slug":"operational-foundation","content":""}` |

### Files to delete (if writeups are retired — owner decides)

| File | Verdict |
|---|---|
| `public/writeups/the-architect-of-the-prismatic-apex-*.json` | Retire |
| `public/writeups/the-orchestrated-landscape-*.json` | Retire |
| `public/writeups/the-sustainable-architect-*.json` | Retire |
| `public/writeups/the-builders-ledger-*.json` | Retire |
| `public/pdfs/builders-ledger.pdf` | Follows writeup |
| `public/pdfs/orchestrated-landscape.pdf` | Follows writeup |
| `public/pdfs/sustainable-architect.pdf` | Follows writeup |

### Files to update outside repo

| File | Change |
|---|---|
| `~/.claude/projects/portfolio/CONTEXT.md` | Update identity string + audit score |

---

## B. Dependency Ordering

```
Phase    Description                         Depends on
──────────────────────────────────────────────────────────────
  1      Registry refactor                   Nothing
         (projects.rs struct + enum + data)

  2      Template refactor                   Phase 1
         (detail.rs, docs.rs, demo.rs
          status-gated rendering)

  3      Platform page                       Phase 1
         (platform.rs + route + CSS)

  4      Identity sweep                      Phase 1
         (PROFESSIONAL_TITLE + all
          hardcoded strings in Rust pages)

  5      Registry consumers                  Phase 1 + 4
         (resume.rs, one_pager.rs,
          telemetry.rs — read from registry
          instead of hardcoding)

  6      Non-Rust metadata                   Phase 4
         (index.html, README, manifest,
          sitemap, JSON-LD)

  7      Content audit + rewrites            Phase 4 + 5
         (writeup retitles/retirements,
          banned-word fixes, voice rewrites)

  8      Per-project doc directories         Phase 1
         (docs/projects/<slug>/)
```

---

## C. Per-Change Classification

### Phase 1: Registry refactor

| Change | Complexity | Reversible | Breaks build | Affects `just check` | Affects SEO |
|---|---|---|---|---|---|
| Rename `SystemStatus` → `ProjectStatus` | Moderate | Yes (git revert) | **Yes** — every consumer must update simultaneously | Yes — all 4 gates | No |
| Add `Shipped/InArchitecture/Planned` variants | Moderate | Yes | Yes — match arms must be exhaustive | Yes | No |
| Add `one_liner` field to `ProjectIndex` | Trivial | Yes | Yes — struct init must include new field | Yes | No |
| Add `domain` field + `ProjectDomain` enum | Moderate | Yes | Yes — same reason | Yes | No |
| Add `produces`/`consumes` fields | Trivial | Yes | Yes — same reason | Yes | No |
| Remove `one_liner_for_project()` function | Trivial | Yes | Yes — callers must update | Yes | No |
| Add 3 new `Planned` entries | Trivial | Yes | No | No | No |
| Rename `subtitle` → `tagline`, `description` → `summary` | Moderate | Yes | **Yes** — every consumer | Yes | No |

**NOTE:** All registry changes must land in a single commit. The
struct, enum, and all four `init_projects_index()` entries must be
consistent. Partial changes will not compile.

### Phase 2: Template refactor

| Change | Complexity | Reversible | Breaks build | Affects `just check` | Affects SEO |
|---|---|---|---|---|---|
| Status-gated rendering in detail.rs | Substantial | Yes | Temporarily (during development) | Yes | No |
| Status-gated rendering in docs.rs | Moderate | Yes | Temporarily | Yes | No |
| Status-gated rendering in demo.rs | Moderate | Yes | Temporarily | Yes | No |
| Connections section from registry | Moderate | Yes | Temporarily | Yes | No |

### Phase 3: Platform page

| Change | Complexity | Reversible | Breaks build | Affects `just check` | Affects SEO |
|---|---|---|---|---|---|
| New `platform.rs` component | Substantial | Yes | No (additive) | No | Yes (new route) |
| Route in lib.rs | Trivial | Yes | No | No | Yes |
| `style/pages/platform.css` | Moderate | Yes | No | No | No |

### Phase 4: Identity sweep

| Change | Complexity | Reversible | Breaks build | Affects `just check` | Affects SEO |
|---|---|---|---|---|---|
| Update `PROFESSIONAL_TITLE` | Trivial | Yes | No | No | Yes (title tags change) |
| 11 Rust page string updates | Trivial each | Yes | No | No | Yes |
| Footer string update | Trivial | Yes | No | No | No |

### Phase 5: Registry consumers

| Change | Complexity | Reversible | Breaks build | Affects `just check` | Affects SEO |
|---|---|---|---|---|---|
| resume.rs reads from registry | Moderate | Yes | No | No | No |
| one_pager.rs reads from registry | Moderate | Yes | No | No | No |
| telemetry.rs reads from registry | Trivial | Yes | No | No | No |

### Phase 6: Non-Rust metadata

| Change | Complexity | Reversible | Breaks build | Affects `just check` | Affects SEO |
|---|---|---|---|---|---|
| index.html: 9 string updates | Trivial | Yes | No | No | **Yes** — OG cards, JSON-LD |
| README.md: 2 string updates | Trivial | Yes | No | No | No |
| manifest.json: 1 string update | Trivial | Yes | No | No | No |
| sitemap.xml: update routes | Moderate | Yes | No | No | **Yes** |

---

## D. Recommended Change Batches

Each batch is an individually-deployable unit. The site is never in a
half-restructured state between batches.

### Batch 1: Registry foundation
**Phases 1 + partial 2 (struct-compatible template changes)**
- Restructure `ProjectIndex` struct + enums
- Update all 4 existing entries to new field names
- Add 3 new Planned entries
- Update all consumers to compile (detail.rs, docs.rs, demo.rs, home.rs, palette.rs, nav.rs, db.rs, ssg.rs)
- Add basic status-gated rendering (can be minimal: just suppress sections for non-Shipped)
- Verify: all 4 `just check` gates pass

**Commit message:** `refactor(data): restructure project registry for scalability contract`

### Batch 2: Identity sweep
**Phases 4 + 6**
- Update `PROFESSIONAL_TITLE`
- Update all 28 hardcoded strings (Rust + HTML + Markdown + JSON)
- Update footer
- Verify: `just check` passes, `trunk build --release` produces correct `<title>` tags

**Commit message:** `fix(identity): centralize on Linux Systems Administrator`

### Batch 3: Registry consumers
**Phase 5**
- Refactor resume.rs to iterate registry instead of hardcoded vec
- Refactor one_pager.rs to iterate registry instead of hardcoded array
- Refactor telemetry.rs to iterate registry for probe slugs
- Verify: `just check` passes

**Commit message:** `refactor(pages): source project data from registry`

### Batch 4: Platform page
**Phase 3**
- Create `src/pages/platform.rs` with connection graph rendering
- Add route in `lib.rs`
- Add `style/pages/platform.css`
- Update sitemap
- Verify: `just check` passes, new route loads correctly

**Commit message:** `feat(platform): add connection-graph page`

### Batch 5: Content audit implementation
**Phase 7 — separate session per CLAUDE.md ("one phase per session")**
- Writeup retitles/retirements
- Banned-word replacements (10 "engineered" instances + others)
- Voice rewrites on hero, about, resume, contact copy
- Per-project doc directories

**Multiple commits, one per content area.**

---

## E. Kill-Switch Analysis

| Batch | Safe to revert individually? | Side effects of revert |
|---|---|---|
| Batch 1 (Registry) | **No** — all subsequent batches depend on the new struct | Reverting this requires reverting everything |
| Batch 2 (Identity) | **Yes** — string-only changes, no structural dependency | Old identity strings return; no build breakage |
| Batch 3 (Consumers) | **Yes** — restores hardcoded lists; registry still works | Adds back the scalability violations but nothing breaks |
| Batch 4 (Platform) | **Yes** — additive; removing the route + component is clean | /platform route disappears, no other pages affected |
| Batch 5 (Content) | **Yes per commit** — each content change is independent | Individual content changes revert cleanly |

**Safe abort point:** After Batch 2, the site has correct identity
everywhere but still has the old hardcoded project lists. This is a
valid ship state (better than current).

**Point of no return:** Batch 1 (registry refactor). Once the struct
changes, reverting requires reverting all consumer updates too. But
since Batch 1 includes consumer updates to compile, it is
self-consistent and can stay even if later batches are abandoned.

---

## F. Honest Status Representation

The diff plan must ensure the site never pretends unbuilt work exists.

### What the three `Planned` entries look like on the live site

**Home page:** Card with muted styling, "PLANNED" badge, title and
tagline only. No "View Case Study" link — replaced with "Coming soon."

**Platform page:** Node appears in the graph (dotted outline) with
connections declared. Story paragraph acknowledges: "Three projects
are in planning."

**Project page (if navigated to directly):** Title, tagline, status
badge, and connections section. All other sections suppressed. A clear
note: "This project is planned. Design documentation and case study
will appear as the project progresses."

**Not listed in:** resume, one-pager, JSON-LD, sitemap (until at
least InArchitecture).

### What the three `InArchitecture` entries look like

**Home page:** Card with standard styling, "IN ARCHITECTURE" badge.
"View Case Study" link works — detail page shows what exists.

**Project page:** Context, Architecture, Decisions, Scope Boundaries,
and Connections render. Measurement, Failure Modes, Demo, and "What
I'd Do Differently" sections are suppressed with a note.

**Listed in:** sitemap (priority 0.5), platform page (dashed node).
**Not listed in:** resume, one-pager, JSON-LD.

---

## G. Adding Project N+1 — Walkthrough

After the restructure ships, adding a new project:

**Step 1:** Add a new `ProjectIndex` entry to `init_projects_index()`
in `src/data/projects.rs`. Set the status to `Planned`,
`InArchitecture`, or `Shipped`. Declare `produces` and `consumes`.

**Step 2:** Create `public/projects/<slug>.json` with the content
appropriate for the status level. For `Planned`, this can be
`{"slug":"<slug>","content":""}`.

**Step 3:** (Shipped projects only) Create `docs/projects/<slug>/`
with design docs, ADRs, threat model, runbooks as appropriate.

**That's it.** The project appears on:
- Home page (registry-driven card iteration)
- Platform page (graph auto-updates from `produces`/`consumes`)
- Sitemap (generated from registry, if status >= InArchitecture)
- Command palette (reads from registry)
- SQLite search index (reads from registry)
- SSG route list (reads from registry)

No other files need editing. If this walkthrough ever requires a
fourth step, the registry design has a bug.
