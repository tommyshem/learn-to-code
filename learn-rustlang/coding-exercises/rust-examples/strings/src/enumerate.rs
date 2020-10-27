#![allow(unused_variables, dead_code)]
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn first_word_test() {
    let my_string = String::from("hello world");
    println!("{:?}", first_word(&my_string));
}
