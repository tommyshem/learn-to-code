use parse_arguments::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // parse the arguments and copy into config struct
    let config = Config::new(&args).unwrap_or_else(|err| {
        // closure to output message and terminate program
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // print arguments out
    println!("In file {}", config.filename);
    println!("Searching for {}", config.query);

    // run config and exit if error is passed back
    if let Err(e) = parse_arguments::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
