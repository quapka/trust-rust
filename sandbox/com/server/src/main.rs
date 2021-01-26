use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("processing the stream: {:?}", stream);
    // let mut reader = BufReader::new(stream);

    let mut buffer = [0; 1024];

    loop {
        let n = stream.read(&mut buffer).unwrap();
        if buffer.iter().take(n).zip(b"goodbye").all(|(a, b)| a == b) {
            break;
        }
        println!("read: {:?}", n);
        println!("data: {:?}", String::from_utf8(buffer.to_vec()));
    }
    println!("client ended the communication");
    stream.shutdown(Shutdown::Both);
}

fn main() -> std::io::Result<()> {
    println!("Server side");

    let listener = TcpListener::bind("127.0.0.1:65432")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
