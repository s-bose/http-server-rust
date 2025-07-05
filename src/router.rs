use crate::{
    common::{HttpMethod, join_path},
    routing::{HTTPHandler, Handler, Route, RouteError, RouteResolver},
};

pub struct Router {
    prefix: String,
    routes: Vec<Route>,
}

pub struct RouteGroup<'a> {
    pub prefix: String,
    pub routes: &'a mut Vec<Route>,
}

impl HTTPHandler for RouteGroup<'_> {
    type Error = RouteError;

    fn register_route(&mut self, path: &str, method: HttpMethod, handler: Handler) {
        self.routes.push(Route {
            path: join_path(&self.prefix, path),
            method,
            handler,
        });
    }
}

impl RouteResolver for Router {}

impl Router {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            routes: Vec::new(),
        }
    }

    pub fn group<F>(&mut self, prefix: &str, config: F)
    where
        F: FnOnce(&mut RouteGroup),
    {
        let mut group = RouteGroup {
            prefix: join_path(&self.prefix, prefix),
            routes: &mut self.routes,
        };

        config(&mut group);
    }
}

impl HTTPHandler for Router {
    type Error = RouteError;

    fn register_route(&mut self, path: &str, method: HttpMethod, handler: Handler) {
        let path = join_path(&self.prefix, path);
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
                    path,
                    method,
                    handler,
                },
            );
        } else {
            self.routes.push(Route {
                path,
                method,
                handler,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::HttpMethod, response::HttpResponse};

    use super::*;

    #[test]
    fn test_group() {
        let mut router = Router::new("/api");
        router.group("/v1", |group| {
            group.get("/users", |_| Ok(HttpResponse::ok()))
        });

        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, HttpMethod::GET);
        assert_eq!(router.routes[0].path, "/api/v1/users");
    }

    #[test]
    fn test_router_register_route() {
        let mut router = Router::new("/api");
        router.register_route("/users", HttpMethod::GET, |_| Ok(HttpResponse::ok()));

        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, HttpMethod::GET);
        assert_eq!(router.routes[0].path, "/api/users");
    }

    #[test]
    fn test_router_http_verbs() {
        let mut router = Router::new("/api");
        router.get("/users", |_| Ok(HttpResponse::ok()));
        router.post("/users", |_| Ok(HttpResponse::ok()));
        router.put("/users", |_| Ok(HttpResponse::ok()));
        router.patch("/users", |_| Ok(HttpResponse::ok()));
        router.delete("/users", |_| Ok(HttpResponse::ok()));
        router.head("/users", |_| Ok(HttpResponse::ok()));
        router.options("/users", |_| Ok(HttpResponse::ok()));

        assert_eq!(router.routes.len(), 7);
        assert_eq!(router.routes[0].method, HttpMethod::GET);
        assert_eq!(router.routes[0].path, "/api/users");
        assert_eq!(router.routes[1].method, HttpMethod::POST);
        assert_eq!(router.routes[1].path, "/api/users");
        assert_eq!(router.routes[2].method, HttpMethod::PUT);
        assert_eq!(router.routes[2].path, "/api/users");
        assert_eq!(router.routes[3].method, HttpMethod::PATCH);
        assert_eq!(router.routes[3].path, "/api/users");
        assert_eq!(router.routes[4].method, HttpMethod::DELETE);
        assert_eq!(router.routes[4].path, "/api/users");
        assert_eq!(router.routes[5].method, HttpMethod::HEAD);
        assert_eq!(router.routes[5].path, "/api/users");
        assert_eq!(router.routes[6].method, HttpMethod::OPTIONS);
        assert_eq!(router.routes[6].path, "/api/users");
    }
}
