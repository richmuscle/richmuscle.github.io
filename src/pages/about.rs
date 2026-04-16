use crate::data::{EMAIL, GITHUB_URL, LINKEDIN_URL, PROFESSIONAL_TITLE};
use leptos::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn AboutPage() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);

    view! {
        <Title text=move || format!("About · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="About Richard Mussell — Systems Administrator & DevOps Engineer based in Oklahoma City, OK. Lab projects spanning IaC, Linux automation, observability, and zero-trust networking."/>
        <main id="main-content" class="min-h-screen page-enter about-main" style="padding-top:80px;">
            <div class="about-wrap" style="max-width:760px;margin:0 auto;padding:clamp(40px,8vw,80px) clamp(16px,5vw,40px) clamp(60px,10vw,100px);padding-top:96px;">

                <h1 class="sr-only">"About Richard Mussell"</h1>

                <section>
                    <div style="margin-bottom:20px;">
                        <p class="font-mono text-[#22d3ee] uppercase" style="font-size:9px;font-weight:600;letter-spacing:0.2em;margin-bottom:0;">"About"</p>
                        <div style="display:flex;align-items:center;gap:16px;margin-bottom:0;">
                            <h2 style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;margin:0;">"Who I Am"</h2>
                            <div style="flex:1;height:1px;background:#1a2540;"></div>
                        </div>
                    </div>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:24px;">
                        "Richard Mussell is a Systems Administrator and DevOps Engineer based in Oklahoma City. Cybersecurity-specialized ITAM graduate (Central Washington University, 2024) with SOC analyst experience monitoring 13 municipal government networks on ELK. Operating production-grade infrastructure labs spanning Intune/Autopilot zero-touch deployment, Active Directory with GPO, WSUS patch automation, and a 12-tool SOC homelab (Wazuh, Suricata, Falco, TheHive, MISP, n8n SOAR). Pursuing GCP Associate Cloud Engineer (Q3 2026) and Certified Kubernetes Administrator (Q4 2026)."
                    </p>
                    <p style="color:var(--text-muted);font-size:12px;line-height:1.7;max-width:620px;margin-bottom:24px;font-family:'JetBrains Mono',monospace;">
                        "This portfolio is a Rust + Leptos application compiled to wasm32-unknown-unknown. It ships in CSR mode (default deploy); SSR, hydrate, and SSG build targets are feature-gated in Cargo.toml and compile-validated in CI, ready for Phase 2. Served as a static binary on GitHub Pages — zero server runtime, zero GC."
                    </p>
                    <p style="color:var(--text-muted);font-size:12px;line-height:1.7;max-width:620px;margin-bottom:64px;font-family:'JetBrains Mono',monospace;">
                        "Visual system: fibonacci spacing scale + golden-ratio type scale, single-accent cyan, layered SCSS (tokens → base → components → pages)."
                    </p>
                </section>

                <section>
                    <div style="display:flex;align-items:center;gap:16px;margin-top:0;margin-bottom:24px;">
                        <h2 style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;margin:0;">"Technical Trajectory"</h2>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:32px;">
                        "Where I have hands-on experience and where I am actively building depth."
                    </p>
                    <div class="about-card-grid" style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:72px;">
                        {
                            let cards = vec![
                                ("Terraform & Infrastructure as Code",
                                 "Building modular, version-controlled cloud environments with remote state locking, drift detection, and policy-validated deployments. Hands-on with GCP and AWS."),
                                ("Linux Systems Administration",
                                 "RHEL and Ubuntu server management, CIS-standard hardening, sysctl tuning, fstab security flags, and idempotent provisioning with POSIX-compliant Bash scripts."),
                                ("Active Directory & Identity Management",
                                 "GPO management, RBAC-aligned user provisioning, LDAP-based authorization gates, and identity lifecycle automation with PowerShell."),
                                ("Observability & SIEM",
                                 "Prometheus metrics collection, ELK Stack log aggregation with Logstash filters, Grafana dashboards tied to SLOs, and KQL-based log analysis from SOC experience."),
                                ("WireGuard & Zero-Trust Networking",
                                 "Identity-based network access with WireGuard tunnels, micro-segmentation, MSS clamping for cross-cloud stability, and out-of-band peer authorization via AD."),
                                ("Kubernetes (CKA Prep)",
                                 "Deploying workloads with kubectl; studying the control plane and writing custom controllers via CKA prep — scheduler internals, etcd consistency model, controller-runtime from first principles."),
                                ("Containers & CI/CD",
                                 "Docker containerization, GitHub Actions workflows for automated build/test/deploy, and Trunk-based WASM compilation pipelines."),
                                ("Security Frameworks & Compliance",
                                 "NIST 800-53 and 800-207 controls, CIS benchmarks, network defense coursework, and hands-on SOC monitoring across municipal government entities."),
                            ];

                            cards.into_iter().enumerate().map(|(i, (label, desc))| {
                                let n = 8usize;
                                let base_style = "background:#080c14;border:1px solid #1a2540;border-radius:8px;padding:24px 28px;";
                                let style = if i == n - 1 && n % 2 == 1 {
                                    format!("{}grid-column: 1 / -1; max-width: calc(50% - 8px);", base_style)
                                } else { base_style.to_string() };
                                view! {
                                    <div class="about-card" style=style>
                                        <h3 style="font-size:11px;letter-spacing:0.12em;color:#22d3ee;font-family:'JetBrains Mono',monospace;margin:0 0 12px 0;display:block;">{label}</h3>
                                        <p style="font-size:12.5px;color:var(--text-body);line-height:1.8;font-family:var(--font-body);">{desc}</p>
                                    </div>
                                }
                            }).collect_view()
                        }
                    </div>
                </section>

                <blockquote class="about-quote" style="border-left:2px solid #22d3ee;padding:28px 36px;margin:0 0 72px 0;background:#080c14;border-radius:0 8px 8px 0;font-style:normal;">
                    <p style="font-size:15px;color:#94a3b8;line-height:1.9;margin:0;">
                        "The professionals I admire most can automate a repetitive onboarding workflow, debug a complex Active Directory sync issue, and maintain high-availability systems with quiet precision. That is the range I operate in: reliable, automation-focused, operationally grounded. I built and shipped a Rust/WASM portfolio with SQLite FFI and self-gated CI to demonstrate, not declare, that standard."
                    </p>
                </blockquote>

                <section style="margin-top:48px;">
                    <div style="display:flex;align-items:center;gap:16px;margin-bottom:20px;">
                        <h2 style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;margin:0;">"How I Think About Systems"</h2>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <p style="font-size:14px;color:var(--text-secondary);line-height:1.9;margin:0 0 16px 0;">
                        "Every system I manage starts with the question: how will I know when this is failing? Before deploying a configuration change, I verify the monitoring and alerting stack. A system without clear visibility into its logs and health metrics is an operational risk. I prioritize observability to ensure that I can identify and resolve issues before they impact the end-user experience. (See the Prometheus/Grafana SLO pipeline lab; the portfolio's own telemetry page instruments LCP and heap metrics as a live example.)"
                    </p>
                    <p style="font-size:14px;color:var(--text-secondary);line-height:1.9;margin:0 0 16px 0;">
                        "I am drawn to systems that are predictable and well-documented. In production environments, consistency is the key to stability. I believe in minimizing manual intervention through scripting and Infrastructure as Code (IaC). By standardizing deployments and eliminating configuration drift, we ensure that infrastructure remains reliable as it scales. (See the Terraform GCP landing zone — modular, remote-state, policy-validated, CIS-aligned.)"
                    </p>
                    <p style="font-size:14px;color:var(--text-secondary);line-height:1.9;margin:0 0 48px 0;">
                        "The core infrastructure—Active Directory, Linux, and Cloud services—should be the reliable foundation of any organization. The interesting work is making these systems more efficient: managing secure identity fabrics (IAM), automating user lifecycles with PowerShell and Bash, and ensuring that security and compliance are baked into every workflow. I want to handle the technical complexities so that the infrastructure remains a seamless service for the business. (See the Active Directory + GPO + WireGuard zero-trust lab; PowerShell identity lifecycle automation.)"
                    </p>
                </section>

                <section>
                    <div style="display:flex;align-items:center;gap:16px;margin-bottom:20px;">
                        <h2 style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;margin:0;">"Adjacent Technologies"</h2>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <div class="about-pills-row" style="display:flex;flex-wrap:wrap;gap:8px;margin-bottom:72px;">
                        {vec![
                            "Kubernetes (CKA path)", "Ansible", "Crossplane", "Pulumi",
                            "Helm Charts", "ArgoCD", "Vault", "SPIRE/SPIFFE",
                            "Cilium", "Talos Linux", "Packer", "Rust (systems tooling)",
                        ].into_iter().map(|pill| view! {
                            <span style="background:#080c14;border:1px solid #1a2540;border-radius:3px;padding:6px 14px;font-family:'JetBrains Mono',monospace;font-size:10px;color:#22d3ee;letter-spacing:0.08em;">{pill}</span>
                        }).collect_view()}
                    </div>
                </section>

                <section style="border-top:1px solid #1a2540;padding-top:40px;">
                    <div style="display:flex;align-items:center;gap:40px;font-size:13px;color:#22d3ee;flex-wrap:wrap;">
                        <button
                            type="button"
                            class="about-email-copy cursor-pointer hover:opacity-80 transition-opacity"
                            aria-label="Copy email address to clipboard"
                            aria-live="polite"
                            on:click=move |_| {
                                #[cfg(not(feature = "ssr"))]
                                let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                                set_email_copied.set(true);
                                set_timeout(move || set_email_copied.set(false), std::time::Duration::from_millis(2000));
                            }
                        >
                            {move || if email_copied.get() { "Copied!" } else { EMAIL }}
                        </button>
                        // VERIFY: https://www.linkedin.com/in/richard-mussell/ — full URL, target=_blank, noopener.
                        <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer" class="no-underline hover:opacity-80 transition-opacity" style="color:#22d3ee;">"LinkedIn"</a>
                        <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="no-underline hover:opacity-80 transition-opacity" style="color:#22d3ee;">"GitHub"</a>
                    </div>
                </section>
            </div>
        </main>
    }
}

// ============================================================
//  WRITING PAGE
