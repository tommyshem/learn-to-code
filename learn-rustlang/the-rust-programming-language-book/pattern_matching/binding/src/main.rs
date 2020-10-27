fn main() {
    enum Message {
        Hello { id: i32 },
    }
    // try changing id value
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // id_variable is binding to id so it can be used in the
            // matched arm(by using the @ after the name)
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // normal range check with no bindings
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        // use the normal id name
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
