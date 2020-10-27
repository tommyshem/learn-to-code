use std::collections::HashMap;
// Hashmap can only have one value per key and the key is un
//
fn main() {
    // create Hashmap with capacity of ten - used if size is known.
    let mut capmap: HashMap<&str, i32> = HashMap::with_capacity(10);
    capmap.insert("test", 34);
    // Shrink capacity
    capmap.shrink_to_fit();

    // create hashmap with 0 capacity and will grow when key,value pairs added
    let mut marks = HashMap::new();

    // Add to Hashmap - insert(key,value)
    marks.insert("Maths", 65);
    marks.insert("English", 80);

    // insert a key only if it doesn't already exist
    marks.entry("English").or_insert(50);
    marks.entry("German").or_insert(60);

    // replace value in Hashmap
    marks.insert("Maths", 70);

    //check if hashmap contain key
    if marks.contains_key("Maths") {
        println!("Key Maths in hashmap")
    }

    // Returns the number of elements in the map
    println!("Elements = {}", marks.len());

    // Print Hashmap
    println!("{:?}", marks);

    // check if hashmap is empty
    if marks.is_empty() {
        println!("Hashmap is empty")
    }

    //Get a single value - Check for Maths key
    match marks.get("Maths") {
        Some(mark) => println!("You got {} for Maths.", mark), // if key matches Maths
        None => println!("No mark for Maths"),                 // if no Maths key
    }
    // remove key value pair if in Hashmap
    marks.remove("Maths");

    //Removes a key from the map, returning the stored key and value if the key was previously in the map
    match marks.remove_entry("Maths") {
        Some((key, value)) => println!("Key {} Value {}", key, value), // if key matches Maths
        None => println!("No mark for Maths"),                         // if no Maths key
    }

    // loop through hashmap - (key,value) can be renamed to anything
    // for (key,valve) in marks {
    for (subject, mark) in &marks {
        println!("Subject {} you got mark {}%", subject, mark)
    }

    // loop through keys
    for key in marks.keys() {
        println!("{}", key);
    }
    // loop through values
    for val in marks.values() {
        println!("{}", val);
    }

    // Update all values
    for (_, val) in marks.iter_mut() {
        *val *= 2;
    }

    // loop through values and change them
    for val in marks.values_mut() {
        *val = *val + 10;
    }

    //insert vectors into Hashmap teams
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    //Updating old values in Hashmap
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //not in hashmap then insert key with value 0
        *count += 1;
    }
    println!("{:?}", map);

    // A HashMap with fixed list of elements can be initialized from an array
    let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();
    // use the values stored in map
    println!("{:?}", timber_resources);
}
