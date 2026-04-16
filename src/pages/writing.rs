use crate::components::ComponentErrorFallback;
use crate::data::{WriteUpDetail, WRITEUPS};
use crate::error::AppError;
use crate::utils::{sanitize_slug, track};
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::use_params_map;

#[component]
pub fn WritingPage() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (active_category, set_active_category) = create_signal(None::<&'static str>);
    let (sheet_open, set_sheet_open) = create_signal(false);

    // Node refs for focus management — trigger button and sheet close button.
    // The refs are consumed in view! regardless of target; the _ prefix silences
    // SSR warnings since focus-management effects are wasm32-only.
    #[allow(unused_variables)]
    let trigger_ref = create_node_ref::<html::Button>();
    #[allow(unused_variables)]
    let close_ref = create_node_ref::<html::Button>();

    // Body scroll-lock while the filter bottom-sheet is open.
    #[cfg(not(feature = "ssr"))]
    create_effect(move |_| {
        crate::utils::set_body_scroll_lock(sheet_open.get());
    });
    on_cleanup(|| crate::utils::set_body_scroll_lock(false));

    // Focus management: on open → focus close button; on close → return focus to trigger.
    #[cfg(not(feature = "ssr"))]
    create_effect(move |prev: Option<bool>| {
        let current = sheet_open.get();
        let was_open = prev.unwrap_or(false);
        if current && !was_open {
            if let Some(el) = close_ref.get() {
                let _ = el.focus();
            }
        } else if !current && was_open {
            if let Some(el) = trigger_ref.get() {
                let _ = el.focus();
            }
        }
        current
    });

    // Escape-key closes the sheet. Listener installed once per component lifetime;
    // handler checks signal inside so it's a no-op when the sheet is closed.
    #[cfg(not(feature = "ssr"))]
    {
        if let Some(window) = web_sys::window() {
            let closure = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
                move |ev: web_sys::KeyboardEvent| {
                    if sheet_open.get_untracked() && ev.key() == "Escape" {
                        ev.prevent_default();
                        set_sheet_open.set(false);
                    }
                },
            );
            let _ = window.add_event_listener_with_callback(
                "keydown",
                closure.as_ref().unchecked_ref(),
            );
            closure.forget();
        }
    }
    let categories: [&'static str; 5] = [
        "PLATFORM ARCHITECTURE",
        "CYBERSECURITY & NIST",
        "OBSERVABILITY & SOC-OPS",
        "NETWORKING & INFRA",
        "STRATEGY & GOVERNANCE",
    ];
    let core_order: [&'static str; 9] = [
        "the-architect-of-the-prismatic-apex-orchestrating-equilibrium-in-a-holographic-landscape",
        "the-orchestrated-landscape-building-a-high-integrity-ecosystem",
        "the-sustainable-architect-engineering-low-entropy-landscapes-for-the-50-year-lookout",
        "the-builders-ledger-orchestrating-technical-outcomes-through-project-governance",
        "the-orchestrator-of-intent-reflections-on-service-provisioning",
        "the-architect-of-oceanic-visibility-soc-operations-at-universal-scale",
        "the-connectivity-fabric-mastering-the-bedrock-of-the-universal-control-plane",
        "the-mirror-universe-architecting-deterministic-enterprise-simulations",
        "universal-dialects-the-role-of-linux-and-shell-in-the-unified-control-plane",
    ];
    let filtered = create_memo(move |_| {
        let q = search_query.get().to_lowercase();
        let category_filter = active_category.get();
        let mut list = WRITEUPS
            .iter()
            .filter(|w| {
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
        <Title text="Writing · Richard Mussell · Systems Engineering"/>
        <Meta name="description" content="Technical writing by Richard Mussell on IT systems and security operations: NIST-aligned automation, sysadmin hardening, zero-trust enforcement, and SIEM alert hygiene."/>
        <main class="min-h-screen pt-24 pb-24 page-enter">
            <div class="writing-page-container max-w-3xl mx-auto px-6">
                <section class="pt-12 pb-14 border-b border-[var(--border-subtle)]">
                    <p class="text-[11px] font-mono text-[var(--text-muted)] uppercase tracking-[0.15em] mb-4">"Writing"</p>
                    <h1 class="text-[36px] font-bold text-[var(--text-primary)] tracking-tight mb-3">"Technical Write-ups"</h1>
                    <p class="text-[14px] font-mono text-[var(--text-muted)]">"Architectural manifestos and operational deep-dives focused on orchestrating equilibrium within high-integrity ecosystems-from the technical bedrock of connectivity to the 50-year lookout of strategic governance."</p>
                </section>
                // Desktop inline category pills block REMOVED — the search
                // input is the sole filter path on desktop. Mobile users get
                // the .filter-trigger-btn → bottom-sheet drawer below.

                // Mobile: filter trigger button (hidden on desktop via CSS)
                <button
                    type="button"
                    class="filter-trigger-btn"
                    node_ref=trigger_ref
                    on:click=move |_| set_sheet_open.set(true)
                    aria-haspopup="dialog"
                    aria-expanded=move || if sheet_open.get() { "true" } else { "false" }
                >
                    <span class="filter-trigger-label">"FILTER"</span>
                    <span class="filter-trigger-value">
                        {move || active_category.get().map(|c| c.to_string()).unwrap_or_else(|| "ALL CATEGORIES".into())}
                    </span>
                    <span class="filter-trigger-chevron" aria-hidden="true">"▾"</span>
                </button>

                // Mobile: bottom-sheet drawer (always in DOM; visibility driven by is-open class)
                <div
                    class=move || format!("filter-sheet-backdrop {}", if sheet_open.get() { "is-open" } else { "" })
                    on:click=move |_| set_sheet_open.set(false)
                    aria-hidden=move || if sheet_open.get() { "false" } else { "true" }
                ></div>
                <div
                    class=move || format!("filter-sheet {}", if sheet_open.get() { "is-open" } else { "" })
                    role="dialog"
                    aria-modal="true"
                    aria-label="Filter write-ups by category"
                    aria-hidden=move || if sheet_open.get() { "false" } else { "true" }
                >
                    <div class="filter-sheet-grabber" aria-hidden="true"></div>
                    <div class="filter-sheet-header">
                        // <p> instead of <h2> — the dialog's accessible name is set
                        // via aria-label on the parent; a heading here would conflict
                        // with the <h2> writeup card titles in the list beneath.
                        <p class="filter-sheet-title" aria-hidden="true">"Filter by Category"</p>
                        <button
                            type="button"
                            class="filter-sheet-close"
                            node_ref=close_ref
                            on:click=move |_| set_sheet_open.set(false)
                            aria-label="Close filter menu"
                        >"×"</button>
                    </div>
                    <div class="filter-sheet-list" role="list">
                        <button
                            type="button"
                            role="listitem"
                            class=move || format!("filter-sheet-item {}", if active_category.get().is_none() { "filter-sheet-item-active" } else { "" })
                            on:click=move |_| {
                                set_active_category.set(None);
                                set_sheet_open.set(false);
                            }
                        >
                            <span class="filter-sheet-item-label">"All Categories"</span>
                            <span class="filter-sheet-item-check" aria-hidden="true">
                                {move || if active_category.get().is_none() { "●" } else { "" }}
                            </span>
                        </button>
                        {move || categories.iter().map(|&cat| {
                            let cat_click = cat;
                            let is_active = move || active_category.get() == Some(cat_click);
                            view! {
                                <button
                                    type="button"
                                    role="listitem"
                                    class=move || format!("filter-sheet-item {}", if is_active() { "filter-sheet-item-active" } else { "" })
                                    on:click=move |_| {
                                        set_active_category.set(Some(cat_click));
                                        set_sheet_open.set(false);
                                    }
                                >
                                    <span class="filter-sheet-item-label">{cat}</span>
                                    <span class="filter-sheet-item-check" aria-hidden="true">
                                        {move || if is_active() { "●" } else { "" }}
                                    </span>
                                </button>
                            }
                        }).collect_view()}
                    </div>
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
    let slug = move || {
        let raw = params.with(|p| p.get("slug").cloned().unwrap_or_default());
        sanitize_slug(&raw)
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
