use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::utils::http::{Method, Version};

#[derive(Debug)]
pub enum RequestError {
    ReadError(std::io::Error),
    InvalidRequest(std::io::Error),
}

pub struct Request {
    method: Method,
    path: String,
    version: Version,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn from_stream(&self, stream: &mut TcpStream) -> Result<(), RequestError> {
        let buffer = BufReader::new(stream);
        let mut lines: Vec<_> = buffer
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if lines.is_empty() {
            return Err(RequestError::EmptyRequest.into());
        }
        Ok(())
    }
}
