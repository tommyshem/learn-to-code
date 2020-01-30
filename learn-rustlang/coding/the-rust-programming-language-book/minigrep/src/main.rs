use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command_name = &args[0]; //the first arg[0] is always the command path and name
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\nCommand used {}", command_name);
    println!("Searching for {}", config.query);
    println!("In File {}\n", config.filename);
    run(config);
}

// config passed in and load the file
fn run(config: Config) {
    let mut f = File::open(config.filename).expect("File not found"); // try load file

    let mut contents = String::new(); // owned string to load file contents into
    f.read_to_string(&mut contents) // read file contents into owned string
        .expect("Something went wrong reading the file"); // if error panic and display message

    println!("With text:\n{}", contents); // prinf contents to console output
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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // check there is 3 or more arguments passed in
            return Err("Not enough arguments - need query and filename"); // if not enough arguments passed in
        }
        let query = args[1].clone(); // clone string so config struct as ownership
        let filename = args[2].clone(); // clone string so config struct as ownership
        Ok(Config { query, filename }) // return config structure
    }
}
