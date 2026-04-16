//! Project detail / docs / demo route surface.
//!
//! All three routes share a surface-tab navigation strip (Case Study,
//! Documentation, Demo) and a thin meta strip (last-updated · reading time ·
//! status). This module owns those helpers and re-exports the page components
//! implemented in the sibling files.

mod demo;
mod detail;
mod docs;

pub use demo::ProjectDemoPage;
pub use detail::ProjectDetailPage;
pub use docs::ProjectDocsPage;

use leptos::*;
use leptos_router::A;

// ============================================================
//   SURFACE TAB BAR — shared across all 3 project sub-pages
// ============================================================
// Renders below the hero on detail/docs/demo. Current surface is
// highlighted. aria-current="page" on the active tab. Links use
// <A> so navigation is client-side.

#[derive(Copy, Clone, PartialEq)]
pub(super) enum Surface {
    Detail,
    Docs,
    Demo,
}

// Thin meta strip rendered between the surface tabs and first content section.
// Shows: Last updated · Reading time · Status.
// Any field that is None is silently skipped. If all three are None, the
// strip itself doesn't render.
pub(super) fn meta_strip(
    last_updated: Option<String>,
    reading_time_minutes: Option<u32>,
    status_label: Option<String>,
) -> impl IntoView {
    let has_any =
        last_updated.is_some() || reading_time_minutes.is_some() || status_label.is_some();
    if !has_any {
        return view! { <span></span> }.into_view();
    }
    view! {
        <div class="pd-meta-strip" role="complementary" aria-label="Page metadata">
            {last_updated.map(|d| view! {
                <span class="pd-meta-item">
                    <span class="pd-meta-kicker">"Updated"</span>
                    <span class="pd-meta-value">{d}</span>
                </span>
            })}
            {reading_time_minutes.map(|t| view! {
                <span class="pd-meta-item">
                    <span class="pd-meta-kicker">"Read"</span>
                    <span class="pd-meta-value">{format!("~{} min", t)}</span>
                </span>
            })}
            {status_label.map(|s| view! {
                <span class="pd-meta-item">
                    <span class="pd-meta-kicker">"Status"</span>
                    <span class="pd-meta-value pd-meta-value-accent">{s}</span>
                </span>
            })}
        </div>
    }
    .into_view()
}

pub(super) fn surface_tabs(slug: &'static str, current: Surface) -> impl IntoView {
    let tab = move |label: &'static str, path_suffix: &'static str, variant: Surface| {
        let href = if path_suffix.is_empty() {
            format!("/project/{}", slug)
        } else {
            format!("/project/{}/{}", slug, path_suffix)
        };
        let is_active = variant == current;
        let class = if is_active {
            "pd-surface-tab pd-surface-tab-active"
        } else {
            "pd-surface-tab"
        };
        let aria = if is_active { "page" } else { "" };
        view! {
            <A href=href class=class attr:aria-current=aria>
                {label}
            </A>
        }
    };
    view! {
        <nav class="pd-surface-tabs" aria-label="Project surface navigation">
            {tab("Case Study", "", Surface::Detail)}
            {tab("Documentation", "docs", Surface::Docs)}
            {tab("Demo", "demo", Surface::Demo)}
        </nav>
    }
}
