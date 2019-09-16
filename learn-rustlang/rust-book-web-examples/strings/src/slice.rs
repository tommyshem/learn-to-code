#![allow(unused_variables, dead_code)]
pub fn slice_example(arg: &str) {
    let s = String::from("test sentence");

    // First to bytes as a slice using range ..
    let slice1 = &s[0..2];
    println!("{:?}", slice1);
    let slice2 = &s[..2]; // same as slice1
    println!("{:?}", slice2);

    // from set point to end as slice using range ..
    let s1 = String::from("hello");
    let len = s1.len();
    let slice3 = &s1[3..len];
    let slice4 = &s1[3..]; // same as above
    println!("{:?}", slice3);
    println!("{:?}", slice4)
}

#[test]
fn slice_example_test() {
    slice_example("test");
}
