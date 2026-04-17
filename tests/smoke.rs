//! Crate-root integration smoke test — asserts the data layer boots.
//!
//! Extends Wave 5's inline unit tests with an integration-level sanity check.
//! Runs under the `ssr` feature because the default csr+sqlite build targets
//! wasm32 and cannot execute on host.

use richardmussell::data::{all_writeups, get_infrastructure_fleet};

#[test]
fn portfolio_data_is_nonempty() {
    assert!(
        !get_infrastructure_fleet().is_empty(),
        "project index is empty — static data layer boot failure"
    );
    assert!(
        !all_writeups().is_empty(),
        "writeup index is empty — static data layer boot failure"
    );
}
