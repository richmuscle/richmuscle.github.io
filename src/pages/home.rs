use leptos::*;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::{Meta, Title};
use leptos_router::{A, use_navigate};
use crate::data::{get_certifications, get_infrastructure_fleet, GITHUB_URL, LINKEDIN_URL, ProjectCardSignals, ProjectCategory, EMAIL, PROFESSIONAL_TITLE};
use crate::components::ProjectCard;
use std::sync::LazyLock;

fn init_boot_lines() -> Vec<String> {
    vec![
            "Mounting Enterprise Infrastructure Fabric...".into(),
            "Verifying Active Directory Group Policies (GPO)...".into(),
            "Executing User Onboarding Automation...".into(),
            "Hardening Linux/Windows Production Baselines...".into(),
            "richard@sysadmin-ops:~$".into(),
        ]
}

static BOOT_LINES: LazyLock<Vec<String>> = LazyLock::new(init_boot_lines);

#[component]
fn Terminal() -> impl IntoView {
    let navigate = store_value(use_navigate());
    let boot_lines = store_value(BOOT_LINES.clone());
    let (log_output, set_log_output) = create_signal(String::new());
    let (user_input, set_user_input) = create_signal(String::new());
    let initialized = store_value(false);

    create_effect(move |_| {
        if initialized.get_value() { return; }
        initialized.set_value(true);
        let lines = boot_lines.get_value();
        for (i, line) in lines.into_iter().enumerate() {
            let line = line.clone();
            set_timeout(
                move || {
                    set_log_output.update(|output| {
                        if !output.is_empty() { *output += "\n"; }
                        *output += &line;
                    });
                },
                std::time::Duration::from_millis(350 * (i as u64 + 1)),
            );
        }

        // Prime the CLI for immediate command entry on load.
        set_timeout(
            move || {
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(el) = doc.get_element_by_id("terminal-input") {
                        if let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>() {
                            let _ = input.focus();
                        }
                    }
                }
            },
            std::time::Duration::from_millis(120),
        );
    });

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            let raw = user_input.get();
            let input = raw.trim()
                .chars()
                .filter(|c| !c.is_control())
                .take(100)
                .collect::<String>();
            if !input.is_empty() {
                let mut navigate_to: Option<String> = None;
                let lower_input = input.to_lowercase();
                let tokens: Vec<&str> = lower_input.split_whitespace().collect();
                let response = match tokens.as_slice() {
                    ["help"] => "> Commands: help, status, ls, cd [path], cat [file], clear".to_string(),
                    ["status"] => "> Fleet: OPERATIONAL | Identity: Active Directory/Entra ID | Automation: PowerShell/GPO | Infrastructure: Windows Server 2022/Linux".to_string(),
                    ["projects"] => {
                        let project_lines = get_infrastructure_fleet()
                            .iter()
                            .map(|p| format!("  {} [{}] ● operational", p.slug, p.category.label()))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!("> Projects:\n{project_lines}")
                    }
                    ["contact"] => format!("> Email: {} | LinkedIn: /in/richard-mussell | OKC, OK", EMAIL),
                    ["ls"] => {
                        let projects = get_infrastructure_fleet()
                            .iter()
                            .map(|p| format!("  {} [{}] ● operational", p.slug, p.category.label()))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!(
                            "> projects:\n{projects}\n> routes:\n  about\n  resume\n  contact\n  telemetry"
                        )
                    }
                    ["cat", "resume"] => {
                        navigate_to = Some("/resume".to_string());
                        "> Opening resume spec...".to_string()
                    }
                    ["cat", "telemetry"] => {
                        navigate_to = Some("/telemetry".to_string());
                        "> Opening telemetry dashboard...".to_string()
                    }
                    ["cd", target] => {
                        let maybe_project = get_infrastructure_fleet()
                            .iter()
                            .find(|p| p.slug.eq_ignore_ascii_case(target))
                            .map(|p| format!("/project/{}", p.slug));
                        let maybe_route = match *target {
                            "about" => Some("/about".to_string()),
                            "resume" => Some("/resume".to_string()),
                            "contact" => Some("/contact".to_string()),
                            "telemetry" => Some("/telemetry".to_string()),
                            "writing" => Some("/writing".to_string()),
                            "home" | "/" => Some("/".to_string()),
                            _ => None,
                        };
                        if let Some(path) = maybe_project.or(maybe_route) {
                            let path_hint = path.clone();
                            navigate_to = Some(path);
                            format!("> Navigating to path: {path_hint}...")
                        } else {
                            "> Error: Path not found.".to_string()
                        }
                    }
                    ["clear"] => {
                        set_log_output.set(String::new());
                        set_user_input.set(String::new());
                        return;
                    }
                    _ => format!("> Command not found: '{}' — type 'help' for available commands", input),
                };
                set_log_output.update(|output| {
                    if !output.is_empty() { *output += "\n"; }
                    *output += &format!("> {}\n{}", input, response);
                });
                set_user_input.set(String::new());

                if let Some(path) = navigate_to {
                    let nav = navigate.get_value();
                    set_timeout(
                        move || nav(&path, Default::default()),
                        std::time::Duration::from_millis(300),
                    );
                }
            }
        }
    };

    view! {
        <div class="terminal-glow bg-[var(--bg-base)] p-5 rounded-xl font-mono text-sm border border-[var(--border-subtle)]">
            <div class="flex gap-2 mb-4">
                <div class="w-3 h-3 rounded-full bg-red-500"></div>
                <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                <div class="w-3 h-3 rounded-full bg-[#3b82f6]"></div>
                <span class="ml-2 text-xs text-[var(--text-muted)] font-mono">"richard@sys-admin-ops — zsh"</span>
            </div>
            <div role="region" aria-live="polite" aria-atomic="false" class="terminal-output space-y-1 min-h-[120px]">
                {move || {
                    let output = log_output.get();
                    if output.is_empty() { return view! { <span></span> }.into_view(); }
                    let lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();
                    view! {
                        <For
                            each=move || lines.clone()
                            key=|line| line.clone()
                            children=move |line_str: String| {
                                let is_error = line_str.starts_with("> Command not found");
                                view! {
                                    <p class=format!("terminal-line text-xs {}", if is_error { "text-red-400" } else if line_str.starts_with(">") && !line_str.contains("richard@") { "text-[var(--text-primary)]" } else { "text-[var(--text-secondary)]" })>
                                        {if line_str.starts_with(">") && !line_str.contains("$") {
                                            view! {
                                                <span>
                                                    <span class="text-[#3b82f6]">"> "</span>
                                                    <span class="text-[#3b82f6]">{line_str.chars().skip(2).collect::<String>()}</span>
                                                </span>
                                            }.into_view()
                                        } else if line_str.contains("$") {
                                            let mut parts = line_str.splitn(2, '$');
                                            let before = parts.next().unwrap_or("").to_string();
                                            let after  = parts.next().unwrap_or("").to_string();
                                            view! {
                                                <span>
                                                    <span>{before}</span>
                                                    <span class="text-[#3b82f6]">"$"</span>
                                                    <span>{after}</span>
                                                </span>
                                            }.into_view()
                                        } else if line_str.contains("● operational") {
                                            let mut parts = line_str.splitn(2, "● operational");
                                            let left = parts.next().unwrap_or("").to_string();
                                            view! {
                                                <span>
                                                    <span class="text-[#3b82f6]">{left}</span>
                                                    <span class="text-[#22c55e]">"● operational"</span>
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! { <span>{line_str}</span> }.into_view()
                                        }}
                                    </p>
                                }
                            }
                        />
                    }.into_view()
                }}
            </div>
            <div class="terminal-input-row flex items-center gap-2 mt-3 border-t border-[var(--border-subtle)] pt-3">
                <span class="text-[#3b82f6] text-xs">"richard@it-systems-ops:~$"</span>
                <input
                    id="terminal-input"
                    type="text"
                    maxlength="100"
                    aria-label="Interactive terminal. Type 'help' for commands."
                    class="flex-1 bg-transparent text-[#3b82f6] text-xs outline-none caret-[#3b82f6] font-mono"
                    placeholder="type 'help' for commands..."
                    prop:value=move || user_input.get()
                    on:input=move |ev| set_user_input.set(event_target_value(&ev))
                    on:keydown=handle_keydown
                />
                <span class="cursor-blink w-2 h-4 bg-[#3b82f6] inline-block"></span>
            </div>
        </div>
    }
}

// ============================================================
//  HOME PAGE
// ============================================================


#[component]
fn CertificationsSection() -> impl IntoView {
    let certs = store_value(get_certifications());
    view! {
        <section class="certifications-section">
            <div class="cert-section-header">
                <p class="cert-section-name">"Certifications & Development"</p>
                <div class="cert-section-line"></div>
            </div>
            <div role="list" aria-label="Certifications" class="cert-list">
                {certs.get_value().clone().into_iter().map(|cert| {
                    let status_class = match cert.status.as_str() {
                        "Completed"  => "cert-status cert-status-completed",
                        "Pursuing"   => "cert-status cert-status-pursuing",
                        "Studying"   => "cert-status cert-status-studying",
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
        let cyber = all.iter().filter(|p| p.category == ProjectCategory::CyberSecurity).count();
        let cloud = all.iter().filter(|p| p.category == ProjectCategory::CloudInfrastructure).count();
        let admin = all.iter().filter(|p| p.category == ProjectCategory::SystemsAdmin).count();
        let net = all.iter().filter(|p| p.category == ProjectCategory::Networking).count();
        (all_len, cyber, cloud, admin, net)
    });

    let ProjectCardSignals { expanded_slug, set_expanded_slug, did_drag } =
        use_context::<ProjectCardSignals>().expect("App provides ProjectCardSignals");

    let drag_start_x = create_rw_signal(0.0_f64);
    let drag_start_y = create_rw_signal(0.0_f64);

    view! {
        <Title text=move || format!("Richard Mussell · {}", PROFESSIONAL_TITLE)/>
        <Meta name="description" content="Information Technology & Systems Professional with lab projects spanning IaC, Linux automation, observability dashboards, and zero-trust networking."/>
        <main id="main-content" role="main" class="home-main min-h-screen pt-24 page-enter">
            <div class="home-page-wrap">

                // ── HERO ─────────────────────────────────────────────
                <section class="hero-section" aria-labelledby="hero-heading">
                    <div class="hero-grid">
                        <div class="hero-left">
                            <h1 id="hero-heading" class="hero-name">"Richard J. Mussell"</h1>
                            <p class="hero-subtitle">"Systems Administrator | IT Operations"</p>
                            <p class="hero-body">
                                "Disciplined IT Systems Professional and BS in ITAM graduate. I specialize in the reliable administration of hybrid-cloud environments, from managing core Active Directory/Entra ID fabrics to automating system lifecycles via PowerShell and Terraform. Grounded in operational discipline and building toward modern platform operations."
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
                                <span class="hero-stat-label">"Primary Focus"</span>
                                <span class="hero-stat-value">"Systems Administration & IT Operations"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"Core Systems"</span>
                                <span class="hero-stat-value">"Windows Server · Linux (RHEL) · Active Directory"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"Foundations"</span>
                                <span class="hero-stat-value">"Networking (CCNA) · Identity (IAM) · NIST"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"Automation"</span>
                                <span class="hero-stat-value">"PowerShell · Bash · Terraform (IaC)"</span>
                            </div>
                            <div class="hero-stat">
                                <span class="hero-stat-label">"Status"</span>
                                <span class="hero-stat-value">"Open to Entry-Level / Junior Roles"</span>
                            </div>
                        </div>
                    </div>
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
                                let email = EMAIL;
                                let _ = js_sys::eval(&format!("navigator.clipboard.writeText('{}').catch(function(){{}})", email));
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
