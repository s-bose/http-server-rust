use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use crate::{
    common::{HttpMethod, RoutePath},
    utils::http::Version,
};

#[derive(Debug)]
pub enum RequestError {
    ReadError(std::io::Error),
    InvalidRequest(std::io::Error),
}

pub struct Request {
    pub method: HttpMethod,
    pub path: RoutePath,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn from_stream<R: Read>(mut buffer: BufReader<R>) -> Result<Self, RequestError> {
        // Parse request line and headers (until empty line)
        let mut lines = Vec::new();
        let mut line = String::new();
        while buffer
            .read_line(&mut line)
            .map_err(RequestError::ReadError)?
            > 0
        {
            if line.trim().is_empty() {
                break; // End of headers
            }
            lines.push(line.trim().to_string());
            line.clear();
        }

        if lines.is_empty() {
            return Err(RequestError::InvalidRequest(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Connection closed or empty request",
            )));
        }

        // Parse request line
        let (method, path, version) = Self::parse_request_line(&lines[0])?;

        // Parse headers
        let headers = Self::parse_headers(&lines[1..]);

        // Parse body (read remaining content)
        let body = Self::parse_body(&mut buffer, &headers)?;

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    fn parse_request_line(line: &str) -> Result<(HttpMethod, String, Version), RequestError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(RequestError::InvalidRequest(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid request line",
            )));
        }

        let method = HttpMethod::from_str(parts[0]).ok_or({
            RequestError::InvalidRequest(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid HTTP method",
            ))
        })?;

        let version = Version::from_str(parts[2]).map_err(|_| {
            RequestError::InvalidRequest(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid HTTP version",
            ))
        })?;

        Ok((method, parts[1].to_string(), version))
    }

    fn parse_headers(lines: &[String]) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }
        headers
    }

    fn parse_body<R: Read>(
        buffer: &mut BufReader<R>,
        headers: &HashMap<String, String>,
    ) -> Result<String, RequestError> {
        let content_length = headers
            .get("content-length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        if content_length == 0 {
            return Ok(String::new());
        }

        let mut body = vec![0; content_length];
        buffer
            .read_exact(&mut body)
            .map_err(RequestError::ReadError)?;

        String::from_utf8(body).map_err(|_| {
            RequestError::InvalidRequest(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-8 in request body",
            ))
        })
    }
}
