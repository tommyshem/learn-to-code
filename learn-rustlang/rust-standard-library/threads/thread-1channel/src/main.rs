use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // you can not use the val again in the println below because it has been sent in to the channel and 
        // has gone out of scope.
        //    		println!("val is {}",val);
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
