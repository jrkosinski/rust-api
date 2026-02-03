//! RustAPI: FastAPI-inspired REST framework for Rust
//!
//! RustAPI brings the developer experience of FastAPI and NestJS to Rust,
//! with automatic OpenAPI generation, built-in validation, and dependency
//! injection.
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
//! use rust_api::prelude::*;
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
pub mod app;
pub mod di;
pub mod error;
pub mod router;
pub mod server;

// Re-export core types
pub use app::App;
pub use di::{Container, Injectable};
pub use error::{Error, Result};
pub use router::{Router, RouterExt};
pub use server::RustAPI;

// Re-export routing methods from Axum
// These are used to define route handlers (get, post, put, delete, etc.)
pub mod routing {
    pub use axum::routing::*;
}

// Re-export common middleware layers
// Re-export commonly used axum types
pub use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
// Re-export macros
pub use rust_api_macros::{delete, get, patch, post, put};
// Re-export serde for user convenience
pub use serde::{Deserialize, Serialize};
pub use tower_http::{cors::CorsLayer, trace::TraceLayer};

/// Prelude module for convenient imports
///
/// Import everything you need with:
/// ```ignore
/// use rust_api::prelude::*;
/// ```
pub mod prelude {
    // Also re-export tokio for async runtime
    pub use tokio;

    pub use super::{
        delete,
        // Macros
        get,
        patch,

        post,
        put,
        router,
        routing,

        App,
        // Core
        Container,
        // Middleware
        CorsLayer,
        Deserialize,
        Error,
        Injectable,
        IntoResponse,
        // Axum
        Json,
        Path,
        Query,
        Response,

        Result,
        Router,
        RouterExt,
        RustAPI,
        // Serde
        Serialize,
        State,
        StatusCode,
        TraceLayer,
    };
}
