# Stage 2.3G Complete — OG Image Regeneration to Linux SysAdmin Positioning

Single-commit stage. Regenerated `public/og-image.png` from updated `scripts/og-image-template.html`. Both files in one commit because they represent one logical change — the rendered image is derived from the template and shipping one without the other would break the artifact's coherence.

The previous OG image (last modified 2026-03-03) predated the entire Stage 2.x identity sweep. It rendered "Systems Engineer" with a fictional tech tagline ("Rust · WebAssembly · Kubernetes · eBPF") and five fabricated project lines (Rust-to-Wasm Edge Runtime, Kubernetes Custom Controller, eBPF Kernel Monitor, OTel Tracing Pipeline, Bare-Metal IaC Provisioner) — none of which correspond to any real work in the portfolio or the floorp monorepo. This stage replaces all of it with content calibrated against the about-page anchor.

## Commit

| Commit | Scope | Change |
|---|---|---|
| `21490f0` | `scripts/og-image-template.html`, `public/og-image.png` | Template content + CSS rewrite; rendered PNG regenerated at 1200×630; 13 insertions, 17 deletions |

## Why this surface matters disproportionately

The OG image is the highest-impression single artifact in the portfolio. Every social share, every link unfurl in Slack/Discord/email, every recruiter scrolling LinkedIn previews — all see this image once, and most never click through. A stale OG image with fabricated content was the loudest unaddressed honesty failure on the public surface, and it had been live for six weeks.

The audit (2026-04-20, composite 6.2/10) didn't list OG image regeneration as a P0, but it surfaced through the identity sweep as the single most-public artifact still carrying pre-2.1 fictional positioning. The cost-to-impression ratio justified treating it with the same calibration discipline as the about-page anchor, not as a hygiene-tier item.

## Calibration approach

Three drafts were considered before locking content. The selected draft (D) emerged from staff-engineer review of an earlier draft (C) that had three problems:

1. **Promised content the public site doesn't deliver.** Earlier draft included "hardening a bare-metal SOC stack" — true work, but lives in floorp's `soc-stack/` submodule which isn't on the public portfolio yet (per owner's "land jobs first, port the rest later" stance). The OG image must not promise what the site can't immediately back when clicked through.

2. **Tagline included "Observability."** Substantive observability work is in floorp (Prometheus/Grafana, threat-intel-aggregator integration), not yet on the public portfolio. Same overclaim risk.

3. **Geographic anchor dropped "open to remote."** The about page says "Oklahoma City, open to remote." The "open to remote" half is load-bearing — without it, location-only reads as accidentally limiting rather than situational context.

Draft D resolved all three:

- **Title:** "Linux Systems Administrator" — direct from about-page anchor, no hedge.
- **Tagline:** "Operations · Infrastructure as Code · Honest about scope" — "Operations" matches anchor, "Infrastructure as Code" maps to clickable Terraform work in resume variants and the substantive `terraform-landing-zone` floorp submodule, "Honest about scope" names the discipline the entire 2.3 series has been building toward.
- **Right column:** "Documenting how I operate, not what title I'm claiming." + "Oklahoma City · open to remote" — direct paraphrase of the about-page line plus both halves of the location anchor.

## "Honest about scope" as tagline element

This is unconventional — OG taglines conventionally list tech tags. The choice was made deliberately and the risk was named before drafting:

- **Read A (the bet):** A staff-level reader sees "Honest about scope" and infers calibration discipline. That's the rare positive signal — most candidates' OGs are marketing voice; this one is engineering voice. The 2.3 series produced an entire site where claims and evidence match. Naming it on the highest-impression surface signals the differentiator is intentional, not accidental.
- **Read B (the risk):** A non-technical screener parses honesty-as-tagline as overcompensation or weakness.

Read A wins for the audience that matters (staff-level readers who decide whether to advance a candidate). Read B is a real risk for non-technical screening but a smaller cost than continuing to ship fabricated tech tags. An alternative tagline ("Lab-scale, calibrated") was named as Read-B-safer fallback during scope review; owner approved Draft D as drafted.

The tagline is descriptive, not aspirational — the about page, resume variants, project case studies (after today's identity reframing), and writeups all demonstrate the same calibration. The OG names what the site already does.

## What changed

### Template content (HTML)

Before:
- Title: "Systems Engineer"
- Tagline: "Rust · WebAssembly · Kubernetes · eBPF"
- Right column: 5 project lines with colored dots

After:
- Title: "Linux Systems Administrator"
- Tagline: "Operations · Infrastructure as Code · Honest about scope"
- Right column: two text blocks (`.right-text` + `.right-meta`)

### Template CSS

Before: `.project-line` (font-size 14, flex layout with gap) + `.project-dot` (10×10 colored circle).

After: `.right-text` (font-size 18, line-height 1.5, color `#e2e8f0`) + `.right-meta` (font-size 14, color `#94a3b8`, letter-spacing 1px).

The `.right` flex container's existing properties (justify-content: center, gap: 10px) produced the correct vertical layout for two stacked text blocks without further changes.

### Rendered PNG

`public/og-image.png` regenerated at 1200×630 (or 2x retina 2400×1260, browser-dependent). Confirmed present in `dist/og-image.png` after `trunk build --release` per the existing copy-file directive at index.html:87.

## Verification

- cargo fmt --check: clean
- cargo clippy --target wasm32-unknown-unknown -- -D warnings: clean
- cargo check --target wasm32-unknown-unknown: green
- cargo check --features hydrate --target wasm32-unknown-unknown: green
- cargo check --no-default-features --features ssr: green
- cargo test --no-default-features --features ssr: 15/15 passing
- trunk build --release: success
- dist/og-image.png present, current build timestamp confirmed

Post-apply greps confirmed:
- "Linux Systems Administrator", "Honest about scope", "Documenting how I operate", "Oklahoma City" all present in template
- Zero residual hits on "Systems Engineer", "Rust-to-Wasm Edge Runtime", "Kubernetes Custom Controller", "eBPF Kernel Monitor", "OTel Tracing Pipeline", "Bare-Metal IaC Provisioner", `.project-line`, `.project-dot`

The HTML head meta tags (`og:image`, `og:image:width=1200`, `og:image:height=630`) at index.html:22-24 unchanged — they reference the filename, not the image content, so the regeneration didn't require head-tag edits.

## Cross-surface consistency after this stage

| Surface | Title framing | Status |
|---|---|---|
| `about.rs` | "I'm a Linux systems administrator in Oklahoma City, open to remote" | Anchor (calibrated 2.3A) |
| OG image | "Linux Systems Administrator" + "Oklahoma City · open to remote" | **Aligned** (this stage) |
| `home.rs` | "Linux systems administrator" framing in hero copy | Consistent |
| Resume variants (devops/platform/sysadmin) | Multi-tier comp targets, Linux/sysadmin foundation | Pending — addressed by 2.3D-expanded positioning work in next session |
| Site title (HTML head) | "Richard Mussell — Linux Systems Administrator" or similar | Worth a future check, not in scope here |

Five public-facing surfaces now consistently anchor on "Linux Systems Administrator." The remaining tension lives in the resume variants' three-tier comp framing, which `PORTFOLIO_PLAN.md` (sysadmin-first) and the about page anchor both resolve toward sysadmin-primary. That's a 2.3D-expanded conversation for next session, not this stage's scope.

## Audit baseline

Staff-lens audit composite was 6.2/10 at `docs/audits/2026-04-20-staff-lens.md`. This stage closes the OG image regeneration item flagged in the original briefing. Contributes to `positioning_alignment` (weight 12) and `cross_surface_consistency` lenses by aligning the highest-impression surface to the established anchor.

## Discipline carrying forward

The OG image is a binary artifact, but the calibration discipline applied to text surfaces extends to it cleanly. The template is HTML — review the prose with the same staff-engineer lens as any other content stage. The render is mechanical. The screenshot is verification. The single-commit scope (template + image together) preserves the audit trail: anyone reviewing the commit sees the source of the change AND the resulting artifact in one place, no need to reconstruct what the rendered image looked like before/after.

Three-draft review on a high-impression surface earns its cost. Earlier draft (C) had problems — promising floorp content not on the public site, geographic anchor incomplete — that wouldn't have caused build failures but would have shipped a subtly miscalibrated image to the highest-impression surface in the portfolio. Reviewing drafts before locking content matters most when the artifact is hardest to see problems in. A staff reader scanning a 1200×630 PNG won't see "the SOC stack isn't actually on this site yet" — but they will form an inaccurate mental model, and the calibration cost compounds.

Naming an unconventional choice as a deliberate bet is itself the discipline. "Honest about scope" as a tagline element is non-standard. The choice was made, the read-A and read-B framings were named explicitly, the audience prioritization (staff-level over screener-level) was the deciding factor. Future readers of this closeout — including future audits — see the reasoning, not just the outcome. That's the difference between "we shipped something unusual" and "we shipped something unusual on purpose, and here's why."

Stage names sometimes carry information their numbers don't. 2.3G was numbered as a routine OG image regen in the original briefing, but the actual stage was a calibration of the most-public surface in the entire portfolio against the most-anchored sentence in the entire about page. The number understated the surface; the closeout corrects the record.
