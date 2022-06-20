# Simple Web Server in Rust

This is the first part of a series of tutorials for creating and deploying a blog with Rust.

The [Rust Lang book](https://doc.rust-lang.org/book) is a great resource that should not be skipped over.  [Chapter 20: Building a Single-Threaded Web Server](https://doc.rust-lang.org/book/ch20-01-single-threaded.html) is the perfect start to learning to use Rust to server static HTML files.

Follow the tutorial as they have it written, and then come back here for some modifications I have made to prepare the web server for returning dynamic HTML instead of static HTML files. I will be using this as the base code for a few projects.  

```rust
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
```

## Using Named Arguments with `format!`

```rust
contents = format!("
    <html>
        <title>{title}</title>
        <body>
            <h1>{title}</h1>
        </body>
    </html>
", title = title);
```

This is a neat way to use the same variable in multiple places of a format string. See the [std::format](https://doc.rust-lang.org/std/macro.format.html) docs for mor on how this works.

## Wrap-up

The primary difference here is the removal of loading the `hello.html` file.  In the next post this will be replaced by a dynamically generated HTML file based on the structure of of the directory the program is run from.