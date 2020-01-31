extern crate minigrep;

use std::env; // used to get arguments from command line
use std::process; // used for ending process // used for Error
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // get arguments from the command line
    let command_name = &args[0]; //the first arg[0] is always the command path and name
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\nCommand used {}", command_name);
    println!("Searching for {}", config.query);
    println!("In File {}\n", config.filename);

    if let Err(e) = minigrep::run(config) {
        // check if error return
        println!("Application error: {}", e); // if error then print message and exit
        process::exit(1); // exit program with error code 1
    }
}
