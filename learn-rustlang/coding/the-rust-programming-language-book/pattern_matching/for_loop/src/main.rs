
fn main() {
    // create a vector of characters
    let v = vec!['a', 'b', 'c'];
    // loop through all the characters in the vector
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
