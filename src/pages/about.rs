use crate::data::PROFESSIONAL_TITLE;
use leptos::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text=move || format!("About · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="About Richard Mussell — Linux Systems Administrator based in Oklahoma City, OK. Lab projects spanning IaC, Linux hardening, observability, and zero-trust networking."/>
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
                        "I'm a Linux systems administrator in Oklahoma City, open to remote. This site documents how I operate, not what title I'm claiming."
                    </p>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:24px;">
                        "My background includes an academic SOC rotation, not operator tenure. I spent a term at PISCES — Central Washington's student SOC — building visualizations on the ELK stack, working the ticketing system as first-line triage, and watching analysts handle escalation across thirteen municipal networks. The rotation was part of my BS in IT and Administrative Management, cybersecurity specialization, Central Washington University, 2024. I did not own detections or run shifts at volume. What I took from it was a stance, not a skill: most of what breaks production gets buried under noise that looked routine yesterday, and the work of operations is often the work of making that noise visible before it matters."
                    </p>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:24px;">
                        "I build with that bias now — observability before automation, automation before features, honest limits before claimed outcomes."
                    </p>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:64px;">
                        "The lab is where that plays out. I run a hardened Windows fleet with Intune and Autopilot, an Active Directory domain with GPO-driven baselines, WSUS patching, and a twelve-tool SOC stack — Wazuh, Suricata, Falco, TheHive, MISP, n8n, and the rest — on bare metal. The Terraform landing zone in this portfolio passes 87 of 92 CIS GCP controls under nightly drift detection. That's lab scale, not enterprise scale, and I'll say so on any page that claims a number."
                    </p>
                </section>

                <section>
                    <div style="display:flex;align-items:center;gap:16px;margin-top:0;margin-bottom:24px;">
                        <h2 style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;margin:0;">"Technical Trajectory"</h2>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin-bottom:32px;">
                        "Eight areas, split honestly between hands-on lab work and deliberate study. Where a card says studying, I haven't shipped production work in it yet."
                    </p>
                    <div class="about-card-grid" style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:72px;">
                        {
                            let cards = vec![
                                ("Terraform & Infrastructure as Code",
                                 "Modular Terraform on GCP, with remote state in GCS, nightly drift detection, and tfsec + Checkov gates in CI. The lab landing zone passes 87 of 92 CIS controls. I own the state-locking failure modes because state lives in my GCS bucket; the tradeoff was auditability over managed-service ease."),
                                ("Linux Systems Administration",
                                 "RHEL and Ubuntu server administration in lab: CIS-aligned hardening baselines, sysctl tuning, fstab security flags (noexec, nosuid), and idempotent provisioning through POSIX Bash with strict mode. Hands-on. Not production-scale fleet management yet — lab scale."),
                                ("Active Directory & Identity",
                                 "Hands-on in lab: GPO-driven baselines, OU-based RBAC, PowerShell for user lifecycle automation, AutomatedLab for deterministic Windows Server 2022 domains. Production-scale AD operations — large directories, federation, lifecycle at headcount — I haven't done."),
                                ("Observability & SIEM",
                                 "Lab stack: Prometheus metrics, Grafana dashboards tied to SLOs, ELK for log aggregation. Wazuh, Suricata, Falco, TheHive, MISP wired up end-to-end on bare metal for the SOC homelab. Tuning detections at production noise levels is exposure, not experience."),
                                ("Zero-Trust & WireGuard",
                                 "Identity-based admin access via WireGuard tunnels, AD-gated authorization, and micro-segmentation enforced at the network layer. In-development lab — the design exists end-to-end, the reconciliation against production-grade identity infrastructure hasn't happened."),
                                ("Kubernetes",
                                 "Studying. I can deploy workloads with kubectl and read the scheduler and etcd at a conceptual level; I haven't operated a cluster at scale or written controllers. Not my current priority — listed because the field asks."),
                                ("Containers & CI/CD",
                                 "Docker for lab service packaging. GitHub Actions for the build-check-deploy pipeline on this portfolio (four cargo-check gates across wasm32, ssr, hydrate, and ssg targets; wasm-opt; gh-pages deploy). Hands-on on a single-repo scale, not multi-service CI at team scale. Reading toward Ansible for fleet configuration, Vault for secrets management, ArgoCD for GitOps delivery, and Helm for Kubernetes packaging — haven't shipped with any of them yet."),
                                ("Security Frameworks & Compliance",
                                 "NIST 800-53 and 800-207 controls referenced in the Terraform and zero-trust labs. CIS benchmarks enforced through Terraform gates on the GCP baseline. Academic SOC exposure at PISCES — the scope described in Who I Am. Framework literacy, not compliance-program ownership."),
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

                <section style="margin-top:48px;">
                    <div style="display:flex;align-items:center;gap:16px;margin-bottom:20px;">
                        <h2 style="font-size:9px;font-weight:600;letter-spacing:0.2em;color:#22d3ee;font-family:'JetBrains Mono',monospace;white-space:nowrap;margin:0;">"How I Operate"</h2>
                        <div style="flex:1;height:1px;background:#1a2540;"></div>
                    </div>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin:0 0 16px 0;">
                        "Every system I run gets its observability wired before it gets anything else. When a change lands, I want to know what it affected before the next ticket comes in. The tradeoff is upfront time — instrumenting a service takes longer than just deploying it — and I pay that cost on purpose, because the alternative is debugging production in the dark. The portfolio's telemetry page instruments LCP, heap, TTFB, and network probes as a live example; the Prometheus and ELK lab does the same pattern at infrastructure scale."
                    </p>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin:0 0 16px 0;">
                        "I don't do manual changes to systems I expect to run twice. If a task is worth doing once by hand, it's worth the next hour writing it as code — Bash when the scope is small and local, Terraform when the scope is infrastructure, PowerShell when it's a Windows domain. First runs cost more: my Terraform landing zone took longer to build than console click-through would have. The return is that the second environment costs me five minutes, the tenth is free, and every one of them is auditable."
                    </p>
                    <p style="color:#94a3b8;font-size:15px;line-height:1.9;max-width:620px;margin:0 0 48px 0;">
                        "Security and identity are not a layer I add at the end. The zero-trust lab is built around AD-gated WireGuard tunnels because adding identity to a network that didn't start with it means undoing assumptions everywhere. The Terraform baseline runs CIS checks on every apply, not quarterly. The tradeoff is that early decisions constrain later ones — I can't trivially add a service that doesn't fit the access model — and that constraint is the point. Boundaries I can't slip past are the ones that hold."
                    </p>
                </section>

                // Email / LinkedIn / GitHub were here; removed because the
                // site-wide <SiteFooter /> now renders them consistently at
                // the bottom of every page. Avoids the pattern of having one
                // contact block visually 20 px above another at the fold.
            </div>
        </main>
    }
}

// ============================================================
//  WRITING PAGE
