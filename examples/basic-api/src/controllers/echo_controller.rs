use rust_api::prelude::*;

use crate::services::echo_service::{EchoResponse, EchoService};

/// Request body for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

/// POST /echo — echoes the message with an invocation counter.
///
/// The `#[post]` annotation is a binding contract: this handler is always
/// registered as a POST endpoint. The verb cannot be overridden at mount time.
#[post("/echo")]
pub async fn echo(
    State(svc): State<Arc<EchoService>>,
    Json(payload): Json<EchoRequest>,
) -> Json<EchoResponse> {
    Json(svc.echo(&payload.message))
}

/// Controller marker for echo-related routes.
///
/// Has no routing knowledge — `mount_handlers!` generates the Kleisli arrow
/// that the `RouterPipeline` threads via `and_then`.
pub struct EchoController;

// Generates `impl Controller for EchoController` with the Kleisli mount fn.
mount_handlers!(EchoController, EchoService, [
    (__echo_route, echo),
]);
