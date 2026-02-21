//! Request-lifecycle middleware utilities and Tower layer factories.
//!
//! This module provides utilities that operate at **request time**, not at
//! route registration time. These are Tower layer factories — they produce
//! middleware that wraps individual routes or entire routers.
//!
//! # Module Boundaries
//!
//! - `pipeline.rs` — build-time: compose routes into a `Router`
//! - `controller.rs` — build-time: declare which handlers belong to a controller
//! - `middleware.rs` — request-time: inspect/modify requests and responses
//!
//! # Protected Route Groups
//!
//! Auth is a **cross-cutting concern** — it belongs here as a router transform,
//! not inside a controller handler. Apply it to a route group via `.map()`:
//!
//! ```ignore
//! RouterPipeline::new()
//!     .mount_guarded::<AdminController, _>(admin_svc, || { /* config check */ })
//!     .map(require_bearer(admin_key))
//! ```
//!
//! Or scoped to just a sub-group:
//!
//! ```ignore
//! RouterPipeline::new()
//!     .group("/admin", |g| g
//!         .mount::<AdminController>(admin_svc)
//!         .map(require_bearer(admin_key))   // only admin routes are protected
//!     )
//! ```

use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse, Router};

// ---------------------------------------------------------------------------
// require_bearer
// ---------------------------------------------------------------------------

/// Returns a `Router -> Router` transform that enforces `Authorization: Bearer
/// <token>` on every request passing through the router it is applied to.
///
/// Returns `401 Unauthorized` if the header is absent, malformed, or if the
/// token does not match `expected` (compared in **constant time** to prevent
/// timing oracles).
///
/// # Usage
///
/// Pass directly to `.map()` — the function signature matches `.map()`'s
/// expected `Fn(Router<()>) -> Router<()>`:
///
/// ```ignore
/// use rust_api::prelude::*;
///
/// RouterPipeline::new()
///     .mount_guarded::<AdminController, _>(admin_svc, || { /* config check */ })
///     .map(require_bearer(admin_key))
///     .build()?
/// ```
pub fn require_bearer(
    expected: impl Into<String>,
) -> impl Fn(Router<()>) -> Router<()> + Clone + Send + 'static {
    let expected = expected.into();
    move |router: Router<()>| {
        let expected = expected.clone();
        router.layer(axum::middleware::from_fn(
            move |req: Request<Body>, next: Next| {
                let expected = expected.clone();
                async move {
                    let authorized = req
                        .headers()
                        .get(axum::http::header::AUTHORIZATION)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.strip_prefix("Bearer "))
                        .map(|token| constant_time_eq(token.as_bytes(), expected.as_bytes()))
                        .unwrap_or(false);

                    if authorized {
                        next.run(req).await
                    } else {
                        axum::http::StatusCode::UNAUTHORIZED.into_response()
                    }
                }
            },
        ))
    }
}

// ---------------------------------------------------------------------------
// guard
// ---------------------------------------------------------------------------

/// Creates a middleware function that guards a route with a request predicate.
///
/// Returns `403 Forbidden` if `guard_fn(&request)` returns `false`. The
/// predicate always receives the request before any extractors run, so it has
/// access to headers, URI, and method.
///
/// For **authentication**, prefer [`require_bearer`] applied via `.map()` on a
/// route group. `guard` is suited for non-auth predicates (e.g., IP allowlists,
/// feature flags evaluated per-request, method restrictions).
///
/// ```ignore
/// use rust_api::prelude::*;
///
/// // Protect a whole router with an IP allowlist:
/// router.layer(axum::middleware::from_fn(guard(|req| is_allowed_ip(req))))
/// ```
pub fn guard<G>(
    guard_fn: G,
) -> impl Fn(
    axum::http::Request<axum::body::Body>,
    axum::middleware::Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = axum::response::Response> + Send>>
       + Clone
       + Send
       + 'static
where
    G: Fn(&axum::http::Request<axum::body::Body>) -> bool + Clone + Send + Sync + 'static,
{
    move |req, next| {
        let guard_fn = guard_fn.clone();
        Box::pin(async move {
            if guard_fn(&req) {
                next.run(req).await
            } else {
                axum::http::StatusCode::FORBIDDEN.into_response()
            }
        })
            as std::pin::Pin<Box<dyn std::future::Future<Output = axum::response::Response> + Send>>
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Constant-time byte-slice equality — prevents timing oracle attacks.
///
/// XORs every byte of both slices (zero-padded to the longer length) and
/// accumulates the differences. No early exit: a short token cannot
/// short-circuit the comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len = a.len().max(b.len());
    let mut diff: u8 = 0;
    for i in 0..len {
        let ab = a.get(i).copied().unwrap_or(0);
        let bb = b.get(i).copied().unwrap_or(0);
        diff |= ab ^ bb;
    }
    diff == 0
}
