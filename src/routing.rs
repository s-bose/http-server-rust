use crate::common::HttpMethod;
use crate::request::Request;
use crate::response::HttpResponse;

pub type Handler = fn(&Request) -> std::io::Result<HttpResponse>;

#[derive(Debug, PartialEq)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: Handler,
}

#[derive(Debug, PartialEq)]
pub enum RouteError {
    NotFound,
    MethodNotAllowed,
    RouteAlreadyExists,
}

pub fn match_route(route: &str, incoming: &str) -> bool {
    let route_parts = route.split('/').collect::<Vec<&str>>();
    let incoming_parts = incoming.split('/').collect::<Vec<&str>>();

    if route_parts.len() != incoming_parts.len() {
        return false;
    }

    for (route_part, incoming_part) in route_parts.iter().zip(incoming_parts.iter()) {
        if route_part.starts_with(':') {
            continue;
        }
        if route_part != incoming_part {
            return false;
        }
    }

    true
}

pub trait RouteResolver {
    fn resolve<'a>(
        &self,
        path: &str,
        method: HttpMethod,
        routes: &'a Vec<Route>,
    ) -> Result<&'a Route, RouteError> {
        for route in routes {
            if match_route(&route.path, &path) {
                if route.method == method {
                    return Ok(route);
                }
                return Err(RouteError::MethodNotAllowed);
            }
        }

        Err(RouteError::NotFound)
    }
}

pub trait HTTPHandler {
    type Error;
    fn register_route(&mut self, path: &str, method: HttpMethod, handler: Handler);

    fn get(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::GET, handler)
    }

    fn post(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::POST, handler)
    }

    fn put(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::PUT, handler)
    }

    fn patch(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::PATCH, handler)
    }

    fn delete(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::DELETE, handler)
    }

    fn head(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::HEAD, handler)
    }

    fn options(&mut self, path: &str, handler: Handler) {
        self.register_route(path, HttpMethod::OPTIONS, handler)
    }

    fn add_route(&mut self, method: HttpMethod, path: &str, handler: Handler) {
        self.register_route(path, method, handler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_route() {
        assert!(match_route("/", "/"));
        assert!(match_route("/users", "/users"));
        assert!(match_route("/users/:id", "/users/123"));
        assert_eq!(
            match_route("/users/messages/:message_id", "/users/:userid"),
            false
        );
        assert_eq!(
            match_route(
                "/users/:user_id/messages/:message_id",
                "/users/123/messages/456"
            ),
            true
        );
        assert_eq!(
            match_route("/users/messages/:message_id", "/users/123/messages/456/"),
            false
        );
    }

    #[test]
    fn test_route_resolver() {
        struct TestRouter {
            routes: Vec<Route>,
        }

        impl RouteResolver for TestRouter {}

        let router = TestRouter {
            routes: vec![
                Route {
                    method: HttpMethod::GET,
                    path: "/users".to_string(),
                    handler: |_| Ok(HttpResponse::ok()),
                },
                Route {
                    method: HttpMethod::POST,
                    path: "/users".to_string(),
                    handler: |_| Ok(HttpResponse::ok()),
                },
                Route {
                    method: HttpMethod::GET,
                    path: "/users/:id".to_string(),
                    handler: |_| Ok(HttpResponse::ok()),
                },
                Route {
                    method: HttpMethod::GET,
                    path: "/users/:id/messages/:message_id".to_string(),
                    handler: |_| Ok(HttpResponse::ok()),
                },
            ],
        };

        let route = router.resolve("/users", HttpMethod::GET, &router.routes);
        assert!(route.is_ok());
        assert_eq!(route.unwrap().path, "/users");

        let route = router.resolve("/users/123", HttpMethod::GET, &router.routes);
        assert!(route.is_ok());
        assert_eq!(route.unwrap().path, "/users/:id");

        let route = router.resolve("/users/123/messages/456", HttpMethod::GET, &router.routes);
        assert!(route.is_ok());
        assert_eq!(route.unwrap().path, "/users/:id/messages/:message_id");

        let route = router.resolve("/users/123/messages/456", HttpMethod::POST, &router.routes);
        assert!(route.is_err());
        assert_eq!(route.unwrap_err(), RouteError::MethodNotAllowed);
    }
}
