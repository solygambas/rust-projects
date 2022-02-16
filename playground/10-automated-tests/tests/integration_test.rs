// cargo test --test integration_test // run integration test only

use automated_tests; // each file in the tests directory is a separate crate, so we need to bring our library
                     // a lib.rs file is necessary to import the crate (we can't import functions from main.rs)
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, automated_tests::add_two(2));
}
