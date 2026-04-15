# Task runner for richardmussell.github.io
# Usage: `just <recipe>` — `just` alone lists available commands.

default:
    @just --list

# Serve locally via Trunk (see Trunk.toml for port).
serve:
    trunk serve

# Run all 4 cargo check gates (mirrors .github/workflows/ci.yml).
check:
    cargo check --target wasm32-unknown-unknown
    cargo check --no-default-features --features ssr
    cargo check --no-default-features --features "hydrate sqlite" --target wasm32-unknown-unknown
    cargo check --features ssg --bin ssg

# Run the unit test suite under the ssr feature (host-native target).
test:
    cargo test --no-default-features --features ssr

# Release build via Trunk — output to dist/.
build:
    trunk build --release

# Full pre-deploy pipeline: check, test, release build, then push revamp to deploy remote.
deploy: check test build
    git push deploy revamp

# Format check + clippy lint under the wasm32 target.
lint:
    cargo fmt --check
    cargo clippy --target wasm32-unknown-unknown -- -D warnings
