# CLAUDE.md

Personal eportfolio for Richard Mussell. Rust + Leptos 0.6 (CSR) compiled to wasm32-unknown-unknown via Trunk, deployed to GitHub Pages through a GitHub Actions workflow, live at richardmussell.github.io. The site's purpose is to effectively market the owner to technical recruiters and hiring managers for senior platform and infrastructure engineering roles.

## Branch and remote setup

Work happens on the `revamp` branch, which tracks `revamp-origin` (github.com/richmuscle/portfolio-revamp, a staging repo). The `main` branch tracks `origin` (github.com/richardmussell/richardmussell.github.io, the live site). `origin` is untouched until the revamp is ready to ship.

## Documentation

- Read `INVENTORY.md` for the complete current-state codebase map.
- Read `docs/ROADMAP.md` for the phased engineering upgrade plan and current phase.
- Read `docs/DECISIONS.md` for architectural decision records.

## Working agreements

- All work on feature branches, never main.
- One roadmap phase per session. Do not bundle phases. Plan before editing, get explicit approval before executing.
- Do not add `Co-Authored-By: Claude` trailers to commit messages on this repo. Git history reads as authored by the owner only.
- File is the memory. Any important understanding goes into a committed file, not just conversation.
- When starting a new session, read `INVENTORY.md`, `docs/ROADMAP.md`, and `docs/DECISIONS.md`, then run `git log --oneline -10`, `git status`, and `git branch -vv`, and summarize the state back to the user in under 200 words before proceeding.
- Check `git status` and `git log` before any git operation — work may have happened in a separate shell.
- Push back on scope creep. The owner's instinct is to say "do all of it" — when that's wrong, say so.
- Content fixes (title drift across pages, no completed certs, no real PDF resume, unquantified work experience, broken sitemap) matter more for the "land roles" goal than engineering chrome. Remind the owner of these when relevant.

## Hard stack constraints

- Stack stays: Rust + Leptos + WASM. No JS framework migrations.
- Output must remain deployable to GitHub Pages unless a phase explicitly changes hosting.
- No new dependencies without justifying against the WASM bundle size.
