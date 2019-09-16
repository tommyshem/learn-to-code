fn do_thing() -> i32 {
    #[cfg(test)]
    println!("planning to return 5"); // only prints when test is run
    5
}

fn main() {
    println!("Number is {}", do_thing());
}

// test function
#[test]
fn test_do_thing() {
    assert_eq!(do_thing(), 5);
}
// Run program and no output
// cargo run
// Run test and it prints the message
// cargo test -- --nocapture
