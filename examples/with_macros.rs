//! Example demonstrating the route macro system
//!
//! This shows how to use #[get], #[post], etc. macros for cleaner route definitions.

use axum::{
    extract::Path,
    http::StatusCode,
    Json,
    Router,
};
use rust_api_macros::{get, post};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EchoRequest {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EchoResponse {
    echo: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

//route handlers using macros

#[get("/")]
async fn root() -> &'static str {
    "Welcome to rust_api with macros!"
}

#[get("/health")]
async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
        }),
    )
}

#[post("/echo")]
async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    Json(EchoResponse {
        echo: payload.message,
    })
}

#[get("/users/{id}")]
async fn get_user(Path(id): Path<String>) -> Json<User> {
    Json(User {
        id: id.clone(),
        name: format!("User {}", id),
    })
}

#[tokio::main]
async fn main() {
    //initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_macros=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    //build router using the generated route helpers
    let app = Router::new()
        .route(__root_route.0, __root_route.1())
        .route(__health_check_route.0, __health_check_route.1())
        .route(__echo_route.0, __echo_route.1())
        .route(__get_user_route.0, __get_user_route.1());

    //start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
