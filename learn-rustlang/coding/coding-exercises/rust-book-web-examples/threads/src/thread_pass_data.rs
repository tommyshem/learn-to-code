use std::thread;

pub fn thread_pass_data() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // need move keyword so it can use the data v
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
