use std::{
    collections::HashMap,
    io::{BufRead, BufReader, ErrorKind, Read},
    str::FromStr,
};

use crate::http::{HttpMethod, Version};

pub enum RequestError {
    ReadError,
    InvalidRequest,
    RequestTooLarge,
    ConnectionClosed,
    ConnectionTimedOut,
    ParseError,
}

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl Request {
    pub fn read<R: Read>(mut buffer: BufReader<R>) -> Result<Self, RequestError> {
        let mut lines = Vec::new();
        let mut line = String::new();

        loop {
            match buffer.read_line(&mut line) {
                Ok(0) => {
                    // End of stream reached
                    if lines.is_empty() {
                        return Err(RequestError::ConnectionClosed);
                    }
                    break;
                }
                Ok(_) => {
                    if line.trim().is_empty() {
                        break; // End of headers
                    }
                    lines.push(line.trim().to_string());
                    line.clear();
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
                        return Err(RequestError::ConnectionClosed);
                    }
                    std::io::ErrorKind::TimedOut => {
                        return Err(RequestError::ConnectionTimedOut);
                    }
                    std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::BrokenPipe => {
                        return Err(RequestError::ConnectionClosed);
                    }
                    _ => {
                        return Err(RequestError::ReadError);
                    }
                },
            }
        }

        if lines.is_empty() {
            return Err(RequestError::ConnectionClosed);
        }

        if buffer.buffer().len() > 1024 * 1024 * 10 {
            return Err(RequestError::RequestTooLarge);
        }

        // Parse request line
        let (method, path, version) = Self::parse_request_line(&lines[0])?;

        // Parse headers
        let headers = Self::parse_headers(&lines[1..]);

        // Parse body (read remaining content)
        let body = Self::parse_body(&mut buffer, &headers)?;

        let (path, query) = path.split_once('?').unwrap_or((&path, ""));

        Ok(Request {
            method,
            path: path.to_string(),
            version,
            headers,
            body,
            params: HashMap::new(),
            query: Self::parse_query(query),
        })
    }

    fn parse_request_line(line: &str) -> Result<(HttpMethod, String, Version), RequestError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(RequestError::ParseError);
        }

        let method = HttpMethod::from_str(parts[0]).ok_or(RequestError::ParseError)?;

        let version = Version::from_str(parts[2]).map_err(|_| RequestError::InvalidRequest)?;

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
        match buffer.read_exact(&mut body) {
            Ok(()) => {}
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => {
                    return Err(RequestError::ConnectionClosed);
                }
                ErrorKind::TimedOut => {
                    return Err(RequestError::ConnectionTimedOut);
                }
                ErrorKind::ConnectionReset
                | ErrorKind::ConnectionAborted
                | ErrorKind::BrokenPipe => {
                    return Err(RequestError::ConnectionClosed);
                }
                _ => {
                    return Err(RequestError::ReadError);
                }
            },
        }

        String::from_utf8(body).map_err(|_| RequestError::ParseError)
    }

    fn parse_query(url: &str) -> HashMap<String, String> {
        let mut query_map = HashMap::new();
        for pair in url.split('&') {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            query_map.insert(key.to_string(), value.to_string());
        }

        query_map
    }
}
