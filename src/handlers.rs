use std::fs::File;
use std::io::{Read,Write};
use std::net::{TcpStream};
use std::env;

pub fn http_handler(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(len) => len,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };
    

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let _res = match req.parse(&buffer) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return; 
        }
    };

    get_file(stream, req.path.unwrap()); 
}

// function that gets the file for web server
pub fn get_file(stream: TcpStream, path: &str) {

    let root: String = env::var("FILE_ROOT")
            .expect("File root not found in environment variables.");

    let mut path = root + path;

    let file_name = path.split('/').last().unwrap_or("");

    if file_name == ""{
        path = path + "index.html";
    }


    let file_extension = match path.split('.').last() {
        Some(ext) => ext,
        None => ""
    };

    match file_extension {
        "html" => {
            get_html(stream, &path);
        },
        "png" => {
            get_image(stream, &path);
        },
        _ => { 
            resp_notfound(stream);
        }
    };
}

// function to transmit a requested html page to a client
pub fn get_html(mut stream: TcpStream, path: &String) {

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => { 
            eprintln!("Error: {}", err);
            eprintln!("Path requested: {}", &path);
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

//function to transmit a requested image to a client 
pub fn get_image(mut stream: TcpStream, path: &String) {

    let headers = "HTTP/1.1 200 OK\r\nContent-Type: image/png\r\n\r\n";
    stream.write(headers.as_bytes()).unwrap();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => { 
            eprintln!("Error: {}", err);
            eprintln!("Path requested: {}", &path);
            resp_notfound(stream);
            return;
        }
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}
//Function to display an error message when the client requests a resource that does not exist.
pub fn resp_notfound(mut stream: TcpStream){

    let html_body = "<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1>rust-webserver</body></html>";
    let response = format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html_body.len(),
        html_body
    );
        
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}