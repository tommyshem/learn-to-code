fn main() {
    let favorite_color: Option<&str> = None; // add string color to see different result
    let is_tuesday = false;  // change to true to see different result
    let age: Result<u8, _> = "34".parse(); // parse string into another type

    // using the 'if let' instead of a 'match' as it gives more flexibility
    // print background to stdout
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday { 
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

