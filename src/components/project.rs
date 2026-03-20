//! ProjectCard, CodeBlock, TimelineSection, BeforeAfterSection, RelatedProjects, StatCard.
use crate::data::{BeforeAfter, CodeSnippet, ProjectIndex, SystemStatus, TimelineEntry, get_infrastructure_fleet};
use crate::utils::{highlight_code, sanitize_slug, track};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn ProjectCard(
    project: ProjectIndex,
    set_expanded: WriteSignal<Option<String>>,
    did_drag: RwSignal<bool>,
) -> impl IntoView {
    let slug            = sanitize_slug(project.slug);
    let slug_for_nav    = slug.clone();
    let slug_for_href   = slug.clone();
    let slug_for_expand = slug.clone();
    let tech_tags       = project.tech_stack.iter().take(3).map(|s| *s).collect::<Vec<_>>();
    let navigator       = use_navigate();
    let project_title   = project.title;
    let cat_accent      = project.category.accent().to_string();

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            navigator(&format!("/project/{}", slug_for_nav), Default::default());
        }
    };

    view! {
        <a
            href=format!("/project/{}", slug_for_href)
            aria-label=format!("View {} project case study", project_title)
            class="project-card-link"
            on:click=move |ev| {
                ev.prevent_default();
                if !did_drag.get() {
                    let slug_track = slug_for_expand.clone();
                    set_expanded.set(Some(slug_track.clone()));
                    track("project_view", &format!(r#"{{"slug":"{}"}}"#, slug_track));
                }
            }
        >
            <article
                role="listitem"
                tabindex="0"
                class="project-card-article"
                style=format!("--card-accent:{}", cat_accent)
                on:keydown=handle_keydown
                aria-label=format!("{} — {} project", project_title, project.category.label())
            >
                <div class="project-card-top-row">
                    <span class="category-name text-[9px] font-mono uppercase tracking-[0.15em]"
                          style=format!("color:{}", cat_accent)>
                        {project.category.label()}
                    </span>
                    {match project.status {
                        SystemStatus::Operational => view! {
                            <span class="project-card-status-chip" style="color:#10b981;">"● LIVE"</span>
                        }.into_view(),
                        SystemStatus::Degraded => view! {
                            <span class="project-card-status-chip" style="color:#f59e0b;">"◐ PARTIAL"</span>
                        }.into_view(),
                        SystemStatus::Maintenance => view! {
                            <span class="project-card-status-chip" style="color:#64748b;">"○ WIP"</span>
                        }.into_view(),
                    }}
                </div>
                <h3 class="project-card-title">{project.title}</h3>
                <p class="project-card-subtitle">{project.subtitle}</p>
                <p class="project-card-description">{project.description}</p>
                <div class="project-card-tags">
                    {tech_tags.iter().map(|tech| view! {
                        <span class="project-card-tag">{*tech}</span>
                    }).collect_view()}
                </div>
                <span class="project-card-cta">"View case study →"</span>
            </article>
        </a>
    }
}

#[component(transparent)]
pub fn StatCard(metric: String, value: String, unit: String) -> impl IntoView {
    let metric_clone = metric.clone();
    let value_clone  = value.clone();
    let unit_clone   = unit.clone();
    view! {
        <div aria-label=format!("Metric: {} achieved {}{}", metric_clone, value_clone, unit_clone)
             class="metric-card bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-xl p-6 text-center hover:border-[var(--border-active)] transition-colors duration-150">
            <div class="text-4xl font-black text-white leading-tight mb-2">
                {value}<span class="text-[15px] text-white/80 ml-1">{unit}</span>
            </div>
            <div class="text-[11px] uppercase tracking-wider text-[var(--text-secondary)] font-mono leading-tight">{metric}</div>
        </div>
    }
}

#[component]
pub fn CodeBlock(snippet: CodeSnippet) -> impl IntoView {
    let (copied, set_copied) = create_signal(false);
    let code_raw    = snippet.code.clone();
    let lang        = snippet.lang.clone();
    let label       = snippet.label.clone();
    let highlighted = highlight_code(&snippet.lang, &snippet.code);

    view! {
        <div role="region" aria-label=format!("{} code sample", lang) class="code-block">
            <div class="code-header">
                <div class="code-header-left">
                    <span class="code-lang">{lang}</span>
                    <span class="code-sep" aria-hidden="true">"·"</span>
                    <span class="code-label">{label}</span>
                </div>
                <button
                    class=move || format!("code-copy-btn cb-copy {}", if copied.get() { "copied" } else { "" })
                    aria-label="Copy code to clipboard"
                    on:click=move |_| {
                        let escaped: String = code_raw.chars().flat_map(|c| match c {
                            '\\'  => vec!['\\','\\'],
                            '"'   => vec!['\\','"'],
                            '\n'  => vec!['\\','n'],
                            '\r'  => vec!['\\','r'],
                            '\t'  => vec!['\\','t'],
                            other => vec![other],
                        }).collect();
                        let js = format!(r#"navigator.clipboard.writeText("{}").catch(function(){{}})"#, escaped);
                        let _ = js_sys::eval(&js);
                        set_copied.set(true);
                        set_timeout(move || set_copied.set(false), std::time::Duration::from_millis(2000));
                    }
                >
                    {move || if copied.get() { "✓" } else { "Copy" }}
                </button>
            </div>
            <div class="code-body">
                <pre inner_html=highlighted></pre>
            </div>
        </div>
    }
}

#[component(transparent)]
pub fn TimelineSection(entries: Vec<TimelineEntry>) -> impl IntoView {
    view! {
        <div role="list" aria-label="Project timeline" class="timeline">
            {entries.into_iter().map(|entry| view! {
                <div role="listitem" class="timeline-item">
                    <div class="timeline-date">{entry.date.clone()}</div>
                    <div class="timeline-title">{entry.title.clone()}</div>
                    <div class="timeline-body">{entry.body.clone()}</div>
                </div>
            }).collect_view()}
        </div>
    }
}


#[component(transparent)]
pub fn BeforeAfterSection(items: Vec<BeforeAfter>) -> impl IntoView {
    view! {
        <div role="table" aria-label="Before and after comparison" class="space-y-4">
            {items.into_iter().map(|item| view! {
                <div>
                    <p class="text-[11px] font-mono text-[var(--text-muted)] uppercase tracking-widest mb-2">{item.label.clone()}</p>
                    <div class="comparison-grid">
                        <div class="comparison-before" role="cell">
                            <div class="comparison-label text-red-400" role="columnheader" scope="col">"✗ Before"</div>
                            <p class="text-[15px] text-[#cbd5e1] leading-7">{item.before.clone()}</p>
                        </div>
                        <div class="comparison-after" role="cell">
                            <div class="comparison-label text-[var(--accent-cyan)]" role="columnheader" scope="col">"✓ After"</div>
                            <p class="text-[15px] text-[#cbd5e1] leading-7">{item.after.clone()}</p>
                        </div>
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}


#[component]
pub fn RelatedProjects(slugs: Vec<String>) -> impl IntoView {
    let all = get_infrastructure_fleet();
    let related: Vec<ProjectIndex> = slugs.iter()
        .map(|s| sanitize_slug(s))
        .filter_map(|sanitized| all.iter().find(|p| p.slug == sanitized).cloned())
        .collect();
    if related.is_empty() { return view! { <span></span> }.into_view(); }
    view! {
        <div class="skills-sidebar space-y-0">
            <h3 class="text-[10px] font-mono text-[var(--text-muted)] uppercase tracking-widest mb-4">"RELATED"</h3>
            {related.into_iter().map(|p| {
                let slug    = sanitize_slug(p.slug);
                let title   = p.title;
                let subtitle = p.subtitle;
                view! {
                    <a href=format!("/project/{}", slug)
                       class="related-project-link block py-4 border-b border-[var(--border-subtle)] last:border-0 group">
                        <div class="text-[13px] font-semibold text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors duration-150 inline-flex items-center gap-2">
                            {title}
                            <span class="text-[var(--text-muted)] group-hover:text-[var(--text-secondary)] transition-colors" aria-hidden="true">"→"</span>
                        </div>
                        <div class="text-[11px] font-mono text-[var(--text-muted)] mt-1">{subtitle}</div>
                    </a>
                }
            }).collect_view()}
        </div>
    }.into_view()
}

