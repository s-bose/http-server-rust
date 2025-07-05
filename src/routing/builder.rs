use crate::http::HttpMethod;
use crate::routing::route::RouteHandler;

pub trait RouteBuilder {
    type Error;
    fn register(&mut self, path: &str, method: HttpMethod, handler: RouteHandler);
}
