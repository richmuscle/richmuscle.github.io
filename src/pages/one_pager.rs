use crate::data::{get_infrastructure_fleet, EMAIL, PROFESSIONAL_TITLE};
use crate::utils::track;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::A;

#[component]
pub fn OnePageSummary() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);
    #[cfg(feature = "ssr")]
    let _ = set_email_copied;
    let projects = get_infrastructure_fleet();
    view! {
        <Title text=move || format!("One-Pager · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="One-page summary of Richard Mussell's platform engineering, DevOps, and systems engineering targets, featured projects, and technical depth."/>
        <main id="main-content" class="one-pager min-h-screen pt-16 pb-24">
            <div class="one-pager-inner">
                <section class="one-pager-section">
                    <h1 class="one-pager-name">"Richard Mussell"</h1>
                    <p class="one-pager-title">"Systems Administrator & DevOps Engineer"</p>
                    <p class="one-pager-meta">{format!("{} · Oklahoma City, OK (remote open)", EMAIL)}</p>
                    <div class="one-pager-actions one-pager-action-row">
                        <button
                            type="button"
                            class="hero-btn one-pager-print-btn"
                            on:click=move |_| {
                                #[cfg(not(feature = "ssr"))]
                                {
                                    let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                                    set_email_copied.set(true);
                                    track("email_copy", r#"{"source":"one-pager"}"#);
                                    let _ = gloo_timers::callback::Timeout::new(2000, move || set_email_copied.set(false));
                                }
                            }
                        >
                            {move || if email_copied.get() { "✓ Copied" } else { "Copy Email" }}
                        </button>
                        <A href="/" class="hero-btn">"View Full Site →"</A>
                    </div>
                </section>

                <section class="one-pager-section">
                    <p class="one-pager-sentence">"I build repeatable infrastructure with Terraform, automate system administration with PowerShell and Bash, and manage identity and access through Active Directory. Looking for a team where infrastructure is code and operations run on automation, not ticket queues."</p>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"Featured Projects"</h2>
                    {[
                        (
                            "security-baseline-audit",
                            "Security Baseline & Continuous Audit Toolkit",
                            "CIS GCP 87/92 controls passing, Terraform compliance gates, nightly drift detection, and automated reconciliation back to hardened baseline.",
                            ["Terraform", "CIS Benchmark", "tfsec"],
                        ),
                        (
                            "observability-operational-intelligence",
                            "Observability & Operational Intelligence Platform",
                            "Multi-tier alerting pipeline: Prometheus metrics, ELK log enrichment, Grafana SLO dashboards, and automated anomaly detection.",
                            ["Prometheus", "Grafana", "ELK Stack"],
                        ),
                        (
                            "identity-access-lifecycle",
                            "Identity & Access Lifecycle Platform",
                            "Identity-based admin access via WireGuard tunnels, AD-gated authorization, micro-segmentation, and instant credential revocation.",
                            ["WireGuard", "AWS VPC", "Active Directory"],
                        ),
                    ]
                    .into_iter()
                    .filter_map(|(slug, display_title, summary, tags)| {
                        let p = projects.iter().find(|p| p.slug == slug)?;
                        let accent = p.category.accent();
                        Some((slug, display_title, summary, tags, accent))
                    })
                    .map(|(slug, display_title, summary, tags, accent)| {
                        view! {
                            <div class="one-pager-project-row">
                                <span class="one-pager-dot one-pager-project-dot" style=format!("color:{}", accent) aria-hidden="true">"●"</span>
                                <div class="one-pager-project-body">
                                    <A href=format!("/project/{}", slug) class="one-pager-project-title">{display_title}</A>
                                    <span class="one-pager-project-outcome">{summary}</span>
                                    <div class="one-pager-project-pills">
                                        {tags.into_iter().map(|t| view! { <span class="one-pager-pill">{t}</span> }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"Technical Focus Areas"</h2>
                    <div class="one-pager-three-col one-pager-tech-grid">
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Systems Administration"</strong>
                            <p>"Linux (RHEL/Ubuntu) and Windows Server administration, Active Directory GPO management, user lifecycle automation, and CIS-standard system hardening."</p>
                        </div>
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Infrastructure as Code"</strong>
                            <p>"Terraform modules with remote state locking, reproducible cloud environments, drift detection, and policy-validated deployments on GCP and AWS."</p>
                        </div>
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Observability & Security"</strong>
                            <p>"Prometheus metrics, ELK Stack log aggregation, Grafana dashboards, WireGuard VPN, and NIST-aligned security controls for hybrid environments."</p>
                        </div>
                    </div>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"What I am looking for"</h2>
                    <ul class="contact-wish-list">
                        <li><span class="contact-arrow">"->"</span>" Teams that treat infrastructure as code and value reproducible, auditable deployments."</li>
                        <li><span class="contact-arrow">"->"</span>" Environments where systems administration includes automation, not just ticket queues."</li>
                        <li><span class="contact-arrow">"->"</span>" Organizations investing in observability and proactive monitoring over reactive firefighting."</li>
                        <li><span class="contact-arrow">"->"</span>" Hybrid-cloud or on-prem environments with real security requirements (NIST, CIS, zero-trust)."</li>
                        <li><span class="contact-arrow">"->"</span>" Remote-first or Oklahoma City-based. Open to relocation for the right role."</li>
                    </ul>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"Education"</h2>
                    <div class="one-pager-edu-block">
                        <p class="one-pager-edu-row">
                            <span class="one-pager-edu-label">"Institution:"</span>
                            "Central Washington University (2020 - 2024)"
                        </p>
                        <p class="one-pager-edu-row">
                            <span class="one-pager-edu-label">"Degree:"</span>
                            "Bachelor of Science (BS), Information Technology — "
                            <span class="one-pager-edu-accent">"Cybersecurity Specialization"</span>
                        </p>
                        <p class="one-pager-edu-row one-pager-edu-focus">
                            <span class="one-pager-edu-label">"Minors:"</span>
                        </p>
                        <ul class="one-pager-edu-list">
                            <li>"Sustainable Practices in IT"</li>
                            <li>"Project Management"</li>
                        </ul>
                        <p class="one-pager-edu-summary">
                            "Coursework in network defense, NIST cybersecurity frameworks, and secure enterprise IT delivery. SOC internship monitoring 13 municipal entities with ELK Stack."
                        </p>
                    </div>
                    <div class="one-pager-edu-divider" aria-hidden="true"></div>
                    <p class="one-pager-h2">"Currently Studying"</p>
                    <ul class="one-pager-edu-list one-pager-edu-list-certs">
                        <li><span class="one-pager-edu-label">"GCP ACE"</span>" — Google Cloud Associate Cloud Engineer (Target Q3 2026)"</li>
                        <li><span class="one-pager-edu-label">"CKA"</span>" — Certified Kubernetes Administrator (Target Q4 2026)"</li>
                        <li><span class="one-pager-edu-label">"Cisco CCNA"</span>" — Networking Academy Coursework (2018–2019)"</li>
                    </ul>
                </section>

                <section class="one-pager-section">
                    <button type="button" class="hero-btn one-pager-print-btn" on:click=move |_| {
                        track("print", r#"{"page":"one-pager"}"#);
                        #[cfg(not(feature = "ssr"))]
                        if let Some(w) = web_sys::window() { let _ = w.print(); }
                    }>
                        "🖨 Print / Save as PDF"
                    </button>
                </section>
            </div>
        </main>
    }
}

// ============================================================
//  ABOUT PAGE  — Systems Engineer persona
// ============================================================
