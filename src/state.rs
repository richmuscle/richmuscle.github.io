//! Consolidated client application state (CSR). Provided once from [`crate::App`].
use crate::data::{ProjectCardSignals, ProjectCategory, ReadProgressSignals};
use leptos::*;

/// Root reactive state for the portfolio shell: theme, global UI toggles, portfolio filtering, and shared signals for layout chrome.
///
/// Keyboard shortcuts that mutate this struct are handled by [`crate::components::KeyboardNav`]
/// (mounted inside [`crate::components::AppShell`]). See [`crate::App`] for the shortcut map.
#[derive(Clone)]
pub struct GlobalAppState {
    /// Dark mode when `true` (drives `html` / `body` classes in `App`).
    pub is_dark: ReadSignal<bool>,
    pub set_is_dark: WriteSignal<bool>,
    /// When `true`, the keyboard shortcuts help dialog is visible.
    pub shortcuts_open: RwSignal<bool>,
    /// When `true`, the command palette is visible (`Cmd`/`Ctrl`+`K` toggles).
    pub palette_open: RwSignal<bool>,
    /// Scroll-linked reading progress for the top progress bar.
    pub read_progress: ReadProgressSignals,
    /// Project card expand / drag coordination on the home grid.
    pub project_cards: ProjectCardSignals,
    /// Category filter for the home project grid (`None` = all categories).
    pub portfolio_category: RwSignal<Option<ProjectCategory>>,
    /// Case-insensitive substring filter over project title, subtitle, description, slug, and tech stack labels.
    pub portfolio_search: RwSignal<String>,
    /// Bumped after async SQLite enrichment from `/projects/*.json` so home-grid memos re-query the index.
    pub portfolio_index_tick: RwSignal<u32>,
}
