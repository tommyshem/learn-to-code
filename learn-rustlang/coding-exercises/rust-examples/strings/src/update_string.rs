#[test]
fn update_string_test() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("Updated string {}", s);
}

#[test]
fn update_slice_string_test() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

#[test]
fn concatenation_string_test() {
    #![allow(unused_variables)]
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

#[test]
fn unwiedly_concatenation_string_test() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Concatenation string {}", s);
}

// For more complicated string combining, we can use the format! macro:
// does the same as above but using thge format! instead of +
#[test]
fn format_string_test() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Concatenation format string {}", s);
}
