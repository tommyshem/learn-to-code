// struct
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("\ndestructing struct");
    destructing_struct();
    println!("\npattern matching destructing");
    pattern_matching_destructing();
}

// destructing a struct with different names and same names
fn destructing_struct() {
    // create new Point struct with initialization
    let p1 = Point { x: 0, y: 7 };
    
    // p1 is destructed with different names
    let Point { x: a, y: b } = p1;
    println!("Different names used");
    assert_eq!(0, a);
    println!("a: {}",a);
    assert_eq!(7, b);
    println!("b: {}",b);

    // create new Point struct with initialization
      let p2 = Point { x: 2, y: 9 };

    // p2 is destructed with same names
    let Point { x, y } = p2;
    println!("Same names used");
    assert_eq!(2, x);
    println!("x: {}",x);
    assert_eq!(9, y);
    println!("y: {}",y);
}

/// pattern matching destructing
fn pattern_matching_destructing(){
      let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}