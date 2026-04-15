//! Injects CI build metadata into the WASM crate via `env!` / `option_env!` consumers.
fn main() {
    println!("cargo:rerun-if-env-changed=PORTFOLIO_BUILD_GIT_SHA");
    println!("cargo:rerun-if-env-changed=PORTFOLIO_BUILD_TIME");
    let sha = std::env::var("PORTFOLIO_BUILD_GIT_SHA").unwrap_or_else(|_| "local".to_string());
    let time = std::env::var("PORTFOLIO_BUILD_TIME").unwrap_or_else(|_| "local".to_string());
    println!("cargo:rustc-env=PORTFOLIO_BUILD_GIT_SHA={sha}");
    println!("cargo:rustc-env=PORTFOLIO_BUILD_TIME={time}");
}
