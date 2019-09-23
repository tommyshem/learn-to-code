pub fn channel_multi_values() {
    let (tx, rx) = std::sync::mpsc::channel();

    // Create thread and pass message
    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // send all values to the channel
        for val in vals {
            tx.send(val).expect("Could not send into channel");
            std::thread::sleep(std::time::Duration::from_secs(1)); // sleep for 1 second
        }
    });

    // Main thread
    // this will block main thread until it receives a message from the other thread
    for received in rx {
        println!("Got: {}", received);
    }
}
