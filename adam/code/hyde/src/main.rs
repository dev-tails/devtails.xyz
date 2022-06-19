use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut links = vec![];

    for entry in fs::read_dir("posts").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        for post_file in fs::read_dir(path).unwrap() {
            let post_file = post_file.unwrap();
            let post_file_path = post_file.path();

            links.push(format!("{}", post_file_path.as_path().display()));
        }
    }

    let mut contents = String::new();

    
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    let mut status_code = 200;
    let mut status_msg = "OK";
    let get = b"GET / HTTP/1.1\r\n";
    let hello_get = b"GET /posts/@adam/hello HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        for link in links {
            contents = format!("{}<a href='/{}'>{}</a>", contents, link, link);
        }
    } else if buffer.starts_with(hello_get) {
        contents = fs::read_to_string("posts/@adam/hello/hello.md").unwrap();
    } else {
        status_code = 404;
        status_msg = "Not Found";
    }

    let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", status_code, status_msg, contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}