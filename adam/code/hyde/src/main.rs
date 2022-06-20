use std::fs;
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
    let hello_get = b"GET /@adam/posts/how-to-build-and-deploy-a-blog-in-rust HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        contents = format!("
            <html>
                <title>dev/tails</title>
                <body>
                    <h1>dev/tails</h1>
                    <a href='/@adam/posts/how-to-build-and-deploy-a-blog-in-rust'>How to Build and Deploy a Blog in Rust</a>
                    <div>@adam - Sunday June 19, 2022</div>
                    <!--<div><a href='/@adam'>@adam</a></div>-->
                    <!--<div><a href='/#rust'>#rust</a></div>-->
                </body>
            </html>
        ");
    } else if buffer.starts_with(hello_get) {
        contents = fs::read_to_string("site/@adam/posts/how-to-build-and-deploy-a-blog-in-rust/how-to-build-and-deploy-a-blog-in-rust.md").unwrap();
    } else {
        status_code = 404;
        status_msg = "Not Found";
    }

    let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", status_code, status_msg, contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}