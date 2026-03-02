use rust_api::prelude::*;

/// Response type for the admin status endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminStatusResponse {
    pub status: String,
    pub message: String,
}

/// Admin service — pure business logic, no auth knowledge.
///
/// Authentication is handled by the `require_bearer` Tower layer applied to the
/// admin route group in the pipeline. This service receives only authenticated
/// requests and has no concept of tokens or keys.
pub struct AdminService;

impl AdminService {
    pub fn new() -> Self {
        Self
    }

    pub fn status(&self) -> AdminStatusResponse {
        AdminStatusResponse {
            status: "ok".to_string(),
            message: "Admin subsystem is operational.".to_string(),
        }
    }
}
