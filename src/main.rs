use axum::{
    routing::{get, post},
    Router,
};

use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod services;

use controllers::health_controller::HealthController;
use controllers::echo_controller::EchoController;

use crate::controllers::echo_controller;

/// Main entry point for the rusty-resty REST API server.
/// Initializes logging, sets up routes, and starts the HTTP server.
#[tokio::main]
async fn main() {
    initialize_tracing();
    let app = build_router();
    let listener = create_listener().await;
    run_server(listener, app).await;
}

/// Initializes the tracing subscriber for logging
fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_resty=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Builds the application router with all routes and middleware
fn build_router() -> Router {
    // Initialize controllers with their dependencies
    let health_controller = Arc::new(HealthController::new());
    let echo_controller = Arc::new(EchoController::new());

    // Create nested routers with individual states
    let health_router = Router::new()
        .route("/health", get(HealthController::health_check))
        .with_state(health_controller);

    let echo_router = Router::new()
        .route("/echo", post(EchoController::echo))
        .with_state(echo_controller);

    // Merge all routers together
    Router::new()
        .route("/", get(root))
        .merge(health_router)
        .merge(echo_router)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

/// Creates and binds the TCP listener on port 3000
async fn create_listener() -> tokio::net::TcpListener {
    tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap()
}

/// Runs the server with the given listener and application router
async fn run_server(listener: tokio::net::TcpListener, app: Router) {
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app)
        .await
        .unwrap();
}

/// Root endpoint handler that returns a welcome message.
async fn root() -> &'static str {
    "Welcome to rusty-resty!"
}
