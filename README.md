# Schnell

Schnell is a toy HTTP server library written in Rust from scratch. It provides a simple, lightweight HTTP server implementation with basic routing capabilities.

> **Note**: This is a work in progress and is not suited for production use.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)

## Features

- ✅ Basic HTTP/1.1 server implementation
- ✅ HTTP response handling with proper status codes
- ✅ Custom headers and cookies support
- ✅ Multiple HTTP methods (GET, POST, PUT, DELETE, etc.)
- 🚧 Request parsing and routing (in development)
- 🚧 Middleware support (planned)
- 🚧 Static file serving (planned)

## Installation

Add Schnell to your `Cargo.toml`:

```toml
[dependencies]
schnell = "0.1.0"
```

Or use it as a library in your project:

```bash
cargo add schnell
```

## Quick Start

### Basic Server

```rust
use schnell::response::HttpResponse;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server running on http://127.0.0.1:8080");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = HttpResponse::new(200, "text/html", 
        "<h1>Hello from Schnell!</h1>".to_string());
    
    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### Building and Running

```bash
# Build the project
cargo build

# Run the server
cargo run

# Run with release optimizations
cargo run --release
```

## API Reference

### HttpResponse

The main response structure for handling HTTP responses.

#### Constructor

```rust
HttpResponse::new(status_code: u16, content_type: &str, body: String) -> Self
```

#### Methods

- `to_string(&self) -> String` - Converts the response to a proper HTTP response string
- `add_header(&mut self, key: &str, value: &str)` - Adds a custom header
- `add_cookie(&mut self, cookie: &str)` - Adds a Set-Cookie header

#### Properties

- `status_code: u16` - HTTP status code
- `content_type: String` - Response content type
- `body: String` - Response body content
- `headers: HashMap<String, String>` - Custom headers
- `cookies: Vec<String>` - Cookie values

### HttpMethod

Enumeration of supported HTTP methods:

```rust
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
}
```

### Server (Work in Progress)

Server structure for handling HTTP requests with routing:

```rust
pub struct Server {
    // Implementation in progress
}
```

## Examples

### JSON Response

```rust
use schnell::response::HttpResponse;

let mut response = HttpResponse::new(
    200, 
    "application/json", 
    r#"{"message": "Hello, World!"}"#.to_string()
);

response.add_header("Access-Control-Allow-Origin", "*");
println!("{}", response.to_string());
```

### HTML Response with Cookies

```rust
use schnell::response::HttpResponse;

let mut response = HttpResponse::new(
    200,
    "text/html",
    "<html><body><h1>Welcome!</h1></body></html>".to_string()
);

response.add_cookie("session_id=abc123; HttpOnly; Secure");
response.add_header("X-Powered-By", "Schnell");
```

### Error Response

```rust
use schnell::response::HttpResponse;

let response = HttpResponse::new(
    404,
    "text/plain",
    "Page not found".to_string()
);
```

### Status Codes

Schnell supports all standard HTTP status codes:

- **1xx**: Informational responses (100-103)
- **2xx**: Successful responses (200-226)
- **3xx**: Redirection messages (300-308)
- **4xx**: Client error responses (400-451)
- **5xx**: Server error responses (500-511)

## Development

### Project Structure

```
src/
├── lib.rs          # Library entry point
├── main.rs         # Binary entry point
├── common.rs       # Common types and enums
├── constants.rs    # HTTP constants
├── request.rs      # HTTP request handling (WIP)
├── response.rs     # HTTP response handling
├── router.rs       # Request routing (WIP)
├── server.rs       # Server implementation (WIP)
└── utils/
    ├── mod.rs      # Utils module
    └── http.rs     # HTTP parsing utilities
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/schnell.git
cd schnell

# Build the project
cargo build

# Run the example server
cargo run
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_new_response_creation
```

### Test Coverage

Current test coverage includes:
- ✅ HTTP response creation and formatting
- ✅ Status code handling
- ✅ Header management
- ✅ Cookie handling
- 🚧 Request parsing (in development)
- 🚧 Routing (in development)

## Contributing

Contributions are welcome! This is a learning project, so feel free to:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation for API changes
- Keep commits focused and descriptive

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Complete request parsing implementation
- [ ] Implement routing system
- [ ] Add middleware support
- [ ] Static file serving
- [ ] WebSocket support
- [ ] Performance optimizations
- [ ] Documentation improvements

---

*Schnell* means "fast" in German 🚀


