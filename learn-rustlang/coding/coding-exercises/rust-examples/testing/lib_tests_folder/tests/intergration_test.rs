use lib_tests_folder; // name of project in toml file
mod common; // pull in common module

#[test]
fn it_adds_two() {
    common::setup(); // common functions used for setting up tests
    assert_eq!(4, lib_tests_folder::add_two(2));
}
