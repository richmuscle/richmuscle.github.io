# Stage 2.3A Complete — About Page Voice Rewrite + Positioning Alignment

Five commits rewrote the about page from third-person academic framing to first-person staff-lens sysadmin voice, calibrated against the security-baseline-audit case study reframe (Stage 2.2B). The page now reads as a working operator describing how they operate — scope stated honestly per section, tradeoffs named where decisions were made, lab vs. production distinguished everywhere a number appears. CLAUDE.md was updated to remove the "senior" target-tier declaration; the site no longer claims a tier.

## Commits

| Commit | Scope | Change |
|---|---|---|
| `3f78219` | CLAUDE.md | Remove senior target-level declaration; align with no-tier positioning |
| `8b3878b` | about: Who I Am | First-person rewrite, academic-rotation framing for PISCES, lab scope stated, cert language dropped |
| `03db81e` | about: Technical Trajectory | 8 cards rewritten — hands-on vs. studying vs. exposure stated per card, tradeoffs named |
| `2ebbd86` | about: How I Operate | Section retitled; 3 principle paragraphs with named tradeoffs; pull-quote removed |
| `583215e` | about: Studying fold | Standalone Adjacent Technologies dropped; Ansible/Vault/ArgoCD/Helm folded into Card 7 with "haven't shipped" framing |

## Voice calibration

Source: security-baseline-audit JSON (Problem, Approach reframed in Stage 2.2B). The about page now matches: first-person throughout, operational specificity over adjectives, tradeoffs named where a real decision was made, scope stated honestly (lab vs. production, hands-on vs. studying vs. exposure). Zero banned-word hits in final about page (verified via `grep -ni` pre-flight).

## Positioning shift

CLAUDE.md's "Target level: Senior" declaration removed. About page no longer declares target tier — opening sentence says "This site documents how I operate, not what title I'm claiming." Owner is open to senior, mid, or entry placements; rigor is the evidence. Staff-engineer lens on how work is documented remains unchanged.

## Verification

- 4/4 cargo check gates: green
- 17/17 tests: passing
- cargo fmt --check: clean (exit 0)
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean (exit 0)
- trunk build --release: success
- WASM bundle: 1.6 MB (unchanged from pre-2.3A baseline — content-only changes)

## Deferred items

### Stage 2.3G — OG image regeneration
`scripts/og-image-template.html` still says "Systems Engineer" and lists "Rust · WebAssembly · Kubernetes · eBPF". `public/og-image.png` predates the Stage 2.1 identity sweep. LinkedIn/Twitter card previews show stale identity.

### Stage 2.3H — Cert honesty sweep
Four surfaces still reference certs with "Pursuing" / "In Progress" / "Target Q3/Q4 2026": `src/data/certs.rs`, `src/pages/home.rs` CertificationsSection, `src/pages/resume.rs` Certifications, `src/pages/one_pager.rs` "Currently Studying". The about page dropped cert language entirely in Stage 2.3A. These surfaces now contradict the about page.

### Stage 2.3I — Senior-tier residue sweep
User-visible pages not yet scanned for "senior" self-declarations after the CLAUDE.md positioning change. Check: home.rs hero, resume.rs header/summary, one_pager.rs, contact.rs.

### Stage 2.3J — Resume PISCES framing alignment
`src/pages/resume.rs` still claims "Triaged ~50 alerts/day" and "Authored KQL detection logic" for the PISCES role. The about page corrected this to academic-rotation scope. Resume needs the same honesty pass.

### Stage 2.3K — Internal resume variants
`docs/resumes/resume-devops.md` contains "Engineered" at lines 42, 44. Internal but should follow banned-word discipline if sent to a recruiter.

### Dead CSS residue
`.about-pills-row` in `style/pages/about.css` is unused after Section 4 removal. Harmless.

### Stages 2.3B–2.3F (restated)
B: writing page intro rewrite. C: unsupported metrics resolution. D: InDevelopment project content voice. E: /platform page build. F: KEEP-writeup voice pass.

## Discipline carrying forward

Pre-flight grep discipline (corrected to `\borchestrat\w*` per owner feedback) caught issues before drafting. Section-by-section cadence held — no voice work batched, each section owner-approved before Rust was written. Evidence-backed pushback was load-bearing: verified TTFB claim against `telemetry.rs:39`, verified 87/92 against `security-baseline-audit.json:8`, verified studying-item absence across all project JSONs. No Co-Authored-By trailers. Zero banned-word hits in final about page.
