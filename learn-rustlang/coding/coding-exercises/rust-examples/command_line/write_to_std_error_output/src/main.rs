use std::env;

fn main() {
    let key = "HOME";
    println!("key = {} : value = {}", key, check_for_environment_key(key));
}

fn check_for_environment_key(key: &str) -> String {
    // check if the key is set and get value
    let s = String::new();
    match env::var(key) {
        Ok(val) => return val, //println!("Key = {} : value ={:?}", key, val), // print to std output
        Err(e) => eprintln!("couldn't interpret {}: {}", key, e), // print to std error output Now we see the error onscreen and output.txt contains nothing,
    };
    s
}

// Test functions below.
#[test]
fn check_user_environment_variable() {
    let result = check_for_environment_key("USER");
    let should_be = "tommy";
    println!("User - result = {} should be = {}", result, should_be);
    assert_eq!(result, should_be);
}
#[test]
fn check_home_environment_variable() {
    assert_eq!(check_for_environment_key("HOME"), "/home/tommy");
}
