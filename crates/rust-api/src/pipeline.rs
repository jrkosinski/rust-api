//! Monadic router pipeline for composable, error-propagating route registration.
//!
//! [`RouterPipeline`] wraps `Result<Router<()>>` and provides a fluent builder
//! API where every step is `Result::and_then` (`>>=`). A failed step
//! short-circuits all subsequent steps. The error surfaces at `.build()`.
//!
//! # The Kleisli Model
//!
//! Each `mount::<C>(state)` call creates a Kleisli arrow
//! `Router<()> -> Result<Router<()>>` from the controller's `mount` fn and
//! threads it through the pipeline via `and_then`. The pipeline IS the
//! Kleisli compositor — controllers are pure arrows, they don't compose
//! themselves.
//!
//! # Algebraic Operations
//!
//! | Method | Concept | Description |
//! |---|---|---|
//! | `map(f)` | Functor (`fmap`) | Infallible `Router -> Router` transform |
//! | `and_then(f)` | Monad bind (`>>=`) | Fallible `Router -> Result<Router>` |
//! | `mount::<C>(state)` | Kleisli bind | Thread router through a `Controller` arrow |
//! | `mount_if::<C>(bool, state)` | Conditional bind | Mount only when condition is `true` |
//! | `mount_guarded::<C>(state, g)` | Guarded bind | Mount only when guard `g()` succeeds |
//! | `fold(steps)` | Catamorphism | Apply a dynamic list of fallible steps |
//! | `layer_all(transforms)` | `fold` over transforms | Apply a list of `Router -> Router` fns |
//! | `group(prefix, f)` | Scoped functor | Sub-pipeline with path prefix applied |
//! | `route(info, handler)` | Route registration | Stateless route via route info tuple |
//! | `build()` | Interpreter / run | Consume pipeline, surface `Result<Router<()>>` |
//!
//! # Example
//!
//! ```ignore
//! let health_svc = Arc::new(HealthService::new());
//! let echo_svc   = Arc::new(EchoService::new());
//!
//! let app = RouterPipeline::new()
//!     .mount::<HealthController>(health_svc)
//!     .mount_if::<EchoController>(config.enable_echo, echo_svc)
//!     .route(__root_route, root_handler)
//!     .map(|r| r.layer(TraceLayer::new_for_http()))
//!     .map(|r| r.layer(CorsLayer::permissive()))
//!     .build()?;
//! ```

use std::sync::Arc;

use crate::{
    controller::Controller,
    error::Result,
    router::{ApiRoute, Router},
};

/// A boxed, infallible router transformation.
///
/// Used with [`RouterPipeline::layer_all`] to apply a dynamic collection of
/// transforms (e.g., middleware layers) to the pipeline.
pub type RouterTransform = Box<dyn FnOnce(Router<()>) -> Router<()>>;

/// Monadic router builder that propagates errors through the pipeline via
/// Kleisli composition.
///
/// Wraps `Result<Router<()>>`. Each step is `Result::and_then` — any error
/// short-circuits the rest of the chain. Call [`build`](RouterPipeline::build)
/// at the end to surface the final `Result<Router<()>>`.
///
/// See [module-level docs](self) for the full operation table.
pub struct RouterPipeline(Result<Router<()>>);

impl RouterPipeline {
    /// Start a new pipeline with an empty `Router<()>`.
    pub fn new() -> Self {
        Self(Ok(crate::router::build()))
    }

    // -----------------------------------------------------------------------
    // Core operations
    // -----------------------------------------------------------------------

    /// Kleisli bind: thread the router through a [`Controller`]'s Kleisli arrow.
    ///
    /// Calls `C::mount(state)` to obtain the arrow, then threads it via
    /// `and_then`. The controller's routes are merged into the pipeline's
    /// router. Short-circuits if any previous step failed.
    ///
    /// The controller has **no knowledge of routing infrastructure** — it only
    /// provides the Kleisli arrow. The pipeline is the sole compositor.
    pub fn mount<C: Controller>(self, state: Arc<C::State>) -> Self {
        Self(self.0.and_then(C::mount(state)))
    }

    /// Functor map (`fmap`): apply an infallible `Router -> Router` transform.
    ///
    /// The most common use is adding a middleware layer:
    /// ```ignore
    /// pipeline.map(|r| r.layer(TraceLayer::new_for_http()))
    /// ```
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Router<()>) -> Router<()>,
    {
        Self(self.0.map(f))
    }

    /// Monad bind (`>>=`): apply a fallible `Router -> Result<Router>` transform.
    ///
    /// Short-circuits on any previous error. Use for transforms that can fail.
    pub fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Router<()>) -> Result<Router<()>>,
    {
        Self(self.0.and_then(f))
    }

    /// Register a stateless route (no service state) using a route info tuple.
    ///
    /// The `route_info` tuple is produced by a route macro annotation:
    /// `__root_route` is `("/", "GET")` when annotated `#[get("/")]`.
    /// The HTTP verb is enforced by [`ApiRoute::api_route`].
    pub fn route<H, T>(self, route_info: (&'static str, &'static str), handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.map(|r| r.api_route(route_info, handler))
    }

    /// Terminate the pipeline and return the built `Router<()>`.
    ///
    /// Use `?` at the call site to propagate any error that occurred during
    /// pipeline construction:
    /// ```ignore
    /// let app = RouterPipeline::new()
    ///     .mount::<HealthController>(Arc::new(HealthService::new()))
    ///     .build()?;
    /// ```
    pub fn build(self) -> Result<Router<()>> {
        self.0
    }

    // -----------------------------------------------------------------------
    // Conditional and guarded mounting
    // -----------------------------------------------------------------------

    /// Conditional mount: mount a [`Controller`] only when `condition` is `true`.
    ///
    /// When `false`, the pipeline passes through unchanged — no error produced.
    /// The `state` value is moved into the mount call when `condition` is `true`,
    /// or dropped when `false`.
    ///
    /// ```ignore
    /// RouterPipeline::new()
    ///     .mount::<HealthController>(health_svc)
    ///     .mount_if::<MetricsController>(config.enable_metrics, metrics_svc)
    ///     .mount_if::<AdminController>(env.is_dev(), admin_svc)
    ///     .build()?
    /// ```
    pub fn mount_if<C: Controller>(self, condition: bool, state: Arc<C::State>) -> Self {
        if condition {
            self.mount::<C>(state)
        } else {
            // identity — drop state, pass the pipeline through unchanged
            self
        }
    }

    /// Guarded mount: mount a [`Controller`] only when `guard()` returns `Ok(())`.
    ///
    /// The guard is a fallible predicate evaluated before the controller's
    /// Kleisli arrow runs. A guard error short-circuits the pipeline the same
    /// way as a failed `mount`.
    ///
    /// Use this for runtime checks (required config, capability flags, etc.):
    ///
    /// ```ignore
    /// RouterPipeline::new()
    ///     .mount_guarded::<AdminController>(admin_svc, || {
    ///         if config.admin_secret.is_empty() {
    ///             Err(Error::other("admin_secret must be set"))
    ///         } else {
    ///             Ok(())
    ///         }
    ///     })
    ///     .build()?
    /// ```
    pub fn mount_guarded<C: Controller, G>(self, state: Arc<C::State>, guard: G) -> Self
    where
        G: FnOnce() -> Result<()>,
    {
        Self(self.0.and_then(|router| {
            guard()?;
            C::mount(state)(router)
        }))
    }

    // -----------------------------------------------------------------------
    // Collection operations
    // -----------------------------------------------------------------------

    /// Catamorphism (fold): apply a dynamic, ordered collection of fallible
    /// `Router -> Result<Router>` steps, left-to-right.
    ///
    /// Short-circuits on the first error. Replaces imperative `for` loops
    /// when the set of pipeline steps is known only at runtime.
    ///
    /// ```ignore
    /// let steps: Vec<Box<dyn FnOnce(Router<()>) -> Result<Router<()>>>> = vec![
    ///     Box::new(HealthController::mount(health_svc)),
    ///     Box::new(EchoController::mount(echo_svc)),
    /// ];
    ///
    /// RouterPipeline::new().fold(steps).build()?
    /// ```
    pub fn fold<I, F>(self, steps: I) -> Self
    where
        I: IntoIterator<Item = F>,
        F: FnOnce(Router<()>) -> Result<Router<()>>,
    {
        steps.into_iter().fold(self, |p, step| p.and_then(step))
    }

    /// Apply a dynamic collection of infallible `Router -> Router` transforms,
    /// left-to-right (fold over `map`).
    ///
    /// Each item is a [`RouterTransform`] (`Box<dyn FnOnce(Router<()>) -> Router<()>>`)
    /// so heterogeneous transforms (different layer types) can coexist in one
    /// collection. For a small, static set of layers, chaining `.map()` is cleaner.
    ///
    /// ```ignore
    /// let transforms: Vec<RouterTransform> = vec![
    ///     Box::new(|r| r.layer(TraceLayer::new_for_http())),
    ///     Box::new(|r| r.layer(CorsLayer::permissive())),
    /// ];
    ///
    /// RouterPipeline::new()
    ///     .mount::<HealthController>(svc)
    ///     .layer_all(transforms)
    ///     .build()?
    /// ```
    pub fn layer_all(self, transforms: impl IntoIterator<Item = RouterTransform>) -> Self {
        transforms.into_iter().fold(self, |p, f| p.map(f))
    }

    /// Run a sub-pipeline and nest all of its routes under `prefix`.
    ///
    /// All controllers and routes registered inside the closure `f` will have
    /// `prefix` prepended to their paths before being merged into the outer
    /// router. This is the scoped functor: mapping a prefix transformation
    /// over an enclosed group of routes.
    ///
    /// ```ignore
    /// RouterPipeline::new()
    ///     .group("/api/v1", |g| g
    ///         .mount::<HealthController>(health_svc)
    ///         .mount::<EchoController>(echo_svc)
    ///     )
    ///     .group("/internal", |g| g
    ///         .mount_if::<MetricsController>(config.enable_metrics, metrics_svc)
    ///     )
    ///     .build()?
    /// ```
    pub fn group<F>(self, prefix: &str, f: F) -> Self
    where
        F: FnOnce(RouterPipeline) -> RouterPipeline,
    {
        let prefix = prefix.to_owned();
        self.and_then(move |outer| {
            let inner = f(RouterPipeline::new()).build()?;
            Ok(outer.merge(Router::new().nest(&prefix, inner)))
        })
    }
}

impl Default for RouterPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{controller::Controller, error::Result, router::Router};
    use axum::{body::Body, http::Request, routing::get};
    use tower::ServiceExt;

    // -----------------------------------------------------------------------
    // Minimal test controller — state is `()`, handler returns a static string.
    // Manually implements `Controller` so the test module has no external deps.
    // -----------------------------------------------------------------------

    struct PingController;

    impl Controller for PingController {
        type State = ();
        fn mount(state: Arc<Self::State>) -> impl FnOnce(Router<()>) -> Result<Router<()>> {
            move |router| {
                let scoped: Router<Arc<()>> =
                    Router::new().route("/ping", get(|| async { "pong" }));
                Ok(router.merge(scoped.with_state(state)))
            }
        }
    }

    fn ping_state() -> Arc<()> {
        Arc::new(())
    }

    async fn status(app: Router<()>, uri: &str) -> u16 {
        app.oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
        .as_u16()
    }

    // -----------------------------------------------------------------------
    // mount_guarded
    // -----------------------------------------------------------------------

    #[test]
    fn mount_guarded_short_circuits_on_err_guard() {
        let result = RouterPipeline::new()
            .mount_guarded::<PingController, _>(ping_state(), || {
                Err(crate::error::Error::other("guard failed"))
            })
            .build();

        assert!(result.is_err(), "build() should return Err when guard fails");
    }

    #[tokio::test]
    async fn mount_guarded_registers_route_on_ok_guard() {
        let app = RouterPipeline::new()
            .mount_guarded::<PingController, _>(ping_state(), || Ok(()))
            .build()
            .expect("build should succeed when guard passes");

        assert_eq!(status(app, "/ping").await, 200);
    }

    // -----------------------------------------------------------------------
    // mount_if
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn mount_if_false_route_returns_404() {
        let app = RouterPipeline::new()
            .mount_if::<PingController>(false, ping_state())
            .build()
            .expect("build should succeed even when mount_if is false");

        assert_eq!(status(app, "/ping").await, 404);
    }

    #[tokio::test]
    async fn mount_if_true_route_returns_200() {
        let app = RouterPipeline::new()
            .mount_if::<PingController>(true, ping_state())
            .build()
            .expect("build should succeed when mount_if is true");

        assert_eq!(status(app, "/ping").await, 200);
    }

    // -----------------------------------------------------------------------
    // group prefix
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn group_prefix_is_applied_to_routes() {
        let app = RouterPipeline::new()
            .group("/v1", |g| g.mount::<PingController>(ping_state()))
            .build()
            .expect("build should succeed");

        assert_eq!(status(app.clone(), "/v1/ping").await, 200, "/v1/ping should be 200");
        assert_eq!(status(app, "/ping").await, 404, "/ping without prefix should be 404");
    }

    // -----------------------------------------------------------------------
    // Error propagation
    // -----------------------------------------------------------------------

    #[test]
    fn error_from_and_then_propagates_through_remaining_steps() {
        let result = RouterPipeline::new()
            .and_then(|_| Err(crate::error::Error::other("intentional failure")))
            .mount::<PingController>(ping_state()) // should never run
            .build();

        assert!(result.is_err(), "error should propagate through the rest of the pipeline");
    }
}
