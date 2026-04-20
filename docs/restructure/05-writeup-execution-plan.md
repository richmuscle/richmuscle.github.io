# 05 — Writeup Execution Plan (Stage 2.2A)

Produced: 2026-04-20. All 17 writeup JSON files read in full.

**Correction:** Previous assessments (04-writeup-verdicts.md, 04-content-audit.md) described writeup JSON files as "1-line stubs." This was wrong. The files are minified JSON (single line), but the `content` field contains substantial HTML — 380 to 2,800 words per piece. Every verdict below is based on the actual body content.

---

## Section 1 — Content Extraction

| # | Slug | Words | Class | Summary |
|---|---|---|---|---|
| 1 | hardening-linux-municipal-environments | ~550 | FULL | Fleet-ready Linux hardening for municipal environments: CIS-aligned posture, idempotent enforcement, SOC-ready audit evidence. Focuses on check-then-apply convergence and drift elimination. |
| 2 | automating-nist-800-53-compliance-with-terraform | ~460 | FULL | Encoding NIST 800-53 control intent into Terraform module patterns with remote state locking, governance-grade outputs, and compliance reconciliation loops. |
| 3 | zero-trust-moving-beyond-bastion-hosts | ~380 | FULL | Identity-governed connectivity replacing bastion hosts: out-of-band peer authorization, MSS clamping for tunnel stability, instant revocation via directory policy gates. |
| 4 | siem-alert-hygiene-reducing-noise-in-the-soc | ~380 | FULL | Reducing SOC alert fatigue through enrichment-first detection, cardinality-aware signal design, and SLO/MTTR-aligned dispatch logic. |
| 5 | kubernetes-controller-reconciliation-deep-dive | ~2,800 | LONG | Deep analysis of K8s controller-runtime: informer cache staleness, 409 optimistic locking, finalizer deletion races, level-triggered reconciliation, status conditions as contracts. |
| 6 | otel-ebpf-tracing-without-instrumentation | ~2,200 | LONG | eBPF syscall-layer distributed tracing without SDK instrumentation: span generation from kprobes, clock skew detection, bounded MPSC back-pressure, W3C trace context propagation. |
| 7 | rust-wasm-edge-runtime-internals | ~2,000 | LONG | Wasm/host ABI boundary analysis: 64-slot ring buffer batching reducing overhead from 73% to 3%, AOT via Cranelift for cold-start, deterministic memory layout with bump allocator. |
| 8 | ebpf-from-zero-to-prod | ~2,000 | LONG | Production eBPF guide: verifier rejection modes, CO-RE kernel portability via BTF, ring buffer back-pressure, sched_setaffinity pinning achieving 12x latency improvement. |
| 9 | the-orchestrator-of-intent-... | ~1,100 | FULL | Reflective essay on Cox Communications CRM-to-ICOMS provisioning as a control plane. Draws parallels between telecom service activation and modern IDP declarative intent. |
| 10 | the-architect-of-oceanic-visibility-... | ~1,300 | FULL | PISCES SOC observability: ELK Stack log aggregation, KQL semantic search, temporal correlation for APT detection, MantisBT as ledger of truth for incident state tracking. |
| 11 | the-connectivity-fabric-... | ~1,200 | FULL | Foundational networking via Cisco IOS: VLSM subnetting, port security, OSI-model troubleshooting. Positions these as bedrock principles for infrastructure operations. |
| 12 | the-orchestrated-landscape-... | ~1,500 | LONG | Strategic manifesto: identity-as-code (ADIIG/Fortress), fabric controllers, vulnerability remediation (Richclock-Engine), telemetry (Sentry Suite). Names appear fictional; voice is heavily metaphorical. |
| 13 | the-mirror-universe-... | ~1,000 | FULL | Lab provisioning with PowerShell and AutomatedLab: deterministic Windows Server 2022 simulation, AD schema architecture, GPO governance, virtual network sovereignty for safe testing. |
| 14 | the-sustainable-architect-... | ~1,400 | FULL | Sustainable architecture manifesto: Validating Admission Policies as "technical EPA," Crossplane multi-cloud, 50-year design outlook privileging continuity. Speculative strategy, not operational experience. |
| 15 | the-builders-ledger-... | ~1,400 | FULL | Project governance as control plane: declarative planning, reconciliation loops, admission control applied to organizational initiatives. Management theory in systems vocabulary. |
| 16 | universal-dialects-... | ~1,400 | FULL | Bridging Windows/Linux hybrid operations: PowerShell-as-code, kernel-level determinism, unified control planes, configuration-as-code evolution from script-ops. |
| 17 | the-architect-of-the-prismatic-apex-... | ~1,500 | LONG | Grand architectural manifesto using geometric metaphors (silvered edge, golden spark, holographic convergence, pyramid of stability). Contains a NIST reference but is primarily philosophical. |

---

## Section 2 — Verdict Corrections

| # | Slug | Original | Corrected | Flipped? | Evidence | Proposed new title (if RETITLE) |
|---|---|---|---|---|---|---|
| 1 | hardening-linux-municipal-environments | KEEP | **KEEP** | No | "check-then-apply so repeated runs converge to the same state" | — |
| 2 | automating-nist-800-53-compliance-with-terraform | KEEP | **KEEP** | No | "remote state backends and state locking to ensure a single source of truth" | — |
| 3 | zero-trust-moving-beyond-bastion-hosts | KEEP | **KEEP** | No | "Disable the user in the directory and observe that access terminates" | — |
| 4 | siem-alert-hygiene-reducing-noise-in-the-soc | KEEP | **KEEP** | No | "Add security context before alerting so analysts receive actionable briefs" | — |
| 5 | kubernetes-controller-reconciliation-deep-dive | DEMOTE | **DEMOTE** | No | "Never read from the informer cache immediately after a write" — deep K8s, not core sysadmin | — |
| 6 | otel-ebpf-tracing-without-instrumentation | DEMOTE | **DEMOTE** | No | "kprobes operate below the language layer, at the syscall boundary" — adjacent observability | — |
| 7 | rust-wasm-edge-runtime-internals | RETIRE | **DEMOTE** | **Yes** | "64-slot ring buffer fits in a single cache line on ARM Cortex-A55" — 2,000 words of real systems depth; not sysadmin but earns archival status | — |
| 8 | ebpf-from-zero-to-prod | KEEP | **KEEP** | No | "verifier performs static analysis...checking for unbounded loops, OOB accesses" — kernel-level production guide | — |
| 9 | the-orchestrator-of-intent-... | RETITLE | **RETITLE** | No | "By hitting 'Submit,' I was defining a 'Desired State'" — real telecom ops experience | "What Running a Telecom Taught Me About Service Provisioning" |
| 10 | the-architect-of-oceanic-visibility-... | RETITLE | **RETITLE** | No | "correlating minor IDS alerts with outbound spikes" — real PISCES SOC work | "Building SOC Observability at PISCES: ELK, KQL, and Threat Correlation" |
| 11 | the-connectivity-fabric-... | RETITLE | **RETITLE** | No | "mastering the mathematical rigor of VLSM" — real networking fundamentals | "Cisco IOS Fundamentals: Subnetting, Port Security, and OSI Troubleshooting" |
| 12 | the-orchestrated-landscape-... | RETIRE | **RETIRE** | No | "ADIIG/Fortress," "Richclock-Engine," "Sentry Suite" — tool names appear fictional; manifesto voice | — |
| 13 | the-mirror-universe-... | RETITLE | **RETITLE** | No | "automating OU distribution (IT, HR, Sales) and correlating them with Security Groups" — real AD lab work | "Building a Windows Server Lab with PowerShell and AutomatedLab" |
| 14 | the-sustainable-architect-... | RETIRE | **RETIRE** | No | "Validating Admission Policies act as a technical EPA" — real tool but speculative context | — |
| 15 | the-builders-ledger-... | RETIRE | **RETIRE** | No | "Reconciliation Loop applies controller logic to organizational systems" — management theory | — |
| 16 | universal-dialects-... | RETITLE | **RETITLE** | No | "kernel-level behavior is deterministic, upper layers can innovate" — real hybrid-ops perspective | "Linux and PowerShell: Bridging Hybrid Infrastructure" |
| 17 | the-architect-of-the-prismatic-apex-... | RETIRE | **RETIRE** | No | "Silvered Edge of Form," "Golden Spark," "Holographic Convergence" — metaphysical, beyond repair | — |

### Delta from original verdict table

| Metric | Original (04-content-audit.md) | Corrected | Change |
|---|---|---|---|
| KEEP | 5 | 5 | — |
| RETITLE | 5 | 5 | — |
| DEMOTE | 2 | 3 | +1 |
| RETIRE | 5 | 4 | -1 |

**One flip:** `rust-wasm-edge-runtime-internals` moved from RETIRE to DEMOTE. Reason: 2,000 words of genuine systems analysis (ABI boundary overhead, ring buffer cache-line optimization, AOT compilation) earns deep-link preservation. The content is not sysadmin-voiced but demonstrates real technical depth that a reader following a direct link would value.

---

## Section 3 — Execution Plan

### Batch 1 — RETIRE (4 writeups)

**Writeups to remove from site:**
| Slug | Title | Words | Reason |
|---|---|---|---|
| the-orchestrated-landscape-... | The Orchestrated Landscape: Building a High-Integrity Ecosystem | 1,500 | Fictional tool names (ADIIG, Richclock-Engine), manifesto voice |
| the-sustainable-architect-... | The Sustainable Architect: Engineering Low-Entropy Landscapes... | 1,400 | Speculative strategy, banned word in title |
| the-builders-ledger-... | The Builder's Ledger: Orchestrating Technical Outcomes... | 1,400 | Management theory in systems vocabulary, banned word in title |
| the-architect-of-the-prismatic-apex-... | The Architect of the Prismatic Apex: Orchestrating Equilibrium... | 1,500 | Metaphysical philosophy, multiple banned words |

**Files to delete:**
- `public/writeups/the-orchestrated-landscape-building-a-high-integrity-ecosystem.json`
- `public/writeups/the-sustainable-architect-engineering-low-entropy-landscapes-for-the-50-year-lookout.json`
- `public/writeups/the-builders-ledger-orchestrating-technical-outcomes-through-project-governance.json`
- `public/writeups/the-architect-of-the-prismatic-apex-orchestrating-equilibrium-in-a-holographic-landscape.json`

**Registry entries to remove from `src/data/writeups.rs`:**
- 4 entries from `init_writeups_index()`

**Writing page core_order to update:**
- `src/pages/writing.rs:15-25` — remove the 4 retired slugs from the ordering array

**Sitemap entries to remove:**
- `public/sitemap.xml` — 4 writeup URLs

**Paired PDFs to delete:**
- `public/pdfs/orchestrated-landscape.pdf`
- `public/pdfs/sustainable-architect.pdf`
- `public/pdfs/builders-ledger.pdf`
- `public/pdfs/platform-architecture-blueprint.pdf` (paired with prismatic-apex)

**Redirects:** No redirects needed. The retired writeups are not linked from any project page or from the home page. The `/writing` index listing is the only surface, and it will stop listing them after registry removal. Direct URL hits will 404, which is acceptable for content that should not exist.

**Commit message draft:**
```
refactor(writeups): retire 4 voice-mismatched manifesto pieces

Remove the-orchestrated-landscape, the-sustainable-architect,
the-builders-ledger, and the-architect-of-the-prismatic-apex.
Content was manifesto/philosophical, not operational sysadmin.
Paired PDFs removed. Sitemap updated.
```

### Batch 2 — RETITLE (5 writeups)

| Old slug | New slug | Old title | New title |
|---|---|---|---|
| the-orchestrator-of-intent-reflections-on-service-provisioning | telecom-service-provisioning-as-control-plane | The Orchestrator of Intent: Reflections on Service Provisioning | What Running a Telecom Taught Me About Service Provisioning |
| the-architect-of-oceanic-visibility-soc-operations-at-universal-scale | soc-observability-elk-kql-threat-correlation | The Architect of High-Fidelity Observability: SOC Operations at Universal Scale | Building SOC Observability at PISCES: ELK, KQL, and Threat Correlation |
| the-connectivity-fabric-mastering-the-bedrock-of-the-universal-control-plane | cisco-ios-subnetting-port-security-osi | The Connectivity Fabric: Mastering the Bedrock of the Universal Control Plane | Cisco IOS Fundamentals: Subnetting, Port Security, and OSI Troubleshooting |
| the-mirror-universe-architecting-deterministic-enterprise-simulations | windows-server-lab-powershell-automatedlab | The Mirror Universe: Architecting Deterministic Enterprise Simulations | Building a Windows Server Lab with PowerShell and AutomatedLab |
| universal-dialects-the-role-of-linux-and-shell-in-the-unified-control-plane | linux-powershell-bridging-hybrid-infrastructure | Universal Dialects: The Role of Linux and Shell in the Unified Control Plane | Linux and PowerShell: Bridging Hybrid Infrastructure |

**Files touched per retitle:**
1. Rename `public/writeups/<old-slug>.json` → `public/writeups/<new-slug>.json`
2. Update `slug` field inside the JSON
3. Update entry in `src/data/writeups.rs` — slug and title fields
4. Update `src/pages/writing.rs:15-25` — replace old slug in core_order array
5. Add entry to a writeup legacy redirect mechanism (analogous to project redirects)
6. Update `public/sitemap.xml` — replace old URLs with new

**Redirect handling:** Old writeup URLs will need redirects since they may be indexed by search engines or bookmarked. Propose adding a `LEGACY_WRITEUP_REDIRECTS` array in `src/data/writeups.rs` and redirect logic in `WriteupDetailPage` (same pattern as project redirects).

**Paired PDF rename:** `universal-dialects.pdf` stays (content is RETITLE, not slug-renamed at the PDF level).

**Commit message draft:**
```
refactor(writeups): retitle 5 pieces to sysadmin voice

Rename slugs and titles from essay-style to operational:
orchestrator-of-intent, oceanic-visibility, connectivity-fabric,
mirror-universe, universal-dialects. Legacy URL redirects added.
```

### Batch 3 — DEMOTE (3 writeups)

| Slug | Title | Reason for demotion |
|---|---|---|
| kubernetes-controller-reconciliation-deep-dive | Why Your Kubernetes Controller Is Lying to You | K8s dev, not core sysadmin; 2,800 words of real depth |
| otel-ebpf-tracing-without-instrumentation | Distributed Tracing Without Touching Your App Code | Adjacent observability, not core sysadmin; 2,200 words |
| rust-wasm-edge-runtime-internals | Building a Zero-Copy Wasm Edge Runtime in Rust | Rust systems dev, not sysadmin; 2,000 words of real depth |

**Implementation mechanism:** Add an `is_demoted: bool` field to `WriteUpIndex` in `src/data/writeups.rs`. The `/writing` index page filters out demoted entries from the main listing. The detail URL (`/writing/<slug>`) remains live — direct links work, but the piece doesn't appear in the browse index.

No file deletions. No slug changes. No sitemap changes (demoted writeups keep their URLs).

**Commit message draft:**
```
refactor(writeups): demote 3 non-sysadmin technical pieces

Add is_demoted flag. k8s-controller, otel-ebpf, rust-wasm-edge
remain accessible by URL but are removed from the /writing index.
Content has archival value but doesn't serve the sysadmin positioning.
```

### Batch 4 — KEEP (5 writeups, no action)

| Slug | Title | Project tie |
|---|---|---|
| hardening-linux-municipal-environments | Hardening Linux for Municipal Environments | security-baseline-audit |
| automating-nist-800-53-compliance-with-terraform | Automating NIST 800-53 Compliance with Terraform | security-baseline-audit |
| zero-trust-moving-beyond-bastion-hosts | Zero-Trust: Moving Beyond Bastion Hosts | identity-access-lifecycle |
| siem-alert-hygiene-reducing-noise-in-the-soc | SIEM Alert Hygiene: Reducing Noise in the SOC | observability-operational-intelligence |
| ebpf-from-zero-to-prod | eBPF From Zero to Production | standalone (Linux kernel depth) |

No changes. These 5 writeups are aligned with the sysadmin positioning, have direct operational titles, and tie to canonical projects or stand as defensible independent content.
