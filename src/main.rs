use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");

    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Failed to read stream");

        println!("Request:\n{}", String::from_utf8_lossy(&buffer));

        let html = fs::read_to_string("public/index.html").unwrap_or_else(|_| {
            "<h1>Failed to load page</h1>".to_string()
        });

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
            html
        );

        stream.write_all(response.as_bytes()).expect("Failed to write response");
    }
}

