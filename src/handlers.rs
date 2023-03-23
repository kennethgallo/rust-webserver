use std::io::{Read, Write};
use std::net::{TcpStream};

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
}

pub fn http_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut headers = Vec::new();

    stream.read(&mut buffer).unwrap();
    let request_str = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request_str.lines();

    // Parse the first line of the request
    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();

    // Parse the headers
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let mut parts = line.splitn(2, ':');
        let name = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        headers.push((name, value));
    }

    let request = Request {
        method,
        path,
        headers,
    };
    println!("{:#?}", request);

    // Respond to the client
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

