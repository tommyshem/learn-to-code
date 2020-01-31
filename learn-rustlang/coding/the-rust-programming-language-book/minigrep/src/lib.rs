use std::error::Error;
use std::fs::File; // used for opening file
use std::io::prelude::*;

// config structure for holding the arguments passed in from the command line
pub struct Config {
    pub query: String,    // owned string
    pub filename: String, // owned string
}
// parse command arguments passed in
// args = args vector passed in
// returns Config structure
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // check there is 3 or more arguments passed in
            return Err("Not enough arguments - need query and filename"); // if not enough arguments passed in
        }
        let query = args[1].clone(); // clone string so config struct as ownership
        let filename = args[2].clone(); // clone string so config struct as ownership
        Ok(Config { query, filename }) // return config structure
    }
}

// config passed in and load the file
// using ? to return error instead of panic with expect()
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // result returns nothing and box returns any error
    let mut f = File::open(config.filename)?; // try load file

    let mut contents = String::new(); // owned string to load file contents into
    f.read_to_string(&mut contents)?; // read file contents into owned string
                                      // .expect("Something went wrong reading the file"); // if error panic and display message

    println!("With text:\n{}", contents); // prinf contents to console output
    Ok(()) // return ok result no return parameters
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rustt:
safe, fast, productive.
Pick three.
    ";
        assert_eq!(vec!["safe,fast, productive."], search(query, contents));
    }
}
// lifetime added as 2 str passed in and the compiler does not
// know which one is used. contents passed in and out.
pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    vec![]
}