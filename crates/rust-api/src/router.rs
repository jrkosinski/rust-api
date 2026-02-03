//! Router utilities for RustAPI framework
//!
//! Provides a builder API for creating routers without directly exposing Axum
//! types. Users interact through the router module rather than importing Router
//! directly.

/// Re-export Axum's Router type
///
/// Note: In Axum's type system, `Router<S>` means a router that "needs" state
/// of type S.
/// - `Router<()>` = a stateless router (needs no state)
/// - `Router<AppState>` = a router that needs AppState to be provided via
///   `.with_state()`
///
/// Users should use `router::build()` to create routers rather than importing
/// this type.
pub type Router<S = ()> = axum::Router<S>;

/// Create a new router builder
///
/// This is the recommended entry point for creating routers. Returns an Axum
/// Router that can be configured using the fluent builder API.
///
/// # Example
///
/// ```ignore
/// use rust_api_core::{router, routing};
///
/// let app = router::build()
///     .route("/health", routing::get(health_check))
///     .layer(TraceLayer::new_for_http())
///     .finish();
/// ```
pub fn build() -> Router<()> {
    axum::Router::new()
}

/// Extension trait to add a `finish()` method to Router
///
/// This provides a clear endpoint to router building, making the API more
/// explicit.
pub trait RouterExt<S> {
    /// Finishes building the router and returns it
    ///
    /// This is a no-op that just returns self, but makes the builder API more
    /// explicit.
    fn finish(self) -> Router<S>;
}

impl<S> RouterExt<S> for Router<S> {
    fn finish(self) -> Router<S> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let _router = build();
    }

    #[test]
    fn test_router_finish() {
        let _router = build().finish();
    }
}
