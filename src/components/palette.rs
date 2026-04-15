use crate::data::{get_infrastructure_fleet, WRITEUPS};
use crate::GlobalAppState;
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_router::use_navigate;

#[derive(Clone, PartialEq)]
struct PaletteItem {
    kind: &'static str,
    icon: &'static str,
    title: String,
    detail: String,
    path: String,
}

fn score_item(query: &str, haystack: &str) -> Option<i32> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Some(0);
    }
    let h = haystack.to_lowercase();
    if let Some(pos) = h.find(&q) {
        return Some(pos as i32);
    }
    let mut qi = 0usize;
    let qchars: Vec<char> = q.chars().collect();
    for c in h.chars() {
        if qi < qchars.len() && c == qchars[qi] {
            qi += 1;
        }
    }
    if qi == qchars.len() {
        Some(250)
    } else {
        None
    }
}

fn build_index() -> Vec<PaletteItem> {
    let mut items = Vec::new();

    for p in get_infrastructure_fleet().iter() {
        items.push(PaletteItem {
            kind: "Project",
            icon: "📁",
            title: p.title.to_string(),
            detail: format!("{} · {}", p.category.label(), p.tech_stack.join(", ")),
            path: format!("/project/{}", p.slug),
        });
    }

    for w in WRITEUPS.iter() {
        items.push(PaletteItem {
            kind: "Writing",
            icon: "🖊",
            title: w.title.to_string(),
            detail: format!("{} · {}", w.date, w.tags.join(", ")),
            path: format!("/writing/{}", w.slug),
        });
    }

    let nav_items = [
        ("Navigation", "↳", "Home", "Primary route", "/"),
        ("Navigation", "↳", "About", "Professional profile", "/about"),
        (
            "Navigation",
            "↳",
            "Resume",
            "Experience and background",
            "/resume",
        ),
        (
            "Navigation",
            "↳",
            "Contact",
            "Direct contact methods",
            "/contact",
        ),
        (
            "Navigation",
            "↳",
            "Telemetry",
            "Runtime observability dashboard",
            "/telemetry",
        ),
    ];
    for (kind, icon, title, detail, path) in nav_items {
        items.push(PaletteItem {
            kind,
            icon,
            title: title.to_string(),
            detail: detail.to_string(),
            path: path.to_string(),
        });
    }

    items
}

#[component]
pub fn CommandPalette() -> impl IntoView {
    let palette_open = use_context::<GlobalAppState>()
        .map(|s| s.palette_open)
        .unwrap_or_else(|| create_rw_signal(false));
    let navigate = store_value(use_navigate());
    let (query, set_query) = create_signal(String::new());
    let (selected_idx, set_selected_idx) = create_signal(0usize);
    let index = store_value(build_index());

    let results = create_memo(move |_| {
        let q = query.get();
        let mut ranked: Vec<(i32, PaletteItem)> = index
            .get_value()
            .into_iter()
            .filter_map(|item| {
                let hay = format!("{} {} {} {}", item.kind, item.title, item.detail, item.path);
                score_item(&q, &hay).map(|score| (score, item))
            })
            .collect();
        ranked.sort_by_key(|(score, item)| (*score, item.title.clone()));
        ranked
            .into_iter()
            .map(|(_, item)| item)
            .take(24)
            .collect::<Vec<_>>()
    });

    #[cfg(not(feature = "ssr"))]
    create_effect(move |_| {
        if palette_open.get() {
            set_selected_idx.set(0);
            let document = web_sys::window().and_then(|w| w.document());
            if let Some(doc) = document {
                let timeout = gloo_timers::callback::Timeout::new(0, move || {
                    if let Some(el) = doc.get_element_by_id("command-palette-input") {
                        if let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>() {
                            let _ = input.focus();
                            input.select();
                        }
                    }
                });
                timeout.forget();
            }
        } else {
            set_query.set(String::new());
            set_selected_idx.set(0);
        }
    });

    view! {
        <Show when=move || palette_open.get() fallback=|| ()>
            <div class="palette-scrim" role="presentation" on:click=move |_| palette_open.set(false)>
                <section class="palette-modal" role="dialog" aria-modal="true" aria-label="Command palette" on:click=move |ev| ev.stop_propagation()>
                    <div class="palette-header">
                        <input
                            id="command-palette-input"
                            class="palette-input"
                            type="text"
                            autocomplete="off"
                            placeholder="Search projects, writing, routes..."
                            prop:value=move || query.get()
                            on:input=move |ev| set_query.set(event_target_value(&ev))
                            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                let rows = results.get();
                                match ev.key().as_str() {
                                    "ArrowDown" => {
                                        ev.prevent_default();
                                        if rows.is_empty() {
                                            set_selected_idx.set(0);
                                        } else {
                                            let next = (selected_idx.get() + 1).min(rows.len().saturating_sub(1));
                                            set_selected_idx.set(next);
                                        }
                                    }
                                    "ArrowUp" => {
                                        ev.prevent_default();
                                        set_selected_idx.update(|i| {
                                            if *i > 0 {
                                                *i -= 1;
                                            }
                                        });
                                    }
                                    "Enter" => {
                                        ev.prevent_default();
                                        let rows = results.get();
                                        if let Some(item) = rows.get(selected_idx.get()) {
                                            palette_open.set(false);
                                            let nav = navigate.get_value();
                                            nav(&item.path, Default::default());
                                        }
                                    }
                                    "Escape" => {
                                        ev.prevent_default();
                                        palette_open.set(false);
                                    }
                                    _ => {}
                                }
                            }
                        />
                    </div>
                    <div class="palette-results">
                        {move || {
                            let rows = results.get();
                            if rows.is_empty() {
                                view! { <p class="palette-empty">"No results found."</p> }.into_view()
                            } else {
                                rows.into_iter()
                                    .enumerate()
                                    .map(|(i, row)| {
                                        let path = row.path.clone();
                                        view! {
                                            <button
                                                type="button"
                                                class=move || if selected_idx.get() == i { "palette-row palette-row-active" } else { "palette-row" }
                                                on:mouseenter=move |_| set_selected_idx.set(i)
                                                on:click=move |_| {
                                                    palette_open.set(false);
                                                    let nav = navigate.get_value();
                                                    nav(&path, Default::default());
                                                }
                                            >
                                                <span class="palette-icon">{row.icon}</span>
                                                <span class="palette-main">
                                                    <span class="palette-title">{row.title.clone()}</span>
                                                    <span class="palette-detail">{format!("{} · {}", row.kind, row.detail)}</span>
                                                </span>
                                                <span class="palette-enter">"↵"</span>
                                            </button>
                                        }
                                    })
                                    .collect_view()
                                    .into_view()
                            }
                        }}
                    </div>
                </section>
            </div>
        </Show>
    }
}
