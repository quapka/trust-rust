use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    println!("client");

    let stdin = io::stdin();
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:65432") {
        println!("connected");
        for line in stdin.lock().lines() {
            match stream.write(line.unwrap().as_bytes()) {
                Ok(len) => println!("data write okeyed: {}", len),
                Err(e) => println!("data write errored: {}", e.to_string()),
            };
        }
    } else {
        println!("connection failed");
    }
}
