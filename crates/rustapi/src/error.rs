//! Error types for rustapi framework

use thiserror::Error;

/// Result type alias for rustapi operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the rustapi framework
#[derive(Error, Debug)]
pub enum Error {
    /// Service not found in the DI container
    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    /// Service registration error
    #[error("Service registration failed: {0}")]
    RegistrationError(String),

    /// HTTP server error
    #[error("HTTP server error: {0}")]
    ServerError(String),

    /// Route registration error
    #[error("Route registration failed: {0}")]
    RouteError(String),

    /// Generic error
    #[error("Error: {0}")]
    Other(String),
}

impl Error {
    /// Create a ServiceNotFound error
    pub fn service_not_found(service_name: impl Into<String>) -> Self {
        Self::ServiceNotFound(service_name.into())
    }

    /// Create a RegistrationError
    pub fn registration_error(msg: impl Into<String>) -> Self {
        Self::RegistrationError(msg.into())
    }

    /// Create a ServerError
    pub fn server_error(msg: impl Into<String>) -> Self {
        Self::ServerError(msg.into())
    }

    /// Create a RouteError
    pub fn route_error(msg: impl Into<String>) -> Self {
        Self::RouteError(msg.into())
    }

    /// Create an Other error
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}
