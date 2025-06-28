# Schnell Web Server - Development Roadmap

A comprehensive roadmap to build a feature-rich, educational Rust HTTP server library.

## 📋 Table of Contents

- [Project Overview](#project-overview)
- [Current State](#current-state)
- [Development Phases](#development-phases)
- [Implementation Timeline](#implementation-timeline)
- [Dependencies](#dependencies)
- [Success Metrics](#success-metrics)
- [Getting Started](#getting-started)

## 🎯 Project Overview

Schnell is a toy HTTP server library written in Rust from scratch for learning purposes. The goal is to create a feature-rich, educational web server that showcases modern web development concepts while remaining approachable for learning Rust and HTTP servers.

**Key Principles:**
- Educational first, production-ready second
- Implement core web server concepts from scratch
- Showcase Rust's strengths in systems programming
- Provide comprehensive examples and documentation

## 📊 Current State

### ✅ Implemented Features
- Basic HTTP/1.1 server implementation
- HTTP response handling with proper status codes
- Custom headers and cookies support
- Multiple HTTP methods (GET, POST, PUT, DELETE, etc.)
- Thread-pooled request handling
- Basic routing system
- Request parsing with error handling
- JSON response utilities

### 🚧 In Progress
- Request parsing and routing
- Router structure (basic implementation exists)

### 📝 Planned
- Middleware support
- Static file serving
- Advanced routing features
- Template engine integration

## 🛣️ Development Phases

### **Phase 1: Core Foundation** (High Priority)

#### 1.1 Advanced Routing System
**Goal:** Build a robust routing system that can handle complex URL patterns

**Current State:** Basic method-based routing implemented
**Add:** 
- Path parameters (`/users/:id`, `/api/v1/users/:user_id/posts/:post_id`)
- Query parameter parsing (`/search?q=rust&limit=10`)
- Route groups/prefixes (`/api/v1/...`)
- Wildcard routes (`/static/*filepath`)
- Route priority and conflicts resolution

**Files to modify:**
- `src/router.rs` (expand current implementation)
- `src/request.rs` (add query parsing)

**Example API:**
```rust
server.get("/users/:id", |req| {
    let user_id = req.params.get("id").unwrap();
    // Handle user request
});

server.group("/api/v1", |group| {
    group.get("/users", users_handler);
    group.post("/users", create_user_handler);
});
```

#### 1.2 Request Body Parsing
**Goal:** Handle various request body formats properly

**Current State:** Raw body string parsing
**Add:**
- JSON body parsing with serde
- URL-encoded form data parsing (`application/x-www-form-urlencoded`)
- Multipart form data (file uploads)
- Raw bytes handling
- Content-Type validation and automatic parsing

**New files:**
- `src/body.rs` - Body parsing utilities
- `src/multipart.rs` - Multipart form handling

**Example API:**
```rust
server.post("/api/users", |req| {
    let user: User = req.json()?; // Automatic JSON parsing
    // Handle user creation
});

server.post("/upload", |req| {
    let files = req.multipart()?; // File upload handling
    // Process uploaded files
});
```

#### 1.3 Enhanced Request Object
**Goal:** Provide convenient access to request data

**Add:**
- Path parameters extraction
- Query parameters as HashMap
- Cookies parsing from headers
- Helper methods for common operations
- Request extensions for custom data

**Files to modify:**
- `src/request.rs`

**Example API:**
```rust
impl Request {
    pub fn param(&self, key: &str) -> Option<&String>;
    pub fn query(&self, key: &str) -> Option<&String>;
    pub fn cookie(&self, name: &str) -> Option<&str>;
    pub fn json<T: DeserializeOwned>(&self) -> Result<T>;
}
```

### **Phase 2: Essential Web Features** (High Priority)

#### 2.1 Middleware System
**Goal:** Create a flexible middleware system for cross-cutting concerns

**Add:**
- Middleware trait definition
- Middleware chain execution (before/after request)
- Built-in middleware: logging, CORS, compression, rate limiting
- Custom middleware support
- Error handling in middleware

**New files:**
- `src/middleware/mod.rs` - Core middleware traits
- `src/middleware/cors.rs` - CORS middleware
- `src/middleware/logger.rs` - Request logging
- `src/middleware/rate_limit.rs` - Rate limiting
- `src/middleware/compression.rs` - Response compression

**Example API:**
```rust
server.use_middleware(LoggerMiddleware::new());
server.use_middleware(CorsMiddleware::new()
    .allow_origin("*")
    .allow_methods(vec!["GET", "POST"]));

// Custom middleware
server.use_middleware(|req, next| {
    // Before request processing
    let response = next(req)?;
    // After request processing
    Ok(response)
});
```

#### 2.2 Static File Serving
**Goal:** Efficiently serve static files with proper headers

**Add:**
- Static file handler with configurable root directory
- MIME type detection based on file extension
- Directory listing (optional, configurable)
- Caching headers (ETag, Last-Modified, Cache-Control)
- Range requests support for large files
- Gzip compression for text files

**New files:**
- `src/static_files.rs` - Static file serving
- `src/mime.rs` - MIME type detection

**Example API:**
```rust
server.static_files("/static", "./public")
    .with_index("index.html")
    .with_directory_listing(false)
    .with_cache_max_age(3600);
```

#### 2.3 Error Handling & Status Pages
**Goal:** Provide comprehensive error handling and custom error pages

**Add:**
- Custom error pages (404, 500, etc.)
- Error middleware for centralized error handling
- Result type wrappers for handlers
- Panic recovery and graceful error responses
- Error logging and debugging information

**New files:**
- `src/error.rs` - Error types and handling

**Example API:**
```rust
server.error_handler(404, |_req| {
    HttpResponse::not_found().html(include_str!("templates/404.html"))
});

server.error_handler(500, |_req| {
    HttpResponse::internal_server_error().json(json!({
        "error": "Internal server error"
    }))
});
```

### **Phase 3: Developer Experience** (Medium Priority)

#### 3.1 Template Engine Integration
**Goal:** Add server-side rendering capabilities

**Add:**
- Integration with Handlebars or Tera template engine
- Template caching for performance
- Helper functions for common template operations
- Layout and partial template support

**New files:**
- `src/templates.rs` - Template engine integration

**Example API:**
```rust
server.get("/users/:id", |req| {
    let user = get_user(req.param("id")?)?;
    Ok(HttpResponse::ok().template("user.hbs", json!({
        "user": user,
        "title": "User Profile"
    })))
});
```

#### 3.2 Session Management
**Goal:** Provide session handling for stateful applications

**Add:**
- Cookie-based sessions
- Session storage backends (memory, file-based)
- Session middleware for automatic session handling
- CSRF protection utilities
- Secure session configuration

**New files:**
- `src/session.rs` - Session management

**Example API:**
```rust
server.use_middleware(SessionMiddleware::new()
    .with_secret("your-secret-key")
    .with_storage(MemorySessionStorage::new()));

server.post("/login", |req| {
    // Authenticate user
    req.session().set("user_id", user.id);
    Ok(HttpResponse::ok().redirect("/dashboard"))
});
```

#### 3.3 Configuration Management
**Goal:** Provide flexible configuration options

**Add:**
- Configuration file support (TOML/JSON)
- Environment variable support
- Default configuration with validation
- Runtime configuration updates
- Configuration documentation

**New files:**
- `src/config.rs` - Configuration management

**Example API:**
```rust
#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
}

let config = Config::from_file("config.toml")?
    .with_env_vars()
    .validate()?;
```

### **Phase 4: Advanced Features** (Medium Priority)

#### 4.1 Authentication Utilities
**Goal:** Provide common authentication patterns

**Add:**
- Basic Auth middleware
- JWT token creation and validation utilities
- Password hashing utilities (bcrypt)
- OAuth 2.0 helpers
- API key authentication

**New files:**
- `src/auth/mod.rs` - Authentication utilities
- `src/auth/jwt.rs` - JWT handling
- `src/auth/basic.rs` - Basic authentication
- `src/auth/oauth.rs` - OAuth helpers

**Example API:**
```rust
server.use_middleware(JwtMiddleware::new("secret-key"));

server.post("/auth/login", |req| {
    let credentials: LoginRequest = req.json()?;
    let user = authenticate_user(credentials)?;
    let token = create_jwt_token(&user)?;
    Ok(HttpResponse::ok().json(json!({ "token": token })))
});
```

#### 4.2 Database Integration Helpers
**Goal:** Simplify database integration patterns

**Add:**
- Connection pool utilities
- Database middleware for request-scoped connections
- Basic migration helpers
- Query builder utilities
- Transaction helpers

**New files:**
- `src/database.rs` - Database utilities

**Example API:**
```rust
server.use_middleware(DatabaseMiddleware::new(connection_pool));

server.get("/users", |req| {
    let db = req.db_connection()?;
    let users = User::find_all(&db)?;
    Ok(HttpResponse::ok().json(users))
});
```

#### 4.3 Compression & Performance
**Goal:** Optimize server performance and response sizes

**Add:**
- Gzip/Deflate compression middleware
- Response caching utilities
- Request/response size limits
- Performance monitoring middleware
- Memory usage tracking

**New files:**
- `src/compression.rs` - Response compression
- `src/cache.rs` - Caching utilities
- `src/metrics.rs` - Performance metrics

### **Phase 5: Nice-to-Have Features** (Lower Priority)

#### 5.1 WebSocket Support
**Goal:** Enable real-time communication

**Add:**
- WebSocket upgrade handling
- WebSocket connection management
- Message broadcasting utilities
- Simple chat room example

**New files:**
- `src/websocket.rs` - WebSocket implementation

#### 5.2 Advanced Routing Features
**Goal:** Support complex routing scenarios

**Add:**
- Regex-based routes
- Route constraints (type validation)
- Subdomain routing
- API versioning utilities
- Route documentation generation

#### 5.3 CLI & Development Tools
**Goal:** Improve developer experience

**Add:**
- CLI for generating projects and boilerplate
- Hot reloading for development
- Server statistics dashboard
- Load testing utilities
- Development proxy

**New files:**
- `src/cli.rs` - Command-line interface
- `src/dev_server.rs` - Development utilities

### **Phase 6: Polish & Examples** (Lower Priority)

#### 6.1 Comprehensive Examples
**Add to `examples/` directory:**
- **REST API** (`examples/rest_api/`) - Complete CRUD API with JSON
- **Static Server** (`examples/static_server/`) - File serving with caching
- **Blog Engine** (`examples/blog/`) - Template-based blog with markdown
- **Chat App** (`examples/chat/`) - Real-time chat with WebSockets
- **Auth System** (`examples/auth/`) - Complete authentication system
- **File Upload** (`examples/file_upload/`) - File upload with progress
- **Microservice** (`examples/microservice/`) - Complete microservice setup

#### 6.2 Documentation & Testing
**Add:**
- Integration test suite
- Performance benchmarks
- API documentation with examples
- Tutorial series (blog posts)
- Best practices guide
- Troubleshooting guide

## 📅 Implementation Timeline

### **Week 1-2: Foundation**
- [ ] Path parameters and query parsing
- [ ] Request body parsing (JSON, forms)
- [ ] Enhanced Request object with helper methods

### **Week 3-4: Core Web Features**
- [ ] Middleware system architecture
- [ ] CORS and logging middleware
- [ ] Static file serving with MIME types

### **Week 5-6: Developer Experience**
- [ ] Error handling improvements
- [ ] Configuration management
- [ ] Basic template engine integration

### **Week 7-8: Advanced Features**
- [ ] Session management
- [ ] Authentication utilities
- [ ] Database integration helpers

### **Week 9-10: Polish & Examples**
- [ ] Comprehensive examples
- [ ] Documentation improvements
- [ ] Performance optimizations

## 📦 Dependencies

### **Current Dependencies**
```toml
[dependencies]
scoped_threadpool = "0.1.9"
num_cpus = "1.17.0"
env = "1.0.1"
log = "0.4.27"
pretty_env_logger = "0.5.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### **Additional Dependencies to Add**

```toml
[dependencies]
# Routing and parsing
regex = "1.0"              # For advanced routing patterns
url = "2.0"                # For URL parsing
percent-encoding = "2.0"   # For URL encoding/decoding

# File handling and static content
mime_guess = "2.0"         # For MIME type detection
walkdir = "2.0"           # For directory traversal

# Template engines (choose one)
handlebars = "4.0"        # Handlebars template engine
# OR tera = "1.0"         # Tera template engine

# Session and authentication
uuid = "1.0"              # For session IDs
bcrypt = "0.15"           # For password hashing
jsonwebtoken = "8.0"      # For JWT tokens

# Configuration
toml = "0.8"              # For TOML config files
config = "0.13"           # Configuration management

# Compression
flate2 = "1.0"            # For gzip compression

# CLI (optional)
clap = "4.0"              # For command-line interface

# Development dependencies
[dev-dependencies]
criterion = "0.5"         # For benchmarking
tempfile = "3.0"          # For temporary files in tests
```

## 🎯 Success Metrics

By the end of this roadmap, your Schnell library should be able to:

### **Functional Requirements**
- ✅ Handle complex routing with path and query parameters
- ✅ Parse various request body formats (JSON, forms, multipart)
- ✅ Serve static files efficiently with proper headers
- ✅ Support flexible middleware chains
- ✅ Provide session management and authentication helpers
- ✅ Include comprehensive error handling
- ✅ Support template rendering for server-side rendered apps

### **Non-Functional Requirements**
- ✅ Handle concurrent requests efficiently
- ✅ Provide clear, educational code examples
- ✅ Include comprehensive documentation
- ✅ Have extensive test coverage (>80%)
- ✅ Demonstrate Rust best practices
- ✅ Be easily extensible for new features

### **Educational Goals**
- ✅ Showcase HTTP protocol implementation
- ✅ Demonstrate concurrent programming in Rust
- ✅ Illustrate web server architecture patterns
- ✅ Provide real-world Rust programming examples
- ✅ Show performance optimization techniques

## 🚀 Getting Started

### **Setup Development Environment**

1. **Clone and setup:**
```bash
cd schnell
cargo build
cargo test
```

2. **Run the current server:**
```bash
cargo run
# Visit http://localhost:8080
```

3. **Start with Phase 1.1 - Advanced Routing:**
   - Begin by enhancing the `src/router.rs` file
   - Add path parameter extraction
   - Implement query parameter parsing
   - Write tests for new functionality

### **Development Workflow**

1. **Pick a feature from the roadmap**
2. **Write tests first** (TDD approach)
3. **Implement the feature**
4. **Update documentation**
5. **Create an example** demonstrating the feature
6. **Update the README** with new capabilities

### **Code Organization Tips**

- Keep modules small and focused
- Use `pub(crate)` for internal APIs
- Write comprehensive doc comments
- Include code examples in documentation
- Follow Rust API design guidelines

## 📚 Resources

### **Learning Resources**
- [HTTP/1.1 Specification (RFC 7230)](https://tools.ietf.org/html/rfc7230)
- [Rust Book - Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Web Framework Design Patterns](https://docs.rs/warp/latest/warp/)

### **Inspiration from Other Frameworks**
- **Actix-web** - High-performance patterns
- **Warp** - Filter-based routing
- **Axum** - Handler patterns
- **Express.js** - Middleware architecture
- **Sinatra** - Simple DSL design

---

**Happy coding! 🦀✨**

*Remember: The goal is learning and showcasing Rust capabilities, not building a production-ready framework. Focus on educational value and code clarity.*