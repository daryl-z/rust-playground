use std::net::TcpStream;
use std::io::{Write, Read};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!("Response from Server: {}", str::from_utf8(&buffer).unwrap());
}
