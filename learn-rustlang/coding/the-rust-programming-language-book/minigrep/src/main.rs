use std::env; // used to get arguments from command line
use std::error::Error;
use std::fs::File; // used for opening file
use std::io::prelude::*;
use std::process; // used for ending process // used for Error

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

    if let Err(e) = run(config) {
        // check if error return
        println!("Application error: {}", e); // if error then print message and exit
        process::exit(1); // exit program with error code 1
    }
}

// config passed in and load the file
// using ? to return error instead of panic with expect()
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // result returns nothing and box returns any error
    let mut f = File::open(config.filename)?; // try load file

    let mut contents = String::new(); // owned string to load file contents into
    f.read_to_string(&mut contents)?; // read file contents into owned string
                                      // .expect("Something went wrong reading the file"); // if error panic and display message

    println!("With text:\n{}", contents); // prinf contents to console output
    Ok(()) // return ok result no return parameters
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
