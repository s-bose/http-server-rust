use crate::constants::HTTP_VERSION;
use std::collections::HashMap;

pub struct HttpResponse {
    pub status_code: u16,
    pub content_type: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub cookies: Vec<String>,
}

impl HttpResponse {
    pub fn new(status_code: u16, content_type: &str, body: String) -> Self {
        Self {
            status_code,
            content_type: content_type.to_string(),
            headers: HashMap::new(),
            body,
            cookies: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();

        response.push_str(&format!(
            "{} {} {}\r\n",
            HTTP_VERSION,
            self.status_code,
            self.get_status_text(self.status_code)
        ));

        // Content-Type
        response.push_str(&format!("Content-Type: {}\r\n", self.content_type));

        // Content-Length
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        // Custom Headers
        let mut header_keys: Vec<_> = self.headers.keys().collect();
        header_keys.sort();
        for key in header_keys {
            let value = &self.headers[key];
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Cookies
        for cookie in &self.cookies {
            response.push_str(&format!("Set-Cookie: {}\r\n", cookie));
        }

        // Empty line
        response.push_str("\r\n");

        // Body
        response.push_str(&self.body);

        response
    }

    fn get_status_text(&self, code: u16) -> &'static str {
        match code {
            // 1xx
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            103 => "Early Hints",
            // 2xx
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            226 => "IM Used",
            // 3xx
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            // 4xx
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            418 => "I'm a teapot",
            421 => "Misdirected Request",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            425 => "Too Early",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",
            // 5xx
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => "Unknown",
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn add_cookie(&mut self, cookie: &str) {
        self.cookies.push(cookie.to_string());
    }
}
