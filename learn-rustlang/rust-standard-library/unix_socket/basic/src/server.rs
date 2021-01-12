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
    let socket = Path::new("/tmp/my-rust.sock");

    if socket.exists() {
        fs::remove_file(&socket).expect("Could not remove file");
    }

    let listener = UnixListener::bind("/tmp/pfych-rust.sock").unwrap();

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
