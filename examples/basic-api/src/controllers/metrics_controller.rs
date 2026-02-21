use rust_api::prelude::*;

use crate::services::metrics_service::{MetricsResponse, MetricsService};

/// GET /metrics — returns runtime counters.
///
/// This controller is only wired when `ENABLE_METRICS` is set in the
/// environment. The `mount_if` call in `main.rs` handles the condition —
/// this file has no knowledge of it.
#[get("/metrics")]
pub async fn metrics(
    State(svc): State<Arc<MetricsService>>,
) -> Json<MetricsResponse> {
    Json(svc.record_and_snapshot())
}

/// Controller marker for metrics routes.
pub struct MetricsController;

mount_handlers!(MetricsController, MetricsService, [
    (__metrics_route, metrics),
]);
