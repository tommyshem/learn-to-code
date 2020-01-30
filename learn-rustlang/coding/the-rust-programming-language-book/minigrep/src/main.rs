use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	let command_name = &args[0];  // command path and name
	let search = &args[1];  // first argument if passed
	let filename = &args[2];  // second argument if passed 

	println!("\nCommand used {}",command_name);
	println!("Searching for {}",search);
	println!("In File {}\n",filename);
	
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();

	f.read_to_string(&mut contents).expect("Something went wrong reading the file");
	println!("With text:\n{}",contents);	
}
