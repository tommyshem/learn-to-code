use std::thread;
use std::time::Duration;

fn main() {
    // create seperate thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // main thread
    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // blocks main thread until the spawned thread is finished
    handle.join().unwrap();
}
