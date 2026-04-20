use crate::data::{EMAIL, GITHUB_URL, LINKEDIN_URL, PROFESSIONAL_TITLE};
#[cfg(not(feature = "ssr"))]
use crate::utils::track;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::A;

#[component]
pub fn ContactPage() -> impl IntoView {
    let (email_copied, set_email_copied) = create_signal(false);
    #[cfg(feature = "ssr")]
    let _ = set_email_copied;
    view! {
        <Title text=move || format!("Contact · Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="Contact Richard Mussell — Linux Systems Administrator available for systems administration and infrastructure roles."/>
        <main id="main-content" class="contact-page page-enter">
            <div class="contact-header">
                <div class="contact-header-inner">
                    <p class="contact-header-label">"Contact"</p>
                    <h1 class="contact-heading">"Let's talk infrastructure, reliability, and automation."</h1>
                    <p class="contact-subtext">
                        "Linux Systems Administrator with a BS in IT & Administrative Management. I build repeatable infrastructure with Terraform, automate system administration with PowerShell and Bash, and manage identity and access through Active Directory."
                    </p>
                </div>
            </div>

            <div class="contact-body">
                <div class="contact-left">
                    <p class="contact-section-label">"Reach me"</p>
                    <div class="contact-row">
                        <span class="contact-row-glyph" aria-hidden="true">"✉"</span>
                        <div class="contact-row-content">
                            <span class="contact-row-label">"Email"</span>
                            <span class="contact-row-value">{EMAIL}</span>
                            <button
                                type="button"
                                class="contact-copy-btn"
                                on:click=move |_| {
                                    #[cfg(not(feature = "ssr"))]
                                    {
                                        let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?}).catch(function(){{}})", EMAIL));
                                        set_email_copied.set(true);
                                        track("email_copy", r#"{"source":"contact"}"#);
                                        let _ = gloo_timers::callback::Timeout::new(2000, move || set_email_copied.set(false));
                                    }
                                }
                                aria-label="Copy email address"
                            >
                                {move || if email_copied.get() { "✓ Copied" } else { "Copy" }}
                            </button>
                        </div>
                    </div>
                    <div class="contact-row">
                        <span class="contact-row-glyph contact-glyph-in" aria-hidden="true">"in"</span>
                        <div class="contact-row-content">
                            <span class="contact-row-label">"LinkedIn"</span>
                            <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer" class="contact-row-value">"/in/richard-mussell-iii"</a>
                        </div>
                    </div>
                    <div class="contact-row">
                        <span class="contact-row-glyph contact-glyph-gh" aria-hidden="true">"gh"</span>
                        <div class="contact-row-content">
                            <span class="contact-row-label">"GitHub"</span>
                            <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="contact-row-value">"richardmussell"</a>
                        </div>
                    </div>
                    <div class="contact-row">
                        <span class="contact-row-glyph" aria-hidden="true">"◎"</span>
                        <div class="contact-row-content">
                            <span class="contact-row-label">"Location"</span>
                            <span class="contact-row-value">"Oklahoma City, OK"</span>
                        </div>
                    </div>
                </div>

                <div class="contact-divider" aria-hidden="true"></div>

                <div class="contact-right">
                    <p class="contact-section-label">"What I am looking for"</p>
                    <div class="contact-want-list">
                        <div class="contact-want-item"><span class="contact-want-arrow">"->"</span>" Teams that treat infrastructure as code and value reproducible, auditable deployments."</div>
                        <div class="contact-want-item"><span class="contact-want-arrow">"->"</span>" Environments where systems administration includes automation, not just ticket queues."</div>
                        <div class="contact-want-item"><span class="contact-want-arrow">"->"</span>" Organizations investing in observability and proactive monitoring over reactive firefighting."</div>
                        <div class="contact-want-item"><span class="contact-want-arrow">"->"</span>" Hybrid-cloud or on-prem environments with real security requirements (NIST, CIS, zero-trust)."</div>
                        <div class="contact-want-item"><span class="contact-want-arrow">"->"</span>" Remote-first or Oklahoma City-based. Open to relocation for the right role."</div>
                    </div>
                    <div class="contact-action-buttons">
                        <A href="/resume" class="hero-btn">"Download Resume"</A>
                        <A href="/" class="hero-btn">"View Projects"</A>
                    </div>
                </div>
            </div>

            <div class="contact-footer-band">
                "Available for interviews immediately · Response time: same day"
            </div>
        </main>
    }
}

// ============================================================
//  ONE-PAGER  — Recruiter 90-second summary
// ============================================================
