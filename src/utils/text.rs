//! Text helpers — slug sanitization and HTML escaping.

#[cfg(debug_assertions)]
use leptos::logging::log;

pub fn sanitize_slug(slug: &str) -> String {
    let out: String = slug
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .take(100)
        .collect();
    #[cfg(debug_assertions)]
    if out != slug {
        log!("sanitize_slug: {:?} -> {:?}", slug, out);
    }
    out
}

/// HTML-escape — used by both `utils::syntax` highlighters and any caller
/// needing minimal escaping. `pub(crate)` keeps it reachable across the split
/// without widening the public surface.
pub(crate) fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\'' => o.push_str("&#39;"),
            _ => o.push(c),
        }
    }
    o
}
