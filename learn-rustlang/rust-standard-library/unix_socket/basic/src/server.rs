use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap())
    }
}

fn main() {
    // socket name to use
    let socket_name = "/tmp/my-rust.sock";
    let socket = Path::new(socket_name);
    // delete the socket if it already exits
    if socket.exists() {
        fs::remove_file(&socket).expect("Could not remove file");
    }
    // create socket
    let listener = UnixListener::bind(socket_name).unwrap();
    // listen on the socket
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
