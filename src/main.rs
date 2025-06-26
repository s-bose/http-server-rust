use schnell::response::HttpResponse;
use schnell::server::Server;
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn main() {
    pretty_env_logger::init();

    let mut server = Server::new("127.0.0.1", 8080, None);
    server.get("/", |_| {
        Ok(HttpResponse::ok().html("<h1>Hello, world!</h1>"))
    });
    server.get("/about", |_| {
        Ok(HttpResponse::ok().html("<h1>About page</h1>"))
    });
    server.post("/add-todo", |req| {
        Ok(HttpResponse::ok().text(&format!("Todo added: {}", req.body)))
    });
    server.get("/todos", |_| {
        Ok(HttpResponse::ok().json(vec![
            Todo {
                id: 1,
                title: "Buy groceries".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Buy groceries".to_string(),
                completed: false,
            },
        ]))
    });
    server.listen();
}
