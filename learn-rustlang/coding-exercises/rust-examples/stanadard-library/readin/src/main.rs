use std::io::prelude::*;
use std::io::stdin;
use std::io::BufReader;
use std::process;

fn main() {
    let stdin = BufReader::new(stdin());
    for line in stdin.lines() {
        match line {
            Ok(ref x) if x == "quit" => process::exit(1),
            Ok(_) => println!("{:#?}", line),
            Err(err) => println!("error: {:?}", err),
        };
    }
}
