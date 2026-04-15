#[cfg(not(feature = "ssr"))]
use crate::utils::wasm_start_time_ms;
#[cfg(not(feature = "ssr"))]
use gloo_net::http::Request;
#[cfg(not(feature = "ssr"))]
use gloo_timers::callback::Interval;
#[cfg(not(feature = "ssr"))]
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::{Meta, Title};
use std::collections::VecDeque;

#[cfg(not(feature = "ssr"))]
fn perf_now_ms() -> Option<f64> {
    let window = web_sys::window()?;
    let perf =
        js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("performance")).ok()?;
    let now_fn = js_sys::Reflect::get(&perf, &wasm_bindgen::JsValue::from_str("now")).ok()?;
    let now_fn = now_fn.dyn_into::<js_sys::Function>().ok()?;
    now_fn.call0(&perf).ok()?.as_f64()
}

#[cfg(not(feature = "ssr"))]
fn read_heap_bytes() -> Option<(f64, f64)> {
    let window = web_sys::window()?;
    let perf_js =
        js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("performance")).ok()?;
    let memory = js_sys::Reflect::get(&perf_js, &wasm_bindgen::JsValue::from_str("memory")).ok()?;
    let used = js_sys::Reflect::get(&memory, &wasm_bindgen::JsValue::from_str("usedJSHeapSize"))
        .ok()?
        .as_f64()?;
    let total = js_sys::Reflect::get(&memory, &wasm_bindgen::JsValue::from_str("totalJSHeapSize"))
        .ok()?
        .as_f64()?;
    Some((used, total))
}

#[cfg(not(feature = "ssr"))]
fn read_ttfb_ms() -> Option<f64> {
    let window = web_sys::window()?;
    let perf =
        js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("performance")).ok()?;
    let entries_fn =
        js_sys::Reflect::get(&perf, &wasm_bindgen::JsValue::from_str("getEntriesByType")).ok()?;
    let entries_fn = entries_fn.dyn_into::<js_sys::Function>().ok()?;
    let entries = entries_fn
        .call1(&perf, &wasm_bindgen::JsValue::from_str("navigation"))
        .ok()?;
    let arr = entries.dyn_into::<js_sys::Array>().ok()?;
    let first = arr.get(0);
    if first.is_undefined() || first.is_null() {
        return None;
    }
    let request_start =
        js_sys::Reflect::get(&first, &wasm_bindgen::JsValue::from_str("requestStart"))
            .ok()?
            .as_f64()?;
    let response_start =
        js_sys::Reflect::get(&first, &wasm_bindgen::JsValue::from_str("responseStart"))
            .ok()?
            .as_f64()?;
    Some((response_start - request_start).max(0.0))
}

fn fmt_bytes(bytes: f64) -> String {
    let mb = bytes / (1024.0 * 1024.0);
    format!("{mb:.2} MB")
}

#[cfg(not(feature = "ssr"))]
fn push_log(logs: &WriteSignal<VecDeque<String>>, line: String) {
    logs.update(|items| {
        items.push_front(line);
        while items.len() > 10 {
            let _ = items.pop_back();
        }
    });
}

#[cfg(not(feature = "ssr"))]
async fn run_network_probe(
    set_network_rows: WriteSignal<Vec<String>>,
    set_logs: WriteSignal<VecDeque<String>>,
) {
    let probes = vec![
        "linux-admin-scripting",
        "monitoring-observability",
        "terraform-gcp",
        "zero-trust-networking",
    ];
    let mut rows = Vec::new();
    for slug in probes {
        let path = format!("/projects/{slug}.json");
        let start = perf_now_ms().unwrap_or(0.0);
        let result = Request::get(&path).send().await;
        let elapsed = perf_now_ms().unwrap_or(start) - start;
        match result {
            Ok(resp) if resp.ok() => {
                rows.push(format!("{path} — {:.2}ms", elapsed.max(0.0)));
                push_log(
                    &set_logs,
                    format!(
                        "[INFO] Resource '{slug}.json' fetched in {:.0}ms",
                        elapsed.max(0.0)
                    ),
                );
            }
            Ok(resp) => {
                rows.push(format!("{path} — HTTP {}", resp.status()));
                push_log(
                    &set_logs,
                    format!(
                        "[WARN] Resource '{slug}.json' returned HTTP {}",
                        resp.status()
                    ),
                );
            }
            Err(_) => {
                rows.push(format!("{path} — request failed"));
                push_log(
                    &set_logs,
                    format!("[WARN] Resource '{slug}.json' fetch failed"),
                );
            }
        }
    }
    set_network_rows.set(rows);
}

#[component]
pub fn TelemetryPage() -> impl IntoView {
    let (heap_used, set_heap_used) = create_signal(None::<f64>);
    let (heap_total, set_heap_total) = create_signal(None::<f64>);
    let (wasm_init_ms, set_wasm_init_ms) = create_signal(0.0_f64);
    let (ttfb_ms, set_ttfb_ms) = create_signal(None::<f64>);
    let (ua, set_ua) = create_signal(String::from("Unknown"));
    let (network_rows, set_network_rows) = create_signal(Vec::<String>::new());
    let (logs, set_logs) = create_signal(VecDeque::<String>::new());
    // === Browser-only runtime instrumentation ===
    // Under SSR this entire block is gated out; the view below renders with
    // initial signal values (e.g. "Heap metrics unavailable", "Waiting for
    // telemetry events..."), which matches the first client render before
    // any effect fires — so SSR HTML hydrates cleanly.
    #[cfg(not(feature = "ssr"))]
    {
        let last_heap_used = create_rw_signal(None::<f64>);

        create_effect(move |_| {
            if let Some(window) = web_sys::window() {
                let nav = window.navigator();
                set_ua.set(
                    nav.user_agent()
                        .unwrap_or_else(|_| String::from("Unavailable")),
                );
            }

            if let (Some(now), Some(start)) = (perf_now_ms(), wasm_start_time_ms()) {
                let init = (now - start).max(0.0);
                set_wasm_init_ms.set(init);
                push_log(
                    &set_logs,
                    format!("[INFO] WASM initialized in {:.2}ms", init),
                );
            } else {
                push_log(
                    &set_logs,
                    String::from("[WARN] WASM initialization timer unavailable"),
                );
            }

            if let Some(ttfb) = read_ttfb_ms() {
                set_ttfb_ms.set(Some(ttfb));
                push_log(
                    &set_logs,
                    format!("[INFO] Navigation TTFB measured at {:.2}ms", ttfb),
                );
            } else {
                push_log(
                    &set_logs,
                    String::from("[DEBUG] Navigation timing unavailable for TTFB"),
                );
            }

            if let Some((used, total)) = read_heap_bytes() {
                set_heap_used.set(Some(used));
                set_heap_total.set(Some(total));
                last_heap_used.set(Some(used));
                push_log(
                    &set_logs,
                    format!(
                        "[INFO] JS heap baseline: used={} total={}",
                        fmt_bytes(used),
                        fmt_bytes(total)
                    ),
                );
            } else {
                push_log(
                    &set_logs,
                    String::from("[DEBUG] JS heap metrics unavailable in this browser"),
                );
            }
        });

        let heap_interval = {
            let set_heap_used = set_heap_used;
            let set_heap_total = set_heap_total;
            let set_logs = set_logs;
            Interval::new(2000, move || {
                if let Some((used, total)) = read_heap_bytes() {
                    set_heap_used.set(Some(used));
                    set_heap_total.set(Some(total));
                    if let Some(previous) = last_heap_used.get_untracked() {
                        let delta = used - previous;
                        if delta.abs() > 65536.0 {
                            let sign = if delta >= 0.0 { "+" } else { "-" };
                            push_log(
                                &set_logs,
                                format!(
                                    "[DEBUG] WASM Heap reallocated: {}{:.0}kb",
                                    sign,
                                    delta.abs() / 1024.0
                                ),
                            );
                        }
                    }
                    last_heap_used.set(Some(used));
                }
            })
        };

        {
            let set_network_rows = set_network_rows;
            let set_logs = set_logs;
            spawn_local(async move {
                run_network_probe(set_network_rows, set_logs).await;
            });
        }
        let network_interval = {
            let set_network_rows = set_network_rows;
            let set_logs = set_logs;
            Interval::new(15000, move || {
                let set_network_rows = set_network_rows;
                let set_logs = set_logs;
                spawn_local(async move {
                    run_network_probe(set_network_rows, set_logs).await;
                });
            })
        };
        on_cleanup(move || {
            drop(heap_interval);
            drop(network_interval);
        });
    }

    // Suppress unused-setter warnings under SSR (signals are written only
    // from the gated block above; their getters are still read by the view).
    #[cfg(feature = "ssr")]
    let _ = (
        set_heap_used,
        set_heap_total,
        set_wasm_init_ms,
        set_ttfb_ms,
        set_ua,
        set_network_rows,
        set_logs,
    );

    view! {
        <Title text="Telemetry | Richard J. Mussell" />
        <Meta name="description" content="Observability dashboard with live runtime telemetry, heap metrics, and network IO probes." />

        <main id="main-content" class="telemetry-page min-h-screen pt-28">
            <section class="telemetry-wrap">
                <header class="telemetry-head">
                    <p class="telemetry-kicker">"System Telemetry"</p>
                    <h1 class="telemetry-title">"Observability Dashboard"</h1>
                    <p class="telemetry-subtitle">
                        "Live browser runtime instrumentation focused on performance, stability, and disciplined systems visibility."
                    </p>
                </header>

                <section class="telemetry-grid">
                    <article class="telemetry-card">
                        <div class="telemetry-card-head">
                            <span class="telemetry-dot"></span>
                            <span class="telemetry-label">"WASM Memory"</span>
                            <span class="telemetry-state">"OPERATIONAL"</span>
                        </div>
                        <p class="telemetry-value">
                            {move || match (heap_used.get(), heap_total.get()) {
                                (Some(used), Some(total)) => format!("{} / {}", fmt_bytes(used), fmt_bytes(total)),
                                _ => String::from("Heap metrics unavailable"),
                            }}
                        </p>
                        <p class="telemetry-meta">"Source: window.performance.memory (graceful fallback when unsupported)"</p>
                    </article>

                    <article class="telemetry-card">
                        <div class="telemetry-card-head">
                            <span class="telemetry-dot"></span>
                            <span class="telemetry-label">"WASM Initialization"</span>
                            <span class="telemetry-state">"OPERATIONAL"</span>
                        </div>
                        <p class="telemetry-value">{move || format!("{:.2}ms", wasm_init_ms.get())}</p>
                        <p class="telemetry-meta">"Measured from WASM execution start to app mount."</p>
                    </article>

                    <article class="telemetry-card">
                        <div class="telemetry-card-head">
                            <span class="telemetry-dot"></span>
                            <span class="telemetry-label">"Network IO"</span>
                            <span class="telemetry-state">"OPERATIONAL"</span>
                        </div>
                        <ul class="telemetry-list">
                            <For
                                each=move || network_rows.get()
                                key=|row| row.clone()
                                children=move |row| view! { <li>{row}</li> }
                            />
                        </ul>
                    </article>

                    <article class="telemetry-card">
                        <div class="telemetry-card-head">
                            <span class="telemetry-dot"></span>
                            <span class="telemetry-label">"TTFB"</span>
                            <span class="telemetry-state">"OPERATIONAL"</span>
                        </div>
                        <p class="telemetry-value">
                            {move || match ttfb_ms.get() {
                                Some(v) => format!("{v:.2}ms"),
                                None => String::from("TTFB unavailable"),
                            }}
                        </p>
                        <p class="telemetry-meta">"Navigation Timing: requestStart to responseStart."</p>
                    </article>

                    <article class="telemetry-card">
                        <div class="telemetry-card-head">
                            <span class="telemetry-dot"></span>
                            <span class="telemetry-label">"Runtime Environment"</span>
                            <span class="telemetry-state">"OPERATIONAL"</span>
                        </div>
                        <p class="telemetry-meta"><strong>"Target: "</strong>"wasm32-unknown-unknown"</p>
                        <p class="telemetry-meta"><strong>"Build: "</strong>"release"</p>
                        <p class="telemetry-meta telemetry-ua"><strong>"User-Agent: "</strong>{move || ua.get()}</p>
                    </article>
                </section>

                <section class="telemetry-logs">
                    <div class="telemetry-card-head">
                        <span class="telemetry-dot"></span>
                        <span class="telemetry-label">"System Logs"</span>
                        <span class="telemetry-state">"OPERATIONAL"</span>
                    </div>
                    <pre class="telemetry-terminal">
                        {move || {
                            let lines = logs.get().iter().cloned().collect::<Vec<_>>();
                            if lines.is_empty() {
                                String::from("[INFO] Waiting for telemetry events...")
                            } else {
                                lines.join("\n")
                            }
                        }}
                    </pre>
                </section>
            </section>
        </main>
    }
}
