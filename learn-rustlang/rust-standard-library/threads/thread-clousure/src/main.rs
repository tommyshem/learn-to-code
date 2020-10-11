use std::thread;

fn main() {
    closure_move();
}

// you have to move the vector in to the closure as rust will not let you use it as it will be dropped
// if the closure does not own the vector, as it goes out of scope the vector will be dropped.
fn closure_move() {
    let v = vec![1, 2, 3];
    // need to move the v in to the closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
