use std::io::{BufReader, prelude::*};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);

    let request_lines: Vec<String> = buffer
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request_lines[0] != "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n";
        let content = "Not Found";
        let length = content.len();

        let response = format!("{status_line}Content-Length: {length}\r\n\r\n{content}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 200 OK\r\n";
        let content = "Hello, world!";
        let length = content.len();

        let response = format!("{status_line}Content-Length: {length}\r\n\r\n{content}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
