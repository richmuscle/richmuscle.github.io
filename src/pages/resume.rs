use crate::data::{EMAIL, GITHUB_URL, LINKEDIN_URL, PROFESSIONAL_TITLE};
use crate::utils::track;
use leptos::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn ResumePage() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);
    create_effect(move |_| {
        track("resume_view", "{}");
    });

    view! {
        <Title text=move || format!("Resume · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="Resume of Richard Mussell — Systems Administrator & DevOps Engineer with lab projects spanning IaC, Linux automation, observability, and zero-trust networking."/>
        <main id="main-content" class="min-h-screen pt-16 pb-24 bg-[var(--bg-base)]">
            <div class="resume-container max-w-3xl mx-auto px-6 py-12">

                <div class="resume-header flex items-center justify-between mb-12">
                    <a href="/" class="text-[13px] font-mono text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors">"← Back"</a>
                </div>

                <header class="mb-10 pb-8 border-b border-[var(--border-subtle)]">
                    <h1 class="text-4xl font-bold text-[var(--text-primary)] tracking-tight mb-1">"Richard J. Mussell"</h1>
                    <p class="text-[16px] text-[var(--text-secondary)] mb-3">"Systems Administrator & DevOps Engineer"</p>
                    <div style="white-space:nowrap;font-size:13px;font-family:'JetBrains Mono',monospace;color:var(--text-muted);line-height:1.6;">
                        <span>"Oklahoma City, OK"</span>
                        <span style="color:var(--border-subtle);padding:0 10px;">"·"</span>
                        <button on:click=move |_| {
                            #[cfg(not(feature = "ssr"))]
                            let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                            set_email_copied.set(true);
                            set_timeout(move || set_email_copied.set(false), std::time::Duration::from_millis(2000));
                        } style="font-size:inherit;font-family:inherit;line-height:inherit;vertical-align:baseline;color:#22d3ee;background:none;border:none;padding:0;margin:0;cursor:pointer;" aria-label="Copy email">
                            {move || if email_copied.get() { "✓ Copied!" } else { EMAIL }}
                        </button>
                        <span style="color:var(--border-subtle);padding:0 10px;">"·"</span>
                        // VERIFY: https://www.linkedin.com/in/richard-mussell/ — full URL, target=_blank, noopener.
                        <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer" style="color:#22d3ee;text-decoration:none;vertical-align:baseline;">"LinkedIn"</a>
                        <span style="color:var(--border-subtle);padding:0 10px;">"·"</span>
                        // VERIFY: https://github.com/richardmussell — full URL, target=_blank, noopener.
                        <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" style="color:#22d3ee;text-decoration:none;vertical-align:baseline;">"GitHub"</a>
                    </div>
                    <a href="/pdfs/resume.pdf" target="_blank" rel="noopener noreferrer"
                        aria-label="Download resume as PDF (opens in new tab)"
                        class="inline-flex items-center gap-2 text-[12px] font-mono text-[var(--text-muted)] border border-[var(--border-subtle)] px-4 py-2 rounded-lg hover:border-[var(--border-active)] hover:text-[var(--accent-cyan)] transition-colors cursor-pointer bg-transparent mt-4 no-underline w-fit"
                        style="min-height:44px;">
                        "Download Resume (PDF)"
                        <span class="sr-only">"(opens in new tab)"</span>
                    </a>
                </header>

                // ── SUMMARY ──
                <section class="mb-10">
                    <h2 class="text-[11px] font-mono font-semibold text-[var(--accent-cyan)] uppercase tracking-[0.15em] mb-4">"Summary"</h2>
                    <p class="text-[15px] text-[var(--text-secondary)] leading-7 max-w-2xl">
                        "Systems Administrator and DevOps Engineer with a BS in IT & Administrative Management (Cybersecurity specialization). Hands-on experience in SOC monitoring, ELK Stack log analysis, and enterprise toolchain operations (ICOMS, Salesforce). Lab projects spanning Terraform IaC, Linux hardening, SIEM observability pipelines, and WireGuard zero-trust networking."
                    </p>
                </section>

                // ── BACKGROUND ──
                <section class="mb-10">
                    <h2 class="text-[11px] font-mono font-semibold text-[var(--accent-cyan)] uppercase tracking-[0.15em] mb-4">"Background"</h2>
                    <div class="resume-timeline">
                        <div class="resume-timeline-item mb-12">
                            <p class="resume-date">"JUNE 2024"</p>
                            <div>
                                <p class="resume-institution">"CENTRAL WASHINGTON UNIVERSITY (CWU)"</p>
                                <p class="resume-degree">"B.S. IN INFORMATION TECHNOLOGY & ADMIN MANAGEMENT"</p>
                                <p class="resume-meta">"Specialization: Cybersecurity | GPA: 3.018"</p>
                                <p class="resume-desc">"Completed advanced curriculum in Network Defense and NIST Cybersecurity Frameworks. Specialized in the intersection of administrative protocols and secure enterprise IT delivery."</p>
                            </div>
                        </div>

                        <div class="resume-timeline-item">
                            <p class="resume-date">"2018 – 2019"</p>
                            <div>
                                <p class="resume-institution">"CISCO NETWORKING ACADEMY"</p>
                                <p class="resume-degree">"CCNA: INTRODUCTION TO NETWORKS"</p>
                                <p class="resume-meta">"Technical Training Portfolio"</p>
                                <p class="resume-desc">"Gained hands-on experience in physical infrastructure provisioning and structured Layer 1-3 troubleshooting. Developed proficiency in Cisco IOS CLI, VLSM subnetting, and port security implementation."</p>
                            </div>
                        </div>
                    </div>
                </section>

                // ── PROFESSIONAL EXPERIENCE ──
                <section class="mb-10">
                    <h2 class="text-[11px] font-mono font-semibold text-[var(--accent-cyan)] uppercase tracking-[0.15em] mb-4">"Professional Experience"</h2>
                    <div class="space-y-4">
                        <div class="resume-timeline-dot-item">
                            <span class="resume-timeline-dot" aria-hidden="true"></span>
                            <span class="resume-timeline-year">"March 2024 – June 2024"</span>
                            <span class="resume-timeline-label">"PISCES Security Operations Center (SOC)"</span>
                            <p class="resume-timeline-detail">
                                <span>"Role: Student SOC Analyst (Internship)"</span><br/>
                                <span>"• Triaged ~50 alerts/day across 13 municipal entities' ELK Stack deployments; escalated ~3–5 incidents/week to senior analysts."</span><br/>
                                <span>"• Authored KQL detection logic for anomalous endpoint behavior and lateral movement indicators."</span><br/>
                                <span>"• Produced weekly incident briefs summarizing environment-wide threat landscape for SOC leadership and municipal stakeholders."</span><br/>
                                <span>"• Maintained ELK Stack dashboards and correlation searches during shift rotations."</span>
                            </p>
                        </div>
                        <div class="resume-timeline-dot-item">
                            <span class="resume-timeline-dot" aria-hidden="true"></span>
                            <span class="resume-timeline-year">"Oct 2024 – April 2025"</span>
                            <span class="resume-timeline-label">"Cox Communications"</span>
                            <p class="resume-timeline-detail">
                                <span>"Role: Account Management & Retention Specialist"</span><br/>
                                <span>"• Managed ~40–60 customer accounts/day using ICOMS and Salesforce enterprise systems; resolved multi-service configuration conflicts across billing, network, and provisioning layers."</span><br/>
                                <span>"• Met retention targets through consultative troubleshooting and structured escalation paths for Tier-1 ISP service recovery."</span><br/>
                                <span>"• Navigated enterprise database operations (ICOMS) for account state reconciliation and billing dispute resolution."</span>
                            </p>
                        </div>
                        <div class="resume-timeline-dot-item">
                            <span class="resume-timeline-dot" aria-hidden="true"></span>
                            <span class="resume-timeline-year">"Sep 2025 – Present"</span>
                            <span class="resume-timeline-label">"Self-directed Platform Engineering / Homelab"</span>
                            <p class="resume-timeline-detail">
                                <span>"Role: Independent"</span><br/>
                                <span>"• Built and shipped a Rust + Leptos + WASM32 portfolio (5,705 LOC, SQLite FFI with 12 unsafe blocks and AtomicU64 telemetry counters) deployed via Trunk to GitHub Pages with a 4-target cargo-check CI matrix."</span><br/>
                                <span>"• Operating a 12-tool SOC homelab on bare metal — Wazuh, Suricata, Falco, TheHive, MISP, n8n SOAR — end-to-end detection/response pipeline."</span><br/>
                                <span>"• Authoring IaC labs: Terraform GCP landing zone (CIS-aligned, policy-validated), WireGuard + AD zero-trust fabric, Prometheus/Grafana SLO-aligned monitoring pipeline."</span><br/>
                                <span>"• Pursuing GCP Associate Cloud Engineer (target: Q3 2026) and Certified Kubernetes Administrator (target: Q4 2026)."</span><br/>
                                <span>"• Concurrent income: Product Brand Ambassador at Club Demonstration Services (Costco wholesale partner) — non-technical, disclosed for continuity."</span>
                            </p>
                        </div>
                    </div>
                </section>

                // ── CERTIFICATIONS ──
                <section class="mb-10">
                    <h2 class="text-[11px] font-mono font-semibold text-[var(--accent-cyan)] uppercase tracking-[0.15em] mb-4">"Certifications"</h2>
                    <div class="space-y-2">
                        <p class="text-[13px] text-[var(--text-secondary)] font-mono">
                            "• GCP Associate Cloud Engineer — Pursuing (target exam: Q3 2026)"
                        </p>
                        <p class="text-[13px] text-[var(--text-secondary)] font-mono">
                            "• Certified Kubernetes Administrator — Pursuing (target exam: Q4 2026)"
                        </p>
                    </div>
                </section>

                // ── TECHNICAL SKILLS ──
                <section class="mb-10">
                    <h2 class="text-[11px] font-mono font-semibold text-[var(--accent-cyan)] uppercase tracking-[0.15em] mb-4">"Technical Skills"</h2>
                    <div class="resume-skills-grid grid grid-cols-2 md:grid-cols-2 gap-2">
                        {vec![
                            (
                                "Cloud & Orchestration",
                                "Terraform (IaC), Pulumi, Kubernetes, Docker, GitHub Actions, AWS/Azure Basics",
                            ),
                            (
                                "Systems & Administration",
                                "Linux (RHEL/Ubuntu), Windows Server 2022, Active Directory (GPO), NIST Framework, Linux Hardening",
                            ),
                            (
                                "Networking & Connectivity",
                                "TCP/IP, VLANs, WireGuard VPN, DNS/DHCP, Cisco IOS CLI, Layer 1-3 Troubleshooting",
                            ),
                            (
                                "Automation & Identity",
                                "Bash Scripting, PowerShell, Cron Jobs, Identity Lifecycle (IAM), RBAC",
                            ),
                        ]
                        .into_iter()
                        .map(|(category, tools)| {
                            view! {
                                <div class="flex flex-col gap-1 py-1">
                                    <div class="flex items-center gap-2">
                                        <span class="w-1 h-1 rounded-full shrink-0" style="background-color:var(--accent-cyan)"></span>
                                        <span class="text-[13px] font-bold" style="color:var(--accent-cyan);">{category}</span>
                                    </div>
                                    <span class="text-[var(--text-secondary)] text-[13px]">{tools}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                    </div>
                </section>

                // ── PROJECTS ──
                <section class="mb-10">
                    <h2 class="text-[11px] font-mono font-semibold text-[var(--accent-cyan)] uppercase tracking-[0.15em] mb-6">"Projects"</h2>
                    <div class="space-y-8">
                        {vec![
                            (
                                "Hardened Cloud Landing Zone (IaC)",
                                "Tags: Terraform, GCP, GCS Backend, NIST 800-53",
                                "Summary: Engineered a secure, modular landing zone using Terraform and GCP. Implemented GCS remote state-locking and architected \"Private-First\" VPCs with zero public-facing entry points, aligned with NIST 800-53 security controls.",
                                vec!["Terraform", "GCP", "GCS Backend", "NIST 800-53"],
                            ),
                            (
                                "Systems Lifecycle Automation Framework",
                                "Tags: Bash (POSIX), Linux, Idempotency, Hardening",
                                "Summary: Built an idempotent systems lifecycle framework using POSIX-compliant Bash (set -euo pipefail). Automated RBAC-aligned user provisioning and CIS-standard system hardening to eliminate configuration drift and operational toil.",
                                vec!["Bash (POSIX)", "Linux", "Idempotency", "Hardening"],
                            ),
                            (
                                "Multi-Tier Strategic Observability Pipeline",
                                "Tags: Prometheus, ELK Stack, Grafana, SLO/SLI",
                                "Summary: Architected a multi-tier observability pipeline using Prometheus and ELK. Integrated Logstash filters for security-context enrichment and designed Grafana dashboards mapping technical metrics to operational SLOs.",
                                vec!["Prometheus", "ELK Stack", "Grafana", "SLO/SLI"],
                            ),
                            (
                                "Zero-Trust Administrative Fabric",
                                "Tags: WireGuard, AWS VPC, Active Directory, NIST 800-207",
                                "Summary: Engineered an identity-based SASE administrative fabric using WireGuard and AWS. Implemented MSS clamping to prevent packet fragmentation across cloud boundaries and integrated out-of-band peer authorization via Active Directory.",
                                vec!["WireGuard", "AWS VPC", "Active Directory", "NIST 800-207"],
                            ),
                        ].into_iter().map(|(title, subtitle, summary, stack)| {
                            let stack_view = stack.into_iter().take(5).map(|t| view! {
                                <span style="display:inline-block;font-size:10px;font-family:'JetBrains Mono',monospace;color:#22d3ee;background:#0d1320;border:1px solid #1a2540;border-radius:3px;padding:2px 8px;white-space:nowrap;">{t}</span>
                            }).collect_view();
                            view! {
                                <div class="resume-project-card pb-8 border-b border-[#1a2540] last:border-0 last:pb-0">
                                    <div class="flex items-start justify-between gap-4 mb-2">
                                        <div>
                                            <h3 class="text-[15px] font-bold text-[#f1f5f9]">{title}</h3>
                                            <p class="text-[12px] font-mono text-[#64748b] mt-0.5">{subtitle}</p>
                                        </div>
                                        <span class="text-[10px] font-mono text-[#10b981] shrink-0 mt-0.5">"● SELF-DIRECTED"</span>
                                    </div>
                                    <p class="text-[14px] text-[var(--text-secondary)] leading-6 mb-4 max-w-xl">{summary}</p>
                                    <div class="flex flex-wrap gap-2">{stack_view}</div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </section>

                <footer class="pt-8 border-t border-[var(--border-subtle)] flex items-center justify-between">
                    <p class="text-[11px] font-mono text-[var(--text-muted)]">"Built with Leptos — compiled to WebAssembly"</p>
                    <button on:click=move |_| {
                        #[cfg(not(feature = "ssr"))]
                        let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                        set_email_copied.set(true);
                        set_timeout(move || set_email_copied.set(false), std::time::Duration::from_millis(2000));
                    } class="text-[11px] font-mono text-[var(--text-muted)] hover:text-[#22d3ee] transition-colors cursor-pointer bg-transparent border-0 p-0" aria-label="Copy email">
                        EMAIL
                    </button>
                </footer>
            </div>
        </main>
    }
}

// ============================================================
//  SHARED: What I am looking for (Contact + One-Pager)
// ============================================================
