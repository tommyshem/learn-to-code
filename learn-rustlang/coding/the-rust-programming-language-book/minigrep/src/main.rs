use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	let command_name = &args[0];  //the first arg[0] is always the command path and name
	let (search, filename) = parse_args(&args);   

	println!("\nCommand used {}",command_name);
	println!("Searching for {}",search);
	println!("In File {}\n",filename);
	
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();

	f.read_to_string(&mut contents).expect("Something went wrong reading the file");
	println!("With text:\n{}",contents);	
}

// parse command arguments passed in
// args = args vector passed in
// returns tuple of string slices as refs
fn parse_args(args: &[String]) -> (&str,&str){
	let query = &args[1];
	let filename = &args[2];
	(query,filename)

}