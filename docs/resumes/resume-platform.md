# Resume Draft — Platform Engineer / Infrastructure Engineer / Junior SRE Variant
Target tier: Junior-to-Mid Platform Eng / Infra Eng
Target comp: $90,000–$120,000
Best market fit per persona audit: Defense FDE (Palantir/Anduril class), Rust-adjacent startups, Cyber-focused Series A/B
Variant generated: 2026-04-15

---

## Professional Summary

Systems-curious platform engineer shipping Rust/WASM with managed unsafe FFI; SOC-trained across 13 government entities; pursuing K8s and cloud certification. Built a 5,705-LOC Rust + Leptos + WASM32 portfolio platform solo — SQLite C API wrapped in safe Rust, feature-gated across four compilation targets, self-authored CI/CD pipeline. Homelab spans a 12-tool detection-and-response stack on bare metal with Terraform-provisioned GCP and a WireGuard zero-trust fabric. Grounded in incident triage, log pipeline ownership, and Linux operations with a degree emphasis in cybersecurity.

---

## Core Competencies

- Rust + WASM32 — systems programming, unsafe FFI lifecycle, feature-gated compilation
- Feature-gated build systems — csr/hydrate/ssr/ssg/sqlite targets, pinned toolchain, wasm-opt pipeline
- GitHub Actions CI/CD — multi-target matrix, cargo clippy/fmt/deny/audit, artifact verification
- Terraform / IaC — GCP landing zone, CIS-aligned, remote state, policy validation
- Kubernetes — in progress (CKA target Q4 2026)
- Linux systems administration — RHEL/Ubuntu, idempotent automation, hardening, RBAC
- Observability — Prometheus, Grafana, ELK Stack, SLO/SLI alerting
- Zero-trust networking — WireGuard, AD identity integration, micro-segmentation, MSS clamping
- Incident response and SIEM — Wazuh, Suricata, Falco, KQL-based detection, alert triage
- Container operations — Docker, bare-metal service orchestration

---

## Professional Experience

### Self-directed Platform Engineering / Homelab — Sep 2025–Present

- **Rust + Leptos + WASM32 static platform, 5,705 LOC, shipped to GitHub Pages.** Core depth: SQLite FFI wrapping the sqlite3 C API — `CString::new()` error-propagated on every boundary, `sqlite3_finalize()` called on both success and error paths, `db_handle()` null-checked before every use. AtomicU64 telemetry counters (`Relaxed` ordering) expose per-operation microsecond latency. Pure-Rust fallback path mirrors the SQLite relevance-scoring weights exactly — search is never broken, only slower.
- **Self-authored CI/CD pipeline:** 4-target `cargo check` matrix (CSR/hydrate/SSR/SSG); `cargo test`, `cargo clippy`, `cargo fmt --check`, `cargo deny check`, `cargo audit` gates; `rust-toolchain.toml` pins the exact compiler; `wasm-opt -Oz` post-build pass produces a 1.38 MB WASM bundle; `just` recipe book for reproducible developer commands.
- **12-tool SOC homelab on bare metal:** Wazuh (HIDS/SIEM), Suricata (IDS), Falco (runtime syscall), TheHive (case management), MISP (threat intel), n8n (SOAR automation) — end-to-end detection, triage, and response loop without managed cloud.
- **Terraform GCP landing zone:** CIS-aligned, GCS remote state with locking, micro-segmented VPC peering, Cloud NAT with no public-facing listeners, policy validation in CI.
- **Zero-trust administrative fabric:** WireGuard mesh with AD identity integration, MSS clamping on cross-cloud boundaries, no public ingress listeners — `Verify Explicitly` and `Least Privilege` enforced at the network layer.
- **In-progress certifications:** Google Cloud Associate Cloud Engineer (target Q3 2026); Certified Kubernetes Administrator (target Q4 2026).

*Concurrent: Product Brand Ambassador, Club Demonstration Services — income continuity, non-technical.*

---

### Account Management & Retention Specialist — Cox Communications — Oct 2024–Apr 2025

- Navigated ICOMS and Salesforce enterprise platforms for complex multi-service account resolution and technical billing escalations across a Tier 1 ISP.
- Executed structured retention workflows under strict performance targets; exposure to large-scale CRM toolchains and enterprise escalation runbooks.

---

### Student SOC Analyst (Internship) — PISCES SOC — Mar 2024–Jun 2024

Academic SOC rotation — exposure to production SOC operations, not operator tenure.

- Observed analysts triaging alerts across 13 municipal network feeds on the ELK stack during shift rotations.
- Built visualizations in Kibana as the rotation's hands-on deliverable; worked the ticketing system alongside the analyst shift, tracking how cases moved from alert to resolution.
- Did not author detections, run shifts, or produce stakeholder briefs. What the rotation taught was how a SOC functions in practice at regional government scale.

---

## Certifications & Education

| | |
|---|---|
| CWU BS IT & Administrative Management, Cybersecurity | Jun 2024, GPA 3.018 |
| Cisco CCNA coursework | 2018–2019 (academic foundation) |
| GCP Associate Cloud Engineer | In Progress — target Q3 2026 |
| Certified Kubernetes Administrator (CKA) | In Progress — target Q4 2026 |

---

## Technical Projects & Depth

**Rust + Leptos + WASM32 Portfolio Platform** — Production-shipped.
SQLite FFI correctness: 12 unsafe blocks wrapping `sqlite3_open_v2`, `sqlite3_prepare_v2`, `sqlite3_bind_*`, `sqlite3_step`, `sqlite3_finalize`, `sqlite3_close`; correct finalize-on-error paths; `CString` boundary guards. Feature-gated architecture: `csr`/`hydrate`/`ssr`/`ssg`/`sqlite` — no SSR symbols leak into the WASM build. AtomicU64 telemetry with `Relaxed` ordering. Per-section `ErrorBoundary` via `thiserror` `AppError` (`Fetch`/`Parse`/`Logic`). `GlobalAppState` single `provide_context` eliminates type-collision risk across reactive signals.

**Terraform GCP Landing Zone** — CIS-aligned, multi-environment, GCS remote state, micro-segmented VPCs, no public-facing entry points. NIST 800-53 controls encoded in IaC.

**Zero-Trust Administrative Fabric** — WireGuard + Active Directory, NIST 800-207 `Verify Explicitly` and `Least Privilege`, MSS-clamped cross-cloud boundaries, out-of-band peer authorization.

**Performance Telemetry Pipeline** — Prometheus + Grafana, SLO/SLI alerting, Logstash security-context enrichment, dashboards mapped to operational impact.

**Technical Writing:** NIST 800-53 automation, zero-trust beyond the bastion, eBPF from zero to production, SIEM alert hygiene.

---

## Contact

Richard.Mussell@yahoo.com · github.com/richmuscle · linkedin.com/in/richard-mussell · Oklahoma City, OK
