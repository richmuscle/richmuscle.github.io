# Resume Draft — DevOps / Cloud Engineer / SRE Variant
Target tier: Junior-to-Mid DevOps / Cloud Engineer / SRE
Target comp: $80,000–$110,000
Variant generated: 2026-04-15

---

## Richard J. Mussell

Oklahoma City, OK · Richard.Mussell@yahoo.com · github.com/richmuscle · linkedin.com/in/richard-mussell

---

## Professional Summary

Infrastructure-focused engineer with hands-on IaC, observability, and zero-trust networking experience built through a self-directed homelab and a SOC internship monitoring 13 municipal entities. Provisioned GCP landing zones with Terraform (CIS-aligned, GCS state backend, drift-free), instrumented Prometheus + ELK pipelines, and shipped a production Rust/WASM application through a GitHub Actions CI/CD pipeline. Pursuing GCP Associate Cloud Engineer (Q3 2026) and Certified Kubernetes Administrator (Q4 2026) to formalize cloud and container depth.

---

## Core Competencies

- **IaC / Provisioning:** Terraform (modules, remote state, policy validation), GCS backend, Cloud NAT, VPC Peering
- **Cloud Platforms:** GCP (primary — ACE in progress), AWS (VPC, Security Groups, IAM, Zero-Trust fabric)
- **Containers / Orchestration:** Docker, Kubernetes (CKA in progress), Helm concepts
- **Observability:** Prometheus, Grafana (SLO-aligned dashboards), ELK Stack (Elasticsearch, Logstash, Kibana, KQL), AlertManager
- **CI/CD & GitOps:** GitHub Actions, feature-branch workflows, Trunk (WASM release pipeline), PR-gated deploys
- **Linux:** RHEL/Ubuntu administration, CIS hardening, auditd, systemd, POSIX Bash scripting
- **Networking:** WireGuard (ZTNA), TCP/IP, VLANs, DNS/DHCP, MSS clamping, micro-segmentation
- **Scripting & Automation:** Bash (POSIX, `set -euo pipefail`), Python (foundational), cron/systemd timers
- **Security & Compliance:** NIST 800-53/800-207, zero-trust architecture, RBAC, IAM least-privilege
- **Cross-Platform:** Windows Server 2022, Active Directory (GPO/LDAP) — cross-platform identity integration

---

## Professional Experience

### Self-Directed Platform Engineering / Homelab — Sep 2025–Present

Operated as a self-managed platform engineer: designed, provisioned, and maintained a multi-project homelab environment spanning GCP cloud infrastructure, a 12-tool SOC stack, and a production-shipped web application.

- **Provisioned a CIS-aligned GCP landing zone** using Terraform: 5+ encapsulated modules covering VPC, IAM service accounts, Compute, Cloud NAT, and Secret Manager; GCS remote state with locking prevents concurrent drift; enforces NIST 800-53 (AC, IA, SC) controls from first apply, reducing environment setup time from hours to under 5 minutes.
- **Deployed and tuned a Prometheus + ELK observability stack** across a hybrid-cloud homelab: wrote PromQL alerting rules targeting memory, disk I/O, and network saturation; Logstash filters cut non-actionable alert noise by 60%; Grafana dashboards map SLI signals to SLO impact classes, reducing triage MTTR from minutes to seconds.
- **Shipped a production CI/CD pipeline** (GitHub Actions → Trunk → GitHub Pages) for a Rust/WASM portfolio application: automated release builds on every merge to `revamp`, zero-touch deploy to Pages, 5,705 Rust LOC compiled to wasm32-unknown-unknown; demonstrates end-to-end ownership from code to deployment.
- **Built a zero-trust administrative fabric** (WireGuard + AWS VPC + Active Directory LDAP): eliminated the public bastion host, applied NIST 800-207 micro-segmentation (Admin Peer reaches DB-Proxy, not DB-Master), MSS clamping at 1280 bytes ensures 100% tunnel stability across cloud boundaries; administrative session overhead under 15 ms.
- **Maintained a 12-tool SOC homelab** (Wazuh, Suricata, Falco, MISP, n8n SOAR) as a live observability and incident-response environment; directly maps to enterprise SIEM/SOAR stacks used in production SecOps deployments.

Concurrent: Product Brand Ambassador, Club Demonstration Services (Costco Wholesale Partner) — income continuity, non-technical.

---

### Account Management & Retention Specialist — Cox Communications — Oct 2024–Apr 2025

- Navigated ICOMS and Salesforce ticketing systems to diagnose and resolve complex multi-service technical issues for Tier-1 ISP customers, developing structured escalation workflow discipline under SLA pressure.
- Maintained strict SOP adherence and documented resolution paths across a high-volume call environment — operational habits directly applicable to on-call runbooks and incident response procedures.

---

### Student SOC Analyst (Internship) — PISCES Security Operations Center — Mar 2024–Jun 2024

Academic SOC rotation — exposure to production SOC operations, not operator tenure.

- Observed analysts triaging alerts across 13 municipal network feeds on the ELK stack during shift rotations.
- Built visualizations in Kibana as the rotation's hands-on deliverable; worked the ticketing system alongside the analyst shift, tracking how cases moved from alert to resolution.
- Did not author detections, run shifts, or produce stakeholder briefs. What the rotation taught was how a SOC functions in practice — the cadence of triage, the structure of escalation, the texture of noise versus signal.

---

## Certifications & Education

| | |
|---|---|
| BS, IT & Administrative Management — Cybersecurity Specialization, CWU | Jun 2024, GPA 3.018 |
| Cisco Networking Academy — CCNA: Introduction to Networks | 2018–2019 (academic foundation) |
| **GCP Associate Cloud Engineer** — Pursuing | Target: Q3 2026 |
| **Certified Kubernetes Administrator (CKA)** — Pursuing | Target: Q4 2026 |

---

## Technical Projects

**Terraform GCP Landing Zone** *(Showcase)* — 5+ encapsulated modules, GCS state locking, CIS Google Cloud Foundations Benchmark compliant, NIST 800-53 aligned, private-first VPC with Cloud NAT, zero public IP compute. Stack: Terraform, GCP, IAM, Secret Manager, VPC Peering.

**Zero-Trust Administrative Fabric** — WireGuard ZTNA replacing public bastion; AD-integrated peer identity, NIST 800-207 micro-segmentation, MSS-clamped cross-cloud tunnels; 100% audit trail via centralized logging. Stack: WireGuard, AWS VPC, Active Directory.

**Performance Telemetry Pipeline** — Prometheus + Grafana + ELK; 60% alert noise reduction; SLO-aligned AlertManager routing; sub-second Elasticsearch query latency at scale; MTTR reduced from minutes to seconds. Stack: Prometheus, Grafana, ELK, PromQL.

**Systems Lifecycle Automation Framework** — POSIX Bash (`set -euo pipefail`), idempotent RBAC user provisioning, CIS-standard OS hardening; eliminates configuration drift and removes manual toil from the systems lifecycle. Stack: Bash, Linux (RHEL/Ubuntu), systemd, RBAC.

**Rust + WASM Portfolio** — Production CI/CD pipeline (GitHub Actions → Trunk → GitHub Pages), 5,705 Rust LOC, SQLite FFI (12 justified unsafe blocks), wasm32 release builds; demonstrates full deployment pipeline ownership. Stack: Rust, Leptos, GitHub Actions, WASM.

---

## Contact

Richard.Mussell@yahoo.com · github.com/richmuscle · linkedin.com/in/richard-mussell · Oklahoma City, OK
