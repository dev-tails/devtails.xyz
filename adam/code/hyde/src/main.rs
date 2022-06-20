// src/main.rs
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut contents = String::new();

    let mut status_code = 200;
    let mut status_msg = "OK";
    let get = b"GET / HTTP/1.1\r\n";

    let title = "Blog";

    if buffer.starts_with(get) {
        contents = format!("
            <html>
                <title>{title}</title>
                <body>
                    <h1>{title}</h1>
                </body>
            </html>
        ", title = title);
    } else {
        status_code = 404;
        status_msg = "Not Found";
    }

    let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", status_code, status_msg, contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}