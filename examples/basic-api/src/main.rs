use rustapi::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod services;

// Import controller handlers and their macro-generated path constants
use controllers::echo_controller::{echo, __echo_route};
use controllers::health_controller::{health_check, __health_check_route};
use services::echo_service::EchoService;
use services::health_service::HealthService;

/// Root endpoint handler that returns a welcome message.
#[get("/")]
async fn root() -> &'static str {
    "Welcome to RustAPI!"
}

/// Main entry point for the rustapi REST API server.
/// Demonstrates FastAPI-style routing with decorator macros and dependency injection.
#[tokio::main]
async fn main() {
    initialize_tracing();
    let container = setup_container();
    let app = build_router(&container);

    // Start the server using RustAPI framework
    RustAPI::new(app)
        .port(3000)  // Configurable port (default is 3000)
        .serve()
        .await
        .expect("Failed to start server");
}

/// Initializes the tracing subscriber for logging
fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustapi=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Sets up the DI container with all services
fn setup_container() -> Container {
    let mut container = Container::new();

    // Register services
    container.register_factory(|| HealthService::new());
    container.register_factory(|| EchoService::new());

    container
}

/// Builds the application router using FastAPI-style route decorators
/// Routes use macro-generated path constants for true decorator-based routing
fn build_router(container: &Container) -> Router {
    // Resolve services from container
    let health_service = container.resolve::<HealthService>().unwrap();
    let echo_service = container.resolve::<EchoService>().unwrap();

    // Build separate routers for each service with their own state
    // Note: Routes are added before calling with_state() - this is Axum's pattern
    // Path comes from the #[get("/health")] macro!
    let health_router = Router::new()
        .route(__health_check_route, routing::get(health_check))
        .with_state(health_service);

    // Path comes from the #[post("/echo")] macro!
    let echo_router = Router::new()
        .route(__echo_route, routing::post(echo))
        .with_state(echo_service);

    // Merge all routers together
    // Using router::build() as recommended entry point, but Router::new() also works
    router::build()
        .route(__root_route, routing::get(root))
        .merge(health_router)
        .merge(echo_router)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
