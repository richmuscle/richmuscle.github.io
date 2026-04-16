use super::{meta_strip, surface_tabs, Surface};
use crate::data::{find_project, DemoDetail};
use crate::error::AppError;
use crate::utils::sanitize_slug;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::{use_params_map, A};

// ============================================================
//   V2 STRUCTURED DEMO RENDER
// ============================================================
//
// Enterprise demo layout: hero media → narration cues (timestamps)
// → verification (criterion/method/observed) → reproduce steps →
// output snippets (real terminal output) → not-demonstrated honesty list.

fn render_v2_demo(d: DemoDetail, p_slug: &'static str) -> impl IntoView {
    let hero_src = d.hero_media_src.clone();
    let hero_caption = d.hero_caption.clone();
    let narration = d.narration.clone();
    let verification = d.verification.clone();
    let reproduce = d.reproduce.clone();
    let output_snippets = d.output_snippets.clone();
    let not_demonstrated = d.not_demonstrated.clone();
    let last_updated = d.last_updated.clone();
    let reading_time = d.reading_time_minutes;
    let status_label = d.status_label.clone();

    view! {
        <article class="pd-v2">
            {meta_strip(last_updated, reading_time, status_label)}

            // § Hero media (mandatory for V2 demo)
            {hero_src.map(|src| view! {
                <section class="pd-v2-section" id="hero">
                    <div class="pd-v2-diagram">
                        <img src=src alt="Operational demo flow" loading="lazy"/>
                    </div>
                    {hero_caption.clone().map(|c| view! {
                        <p class="pd-v2-prose" style="text-align:center; margin-top:13px;">{c}</p>
                    })}
                </section>
            })}

            // § Narration cues
            {if !narration.is_empty() {
                view! {
                    <section class="pd-v2-section" id="narration">
                        <h2 class="pd-v2-heading">"§ What You're Seeing"</h2>
                        <ol class="pd-v2-narration">
                            {narration.into_iter().map(|n| view! {
                                <li class="pd-v2-narration-item">
                                    <span class="pd-v2-narration-ts">{n.timestamp}</span>
                                    <span class="pd-v2-narration-caption">{n.caption}</span>
                                </li>
                            }).collect_view()}
                        </ol>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // § Verification checks
            {if !verification.is_empty() {
                view! {
                    <section class="pd-v2-section" id="verification">
                        <h2 class="pd-v2-heading">"§ Verification Criteria"</h2>
                        <p class="pd-v2-prose" style="max-width:72ch;">"Every claim below has an explicit pass/fail check, the command used to measure it, and what was actually observed."</p>
                        <div class="pd-v2-verifications">
                            {verification.into_iter().map(|v| view! {
                                <article class="pd-v2-verification">
                                    <h3 class="pd-v2-verification-criterion">{v.criterion}</h3>
                                    <dl class="pd-v2-verification-body">
                                        <dt>"Method"</dt>
                                        <dd><pre class="pd-v2-code-inline"><code>{v.method}</code></pre></dd>
                                        <dt>"Observed"</dt>
                                        <dd class="pd-v2-verification-observed">{v.observed}</dd>
                                    </dl>
                                </article>
                            }).collect_view()}
                        </div>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // § Reproduce locally
            {reproduce.map(|r| {
                let prereqs = r.prereqs.clone();
                let env_vars = r.env_vars.clone();
                let steps = r.steps.clone();
                view! {
                    <section class="pd-v2-section" id="reproduce">
                        <h2 class="pd-v2-heading">"§ Reproduce Locally"</h2>
                        {if !prereqs.is_empty() {
                            view! {
                                <h3 class="pd-v2-subheading pd-v2-in">"Prerequisites"</h3>
                                <ul class="pd-v2-list">
                                    {prereqs.into_iter().map(|p| view! { <li>{p}</li> }).collect_view()}
                                </ul>
                            }.into_view()
                        } else { view! { <span></span> }.into_view() }}
                        {if !env_vars.is_empty() {
                            view! {
                                <h3 class="pd-v2-subheading pd-v2-in">"Environment"</h3>
                                <pre class="pd-v2-code pd-v2-code-plain"><code>{env_vars.join("\n")}</code></pre>
                            }.into_view()
                        } else { view! { <span></span> }.into_view() }}
                        {if !steps.is_empty() {
                            view! {
                                <h3 class="pd-v2-subheading pd-v2-in">"Steps"</h3>
                                <pre class="pd-v2-code pd-v2-code-plain"><code>{steps.join("\n")}</code></pre>
                            }.into_view()
                        } else { view! { <span></span> }.into_view() }}
                    </section>
                }
            })}

            // § Output snippets (real terminal output, verbatim)
            {if !output_snippets.is_empty() {
                view! {
                    <section class="pd-v2-section" id="output">
                        <h2 class="pd-v2-heading">"§ Captured Output"</h2>
                        <div class="pd-v2-highlights">
                            {output_snippets.into_iter().map(|s| view! {
                                <figure class="pd-v2-highlight">
                                    <figcaption class="pd-v2-highlight-caption">
                                        <span class="pd-v2-highlight-filename">{s.label}</span>
                                        <span class="pd-v2-highlight-lang">{s.lang}</span>
                                    </figcaption>
                                    <pre class="pd-v2-code"><code>{s.output}</code></pre>
                                </figure>
                            }).collect_view()}
                        </div>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // § Not demonstrated (honesty)
            {if !not_demonstrated.is_empty() {
                view! {
                    <section class="pd-v2-section" id="not-demonstrated">
                        <h2 class="pd-v2-heading">"§ What This Demo Does Not Show"</h2>
                        <p class="pd-v2-prose">"Explicit credibility builder — the limitations of this walkthrough. Each item is either deferred work, a future-production concern, or out-of-scope for a lab demo."</p>
                        <ul class="pd-v2-list">
                            {not_demonstrated.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}
                        </ul>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // Footer nav
            <section class="pd-v2-section pd-v2-footer-nav">
                <A href=format!("/project/{}", p_slug) class="pd-docs-back-cta">"← Case study"</A>
                <A href=format!("/project/{}/docs", p_slug) class="pd-docs-back-cta">"Docs →"</A>
            </section>
        </article>
    }
}

#[component]
pub fn ProjectDemoPage() -> impl IntoView {
    #[cfg(not(feature = "ssr"))]
    {
        crate::utils::set_body_scroll_lock(false);
        use gloo_timers::callback::Timeout;
        Timeout::new(0, || crate::utils::set_body_scroll_lock(false)).forget();
        Timeout::new(50, || crate::utils::set_body_scroll_lock(false)).forget();
    }

    let params = use_params_map();
    let slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
    };
    let project = create_memo(move |_| find_project(&slug()));

    // V2 demo resource — fetches /demos/{slug}.json. 404 (project without
    // a demo JSON) resolves to an error that the legacy fallback catches
    // via is_populated() check.
    let demo_data = create_resource(slug, |s| async move {
        #[cfg(feature = "ssr")]
        {
            let _ = s;
            return Err::<DemoDetail, AppError>(AppError::logic("ssr build skips fetch"));
        }
        #[cfg(not(feature = "ssr"))]
        {
            if s.is_empty() {
                return Err(AppError::logic("empty slug"));
            }
            let url = format!("/demos/{}.json", s);
            let resp = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| AppError::fetch(e.to_string()))?;
            if !resp.ok() {
                return Err(AppError::fetch(format!("no demo JSON for {}", s)));
            }
            resp.json::<DemoDetail>()
                .await
                .map_err(|e| AppError::parse(e.to_string()))
        }
    });

    view! {
        {move || match project.get() {
            None => view! {
                <main class="min-h-screen pt-28 flex items-center justify-center">
                    <div class="text-center font-mono">
                        <p class="text-6xl text-[var(--text-muted)] mb-4">"404"</p>
                        <a href="/" class="text-[#22d3ee] text-[13px] hover:underline">"← Return to base"</a>
                    </div>
                </main>
            }.into_view(),
            Some(p) => {
                struct DemoStep { title: &'static str, body: &'static str }

                let (demo_header, video_caption, steps) = match p.slug {
                    "zero-trust-networking" => (
                        "Operational Demo: ZTNA Identity Enforcement",
                        "Demonstrating stateless tunnel establishment and instant access revocation via Active Directory.",
                        vec![
                            DemoStep { title: "1) Identity Verification (Out-of-Band LDAP Check)", body: "Perform an LDAP group membership query for 'Network-Admins' before establishing data-plane access. Verification succeeds only when the user is explicitly authorized." },
                            DemoStep { title: "2) Tunnel Initiation (WireGuard Handshake)", body: "Initiate the WireGuard handshake (UDP). The 'Dark Node' remains inaccessible until the authenticated peer is authorized via the out-of-band AD/LDAP gate." },
                            DemoStep { title: "3) Policy Enforcement (403/Drop Demonstration)", body: "Attempt to reach the micro-segmented database tier from the Admin peer. Expect a 403 response and dropped connectivity due to micro-segmentation rules." },
                            DemoStep { title: "4) Access Revocation (AD Disablement)", body: "Disable the user account in AD. Show that tunnel access terminates instantly and the permitted route is withdrawn immediately." },
                            DemoStep { title: "5) Verification (Prove Success End-to-End)", body: "Verify: (a) LDAP authorization gates the tunnel, (b) DB tier access is blocked, (c) post-disable connectivity is revoked and no route remains for the disabled identity." },
                        ],
                    ),
                    "terraform-gcp" => (
                        "Operational Demo: Deterministic Cloud Provisioning",
                        "Demonstrating modular deployment and the identification/reversal of configuration drift.",
                        vec![
                            DemoStep { title: "1) Plan & Apply (GCS Backend + State Locking)", body: "Execute Terraform with the GCS remote backend. Confirm state locking prevents concurrent execution and preserves a single source of truth." },
                            DemoStep { title: "2) Injected Drift (Out-of-Band Console Change)", body: "Manually modify a firewall rule or IAM policy in the GCP Console to create an unauthorized configuration delta." },
                            DemoStep { title: "3) Drift Detection (terraform plan Diff)", body: "Run 'terraform plan' and confirm it identifies the unauthorized change as a drift delta against the hardened desired state." },
                            DemoStep { title: "4) Reconciliation (terraform apply Back to Baseline)", body: "Re-run 'terraform apply' to revert the environment to the hardened baseline that enforces deterministic, policy-validated configuration." },
                            DemoStep { title: "5) Verification (Success Criteria + No-Diff Check)", body: "Verify that the post-reconciliation 'terraform plan' reports zero changes (no diff) and the backend state remains consistent and locked during the apply cycle." },
                        ],
                    ),
                    "linux-admin-scripting" => (
                        "Operational Demo: Idempotent Lifecycle Automation",
                        "Demonstrating automated user provisioning and CIS-standard system hardening.",
                        vec![
                            DemoStep { title: "1) Provisioning Engine (Entropy Credentials + RBAC)", body: "Run the Bash automation to onboard a new user with entropy-based credentials. Apply RBAC permissions and seed localized environment variables as specified by the desired state." },
                            DemoStep { title: "2) Idempotency Check (Strict Convergence)", body: "Re-run the same script. Demonstrate zero changes: no duplicate users, no duplicate mounts, and no repeated side effects when the system already matches the desired configuration." },
                            DemoStep { title: "3) Hardening Verification (sysctl + fstab Flags)", body: "Execute the security audit script to validate /etc/sysctl.conf network hardening parameters and confirm /etc/fstab mount options like noexec/nosuid." },
                            DemoStep { title: "4) Audit Log (Structured SIEM Ingestion)", body: "Tail /var/log/syslog to display structured log entries emitted by logger from the automation, ensuring centralized monitoring and auditability." },
                            DemoStep { title: "5) Verification (Golden Baseline + Audit Consistency)", body: "Verify: (a) strict-mode execution prevents silent failure, (b) idempotency converges without diffs, and (c) syslog shows expected audit trails for the automated actions." },
                        ],
                    ),
                    "monitoring-observability" => (
                        "Operational Demo: Multi-Tier Alerting & Log Enrichment",
                        "Demonstrating anomaly detection in Prometheus and root-cause analysis in Kibana.",
                        vec![
                            DemoStep { title: "1) Simulated Stress (Trigger a Controlled Incident)", body: "Trigger a resource exhaustion event such as a simulated memory leak or network spike to generate measurable metric anomalies and correlated logs." },
                            DemoStep { title: "2) PromQL Alerting (Delta/Rate-of-Change)", body: "Show Prometheus AlertManager firing using a Rate-of-Change (Delta) threshold to detect anomaly onset before system degradation becomes widespread." },
                            DemoStep { title: "3) Dashboard Pivot (Grafana → SLO Degradation)", body: "Pivot in Grafana to visualize correlated technical metrics and demonstrate SLO/SLI impact as the incident progresses and stabilizes." },
                            DemoStep { title: "4) Forensic Analysis (ELK Enriched Logs)", body: "Use ELK/Kibana to filter enriched logs and identify the specific process/IP responsible for the anomaly from the structured event stream." },
                            DemoStep { title: "5) Verification (Close the Loop with Proof)", body: "Verify success end-to-end: confirm alert resolution after mitigation, SLO recovery on dashboards, and log search evidence matching the incident window." },
                        ],
                    ),
                    _ => (
                        "Operational Demo",
                        "Demonstrating project execution and verification outcomes.",
                        vec![],
                    ),
                };

                view! {
                    <Title text=format!("Demo: {} | Richard Mussell", p.title)/>
                    <Meta name="description" content=format!("{} — operational demo walkthrough.", p.subtitle)/>
                    <main id="main-content" class="min-h-screen pt-16 pb-24 page-enter">
                        <div class="pd-container">
                            <nav class="pd-breadcrumb-row" aria-label="Breadcrumb">
                                <a href="/">"Portfolio"</a>
                                <span aria-hidden="true">" / "</span>
                                <a href=format!("/project/{}", p.slug)>{p.title}</a>
                                <span aria-hidden="true">" / Demo"</span>
                            </nav>
                            <header class="pd-header">
                                <div class="pd-category-label" style=format!("color:{}", p.category.accent())>"DEMO"</div>
                                <h1 class="pd-title">{demo_header}</h1>
                                <p class="pd-subtitle">{video_caption}</p>
                                <div class="pd-tech-pills">
                                    {p.tech_stack.iter().map(|tech| view! { <span class="pd-pill">{*tech}</span> }).collect_view()}
                                </div>
                            </header>

                            {surface_tabs(p.slug, Surface::Demo)}

                            // V2 demo if /demos/{slug}.json exists AND is populated;
                            // otherwise render the legacy hardcoded walkthrough steps.
                            {
                                let p_slug = p.slug;
                                move || match demo_data.get() {
                                    Some(Ok(d)) if d.is_populated() => render_v2_demo(d, p_slug).into_view(),
                                    _ => view! {
                                        <div class="pd-section" style="margin-top: 2.25rem;">
                                            <div class="video-placeholder">"Documented operational scenario — walkthrough steps below."</div>
                                        </div>
                                        <div class="pd-section">
                                            <h2 class="pd-section-heading">Technical Walkthrough (Operational Scenario)</h2>
                                            <div class="pd-challenges">
                                                {steps.iter().map(|s| view! {
                                                    <div class="pd-challenge">
                                                        <div class="pd-challenge-title">{s.title}</div>
                                                        <p class="pd-challenge-body">{s.body}</p>
                                                    </div>
                                                }).collect_view()}
                                            </div>
                                        </div>
                                        <A href=format!("/project/{}", p_slug) class="pd-docs-back-cta">"View case study →"</A>
                                        <footer class="pd-footer-nav mt-16 pt-8 border-t border-subtle">
                                            <a href=format!("/project/{}", p_slug) class="pd-footer-btn">"Case Study"</a>
                                            <a href=format!("/project/{}/docs", p_slug) class="pd-footer-btn">"Documentation"</a>
                                        </footer>
                                    }.into_view(),
                                }
                            }
                        </div>
                    </main>
                }.into_view()
            }
        }}
    }
}
