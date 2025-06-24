use http_server::response::HttpResponse;

#[test]
fn test_new_response_creation() {
    let response = HttpResponse::new(200, "text/html", "Hello, world!".to_string());
    assert_eq!(response.status_code, 200);
    assert_eq!(response.content_type, "text/html");
    assert_eq!(response.body, "Hello, world!");
}

#[test]
fn test_new_response_to_string() {
    let response = HttpResponse::new(200, "text/html", "Hello, world!".to_string());
    let expected_response = "HTTP/1.1 200 OK\r\n\
                             Content-Type: text/html\r\n\
                             Content-Length: 13\r\n\
                             \r\n\
                             Hello, world!";
    assert_eq!(response.to_string(), expected_response);
}

#[test]
fn test_response_status_code_status_texts() {
    let status_code_text_pairs = [
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (204, "No Content"),
        (206, "Partial Content"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (303, "See Other"),
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (403, "Forbidden"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (406, "Not Acceptable"),
        (408, "Request Timeout"),
        (409, "Conflict"),
        (500, "Internal Server Error"),
    ];

    for (status_code, status_text) in status_code_text_pairs {
        let response = HttpResponse::new(status_code, "text/html", "Hello, world!".to_string());
        let expected_response = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, world!",
            status_code, status_text
        );
        assert_eq!(response.to_string(), expected_response);
    }
}

#[test]
fn test_new_response_headers() {
    let mut response = HttpResponse::new(200, "text/html", "Hello, world!".to_string());

    response.add_header("X-Custom-Header", "Custom Value");
    response.add_header("X-Another-Header", "Another Value");

    let expected_response = "HTTP/1.1 200 OK\r\n\
                             Content-Type: text/html\r\n\
                             Content-Length: 13\r\n\
                             X-Another-Header: Another Value\r\n\
                             X-Custom-Header: Custom Value\r\n\
                             \r\n\
                             Hello, world!";
    assert_eq!(response.to_string(), expected_response);
}

#[test]
fn test_new_response_cookies() {
    let mut response = HttpResponse::new(200, "text/html", "Hello, world!".to_string());

    response.add_cookie("session_id=1234567890; path=/; HttpOnly");
    response.add_cookie("theme=dark; path=/; HttpOnly");

    let expected_response = "HTTP/1.1 200 OK\r\n\
                             Content-Type: text/html\r\n\
                             Content-Length: 13\r\n\
                             Set-Cookie: session_id=1234567890; path=/; HttpOnly\r\n\
                             Set-Cookie: theme=dark; path=/; HttpOnly\r\n\
                             \r\n\
                             Hello, world!";
    assert_eq!(response.to_string(), expected_response);
}
