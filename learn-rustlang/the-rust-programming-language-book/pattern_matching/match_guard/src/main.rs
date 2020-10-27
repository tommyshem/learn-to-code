fn main() {
    println!("\nmatch guard example 1");
    match_guard_example1();
    println!("\nmatch guard example 2");
    match_guard_example2();
}

/// match guard example
fn match_guard_example1() {
    // try changing x value - 10 to 50 or a different number
    let x = Some(10);
    let y = 10;

    match x {
        // match guard by using the if and then guard condition
        // guard is n equal y
        Some(n) if n == y => println!("Matched, n = {}", n),
        // normal match
        Some(50) => println!("Got 50"),
        // default if above is not found
        _ => println!("Default case, x = {:?}", x),
    }
    // print values used
    println!("at the end: x = {:?}, y = {}", x, y);
}

/// match guard example
fn match_guard_example2() {
    let x = 4;
    // try changing y to true
    let y = false;

    match x {
        // match guard only prints if y is equal to true
        4 | 5 | 6 if y => println!("yes"),
        // default if above is not found
        _ => println!("no"),
    }
}
