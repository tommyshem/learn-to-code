#[test]
fn if_let_long_test() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

#[test]
fn if_let_short_test() {
    // this is the same as the above but more readable and less code
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
