use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// HTTP is a text-based protocol
// A request takes this format:
// get has no message body

// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body

// Example below from curl

// Request: GET /test/path HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: curl/7.64.1
// Accept: */*

/// entry point to the program
fn main() {
    // setup tcp listener on address:port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // loop through incoming stream
    for stream in listener.incoming() {
        // stream
        let stream = stream.unwrap();
        // 
        handle_connection(stream);
    }
}
/// handle connection to server
fn handle_connection(mut stream: TcpStream) {
    // create a buffer
    let mut buffer = [0; 512];
    // read stream and put in buffer
    stream.read(&mut buffer).unwrap();
    // print stdout the request received
    println!("\n\nHandle Connection method called");
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}