use crate::components::ComponentErrorFallback;
use crate::data::{find_project, get_infrastructure_fleet};
use crate::error::AppError;
use crate::utils::sanitize_slug;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::{use_params_map, A};
#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
    };
    let project = create_memo(move |_| find_project(&slug()));

    let detail = create_resource(slug, |s| async move {
        #[cfg(feature = "ssr")]
        {
            let _ = s;
            return Err::<crate::data::ProjectDetail, AppError>(AppError::logic(
                "ssr build skips fetch",
            ));
        }
        #[cfg(not(feature = "ssr"))]
        {
            if s.is_empty() {
                return Err(AppError::logic("empty slug"));
            }
            let url = format!("/projects/{}.json", s);
            let resp = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| AppError::fetch(e.to_string()))?;
            resp.json::<crate::data::ProjectDetail>()
                .await
                .map_err(|e| AppError::parse(e.to_string()))
        }
    });

    let related_row = create_memo(move |_| {
        let current = slug();
        let all = get_infrastructure_fleet();
        let cat = all
            .iter()
            .find(|p| p.slug == current.as_str())
            .map(|p| p.category.clone());
        let mut same_cat: Vec<(String, String, String)> = all
            .iter()
            .filter(|p| p.slug != current.as_str() && Some(&p.category) == cat.as_ref())
            .take(2)
            .map(|p| {
                (
                    p.slug.to_string(),
                    p.title.to_string(),
                    p.category.label().to_string(),
                )
            })
            .collect();
        if same_cat.len() < 2 {
            let existing: std::collections::HashSet<String> =
                same_cat.iter().map(|(s, _, _)| s.clone()).collect();
            let others: Vec<_> = all
                .iter()
                .filter(|p| !existing.contains(p.slug))
                .take(2 - same_cat.len())
                .map(|p| {
                    (
                        p.slug.to_string(),
                        p.title.to_string(),
                        p.category.label().to_string(),
                    )
                })
                .collect();
            same_cat.extend(others);
        }
        same_cat
    });

    let title = move || {
        format!(
            "{} · Richard Mussell",
            project.get().map(|p| p.title).unwrap_or("")
        )
    };
    let desc = move || {
        project
            .get()
            .map(|p| p.subtitle.to_string())
            .unwrap_or_default()
    };
    view! {
        <Title text=title/>
        <Meta name="description" content=desc/>
        {move || match project.get() {
            None => view! {
                <main id="main-content" class="min-h-screen pt-28">
                    <div class="max-w-6xl mx-auto px-4 sm:px-8 pt-12">
                        <div class="skeleton-title-area mb-8">
                            <div class="skeleton-bar" style="width:60%;"></div>
                            <div class="skeleton-bar" style="width:40%;"></div>
                        </div>
                        <div class="skeleton-block"></div>
                    </div>
                    <div class="text-center font-mono mt-16">
                        <p class="text-6xl text-[var(--text-muted)] mb-4">"404"</p>
                        <p class="text-[15px] text-[var(--text-muted)] mb-6 leading-7">"Project not found."</p>
                        <a href="/" aria-label="Back to all projects" class="text-[#22d3ee] text-[13px] hover:text-[var(--cyan-mid)] transition-colors duration-150"><span aria-hidden="true">"← "</span>"Return to base"</a>
                    </div>
                </main>
            }.into_view(),
            Some(p) => {
                let cat_accent = p.category.accent();
                view! {
                    <main id="main-content" class="min-h-screen pt-16 pb-24 page-enter">
                        <div class="pd-container">
                            <nav class="pd-breadcrumb-row" aria-label="Breadcrumb">
                                <div>
                                    <a href="/">"Portfolio"</a>
                                    <span aria-hidden="true">" / "</span>
                                    <span>{p.title}</span>
                                </div>
                            </nav>

                            <header class="pd-header">
                                <div class="pd-category-label" style=format!("color:{}", cat_accent)>{p.category.label()}</div>
                                <h1 class="pd-title">{p.title}</h1>
                                <p class="pd-subtitle">{p.subtitle}</p>
                                <div class="pd-tech-pills">
                                    {p.tech_stack.iter().map(|tech| view! { <span class="pd-pill">{*tech}</span> }).collect_view()}
                                </div>
                            </header>

                            <ErrorBoundary fallback=|errors| view! { <ComponentErrorFallback errors/> }>
                                <Suspense fallback=move || view! {
                                    <div class="font-mono text-[var(--text-muted)] py-16 text-center">"Loading…"</div>
                                }>
                                    {move || detail.get().map(|res| res.map(|d| view! {
                                        // No max-width on the article wrapper — child pd-section
                                        // blocks (stat-bar, before-after, challenges) need full
                                        // container width. Child <p> elements get their own
                                        // reading width via .pd-body-text in the CSS.
                                        <article class="pd-article" inner_html=d.content></article>
                                    }))}
                                </Suspense>
                            </ErrorBoundary>

                            <div class="pd-section pd-footer-actions flex items-center gap-3 flex-wrap mt-12">
                                <A href=format!("/project/{}/docs", p.slug) class="pd-footer-btn">
                                    <span aria-hidden="true">"📄"</span>"View Documentation"
                                </A>
                                <A href=format!("/project/{}/demo", p.slug) class="pd-footer-btn">
                                    <span aria-hidden="true">"▶"</span>"Watch Demo"
                                </A>
                            </div>

                            {move || {
                                let list = related_row.get();
                                if list.is_empty() { return view! { <span></span> }.into_view(); }
                                view! {
                                    <div class="pd-section">
                                        <h2 class="pd-section-heading">"Related Projects"</h2>
                                        <div class="related-projects-grid">
                                            {list.into_iter().map(|(s, title, label)| {
                                                let href = format!("/project/{}", sanitize_slug(&s));
                                                view! {
                                                    <a href=href class="related-project-mini-card">
                                                        <span class="related-project-mini-title">{title}</span>
                                                        <span class="related-project-mini-label">{label}</span>
                                                        <span class="related-project-mini-cta">"View →"</span>
                                                    </a>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_view()
                            }}

                            <div class="flex items-center gap-6 pt-8 border-t border-subtle" style="margin-top: 24px;">
                                <A href="/" class="text-[13px] font-mono text-[var(--text-muted)] transition-colors pd-back-link">"← All Projects"</A>
                            </div>
                        </div>
                    </main>
                }.into_view()
            }
        }}
    }
}

// ============================================================
//  PROJECT DOCS PAGE  (structure preserved, content live)
// ============================================================

#[component]
pub fn ProjectDocsPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
    };
    let project = create_memo(move |_| find_project(&slug()));
    let docs = create_resource(slug, |s| async move {
        #[cfg(feature = "ssr")]
        {
            let _ = s;
            return Err::<crate::data::ProjectDetail, AppError>(AppError::logic(
                "ssr build skips fetch",
            ));
        }
        #[cfg(not(feature = "ssr"))]
        {
            if s.is_empty() {
                return Err(AppError::logic("empty slug"));
            }
            let url = format!("/docs/{}.json", s);
            let resp = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| AppError::fetch(e.to_string()))?;
            resp.json::<crate::data::ProjectDetail>()
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
                view! {
                    <Title text=format!("Docs: {} | Richard Mussell", p.title)/>
                    <main id="main-content" class="min-h-screen pt-16 pb-24">
                        <div class="pd-container max-w-3xl mx-auto px-6">
                            <nav class="pd-breadcrumb-row">
                                <a href="/">"Portfolio"</a>
                                <span aria-hidden="true">" / "</span>
                                <a href=format!("/project/{}", p.slug)>{p.title}</a>
                                <span aria-hidden="true">" / Docs"</span>
                            </nav>
                            <header class="pd-header mb-12">
                                <div class="pd-category-label" style=format!("color:{}", p.category.accent())>{p.category.label()}</div>
                                <h1 class="pd-title">{p.title}</h1>
                                <p class="pd-subtitle">{p.subtitle}</p>
                                <div class="pd-tech-pills">
                                    {p.tech_stack.iter().map(|tech| view! { <span>{*tech}</span> }).collect_view()}
                                </div>
                            </header>
                            <ErrorBoundary fallback=|errors| view! { <ComponentErrorFallback errors/> }>
                                {move || match docs.get() {
                                    None => Ok(view! {
                                        <p class="pd-body-text mb-8">"Loading…"</p>
                                    }.into_view()),
                                    Some(Ok(d)) => Ok(view! {
                                        <article class="pd-body-text mb-8" inner_html=d.content></article>
                                        <A href=format!("/project/{}", p.slug) class="text-[#22d3ee] font-mono text-[13px] hover:underline">"View case study →"</A>
                                    }.into_view()),
                                    Some(Err(e)) => Err::<leptos::View, AppError>(e),
                                }}
                            </ErrorBoundary>
                        </div>
                    </main>
                }.into_view()
            }
        }}
    }
}

//  PROJECT DEMO PAGE
// ============================================================

#[component]
pub fn ProjectDemoPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
    };
    let project = create_memo(move |_| find_project(&slug()));

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
                    <main id="main-content" class="min-h-screen pt-16 pb-24 bg-[var(--bg-base)] page-enter">
                        <div class="pd-wrap pb-16 max-w-3xl mx-auto px-6">
                            <nav class="pd-breadcrumb" aria-label="Breadcrumb">
                                <a href="/">"Projects"</a>
                                <span aria-hidden="true">"/"</span>
                                <span>{p.title}</span>
                            </nav>
                            <p style="margin-top: -1rem; margin-bottom: 2rem;">
                                <a href="/" class="pd-back">"← Projects"</a>
                            </p>
                            <header class="pd-header">
                                <p class="pd-header-cat">"DEMO"</p>
                                <h1 class="pd-header-title">{demo_header}</h1>
                                <p class="pd-header-subtitle">{video_caption}</p>
                                <div class="pd-header-pills">
                                    {p.tech_stack.iter().map(|tech| view! { <span class="pd-pill">{*tech}</span> }).collect_view()}
                                </div>
                            </header>
                            <div class="pd-section" style="margin-top: 2.25rem;">
                                <div class="video-placeholder">"Documented operational scenario — walkthrough steps below."</div>
                            </div>

                            <div class="pd-section">
                                <h2 class="pd-section-heading">Technical Walkthrough (Operational Scenario)</h2>
                                <div class="pd-challenges">
                                    {steps.into_iter().map(|s| view! {
                                        <div class="pd-challenge">
                                            <div class="pd-challenge-title">{s.title}</div>
                                            <p class="pd-challenge-body">{s.body}</p>
                                        </div>
                                    }).collect_view()}
                                </div>
                            </div>

                            <A href=format!("/project/{}", p.slug) class="text-[#22d3ee] font-mono text-[13px] hover:underline mt-4 inline-block">"View case study →"</A>
                            <footer class="pd-footer-nav mt-16 pt-8 border-t border-[var(--border-subtle)]">
                                <a href=format!("/project/{}", p.slug) class="pd-footer-btn">"Case Study"</a>
                                <a href=format!("/project/{}/docs", p.slug) class="pd-footer-btn">"Documentation"</a>
                            </footer>
                        </div>
                    </main>
                }.into_view()
            }
        }}
    }
}

//  RESUME PAGE  — Systems Engineer persona
