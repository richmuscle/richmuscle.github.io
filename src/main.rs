//! WASM entry point. Compiled with `--features hydrate` (or `csr`) by Trunk.
//! All app logic lives in src/lib.rs.
//!
//! Under SSR this binary still compiles (it's the package's default bin) but
//! `main()` is a no-op — the SSG path uses `src/bin/ssg.rs` instead.

#[cfg(not(feature = "ssr"))]
use leptos::*;
#[cfg(not(feature = "ssr"))]
use richardmussell::App;

#[cfg(feature = "ssr")]
fn main() {}

#[cfg(not(feature = "ssr"))]
fn main() {
    richardmussell::utils::capture_wasm_start_time();

    std::panic::set_hook(Box::new(|info| {
        let msg = format!("WASM panic: {}", info);
        let _ = (|| -> Option<()> {
            let doc = web_sys::window()?.document()?;
            let body = doc.body()?;
            let div = doc.create_element("div").ok()?;
            div.set_attribute(
                "style",
                "position:fixed;inset:0;background:#7f1d1d;color:#fca5a5;\
                 font-family:monospace;font-size:13px;padding:32px;\
                 z-index:999999;overflow:auto;white-space:pre-wrap;",
            )
            .ok()?;
            div.set_text_content(Some(&msg));
            body.prepend_with_node_1(&div).ok()
        })();
        web_sys::console::error_1(&msg.into());
    }));

    mount_to_body(|| view! { <App /> });

    let _ = (|| -> Option<()> {
        web_sys::window()?
            .document()?
            .get_element_by_id("wasm-init-indicator")?
            .remove();
        Some(())
    })();
}
