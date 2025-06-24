use http_server::response::HttpResponse;
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

    let request_lines: Vec<_> = buffer
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if request_lines[0] != "GET / HTTP/1.1" {
        let response =
            HttpResponse::new(404, "text/plain", "This route does not exist".to_string());
        stream.write(response.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = HttpResponse::new(200, "text/plain", "Hello, world!".to_string());
        stream.write(response.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
