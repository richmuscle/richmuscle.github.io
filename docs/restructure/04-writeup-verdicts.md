# 04 — Writeup Verdict Table

All 17 writeups catalogued from `src/data/writeups.rs`. All JSON files
in `public/writeups/` are 1-line stubs — none contain real article
content. The verdict is based on title, summary, tags, and category
from the Rust index entries.

Owner decides final fate on each row.

---

## Verdict Framework

- **KEEP**: Technically specific, sysadmin voice or close, ties to a canonical project or standalone defensible
- **RETITLE**: Content is sound, only the title violates voice
- **RETIRE**: LLM-pretension title AND stub/shallow content AND voice-mismatched
- **DEMOTE**: Archival value, keep URL live, remove from /writing index

---

## Verdict Table

| # | Slug | Title | Content | Verdict | Proposed retitle | Evidence | Project tie |
|---|---|---|---|---|---|---|---|
| 1 | hardening-linux-municipal-environments | Hardening Linux for Municipal Environments | stub | **KEEP** | — | "CIS-aligned posture, idempotent enforcement" — direct, operational | security-baseline-audit |
| 2 | automating-nist-800-53-compliance-with-terraform | Automating NIST 800-53 Compliance with Terraform | stub | **KEEP** | — | "deterministic Terraform module patterns with state safety" — operational IaC | security-baseline-audit |
| 3 | zero-trust-moving-beyond-bastion-hosts | Zero-Trust: Moving Beyond Bastion Hosts | stub | **KEEP** | — | "replace shared bastion trust with identity-governed connectivity" — operational | identity-access-lifecycle |
| 4 | siem-alert-hygiene-reducing-noise-in-the-soc | SIEM Alert Hygiene: Reducing Noise in the SOC | stub | **KEEP** | — | "Reduce SOC alert fatigue using enrichment" — direct SOC ops | observability-operational-intelligence |
| 5 | kubernetes-controller-reconciliation-deep-dive | Why Your Kubernetes Controller Is Lying to You | stub | **DEMOTE** | — | stub — no content to quote | standalone (K8s adjacent, not core sysadmin) |
| 6 | otel-ebpf-tracing-without-instrumentation | Distributed Tracing Without Touching Your App Code | stub | **DEMOTE** | — | stub — no content to quote | standalone (eBPF adjacent, not core sysadmin) |
| 7 | rust-wasm-edge-runtime-internals | Building a Zero-Copy Wasm Edge Runtime in Rust | stub | **RETIRE** | — | stub — no content to quote; Rust systems dev, not sysadmin | none |
| 8 | ebpf-from-zero-to-prod | eBPF From Zero to Production | stub | **DEMOTE** | — | stub — no content to quote | standalone (niche but defensible as technical depth) |
| 9 | the-orchestrator-of-intent-reflections-on-service-provisioning | The Orchestrator of Intent: Reflections on Service Provisioning | stub | **RETITLE** | "What Running a Telecom Taught Me About Provisioning at Scale" | "high-volume telecommunications operations function as a precursor to modern platform" — real experience, LLM title | standalone |
| 10 | the-architect-of-oceanic-visibility-soc-operations-at-universal-scale | The Architect of High-Fidelity Observability: SOC Operations at Universal Scale | stub | **RETITLE** | "Building a SOC Observability Stack That Reduces Noise" | "Transforming raw telemetry into...actionable intelligence through the ELK Stack" — SOC relevant, title is overwrought | observability-operational-intelligence |
| 11 | the-connectivity-fabric-mastering-the-bedrock-of-the-universal-control-plane | The Connectivity Fabric: Mastering the Bedrock of the Universal Control Plane | stub | **RETITLE** | "Cisco IOS Fundamentals: What Network Admins Actually Need" | "hardware-level configuration, VLSM, and port security" — real networking, title is overwrought | standalone |
| 12 | the-orchestrated-landscape-building-a-high-integrity-ecosystem | The Orchestrated Landscape: Building a High-Integrity Ecosystem | stub | **RETIRE** | — | "self-healing, self-governing infrastructure" — abstract philosophy, no operational grounding | none |
| 13 | the-mirror-universe-architecting-deterministic-enterprise-simulations | The Mirror Universe: Architecting Deterministic Enterprise Simulations | stub | **RETITLE** | "Testing AD Group Policies in a Sandboxed Lab" | "PowerShell and AutomatedLab to build a deterministic Windows Server 2022 simulation" — real AD/GPO, title is misleading | endpoint-management-compliance |
| 14 | the-sustainable-architect-engineering-low-entropy-landscapes-for-the-50-year-lookout | The Sustainable Architect: Engineering Low-Entropy Landscapes for the 50-Year Lookout | stub | **RETIRE** | — | "treating technical debt as system entropy" — abstract, banned word in title | none |
| 15 | the-builders-ledger-orchestrating-technical-outcomes-through-project-governance | The Builder's Ledger: Orchestrating Technical Outcomes through Project Governance | stub | **RETIRE** | — | "Reconceptualizing Project Management as the Governance Layer" — abstract strategy, not sysadmin | none |
| 16 | universal-dialects-the-role-of-linux-and-shell-in-the-unified-control-plane | Universal Dialects: The Role of Linux and Shell in the Unified Control Plane | stub | **RETITLE** | "Linux and PowerShell: The Two Tools Every Sysadmin Needs" | "roles of Linux and PowerShell as the 'hammer and nails'" — real operational content, overwrought title | security-baseline-audit |
| 17 | the-architect-of-the-prismatic-apex-orchestrating-equilibrium-in-a-holographic-landscape | The Architect of the Prismatic Apex: Orchestrating Equilibrium in a Holographic Landscape | stub | **RETIRE** | — | "philosophical analysis of systems design as the convergence of opposing forces" — metaphysical, no operational content | none |

---

## Summary Counts

| Verdict | Count | Slugs |
|---|---|---|
| KEEP | 4 | hardening-linux, nist-terraform, zero-trust-bastion, siem-alert |
| RETITLE | 5 | orchestrator-of-intent, oceanic-visibility, connectivity-fabric, mirror-universe, universal-dialects |
| DEMOTE | 3 | k8s-controller, otel-ebpf, ebpf-zero-to-prod |
| RETIRE | 5 | rust-wasm-edge, orchestrated-landscape, sustainable-architect, builders-ledger, prismatic-apex |

**Net result:** 9 writeups visible on /writing (4 keep + 5 retitled).
3 writeups accessible by direct URL but not listed (demoted).
5 writeups removed entirely (retired).

---

## Paired PDF verdicts

| PDF | Paired writeup | Writeup verdict | PDF verdict |
|---|---|---|---|
| resume.pdf | — | — | **KEEP** |
| platform-architecture-blueprint.pdf | the-architect-of-the-prismatic-apex | RETIRE | **RETIRE** |
| orchestrated-landscape.pdf | the-orchestrated-landscape | RETIRE | **RETIRE** |
| sustainable-architect.pdf | the-sustainable-architect | RETIRE | **RETIRE** |
| builders-ledger.pdf | the-builders-ledger | RETIRE | **RETIRE** |
| universal-dialects.pdf | universal-dialects | RETITLE | **KEEP** (follows retitled writeup) |
