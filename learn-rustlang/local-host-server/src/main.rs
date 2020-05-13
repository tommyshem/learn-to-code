use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Error};

fn main() -> Result<(),Error>{
    // setup socket and tcp listener
    let loop_back_address = Ipv4Addr::new(127,0 ,0,1);
    let socket = SocketAddrV4::new(loop_back_address, 0);  // passing in zero generates a random port number
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    // 
    println!("Listening on {}, access this port to end the program",port);
    // wait for tcp stream
    let (mut tcp_stream,addr) = listener.accept()?;  // block until requested
    // connection received and read stream
    println!("Connection received! {:?} is sending data.",addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    // stdout response received
    println!("{:?} says {}",addr,input);
    Ok(())
}
