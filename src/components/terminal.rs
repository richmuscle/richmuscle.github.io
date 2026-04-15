//! Interactive boot-and-command terminal mounted on the home page.
use leptos::*;
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::JsCast;
use leptos_router::use_navigate;
use crate::data::{get_infrastructure_fleet, EMAIL};
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
pub fn Terminal() -> impl IntoView {
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
                #[cfg(not(feature = "ssr"))]
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

    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
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
