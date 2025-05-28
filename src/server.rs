use crate::response::Response;
use crate::utils::http::Request;
use std::{collections::HashMap, net::TcpListener, net::TcpStream};

pub struct Server {
    ip_addr: String,
    port: u16,
}

impl Server {
    pub fn new(ip_addr: &str, port: u16) -> Self {
        Self {
            ip_addr: ip_addr.to_owned(),
            port,
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.ip_addr, self.port));

        if listener.is_err() {
            return Err(listener.err().unwrap());
        } else {
            for stream in listener.unwrap().incoming() {
                let stream = stream.unwrap();
                self.handle_connection(stream);
            }
        }

        Ok(())
    }

    pub fn handle_connection(&self, _stream: TcpStream) {
        println!("Connection established!");
    }

    pub fn route(&self, path: &str) {}
}
