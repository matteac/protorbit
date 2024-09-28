# ProtOrbit HTTP Library

**ProtOrbit** is a lightweight HTTP library for Rust that provides functionality for parsing and generating HTTP requests and responses. Currently, the library focuses on parsing HTTP messages, but it is designed to be extended with additional features in the future.

## Installation

Add the following line to your `Cargo.toml` file to include ProtOrbit in your project:

```toml
[dependencies]
protorbit = "0.2.2"
```

## Usage

ProtOrbit provides a simple API for working with HTTP messages. Here are two examples of how to use ProtOrbit to handle incoming HTTP requests using the standard library and the Tokio runtime.

### You can run these examples cloning the repo and running `cargo run --example=[async|std]`

### Standard Library Example

```rust
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
```

### Tokio Example

```rust
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

```

## Future Plans

While ProtOrbit currently focuses on HTTP message parsing, future versions of the library will be extended to include additional features and functionalities, making it even more powerful and versatile for various HTTP-related tasks.

## License

This project is licensed under the terms of the MIT license.

---

For any questions, issues, or contributions, please refer to the [GitHub repository](https://github.com/matteac/protorbit) of the project.
