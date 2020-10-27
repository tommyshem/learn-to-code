mod shared_state;

use std::sync::mpsc;
use std::thread; // threads
use std::time::Duration; // time // channels

fn main() {
    // threads from pages 344-346
    println!("\nThread Join examples from pages 344-346");
    println!("\nWait until all threads are finished");
    threads_join_wait_until_all_threads_finished();
    println!(
        "\nWait until all the spawned threads are finished, before running the main thread loop"
    );
    threads_join_spawn_threads_first();
    // move closures in threads from pages 347-349
    println!("\nMove closure thread from pages 347-349");
    move_closure_threads();
    // message passing between threads from pages 349-352
    println!("\nmessage passing between threads from pages 349-352");
    message_passing_between_threads();
    // message passing multiple values from pages 353-354
    println!("\nmessage passing between threads sending multiple values from pages 353-354");
    message_passing_between_threads_sending_multiple_values();
    // message passing multiple values using clone from pages 354-355
    println!("\nmessage passing between threads sending multiple values using clone");
    message_passing_between_threads_sending_multiple_values_using_clone();
    // mutex in single thread from page 356
    println!("\nmutex in single thread from page 356");
    shared_state::mutex::mutex_example();
    // sharing mutex between multiple threads from pages 357-
    println!("sharing mutex between multiple threads from pages 357-");
    shared_state::mutex::sharing_mutex_between_threads_example();
}

/// message passing between threads using multiple values using clone
fn message_passing_between_threads_sending_multiple_values_using_clone() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(3));
        }
    });

    thread::spawn(move || {
        let vals1 = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val1 in vals1 {
            tx.send(val1).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        // [blocked] wait until the thread drops the channel
        println!("Got: {}", received);
    }
}

/// message passing between threads using multiple values
fn message_passing_between_threads_sending_multiple_values() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    for received in rx {
        // [blocked] wait until the thread drops the channel
        println!("Got: {}", received);
    }
}

/// message passing between threads
fn message_passing_between_threads() {
    // mpsc stands for multiple producer, single consumer
    // multiple sending ends but only one receiving end
    // returns a tuple first value sending end and the second value is the receiving end
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap(); // send message using the channel to the main thread
                               // send() takes ownership of the value sent and can not be used
    });
    let received = rx.recv().unwrap(); // [Blocking] will wait for message from thread
    println!("Got: {}", received);
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

/// thread join and wait until all threads are finished
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

/// wait until all spawned threads are run first and then run the main thread loop
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
