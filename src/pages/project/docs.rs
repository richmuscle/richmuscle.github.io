use super::{meta_strip, surface_tabs, Surface};
use crate::components::ComponentErrorFallback;
use crate::data::{find_project, resolve_legacy_slug, DocsDetail};
use crate::error::AppError;
use crate::utils::sanitize_slug;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::{use_params_map, A};

// ============================================================
//   V2 STRUCTURED DOCS RENDER
// ============================================================
//
// Two-column layout at ≥1024px: sticky TOC sidebar | content.
// Single-column stack on narrower viewports.

fn render_v2_docs(d: DocsDetail, p_slug: &'static str) -> impl IntoView {
    let toc = d.toc.clone();
    let overview = d.overview.clone();
    let adrs = d.adrs.clone();
    let config_walkthrough = d.config_walkthrough.clone();
    let runbook = d.runbook.clone();
    let security = d.security.clone();
    let observability = d.observability.clone();
    let testing = d.testing.clone();
    let limitations = d.limitations.clone();
    let references = d.references.clone();
    let last_updated = d.last_updated.clone();
    let reading_time = d.reading_time_minutes;
    let status_label = d.status_label.clone();

    view! {
        {meta_strip(last_updated, reading_time, status_label)}
        <div class="pd-v2-docs-layout">
            // Sticky TOC (desktop only; hidden on mobile via CSS)
            <aside class="pd-v2-docs-toc" aria-label="Document table of contents">
                <p class="pd-v2-docs-toc-kicker">"Contents"</p>
                <ul class="pd-v2-docs-toc-list">
                    {toc.into_iter().map(|e| view! {
                        <li><a href=format!("#{}", e.id) class="pd-v2-docs-toc-link">{e.label}</a></li>
                    }).collect_view()}
                </ul>
                <div class="pd-v2-docs-toc-footer">
                    <A href=format!("/project/{}", p_slug) class="pd-docs-back-cta">"Case study →"</A>
                    <A href=format!("/project/{}/demo", p_slug) class="pd-docs-back-cta">"Demo →"</A>
                </div>
            </aside>

            <article class="pd-v2-docs-content">
                // § Overview
                {overview.map(|o| {
                    let desc = o.description.clone();
                    let components = o.components.clone();
                    view! {
                        <section class="pd-v2-section" id="overview">
                            <h2 class="pd-v2-heading">"§ System Overview"</h2>
                            <p class="pd-v2-prose">{desc}</p>
                            {if !components.is_empty() {
                                view! {
                                    <div class="pd-v2-components">
                                        {components.into_iter().map(|c| view! {
                                            <article class="pd-v2-component">
                                                <h3 class="pd-v2-component-name">{c.name}</h3>
                                                <p class="pd-v2-component-purpose">{c.purpose}</p>
                                                <dl class="pd-v2-component-specs">
                                                    {c.inputs.map(|v| view! { <dt>"Inputs"</dt><dd>{v}</dd> })}
                                                    {c.outputs.map(|v| view! { <dt>"Outputs"</dt><dd>{v}</dd> })}
                                                    {c.slo.map(|v| view! { <dt>"SLO"</dt><dd>{v}</dd> })}
                                                    {c.failure_mode.map(|v| view! { <dt>"Failure mode"</dt><dd>{v}</dd> })}
                                                </dl>
                                            </article>
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}
                        </section>
                    }
                })}

                // § ADRs
                {if !adrs.is_empty() {
                    view! {
                        <section class="pd-v2-section" id="adrs">
                            <h2 class="pd-v2-heading">"§ Design Decisions"</h2>
                            <div class="pd-v2-decisions">
                                {adrs.into_iter().map(|a| {
                                    let status_class = format!("pd-v2-adr-status pd-v2-adr-status-{}", a.status.to_lowercase());
                                    view! {
                                        <article class="pd-v2-decision">
                                            <header class="pd-v2-adr-header">
                                                <span class="pd-v2-adr-id">{a.id}</span>
                                                <span class=status_class>{a.status}</span>
                                            </header>
                                            <h3 class="pd-v2-decision-title">{a.title}</h3>
                                            <dl class="pd-v2-decision-body">
                                                <dt>"Context"</dt>
                                                <dd>{a.context}</dd>
                                                <dt>"Decision"</dt>
                                                <dd class="pd-v2-decision-chose">{a.decision}</dd>
                                                <dt>"Consequences"</dt>
                                                <dd>{a.consequences}</dd>
                                            </dl>
                                        </article>
                                    }
                                }).collect_view()}
                            </div>
                        </section>
                    }.into_view()
                } else { view! { <span></span> }.into_view() }}

                // § Config walkthrough
                {if !config_walkthrough.is_empty() {
                    view! {
                        <section class="pd-v2-section" id="config">
                            <h2 class="pd-v2-heading">"§ Configuration Reference"</h2>
                            <div class="pd-v2-highlights">
                                {config_walkthrough.into_iter().map(|h| view! {
                                    <figure class="pd-v2-highlight">
                                        <figcaption class="pd-v2-highlight-caption">
                                            <span class="pd-v2-highlight-filename">{h.filename}</span>
                                            <span class="pd-v2-highlight-lang">{h.lang}</span>
                                        </figcaption>
                                        <pre class="pd-v2-code"><code>{h.code}</code></pre>
                                        <p class="pd-v2-highlight-why"><strong>"Notes: "</strong>{h.why}</p>
                                    </figure>
                                }).collect_view()}
                            </div>
                        </section>
                    }.into_view()
                } else { view! { <span></span> }.into_view() }}

                // § Runbook
                {runbook.map(|r| {
                    let deploy = r.deploy.clone();
                    let rollback = r.rollback.clone();
                    let common_ops = r.common_ops.clone();
                    let incidents = r.incidents.clone();
                    view! {
                        <section class="pd-v2-section" id="runbook">
                            <h2 class="pd-v2-heading">"§ Runbook"</h2>

                            {if !deploy.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-in">"Deploy"</h3>
                                    <pre class="pd-v2-code pd-v2-code-plain"><code>{deploy.join("\n")}</code></pre>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !rollback.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-out">"Rollback"</h3>
                                    <pre class="pd-v2-code pd-v2-code-plain"><code>{rollback.join("\n")}</code></pre>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !common_ops.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-in">"Common Operations"</h3>
                                    <div class="pd-v2-runbook-ops">
                                        {common_ops.into_iter().map(|op| view! {
                                            <article class="pd-v2-runbook-op">
                                                <h4 class="pd-v2-runbook-op-title">{op.task}</h4>
                                                <ol class="pd-v2-runbook-op-steps">
                                                    {op.steps.into_iter().map(|s| view! { <li>{s}</li> }).collect_view()}
                                                </ol>
                                            </article>
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !incidents.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-out">"Incident Response"</h3>
                                    <div class="pd-v2-runbook-incidents">
                                        {incidents.into_iter().map(|i| view! {
                                            <article class="pd-v2-runbook-incident">
                                                <h4 class="pd-v2-runbook-incident-scenario">{i.scenario}</h4>
                                                <p class="pd-v2-runbook-incident-response">{i.response}</p>
                                            </article>
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}
                        </section>
                    }
                })}

                // § Security posture
                {security.map(|s| {
                    let threats = s.threat_model.clone();
                    let compliance = s.compliance.clone();
                    let unresolved = s.unresolved_risks.clone();
                    view! {
                        <section class="pd-v2-section" id="security">
                            <h2 class="pd-v2-heading">"§ Security Posture"</h2>

                            {if !threats.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-out">"Threat Model"</h3>
                                    <div class="pd-v2-threats">
                                        {threats.into_iter().map(|t| view! {
                                            <article class="pd-v2-threat">
                                                <p class="pd-v2-threat-threat"><strong>"Threat: "</strong>{t.threat}</p>
                                                <p class="pd-v2-threat-mitigation"><strong>"Mitigation: "</strong>{t.mitigation}</p>
                                            </article>
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !compliance.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-in">"Compliance"</h3>
                                    <div class="pd-v2-compliance">
                                        {compliance.into_iter().map(|c| view! {
                                            <div class="pd-v2-compliance-framework">
                                                <span class="pd-v2-compliance-name">{c.framework}</span>
                                                <ul class="pd-v2-compliance-controls">
                                                    {c.controls.into_iter().map(|ctl| view! { <li>{ctl}</li> }).collect_view()}
                                                </ul>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !unresolved.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-out">"Unresolved Risks"</h3>
                                    <ul class="pd-v2-list">
                                        {unresolved.into_iter().map(|r| view! { <li>{r}</li> }).collect_view()}
                                    </ul>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}
                        </section>
                    }
                })}

                // § Observability
                {observability.map(|o| {
                    let metrics = o.metrics.clone();
                    let alerts = o.alerts.clone();
                    let dashboards = o.dashboards.clone();
                    view! {
                        <section class="pd-v2-section" id="observability">
                            <h2 class="pd-v2-heading">"§ Observability"</h2>

                            {if !metrics.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-in">"Metrics"</h3>
                                    <ul class="pd-v2-obs-list">
                                        {metrics.into_iter().map(|m| view! {
                                            <li class="pd-v2-obs-item">
                                                <span class="pd-v2-obs-name">{m.name}</span>
                                                <span class="pd-v2-obs-threshold">{m.threshold}</span>
                                                <span class="pd-v2-obs-source">{m.source}</span>
                                            </li>
                                        }).collect_view()}
                                    </ul>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !alerts.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-out">"Alerts"</h3>
                                    <ul class="pd-v2-obs-list">
                                        {alerts.into_iter().map(|a| view! {
                                            <li class="pd-v2-obs-item">
                                                <span class="pd-v2-obs-name">{a.name}</span>
                                                <span class="pd-v2-obs-threshold">{a.condition}</span>
                                                <span class="pd-v2-obs-source">{a.action}</span>
                                            </li>
                                        }).collect_view()}
                                    </ul>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}

                            {if !dashboards.is_empty() {
                                view! {
                                    <h3 class="pd-v2-subheading pd-v2-in">"Dashboards"</h3>
                                    <ul class="pd-v2-list">
                                        {dashboards.into_iter().map(|d| view! { <li>{d}</li> }).collect_view()}
                                    </ul>
                                }.into_view()
                            } else { view! { <span></span> }.into_view() }}
                        </section>
                    }
                })}

                // § Testing
                {testing.map(|t| view! {
                    <section class="pd-v2-section" id="testing">
                        <h2 class="pd-v2-heading">"§ Testing Strategy"</h2>
                        <p class="pd-v2-prose">{t}</p>
                    </section>
                })}

                // § Limitations
                {if !limitations.is_empty() {
                    view! {
                        <section class="pd-v2-section" id="limitations">
                            <h2 class="pd-v2-heading">"§ Known Limitations"</h2>
                            <ul class="pd-v2-list">
                                {limitations.into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}
                            </ul>
                        </section>
                    }.into_view()
                } else { view! { <span></span> }.into_view() }}

                // § References
                {if !references.is_empty() {
                    view! {
                        <section class="pd-v2-section" id="references">
                            <h2 class="pd-v2-heading">"§ References"</h2>
                            <ul class="pd-v2-artifacts">
                                {references.into_iter().map(|r| {
                                    let target = if r.external { "_blank" } else { "_self" };
                                    let rel = if r.external { "noopener noreferrer" } else { "" };
                                    view! {
                                        <li>
                                            <a href=r.url target=target rel=rel class="pd-v2-artifact-link">
                                                <span>{r.label}</span>
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
        </div>
    }
}

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
    let raw_slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
    };

    #[cfg(not(feature = "ssr"))]
    {
        let nav = leptos_router::use_navigate();
        create_effect(move |_| {
            let s = raw_slug();
            if let Some(new) = resolve_legacy_slug(&s) {
                nav(
                    &format!("/project/{}/docs", new),
                    leptos_router::NavigateOptions {
                        replace: true,
                        ..Default::default()
                    },
                );
            }
        });
    }

    let slug = move || {
        let s = raw_slug();
        resolve_legacy_slug(&s).map(String::from).unwrap_or(s)
    };
    let project = create_memo(move |_| find_project(&slug()));
    let docs = create_resource(slug, |s| async move {
        #[cfg(feature = "ssr")]
        {
            let _ = s;
            return Err::<DocsDetail, AppError>(AppError::logic("ssr build skips fetch"));
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
            resp.json::<DocsDetail>()
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
                            <header class="pd-header">
                                <div class="pd-category-label" style=format!("color:{}", p.category.accent())>{p.category.label()}</div>
                                <h1 class="pd-title">{p.title}</h1>
                                <p class="pd-subtitle">{p.subtitle}</p>
                                <div class="pd-tech-pills">
                                    {p.tech_stack.iter().map(|tech| view! { <span class="pd-pill">{*tech}</span> }).collect_view()}
                                </div>
                            </header>

                            {surface_tabs(p.slug, Surface::Docs)}

                            <ErrorBoundary fallback=|errors| view! { <ComponentErrorFallback errors/> }>
                                <Suspense fallback=move || view! {
                                    <div class="font-mono text-[var(--text-muted)] py-16 text-center">"Loading…"</div>
                                }>
                                    {
                                        let p_slug = p.slug;
                                        move || docs.get().map(|res| res.map(|d| {
                                            if d.is_structured() {
                                                render_v2_docs(d, p_slug).into_view()
                                            } else {
                                                view! {
                                                    <article class="pd-article mb-8" inner_html=d.content></article>
                                                    <A href=format!("/project/{}", p_slug) class="pd-docs-back-cta">"View case study →"</A>
                                                }.into_view()
                                            }
                                        }))
                                    }
                                </Suspense>
                            </ErrorBoundary>
                        </div>
                    </main>
                }.into_view()
            }
        }}
    }
}
