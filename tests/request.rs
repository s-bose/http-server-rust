use schnell::common::HttpMethod;
use schnell::request::Request;
use schnell::utils::http::Version;
use std::io::BufReader;

#[test]
fn test_from_stream_get_request() {
    let request_data =
        "GET /index.html HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: test-client/1.0\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_ok());

    let request = result.unwrap();
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.path, "/index.html");
    assert_eq!(request.version, Version::HTTP1_1);
    assert_eq!(
        request.headers.get("host"),
        Some(&"localhost:8080".to_string())
    );
    assert_eq!(
        request.headers.get("user-agent"),
        Some(&"test-client/1.0".to_string())
    );
    assert_eq!(request.body, "");
}

#[test]
fn test_from_stream_post_request_with_body() {
    let body = "{\"name\": \"John Doe\"}";
    let content_length = body.len();
    let request_data = format!(
        "POST /api/users HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        content_length, body
    );
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_ok());

    let request = result.unwrap();
    assert_eq!(request.method, HttpMethod::POST);
    assert_eq!(request.path, "/api/users");
    assert_eq!(request.version, Version::HTTP1_1);
    assert_eq!(
        request.headers.get("host"),
        Some(&"localhost:8080".to_string())
    );
    assert_eq!(
        request.headers.get("content-type"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        request.headers.get("content-length"),
        Some(&content_length.to_string())
    );
    assert_eq!(request.body, body);
}

#[test]
fn test_from_stream_multiple_headers() {
    let request_data = "GET /test HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Mozilla/5.0\r\nAccept: text/html\r\nAccept-Language: en-US\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_ok());

    let request = result.unwrap();
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.path, "/test");
    assert_eq!(request.headers.len(), 4);
    assert_eq!(
        request.headers.get("host"),
        Some(&"example.com".to_string())
    );
    assert_eq!(
        request.headers.get("user-agent"),
        Some(&"Mozilla/5.0".to_string())
    );
    assert_eq!(
        request.headers.get("accept"),
        Some(&"text/html".to_string())
    );
    assert_eq!(
        request.headers.get("accept-language"),
        Some(&"en-US".to_string())
    );
}

#[test]
fn test_from_stream_empty_request() {
    let request_data = "";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_err());
}

#[test]
fn test_from_stream_invalid_request_line() {
    let request_data = "INVALID REQUEST LINE\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_err());
}

#[test]
fn test_from_stream_invalid_method() {
    let request_data = "INVALID /test HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_err());
}

#[test]
fn test_from_stream_invalid_version() {
    let request_data = "GET /test HTTP/3.0\r\nHost: localhost\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_err());
}

#[test]
fn test_from_stream_headers_case_insensitive() {
    let request_data = "GET /test HTTP/1.1\r\nHOST: localhost\r\nContent-TYPE: text/plain\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_ok());

    let request = result.unwrap();
    assert_eq!(request.headers.get("host"), Some(&"localhost".to_string()));
    assert_eq!(
        request.headers.get("content-type"),
        Some(&"text/plain".to_string())
    );
}

#[test]
fn test_from_stream_body_with_zero_content_length() {
    let request_data = "POST /test HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n";
    let buffer = BufReader::new(request_data.as_bytes());

    let result = Request::from_stream(buffer);
    assert!(result.is_ok());

    let request = result.unwrap();
    assert_eq!(request.method, HttpMethod::POST);
    assert_eq!(request.body, "");
}

#[test]
fn test_from_stream_various_methods() {
    let methods = [
        (HttpMethod::GET, "GET"),
        (HttpMethod::POST, "POST"),
        (HttpMethod::PUT, "PUT"),
        (HttpMethod::DELETE, "DELETE"),
        (HttpMethod::HEAD, "HEAD"),
        (HttpMethod::OPTIONS, "OPTIONS"),
    ];

    for (expected_method, method_str) in methods {
        let request_data = format!("{} /test HTTP/1.1\r\nHost: localhost\r\n\r\n", method_str);
        let buffer = BufReader::new(request_data.as_bytes());

        let result = Request::from_stream(buffer);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.method, expected_method);
        assert_eq!(request.path, "/test");
        assert_eq!(request.version, Version::HTTP1_1);
    }
}
