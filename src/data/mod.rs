//! Static data, types, and shared constants.
//!
//! Split into topical submodules (`projects`, `writeups`, `certs`); this file
//! holds shared profile constants and small cross-cutting types, then
//! `pub use`-re-exports each submodule so existing `use crate::data::*` paths
//! continue to resolve unchanged.

pub mod certs;
pub mod projects;
pub mod writeups;

pub use certs::*;
pub use projects::*;
pub use writeups::*;

// === Profile constants ======================================
pub const EMAIL: &str = "Richard.Mussell@yahoo.com";
pub const GITHUB_URL: &str = "https://github.com/richardmussell";
pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/richard-mussell/";
pub const PROFESSIONAL_TITLE: &str = "Systems Administrator & DevOps Engineer";

// === Shared content types (used by project detail pages) ====
#[derive(Clone, PartialEq)]
pub struct TimelineEntry {
    pub date: String,
    pub title: String,
    pub body: String,
}

#[derive(Clone, PartialEq)]
pub struct CodeSnippet {
    pub lang: String,
    pub label: String,
    pub code: String,
}

#[derive(Clone, PartialEq)]
pub struct BeforeAfter {
    pub label: String,
    pub before: String,
    pub after: String,
}

// === UI signals shared across components ====================
#[derive(Clone)]
pub struct ReadProgressSignals {
    pub progress: leptos::ReadSignal<f64>,
    pub set_progress: leptos::WriteSignal<f64>,
}

#[cfg(test)]
mod tests;
