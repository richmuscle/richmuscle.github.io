# Orchestrating a 22-Agent Audit of My Own Portfolio

*April 15, 2026 — read in 6 minutes*

---

## The problem

I ran a manual audit of this portfolio in the morning, scored it, noted the gaps. By afternoon I had shipped six commits — a Fibonacci/golden-ratio CSS refactor, a hero-stat panel removal, a wasm-opt root-cause investigation. Each change could have introduced regressions in any of five dimensions: code quality, front-end UX, content calibration, build/deploy, security. Re-auditing by hand would have taken two to three hours. I wanted a second full sweep by end of day.

The more interesting question was whether the audit could also simulate the people the portfolio is supposed to persuade — technical hiring managers in specific market segments — rather than producing a single-voice opinion. One auditor can calibrate against one standard. Six auditors with different hiring bars and different first-30-second rejection signals produce a more useful signal.

The constraints were hard: Rust + Leptos + WASM is not the kind of stack a general-purpose audit agent knows by heart. The agent needs to navigate Cargo feature gates, a custom SCSS pipeline, a manually-maintained SSG, and a content surface that spans Rust source files (`resume.rs`, `about.rs`, `project.rs`) as well as JSON blobs and static HTML. Mixing those concerns into a single agent context guarantees shallow coverage of each.

---

## The architecture: five waves, twenty-two agents

The pipeline was orchestrated by Opus 4.6 in Fast mode with structured JSON handoffs between waves. Agent contexts were isolated — no cross-contamination of partial analysis.

```
Wave 1: Six parallel read-only lanes (Rust quality, frontend UX,
         content fit, build/deploy, security, recon)
         Models: 3× Sonnet 4.6, 3× Haiku 4.5
         Output: per-lane JSON with scores, findings, P0/P1 lists

Wave 2: Four narrative-consistency sweeps (bio, resume, projects, meta)
         Models: 4× Haiku 4.5
         Output: raw claim extraction, surface divergence tables

Wave 3: Six recruiter-persona simulations
         (FAANG SRE, SaaS Platform, Rust Shop, Fintech Infra,
          Defense FDE, Startup CTO)
         Models: 6× Sonnet 4.6
         Output: verdict, phone-screen probability, per-item flags

Wave 4: One engineering-patch agent
         Model: Sonnet 4.6
         Constraints: isolated git worktree off `revamp`, P0s only,
         3× cargo check + trunk build --release per commit
         Output: 5 commits, bundle delta, new ADR

Wave 5: Six content-production lanes
         (3 resume variants, live-content rewrite,
          writing-section reorder + one-pager alignment,
          domain unification, this writeup)
         Models: mix of Sonnet 4.6 and Haiku 4.5
         Output: draft files, no auto-commit to production
```

Total: 22 agents across five waves. The audit report header says "16 parallel subagents" because it was written at the end of Wave 3 before Waves 4 and 5 were greenlit. Both numbers are accurate to their scope.

**Model tier rationale:** Haiku 4.5 is roughly 10× cheaper than Sonnet 4.6 on a per-token basis. Wave 2's narrative sweep is pure claim extraction — line-by-line comparison of on-site resume fields against PDF fields, domain string matching across static HTML. No judgment required. Haiku handles it correctly and cheaply. Wave 3 recruiter personas require weighing soft signals, inferring from job-market norms, and simulating decision-making under ambiguity. Sonnet-class judgment is necessary there. The model tier is matched to the task, not assigned uniformly to minimize cost or maximize quality across the board.

**JSON handoffs:** Each wave's output is a structured JSON object conforming to a schema the orchestrator validates before synthesis. The synthesis step (Opus) does not re-read the source code — it consumes the JSON summaries from all six Wave 1 agents, merges scores, identifies disagreements, and weights them into a composite. This makes the orchestration reproducible: run it again tomorrow with the same schemas and the orchestrator behavior is deterministic given the same agent outputs. Re-reading source at synthesis time would introduce a second full-context pass and roughly double the cost.

**Isolated git worktrees:** The engineering-patch agent (Wave 4) ran in an isolated worktree cloned from `revamp`. This is a hard constraint from the project's agent rules (`patch-agent works in an isolated git worktree only`). The reason is mechanical: an auto-patching agent that works in the main checkout can break the working tree during the cargo check verification step, leaving partially-applied changes and a failing build in the branch you're actively using. Isolated worktrees are auto-cleaned after the wave completes. If the verify step fails on a commit, the damage is contained in the throwaway worktree, not in your day's work.

---

## What the swarm found

### Composite score: 6.8 / 10

Down 0.3 from the morning audit after the bundle measurement was added as a weighted P0. The five-lens breakdown:

| Lens | Score |
|---|---|
| Architecture & code organization | 8 |
| CS depth signals | 7 |
| Rust idioms & type discipline | 7 |
| Deploy & build rigor | 6 |
| Content / marketing fit | 4 |

### The engineering finding that mattered most

The recon lane measured the WASM bundle: 1,485,823 bytes uncompressed, 618 KB gzipped — 24% over the 500 KB target. The morning audit had flagged the bundle as unoptimized but had not measured it. The afternoon run measured it.

The root cause turned out to be a single HTML attribute. `trunk.toml` was passing `wasm-opt -Oz` to Trunk's optimization pipeline, but `index.html` had `data-wasm-opt="0"` on the `<link data-trunk>` element, which overrides the config-level setting and silently disables wasm-opt entirely. The fix is a one-line deletion. The patch agent removed the attribute, trimmed three redundant `web-sys` feature declarations, and confirmed the bundle dropped 7% uncompressed on the next `trunk build --release`.

The lesson isn't "check your config attributes" — it's that the kind of silent override that accumulates in a project over many incremental commits is exactly the thing a fresh audit sweep finds that a developer embedded in the codebase misses.

### The content finding that mattered most

Wave 2's narrative sweep compared the on-site resume against the PDF resume field by field. The on-site resume lists 22 skills. The PDF lists 34. The 12 suppressed tools include Prometheus, Grafana, Entra ID, Intune, Autopilot, and Python — not obscure items.

More consequentially: the PDF discloses two in-progress certifications (GCP ACE and CKA) as "pursuing." The on-site resume hides them. Six recruiter personas, independently, cited concealment of in-progress certs as reading worse than honest disclosure. A cert "pursuing with target date Q3 2026" is a forward signal. A cert that appears in the PDF but not the website reads as inconsistency — which a technical reviewer notices and a recruiter skips past.

### Recruiter-persona consensus

None of the six personas advanced the candidate for senior. The consensus table:

| Persona | Verdict | Phone screen (senior tier) | Best-fit |
|---|---|---|---|
| FAANG SRE | reject | 0.04 | no match |
| SaaS Platform | reject | 0.08 | Associate PE |
| Rust Shop (Oxide/Fermyon) | defer | 0.15 | Jr developer-tools |
| Fintech Infra | defer | 0.22 | Associate / Jr SecOps |
| Defense FDE (Palantir/Anduril) | defer (highest fit) | 0.28 | FDE I |
| Startup CTO (A/B) | defer | 0.22 | Jr Platform |

All six cited the same three disqualifiers: current role on live resume (Product Brand Ambassador), zero completed professional certs, and all four portfolio projects unverifiable (no public repos, no CI output, no benchmarks for claimed metrics like "100% CIS compliance" and "80% handshake reduction vs OpenVPN").

The highest-fit segment is Defense / FDE. The public-sector SOC internship (13 municipal government entities, ELK Stack) and the homelab tooling (Wazuh, Suricata, Falco, TheHive, MISP, n8n SOAR) overlap tightly with what Palantir and Anduril deploy into government customers. That overlap is not obvious from a resume scan — it became visible only when a persona agent with knowledge of FDE deployment patterns was given the full diagnostic brief.

The engineering substrate — 5,705 Rust LOC, SQLite FFI with correct unsafe lifecycle, AtomicU64 telemetry, feature-gated csr/hydrate/ssr/ssg/sqlite — was cited as above-average for a portfolio by all six personas. The gap is not raw capability. It is the absence of paid production ownership and the absence of verifiable artifacts for the claimed homelab work.

---

## The patch-and-verify loop

The Wave 4 patch agent worked through five P0 commits in sequence, each verified before proceeding to the next:

1. Root-cause the bundle: remove `data-wasm-opt="0"` override, trim `web-sys` feature surface
2. Fix `deny.toml` schema: rename `[source]` → `[sources]`, remove schema-invalid `unmaintained = "warn"` key
3. Correct the GitHub handle in `src/data/mod.rs:18` from `github.com/richardmussell` (old account, no push access) to `github.com/richmuscle` (active account)
4. Fix the `style.scss` missing `pages/contact` import
5. Document CSP `unsafe-inline` in a new ADR — Trunk 0.21 generates inline `<script>` blocks that cannot be replaced with external sources without ejecting from Trunk's asset pipeline; the constraint is real and the ADR records it as a known limitation rather than an ignored finding

Verification per commit: `cargo check --no-default-features --features ssr`, `cargo check --features hydrate --target wasm32-unknown-unknown`, `cargo check --target wasm32-unknown-unknown`, then `trunk build --release`. Any failure stops the commit sequence. The worktree approach means a verify failure leaves the main branch clean while the patch agent diagnoses the failure in isolation.

The CSP finding is worth dwelling on. The initial audit flagged `unsafe-inline` in `index.html` as a security P0. The patch agent investigated the remediation path: removing `unsafe-inline` requires either nonce-based or hash-based script allowlisting, which requires controlling the `<script>` tags. Trunk 0.21 generates its own inline scripts as part of the WASM bootstrap process. These are not developer-authored scripts — they are emitted by the build tool and their hashes change on every build. Nonce injection would require a build hook that Trunk does not expose. The pragmatic call is to document the constraint and track it against a Trunk upstream fix, rather than shipping a workaround that breaks with the next Trunk upgrade. That is the documented ADR. The finding was real; the fix it implies is not currently tractable.

---

## Lessons from the run

**Scope is the most important agent design parameter.** The six Wave 1 lanes produced better per-lens coverage than a single comprehensive audit agent would have, because each agent had one concern and enough context budget to go deep on it. The recon lane measured the bundle because that was its job and it had room to measure. A generalist would have noted "bundle may be large" and moved on.

**Parallelism wins when lanes do not overlap; serialize when they do.** Waves 1, 2, and 3 are fully parallel within each wave — no lane reads another lane's output while running. The synthesis step (Opus) is serial and fast because it consumes JSON, not raw source. The patch loop (Wave 4) is serial by design: each commit must verify before the next one applies. Mixing parallel execution into the patch loop would require a merge strategy for the git history, which is a complexity cost with no benefit.

**Simulated personas are surprisingly calibrated.** The six recruiter personas were given the same diagnostic brief and asked to evaluate independently. Their verdicts converged tightly on the same three disqualifiers and the same best-fit segment. This is not because the personas were programmed to agree — they were given different hiring contexts and different evaluation weights. Convergence on three independent signals (current role, cert absence, unverifiable projects) suggests those signals are genuinely strong, not artifacts of a single evaluator's bias.

**Verification gates per commit are non-negotiable for automated patches.** An agent that applies five commits without verifying intermediate states can produce a working final commit on top of a broken intermediate. The cargo check + trunk build loop caught zero failures this run — but the run before it, during an earlier phase, the patch agent's first attempt at `deny.toml` introduced a schema error that it caught at verify step 3 and corrected before committing. Without the gate, that schema error would have committed silently.

**The meta-play is legitimate.** Using an automated audit pipeline to find gaps in a portfolio, then publishing the documentation of that pipeline as a writeup in the same portfolio, is not self-referential cleverness for its own sake. The artifact demonstrates the thing it describes. A technical hiring manager reading this can see the pipeline design, the model tier decisions, the worktree isolation rationale, and the verification strategy — and evaluate those decisions as engineering work. The content of this writeup is the proof of the system described.

---

## What this is at a personal scale

Platform engineering work is largely about building systems that observe and correct other systems — monitoring pipelines, CI gates, drift detection loops, runbook automation. The 22-agent audit pipeline is that pattern applied to a portfolio rather than a production service. The mechanics are the same: define the observation surface, parallelize where isolation holds, serialize where dependencies exist, verify at each state transition, document the constraints you cannot fix.

The findings were real. The bundle was 24% over target because of a one-line config override that had accumulated silently. The resume had diverged from the PDF in ways that mattered to the people it was meant to persuade. Six independent evaluators found the same three gaps. A hand audit would have found some of this. A 22-agent parallel sweep found all of it in 30 minutes of wall-clock time.

The source for the portfolio is at [github.com/richmuscle/richmuscle.github.io](https://github.com/richmuscle/richmuscle.github.io). The CI pipeline that runs on every push to `revamp` is there. The audit agent schemas and DECISIONS.md trail are checked in. The worktree isolation, the JSON handoff format, the per-commit verify loop — all of it is in the commit history.

Building tools that observe your own systems is the platform engineering job. This is what that looks like at a personal scale.
