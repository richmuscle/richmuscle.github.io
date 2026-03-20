//! Layout and utility components: ErrorBoundary, ReadingProgress.
use crate::data::ReadProgressSignals;
use leptos::{
    component, create_effect, view, IntoView,
    use_context, SignalGet, SignalSet,
};
use leptos::wasm_bindgen::{self, JsCast};

#[component]
pub fn ReadingProgress() -> impl IntoView {
    let ctx = use_context::<ReadProgressSignals>().expect("App provides ReadProgressSignals");
    create_effect(move |_| {
        let window = web_sys::window().unwrap();
        let set_progress = ctx.set_progress;
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();
            let scroll_top = win.scroll_y().unwrap_or(0.0);
            let doc_height = doc.body().unwrap().scroll_height() as f64;
            let win_height = win.inner_height().unwrap().as_f64().unwrap_or(0.0);
            let denom = doc_height - win_height;
            let pct = if denom > 0.0 { (scroll_top / denom) * 100.0 } else { 0.0 };
            set_progress.set(pct.min(100.0).max(0.0));
        }) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    });
    view! {
        <div class="reading-progress-bar">
            <div class="reading-progress-fill" style=move || format!("width: {}%", ctx.progress.get())></div>
        </div>
    }
}
