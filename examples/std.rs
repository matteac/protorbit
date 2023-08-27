use protorbit::http;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = http::Request::from_string(String::from_utf8(buffer.to_vec()).unwrap()).unwrap();

    let mut response_headers = HashMap::new();
    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
    let response = http::Response::new(
        http::Version::HTTP1_1,
        http::StatusCode::NotFound,
        response_headers,
        format!("Error 404: {} NotFound", request.path),
    );
    stream.write(response.to_string().as_bytes()).unwrap();
}
