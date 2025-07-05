use crate::constants::HTTP_VERSION;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::io::Result;
use std::io::Write;
use std::net::TcpStream;

pub struct HttpResponse {
    pub status_code: u16,
    pub content_type: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub cookies: Vec<String>,
}

impl HttpResponse {
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            content_type: String::from("text/plain"),
            headers: HashMap::new(),
            body: String::new(),
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

    pub fn with_header(self, key: &str, value: &str) -> Self {
        let mut new_response = self;
        new_response
            .headers
            .insert(key.to_string(), value.to_string());
        new_response
    }

    /// Set multiple headers at once
    pub fn with_headers(self, headers: Vec<(&str, &str)>) -> Self {
        let mut new_response = self;
        for (key, value) in headers {
            new_response
                .headers
                .insert(key.to_string(), value.to_string());
        }
        new_response
    }

    pub fn with_content_type(self, content_type: &str) -> Self {
        let mut new_response = self;
        new_response.content_type = content_type.to_string();
        new_response
    }

    /// Utility fns for different content types
    pub fn json<T: Serialize>(self, body: T) -> Self {
        let mut new_response = self;
        new_response.content_type = String::from("application/json");
        new_response.body = serde_json::to_string(&body).unwrap();
        new_response
    }

    /// plaintext content type
    pub fn text(self, body: &str) -> Self {
        let mut new_response = self;
        new_response.content_type = String::from("text/plain");
        new_response.body = body.to_string();
        new_response
    }

    /// html content type
    pub fn html(self, body: &str) -> Self {
        let mut new_response = self;
        new_response.content_type = String::from("text/html");
        new_response.body = body.to_string();
        new_response
    }

    pub fn with_cookie(self, cookie: &str) -> Self {
        let mut new_response = self;
        new_response.cookies.push(cookie.to_string());
        new_response
    }

    pub fn set_cookie(
        self,
        key: &str,
        value: &str,
        samesite: &str,
        http_only: bool,
        secure: bool,
        max_age: Option<u32>,
        expires: Option<DateTime<Utc>>,
    ) -> Self {
        let mut new_response = self;
        let mut cookie_str = format!("{}={}", key, value);
        match samesite {
            "Strict" | "Lax" | "None" => cookie_str.push_str(&format!("; SameSite={}", samesite)),
            _ => {}
        }

        if let Some(max_age) = max_age {
            cookie_str.push_str(&format!("; Max-Age={}", max_age));
        } else {
            cookie_str.push_str(&format!("; Max-Age={}", 60 * 60 * 24 * 30)); // 30 days
        }

        if let Some(expires) = expires {
            cookie_str.push_str(&format!(
                "; Expires={}",
                expires.format("%a, %d %b %Y %H:%M:%S GMT")
            ));
        } else {
            cookie_str.push_str(&format!(
                "; Expires={}",
                (Utc::now() + Duration::days(30)).format("%a, %d %b %Y %H:%M:%S GMT")
            ));
        }

        if http_only {
            cookie_str.push_str("; HttpOnly");
        }

        if secure {
            cookie_str.push_str("; Secure");
        }

        new_response.cookies.push(cookie_str);
        new_response
    }

    /// Add multiple cookies at once
    pub fn with_cookies(self, cookies: Vec<&str>) -> Self {
        let mut new_response = self;
        for cookie in cookies {
            new_response.cookies.push(cookie.to_string());
        }
        new_response
    }

    // any str body
    pub fn with_body(self, body: &str) -> Self {
        let mut new_response = self;
        new_response.body = body.to_string();
        new_response
    }

    /// Set the status code after creation
    pub fn with_status(self, status_code: u16) -> Self {
        let mut new_response = self;
        new_response.status_code = status_code;
        new_response
    }

    pub fn ok() -> Self {
        Self::new(200)
    }

    pub fn created() -> Self {
        Self::new(201)
    }

    pub fn not_found() -> Self {
        Self::new(404)
    }

    pub fn bad_request() -> Self {
        Self::new(400)
    }

    pub fn unauthorized() -> Self {
        Self::new(401)
    }

    pub fn forbidden() -> Self {
        Self::new(403)
    }

    pub fn method_not_allowed() -> Self {
        Self::new(405)
    }

    pub fn internal_server_error() -> Self {
        Self::new(500)
    }

    pub fn bad_gateway() -> Self {
        Self::new(502)
    }

    pub fn request_entity_too_large() -> Self {
        Self::new(413)
    }
}

pub fn write_response(stream: &mut TcpStream, response: HttpResponse) -> Result<()> {
    stream.write_all(response.to_string().as_bytes())?;

    stream.flush()?;
    Ok(())
}

#[macro_export]
macro_rules! send_response {
    ($stream:expr, $response:expr) => {
        if let Err(err) = write_response($stream, $response) {
            error!("Error writing response: {:?}", err);
        }
    };
}
