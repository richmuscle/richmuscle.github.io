use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::use_location;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    let location = use_location();
    let current_path = move || location.pathname.get();
    view! {
        <Title text="404 · Page Not Found · Richard Mussell"/>
        <Meta name="description" content="Page not found — Richard Mussell Information Technology & Systems Professional Portfolio."/>
        <main class="not-found-container min-h-screen pt-28 flex flex-col items-center justify-center px-6">
            <p class="not-found-code text-[96px] font-mono text-[var(--text-muted)] leading-none mb-6">"404"</p>
            <p class="font-mono text-[13px] max-w-md text-center mb-2">
                <span style="color:#ef4444;">"error[E0425]: "</span>
                <span style="color:#f8fafc;">"cannot find route `"</span>
                <span style="color:#f8fafc;">{current_path}</span>
                <span style="color:#f8fafc;">"` in scope"</span>
            </p>
            <p class="font-mono text-[11px] text-[var(--text-muted)] mb-10">"not found in this scope"</p>
            <div class="not-found-actions flex gap-4">
                <button
                    type="button"
                    class="px-4 py-2 font-mono text-[13px] border border-[var(--border-subtle)] rounded hover:bg-[var(--bg-elevated)] transition-colors text-[var(--text-primary)]"
                    on:click=move |_| {
                        let _ = web_sys::window().and_then(|w| w.history().ok()).and_then(|h| h.back().ok());
                    }
                >
                    "← Back"
                </button>
                <a href="/" class="px-4 py-2 font-mono text-[13px] border border-[var(--accent-cyan)] text-[var(--accent-cyan)] rounded hover:opacity-90 transition-opacity no-underline">
                    "→ Home"
                </a>
            </div>
        </main>
    }
}

// ============================================================
