use std::collections::HashMap;

pub struct Response {
    status_code: u16,
    status_text: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new(
        status_code: u16,
        status_text: &str,
        version: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            status_code,
            status_text: status_text.to_string(),
            version,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.version);
        response.push_str(&self.status_code.to_string());
        response.push_str(&self.status_text);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}

struct HttpResponse {
    status_code: u16,
    status_text: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str, version: String, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status_code,
            status_text: status_text.to_string(),
            version,
            headers,
            body,
        }
    }
}