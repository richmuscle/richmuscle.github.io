use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::use_params_map;
use crate::data::{WriteUpDetail, WRITEUPS};
use crate::utils::{sanitize_slug, track};

#[component]
pub fn WritingPage() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (active_tag, set_active_tag) = create_signal(None::<String>);
    let all_tags = create_memo(move |_| {
        let mut tags: Vec<String> = WRITEUPS.iter().flat_map(|w| w.tags.iter().map(|s| s.to_string())).collect();
        tags.sort_unstable();
        tags.dedup();
        tags
    });
    let filtered = create_memo(move |_| {
        let q = search_query.get().to_lowercase();
        let tag_filter = active_tag.get();
        WRITEUPS.iter()
            .filter(|w| {
                let search_ok = q.is_empty()
                    || w.title.to_lowercase().contains(&q)
                    || w.summary.to_lowercase().contains(&q)
                    || w.tags.iter().any(|t| t.to_lowercase().contains(&q));
                let tag_ok = tag_filter.as_ref().map(|t| w.tags.contains(&t.as_str())).unwrap_or(true);
                search_ok && tag_ok
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    view! {
        <Title text="Writing · Richard Mussell · Systems Engineering"/>
        <Meta name="description" content="Technical writing by Richard Mussell on IT systems and security operations: NIST-aligned automation, sysadmin hardening, zero-trust enforcement, and SIEM alert hygiene."/>
        <main class="min-h-screen pt-24 pb-24 page-enter">
            <div class="writing-page-container max-w-3xl mx-auto px-6">
                <section class="pt-12 pb-10 border-b border-[var(--border-subtle)]">
                    <p class="text-[11px] font-mono text-[var(--text-muted)] uppercase tracking-[0.15em] mb-4">"Writing"</p>
                    <h1 class="text-[36px] font-bold text-[var(--text-primary)] tracking-tight mb-3">"Technical Write-ups"</h1>
                    <p class="text-[14px] font-mono text-[var(--text-muted)]">"Operational deep dives into systems hardening, NIST-aligned automation, zero-trust administrative access, and SIEM/SOC signal quality."</p>
                </section>
                <div class="tag-pills-row mb-4">
                    <button
                        type="button"
                        class=move || format!("tag-pill {}", if active_tag.get().is_none() { "tag-pill-active" } else { "" })
                        on:click=move |_| {
                            set_active_tag.set(None);
                        }
                    >
                        "All"
                    </button>
                    {move || all_tags.get().into_iter().map(|tag| {
                        let tag_for_active = tag.clone();
                        let tag_click = tag.clone();
                        let tag_view = tag;
                        let is_active = move || active_tag.get().as_ref() == Some(&tag_for_active);
                        view! {
                            <button
                                type="button"
                                class=move || format!("tag-pill {}", if is_active() { "tag-pill-active" } else { "" })
                                on:click=move |_| {
                                    set_active_tag.update(|v| *v = if *v == Some(tag_click.clone()) { None } else { Some(tag_click.clone()) });
                                }
                            >
                                {tag_view}
                            </button>
                        }
                    }).collect_view()}
                </div>
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
                            list.into_iter().enumerate().map(|(idx, w)| {
                                let slug      = w.slug.to_string();
                                let title     = w.title.to_string();
                                let subtitle  = w.summary.to_string();
                                let date      = w.date.to_string();
                                let read_time = w.read_time.to_string();
                                let tags      = w.tags.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                                let title_class = if idx == 0 {
                                    "text-[20px] font-bold text-[var(--accent-magenta)] group-hover:opacity-90 transition-colors mb-2 leading-snug"
                                } else {
                                    "text-[20px] font-bold text-[var(--text-primary)] group-hover:text-[var(--accent-cyan)] transition-colors mb-2 leading-snug"
                                };
                                view! {
                                    <a href=format!("/writing/{}", sanitize_slug(&slug)) class="writeup-card-link writeup-card block py-8 border-b border-[var(--border-subtle)] group">
                                        <div class="flex items-center gap-3 mb-3">
                                            <span class="text-[11px] font-mono text-[var(--text-muted)]">{date}</span>
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
    let slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
    };
    let index = create_memo(move |_| WRITEUPS.iter().find(|w| w.slug == slug()).cloned());
    let detail = create_resource(
        slug,
        |s| async move {
            if s.is_empty() {
                return None;
            }
            let url = format!("/writeups/{}.json", s);
            gloo_net::http::Request::get(&url)
                .send()
                .await
                .ok()?
                .json::<WriteUpDetail>()
                .await
                .ok()
        },
    );

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
                                <Suspense fallback=move || view! {
                                    <div class="wu-loading">"Loading…"</div>
                                }>
                                    {move || match detail.get() {
                                        None => view! { <span></span> }.into_view(),
                                        Some(None) => view! {
                                            <p class="wu-error">"error[E0404]: writeup not found"</p>
                                        }.into_view(),
                                        Some(Some(d)) => view! {
                                            <article class="wu-prose" inner_html=d.content/>
                                        }.into_view(),
                                    }}
                                </Suspense>

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
