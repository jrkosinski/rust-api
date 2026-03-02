use rust_api::prelude::*;

use crate::services::health_service::{HealthResponse, HealthService};

/// GET /health — returns service status.
///
/// The `#[get]` annotation is a binding contract: this handler is always
/// registered as a GET endpoint. The verb cannot be overridden at mount time.
#[get("/health")]
pub async fn health_check(State(svc): State<Arc<HealthService>>) -> Json<HealthResponse> {
    Json(svc.health_check())
}

/// Controller marker for health-related routes.
///
/// Has no routing knowledge — `mount_handlers!` generates the Kleisli arrow
/// that the `RouterPipeline` threads via `and_then`.
pub struct HealthController;

// Generates `impl Controller for HealthController` with the Kleisli mount fn.
// Controllers know about: their service type, their handlers, and nothing else.
mount_handlers!(
    HealthController,
    HealthService,
    [(__health_check_route, health_check),]
);
