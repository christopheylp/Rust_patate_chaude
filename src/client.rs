mod structs;

use structs::Message;
use std::io::{Read, Write};
use std::net::{TcpStream};
use serde::__private::from_utf8_lossy;
use serde_json;

fn main() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {

            let message = "\"Hello\"";
            write_message_to_server(&mut stream, message);

            let message_length = read_message_server_length(&mut stream);

            let mut buffer = vec![0; message_length as usize];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let text = from_utf8_lossy(&buffer);

                    println!("Reponse du serveur: {:?}", &text.to_string());
                    let message = serde_json::from_str(&text);
                    match message {
                        Ok(message) => {
                            match message {
                                Message::Welcome(welcome) => {
                                    println!("Match ok: {:?}", welcome);
                                    subscribe_new_player(&mut stream);
                                    let server_message_length = read_message_server_length(&mut stream);
                                    println!("{:?}",server_message_length);
                                    let mut buffer = vec![0; server_message_length as usize];
                                    match stream.read(&mut buffer) {
                                        Ok(_) => {
                                            let text = from_utf8_lossy(&buffer);
                                            println!("Reponse du serveur: {:?}", &text.to_string());
                                            let message = serde_json::from_str(&text);
                                            match message {
                                                Ok(message) => {
                                                    match message {
                                                        Message::SubscribeResult(subscribe_result) => {
                                                            println!("Subscribe match: {:?}", subscribe_result);
                                                            loop {
                                                                listen_to_server(&mut stream);
                                                            }
                                                        }
                                                        _ => {
                                                            println!("Match ko");
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("Match ko: {:?}", e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!("Match ko: {:?}", e);
                                        }
                                    }

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

fn write_message_to_server(stream: &mut TcpStream, message: &str) {
    let message_byte = message.as_bytes();
    let message_length = message.len() as u32;

    stream.write(&message_length.to_be_bytes()).unwrap();
    println!("Message envoyé: {:?}", from_utf8_lossy(message_byte));

    stream.write(&message_byte).unwrap();
}

fn read_message_server_length(stream: &mut TcpStream) -> u32 {
    let mut message_length_array = [0; 4];
    stream.read(&mut message_length_array).unwrap();

    let message_length = u32::from_be_bytes(message_length_array);
    println!("Taille message server: {:?}", message_length);

    message_length
}

fn subscribe_new_player(stream: &mut TcpStream) {
    let message = Message::Subscribe(structs::Subscribe {
        name:  "".to_string(),
    });
    let message_json = serde_json::to_string(&message).unwrap();
    write_message_to_server(stream, &message_json);
}

fn listen_to_server(stream: &mut TcpStream) {
    let message_length = read_message_server_length(stream);
    let mut buffer = vec![0; message_length as usize];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let text = from_utf8_lossy(&buffer);
            println!("Reponse du serveur: {:?}", &text.to_string());
            let message = serde_json::from_str(&text);
            match message {
                Ok(message) => {
                    match message {
                        Message::PublicLeaderboard(public_leaderboard) => {
                            println!("Public leaderboard: {:?}", public_leaderboard);
                        }
                        _ => {
                            println!("Unknown message type");
                        }
                    }
                }
                Err(e) => {
                    println!("Match ko: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Match ko: {:?}", e);
        }
    }
}