use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    HTTP1_1,
    HTTP2_0,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Version::HTTP1_1),
            "HTTP/2.0" => Ok(Version::HTTP2_0),
            _ => Err(()),
        }
    }
}

impl HttpMethod {
    pub fn from_str(method: &str) -> Option<Self> {
        match method.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "PATCH" => Some(HttpMethod::PATCH),
            "DELETE" => Some(HttpMethod::DELETE),
            "HEAD" => Some(HttpMethod::HEAD),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "TRACE" => Some(HttpMethod::TRACE),
            _ => None,
        }
    }
}

pub type RoutePath = String;

pub fn join_path<'a>(prefix: &'a str, path: &'a str) -> String {
    Path::new(prefix)
        .join(path.trim_start_matches('/'))
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_path() {
        assert_eq!(join_path("/api", "/v1/users"), "/api/v1/users");
        assert_eq!(
            join_path("/api", &join_path("/v1", "/users")),
            "/api/v1/users"
        );
        assert_eq!(join_path("/api", "v1/users"), "/api/v1/users"); // path without leading slash
        assert_eq!(join_path("/api", "v1/users/"), "/api/v1/users/"); // path with trailing slash
        assert_eq!(join_path("api", "v1/users/"), "api/v1/users/");
    }
}
