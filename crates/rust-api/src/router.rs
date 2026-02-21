//! Router utilities for RustAPI framework
//!
//! Provides a builder API for creating routers without directly exposing Axum
//! types. Users interact through the router module rather than importing Router
//! directly.

use axum::routing::{on, MethodFilter};

/// Maps an HTTP method string (from a route annotation constant) to an Axum
/// [`MethodFilter`]. Used internally by the `mount_handlers!` macro.
///
/// Panics on an unrecognised method string — this indicates a bug in the
/// framework's macro layer, not a user error.
pub fn method_filter_from_str(method: &str) -> MethodFilter {
    match method {
        "GET"    => MethodFilter::GET,
        "POST"   => MethodFilter::POST,
        "PUT"    => MethodFilter::PUT,
        "DELETE" => MethodFilter::DELETE,
        "PATCH"  => MethodFilter::PATCH,
        other    => panic!(
            "Unknown HTTP method '{}' in route annotation. \
             Use #[get], #[post], #[put], #[delete], or #[patch].",
            other
        ),
    }
}

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
///     .api_route(__health_check_route, health_check)
///     .layer(TraceLayer::new_for_http())
///     .finish();
/// ```
pub fn build() -> Router<()> {
    axum::Router::new()
}

/// Extension trait for registering routes using the macro-generated
/// `(&'static str, &'static str)` route info tuple.
///
/// This is the **enforcement contract**: the HTTP verb in the route info tuple
/// (set by the `#[get]`, `#[post]`, etc. annotation) is the sole authority on
/// the HTTP method. It is impossible to accidentally register a `#[get]`
/// handler as a `POST` endpoint.
///
/// # Example
///
/// ```ignore
/// // health_check is annotated #[get("/health")], so __health_check_route is
/// // ("/health", "GET"). api_route enforces that it is registered as GET.
/// router.api_route(__health_check_route, health_check)
/// ```
pub trait ApiRoute<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Register a handler using the `(path, method)` tuple produced by a route
    /// macro annotation. The HTTP verb is taken from the tuple — it cannot be
    /// overridden at the call site.
    fn api_route<H, T>(self, route_info: (&'static str, &'static str), handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static;
}

impl<S> ApiRoute<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn api_route<H, T>(self, route_info: (&'static str, &'static str), handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        let (path, method) = route_info;

        // Map the method string (from the annotation) to a MethodFilter.
        // MethodFilter is Copy, so handler is moved exactly once into on().
        let filter = match method {
            "GET" => MethodFilter::GET,
            "POST" => MethodFilter::POST,
            "PUT" => MethodFilter::PUT,
            "DELETE" => MethodFilter::DELETE,
            "PATCH" => MethodFilter::PATCH,
            other => panic!(
                "Unknown HTTP method '{}' from route annotation. \
                 Use #[get], #[post], #[put], #[delete], or #[patch].",
                other
            ),
        };

        self.route(path, on(filter, handler))
    }
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
