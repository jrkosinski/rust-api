//! RustAPI: FastAPI-inspired REST framework for Rust
//!
//! RustAPI brings the developer experience of FastAPI and NestJS to Rust,
//! with automatic OpenAPI generation, built-in validation, and dependency injection.
//!
//! # Features
//!
//! - **Route Macros**: Define endpoints with `#[get]`, `#[post]`, etc.
//! - **Dependency Injection**: Type-safe DI container for services
//! - **Type-Driven**: Leverage Rust's type system for validation and docs
//! - **Zero-Cost**: Built on Axum and Tokio for production performance
//!
//! # Quick Start
//!
//! ```ignore
//! use rustapi::prelude::*;
//!
//! #[get("/users/{id}")]
//! async fn get_user(Path(id): Path<String>) -> Json<User> {
//!     // handler code
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new()
//!         .route(__get_user_route, routing::get(get_user));
//!
//!     RustAPI::new(app)
//!         .port(3000)
//!         .serve()
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! # Examples
//!
//! See the `examples/` directory for complete working examples:
//!
//! - `basic-api`: Complete example with controllers, services, and DI

// Core modules
pub mod di;
pub mod app;
pub mod error;
pub mod server;
pub mod router;

// Re-export core types
pub use di::{Container, Injectable};
pub use app::App;
pub use error::{Error, Result};
pub use server::RustAPI;
pub use router::{Router, RouterExt};

// Re-export routing methods from Axum
// These are used to define route handlers (get, post, put, delete, etc.)
pub mod routing {
    pub use axum::routing::*;
}

// Re-export common middleware layers
pub use tower_http::cors::CorsLayer;
pub use tower_http::trace::TraceLayer;

// Re-export macros
pub use rustapi_macros::{
    get,
    post,
    put,
    delete,
    patch,
};

// Re-export commonly used axum types
pub use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Re-export serde for user convenience
pub use serde::{Serialize, Deserialize};

/// Prelude module for convenient imports
///
/// Import everything you need with:
/// ```ignore
/// use rustapi::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        // Core
        Container,
        Injectable,
        App,
        Error,
        Result,
        Router,
        RouterExt,
        RustAPI,
        router,
        routing,

        // Macros
        get,
        post,
        put,
        delete,
        patch,

        // Axum
        Json,
        Path,
        Query,
        State,
        StatusCode,
        IntoResponse,
        Response,

        // Middleware
        CorsLayer,
        TraceLayer,

        // Serde
        Serialize,
        Deserialize,
    };

    // Also re-export tokio for async runtime
    pub use tokio;
}
