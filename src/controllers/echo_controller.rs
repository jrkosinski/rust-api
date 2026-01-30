use axum::{extract::State, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use crate::services::{echo_service::EchoResponse, echo_service::EchoService};
use std::sync::Arc;

#[derive(Clone)]
pub struct EchoController {
    echo_service: Arc<EchoService>
}

/// Request type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    message: String,
}

impl EchoController
{
    pub fn new() -> Self {
        Self {
            echo_service: Arc::new(EchoService::new())
        }
    }

    pub async fn echo(State(
        controller): State<Arc<Self>>,
        Json(payload): Json<EchoRequest>) -> (StatusCode, Json<EchoResponse>)
    {
        let response: EchoResponse = controller.echo_service.echo(&payload.message);
        (StatusCode::OK, Json(response))
    }
}