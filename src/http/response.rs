use crate::constants::HTTP_VERSION;
use crate::utils::{get_status_text, sanitize_header_key};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::io::{Result, Write};
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
            get_status_text(self.status_code)
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

    pub fn header(self, key: &str, value: &str) -> Self {
        let mut new_response = self;
        let key = sanitize_header_key(key);
        new_response.headers.insert(key, value.to_string());
        new_response
    }

    pub fn headers<const N: usize>(self, headers: [(&str, &str); N]) -> Self {
        let mut new_response = self;
        for (key, value) in headers {
            let key = sanitize_header_key(key);
            new_response.headers.insert(key, value.to_string());
        }
        new_response
    }

    pub fn body(self, body: &str) -> Self {
        let mut new_response = self;
        new_response.body = body.to_string();
        new_response
    }

    pub fn content_type(self, content_type: &str) -> Self {
        let mut new_response = self;
        new_response.content_type = content_type.to_string();
        new_response
    }

    pub fn status(self, status_code: u16) -> Self {
        let mut new_response = self;
        new_response.status_code = status_code;
        new_response
    }

    pub fn json<T: Serialize>(self, body: T) -> Self {
        let mut new_response = self;
        new_response.content_type = String::from("application/json");
        new_response.body = serde_json::to_string(&body).unwrap();
        new_response
    }

    pub fn text(self, body: &str) -> Self {
        let mut new_response = self;
        new_response.content_type = String::from("text/plain");
        new_response.body = body.to_string();
        new_response
    }

    pub fn html(self, body: &str) -> Self {
        let mut new_response = self;
        new_response.content_type = String::from("text/html");
        new_response.body = body.to_string();
        new_response
    }

    pub fn redirect(self, url: &str) -> Self {
        let mut new_response = self;
        new_response.status_code = 302;
        new_response
            .headers
            .insert(String::from("Location"), url.to_string());
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
