# Stage 2.3H Complete — Cert Honesty Sweep

Single-commit stage. Dropped aspirational cert claims ("Pursuing" / "In Progress" / "Target Q3/Q4 2026") from all surfaces and replaced with completed training only: Cisco Networking Academy CCNA-track coursework (four courses, 2018-2019, no exam taken). Closes audit P1-1 from `docs/audits/2026-04-20-staff-lens.md`.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `207166a` | src/data/certs.rs | Deleted — dead after CertificationsSection removal |
| `207166a` | src/data/mod.rs | Remove `pub mod certs` and `pub use certs::*`; update doc comment |
| `207166a` | src/pages/home.rs | Delete CertificationsSection component (lines 14-45) + call site (line 263); remove `get_certifications` import |
| `207166a` | src/pages/resume.rs | Replace "Certifications" section with "Training" (single coursework line); delete "Pursuing GCP ACE/CKA" bullet from Independent Lab Work section |
| `207166a` | src/pages/one_pager.rs | Replace "Currently Studying" section with "Training" (single coursework line) |

## What changed

### Deleted (4 items)

| Surface | What was removed |
|---|---|
| src/data/certs.rs | Entire file: `Certification` struct, `init_certifications()`, `CERTIFICATIONS` LazyLock, `get_certifications()`, 2 tests |
| home.rs CertificationsSection | Component rendering 3 cert rows (2 "In Progress" + 1 "Coursework") with status badges |
| resume.rs "Certifications" section | "GCP ACE — Pursuing (target exam: Q3 2026)" and "CKA — Pursuing (target exam: Q4 2026)" |
| resume.rs Independent Lab Work bullet | "Pursuing GCP Associate Cloud Engineer (target: Q3 2026) and Certified Kubernetes Administrator (target: Q4 2026)" |
| one_pager.rs "Currently Studying" section | GCP ACE (Target Q3 2026), CKA (Target Q4 2026), Cisco CCNA (2018-2019) |

### Reframed (2 items)

| Surface | Before | After |
|---|---|---|
| resume.rs | "Certifications" heading with 2 Pursuing bullets | "Training" heading with single Cisco coursework line |
| one_pager.rs | "Currently Studying" heading with 3 items | "Training" heading with single Cisco coursework line |

## Cross-surface consistency after this stage

| Surface | Cert/training content | Status |
|---|---|---|
| about.rs | Silent (dropped in 2.3A) | Anchor |
| home.rs | CertificationsSection deleted | **Aligned** (this stage) |
| resume.rs | "Training: Cisco Networking Academy — CCNA track, four courses completed (2018-2019). No exam taken." | **Aligned** (this stage) |
| one_pager.rs | "Training: Cisco Networking Academy — CCNA track, four courses completed (2018-2019). No exam taken." | **Aligned** (this stage) |
| certs.rs | Deleted | **Aligned** (this stage) |

Note: the post-apply grep caught a sixth location — `resume.rs:123` had a "Pursuing GCP ACE/CKA" bullet in the Independent Lab Work timeline entry that wasn't in the original five-file scope. The audit flagged four surfaces; the file contained five claim sites across four surfaces. Same lesson as Stage 2.3C: audit findings set direction; reading the source sets scope.

## Dead CSS follow-up

Fourteen cert-related CSS classes are now unused after CertificationsSection deletion:

`cert-section-header`, `cert-section-name`, `cert-section-line`, `cert-list`, `cert-row`, `cert-name`, `cert-issuer`, `cert-status`, `cert-status-completed`, `cert-status-pursuing`, `cert-status-studying`, `cert-status-interested`, `cert-status-default`, `certifications-section`

Also `one-pager-edu-list-certs` in one_pager.rs CSS. Flagged for a future CSS cleanup pass — not in 2.3H scope.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 16/16 passing (2 cert tests removed with certs.rs)
- trunk build --release: success
- Cert-status language grep (src/): zero hits
- Cert-status language grep (public/): one hit in sixteen-agent-orchestrated-audit.md (meta-writeup describing past audit finding — acceptable)
- certs.rs existence: deleted
- mod.rs certs references: zero hits

## Remaining stages

| Stage | Priority | Title | Status |
|---|---|---|---|
| 2.3D | P1 | InDevelopment project content voice | Open |
| 2.3F | P1 | KEEP-writeup voice pass | Open |
| 2.3K | P1 | Internal resume variants | Open |
| 2.3G | P3 | OG image regeneration | Open |
| 2.3E | P3 | Platform page build | Blocked on 2.3D |
| 2.3I | Low | Senior-tier residue sweep | Pending confirmation grep (audit indicated likely resolved) |

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. Resolved findings:
- P0-1 (Stage 2.3J): PISCES resume contradiction
- P0-2 + P0-3 (Stage 2.3C): unsupported metrics
- P1-1 (this stage): cert status inconsistency
- P1-4 (Stage 2.3L): dead author-line rewriter

Next audit targets: P1-2 (writeup voice pass — Stage 2.3F) and P1-3 (InDevelopment content voice — Stage 2.3D).

## Discipline carrying forward

Naming precision matters — "CCNA" vs "CCNA-track coursework, no exam taken" is the same class of honesty failure as overclaiming a rotation. A staff interviewer will ask "which CCNA?" and the answer needs to be "I completed the four-course academy track but didn't sit the exam." The training line now says exactly that.

Dead-code cleanup can come from stage scope even when the primary stage is content-honesty — removing CertificationsSection and certs.rs was the cleanest path to cross-surface alignment. Preserving them "just in case" would have left orphan data that a future session might accidentally re-expose.

Post-apply greps are part of the stage, not post-commit verification. The scoped recon for this stage identified four cert surfaces. The post-apply grep found a fifth claim site (resume.rs:123 Independent Lab Work bullet) that lived outside the scoped "Certifications" section. Stages with cross-surface alignment goals must grep the full codebase for claim-language AFTER the edit, not just verify the scoped edit succeeded. Catching contamination in the same commit is cheaper than catching it in the next audit.
