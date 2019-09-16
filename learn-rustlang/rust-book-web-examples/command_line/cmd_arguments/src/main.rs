// Note that std::env::args will panic if any argument contains invalid Unicode.
// If your program needs to accept arguments containing invalid Unicode,
// use std::env::args_os instead
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // get command line arguments
    println!("{:?}", args);
    saving_args_values_in_variables();
}

// function will panic if not sent 2 arguments as no error checking logic
// in this simple example
fn saving_args_values_in_variables() {
    let args: Vec<String> = env::args().collect();
    // &arg[0] holds path and command name
    let query = &args[1]; // store first argument
    let filename = &args[2]; // store second argument

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
