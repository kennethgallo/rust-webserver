use std::fs::File;
use std::io::{Read,Write};
use std::net::{TcpStream};

pub fn http_handler(mut stream: TcpStream) {
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();

    get_file(stream, req.path.unwrap()); 
}

pub fn get_file(mut stream: TcpStream, path: &str){

    let full_path = "./www".to_owned() + path;
    let mut file = match File::open(full_path.clone()) {
        Ok(file) => file,
        Err(err) => { 
            eprintln!("Error: {}", err);
            eprintln!("Path requested: {}", path);
            resp_notfound(stream);
            return;
        }
    };

    let mut html = String::new();
    file.read_to_string(&mut html).unwrap();

    let response = format! {
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html

    };
        
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn resp_notfound(mut stream: TcpStream){


    let html_body = "<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1></body></html>";
    let response = format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html_body.len(),
        html_body
    );
        
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

//pub fn get_image(mut stream: TcpStream){
        // Respond to the client
    //let response = "HTTP/1.1 200 OK\r\n\r\n";
    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();

//}
