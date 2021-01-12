use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    //create unix socket
    let mut socket = match UnixStream::connect("/tmp/my-rust.sock") {
        Ok(socket) => socket,
        Err(e) => {
            println!("Could not connect {:?}", e);
            return;
        }
    };
    //write to the socket
    match socket.write_all(b"Hello World!") {
        Ok(_) => {}
        Err(_) => panic!("Could not send message"),
    }
    // // read from the socket
    // let mut response = String::new();
    // match socket.read_to_string(&mut response) {
    //     Ok(_) => {}
    //     Err(_) => panic!("Could not read message"),
    // }
    // println!("{}", response);

    println!("{:?}", socket)
}
