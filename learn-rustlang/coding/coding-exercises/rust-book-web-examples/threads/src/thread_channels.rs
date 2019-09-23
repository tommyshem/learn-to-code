pub fn thread_channel() {
    let (tx, rx) = std::sync::mpsc::channel();

    // Create thread and pass message
    std::thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).expect("Channel should not send message");
        // println!("val is {}",val); // val can not be used as it has been passed
    });
    // this will block main thread until it receives a message from the other thread
    let received = rx.recv().expect("Channel could not receive message");
    // print message received from the thread
    println!("Got: {}", received);
}
