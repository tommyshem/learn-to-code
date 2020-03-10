// enum message
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
 println!("\nDestructuring a enum");
 destructuring_enum();
 println!("Nested destructuring a enum");
 nested_destructuring_enums();
}

/// destructuring enum
fn destructuring_enum(){
       // create message enum
    let msg = Message::ChangeColor(0, 160, 255);

    // match enum
    match msg {
        // match quit
        Message::Quit => {
            println!("The Quit variant has no data to de-structure.")
        }
        // match move and destructuring the variables
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        // match write
        Message::Write(text) => println!("Text message: {}", text),
        // match change color and destructuring the tuple with variable names
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

fn nested_destructuring_enums(){
    // enum color
    enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// nested enum with color
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
    // create enum
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    // match enum
    match msg {
        // match nested enum destructuring color rgb
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        // match nested enum destructing color hsv
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        // default used if above is not found
        _ => (),
    }
}
