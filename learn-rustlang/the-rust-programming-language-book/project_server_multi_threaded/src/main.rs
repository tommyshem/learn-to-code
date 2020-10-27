use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

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
    let address: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();
    println!("Connect to Sever on {}\n", address);
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

    let (header_response, contents_filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK\r\n\r\n", "main_page.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "main_page.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(contents_filename).unwrap();
    let server_response = format!("{}{}", header_response, contents);
    // print out response for debugging purpose
    println!("Response: {}", server_response);

    // send response from the server
    stream.write(server_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// server response header for a blank page
fn response_blank_page() -> String {
    return "HTTP/1.1 200 OK\r\n\r\n".to_string();
}
