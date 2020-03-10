// use function parameters as tuple - variable names
fn print_coordinates(&(x, y): &(i32, i32)) {
    // use the named variables x and y
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    // create tuple
    let point = (3, 5);
    // pass tuple to the function
    print_coordinates(&point);
}
