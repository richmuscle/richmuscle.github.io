//! Utility module — splits concerns into syntax highlighters, browser
//! integration, and text helpers. Public API preserved via `pub use`;
//! callers reference `crate::utils::<name>` unchanged.

pub mod browser;
pub mod syntax;
pub mod text;

pub use browser::{capture_wasm_start_time, set_body_scroll_lock, track, wasm_start_time_ms};
pub use syntax::highlight_code;
pub use text::sanitize_slug;
