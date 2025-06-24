use crate::common::{HttpMethod, RouteKey};
use crate::utils::http::Request;
use std::{collections::HashMap, net::TcpListener, net::TcpStream};

pub type RouteHandler = fn(&Request) -> Response;

pub struct Server {
    ip_addr: String,
    port: u16,
    routes: HashMap<RouteKey, RouteHandler>,
}

#[derive(Debug)]
pub enum ServerError {
    BindError(std::io::Error),
    RequestParseError(std::io::Error),
    ResponseWriteError(std::io::Error),
    RouteNotFound(RouteKey),
    MethodNotAllowed(RouteKey),
}

impl Server {
    pub fn new(ip_addr: &str, port: u16) -> Self {
        Self {
            ip_addr: ip_addr.to_owned(),
            port,
            routes: HashMap::new(),
        }
    }

    pub fn listen(&self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(format!("{}:{}", self.ip_addr, self.port));

        println!("Server listening on {}:{}", self.ip_addr, self.port);

        for stream in listener.unwrap().incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("Error handling connection: {}", e)
                }
            }
        }

        Ok(())
    }

    pub fn handle_connection(&self, _: TcpStream) -> () {
        // let request =
    }

    pub fn get(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::POST, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::PUT, path, handler);
    }

    pub fn patch(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::PATCH, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::DELETE, path, handler);
    }

    pub fn head(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::HEAD, path, handler);
    }

    pub fn options(&mut self, path: &str, handler: RouteHandler) {
        self.add_route(HttpMethod::OPTIONS, path, handler);
    }

    pub fn add_route(&mut self, method: HttpMethod, path: &str, handler: RouteHandler) {
        let key: RouteKey = (method, path.to_string());

        if self.routes.contains_key(&key) {
            println!("Route already exists: {:?}", key);
        } else {
            self.routes.insert(key, handler);
        }
    }
}
