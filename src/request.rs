use std::collections::HashMap;

use crate::utils::http::{Method, Version};

struct Request {
    method: Method,
    path: String,
    version: Version,
    headers: HashMap<String, String>,
    body: String,
}
