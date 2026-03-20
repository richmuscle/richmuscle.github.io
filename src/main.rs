//! # Richard Mussell — Information Technology & Systems Professional Portfolio
//!
//! ## Architecture
//! Client-side Wasm SPA. Rust + Leptos compiled to
//! wasm32-unknown-unknown. Zero server, zero JS framework,
//! zero GC. Served as static files from hosting.
//!
//! ## Build
//! ```
//! trunk build --release
//! ```
//!
//! cargo check: 0 errors, 2 warnings (wasm target-feature names in .cargo/config.toml — safe to ignore).

#![cfg_attr(not(debug_assertions), deny(unused_imports))]

mod data;
mod utils;
mod components;
mod pages;

use crate::data::{ProjectCardSignals, ReadProgressSignals};
use crate::components::{BackToTop, CommandPalette, KeyboardNav, NavBar};
use crate::pages::{
    AboutPage, ContactPage, HomePage, NotFoundPage, OnePageSummary,
    ProjectDemoPage, ProjectDetailPage, ProjectDocsPage, ResumePage,
    TelemetryPage, WriteupDetailPage, WritingPage,
};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};
use crate::utils::capture_wasm_start_time;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    let (is_dark, set_is_dark) = create_signal(true);
    provide_context(is_dark);

    let shortcuts_open = create_rw_signal(false);
    provide_context(shortcuts_open);
    let palette_open = create_rw_signal(false);
    provide_context(palette_open);

    let (read_progress, set_read_progress) = create_signal(0.0_f64);
    provide_context(ReadProgressSignals { progress: read_progress, set_progress: set_read_progress });

    let (expanded_slug, set_expanded_slug) = create_signal(None::<String>);
    let did_drag = create_rw_signal(false);
    provide_context(ProjectCardSignals { expanded_slug, set_expanded_slug, did_drag });

    create_effect(move |_| {
        let Some(window) = web_sys::window() else { return };
        let Some(document) = window.document() else { return };
        let Some(html) = document.document_element() else { return };
        let Some(body) = document.body() else { return };
        if is_dark.get() {
            html.class_list().add_1("dark").ok();
            html.class_list().remove_1("light").ok();
            body.style().set_property("background-color", "#080d14").ok();
        } else {
            html.class_list().add_1("light").ok();
            html.class_list().remove_1("dark").ok();
            body.style().set_property("background-color", "#f0f4f8").ok();
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
                                if let Some(w) = web_sys::window() {
                                    if let Ok(h) = w.history() {
                                        let _ = h.back();
                                    }
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
                <Route path="/one-pager"            view=OnePageSummary />
                <Route path="/*any"                 view=NotFoundPage />
            </Routes>
            </ErrorBoundary>
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
                                <tr><td class="shortcuts-key">"/"</td><td class="shortcuts-desc">"Focus terminal search"</td></tr>
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

fn main() {
    capture_wasm_start_time();

    std::panic::set_hook(Box::new(|info| {
        let msg = format!("WASM panic: {}", info);
        let _ = (|| -> Option<()> {
            let doc = web_sys::window()?.document()?;
            let body = doc.body()?;
            let div = doc.create_element("div").ok()?;
            div.set_attribute("style",
                "position:fixed;inset:0;background:#7f1d1d;color:#fca5a5;\
                 font-family:monospace;font-size:13px;padding:32px;\
                 z-index:999999;overflow:auto;white-space:pre-wrap;",
            ).ok()?;
            div.set_text_content(Some(&msg));
            body.prepend_with_node_1(&div).ok()
        })();
        web_sys::console::error_1(&msg.into());
    }));

    mount_to_body(|| view! { <App /> });

    let _ = (|| -> Option<()> {
        web_sys::window()?.document()?.get_element_by_id("wasm-init-indicator")?.remove();
        Some(())
    })();
}
