# CLAUDE.md — Portfolio

Personal eportfolio for Richard Mussell. Rust + Leptos 0.6 (CSR) compiled to wasm32-unknown-unknown via Trunk, deployed to GitHub Pages through a GitHub Actions workflow, live at richardmussell.github.io. The site's purpose is to effectively market the owner to technical recruiters and hiring managers for senior platform and infrastructure engineering roles.

## Imports

@~/.claude/CLAUDE.md
@~/.claude/projects/portfolio/CONTEXT.md
@~/.claude/projects/portfolio/DECISIONS.md

Global rules and cross-project ADRs are loaded via the imports above. This file holds only what is specific to **this repo** — branch/remote wiring, portfolio-specific verification, working agreements, hard stack constraints, and the session-start drill.

## Branch and remote setup

Work happens on the `revamp` branch, which tracks `revamp-origin` (github.com/richmuscle/portfolio-revamp, a staging repo). The `main` branch tracks `origin` (github.com/richardmussell/richardmussell.github.io, the live site). `origin` is untouched until the revamp is ready to ship — and push access from this machine's credentials is read-only. A third remote, `deploy` (git@github.com:richmuscle/richmuscle.github.io.git), is the live-site mirror under the richmuscle account; pushing `revamp` to `deploy` is allowed when explicitly requested.

## Documentation

- `INVENTORY.md` — complete current-state codebase map.
- `docs/ROADMAP.md` — phased engineering upgrade plan and current phase.
- `docs/DECISIONS.md` — **portfolio-specific** ADRs (stack choice, remote wiring, inventory discipline, etc.).
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

1. Read `INVENTORY.md`, `docs/ROADMAP.md`, `docs/DECISIONS.md`.
2. Run `git log --oneline -10`, `git status`, `git branch -vv`.
3. Summarize the state back to the user in under 200 words before proceeding.

## Hard stack constraints

- Stack stays: Rust + Leptos + WASM. No JS framework migrations.
- Output must remain deployable to GitHub Pages unless a phase explicitly changes hosting.
- No new dependencies without justifying against the WASM bundle size.
- See **portfolio ADR-001** (CSR deploy), **portfolio ADR-007** (feature gating), **portfolio ADR-005** (content before new features).
