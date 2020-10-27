#[allow(dead_code)]
pub fn tuple_struct_without_named_fields() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);
}

#[test]
fn tuple_struct_without_named_fields_test() {
    tuple_struct_without_named_fields();
}
