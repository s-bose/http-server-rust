use crate::http::{HttpMethod, HttpResponse, Request};

pub type RouteHandler = fn(&Request) -> std::io::Result<HttpResponse>;

#[derive(Debug, PartialEq)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: RouteHandler,
}

#[derive(Debug, PartialEq)]
pub enum RouteError {
    NotFound,
    MethodNotAllowed,
}
