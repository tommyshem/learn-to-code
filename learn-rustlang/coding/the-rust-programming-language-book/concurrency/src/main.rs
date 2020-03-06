use std::thread;
use std::time::Duration;
fn main() {
    println!("\nThread Join examples from pages 344-346");
    println!("\nWait until all threads are finished");
    threads_join_wait_until_all_threads_finished();
    println!(
        "\nWait until all the spawned threads are finished, before running the main thread loop"
    );
    threads_join_spawn_threads_first();
    println!("\nMove closure thread from pages 347-349");
    move_closure_threads();
}

/// Example of move variables to the spawned thread
fn move_closure_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // use move so thread takes ownership of values used - v
        println!("Here's a vector from the spawn thread: {:?}", v);
    });
    handle.join().unwrap(); // wait for threads to finish [blocking]
}

pub fn threads_join_wait_until_all_threads_finished() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn threads_join_spawn_threads_first() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // wait for threads to finish [blocking]
}
