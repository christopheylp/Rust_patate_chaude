use std::io::{Read, Write};
use std::net::{TcpStream};
use serde::__private::from_utf8_lossy;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};
use serde_json;
use std::net::{SocketAddr, TcpListener};

#[derive(Debug, Serialize, Deserialize)]
struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Hello,
    Welcome(Welcome),
    // Welcome { version: u8 }
}

fn main() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message = "\"Hello\"";
            let message_byte = message.as_bytes();
            let message_length = message.len() as u32;

            stream.write(&message_length.to_be_bytes()).unwrap();
            println!("{:?}", from_utf8_lossy(message_byte));

            stream.write(&message_byte).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut message_length = [0; 4];
            stream.read(&mut message_length).unwrap();
            println!("{:?}", message_length);

            let message_length = u32::from_be_bytes(message_length);

            let mut buffer = vec![0; message_length as usize];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let text = from_utf8_lossy(&buffer);

                    println!("Reply: {:?}", &text.to_string());
                    let message = serde_json::from_str(&text);
                    match message {
                        Ok(message) => {
                            println!("{:?}", message);
                            match message {
                                Message::Welcome(welcome) => {
                                    println!("Welcome match");
                                    println!("{:?}", welcome);
                                }
                                _ => {
                                    println!("Unknown message type");
                                }
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}