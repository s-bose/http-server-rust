// use std::option::Option;
use std::{collections::HashMap, str::FromStr};

const CRLF: &str = "\r\n";

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(()),
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct Request {
    method: Method,
    path: String,
    version: Version,
}

pub fn parse_req_line(req_line: &str) -> Option<(Method, String, Version)> {
    let parts = req_line.split_whitespace().collect::<Vec<&str>>();

    if parts.len() != 3 {
        None
    } else {
        Some((
            Method::from_str(parts[0]).unwrap(),
            parts[1].to_string(),
            Version::from_str(parts[2]).unwrap(),
        ))
    }
}

pub fn parse_headers(headers: &str) -> HashMap<&str, &str> {
    let mut headers_dict = HashMap::<&str, &str>::new();

    let header_lines = headers.split(CRLF);

    for header in header_lines {
        let parts = header.split(": ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        } else {
            headers_dict.insert(parts[0], parts[1]);
        }
    }

    return headers_dict;
}
