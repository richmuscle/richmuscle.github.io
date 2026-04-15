//! Localized error UI for `ErrorBoundary` fallbacks; logs via `leptos::logging` on WASM.
use leptos::*;
use leptos_dom::Errors;
use leptos_router::A;

/// Logs `errors` with [`leptos::logging::error!`] and renders a compact, accessible fallback so the rest of the app stays mounted.
#[component]
pub fn ComponentErrorFallback(errors: RwSignal<Errors>) -> impl IntoView {
    create_effect(move |_| {
        errors.with(|errs| {
            if !errs.is_empty() {
                #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
                for (_, err) in errs.iter() {
                    leptos::logging::error!("{}", err);
                }
            }
        });
    });
    view! {
        <div class="component-error-fallback" role="alert" aria-live="assertive">
            <p class="font-mono text-[13px] text-red-400">"This section could not be loaded."</p>
            <p class="text-[var(--text-muted)] text-[12px] mt-2">"Other parts of the page should still work."</p>
            <p class="mt-4">
                <A href="/" class="text-[#22d3ee] text-[13px] font-mono hover:underline">"Return home"</A>
            </p>
        </div>
    }
}
