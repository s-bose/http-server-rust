use schnell::response::HttpResponse;
use schnell::server::Server;

fn main() {
    let mut server = Server::new("127.0.0.1", 8080);
    server.get("/", |_| {
        HttpResponse::new(200, "text/plain", "Hello, world!".to_string())
    });
    server.get("/about", |_| {
        HttpResponse::new(200, "text/plain", "About page".to_string())
    });
    server.post("/add-todo", |req| {
        HttpResponse::new(200, "text/plain", format!("Todo added: {}", req.body))
    });
    server.listen().unwrap();
}
