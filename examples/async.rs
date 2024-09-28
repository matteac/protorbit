use protorbit::http;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse the arguments, bind the TCP socket we'll be listening to, spin up
    // our worker threads, and start shipping sockets to those worker threads.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            handle(stream).await;
        });
    }
}

async fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();
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

    let _ = stream
        .write(Into::<String>::into(response).as_bytes())
        .await;
}
