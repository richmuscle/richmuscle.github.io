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
                    <p class="one-pager-sentence">"I build the infrastructure layer between bare metal and the cloud-native control plane — in Rust, with zero JavaScript, and with eBPF tracing from day one."</p>
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
                            <strong class="one-pager-col-label">"Core"</strong>
                            <p>"Rust, WebAssembly/wasm-bindgen, Linux kernel/syscalls"</p>
                        </div>
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Orchestration"</strong>
                            <p>"Kubernetes CRDs, controller-runtime, Helm"</p>
                        </div>
                        <div class="one-pager-col">
                            <strong class="one-pager-col-label">"Observability"</strong>
                            <p>"eBPF CO-RE, OpenTelemetry, Prometheus"</p>
                        </div>
                    </div>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"What I am looking for"</h2>
                    <ul class="contact-wish-list">
                        <li><span class="contact-arrow">"→"</span>" Junior Platform Engineer, DevOps Engineer, or Systems Engineer roles."</li>
                        <li><span class="contact-arrow">"→"</span>" Teams utilizing Kubernetes (k8s) for container orchestration and cloud-native delivery."</li>
                        <li><span class="contact-arrow">"→"</span>" Environments scaling through Infrastructure as Code (Terraform, Pulumi) and automated CI/CD pipelines."</li>
                        <li><span class="contact-arrow">"→"</span>" Engineering-centric cultures focused on Linux internals, developer experience (DevEx), and system reliability."</li>
                        <li><span class="contact-arrow">"→"</span>" Opportunities based in Oklahoma City or Remote-friendly organizations."</li>
                    </ul>
                </section>

                <section class="one-pager-section">
                    <h2 class="one-pager-h2">"Education + Certs"</h2>
                    <p>"BS Information Technology — Central Washington University"</p>
                    <p class="one-pager-certs">"Pursuing: CKA · CKAD · RHCSA"</p>
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
