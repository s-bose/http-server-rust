use crate::common::{HttpMethod, RouteKey};
use crate::request::{Request, RequestError};
use crate::response::HttpResponse;
use log::info;
use scoped_threadpool::Pool;
use std::io::{BufReader, prelude::*};
use std::time::Duration;
use std::{collections::HashMap, net::TcpListener, net::TcpStream};

pub type RouteHandler = fn(&Request) -> HttpResponse;

pub struct Server {
    ip_addr: String,
    port: u16,
    routes: HashMap<RouteKey, RouteHandler>,
    pool_size: Option<usize>,
    timeout_ms: Option<u64>,
}

#[derive(Debug)]
pub enum ServerError {
    BindError(std::io::Error),
    RequestParseError(RequestError),
    ResponseWriteError(std::io::Error),
    RouteNotFound(RouteKey),
    MethodNotAllowed(RouteKey),
}

impl Server {
    pub fn new(
        ip_addr: &str,
        port: u16,
        pool_size: Option<usize>,
        timeout_ms: Option<u64>,
    ) -> Self {
        Self {
            ip_addr: ip_addr.to_owned(),
            port,
            routes: HashMap::new(),
            pool_size,
            timeout_ms,
        }
    }

    pub fn listen(&self) -> ! {
        let listener = TcpListener::bind(format!("{}:{}", self.ip_addr, self.port))
            .expect("Error starting server");

        info!("Server listening on {}:{}", self.ip_addr, self.port);

        self.listen_with_pool(self.pool_size, self.timeout_ms, listener);
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let buffer = BufReader::new(&mut stream);
        let request = Request::from_stream(buffer);

        match request {
            Ok(request) => {
                // Check if route exists but method is different
                let route_key = (request.method.clone(), request.path.clone());
                let method_not_allowed = self.routes.keys().any(|(_, path)| path == &request.path)
                    && !self.routes.contains_key(&route_key);

                let mut response = if method_not_allowed {
                    HttpResponse::new(405, "text/plain")
                } else if let Some(handler) = self.routes.get(&route_key) {
                    handler(&request)
                } else {
                    HttpResponse::new(404, "text/plain")
                };

                // Add Connection: close header to ensure clean connection handling
                response.add_header("Connection", "close");
                self.write_response(&mut stream, response);
            }
            Err(_) => {
                let mut error_response = HttpResponse::new(400, "text/plain");
                error_response.add_header("Connection", "close");
                self.write_response(&mut stream, error_response)
            }
        }
    }

    pub fn listen_with_pool(
        &self,
        pool_size: Option<usize>,
        timeout_ms: Option<u64>,
        listener: TcpListener,
    ) -> ! {
        let logical_cores = num_cpus::get() as u32;
        let pool_size = pool_size.unwrap_or(logical_cores as usize);

        let mut pool = Pool::new(pool_size as u32);
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(20));

        let mut incoming = listener.incoming();

        loop {
            let stream = incoming
                .next()
                .unwrap()
                .expect("Error accepting TCP connection");

            stream
                .set_read_timeout(Some(timeout))
                .expect("Error setting read timeout on socket");

            pool.scoped(|scope| {
                scope.execute(|| {
                    self.handle_connection(stream);
                });
            })
        }
    }

    fn write_response(&self, stream: &mut TcpStream, response: HttpResponse) {
        stream.write_all(response.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
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
