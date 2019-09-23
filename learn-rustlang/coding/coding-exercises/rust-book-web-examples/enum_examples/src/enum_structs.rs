//  Quit has no data associated with it at all.
//  Move includes an anonymous struct inside it.
//  Write includes a single String.
//  ChangeColor includes three i32 values.
#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn enum_struct_test() {
    let _message = Message::Move { x: 6, y: 7 };
}
