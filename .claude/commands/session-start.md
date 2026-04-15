---
description: Catch up on project state at the start of a new session
---

Read the following files in order, then summarize the current project state back to me in under 250 words so I can confirm we're aligned before starting work:

1. `CLAUDE.md` — project description, constraints, working agreements
2. `INVENTORY.md` — complete codebase map
3. `docs/ROADMAP.md` — phased upgrade plan and current phase
4. `docs/DECISIONS.md` — architectural decision records

Then run:
- `git log --oneline -10`
- `git status`
- `git branch -vv`
- `git remote -v`

In your summary, tell me: (a) what phase we're in, (b) what the last committed work was, (c) whether the working tree is clean, (d) which remote the current branch tracks, (e) what the next action should be according to the roadmap. Do not start any phase work until I give the go signal.
