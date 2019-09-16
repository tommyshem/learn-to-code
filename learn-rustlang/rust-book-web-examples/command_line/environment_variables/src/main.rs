use std::env;

fn main() {
    let key = "HOME";
    // check if the key is set and get value
    match env::var(key) {
        Ok(val) => println!("Key = {} : value ={:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
}
