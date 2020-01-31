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
                                      // search for the search query in the line and print it found any part
                                      // in the line.
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    println!("\nFull contents:\n{}", contents); // prinf contents to console output
    Ok(()) // return ok result no return parameters
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
    ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.
    ";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
// lifetime added as 2 str passed in and the compiler does not
// know which one is used. contents passed in and out.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // iterator through the contents one line at a time
    for line in contents.lines() {
        //check contents contains the search query
        if line.contains(query) {
            results.push(line); // push found string into result vector
        }
    }
    results
}

// lifetime added as 2 str passed in and the compiler does not
// know which one is used. contents passed in and out.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query=query.to_lowercase(); // built in function to convert string to lower case
    // query changed to owned string rather and str slice as calling
    // to_lowercase returns a new string and not a str slice
    let mut results = Vec::new();
    // iterator through the contents one line at a time
    for line in contents.lines() {
        //check contents contains the search query
        // now query is a string due to the to_lowercase function call 
        // The contains needs a string slice passed in so
        // we have to use a reference of the string 
        if line.to_lowercase().contains(&query) { 
            results.push(line); // push found string into result vector
        }
    }
    results
}