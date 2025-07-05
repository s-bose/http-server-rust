use crate::common::{HttpMethod, join_path};
use crate::request::{Request, RequestError};
use crate::response::{HttpResponse, write_response};
use crate::router::RouteGroup;
use crate::routing::{HTTPHandler, Handler, Route, RouteError, RouteResolver};

use log::{error, info};
use scoped_threadpool::Pool;
use std::io::BufReader;
use std::time::Duration;
use std::{net::TcpListener, net::TcpStream};

pub struct Server {
    ip_addr: String,
    port: u16,
    routes: Vec<Route>,
    pool_size: Option<usize>,
    read_timeout_ms: Option<Duration>,
    write_timeout_ms: Option<Duration>,
}

#[derive(Debug)]
pub enum ServerError {
    ResponseError(std::io::Error),
}

impl RouteResolver for Server {}

impl HTTPHandler for Server {
    type Error = RouteError;

    fn register_route(&mut self, path: &str, method: HttpMethod, handler: Handler) {
        if let Some(matching_route_idx) = self
            .routes
            .iter()
            .position(|r| r.path == path && r.method == method)
        {
            log::warn!(
                "Route {:?} {:?} already exists and will be overwritten",
                method,
                path
            );
            self.routes.insert(
                matching_route_idx,
                Route {
                    path: path.to_string(),
                    method,
                    handler,
                },
            );
        } else {
            self.routes.push(Route {
                path: path.to_string(),
                method,
                handler,
            });
        }
    }
}

impl Server {
    pub fn new(ip_addr: &str, port: u16, pool_size: Option<usize>) -> Self {
        Self {
            ip_addr: ip_addr.to_owned(),
            port,
            routes: Vec::new(),
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

        let route = match self.resolve(&request.path, request.method.clone(), &self.routes) {
            Ok(route) => route,
            Err(RouteError::MethodNotAllowed) => {
                self.send_response(&mut stream, HttpResponse::method_not_allowed());
                return;
            }
            Err(RouteError::NotFound) => {
                self.send_response(&mut stream, HttpResponse::not_found());
                return;
            }
        };

        let response = (route.handler)(&request);

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

    pub fn group<F>(&mut self, prefix: &str, config: F)
    where
        F: FnOnce(&mut RouteGroup),
    {
        let mut group = RouteGroup {
            prefix: join_path("/", prefix),
            routes: &mut self.routes,
        };

        config(&mut group);
    }

    fn send_response(&self, stream: &mut TcpStream, response: HttpResponse) {
        if let Err(err) = write_response(stream, response) {
            error!("Error writing response: {:?}", err);
        }
    }
}

impl Server {
    /// Get route parameters for a request
    pub fn get_route_params(&self, request: &Request) -> HashMap<String, String> {
        // Find the matching route pattern
        for ((method, route_pattern), _) in self.routes.iter() {
            if method == &request.method
                && crate::routing::match_route(route_pattern, &request.path)
            {
                return self.extract_params(route_pattern, &request.path);
            }
        }
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_group() {
        let mut server = Server::new("127.0.0.1", 8080, None);
        server.group("/api", |group| {
            group.get("/users", |_| Ok(HttpResponse::ok()));
        });

        assert_eq!(server.routes.len(), 1);
        assert_eq!(server.routes[0].path, "/api/users");
        assert_eq!(server.routes[0].method, HttpMethod::GET);
    }
}
