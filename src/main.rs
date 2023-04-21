/*
    Rust-webserver for Spring 2023-ITCS-4102-090:ITCS-5102-090_Combined
    Students Todd Hetrick, Calvin Hathcock, Chase Starr, Gloria Allison, Jacob Dent, Kenneth Gallo

*/
#![warn(unused_extern_crates)]

mod handlers;
use handlers::http_handler;

use dotenv::dotenv;
use std::env;
use std::net::{TcpListener, SocketAddr};


fn main() {
    
    dotenv().ok();
    let address = env::var("SOCKET_ADDR")
            .expect("Socket address not found in environment variables.");

    let listener = TcpListener::bind(address).unwrap();
  
    for stream in listener.incoming() {
        match stream{
            Ok(stream) => {                
                println!("Connection on  {} from {}", stream.local_addr().unwrap(), stream.peer_addr().unwrap());
                http_handler(stream);
            }
            Err(e) => {
                println!("TCP Client conection Error {}",e);
                
            }
        }
    }
}