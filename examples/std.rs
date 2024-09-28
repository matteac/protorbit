use protorbit::http;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle(stream),
            Err(err) => {
                eprintln!("STREAM ERROR: {}", err);
            }
        }
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    let request =
        http::Request::try_from(String::from_utf8(buffer[0..n].to_vec()).unwrap()).unwrap();

    let mut response_headers = HashMap::new();
    response_headers.insert("Content-Type".to_string(), "text/html".to_string());

    let response = http::Response::new(
        http::Version::HTTP1_1,
        http::StatusCode::NotFound,
        "Not Found".into(),
        response_headers,
        format!("Error 404: {} NotFound", request.path),
    );

    let _ = stream.write(Into::<String>::into(response).as_bytes());
}
