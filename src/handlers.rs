use std::io::{Read};
use std::net::{TcpStream};

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    host: String,
    user_agent: String
}

pub fn http_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
   
    stream.read(&mut buffer).unwrap();
    let request_str = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request_str.lines();


    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();

    // Parse the header lines. Not sure if we need this but keep incase we need other header data.
   /*  for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let mut parts = line.splitn(2, ':');
        let name = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        //println!("Header name:{} val{}",name,value);
        
    }*/

    // if req a file call get_file

    //if req an image call get_image

    println!("method:{} path:{}",method,path);


}

pub fn get_file(mut stream: TcpStream){
        // Respond to the client
    //let response = "HTTP/1.1 200 OK\r\n\r\n";
    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();

}

pub fn get_image(mut stream: TcpStream){
        // Respond to the client
    //let response = "HTTP/1.1 200 OK\r\n\r\n";
    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();

}

