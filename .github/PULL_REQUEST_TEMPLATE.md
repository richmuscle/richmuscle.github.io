## Summary

<!-- What does this change and why? One or two sentences. -->

## Verification

<!-- Copy-paste output of `just check` and `just test` (or equivalent). -->

- [ ] `cargo check --target wasm32-unknown-unknown`
- [ ] `cargo check --no-default-features --features ssr`
- [ ] `cargo check --no-default-features --features "hydrate sqlite" --target wasm32-unknown-unknown`
- [ ] `cargo check --features ssg --bin ssg`
- [ ] `cargo fmt --check`
- [ ] `cargo clippy --target wasm32-unknown-unknown -- -D warnings`
- [ ] `cargo test --no-default-features --features ssr`
- [ ] `trunk build --release`

## ADR reference

<!-- If this change relates to or overrides an ADR, cite it: `ADR-XXX` or `portfolio ADR-XXX`. If contradicting an active ADR, include `OVERRIDES ADR-XXX: <reason>`. -->

## Screenshots

<!-- UI-visible changes only. Before/after if applicable. -->
