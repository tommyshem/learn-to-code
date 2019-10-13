// Both do the same thing
// you can use match or if let else

#[test]
fn match_test() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("The value is  {:?}!", some_u8_value),
        _ => println!("No match found"),
    }
}

#[test]
fn if_let_else_test() {
    //let some_u8_value = Some(0u8);
    if let some_u8_value = Some(3) {
        println!("The value is  {:?}!", some_u8_value);
    } else {
        print!("No match found");
    }
}
