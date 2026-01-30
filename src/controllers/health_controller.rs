use axum::{extract::State, http::StatusCode, Json};
use crate::services::health_service::{HealthService, HealthResponse};
use std::sync::Arc;

/// Health controller that manages health check endpoints.
/// This struct can hold shared state, dependencies, and provides lifecycle management.
#[derive(Clone)]
pub struct HealthController {
    health_service: Arc<HealthService>,
}

impl HealthController {
    pub fn new() -> Self {
        Self {
            //initialize dependencies here
            health_service: Arc::new(HealthService::new()),
        }
    }

    /// Health check endpoint that returns the service status.
    /// Returns a 200 OK status with a JSON response indicating the service is healthy.
    /// This is a pass-through to the health service.
    pub async fn health_check(
        State(controller): State<Arc<Self>>
    ) -> (StatusCode, Json<HealthResponse>) {
        let response = controller.health_service.health_check();
        (StatusCode::OK, Json(response))
    }
}

impl Default for HealthController {
    fn default() -> Self {
        Self::new()
    }
}
