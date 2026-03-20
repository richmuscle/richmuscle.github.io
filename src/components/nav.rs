//! NavBar, KeyboardNav, BackToTop, ThemeToggle.
use crate::data::{self, GITHUB_URL, ReadProgressSignals};
use crate::utils::sanitize_slug;
use leptos::*;
use leptos::wasm_bindgen::JsCast;
use leptos_router::{A, use_location, use_navigate};

#[component]
pub fn NavBar(
    is_dark: ReadSignal<bool>,
    set_is_dark: WriteSignal<bool>,
) -> impl IntoView {
    let location = use_location();
    let current_path = move || location.pathname.get();
    let shortcuts_open = use_context::<RwSignal<bool>>().unwrap_or_else(|| create_rw_signal(false));
    let (nav_open, set_nav_open) = create_signal(false);

    // Close drawer when route changes (e.g. keyboard shortcut navigation)
    create_effect(move |_| {
        let _ = location.pathname.get();
        set_nav_open.set(false);
    });

    view! {
        <nav
            class="navbar"
            role="navigation"
            aria-label="Main navigation"
        >
            <div class="navbar-links">
                <A href="/"
                   class=move || if current_path() == "/" { "nav-link-active" } else { "nav-link" }>
                    "Projects"
                </A>
                <A href="/about"
                   class=move || if current_path().starts_with("/about") { "nav-link-active" } else { "nav-link" }>
                    "About"
                </A>
                <A href="/writing"
                   class=move || if current_path().starts_with("/writing") { "nav-link-active" } else { "nav-link" }>
                    "Writing"
                </A>
                // VERIFY: /resume — in-app route, resolves to ResumePage.
                <A href="/resume"
                   class=move || if current_path().starts_with("/resume") { "nav-link-active" } else { "nav-link" }>
                    "Resume"
                </A>
                <A href="/contact"
                   class=move || if current_path().starts_with("/contact") { "nav-link-active" } else { "nav-link" }>
                    "Contact"
                </A>
                // VERIFY: https://github.com/richardmussell — checked; target=_blank, noopener noreferrer.
                <a href=GITHUB_URL
                   target="_blank"
                   rel="noopener noreferrer"
                   class="nav-link">
                    "GitHub"
                </a>
            </div>
            <div style="position:absolute;right:24px;display:flex;align-items:center;gap:8px;">
                <button
                    type="button"
                    class="navbar-mobile-toggle"
                    aria-label="Toggle menu"
                    aria-expanded=move || nav_open.get()
                    on:click=move |_| set_nav_open.update(|v| *v = !*v)
                >
                    {move || if nav_open.get() { "✕" } else { "☰" }}
                </button>
                {move || if nav_open.get() {
                    view! {
                        <div
                            class="navbar-drawer-scrim"
                            on:click=move |_| set_nav_open.set(false)
                        />
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }}
                <button
                    type="button"
                    class="theme-toggle-btn shortcuts-toggle-btn"
                    aria-label="Keyboard shortcuts"
                    on:click=move |_| shortcuts_open.update(|v| *v = !*v)
                >"?"</button>
                <span class="palette-hint" aria-hidden="true">"Cmd + K"</span>
                <ThemeToggle is_dark set_is_dark />
            </div>
            <div class=move || format!("navbar-mobile-drawer {}", if nav_open.get() { "nav-open" } else { "" })>
                <A href="/"
                   class=move || if current_path() == "/" { "active" } else { "" }
                   on:click=move |_| set_nav_open.set(false)>"Projects"</A>
                <A href="/about"
                   class=move || if current_path().starts_with("/about") { "active" } else { "" }
                   on:click=move |_| set_nav_open.set(false)>"About"</A>
                <A href="/writing"
                   class=move || if current_path().starts_with("/writing") { "active" } else { "" }
                   on:click=move |_| set_nav_open.set(false)>"Writing"</A>
                <A href="/resume"
                   class=move || if current_path().starts_with("/resume") { "active" } else { "" }
                   on:click=move |_| set_nav_open.set(false)>"Resume"</A>
                <A href="/contact"
                   class=move || if current_path().starts_with("/contact") { "active" } else { "" }
                   on:click=move |_| set_nav_open.set(false)>"Contact"</A>
                <a href=GITHUB_URL
                   target="_blank"
                   rel="noopener noreferrer"
                   on:click=move |_| set_nav_open.set(false)>"GitHub"</a>
            </div>
        </nav>
    }
}

#[component]
pub fn ThemeToggle(is_dark: ReadSignal<bool>, set_is_dark: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            type="button"
            class="theme-toggle-btn"
            aria-label=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
            title=move || if is_dark.get() { "Light mode" } else { "Dark mode" }
            on:click=move |_| set_is_dark.update(|d| *d = !*d)
        >
            <svg
                class=move || if is_dark.get() { "theme-icon theme-icon-sun" } else { "theme-icon theme-icon-sun theme-icon-hidden" }
                viewBox="0 0 24 24"
                width="16"
                height="16"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <circle cx="12" cy="12" r="5"/>
                <line x1="12" y1="1" x2="12" y2="3"/>
                <line x1="12" y1="21" x2="12" y2="23"/>
                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                <line x1="1" y1="12" x2="3" y2="12"/>
                <line x1="21" y1="12" x2="23" y2="12"/>
                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
            </svg>
            <svg
                class=move || if is_dark.get() { "theme-icon theme-icon-moon theme-icon-hidden" } else { "theme-icon theme-icon-moon" }
                viewBox="0 0 24 24"
                width="16"
                height="16"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
        </button>
    }
}


#[component]
pub fn BackToTop() -> impl IntoView {
    let (visible, set_visible) = create_signal(false);
    let read_progress_ctx = use_context::<ReadProgressSignals>();
    create_effect(move |_| {
        let window = web_sys::window().unwrap();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let scroll_y = web_sys::window().unwrap().scroll_y().unwrap_or(0.0);
            set_visible.set(scroll_y > 300.0);
        }) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    });
    view! {
        <div class=move || format!("back-to-top-wrap {}", if visible.get() { "btt-visible" } else { "" })>
            {read_progress_ctx.map(|ctx| {
                let progress = ctx.progress;
                view! {
                    <svg class="back-to-top-ring" viewBox="0 0 40 40" width="40" height="40" aria-hidden="true">
                        <circle
                            cx="20"
                            cy="20"
                            r="18"
                            fill="none"
                            stroke="var(--accent-cyan)"
                            stroke-width="2"
                            stroke-dasharray="113"
                            stroke-dashoffset=move || format!("{}", 113.0 - (progress.get() / 100.0) * 113.0)
                            transform="rotate(-90 20 20)"
                        />
                    </svg>
                }.into_view()
            }).unwrap_or_else(|| view! { <span></span> }.into_view())}
            <button on:click=move |_| { web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0); } aria-label="Back to top"
                class="back-to-top">
                <span class="back-to-top-glyph">"↑"</span>
            </button>
        </div>
    }
}

#[component]
pub fn KeyboardNav() -> impl IntoView {
    use std::cell::RefCell;
    use std::rc::Rc;
    let navigator = use_navigate();
    let projects  = data::get_infrastructure_fleet();
    let shortcuts_open = use_context::<RwSignal<bool>>().unwrap_or_else(|| create_rw_signal(false));
    let palette_open = use_context::<RwSignal<bool>>().unwrap_or_else(|| create_rw_signal(false));
    let project_card_signals = use_context::<data::ProjectCardSignals>();
    create_effect(move |_| {
        let window   = web_sys::window().expect("window");
        let document = window.document().expect("document");
        let project_card_signals_clone = project_card_signals.clone();
        let last_key: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
        let timeout_handle: Rc<RefCell<Option<gloo_timers::callback::Timeout>>> = Rc::new(RefCell::new(None));
        let last_key_clone       = last_key.clone();
        let timeout_handle_clone = timeout_handle.clone();
        let navigator_clone      = navigator.clone();
        let projects_clone       = projects.clone();
        let document_clone       = document.clone();
        let palette_open_clone   = palette_open;
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            let key = ev.key();

            // Cmd/Ctrl + K opens global command palette and suppresses browser default.
            if (ev.meta_key() || ev.ctrl_key()) && !ev.alt_key() && (key == "k" || key == "K") {
                ev.prevent_default();
                ev.stop_propagation();
                palette_open_clone.update(|open| *open = !*open);
                return;
            }

            // Escape is always handled: close modal, blur focus, close expanded overlays.
            if key == "Escape" {
                if palette_open_clone.get() {
                    palette_open_clone.set(false);
                    return;
                }
                shortcuts_open.set(false);
                if let Some(t) = timeout_handle_clone.borrow_mut().take() { t.cancel(); }
                *last_key_clone.borrow_mut() = String::new();

                if let Some(signals) = project_card_signals_clone.as_ref() {
                    signals.set_expanded_slug.set(None);
                }

                if let Some(active) = document_clone.active_element() {
                    if let Some(el) = active.dyn_ref::<web_sys::HtmlElement>() { let _ = el.blur().ok(); }
                }
                return;
            }

            // Prevent conflicts with inputs/contenteditable before handling other shortcuts.
            let interactive = document_clone.active_element().map(|active| {
                let tag = active.tag_name();
                let is_form = tag == "INPUT" || tag == "TEXTAREA" || tag == "SELECT";
                let is_content_editable = active.get_attribute("contenteditable").is_some();
                is_form || is_content_editable
            }).unwrap_or(false);
            if interactive {
                return;
            }

            // Toggle shortcuts modal via '?' and move focus into the dialog for A11y.
            if key == "?" {
                let currently_open = shortcuts_open.get();
                let next_open = !currently_open;
                shortcuts_open.set(next_open);

                if next_open {
                    let doc_for_focus = document_clone.clone();
                    let t = gloo_timers::callback::Timeout::new(0, move || {
                        if let Some(el) = doc_for_focus.get_element_by_id("keyboard-shortcuts-modal") {
                            if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                                let _ = html_el.focus().ok();
                            }
                        }
                    });
                    t.forget();
                }
                return;
            }

            // Cancel pending 'g' timeout whenever another non-modifier key is pressed.
            if let Some(t) = timeout_handle_clone.borrow_mut().take() { t.cancel(); }

            // 'g' sequence: wait up to 1000ms for h/p/r.
            if key == "g" || key == "G" {
                *last_key_clone.borrow_mut() = "g".to_string();
                let lk2 = last_key_clone.clone();
                let t = gloo_timers::callback::Timeout::new(1000, move || { *lk2.borrow_mut() = String::new(); });
                *timeout_handle_clone.borrow_mut() = Some(t);
                return;
            }

            if *last_key_clone.borrow() == "g" {
                match key.as_str() {
                    "h" | "H" => { navigator_clone("/", Default::default()); }
                    "p" | "P" => {
                        if let Some(p) = projects_clone.first() {
                            navigator_clone(&format!("/project/{}", sanitize_slug(&p.slug)), Default::default());
                        }
                    }
                    "r" | "R" => { navigator_clone("/resume", Default::default()); }
                    _ => {}
                }
                *last_key_clone.borrow_mut() = String::new();
                return;
            }

            // Focus terminal search with '/'
            if key == "/" && !ev.ctrl_key() && !ev.meta_key() && !ev.alt_key() {
                if let Some(el) = document_clone.get_element_by_id("terminal-input") {
                    ev.prevent_default();
                    if let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>() {
                        let _ = input.focus().ok();
                    } else if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = html_el.focus().ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    });
    view! { <span style="display:none;" aria-hidden="true"></span> }
}
