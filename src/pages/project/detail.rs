use super::{meta_strip, surface_tabs, Surface};
use crate::components::ComponentErrorFallback;
use crate::data::{
    find_project, get_infrastructure_fleet, resolve_legacy_slug, ProjectDetail, ProjectStatus,
};
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
    let last_updated = d.last_updated.clone();
    let reading_time = d.reading_time_minutes;

    view! {
        <article class="pd-v2">
            {meta_strip(last_updated, reading_time, None)}

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
                                        // JSON-provided color hints ignored — metric values
                                        // use the single accent token so light-mode theme
                                        // switching doesn't surface a hardcoded hex with
                                        // poor contrast. Visual differentiation is carried
                                        // by the label below each value.
                                        view! {
                                            <div class="pd-v2-metric">
                                                <span class="pd-v2-metric-value">{m.value}</span>
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
                    &format!("/project/{}", new),
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
                let ps = p.project_status.clone();
                let is_planned = ps == ProjectStatus::Planned;
                let is_in_dev = ps == ProjectStatus::InDevelopment;
                let planned_one_liner = p.one_liner.to_string();
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
                                {if !p.tech_stack.is_empty() {
                                    view! {
                                        <div class="pd-tech-pills">
                                            {p.tech_stack.iter().map(|tech| view! { <span class="pd-pill">{*tech}</span> }).collect_view()}
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}
                            </header>

                            {if is_planned {
                                view! {
                                    <div class="pd-v2-section" style="text-align:center;padding:48px 0 32px;">
                                        <span class="pd-v2-status-badge" style="display:inline-block;font-size:11px;font-family:'JetBrains Mono',monospace;letter-spacing:0.1em;color:#475569;border:1px solid #1e293b;border-radius:4px;padding:6px 16px;margin-bottom:24px;">"○ PLANNED"</span>
                                        <p style="color:var(--text-secondary);font-size:15px;line-height:1.8;max-width:560px;margin:0 auto 16px;">{planned_one_liner.clone()}</p>
                                        <p style="color:var(--text-muted);font-size:13px;font-family:'JetBrains Mono',monospace;">"Work has not begun. Design documentation and case study will appear as this project progresses."</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <span></span>
                                }.into_view()
                            }}

                            {if is_in_dev {
                                view! {
                                    <div style="border:1px solid #1e293b;border-radius:6px;padding:14px 20px;margin-bottom:24px;font-size:13px;font-family:'JetBrains Mono',monospace;color:#94a3b8;">
                                        "◐ This case study is currently at V1 depth. A fuller treatment is in progress."
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}

                            {if !is_planned {
                                view! {
                                    <div>
                                        {surface_tabs(p.slug, Surface::Detail)}

                                        <ErrorBoundary fallback=|errors| view! { <ComponentErrorFallback errors/> }>
                                            <Suspense fallback=move || view! {
                                                <div class="font-mono text-[var(--text-muted)] py-16 text-center">"Loading…"</div>
                                            }>
                                                {move || detail.get().map(|res| res.map(|d| {
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
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}

                            // NOTE: docs/demo navigation is already served by the surface
                                //       tab bar at the top AND by the §Artifacts section inside
                                //       render_v2_detail. The duplicate pd-footer-actions
                                //       button row was removed — three CTAs to the same
                                //       destinations was visual noise.

                            {move || {
                                let list = related_row.get();
                                if list.is_empty() { return view! { <span></span> }.into_view(); }
                                view! {
                                    <section class="pd-v2-section pd-v2-related">
                                        <h2 class="pd-v2-heading">"§ Related Projects"</h2>
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
                                    </section>
                                }.into_view()
                            }}

                            <nav class="pd-v2-page-footer" aria-label="Page end navigation">
                                <A href="/" class="pd-v2-page-footer-back">"← All Projects"</A>
                            </nav>
                        </div>
                    </main>
                }.into_view()
            }
        }}
    }
}
