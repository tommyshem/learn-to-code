use std::env;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_name = &args[0]; //the first arg[0] is always the command path and name
    let config = Config::new(&args);

    println!("\nCommand used {}", command_name);
    println!("Searching for {}", config.query);
    println!("In File {}\n", config.filename);

    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// config structure for holding the arguments passed in from the command line
struct Config {
    query: String,    // owned string
    filename: String, // owned string
}
// parse command arguments passed in
// args = args vector passed in
// returns Config structure
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone(); // clone string so config struct as ownership
        let filename = args[2].clone(); // clone string so config struct as ownership
        Config { query, filename } // return config structure
    }
}
