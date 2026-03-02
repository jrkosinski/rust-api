//! Request-lifecycle middleware utilities and Tower layer factories.
//!
//! This module provides utilities that operate at **request time**, not at
//! route registration time. These are Tower layer factories — they produce
//! middleware that wraps individual routes or entire routers.
//!
//! # Module Boundaries
//!
//! - `pipeline.rs` — build-time: compose routes into a `Router`
//! - `controller.rs` — build-time: declare which handlers belong to a
//!   controller
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

/// Returns a `Router -> Router` transform that guards every request with a
/// predicate.
///
/// Returns `403 Forbidden` if `guard_fn(&request)` returns `false`. The
/// predicate runs before any extractors, so it has access to headers, URI,
/// and method.
///
/// For **authentication**, prefer [`require_bearer`] — it handles the
/// `Authorization: Bearer` protocol correctly. `guard` is suited for
/// non-auth predicates (e.g., IP allowlists, feature flags, method
/// restrictions).
///
/// # Usage
///
/// Pass directly to `.map()` on the pipeline:
///
/// ```ignore
/// use rust_api::prelude::*;
///
/// RouterPipeline::new()
///     .mount::<MyController>(svc)
///     .map(guard(|req| is_allowed_ip(req)))
///     .build()?
/// ```
pub fn guard<G>(guard_fn: G) -> impl Fn(Router<()>) -> Router<()> + Clone + Send + 'static
where
    G: Fn(&Request<Body>) -> bool + Clone + Send + Sync + 'static,
{
    move |router: Router<()>| {
        let guard_fn = guard_fn.clone();
        router.layer(axum::middleware::from_fn(
            move |req: Request<Body>, next: Next| {
                let guard_fn = guard_fn.clone();
                async move {
                    if guard_fn(&req) {
                        next.run(req).await
                    } else {
                        axum::http::StatusCode::FORBIDDEN.into_response()
                    }
                }
            },
        ))
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, routing::get, Router};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    // -----------------------------------------------------------------------
    // constant_time_eq
    // -----------------------------------------------------------------------

    #[test]
    fn ct_eq_identical_slices() {
        assert!(constant_time_eq(b"secret", b"secret"));
    }

    #[test]
    fn ct_eq_different_slices() {
        assert!(!constant_time_eq(b"secret", b"wrong!"));
    }

    #[test]
    fn ct_eq_empty_slices() {
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn ct_eq_different_lengths_short_a() {
        assert!(!constant_time_eq(b"abc", b"abcd"));
    }

    #[test]
    fn ct_eq_different_lengths_short_b() {
        assert!(!constant_time_eq(b"abcd", b"abc"));
    }

    #[test]
    fn ct_eq_empty_vs_nonempty() {
        assert!(!constant_time_eq(b"", b"x"));
    }

    // -----------------------------------------------------------------------
    // require_bearer
    // -----------------------------------------------------------------------

    fn bearer_router() -> Router<()> {
        let inner = Router::new().route("/protected", get(|| async { "ok" }));
        require_bearer("correct-token")(inner)
    }

    #[tokio::test]
    async fn bearer_accepts_correct_token() {
        let app = bearer_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", "Bearer correct-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn bearer_rejects_wrong_token() {
        let app = bearer_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", "Bearer wrong-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 401);
    }

    #[tokio::test]
    async fn bearer_rejects_missing_header() {
        let app = bearer_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 401);
    }

    #[tokio::test]
    async fn bearer_rejects_malformed_header() {
        let app = bearer_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", "correct-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 401);
    }

    // -----------------------------------------------------------------------
    // guard
    // -----------------------------------------------------------------------

    fn guard_router(
        predicate: impl Fn(&Request<Body>) -> bool + Clone + Send + Sync + 'static,
    ) -> Router<()> {
        let inner = Router::new().route("/guarded", get(|| async { "ok" }));
        guard(predicate)(inner)
    }

    #[tokio::test]
    async fn guard_allows_request_when_predicate_is_true() {
        let app = guard_router(|_req| true);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/guarded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn guard_blocks_request_with_403_when_predicate_is_false() {
        let app = guard_router(|_req| false);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/guarded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 403);
    }

    #[tokio::test]
    async fn guard_predicate_receives_live_request_headers() {
        let app = guard_router(|req| req.headers().contains_key("x-allowed"));

        // without header → 403
        let blocked = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/guarded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(blocked.status(), 403);

        // with header → 200
        let allowed = app
            .oneshot(
                Request::builder()
                    .uri("/guarded")
                    .header("x-allowed", "yes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(allowed.status(), 200);
    }

    #[tokio::test]
    async fn guard_predicate_receives_live_request_uri() {
        // predicate inspects the URI path
        let app = guard_router(|req| req.uri().path().starts_with("/guarded"));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/guarded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
    }
}
