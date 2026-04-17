//! Syntax highlighter dispatcher.
//!
//! `highlight_code(lang, code)` picks a per-language tokenizer (rust, bash,
//! powershell, yaml) or falls back to plain HTML escaping. Each tokenizer
//! lives in its own child module; common helpers (`tok`, the `esc`
//! re-export) are crate-private so only the syntax children use them.

pub mod bash;
pub mod powershell;
pub mod rust;
pub mod yaml;

use crate::utils::text::esc;

/// Wrap `text` in `<span class="{cls}">…</span>` with HTML-escaped contents.
/// Empty input produces empty output so callers can emit tokens unconditionally.
#[inline]
pub(crate) fn tok(cls: &str, text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    format!("<span class=\"{}\">{}</span>", cls, esc(text))
}

pub fn highlight_code(lang: &str, code: &str) -> String {
    match lang.trim().to_uppercase().as_str() {
        "POWERSHELL" | "PWSH" | "PS1" | "PS" => powershell::hl_ps(code),
        "BASH" | "SHELL" | "SH" => bash::hl_sh(code),
        "YAML" | "YML" => yaml::hl_yaml(code),
        "RUST" | "RS" => rust::hl_rust(code),
        _ => esc(code),
    }
}
