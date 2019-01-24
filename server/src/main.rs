use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
mod protector;
use protector::*;

fn handle_client(mut stream: TcpStream) {
    let mut hash = [0 as u8; 5]; 
    let mut key = [0 as u8; 10];
    let mut mes = [0 as u8;50];
    while match stream.read(&mut hash) {
        Ok(_) => {
            // echo everything!
            stream.read(&mut key);
            stream.read(&mut mes);
            let text1 = from_utf8(&hash).unwrap();
            let text2 = from_utf8(&key).unwrap();
            let new_key = next_session_key(&text1,&text2);
            let result = new_key.clone().into_bytes();
            stream.write(&result).unwrap();
            stream.write(&mes).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}