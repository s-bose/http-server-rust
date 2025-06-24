// use std::option::Option;
use std::{collections::HashMap, str::FromStr};

use crate::common::HttpMethod;

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

pub fn parse_req_line(req_line: &str) -> Option<(HttpMethod, String, Version)> {
    let parts = req_line.split_whitespace().collect::<Vec<&str>>();

    if parts.len() != 3 {
        None
    } else {
        Some((
            HttpMethod::from_str(parts[0]).unwrap(),
            parts[1].to_string(),
            Version::from_str(parts[2]).unwrap(),
        ))
    }
}

pub fn parse_headers(headers: &str) -> HashMap<&str, &str> {
    let mut headers_dict = HashMap::<&str, &str>::new();

    let header_lines = headers.split("\r\n");

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
