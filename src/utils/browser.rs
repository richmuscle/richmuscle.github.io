//! Browser integration — performance-timing capture, scroll-lock toggling,
//! and fire-and-forget custom-event tracking. All DOM paths are cfg-gated
//! out for the `ssr` host build.

#[allow(unused_imports)] // used by track() in release builds only
#[cfg(not(feature = "ssr"))]
use js_sys;
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::{JsCast, JsValue};
use std::sync::OnceLock;

static WASM_START_TIME_MS: OnceLock<f64> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
pub(crate) fn perf_now_ms() -> Option<f64> {
    let window = web_sys::window()?;
    let perf = js_sys::Reflect::get(&window, &JsValue::from_str("performance")).ok()?;
    let now_fn = js_sys::Reflect::get(&perf, &JsValue::from_str("now")).ok()?;
    let now_fn = now_fn.dyn_into::<js_sys::Function>().ok()?;
    now_fn.call0(&perf).ok()?.as_f64()
}

pub fn capture_wasm_start_time() {
    #[cfg(not(feature = "ssr"))]
    if let Some(now) = perf_now_ms() {
        let _ = WASM_START_TIME_MS.set(now);
    }
}

pub fn wasm_start_time_ms() -> Option<f64> {
    WASM_START_TIME_MS.get().copied()
}

/// Set or unset document.body `scroll-locked` class.
/// Explicit add/remove (not toggle_with_force) — observed that toggle_with_force
/// in some Leptos render cycles does not reliably clear the class.
/// Safe on SSR (no-op). Silent on any DOM traversal failure.
#[cfg(not(feature = "ssr"))]
pub fn set_body_scroll_lock(locked: bool) {
    let Some(body) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
    else {
        return;
    };
    let cl = body.class_list();
    if locked {
        let _ = cl.add_1("scroll-locked");
    } else {
        let _ = cl.remove_1("scroll-locked");
    }
}

#[cfg(feature = "ssr")]
pub fn set_body_scroll_lock(_locked: bool) {}

#[allow(unused_variables)]
pub fn track(event: &str, props: &str) {
    #[cfg(all(not(debug_assertions), not(feature = "ssr")))]
    let _ = js_sys::eval(&format!(
        "window.dispatchEvent(new CustomEvent('portfolio:{}', {{ detail: JSON.parse({:?}) }}))",
        event, props
    ));
}
