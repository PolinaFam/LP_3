use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io;
use std::str::from_utf8;
mod protector;
use protector::*;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut data = [0 as u8; 50];
            let mut repl = [0 as u8; 50];
            loop{
                let h = get_hash_str();
                let k = get_session_key();
                let new_key = next_session_key(&h,&k);
                println!("Write message: "); 
                let mut msg = String::new();
                io::stdin().read_line(&mut msg);
                let hash = h.clone().into_bytes();
                let key = k.clone().into_bytes();
                let mes = msg.clone().into_bytes();
                stream.write(&hash).unwrap();
                stream.write(&key).unwrap();
                stream.write(&mes).unwrap();
                match stream.read(&mut data) {
                Ok(size) => {
                        stream.read(&mut repl);
                        let text1 = from_utf8(&data[0..size]).unwrap();
                        let text2 = from_utf8(&repl).unwrap();
                        if text1 == new_key {
                            println!("Ключи совпали");
                        } else {break;}
                        println!("reply: {}", text2);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            }
        },
        Err(e) => { 
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}