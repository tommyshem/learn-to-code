#[derive(Debug)]
#[allow(dead_code)]
//struct defined
pub struct Rectangle {
    width: u32,
    height: u32,
}
//associated-methods
#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
//associated-functions
#[allow(dead_code)]
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[test]
fn associated_method_test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

#[test]
fn associated_function_test() {
    println!("{:#?}", Rectangle::square(30));
}
