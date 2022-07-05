
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::str::from_utf8;
use serde::__private::from_utf8_lossy;

fn main() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message = "\"Hello\"";
            let message_byte = message.as_bytes();
            let message_length = message.len() as u32;
            stream.write(&message_length.to_be_bytes()).unwrap();
            println!("{:?}", message_byte);
            stream.write(&message_byte).unwrap();

            println!("Sent Hello, awaiting reply...");
            let mut length_buffer = [0; 4];
            let buffer_response = stream.read(&mut length_buffer).unwrap();

            let mut buffer = vec![0; buffer_response];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let text = from_utf8_lossy(&buffer);
                    println!("Reply: {:?}", &text.to_string());
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}