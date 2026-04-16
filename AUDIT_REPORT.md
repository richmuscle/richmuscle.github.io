# AUDIT REPORT — 2026-04-15 (second pass)

Six-agent parallel swarm + four-agent narrative sweep + six-agent recruiter-persona battery. Orchestrated on Opus 4.6 in Fast mode, 16 total subagents, isolated contexts, JSON-structured handoffs. Supersedes `AUDIT_REPORT_2026-04-15.md` from the morning run.

---

## § Verdict

| Dimension | Value |
|---|---|
| **Composite score** | **6.8 / 10** (−0.3 vs morning audit; bundle P0 + verified content gap weighted in) |
| **Hire signal** | **Defer for senior. Land-ready for Associate/Junior platform or Junior FDE.** |
| **Best-fit role (calibrated)** | **Associate Platform Engineer** ($85–115K) or **FDE I / Junior Deployment Engineer** at a defense-adjacent shop ($90–115K). Senior platform/infra is a 2-year conversation, not a 2026 one. |
| **Dominant strength** | Unusual-for-a-portfolio Rust/WASM systems-programming substrate: SQLite FFI (12 unsafe blocks, correct lifecycle), AtomicU64 telemetry, feature-gated architecture. 5,705 Rust LOC solo-built. |
| **Root-cause gap** | Paid-production absence + content mis-calibration. Current live-resume role is "Product Brand Ambassador" at Costco. 18-month gap from any technical employment. Every project is self-reported homelab with no repo links, no CI evidence, no benchmarks. About page carries 6 aspirational phrases ("building toward", "studying for") that self-declare junior tier. |

**One-sentence verdict:** The engineering substrate reads senior-capable but the career artifacts (role history, certs, verified output) read junior-present — six independent recruiter personas converged on "defer for senior, hireable at associate tier after 2-3 unblocking moves."

---

## § Five-lens Scores (rolled up from six-agent swarm)

| Lens | Score | Δ | Owner (lane) |
|---|---|---|---|
| Architecture & code organization | 8 | +1 | rust-quality confirms module split clean, GlobalAppState correct, prior P1 drop-order hazard **refuted** |
| CS depth signals | 7 | 0 | 12 unsafe blocks justified; AtomicU64 perf counters correct; pure-Rust fallback mirrors SQLite weights |
| Rust idioms & type discipline | 7 | 0 | AppError propagation clean, but unwrap clusters in layout.rs / nav.rs closures still panic-on-abort the WASM module |
| Deploy & build rigor | 6 | 0 | 3× cargo check + trunk build all pass; **bundle 24% over target**; CI still missing test/clippy/audit/deny |
| Content / marketing fit | 4 | 0 | Live resume PDF is now real (120K); **every other P0 unchanged** — domain collision actually widened when 404.html + robots.txt surfaces were checked |

Composite = 6.8 / 10 (mean of the five).

---

## § Delta vs 2026-04-15 morning audit (six commits since)

**What moved:**
- Six commits: pyramid/fibonacci/φ style refactor + hero-stat panel removal.
- Rust-quality score lifted from 7→8 after the prior P1 drop-order hazard in `src/db.rs` was investigated in depth and refuted: `_mem` stack-drop is safe because `sqlite3_open_v2` pins VFS reference to the connection, not to `MemVfsUtil`.
- Hero panel removal is clean — no orphan CSS selectors, no dead rules.

**What did NOT move (morning audit P0s, status today):**

| Morning P0 | Today's state |
|---|---|
| Live resume PDF | ✅ Live file is 120,159 B (real) — confirmed via curl |
| Public/static PDF pipeline ambiguity | ❌ `public/pdfs/resume.pdf` still the 5,420 B placeholder; two sources of truth remain |
| Canonical/OG/SITE_URL unification | ❌ `index.html` clean, but `404.html` JSON-LD points to `richardmussell.dev`, **both** `robots.txt` files point sitemap to `richardmussell.dev` (unprovisioned), `og-image-template.html` branded `richardmussell.dev`. Collision widened, not narrowed. |
| `cargo test` + `clippy` CI gate | ❌ Not added |

**What regressed:**
- Style refactor introduces mathematical overclaim: tokens.css comment says "golden-ratio steps: 11·13·16·26·42·68" but only 5 distinct values ship. 10 spacing tokens collapse to 7 distinct values. Geist @font-face declarations cover 400-700, but `base.css:321` and `:1152` still apply `font-weight: 800` → synthetic bold fallback on two hero-level typographic moments.
- `style.scss` drops the `pages/contact` import entirely.

**What is newly flagged (not in morning audit):**
- **WASM bundle 1,485,823 bytes / 618 KB gzipped = 24% over 500 KB target.** Morning audit noted bundle unoptimized but did not measure. Now measured.
- **9 unwrap() calls** in `src/components/nav.rs` and `src/components/layout.rs` closure callbacks — in `panic=abort` WASM builds these terminate the module.
- **7 `js_sys::eval()` clipboard call sites** despite `Clipboard` feature already declared in `web-sys` features.
- **palette.rs hot path**: `index.get_value()` clones entire `Vec<PaletteItem>` per keypress (~88 String allocations per keystroke).
- **`deny.toml` schema-broken** — `cargo deny check` fails to parse. Supply-chain enforcement is theater.
- **CSP `unsafe-inline`** in index.html + **4 `inner_html` sites without sanitizer** in writing.rs / project.rs.
- **GitHub handle points to wrong account**: `src/data/mod.rs:18` links `github.com/richardmussell` — the old account with no push access. Live deploy is from `github.com/richmuscle`. Any visitor clicking "GitHub" lands on a stale/empty profile.
- **Resume surface divergence**: on-site resume page suppresses the two in-progress certs (GCP ACE, CKA) that the PDF discloses. On-site lists 22 skills; PDF lists 34. The in-progress certs hidden on-site is the highest-impact self-own in the content layer.

---

## § P0 Actions (block senior outcomes — even more importantly, block associate-tier outcomes)

### Engineering P0s (safe to auto-apply after test gate)

1. **Collapse PDF pipeline to one source** (`public/pdfs/resume.pdf` ← replace 5,420 B placeholder with 120,159 B real; delete `static/pdfs/`; remove the `cp` workaround from `deploy.yml`; add `test $(stat -c%s dist/pdfs/resume.pdf) -gt 100000 || exit 1`).
2. **Add `cargo test` + `cargo clippy -- -D warnings` + `cargo fmt --check` to `.github/workflows/ci.yml`** (10 existing tests currently not gated).
3. **Fix `deny.toml` schema**: remove `unmaintained = "warn"`, rename `[source]` → `[sources]`, regenerate if easier. Add `cargo deny check` + `cargo audit` to CI.
4. **Bundle reduction**: verify `wasm-opt -Oz` pass, audit `sqlite-wasm-rs` feature surface, target ≤400 KB gzipped. Current 618 KB is the single largest engineering P0.
5. **Remove CSP `unsafe-inline`**; nonce or hash the two inline `<script>` blocks in `index.html`.

### Content P0s (require human approval per CONTEXT.md — NOT auto-applied)

6. **Unify canonical domain to `https://richmuscle.github.io/`** across every machine-readable surface:
   - `404.html:78` JSON-LD `url` field (currently `richardmussell.dev`)
   - `robots.txt` (root) + `public/robots.txt` Sitemap directive (currently `richardmussell.dev`)
   - `public/og-image-template.html:92` (currently `richardmussell.dev`)
   - `public/sitemap.xml` (currently `richardmussell.github.io` — also wrong; change all entries)
   - Any remaining `richardmussell.dev` hardcodes in source.

7. **Correct the GitHub handle** in `src/data/mod.rs:18`: change `github.com/richardmussell` → `github.com/richmuscle` (the active account). Same fix needed in `index.html`, `404.html`. Unless the owner explicitly wants visitors to land on the old account — confirm before changing.

8. **Reframe current role on the site**: two options — (A) remove "Product Brand Ambassador / Club Demonstration Services" from the on-site resume, keep it on the PDF only; or (B) reframe the current-position block to lead with "Self-directed Platform Engineering / Homelab (Sept 2025–Present)" and link the 12-tool SOC homelab + GCP ACE/CKA study progress, with CDS disclosed beneath as "concurrent income source." Option B is more honest and still fixes the first-30-seconds recruiter signal.

9. **Surface the in-progress certs on the on-site resume**. The PDF discloses GCP ACE + CKA as "pursuing" — the on-site version hides them. Show them honestly with a concrete study plan or target exam date.

10. **Fix the About page stale CSR claim** (`src/pages/about.rs:27`): "This portfolio is a Rust + Leptos CSR application compiled to wasm32-unknown-unknown..." — update to "Rust + Leptos application compiled to wasm32-unknown-unknown, deployed in CSR mode; SSR/hydrate/SSG feature-gated for Phase 2." The repo HAS all four gates; the on-site copy pretends it's CSR-only.

11. **Publish at least one of the claimed labs as a public GitHub repo** with CI evidence:
    - Terraform GCP landing zone is the highest-leverage. Real `terraform plan` output, `tfsec` / `checkov` scan artifacts, a README with architecture diagram.
    - Every persona independently cited the unverified "100% CIS compliance" as a red flag — it flips from negative to positive the moment a scan-output file is linked.

---

## § P1 Actions (fix this week, unblock associate tier)

### Engineering
- Replace unwrap/expect guards in `src/components/layout.rs:25-31`, `src/components/nav.rs:172-206,240-241` with `let Some(win) = web_sys::window() else { return; }` — match the pattern `main.rs` / `telemetry.rs` / `palette.rs` already use.
- Replace 7 `js_sys::eval()` clipboard call sites with `navigator.clipboard.write_text()` via `web-sys` (Clipboard feature already in Cargo.toml). Eliminates eval + allocation hot-path + XSS surface.
- Remove dead `GlobalAppState.portfolio_category / portfolio_search / portfolio_index_tick` fields (declared, never read) — either wire to `home.rs` or delete.
- Palette hot-path: wrap `index` in `store_value(Arc<Vec<PaletteItem>>)`, pre-lowercase haystacks at build time. Current: ~88 String allocations per keystroke.
- Fix `BackToTop` closure leak (`nav.rs:180`): use the `RefCell<Option<Closure>>` + `on_cleanup` pattern from `layout.rs`.
- Sanitize 4 `inner_html` sites (`writing.rs:291`, `project.rs:139`, `:257`, `components/project.rs:145`) via `ammonia` or equivalent.
- Fix Geist weight-800 gap: either add `geist-800.woff2` or change `base.css:321` + `:1152` to `font-weight: 700`.
- Fix `style.scss` missing `pages/contact` import.
- Collapse duplicate `.pd-stat-bar` declarations in `cards.css` (lines 242, 317, 357).

### Content
- Replace the eight "The [Noun] of [Abstract]: [Metaphor]" writeup titles with technical slugs — or demote them below the four slug-titled technical writeups (`automating-nist-800-53`, `zero-trust-moving-beyond-bastion-hosts`, `ebpf-from-zero-to-prod`, `siem-alert-hygiene`). Sort order in both `writeups.rs` and `sitemap.xml`.
- Rewrite About page blockquote (`about.rs:77-80`) from "the range I am building toward" → "the range I operate in" with a specific shipped artifact.
- Rewrite About "How I Think About Systems" (`about.rs:88-96`) — tie each principle to a named project artifact, not generic philosophy.
- Quantify SOC internship bullets (`resume.rs:93-98`): alert volume, triage time, incident escalation rate.
- Align `one_pager.rs:153-155` cert list with `certs.rs` (currently lists RHCSA + "Professional Cloud Architect"; `certs.rs` lists ACE + CKA).
- Remove "enterprise database systems" from resume summary (`resume.rs:54-55`) — ICOMS/Salesforce is enterprise CRM/telco billing, not DB engineering.

---

## § P2 Actions (queued, do not block merges)

- 10+ items from individual-lane outputs. Full JSON appendix below. Highlights: collapse fibonacci/φ token duplication (10 tokens → 7 distinct values), replace skeleton `@media prefers-color-scheme` with `html.light` class selector, resolve `lru` RUSTSEC advisory post Leptos 0.7 upgrade, document SSR/CSR/hydrate trade-offs in ADR.

---

## § Recruiter-Persona Consensus (details in RECRUITER_FIT.md)

Six independent personas evaluated the candidate. **None advanced for senior.** The consensus table:

| Persona | Verdict | Phone screen pass | Best-fit |
|---|---|---|---|
| FAANG SRE | reject | 0.04 | no match |
| SaaS Platform | reject | 0.08 | Associate PE |
| Rust Shop (Oxide/Fermyon) | defer | 0.15 | Jr dev-tools |
| Fintech Infra | defer | 0.22 | Associate / Jr SecOps |
| Defense FDE (Palantir/Anduril) | **defer (highest fit)** | **0.28** | FDE I |
| Startup CTO (A/B) | defer | 0.22 | Jr Platform |

**Three items cited by ALL six personas as disqualifying or concerning:**
1. Current role: Product Brand Ambassador on live resume
2. Zero completed professional certs (CKA + GCP ACE in-progress, suppressed on-site)
3. Every lab project unverifiable (no repos, no CI, no benchmarks)

**Best segment fit:** Defense / FDE. The public-sector municipal SOC experience and homelab tooling (Wazuh/Suricata/Falco/MISP/n8n SOAR) overlap tightly with what Palantir/Anduril deploy into gov customers. Clearance-eligibility assumed (US citizen, Oklahoma City).

**Recommended target re-scope:** Stop applying to senior platform at FAANG/Stripe-class. Apply to:
- FDE I / Junior Deployment Engineer at Palantir, Anduril, Shield AI, Primer, Scale AI Defense
- Associate Platform / Junior SRE at Series A/B startups in gov-tech or cyber
- Junior Cloud Security / SecOps at GCP-primary fintech (once GCP ACE lands)

---

## § Appendix A — Wave 1 raw agent outputs

**Lane scores & deltas:**
- rust-quality: 8 (+1), FFI drop-order P1 refuted
- frontend-ux: 7 (0), φ/fib refactor math-broken in token duplication
- content-fit: 4 (0), 4 P0s unresolved since morning
- build-deploy: 8 (+2), all checks pass but bundle 24% over
- security: 7 (0), CSP unsafe-inline, 4 inner_html, deny.toml broken
- recon: raw — 5,705 Rust LOC, 5,780 CSS LOC, 20 commits/30d, resume.pdf hash mismatch public vs static

## § Appendix B — Wave 2 narrative sweep

- 4 surfaces scanned (bio / resume / projects / meta)
- **On-site resume lists 22 skills; PDF lists 34** — 12 tools hidden on-site (Entra ID, Intune, Autopilot, M365, VMware, Prometheus, Grafana, BitLocker, Conditional Access, MantisBT, Python, systemd)
- **Four domains** in active circulation (richmuscle.github.io, richardmussell.github.io, richardmussell.dev, + differing OG image URLs)
- **Zero repo links** on any of the four projects
- **Zero completed certs surfaced** anywhere; 2 in-progress only in PDF

## § Appendix C — Patch log

**No automated patches applied this run.** Per project CONTEXT.md ("Never ship content without human approval"), content P0s require owner approval per variant. Engineering P0s (bundle, CI gates, deny.toml, CSP) are scoped and ready to hand off to the engineering-patch-agent in Wave 4, pending owner green-light on the list in § P0 Actions.

---

*Generated by Opus 4.6 orchestration · 16 parallel subagents · 3 waves · 2026-04-15 afternoon*
