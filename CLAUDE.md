# CLAUDE.md — Portfolio

Personal portfolio for Richard Mussell. Rust + Leptos 0.6 (CSR) compiled to wasm32-unknown-unknown via Trunk, deployed to GitHub Pages through a GitHub Actions workflow, live at richmuscle.github.io. The site's purpose is to market the owner to technical recruiters and hiring managers for **senior Linux Systems Administrator** roles.

## Imports

@~/.claude/CLAUDE.md
@~/.claude/projects/portfolio/CONTEXT.md
@~/.claude/projects/portfolio/DECISIONS.md

Global rules and cross-project ADRs are loaded via the imports above. This file holds only what is specific to **this repo** — identity, positioning, voice, content rules, branch/remote wiring, verification, working agreements, hard stack constraints, and the session-start drill.

## Identity and positioning

**Identity being projected:** Linux Systems Administrator with modern infrastructure toolkit (IaC, automation, observability, identity management). NOT "DevOps Engineer." NOT "Platform Engineer." That positioning choice matters throughout the site copy.

**Target level:** Senior. The word "staff" is a lens applied to the depth and rigor of the work (ADRs, failure modes, measurement methodology, scope boundaries). It never appears as a claimed title in user-visible copy.

**Footer label:** `© 2026 Richard J. Mussell · Linux Systems Administrator · Oklahoma City`

**PROFESSIONAL_TITLE constant** (`src/data/mod.rs`) is the single source of truth for the professional title string. Every page that renders the title must reference this constant, not hardcode the string.

## Voice and banned words

Voice is sysadmin: direct, operational, first-person, honest about limits. Confident. Grounded. If a sentence starts with "Engineered a robust..." delete and rewrite.

**Banned words (site copy and restructure docs):** "engineered" (at most once on the entire site), "robust", "leverage" (as verb), "seamless", "cutting-edge", "state-of-the-art", "world-class", "best-in-class", "next-gen".

Voice must match the project repo READMEs: SysAdmin-first, first-person, direct, honest about limitations.

## Content rules

1. **Case study exists only when the project repo is merged and CI green.** No exceptions. No publishing case studies for unbuilt work.
2. **Every case study links to the real GitHub repo, README, ARCHITECTURE, and at least one ADR.** The site is a portal, not a replacement.
3. **Screenshots are real** — from actual project runs, not mockups.
4. **No placeholder content.** If a demo page says "Coming Soon," either fill it or remove the route. The site never pretends unbuilt work exists.
5. **Project status is honest:** `shipped` | `in-architecture` | `planned`. The template renders differently per status. A `planned` project shows title and scope only. An `in-architecture` project has design docs and ADRs but no demo. A `shipped` project has all three tabs populated.

## Project registry (scalability contract)

`src/data/projects.rs` is the **single canonical registry** for all projects. Every surface that lists, counts, links to, or describes projects reads from this registry — home page cards, navigation, the platform page, sitemap, OG metadata, JSON-LD, search index, and cross-references.

Adding a project is a three-file change, maximum:
1. Append to the registry in `src/data/projects.rs`
2. Add `public/projects/<slug>.json` (per-project content)
3. Add `docs/projects/<slug>/` directory (design docs, ADRs)

If adding a project requires editing more than these three locations, the architecture has a bug. See `docs/restructure/` for the full scalability contract.

## The Platform page

`/platform` is the single highest-leverage page. It renders a project-connection graph from registry data (`produces`/`consumes` arrays on each project entry). When a hiring manager opens one page on this site, it should be this one. The graph must work at 6 projects and at 30.

## Case study template (consistent across all projects)

Three tabs per project: **Case Study** (the argument), **Documentation** (the specification), **Demo** (the proof).

Case Study tab sections:
- Context — one paragraph, sysadmin-framed: what operational pain drove this?
- Architecture at a Glance — diagram + 3-5 sentences
- Decisions — 5-8 ADR summaries with rejected alternatives named
- Failure Modes & Operational Reliability — table
- Measurement — numbers with methodology attached
- Scope Boundaries — what was deliberately not built
- Connections — auto-rendered from registry `produces`/`consumes`
- What I'd do differently — honest limitations. **Non-negotiable on every case study.**

## Branch and remote setup

Work happens on the `revamp` branch, which tracks `revamp-origin` (github.com/richmuscle/portfolio-revamp, a staging repo). The `main` branch tracks `origin` (github.com/richardmussell/richardmussell.github.io, the live site). `origin` is untouched until the revamp is ready to ship — and push access from this machine's credentials is read-only. A third remote, `deploy` (git@github.com:richmuscle/richmuscle.github.io.git), is the live-site mirror under the richmuscle account; pushing `revamp` to `deploy` is allowed when explicitly requested.

## Documentation

- `docs/INVENTORY.md` — complete current-state codebase map.
- `docs/ROADMAP.md` — phased engineering upgrade plan and current phase.
- `docs/DECISIONS.md` — **portfolio-specific** ADRs (stack choice, remote wiring, inventory discipline, etc.).
- `docs/restructure/` — structural foundation and content audit for the staff-lens restructure.
- `~/.claude/projects/portfolio/DECISIONS.md` — **portfolio cross-cutting** ADRs (deploy mode, CSS layering, GlobalAppState, sqlite-wasm, content-before-engineering, error handling, feature gating).
- `~/.claude/DECISIONS.md` — unrelated: holds the ferro43 SOC/SIEM project's ADRs. Not relevant to this repo; do not cite.

**Two portfolio-scoped DECISIONS.md files coexist with distinct scopes.** When the unqualified "ADR-XXX" is used in this repo, it refers to the **local** `docs/DECISIONS.md` (very-local architecture records — stack choice, remote wiring, inventory discipline). To cite a portfolio cross-cutting ADR, use **"portfolio ADR-XXX"** explicitly. Override syntax likewise: `OVERRIDES ADR-XXX` (local) vs `OVERRIDES portfolio ADR-XXX`.

## Verification for this repo

Global verification rules reference the SOC stack (`stack-health.sh`, `wazuh-manager`, etc.) and do **not** apply here. Portfolio verification is:

- `cargo check --no-default-features --features ssr`
- `cargo check --features hydrate --target wasm32-unknown-unknown`
- `cargo check --target wasm32-unknown-unknown` (default CSR + sqlite)
- `trunk build --release` before any deploy

Run the relevant check after every Rust change. A merge to `main` requires all three `cargo check` targets green and `trunk build --release` succeeding.

## Working agreements (project-specific)

- All work on feature branches, never `main`.
- **Do not add `Co-Authored-By: Claude` trailers** to commit messages on this repo. Git history reads as authored by the owner only. (Overrides the default assistant-coauthor convention.)
- File is the memory. Any important understanding goes into a committed file, not just conversation.
- Check `git status` and `git log` before any git operation — work may have happened in a separate shell.
- Push back on scope creep. The owner's instinct is to say "do all of it" — when that's wrong, say so.
- Content fixes (title drift across pages, no completed certs, no real PDF resume, unquantified work experience, broken sitemap) matter more for the "land roles" goal than engineering chrome. Remind the owner of these when relevant. See **portfolio ADR-005**.

## Session-start drill

When starting a new session in this repo:

1. Read `CLAUDE.md`, `docs/INVENTORY.md`, `docs/ROADMAP.md`, `docs/DECISIONS.md`.
2. Run `git log --oneline -10`, `git status`, `git branch -vv`.
3. Summarize the state back to the user in under 200 words before proceeding.

## Hard stack constraints

- Stack stays: Rust + Leptos + WASM. No JS framework migrations.
- Output must remain deployable to GitHub Pages unless a phase explicitly changes hosting.
- No new dependencies without justifying against the WASM bundle size.
- See **portfolio ADR-001** (CSR deploy), **portfolio ADR-007** (feature gating), **portfolio ADR-005** (content before new features).
