//! Server runtime for RustAPI framework
//!
//! Provides the main `RustAPI` struct for configuring and running the HTTP server.

use crate::error::Result;
use crate::router::Router;
use std::net::SocketAddr;

/// Main RustAPI server struct with builder pattern for configuration
///
/// # Example
///
/// ```ignore
/// let app = Router::new().route("/", get(handler));
///
/// RustAPI::new(app)
///     .port(8080)
///     .serve()
///     .await?;
/// ```
pub struct RustAPI {
    router: Router,
    port: u16,
    host: String,
}

impl RustAPI {
    /// Create a new RustAPI server with the given router
    ///
    /// Defaults to running on `0.0.0.0:3000`
    pub fn new(router: Router) -> Self {
        Self {
            router,
            port: 3000,
            host: "0.0.0.0".to_string(),
        }
    }

    /// Set the port to listen on (default: 3000)
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the host to bind to (default: "0.0.0.0")
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Start the HTTP server
    ///
    /// This will bind to the configured host and port, and start serving requests.
    pub async fn serve(self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| crate::error::Error::server_error(format!("Invalid address {}: {}", addr, e)))?;

        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .map_err(|e| crate::error::Error::server_error(format!("Failed to bind to {}: {}", socket_addr, e)))?;

        tracing::info!("Server running on http://{}", socket_addr);

        // Router is already Axum's router (type alias), serve it directly
        axum::serve(listener, self.router)
            .await
            .map_err(|e| crate::error::Error::server_error(format!("Server error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_api_defaults() {
        let router = crate::router::new();
        let server = RustAPI::new(router);
        assert_eq!(server.port, 3000);
        assert_eq!(server.host, "0.0.0.0");
    }

    #[test]
    fn test_rust_api_builder() {
        let router = crate::router::new();
        let server = RustAPI::new(router)
            .port(8080)
            .host("127.0.0.1");
        assert_eq!(server.port, 8080);
        assert_eq!(server.host, "127.0.0.1");
    }
}
