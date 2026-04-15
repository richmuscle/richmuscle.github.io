use crate::components::{ProjectCard, Terminal};
use crate::data::{
    get_certifications, get_infrastructure_fleet, ProjectCardSignals, ProjectCategory, EMAIL,
    GITHUB_URL, LINKEDIN_URL, PROFESSIONAL_TITLE,
};
use crate::GlobalAppState;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::A;

// ============================================================
//  HOME PAGE
// ============================================================

#[component]
fn CertificationsSection() -> impl IntoView {
    let certs = store_value(get_certifications());
    view! {
        <section class="certifications-section">
            <div class="cert-section-header">
                <p class="cert-section-name">"Professional Development"</p>
                <div class="cert-section-line"></div>
            </div>
            <div role="list" aria-label="Certifications" class="cert-list">
                {certs.get_value().clone().into_iter().map(|cert| {
                    let status_class = match cert.status.as_str() {
                        "Completed"  => "cert-status cert-status-completed",
                        "Pursuing"   => "cert-status cert-status-pursuing",
                        "Studying"   => "cert-status cert-status-studying",
                        "Planned"    => "cert-status cert-status-interested",
                        "Interested" => "cert-status cert-status-interested",
                        _            => "cert-status cert-status-default",
                    };
                    view! {
                        <div role="listitem" class="cert-row">
                            <span class="cert-name">{cert.name.clone()}</span>
                            <span class="cert-issuer">{cert.issuer.clone()}</span>
                            <span class=status_class>{cert.status.clone()}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let projects = store_value(get_infrastructure_fleet());
    let (email_copied, set_email_copied) = create_signal(false);
    let (active_filter, set_filter) = create_signal(None::<ProjectCategory>);

    let counts = create_memo(move |_| {
        let all = get_infrastructure_fleet();
        let all_len = all.len();
        let cyber = all
            .iter()
            .filter(|p| p.category == ProjectCategory::CyberSecurity)
            .count();
        let cloud = all
            .iter()
            .filter(|p| p.category == ProjectCategory::CloudInfrastructure)
            .count();
        let admin = all
            .iter()
            .filter(|p| p.category == ProjectCategory::SystemsAdmin)
            .count();
        let net = all
            .iter()
            .filter(|p| p.category == ProjectCategory::Networking)
            .count();
        (all_len, cyber, cloud, admin, net)
    });

    let ProjectCardSignals {
        expanded_slug,
        set_expanded_slug,
        did_drag,
    } = use_context::<GlobalAppState>()
        .expect("App provides GlobalAppState")
        .project_cards;

    let drag_start_x = create_rw_signal(0.0_f64);
    let drag_start_y = create_rw_signal(0.0_f64);

    view! {
        <Title text=move || format!("Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="Systems Administrator & DevOps Engineer with lab projects spanning IaC, Linux automation, observability, and zero-trust networking."/>
        <main id="main-content" role="main" class="home-main min-h-screen pt-24 page-enter">
            <div class="home-page-wrap">

                // ── HERO ─────────────────────────────────────────────
                <section class="hero-section" aria-labelledby="hero-heading">
                    <div class="hero-grid">
                        <div class="hero-left">
                            <h1 id="hero-heading" class="hero-name">"Richard J. Mussell"</h1>
                            <p class="hero-subtitle">"Systems Administrator & DevOps Engineer"</p>
                            <p class="hero-body">
                                "SOC-trained infrastructure engineer. I build the automation, identity, and observability layers that keep hybrid-cloud environments running when nobody's looking. Oklahoma City — open to remote."
                            </p>
                            <p class="hero-body">
                                "Monitored 13 municipal entities in an ELK-based SOC. Shipped zero-touch Windows deployment with Intune/Autopilot, WSUS patch automation, and 3-2-1 DR in lab. Running a 12-tool SOC homelab on bare metal."
                            </p>
                            <p class="hero-meta">
                                <span class="text-[#64748b]">"Oklahoma City, OK"</span>
                                <span class="hero-meta-sep">" · "</span>
                                <span class="text-[#64748b]">"Systems Engineering & Lab Projects"</span>
                            </p>
                            <div class="hero-buttons">
                                // VERIFY: /resume opens resume route; user runs window.print() from that page to get PDF — do not change to .pdf href.
                                <A href="/resume" class="hero-btn">"Download Resume"</A>
                                // VERIFY: https://github.com/richardmussell — checked; target=_blank, noopener.
                                <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" aria-label="GitHub (opens in new tab)" class="hero-btn">
                                    "GitHub"
                                    <span class="sr-only">"(opens in new tab)"</span>
                                </a>
                            </div>
                        </div>
                        <div class="hero-stats">
                            <div class="hero-stat">
                                <span class="hero-stat-label">"PRIMARY_FOCUS"</span>
                                <span class="hero-stat-value">"Systems Administration & DevOps Engineering"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"CORE_INFRASTRUCTURE"</span>
                                <span class="hero-stat-value">"Hybrid-Cloud Admin · Identity (AD/Entra ID) · Linux (RHEL)"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"STANDARDS_&_SECURITY"</span>
                                <span class="hero-stat-value">"Infrastructure as Code · NIST 800-53 · Network Security"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"AUTOMATION_ENGINE"</span>
                                <span class="hero-stat-value">"PowerShell Automation · Terraform · CI/CD Pipelines"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"CURRENT_STATUS"</span>
                                <span class="hero-stat-value">"Open to Systems & DevOps Engineering Roles"</span>
                            </div>
                        </div>
                    </div>
                </section>

                // ── INTERACTIVE TERMINAL ──────────────────────────────
                <section class="terminal-section" aria-label="Interactive command terminal">
                    <Terminal/>
                </section>

                // ── PROJECT GRID ──────────────────────────────────────
                <section
                    class="projects-section"
                    on:mousedown=move |e| {
                        drag_start_x.set(e.client_x() as f64);
                        drag_start_y.set(e.client_y() as f64);
                        did_drag.set(false);
                    }
                    on:mousemove=move |e| {
                        if e.buttons() == 1 {
                            let dx = e.client_x() as f64 - drag_start_x.get();
                            let dy = e.client_y() as f64 - drag_start_y.get();
                            if dx * dx + dy * dy > 25.0 { did_drag.set(true); }
                        }
                    }
                >
                    <div class="projects-header">
                        <p class="projects-eyebrow">"Portfolio"</p>
                        <h2 class="projects-title">"4 Projects · 4 Disciplines"</h2>
                        <p class="text-[13px] font-mono text-[#64748b] mt-2">
                            "Terraform IaC for reliability · PowerShell/GPO & Windows Server 2022/Linux automation · SIEM/Observability · Secure admin via WireGuard"
                        </p>
                    </div>

                    // Filter tabs
                    <div class="filter-tabs">
                        {
                            let set_filter_all = set_filter.clone();
                            view! {
                                <button
                                    on:click=move |_| set_filter_all.set(None)
                                    class=move || if active_filter.get().is_none() { "filter-tab filter-tab-active" } else { "filter-tab" }
                                >
                                    "All "
                                    <span class="filter-tab-count">{move || counts.get().0}</span>
                                </button>
                            }
                        }
                        {vec![
                            ProjectCategory::CyberSecurity,
                            ProjectCategory::CloudInfrastructure,
                            ProjectCategory::SystemsAdmin,
                            ProjectCategory::Networking,
                        ].into_iter().map(|cat| {
                            let label        = cat.label();
                            let cat_filter   = cat.clone();
                            let cat_click    = cat.clone();
                            let cat_for_count = cat.clone();
                            let count = move || {
                                let (_all, cyber, cloud, admin, net) = counts.get();
                                match &cat_for_count {
                                    ProjectCategory::CyberSecurity => cyber,
                                    ProjectCategory::CloudInfrastructure => cloud,
                                    ProjectCategory::SystemsAdmin => admin,
                                    ProjectCategory::Networking => net,
                                }
                            };
                            view! {
                                <span class="filter-tab-sep" aria-hidden="true">" · "</span>
                                <button
                                    on:click=move |_| set_filter.set(Some(cat_click.clone()))
                                    class=move || if active_filter.get().as_ref().map(|f| f == &cat_filter).unwrap_or(false) {
                                        "filter-tab filter-tab-active"
                                    } else { "filter-tab" }
                                >
                                    {label}" "
                                    <span class="filter-tab-count">{count}</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    // Category sections
                    {
                        let all_categories = vec![
                            ProjectCategory::CyberSecurity,
                            ProjectCategory::CloudInfrastructure,
                            ProjectCategory::SystemsAdmin,
                            ProjectCategory::Networking,
                        ];
                        all_categories.into_iter().map(|cat| {
                            let cat_for_filter = cat.clone();
                            let cat_for_render = cat.clone();
                            let label    = cat.label();
                            let desc     = cat.description();
                            let accent   = cat.accent();
                            let projects_snap = projects.get_value();
                            let projects_in_cat: Vec<_> = projects_snap
                                .into_iter()
                                .filter(|p| p.category == cat_for_render)
                                .collect();
                            if projects_in_cat.is_empty() {
                                return view! { <span></span> }.into_view();
                            }
                            let count = projects_in_cat.len();
                            let count_label = if count > 1 { format!("{:02} PROJECTS", count) } else { "01 PROJECT".to_string() };
                            view! {
                                <section
                                    class=move || {
                                        let hidden = active_filter.get()
                                            .as_ref()
                                            .map(|f| f != &cat_for_filter)
                                            .unwrap_or(false);
                                        if hidden { "category-section category-section-hidden" } else { "category-section" }
                                    }
                                >
                                    <div class="category-header">
                                        <div>
                                            <p class="category-name" style=format!("color:{}", accent)>{label}</p>
                                            <p class="category-desc">{desc}</p>
                                        </div>
                                        <span class="category-count">{count_label}</span>
                                    </div>
                                    <div class=format!(
                                        "project-cards-grid {}",
                                        if count == 1 { "project-cards-grid-single" } else { "project-cards-grid-double" }
                                    )>
                                        {projects_in_cat.into_iter().map(|p| view! {
                                            <ProjectCard project=p.clone() set_expanded=set_expanded_slug did_drag=did_drag />
                                        }).collect_view()}
                                    </div>
                                </section>
                            }.into_view()
                        }).collect_view()
                    }
                </section>

                <CertificationsSection />

                <footer class="home-footer">
                    <span class="home-footer-left">"© 2026 Richard Mussell"</span>
                    <div class="home-footer-right home-footer-links">
                        <button
                            on:click=move |_| {
                                #[cfg(not(feature = "ssr"))]
                                {
                                    let email = EMAIL;
                                    let _ = js_sys::eval(&format!("navigator.clipboard.writeText('{}').catch(function(){{}})", email));
                                }
                                set_email_copied.set(true);
                                set_timeout(move || set_email_copied.set(false), std::time::Duration::from_millis(2000));
                            }
                            class="home-footer-link"
                            aria-label="Copy email address to clipboard"
                        >
                            {move || if email_copied.get() { "✓ Copied!" } else { EMAIL }}
                        </button>
                        <span class="home-footer-sep">" · "</span>
                        // VERIFY: https://www.linkedin.com/in/richard-mussell/ — full URL, target=_blank, noopener noreferrer.
                        <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer" aria-label="LinkedIn (opens in new tab)" class="home-footer-link">"LinkedIn"<span class="sr-only">"(opens in new tab)"</span></a>
                        <span class="home-footer-sep">" · "</span>
                        <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" aria-label="GitHub (opens in new tab)" class="home-footer-link">"GitHub"<span class="sr-only">"(opens in new tab)"</span></a>
                        <span class="home-footer-sep">" · "</span>
                        // VERIFY: /one-pager — same-site Leptos route; no target=_blank (intentional).
                        <A href="/one-pager" class="home-footer-link">"One-Pager"</A>
                        <span class="home-footer-sep">" · "</span>
                        <a
                            href="/telemetry"
                            class="home-footer-link telemetry-status-link"
                            title="View Real-Time System Telemetry"
                        >
                            "[SYSTEM_STATUS: NOMINAL]"
                        </a>
                    </div>
                </footer>
            </div>

            // ── EXPANDED CARD OVERLAY ─────────────────────────────
            {move || {
                match expanded_slug.get() {
                    None => view! { <span></span> }.into_view(),
                    Some(slug_val) => {
                        let all_projects = get_infrastructure_fleet();
                        match all_projects.into_iter().find(|p| p.slug == slug_val.as_str()) {
                            None => view! { <span></span> }.into_view(),
                            Some(p) => {
                                let p_title     = p.title;
                                let p_subtitle  = p.subtitle;
                                let p_slug      = p.slug;
                                let p_slug2     = p.slug;
                                let p_slug3     = p.slug;
                                let p_desc      = p.description;
                                let p_stack     = p.tech_stack;
                                let cat_label   = p.category.label();
                                let cat_accent  = p.category.accent();
                                let metric_line = crate::data::one_liner_for_project(p.slug);

                                view! {
                                    <div
                                        class="po-scrim"
                                        on:click=move |_| set_expanded_slug.set(None)
                                        aria-hidden="true"
                                    />

                                    <div
                                        class="po-panel"
                                        role="dialog"
                                        aria-modal="true"
                                        aria-labelledby="po-title"
                                    >
                                        <div class="po-accent-bar" style=format!("background:{}", cat_accent)/>

                                        <div class="po-header">
                                            <div class="po-header-left">
                                                <span
                                                    class="po-category-label"
                                                    style=format!("color:{}", cat_accent)
                                                >{cat_label}</span>
                                                <h2 class="po-title" id="po-title">{p_title}</h2>
                                                <p class="po-subtitle">{p_subtitle}</p>
                                            </div>
                                            <button
                                                class="po-close"
                                                on:click=move |_| set_expanded_slug.set(None)
                                                aria-label="Close project preview"
                                            >
                                                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                                                    <path d="M1 1L13 13M13 1L1 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                                </svg>
                                            </button>
                                        </div>

                                        <div class="po-divider"/>

                                        <p class="po-desc">{p_desc}</p>

                                        <div class="po-metric-strip">
                                            <span class="po-metric-icon" aria-hidden="true">"◈"</span>
                                            <span class="po-metric-text">{metric_line}</span>
                                        </div>

                                        <div class="po-tech-row">
                                            {p_stack.iter().map(|tech| view! {
                                                <span class="po-tech-pill">{*tech}</span>
                                            }).collect_view()}
                                        </div>

                                        <div class="po-divider"/>

                                        <div class="po-actions" style=format!("--po-accent:{}", cat_accent)>
                                            <A
                                                href=format!("/project/{}", p_slug)
                                                class="po-btn-primary"
                                            >
                                                "View Case Study"
                                                <svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
                                                    <path d="M2 6h8M6 2l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                            </A>
                                            <A
                                                href=format!("/project/{}/docs", p_slug2)
                                                class="po-btn-secondary"
                                            >"Docs"</A>
                                            <A
                                                href=format!("/project/{}/demo", p_slug3)
                                                class="po-btn-secondary"
                                            >"Demo"</A>
                                        </div>

                                        <div class="po-kbd-hint">
                                            <span class="po-kbd">"Esc"</span>
                                            <span>"to close"</span>
                                            <span class="po-kbd-sep">"·"</span>
                                            <span class="po-kbd">"↵"</span>
                                            <span>"to open"</span>
                                        </div>
                                    </div>
                                }.into_view()
                            }
                        }
                    }
                }
            }}
        </main>
    }
}

// ============================================================
//  PROJECT DETAIL — reuses original structure, updated copy
// ============================================================
