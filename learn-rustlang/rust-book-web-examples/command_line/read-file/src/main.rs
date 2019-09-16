use std::env;
use std::fs;
// use cargo run ./src/main.rs to see file content
fn main() {
    let args: Vec<String> = env::args().collect(); // get command line arguments
    println!("{:?}", args);
    // &arg[0] holds path and command name
    let filename = &args[1];
    read_file(filename);
}

fn read_file(filename: &str) {
    println!("In file {}", filename);
    // read file if not found it will panic with message after expect
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // print out file context to stdout
    println!("With text:\n{}", contents);
}

// use cargo test -- --nocaption to run test and see content of the file
#[test]
fn read_file_test() {
    read_file("./src/main.rs")
}
