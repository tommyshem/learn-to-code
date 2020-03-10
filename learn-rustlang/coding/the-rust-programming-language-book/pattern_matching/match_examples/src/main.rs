fn main() {
    println!("\nmatch numbers");
    match_numbers();
    println!("\nmatch named variables");
    match_named_variables();
    println!("\nmultiple pattern matching");
    multiple_patterns();
    println!("\nrange pattern matching");
    range_patterns();
}

/// match numbers
fn match_numbers() {
    let x = 1;
    // pattern match x
    match x {
        1 => println!("one"),      // match number 1
        2 => println!("two"),      // match number 2
        3 => println!("three"),    // match number 3
        _ => println!("anything"), // if no matches then use this one
    }
}

/// named variables matching
fn match_named_variables() {
    // Option type
    let x = Some(5);
    // basic type
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Because we’re in a new scope inside the match expression,
        // this is a new y variable, not the y we declared
        // at the beginning with the value 10.
        // This new y binding will match any value inside a Some().
        Some(y) => println!("Matched, y = {:?}", y),
        // Default if none of the above match
        _ => println!("Default case, x = {:?}", x),
    }
    // When the match expression is done, its scope ends, and so does
    // the scope of the inner y. The outer y is used in the println!
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

/// multiple pattern matching
fn multiple_patterns() {
    let x = 1;

    match x {
        // multiple pattern matching of 1 or 2
        1 | 2 => println!("one or two"),
        // back to normal pattern matching
        3 => println!("three"),
        // default if above does not match
        _ => println!("anything"),
    }
}

/// match pattern in a range
fn range_patterns() {
    let x = 5;

    match x {
        // match range from 1 to 5
        1..=5 => println!("one through five"),
        // default if above does not match
        _ => println!("something else"),
    }

    // example of range with characters
    let x = 'c';

    match x {
        // match range from a to j
        'a'..='j' => println!("early ASCII letter"),
        // match range from k to z
        'k'..='z' => println!("late ASCII letter"),
        // default if above does not match
        _ => println!("something else"),
    }
}

