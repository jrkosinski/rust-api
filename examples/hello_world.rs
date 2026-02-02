//! Minimal "Hello World" example using rust-api
//!
//! This demonstrates the simplest possible usage of the framework.

use rust_api::prelude::*;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, rust_api!"
}

#[get("/greet/{name}")]
async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize)]
struct Message {
    text: String,
}

#[post("/message")]
async fn post_message(Json(msg): Json<Message>) -> Json<Message> {
    Json(Message {
        text: format!("Received: {}", msg.text),
    })
}

#[tokio::main]
async fn main() {
    //build router using generated route helpers
    let app = Router::new()
        .route(__hello_route.0, __hello_route.1())
        .route(__greet_route.0, __greet_route.1())
        .route(__post_message_route.0, __post_message_route.1());

    //start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
