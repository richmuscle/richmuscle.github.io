pub mod components;
pub mod data;
pub mod db;
pub mod error;
pub mod pages;
pub mod state;
pub mod utils;

pub use db::{search_portfolio_projects, search_projects, sqlite_index_ready};
pub use error::AppError;
pub use state::GlobalAppState;

use crate::components::{BackToTop, CommandPalette, KeyboardNav, NavBar, ReadingProgress};
use crate::data::{ProjectCardSignals, ReadProgressSignals};
use crate::pages::{
    AboutPage, ContactPage, HomePage, NotFoundPage, OnePageSummary, ProjectDemoPage,
    ProjectDetailPage, ProjectDocsPage, ResumePage, TelemetryPage, WriteupDetailPage, WritingPage,
};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_dark, set_is_dark) = create_signal(true);
    let shortcuts_open = create_rw_signal(false);
    let palette_open = create_rw_signal(false);
    let (read_progress, set_read_progress) = create_signal(0.0_f64);
    let (expanded_slug, set_expanded_slug) = create_signal(None::<String>);
    let did_drag = create_rw_signal(false);

    // Single consolidated app state — replaces five individual `provide_context` calls
    // and fixes the latent `RwSignal<bool>` type-collision bug between
    // `shortcuts_open` and `palette_open`. The portfolio_* fields are not yet
    // wired to consumers but are retained on the struct for forward compatibility.
    provide_context(crate::GlobalAppState {
        is_dark,
        set_is_dark,
        shortcuts_open,
        palette_open,
        read_progress: ReadProgressSignals {
            progress: read_progress,
            set_progress: set_read_progress,
        },
        project_cards: ProjectCardSignals {
            expanded_slug,
            set_expanded_slug,
            did_drag,
        },
        portfolio_category: create_rw_signal(None::<crate::data::ProjectCategory>),
        portfolio_search: create_rw_signal(String::new()),
        portfolio_index_tick: create_rw_signal(0u32),
    });

    // Open in-memory SQLite portfolio index and kick off async enrichment from
    // /projects/{slug}.json. Both no-op outside `wasm32 + sqlite` feature.
    #[cfg(not(feature = "ssr"))]
    {
        crate::db::init_portfolio_index();
        leptos::spawn_local(async {
            crate::db::enrich_index_from_static_json().await;
        });
    }

    // Body scroll-lock whenever ANY global modal is open: shortcuts-modal or
    // project-overlay. The writing-page filter sheet owns its own scroll lock.
    #[cfg(not(feature = "ssr"))]
    create_effect(move |_| {
        let any_modal_open = shortcuts_open.get() || expanded_slug.with(|s| s.is_some());
        crate::utils::set_body_scroll_lock(any_modal_open);
    });

    // Project-overlay focus management + background inert.
    // On open: focus .po-close (ARIA APG Dialog); inert .home-page-wrap.
    // On close: remove inert; return focus to the project card that triggered it.
    #[cfg(not(feature = "ssr"))]
    create_effect(move |prev: Option<Option<String>>| {
        use leptos::wasm_bindgen::JsCast;
        let current = expanded_slug.get();
        let prev_slug = prev.clone().flatten();
        let was_open = prev_slug.is_some();
        let is_open = current.is_some();

        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return current;
        };

        // Toggle inert on the home page wrap so background content is neither
        // focusable nor announced while the dialog is open. No-op on other routes.
        if let Ok(Some(wrap)) = document.query_selector(".home-page-wrap") {
            if is_open {
                let _ = wrap.set_attribute("inert", "");
            } else {
                let _ = wrap.remove_attribute("inert");
            }
        }

        if is_open && !was_open {
            if let Ok(Some(el)) = document.query_selector(".po-close") {
                if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html_el.focus();
                }
            }
        } else if !is_open && was_open {
            if let Some(slug) = prev_slug {
                let selector = format!("a.project-card-link[href=\"/project/{}\"]", slug);
                if let Ok(Some(el)) = document.query_selector(&selector) {
                    if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = html_el.focus();
                    }
                }
            }
        }

        current
    });

    // Dark/light mode — browser only. web_sys is a WASM-only dep; the cfg guard
    // prevents a compile error on the native SSR target even though create_effect
    // never runs during SSR rendering.
    create_effect(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };
            let Some(html) = document.document_element() else {
                return;
            };
            let Some(body) = document.body() else { return };
            if is_dark.get() {
                html.class_list().add_1("dark").ok();
                html.class_list().remove_1("light").ok();
                body.style()
                    .set_property("background-color", "#080d14")
                    .ok();
            } else {
                html.class_list().add_1("light").ok();
                html.class_list().remove_1("dark").ok();
                body.style()
                    .set_property("background-color", "#f0f4f8")
                    .ok();
            }
        }
    });

    view! {
        <a href="#main-content" class="skip-to-content" tabindex="0">"Skip to main content"</a>
        <Router>
            <KeyboardNav />
            <NavBar is_dark set_is_dark />
            <CommandPalette />
            <ErrorBoundary fallback=|_errors| {
                view! {
                    <main class="min-h-screen pt-28 flex flex-col items-center justify-center font-mono text-center px-4">
                        <p class="text-[96px] text-[var(--text-muted)] mb-4">"error[E0308]"</p>
                        <p class="text-[14px] text-[var(--text-secondary)] mb-2">"runtime error in component tree"</p>
                        <p class="text-[13px] text-[var(--text-muted)] mb-6">"expected: rendered view"</p>
                        <p class="text-[13px] text-[var(--text-muted)] mb-8">"found: panic"</p>
                        <div class="flex gap-4">
                            <button type="button" class="hero-btn" on:click=move |_| {
                                // window.history().back() — WASM-only dep guard required
                                #[cfg(not(feature = "ssr"))]
                                if let Some(w) = web_sys::window() {
                                    if let Ok(h) = w.history() { let _ = h.back(); }
                                }
                            }>"← Back"</button>
                            <a href="/" class="hero-btn">"→ Home"</a>
                        </div>
                    </main>
                }
            }>
            <Routes>
                <Route path="/"                    view=HomePage />
                <Route path="/about"               view=AboutPage />
                <Route path="/writing"             view=WritingPage />
                <Route path="/writing/:slug"       view=WriteupDetailPage />
                <Route path="/project/:slug"       view=ProjectDetailPage />
                <Route path="/project/:slug/docs"  view=ProjectDocsPage />
                <Route path="/project/:slug/demo"  view=ProjectDemoPage />
                <Route path="/resume"              view=ResumePage />
                <Route path="/contact"             view=ContactPage />
                <Route path="/telemetry"           view=TelemetryPage />
                <Route path="/one-pager"           view=OnePageSummary />
                <Route path="/*any"                view=NotFoundPage />
            </Routes>
            </ErrorBoundary>
            <ReadingProgress />
            <BackToTop />
            <Show when=move || shortcuts_open.get() fallback=|| ()>
                <div
                    class="shortcuts-scrim"
                    role="presentation"
                    on:click=move |_| shortcuts_open.set(false)
                >
                    <div
                        class="shortcuts-modal"
                        role="dialog"
                        aria-modal="true"
                        id="keyboard-shortcuts-modal"
                        tabindex="-1"
                        aria-label="Keyboard shortcuts"
                        on:click=move |ev| ev.stop_propagation()
                    >
                        <h2 class="shortcuts-modal-title">"Keyboard shortcuts"</h2>
                        <table class="shortcuts-table">
                            <tbody>
                                <tr><td class="shortcuts-key">"g then h"</td><td class="shortcuts-desc">"Go to Home"</td></tr>
                                <tr><td class="shortcuts-key">"g then p"</td><td class="shortcuts-desc">"Go to first project"</td></tr>
                                <tr><td class="shortcuts-key">"g then r"</td><td class="shortcuts-desc">"Go to Resume"</td></tr>
                                <tr><td class="shortcuts-key">"Esc"</td><td class="shortcuts-desc">"Close / blur"</td></tr>
                                <tr><td class="shortcuts-key">"?"</td><td class="shortcuts-desc">"Toggle this help modal"</td></tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </Show>
        </Router>
    }
}
