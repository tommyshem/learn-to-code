extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guest the number!");
    // Genrate a random number
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    // Create mutable string to hold the guess
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");  // checks result and if err panics with message
    
	// Have to convert to u32 to compare as have to be the same types with a shadow label name
	// using trim to remove white space and return keys. eg /n
	let guess: u32 = match guess.trim().parse() { // match will check result and handle any errors converting
		Ok(num) => num, // get the number out of OK result and  return the number
		Err(_) => {println!("Please type in a number from 1 - 100 !\n"); continue}, // _ means catch all errors. ask fro number continue will goto start of loop skipping all code below.
	};
	
	
    println!("You guessed: {}",guess);

	// match the guess with compare
    match guess.cmp(&secret_number){
    	Ordering::Less => println!("Too small!"),
    	Ordering::Greater => println!("Too big!"),
    	Ordering::Equal => {
    	println!("You guessed right.\nYour a  winner!");
    	break;  // break out of loop
    }
    }
    }
}
