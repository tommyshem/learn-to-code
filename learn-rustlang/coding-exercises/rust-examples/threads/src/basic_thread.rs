pub fn basic_thread() {
    let handle = std::thread::spawn(|| {
        // closure
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    handle.join().expect("Failed to join threads"); // the program will wait until thread handle has finished
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    //handle.join().unwrap(); // waits until the handle thread has finished
}
