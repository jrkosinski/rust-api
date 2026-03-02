use rust_api::prelude::*;

/// Response type for the health check endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Health service — immutable after construction.
/// All methods take `&self`; no `Mutex` needed.
pub struct HealthService {
    // state fields here
}

impl HealthService {
    pub fn new() -> Self {
        Self {
            //initialize dependencies here
        }
    }

    /// Health check service that returns the service status.
    /// This contains the business logic for determining service health.
    pub fn health_check(&self) -> HealthResponse {
        HealthResponse {
            status: "healthy".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_check_returns_healthy_status() {
        let svc = HealthService::new();
        let resp = svc.health_check();
        assert_eq!(resp.status, "healthy");
    }

    #[test]
    fn health_check_is_idempotent() {
        let svc = HealthService::new();
        let first = svc.health_check();
        let second = svc.health_check();
        assert_eq!(first.status, second.status);
    }
}
