//! SiteFooter — site-wide identity band rendered at the bottom of every
//! page. Separate from per-page footer navigation (prev/next, back-to-top).
//!
//! Contains: copyright, professional title, contact links, and a status
//! indicator linking to the live telemetry page.

use crate::data::{EMAIL, GITHUB_URL, LINKEDIN_URL};
use leptos::*;
use leptos_router::A;

#[component]
pub fn SiteFooter() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);

    view! {
        <footer class="site-footer" role="contentinfo" aria-label="Site footer">
            <div class="site-footer-inner">
                <div class="site-footer-identity">
                    <p class="site-footer-identity-name">"© 2026 Richard J. Mussell"</p>
                    <p class="site-footer-identity-role">
                        "Systems Administrator · DevOps · Platform Engineer · Oklahoma City"
                    </p>
                </div>
                <nav class="site-footer-nav" aria-label="Site footer navigation">
                    <button
                        type="button"
                        class="site-footer-link"
                        on:click=move |_| {
                            #[cfg(not(feature = "ssr"))]
                            {
                                let _ = js_sys::eval(&format!(
                                    "navigator.clipboard.writeText('{}').catch(function(){{}})",
                                    EMAIL
                                ));
                            }
                            set_email_copied.set(true);
                            set_timeout(
                                move || set_email_copied.set(false),
                                std::time::Duration::from_millis(2000),
                            );
                        }
                        aria-label="Copy email address to clipboard"
                    >
                        {move || if email_copied.get() { "✓ Copied".to_string() } else { EMAIL.to_string() }}
                    </button>
                    <span class="site-footer-sep" aria-hidden="true">"·"</span>
                    <a
                        href=LINKEDIN_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        class="site-footer-link"
                        aria-label="LinkedIn (opens in new tab)"
                    >
                        "LinkedIn"
                        <span class="sr-only">"(opens in new tab)"</span>
                    </a>
                    <span class="site-footer-sep" aria-hidden="true">"·"</span>
                    <a
                        href=GITHUB_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        class="site-footer-link"
                        aria-label="GitHub (opens in new tab)"
                    >
                        "GitHub"
                        <span class="sr-only">"(opens in new tab)"</span>
                    </a>
                    <span class="site-footer-sep" aria-hidden="true">"·"</span>
                    <A href="/one-pager" class="site-footer-link">"One-Pager"</A>
                    <span class="site-footer-sep" aria-hidden="true">"·"</span>
                    <a
                        href="/telemetry"
                        class="site-footer-link site-footer-link-status"
                        title="View real-time system telemetry"
                    >
                        "[SYSTEM_STATUS: NOMINAL]"
                    </a>
                </nav>
            </div>
        </footer>
    }
}
