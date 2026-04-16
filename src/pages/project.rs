use crate::components::ComponentErrorFallback;
use crate::data::{find_project, get_infrastructure_fleet, ProjectDetail};
use crate::error::AppError;
use crate::utils::sanitize_slug;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::{use_params_map, A};

// ============================================================
//   V2 STRUCTURED DETAIL RENDER
// ============================================================
//
// FAANG-grade project page layout:
//   §1 Hero metrics strip + status badge
//   §2 Problem & Context
//   §3 Constraints (in / out)
//   §4 Architectural approach + diagram
//   §5 Key decisions (ADR-style)
//   §6 Implementation highlights (annotated code)
//   §7 Outcomes (baseline → result with method)
//   §8 Lessons learned
//   §9 Artifact links
//
// Triggered when ProjectDetail.is_structured() is true.

fn render_v2_detail(d: ProjectDetail) -> impl IntoView {
    let status = d.status_label.clone();
    let hero_metrics = d.hero_metrics.clone();
    let problem = d.problem.clone().unwrap_or_default();
    let constraints_in = d.constraints_in.clone();
    let constraints_out = d.constraints_out.clone();
    let approach = d.approach.clone().unwrap_or_default();
    let approach_diagram = d.approach_diagram_src.clone();
    let decisions = d.decisions.clone();
    let highlights = d.highlights.clone();
    let outcomes = d.outcomes.clone();
    let lessons = d.lessons.clone();
    let artifact_links = d.artifact_links.clone();

    view! {
        <article class="pd-v2">
            // §1  Hero metrics + status badge
            {if status.is_some() || !hero_metrics.is_empty() {
                view! {
                    <section class="pd-v2-hero-strip" aria-label="Project at a glance">
                        {status.clone().map(|s| view! {
                            <span class="pd-v2-status-badge">{s}</span>
                        })}
                        {if !hero_metrics.is_empty() {
                            view! {
                                <div class="pd-v2-metrics">
                                    {hero_metrics.into_iter().map(|m| {
                                        let color = m.color.clone().unwrap_or_else(|| "var(--accent-cyan)".to_string());
                                        view! {
                                            <div class="pd-v2-metric">
                                                <span class="pd-v2-metric-value" style=format!("color:{}", color)>{m.value}</span>
                                                <span class="pd-v2-metric-label">{m.label}</span>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        } else { view! { <span></span> }.into_view() }}
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §2  Problem & Context
            {if !problem.is_empty() {
                view! {
                    <section class="pd-v2-section" id="problem">
                        <h2 class="pd-v2-heading">"§ Problem & Context"</h2>
                        <p class="pd-v2-prose">{problem}</p>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §3  Constraints
            {if !constraints_in.is_empty() || !constraints_out.is_empty() {
                view! {
                    <section class="pd-v2-section" id="constraints">
                        <h2 class="pd-v2-heading">"§ Constraints & Scope"</h2>
                        <div class="pd-v2-constraints-grid">
                            <div class="pd-v2-constraints-col">
                                <h3 class="pd-v2-subheading pd-v2-in">"In scope"</h3>
                                <ul class="pd-v2-list">
                                    {constraints_in.into_iter().map(|c| view! { <li>{c}</li> }).collect_view()}
                                </ul>
                            </div>
                            <div class="pd-v2-constraints-col">
                                <h3 class="pd-v2-subheading pd-v2-out">"Out of scope"</h3>
                                <ul class="pd-v2-list">
                                    {constraints_out.into_iter().map(|c| view! { <li>{c}</li> }).collect_view()}
                                </ul>
                            </div>
                        </div>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §4  Architectural Approach + diagram
            {if !approach.is_empty() || approach_diagram.is_some() {
                view! {
                    <section class="pd-v2-section" id="approach">
                        <h2 class="pd-v2-heading">"§ Architectural Approach"</h2>
                        {approach_diagram.clone().map(|src| view! {
                            <div class="pd-v2-diagram">
                                <img src=src alt="Architecture diagram" loading="lazy"/>
                            </div>
                        })}
                        {if !approach.is_empty() {
                            view! { <p class="pd-v2-prose">{approach}</p> }.into_view()
                        } else { view! { <span></span> }.into_view() }}
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §5  Key Decisions (ADR cards)
            {if !decisions.is_empty() {
                view! {
                    <section class="pd-v2-section" id="decisions">
                        <h2 class="pd-v2-heading">"§ Key Decisions"</h2>
                        <div class="pd-v2-decisions">
                            {decisions.into_iter().map(|d| view! {
                                <article class="pd-v2-decision">
                                    <h3 class="pd-v2-decision-title">{d.title}</h3>
                                    <dl class="pd-v2-decision-body">
                                        <dt>"Options considered"</dt>
                                        <dd>
                                            <ul class="pd-v2-decision-options">
                                                {d.options_considered.into_iter().map(|o| view! { <li>{o}</li> }).collect_view()}
                                            </ul>
                                        </dd>
                                        <dt>"Chose"</dt>
                                        <dd class="pd-v2-decision-chose">{d.chose}</dd>
                                        <dt>"Because"</dt>
                                        <dd>{d.because}</dd>
                                        <dt>"Tradeoff accepted"</dt>
                                        <dd class="pd-v2-decision-tradeoff">{d.tradeoff}</dd>
                                    </dl>
                                </article>
                            }).collect_view()}
                        </div>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §6  Implementation Highlights (annotated code)
            {if !highlights.is_empty() {
                view! {
                    <section class="pd-v2-section" id="highlights">
                        <h2 class="pd-v2-heading">"§ Implementation Highlights"</h2>
                        <div class="pd-v2-highlights">
                            {highlights.into_iter().map(|h| view! {
                                <figure class="pd-v2-highlight">
                                    <figcaption class="pd-v2-highlight-caption">
                                        <span class="pd-v2-highlight-filename">{h.filename}</span>
                                        <span class="pd-v2-highlight-lang">{h.lang}</span>
                                    </figcaption>
                                    <pre class="pd-v2-code"><code>{h.code}</code></pre>
                                    <p class="pd-v2-highlight-why">
                                        <strong>"Why it matters:"</strong>" "{h.why}
                                    </p>
                                </figure>
                            }).collect_view()}
                        </div>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §7  Outcomes (baseline → result with method)
            {if !outcomes.is_empty() {
                view! {
                    <section class="pd-v2-section" id="outcomes">
                        <h2 class="pd-v2-heading">"§ Outcomes"</h2>
                        <div class="pd-v2-outcomes">
                            {outcomes.into_iter().map(|o| view! {
                                <article class="pd-v2-outcome">
                                    <h3 class="pd-v2-outcome-metric">{o.metric}</h3>
                                    <div class="pd-v2-outcome-delta">
                                        <div class="pd-v2-outcome-baseline">
                                            <span class="pd-v2-outcome-kicker">"Baseline"</span>
                                            <span class="pd-v2-outcome-text">{o.baseline}</span>
                                        </div>
                                        <span class="pd-v2-outcome-arrow" aria-hidden="true">"→"</span>
                                        <div class="pd-v2-outcome-result">
                                            <span class="pd-v2-outcome-kicker">"Result"</span>
                                            <span class="pd-v2-outcome-text">{o.result}</span>
                                        </div>
                                    </div>
                                    <p class="pd-v2-outcome-method">
                                        <strong>"Method:"</strong>" "{o.method}
                                    </p>
                                </article>
                            }).collect_view()}
                        </div>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §8  Lessons
            {if !lessons.is_empty() {
                view! {
                    <section class="pd-v2-section" id="lessons">
                        <h2 class="pd-v2-heading">"§ Lessons & What I'd Do Differently"</h2>
                        <ol class="pd-v2-lessons">
                            {lessons.into_iter().map(|l| view! { <li class="pd-v2-lesson">{l}</li> }).collect_view()}
                        </ol>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}

            // §9  Artifacts
            {if !artifact_links.is_empty() {
                view! {
                    <section class="pd-v2-section" id="artifacts">
                        <h2 class="pd-v2-heading">"§ Artifacts"</h2>
                        <ul class="pd-v2-artifacts">
                            {artifact_links.into_iter().map(|a| {
                                let target = if a.external { "_blank" } else { "_self" };
                                let rel = if a.external { "noopener noreferrer" } else { "" };
                                view! {
                                    <li>
                                        <a href=a.url target=target rel=rel class="pd-v2-artifact-link">
                                            <span>{a.label}</span>
                                            <span aria-hidden="true">" →"</span>
                                        </a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    </section>
                }.into_view()
            } else { view! { <span></span> }.into_view() }}
        </article>
    }
}
#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    // Defensive: project pages MUST be scrollable. Release any stale
    // body scroll-lock from an overlay that didn't reset its signal.
    // Three unlock paths: immediate sync, next animation frame, next-tick
    // timeout. At least one runs after any pending reactive effect.
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
                                    {move || detail.get().map(|res| res.map(|d| {
                                        // V2 structured render when the JSON carries FAANG-grade
                                        // fields (problem/decisions/outcomes/…); otherwise fall
                                        // back to the legacy inner_html content string.
                                        if d.is_structured() {
                                            render_v2_detail(d).into_view()
                                        } else {
                                            view! {
                                                <article class="pd-article" inner_html=d.content></article>
                                            }.into_view()
                                        }
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
                    <Meta name="description" content=format!("{} — technical documentation and implementation notes.", p.subtitle)/>
                    <main id="main-content" class="min-h-screen pt-16 pb-24">
                        <div class="pd-container">
                            <nav class="pd-breadcrumb-row" aria-label="Breadcrumb">
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
                                    {p.tech_stack.iter().map(|tech| view! { <span class="pd-pill">{*tech}</span> }).collect_view()}
                                </div>
                            </header>
                            <ErrorBoundary fallback=|errors| view! { <ComponentErrorFallback errors/> }>
                                <Suspense fallback=move || view! {
                                    <div class="font-mono text-[var(--text-muted)] py-16 text-center">"Loading…"</div>
                                }>
                                    {move || docs.get().map(|res| res.map(|d| view! {
                                        <article class="pd-article mb-8" inner_html=d.content></article>
                                        <A href=format!("/project/{}", p.slug) class="pd-docs-back-cta">"View case study →"</A>
                                    }))}
                                </Suspense>
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

                            <A href=format!("/project/{}", p.slug) class="pd-docs-back-cta">"View case study →"</A>
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
