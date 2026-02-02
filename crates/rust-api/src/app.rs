//! Application builder for rust-api framework
//!
//! Provides an ergonomic API for constructing and configuring REST applications.

use crate::di::Container;
use crate::error::Result;
use axum::Router;
use std::net::SocketAddr;

/// Application builder for rust-api framework
///
/// Provides a fluent API for:
/// - Registering services in the DI container
/// - Adding routes to the application
/// - Configuring middleware
/// - Starting the HTTP server
///
/// # Example
///
/// ```ignore
/// let app = App::new()
///     .service::<DatabaseService>()
///     .service::<UserService>()
///     .build();
/// ```
pub struct App {
    container: Container,
    router: Router,
}

impl App {
    /// Create a new application builder
    pub fn new() -> Self {
        Self {
            container: Container::new(),
            router: Router::new(),
        }
    }

    /// Get a reference to the DI container
    pub fn container(&self) -> &Container {
        &self.container
    }

    /// Get a mutable reference to the DI container
    pub fn container_mut(&mut self) -> &mut Container {
        &mut self.container
    }

    /// Get a reference to the router
    pub fn router(&self) -> &Router {
        &self.router
    }

    /// Build and return the configured router
    pub fn build(self) -> Router {
        self.router
    }

    /// Start the HTTP server on the given address
    ///
    /// # Example
    ///
    /// ```ignore
    /// app.serve("0.0.0.0:3000").await?;
    /// ```
    pub async fn serve(self, addr: impl Into<SocketAddr>) -> Result<()> {
        let addr = addr.into();
        let listener = self.create_listener_at(addr).await?;
        let router = self.router;
        Self::run_server_on(listener, router).await
    }

    //create a TCP listener on the given address
    async fn create_listener_at(&self, addr: SocketAddr) -> Result<tokio::net::TcpListener> {
        tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| crate::error::Error::server_error(format!("Failed to bind to {}: {}", addr, e)))
    }

    //run the axum server with the given listener and router
    async fn run_server_on(listener: tokio::net::TcpListener, router: Router) -> Result<()> {
        let addr = listener.local_addr().unwrap();
        tracing::info!("Server running on http://{}", addr);

        axum::serve(listener, router)
            .await
            .map_err(|e| crate::error::Error::server_error(format!("Server error: {}", e)))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(app.container().is_empty());
    }

    #[test]
    fn test_app_default() {
        let app = App::default();
        assert!(app.container().is_empty());
    }
}
