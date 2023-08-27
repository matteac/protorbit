use protorbit::http;
use std::collections::HashMap;
use std::env;
use std::error::Error;
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
            println!("Accepted connection from {}", stream.peer_addr().unwrap());
            if let Err(e) = handle(stream).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }
}

async fn handle(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    stream.try_read(&mut buffer)?;
    let request = http::Request::from_string(String::from_utf8_lossy(&buffer)).unwrap();
    println!("{:#?}", request);
    let mut response_headers = HashMap::new();
    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
    let response = http::Response::new(
        http::Version::HTTP1_1,
        http::StatusCode::NotFound,
        response_headers,
        format!("Error 404: {} NotFound", request.path),
    );
    stream.try_write(response.to_string().as_bytes())?;
    Ok(())
}
