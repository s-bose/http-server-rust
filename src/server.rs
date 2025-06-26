use crate::common::{HttpMethod, RouteKey};
use crate::request::{Request, RequestError};
use crate::response::{HttpResponse, write_response};
use std::io::Result;

use log::{error, info};
use scoped_threadpool::Pool;
use std::io::BufReader;
use std::time::Duration;
use std::{collections::HashMap, net::TcpListener, net::TcpStream};

pub type RouteHandler = fn(&Request) -> Result<HttpResponse>;

pub struct Server {
    ip_addr: String,
    port: u16,
    routes: HashMap<RouteKey, RouteHandler>,
    pool_size: Option<usize>,
    read_timeout_ms: Option<Duration>,
    write_timeout_ms: Option<Duration>,
}

#[derive(Debug)]
pub enum ServerError {
    ResponseError(std::io::Error),
}

impl Server {
    pub fn new(ip_addr: &str, port: u16, pool_size: Option<usize>) -> Self {
        Self {
            ip_addr: ip_addr.to_owned(),
            port,
            routes: HashMap::new(),
            pool_size,
            read_timeout_ms: Some(Duration::from_millis(100_000)),
            write_timeout_ms: Some(Duration::from_millis(100_000)),
        }
    }

    pub fn with_read_timeout(self, timeout_ms: Duration) -> Self {
        let mut server = self;
        server.read_timeout_ms = Some(timeout_ms);
        server
    }

    pub fn with_write_timeout(self, timeout_ms: Duration) -> Self {
        let mut server = self;
        server.write_timeout_ms = Some(timeout_ms);
        server
    }

    pub fn with_timeout(self, timeout_ms: Duration) -> Self {
        let mut server = self;
        server.read_timeout_ms = Some(timeout_ms);
        server.write_timeout_ms = Some(timeout_ms);
        server
    }

    pub fn listen(&self) -> ! {
        let listener = TcpListener::bind(format!("{}:{}", self.ip_addr, self.port))
            .expect("Error starting server");

        info!("Server listening on {}:{}", self.ip_addr, self.port);

        self.listen_with_pool(self.pool_size, listener);
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let request = match Request::read(BufReader::new(&mut stream)) {
            Err(
                RequestError::ReadError(e)
                | RequestError::ParseError(e)
                | RequestError::InvalidRequest(e),
            ) => {
                error!("Error reading request: {:?}", e);
                self.send_response(&mut stream, HttpResponse::internal_server_error());
                return;
            }
            Err(RequestError::RequestTooLarge) => {
                error!("Request too large");
                self.send_response(&mut stream, HttpResponse::request_entity_too_large());
                return;
            }
            Err(RequestError::ConnectionClosed) => {
                info!("Client connection closed");
                return;
            }
            Err(RequestError::ConnectionTimedOut) => {
                error!("Client connection timed out");
                return;
            }
            Ok(request) => request,
        };

        // Check if route exists but method is different
        let route_key = (request.method.clone(), request.path.clone());
        let method_not_allowed = self.routes.keys().any(|(_, path)| path == &request.path)
            && !self.routes.contains_key(&route_key);

        let response = if method_not_allowed {
            Ok(HttpResponse::method_not_allowed())
        } else if let Some(handler) = self.routes.get(&route_key) {
            handler(&request)
        } else {
            Ok(HttpResponse::not_found())
        };

        match response {
            Ok(response) => {
                self.send_response(&mut stream, response);
            }
            Err(err) => {
                error!("Error writing response: {:?}", err);
                self.send_response(&mut stream, HttpResponse::internal_server_error());
            }
        }
    }

    pub fn listen_with_pool(&self, pool_size: Option<usize>, listener: TcpListener) -> ! {
        let logical_cores = num_cpus::get() as u32;
        let pool_size = pool_size.unwrap_or(logical_cores as usize);

        let mut pool = Pool::new(pool_size as u32);

        let mut incoming = listener.incoming();

        loop {
            let mut stream = incoming
                .next()
                .unwrap()
                .expect("Error accepting TCP connection");

            if let Err(e) = stream.set_read_timeout(self.read_timeout_ms) {
                error!("Error setting read timeout: {:?}", e);
                self.send_response(&mut stream, HttpResponse::internal_server_error());
            }

            if let Err(e) = stream.set_write_timeout(self.write_timeout_ms) {
                error!("Error setting write timeout: {:?}", e);
                self.send_response(&mut stream, HttpResponse::internal_server_error());
            }

            pool.scoped(|scope| {
                scope.execute(|| {
                    self.handle_connection(stream);
                });
            })
        }
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

    fn send_response(&self, stream: &mut TcpStream, response: HttpResponse) {
        if let Err(err) = write_response(stream, response) {
            error!("Error writing response: {:?}", err);
        }
    }
}
