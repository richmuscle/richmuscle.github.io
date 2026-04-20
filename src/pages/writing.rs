use crate::components::ComponentErrorFallback;
use crate::data::{resolve_legacy_writeup_slug, WriteUpDetail, PROFESSIONAL_TITLE, WRITEUPS};
use crate::error::AppError;
use crate::utils::{sanitize_slug, track};
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::use_params_map;

#[component]
pub fn WritingPage() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    // Category filter removed from UI; kept in signal for potential deep-link
    // support via URL query param in the future. Always None for now.
    let active_category = create_signal(None::<&'static str>).0;
    let core_order: [&'static str; 4] = [
        "service-provisioning-cox-control-planes",
        "soc-observability-pisces-elk-kql",
        "cisco-ios-fundamentals",
        "windows-server-lab-powershell-automatedlab",
    ];
    let filtered = create_memo(move |_| {
        let q = search_query.get().to_lowercase();
        let category_filter = active_category.get();
        let mut list = WRITEUPS
            .iter()
            .filter(|w| {
                if w.is_demoted {
                    return false;
                }
                let search_ok = q.is_empty()
                    || w.title.to_lowercase().contains(&q)
                    || w.summary.to_lowercase().contains(&q)
                    || w.tags.iter().any(|t| t.to_lowercase().contains(&q));
                let category_ok = category_filter.map(|c| w.category == c).unwrap_or(true);
                search_ok && category_ok
            })
            .cloned()
            .collect::<Vec<_>>();
        list.sort_by(|a, b| {
            if a.is_core != b.is_core {
                return b.is_core.cmp(&a.is_core);
            }
            if a.is_core {
                let a_rank = core_order
                    .iter()
                    .position(|slug| slug == &a.slug)
                    .unwrap_or(usize::MAX);
                let b_rank = core_order
                    .iter()
                    .position(|slug| slug == &b.slug)
                    .unwrap_or(usize::MAX);
                return a_rank.cmp(&b_rank);
            }
            let a_year = a.date.parse::<i32>().unwrap_or(0);
            let b_year = b.date.parse::<i32>().unwrap_or(0);
            b_year.cmp(&a_year).then_with(|| a.title.cmp(b.title))
        });
        list
    });
    view! {
        <Title text=move || format!("Writing · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="Technical writing by Richard Mussell on IT systems and security operations: NIST-aligned automation, sysadmin hardening, zero-trust enforcement, and SIEM alert hygiene."/>
        <main class="min-h-screen pt-24 pb-24 page-enter">
            <div class="writing-page-container max-w-3xl mx-auto px-6">
                <section class="pt-12 pb-14 border-b border-[var(--border-subtle)]">
                    <p class="text-[11px] font-mono text-[var(--text-muted)] uppercase tracking-[0.15em] mb-4">"Writing"</p>
                    <h1 class="text-[36px] font-bold text-[var(--text-primary)] tracking-tight mb-3">"Technical Write-ups"</h1>
                    <p class="text-[14px] font-mono text-[var(--text-muted)]">"Technical notes from lab work and prior experience — Linux hardening, compliance automation, SOC observability, zero-trust networking, and a few pieces on networking and Windows administration."</p>
                </section>
                // Category filter UI (desktop pills + mobile drawer) fully
                // removed. Search input below is the only filter affordance.
                <div role="search" aria-label="Search write-ups">
                    <input
                        type="text"
                        class="writing-search-input"
                        placeholder="Search by title, tag, or keyword..."
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        aria-label="Search write-ups"
                    />
                </div>
                <div class="mt-10 space-y-0">
                    {move || {
                        let list = filtered.get();
                        if list.is_empty() {
                            view! {
                                <p class="text-center text-[var(--text-muted)] font-mono text-[14px] py-12">
                                    "No write-ups match '" {search_query.get()} "'"
                                </p>
                            }.into_view()
                        } else {
                            list.into_iter().map(|w| {
                                let slug      = w.slug.to_string();
                                let title     = w.title.to_string();
                                let subtitle  = w.summary.to_string();
                                let date      = w.date.to_string();
                                let read_time = w.read_time.to_string();
                                let is_core   = w.is_core;
                                let tags      = w.tags.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                                let title_class = if is_core {
                                    "core-essay-title text-[20px] font-bold text-[var(--text-primary)] group-hover:text-[var(--accent-cyan)] transition-colors mb-2 leading-snug"
                                } else {
                                    "text-[20px] font-bold text-[var(--text-primary)] group-hover:text-[var(--accent-cyan)] transition-colors mb-2 leading-snug"
                                };
                                let card_class = if is_core {
                                    "writeup-card-link writeup-card core-essay-card block py-8 border-b border-[var(--border-subtle)] group"
                                } else {
                                    "writeup-card-link writeup-card block py-8 border-b border-[var(--border-subtle)] group"
                                };
                                let detail_href = format!("/writing/{}", sanitize_slug(&slug));
                                // Entire card is a block link so taps on the meta row also navigate.
                                // <article> is nested inside <a> (valid HTML5: <a> accepts flow content).
                                view! {
                                    <a href=detail_href class="writeup-card-main">
                                        <article class=card_class>
                                            <div class="flex items-center gap-3 mb-3">
                                                <span class="text-[11px] font-mono text-[var(--text-muted)]">{date}</span>
                                                {if is_core {
                                                    view! {
                                                        <span class="core-essay-badge">"[CORE_ESSAY]"</span>
                                                    }.into_view()
                                                } else {
                                                    view! { <span></span> }.into_view()
                                                }}
                                                <span class="text-[var(--border-subtle)]">"·"</span>
                                                <span class="text-[11px] font-mono text-[var(--text-muted)]">{read_time}</span>
                                            </div>
                                            <h2 class=title_class>{title}</h2>
                                            <p class="text-[14px] text-[var(--text-secondary)] leading-7 mb-4">{subtitle}</p>
                                            <div class="writeup-card-tags flex flex-wrap gap-2">
                                                {tags.into_iter().map(|tag| view! {
                                                    <span class="text-[10px] font-mono px-2 py-1 rounded bg-[var(--bg-surface)] border border-[var(--border-subtle)] text-[var(--accent-cyan)] uppercase tracking-wider">{tag}</span>
                                                }).collect_view()}
                                            </div>
                                        </article>
                                    </a>
                                }
                            }).collect_view()
                        }
                    }}
                </div>
            </div>
        </main>
    }
}

// ============================================================
//  WRITEUP DETAIL PAGE

#[component]
pub fn WriteupDetailPage() -> impl IntoView {
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
            if let Some(new) = resolve_legacy_writeup_slug(&s) {
                nav(
                    &format!("/writing/{}", new),
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
        resolve_legacy_writeup_slug(&s)
            .map(String::from)
            .unwrap_or(s)
    };
    let index = create_memo(move |_| WRITEUPS.iter().find(|w| w.slug == slug()).cloned());
    let detail = create_resource(slug, |s| async move {
        #[cfg(feature = "ssr")]
        {
            let _ = s;
            return Err::<WriteUpDetail, AppError>(AppError::logic("ssr build skips fetch"));
        }
        #[cfg(not(feature = "ssr"))]
        {
            if s.is_empty() {
                return Err(AppError::logic("empty slug"));
            }
            let url = format!("/writeups/{}.json", s);
            let resp = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| AppError::fetch(e.to_string()))?;
            resp.json::<WriteUpDetail>()
                .await
                .map_err(|e| AppError::parse(e.to_string()))
        }
    });

    create_effect(move |_| {
        let s = slug();
        if !s.is_empty() {
            track("writeup_view", &format!(r#"{{"slug":"{}"}}"#, s));
        }
    });

    let title = move || {
        format!(
            "{} · Richard Mussell",
            index.get().map(|w| w.title).unwrap_or("")
        )
    };
    let desc = move || index.get().map(|w| w.summary).unwrap_or("").to_string();

    view! {
        <Title text=title/>
        <Meta name="description" content=desc/>
        {move || match index.get() {
            None => view! {
                <main class="min-h-screen pt-28 flex items-center justify-center">
                    <div class="text-center font-mono">
                        <p class="text-6xl text-[var(--text-muted)] mb-4">"404"</p>
                        <a href="/writing" class="text-[#22d3ee] text-[13px]">"← Back to Writing"</a>
                    </div>
                </main>
            }.into_view(),
            Some(w) => {
                let title_str = w.title.to_string();
                view! {
                    <main id="main-content" class="wu-main page-enter">

                        <div class="wu-header-band">
                            <div class="wu-container">

                                <nav class="wu-breadcrumb" aria-label="Breadcrumb">
                                    <a href="/" class="wu-breadcrumb-link">"Portfolio"</a>
                                    <span class="wu-breadcrumb-sep" aria-hidden="true">"/"</span>
                                    <a href="/writing" class="wu-breadcrumb-link">"Writing"</a>
                                    <span class="wu-breadcrumb-sep" aria-hidden="true">"/"</span>
                                    <span class="wu-breadcrumb-current">{title_str}</span>
                                </nav>

                                <div class="wu-meta-row">
                                    <span class="wu-meta-item">{w.date}</span>
                                    <span class="wu-meta-dot" aria-hidden="true">"·"</span>
                                    <span class="wu-meta-item">{w.read_time}</span>
                                </div>

                                <h1 class="wu-title">{w.title}</h1>

                                <p class="wu-summary">{w.summary}</p>

                                <div class="wu-tags">
                                    {w.tags.iter().map(|tag| view! {
                                        <span class="wu-tag">{*tag}</span>
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>

                        <div class="wu-body-band">
                            <div class="wu-container">
                                <ErrorBoundary fallback=|errors| view! { <ComponentErrorFallback errors/> }>
                                    <Suspense fallback=move || view! {
                                        <div class="wu-loading">"Loading…"</div>
                                    }>
                                        {move || detail.get().map(|res| res.map(|d| {
                                            let content = d.content
                                                .replace("SECTION_01: THE TACTICAL DASHBOARD", "TECHNICAL FOUNDATIONS")
                                                .replace("EXECUTIVE_SUMMARY_//_TECHNICAL_PILLARS", "Executive Summary")
                                                .replace("//_BEGIN_STRATEGIC_NARRATIVE", "FULL NARRATIVE")
                                                .replace("SECTION_02: THE STRATEGIC NARRATIVE (LaTeX View)", "FULL NARRATIVE")
                                                .replace("SECTION_02: THE STRATEGIC NARRATIVE", "FULL NARRATIVE")
                                                .replace("[DOWNLOAD_STRATEGIC_WHITE_PAPER_//_PDF]", "Download Strategic Brief (PDF)")
                                                .replace("[DOWNLOAD_WHITE_PAPER_PDF]", "Download Strategic Brief (PDF)")
                                                .replace("AUTHOR: Senior Principal Platform Architect", "AUTHOR: Richard Mussell — Principal Platform Architect")
                                                .replace("// ARCHITECT'S SEAL // RICHARD MUSSELL // PRINCIPAL ARCHITECT", "Richard Mussell — Principal Platform Architect")
                                                .replace("// ARCHITECT'S SEAL // Richard Mussell // PRINCIPAL ARCHITECT", "Richard Mussell — Principal Platform Architect");
                                            view! {
                                                <article class="wu-prose" inner_html=content/>
                                            }
                                        }))}
                                    </Suspense>
                                </ErrorBoundary>

                                <div class="wu-footer-nav">
                                    <a href="/writing" class="wu-footer-link">
                                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                                            <path d="M9 2L4 7l5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                        "All Writing"
                                    </a>
                                    <a href="/" class="wu-footer-link">
                                        "All Projects"
                                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                                            <path d="M5 2l5 5-5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                    </a>
                                </div>
                            </div>
                        </div>

                    </main>
                }.into_view()
            }
        }}
    }
}

// ============================================================
//  404 NOT FOUND PAGE
