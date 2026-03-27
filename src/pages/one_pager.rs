use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::A;
use crate::data::{get_infrastructure_fleet, EMAIL, PROFESSIONAL_TITLE};
use crate::utils::track;

#[component]
pub fn OnePageSummary() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);
    let projects = get_infrastructure_fleet();
    view! {
        <Title text=move || format!("One-Pager · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="One-page summary of Richard Mussell's platform engineering, DevOps, and systems engineering targets, featured projects, and technical depth."/>
        <main id="main-content" class="one-pager min-h-screen pt-16 pb-24">
            <div class="one-pager-inner">
                <section class="one-pager-section">
                    <h1 class="one-pager-name">"Richard Mussell"</h1>
                    <p class="one-pager-title">"Systems Engineering & Platform Operations"</p>
                    <p class="one-pager-meta">{format!("{} · Oklahoma City, OK (remote open)", EMAIL)}</p>
                    <div class="one-pager-actions one-pager-action-row">
                        <button
                            type="button"
                            class="hero-btn one-pager-print-btn"
                            on:click=move |_| {
                                let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                                set_email_copied.set(true);
                                track("email_copy", r#"{"source":"one-pager"}"#);
                                let _ = gloo_timers::callback::Timeout::new(2000, move || set_email_copied.set(false));
                            }
                        >
                            {move || if email_copied.get() { "✓ Copied" } else { "Copy Email" }}
                        </button>
                        <A href="/" class="hero-btn">"View Full Site →"</A>
                    </div>
                </section>

                <section class="one-pager-section">
                    <p class="one-pager-sentence">"I orchestrate high-integrity ecosystems between the physical bedrock and the unified control plane — building deterministic environments where complexity is silenced to ensure a resilient, 50-year lookout."</p>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"Featured Projects"</h2>
                    {[
                        (
                            "terraform-gcp",
                            "Hardened Cloud Landing Zone (IaC)",
                            "Engineered a secure, version-controlled GCP Landing Zone with deterministic Terraform modules and backend state management.",
                            ["Terraform", "GCP", "GCS Backend"],
                        ),
                        (
                            "monitoring-observability",
                            "Strategic Observability Pipeline",
                            "Architected an action-oriented observability pipeline with noise reduction, SLO alignment, and MTTR-focused anomaly detection.",
                            ["Prometheus", "Grafana", "ELK Stack"],
                        ),
                        (
                            "linux-admin-scripting",
                            "Systems Lifecycle Automation Framework",
                            "Built an idempotent lifecycle framework using POSIX-compliant Bash to eliminate configuration drift and automate RBAC-aligned provisioning.",
                            ["Bash (POSIX)", "Linux", "Idempotency"],
                        ),
                        (
                            "zero-trust-networking",
                            "Zero-Trust Administrative Fabric",
                            "Engineered a ZTNA administrative fabric with WireGuard identity controls, micro-segmentation, and MSS-clamped stability.",
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
                    <h2 class="one-pager-h2">"Technical Depth"</h2>
                    <div class="one-pager-three-col one-pager-tech-grid">
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Bedrock Sovereignty"</strong>
                            <p>"Deterministic Kernel-level enforcement, Bare-metal state reconciliation, and the orchestration of immutable system lifecycles."</p>
                        </div>
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Unified Control Planes"</strong>
                            <p>"Declarative state enforcement through Crossplane compositions, CRD-based reconciliation loops, and self-healing infrastructure logic."</p>
                        </div>
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"High-Fidelity Signal"</strong>
                            <p>"eBPF-driven telemetry at the syscall layer, semantic observability, and high-integrity temporal correlation of system events."</p>
                        </div>
                    </div>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"What I am looking for"</h2>
                    <ul class="contact-wish-list">
                        <li><span class="contact-arrow">"->"</span>" Designing deterministic environments where infrastructure complexity is silenced to serve organizational velocity."</li>
                        <li><span class="contact-arrow">"->"</span>" Architecting Unified Control Planes (Crossplane) that orchestrate multi-cloud compositions beyond standard cluster management."</li>
                        <li><span class="contact-arrow">"->"</span>" Refactoring 'Old Guard' static IaC into dynamic, self-healing landscapes driven by continuous, deterministic state reconciliation."</li>
                        <li><span class="contact-arrow">"->"</span>" Scaling high-integrity ecosystems where identity, connectivity, and governance function as vital, self-governing organs."</li>
                        <li><span class="contact-arrow">"->"</span>" Strategic partnerships with organizations focused on Pinnacle Orchestration (Remote-first or Oklahoma City-based)."</li>
                    </ul>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"ACADEMIC_FOUNDATION_//_STRATEGIC_PURSUIT"</h2>
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
                            <span class="one-pager-edu-label">"Strategic Focus (Minors):"</span>
                        </p>
                        <ul class="one-pager-edu-list">
                            <li>"Sustainable Practices in IT "
                                <span class="one-pager-edu-accent">"(The 50-Year Lookout)"</span>
                            </li>
                            <li>"Project Management "
                                <span class="one-pager-edu-accent">"(The Builder's Ledger)"</span>
                            </li>
                        </ul>
                        <p class="one-pager-edu-summary">
                            "Architectural foundation focused on the sovereignty of critical infrastructure. Trained in deterministic system hardening, high-fidelity incident response, and the logic of global network security. Specializing in the intersection of environmental sustainability and high-integrity technical governance."
                        </p>
                    </div>
                    <div class="one-pager-edu-divider" aria-hidden="true"></div>
                    <p class="one-pager-h2">"STRATEGIC_REFINEMENT_//_IN_PROGRESS"</p>
                    <ul class="one-pager-edu-list one-pager-edu-list-certs">
                        <li><span class="one-pager-edu-label">"Google Cloud"</span>" — Professional Cloud Architect"</li>
                        <li><span class="one-pager-edu-label">"RHCSA"</span>" — Red Hat Certified System Administrator"</li>
                        <li><span class="one-pager-edu-label">"CKA / CKAD"</span>" — Kubernetes Administrator & Application Developer"</li>
                    </ul>
                </section>

                <section class="one-pager-section">
                    <button type="button" class="hero-btn one-pager-print-btn" on:click=move |_| {
                        track("print", r#"{"page":"one-pager"}"#);
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
