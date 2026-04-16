# RECRUITER-FIT MATRIX — 2026-04-15

Output of a six-agent recruiter-persona battery. Each agent was Sonnet-class, isolated context, given the same diagnostic brief (Wave 1 + Wave 2 synthesis) and asked to evaluate from their market segment's hiring bar.

This document answers the question the owner has avoided asking out loud: **which roles are realistic to target this cycle, and which are not.**

---

## § Executive verdict

**None of the six personas advance this candidate for senior.** The closest phone-screen probability at senior tier is 0.28 (Defense FDE); the lowest is 0.04 (FAANG SRE).

**Calibrated best-fit this cycle:**
- **Primary target:** Defense / FDE track (Palantir FDE, Anduril, Shield AI, Scale AI Defense, Primer). FDE I or Junior Deployment Engineer. $90–115 K. Probability ~0.28.
- **Secondary target:** Junior Platform / Associate SRE at Series A/B startup with gov-tech or cyber posture. $85–105 K. Probability ~0.22.
- **Tertiary target:** Associate Platform / Junior Security Engineer at GCP-primary fintech (post-GCP-ACE). $85–115 K. Probability ~0.22.

**Stretch (realistic at 12-month horizon, not today):**
- Senior Platform at a small startup that values full-stack self-builder signal heavily. Requires CKA + public-repo labs first.

**Unrealistic this cycle (don't burn cycles applying):**
- Any senior role at FAANG / Stripe-class / Oxide-class. Two-year path with structured unblocking.

---

## § Persona matrix

| Persona | Verdict | Rust / CS depth | Compliance / ops | 30-sec reaction summary | Phone screen | Onsite | Realistic best-fit | Salary band |
|---|---|---|---|---|---|---|---|---|
| FAANG SRE (Google/Meta/Amazon) | **reject** | — | — | "Reject in 20s. Costco current role. No production." | 0.04 | 0.00 | No match. L3 new-grad only after 12+mo infra employment. | no offer |
| SaaS Platform (Stripe/Cloudflare/Datadog) | **reject** | — | — | "Interesting craft work, no production ownership. Pile: no." | 0.08 | 0.02 | Associate Platform Eng (post-gap-close) | $75–95 K |
| Rust Shop (Fermyon/Oxide/Materialize) | **defer** | 5/10 fluency, 3/10 depth | — | "Motivated self-learner, not a systems eng. Portfolio-grade, not production-grade." | 0.15 | 0.04 | Jr developer-tools at a non-Oxide-tier shop | $65–85 K |
| Fintech Infra (JPM/Goldman/Jane Street) | **defer** | — | 3/10 readiness, 4/10 security | "Unverified CIS claim + broken deny.toml = compliance liability." | 0.22 | 0.08 | Associate Platform / Jr SecOps | $85–115 K |
| Defense FDE (Palantir/Anduril/Shield AI) | **defer (best fit)** | — | 7/10 scrappy, 5/10 customer | "Public-sector SOC + homelab tooling overlap tightly with gov-customer work." | **0.28** | **0.12** | **FDE I / Junior Deployment Engineer** | **$90–115 K** |
| Startup CTO (Series A/B) | **defer** | — | medium ship-rate | "Smart builder, zero runway. Can't put on-call week one." | 0.22 | 0.08 | Jr Platform / DevOps, 12-month path to mid | $85–105 K |

---

## § Green flags (cited across multiple personas)

1. **Rust + Leptos + WASM substrate is real, not tutorial** — 5,705 LOC, feature-gated architecture, SQLite FFI with managed unsafe. Cited by all 6 personas as above-average-for-a-portfolio.
2. **SQLite FFI lifecycle correctness** — `CString::new()` error-propagated, `sqlite3_finalize()` on both success/error paths, `db_handle()` checks `is_null()` before every use. Rust-shop persona cited this as "correct unsafe discipline."
3. **Feature-gated csr/hydrate/ssr/ssg/sqlite** — no SSR symbols leak into WASM build. All 6 personas acknowledged "systems-aware build strategy."
4. **12-tool SOC homelab on bare metal** — Wazuh, Suricata, Falco, TheHive, MISP, n8n SOAR. Defense FDE and Fintech personas both called out tooling overlap with their deployment patterns.
5. **PISCES SOC internship: 13 municipal government entities on ELK Stack** — public-sector regulated-environment experience. Defense FDE persona weighted this highest.
6. **Intellectual honesty on the About page** — aspirational phrases read as self-aware rather than deceptive. FAANG persona called it "a foundation to build on."

---

## § Red flags (cited by all 6 personas as disqualifying or concerning)

1. **Current role on live resume: Product Brand Ambassador at Club Demonstration Services (Costco)** — 6/6 personas cited this as the #1 first-30-seconds reject signal. Several used phrases like "hard stop," "disqualifying," "liability when presenting to customers."
2. **Zero completed professional certs** — CCNA coursework 2018-19 without certification completion (7 years elapsed). GCP ACE + CKA "in progress" disclosed in PDF but **suppressed on live resume**. Concealment read as worse than absence.
3. **All four projects unverifiable** — no public GitHub repos, no CI output, no benchmark artifacts. Specific claims each persona flagged as fabricated-looking:
   - "100% CIS Google Cloud Foundations Benchmark compliance" (Terraform GCP) — no Terraform plan, no tfsec/Checkov, no repo
   - "80% handshake reduction vs OpenVPN" (zero-trust) — no benchmark methodology, no packet capture
   - "MTTR minutes → seconds" (monitoring pipeline) — no baseline, no incident data
   - "100% toil reduction" (linux admin scripting) — no before/after timing
4. **18-month gap from any technical employment** (April 2025 Cox → April 2026 today, Costco since Sept 2025). 6/6 personas cited.
5. **Portfolio's own CI pipeline fails platform-candidate self-consistency** — 10 tests exist but not gated; no cargo clippy -D warnings; no cargo audit; deny.toml schema broken. Rust shop persona: "Eat your own cooking."

---

## § Secondary red flags

- Writing section leads with 8 metaphysical/philosophical titles ("The Architect of the Prismatic Apex: Orchestrating Equilibrium in a Holographic Landscape") before the 4 technical writeups — cited by 4/6 personas as audience-miscalibration signal.
- 3-domain canonical collision (richmuscle.github.io, richardmussell.github.io, richardmussell.dev) — cited by 5/6 as attention-to-operational-detail concern.
- About page 6 aspirational phrases ("building toward", "studying for", "actively building depth", "the range I am building toward", "Technologies I Want to Go Deeper With") — cited by all 6 as junior-self-labeling.
- WASM bundle 24% over 500 KB target — cited by Rust shop + Startup CTO personas as "cost-aware instinct not yet applied to own work."

---

## § Unblock priorities (ranked by persona consensus weight)

**Tier 1 — single largest leverage, cited by 6/6 personas:**

1. **Publish the claimed labs as public GitHub repos with CI evidence.** Terraform GCP is the highest-leverage: push the real Terraform code, wire GitHub Actions running `tfsec` / `checkov`, include scan output in README. Linux admin scripting + zero-trust + monitoring pipeline follow. Effort: ~40-60 hours. **Impact: flips 4 unverifiable claims into 4 inspectable artifacts — the single largest credibility multiplier available.**

2. **Close the current-role optics gap.** Two paths:
   - (A) Remove CDS from the on-site resume; disclose on PDF only.
   - (B) Reframe current position as "Self-directed Platform Engineering / Homelab (Sept 2025–Present)" with CDS beneath as "concurrent income source."
   - (C) Find a contract / part-time / volunteer technical role — even 10 hrs/week of paid infra work — to break the retail streak.
   
   Effort for A or B: 2 hours. Effort for C: 50-200 hours of applications.

**Tier 2 — cited by 4-5/6 personas:**

3. **Complete CKA** (choose CKA over GCP ACE — it's more operationally credible to platform hiring managers and signals Kubernetes production ownership). Effort: ~80-120 hours study + $395 exam. **Impact: first completed professional credential. Clears ATS for platform roles with cert filters.** Fintech persona separately valued GCP ACE; if targeting GCP-primary fintech, ACE is the closer-to-complete option.

4. **Surface the in-progress certs honestly on the live resume.** The PDF shows them; the on-site hides them. Concealment reads worse than honest disclosure. Effort: 30 minutes.

5. **Unify canonical domain to `richmuscle.github.io`** across `404.html`, both `robots.txt`, `og-image-template.html`, `sitemap.xml`, and verify `src/data/mod.rs` GitHub handle points at the active `github.com/richmuscle` account not the inactive `richardmussell` one. Effort: 1 hour.

6. **Fix portfolio's own CI hygiene** — add `cargo test`, `cargo clippy -- -D warnings`, `cargo audit`, `cargo deny check` gates. Fix `deny.toml` schema. Remove CSP `unsafe-inline`. Effort: ~4 hours. **Impact: removes the "doesn't hold self to production standards" flag that every technical reviewer will spot.**

**Tier 3 — cited by 2-3/6 personas:**

7. Rewrite About page to replace 6 aspirational phrases with specific shipped artifacts.
8. Reorder writing section — technical writeups above metaphysical titles.
9. WASM bundle reduction to ≤400 KB gzipped.
10. Quantify SOC internship bullets (alert volume, triage time, escalations).

---

## § 6-month roadmap (persona-consensus-derived)

**Month 1 (now → 2026-05-15):**
- Tier-1 unblocks: ship 4 lab repos with CI evidence (Terraform GCP first), reframe current role on live resume, unify canonical domain, fix portfolio CI, surface in-progress certs honestly.
- **Outcome:** phone-screen probability across all 6 personas roughly doubles. Defense FDE moves from 0.28 → ~0.50. Startup CTO + Fintech move from 0.22 → ~0.40.

**Month 2-3 (2026-05-15 → 2026-07-15):**
- Complete CKA, badge it publicly.
- Apply actively to FDE I roles at Palantir, Anduril, Shield AI. Apply to Junior Platform roles at A/B startups with gov-tech or cyber posture.
- **Outcome:** first realistic offers.

**Month 4-6 (2026-07-15 → 2026-10-15):**
- Employed in a paid infra/platform/FDE role.
- Begin CKS or GCP Professional Cloud Security Engineer study.
- Start replacing claimed metrics with instrument-backed numbers from production work.
- **Outcome:** by end of 2026 the resume reads "Junior Platform Engineer at [Company] — Rust/K8s/IaC, 6 months production" instead of "Product Brand Ambassador."

**Month 7-12 (2026-10-15 → 2027-04-15):**
- First promotion to mid-level with production ownership.
- Resume is now competitive for SaaS Platform mid-level roles (Stripe/Cloudflare class).

**Senior-tier at target companies is a 2027-2028 conversation.** This is the honest roadmap.

---

## § What this document does NOT claim

- The candidate is not unintelligent — the Rust/WASM/FFI work demonstrates genuine engineering curiosity and above-average ceiling. Six personas agreed on this.
- The gap is NOT one of raw capability — it is one of demonstrated paid production ownership and verifiable artifacts.
- The six personas are simulations; actual recruiter-to-recruiter variance is high. A single well-connected warm intro can override any of the numbers above.

---

*Generated by six Sonnet-class persona agents · parallel isolated contexts · 2026-04-15 afternoon*
