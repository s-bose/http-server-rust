use crate::{common::HttpMethod, server::RouteHandler};

pub struct Router {
    routes: Vec<Route>,
}

pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: RouteHandler,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn get() {}
}
