use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() {
// connect to the unix socket  
let mut socket = match UnixStream::connect("/tmp/pfych-rust.sock") {
    Ok(socket) => socket,
    Err(e) => {
        println!("Could not connect {:?}", e);
        return
    }
};

// write to the stream
match socket.write_all(b"Hello World!") {
    Ok(_) => {},
    Err(_) => panic!("Could not send message")
}
// read from the stream
let mut response = String::new();
match socket.read_to_string(&mut response){
    Ok(_) => {},
    Err(_) => panic!("Could not read message") 
}
println!("{}",response);
// debug socket
println!("{:?}", socket);
}

