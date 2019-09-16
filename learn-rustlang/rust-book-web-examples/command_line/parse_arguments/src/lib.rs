use std::error::Error;
use std::fs;

// hold arguments in structure
pub struct Config {
    pub filename: String,
    pub query: String,
}
// implement new config structure
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // 0 path and command 1 filename 2 query < 3
            return Err("not enough arguments passed in");
        }
        // create copies of the strings so the struct owns the strings
        let filename = args[1].clone();
        let query = args[2].clone();

        Ok(Config { filename, query })
    }
}

// return a dynamic error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // ? will return an error and not panic
    println!("With text:\n{}", contents); // print all the contents from the file
    // search the contents and print the lines found
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// need lifetime here - the data returned by the search function will live as long as the
// data passed into the search function in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
