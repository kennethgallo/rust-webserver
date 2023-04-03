use std::fs::File;
use std::io::{Read,Write};
use std::net::{TcpStream};

pub fn http_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
   
    stream.read(&mut buffer).unwrap();
    let request_str = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request_str.lines();


    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let mut path = parts.next().unwrap().to_string();

    path = "./www".to_owned()+ &path;

    println!("method:{} path:{}",method,path);

    get_file(stream, path);


}

pub fn get_file(mut stream: TcpStream, path: String){

    let mut file = match File::open(path.clone()) {
        Ok(file) => file,
        Err(err) => { 
            eprintln!("Error: {}", err);
            eprintln!("Path requested: {}", path);
            return;}
    };

    let mut html = String::new();
    file.read_to_string(&mut html).unwrap();

        
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(html.as_bytes());
    stream.flush();
    

}

pub fn get_image(mut stream: TcpStream){
        // Respond to the client
    //let response = "HTTP/1.1 200 OK\r\n\r\n";
    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();

}

