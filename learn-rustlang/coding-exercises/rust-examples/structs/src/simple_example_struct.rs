#[allow(dead_code)]
pub struct Rectangle {
    width: u32,
    height: u32,
}
#[allow(dead_code)]
pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[test]
fn area_test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
