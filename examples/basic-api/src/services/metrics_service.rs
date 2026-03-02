use std::sync::atomic::{AtomicU64, Ordering};

use rust_api::prelude::*;

/// Response type for the metrics endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub requests_total: u64,
    pub uptime_seconds: u64,
}

/// Metrics service — tracks basic runtime counters.
///
/// Immutable interface: `&self` only. Counters mutate via atomics, not `Mutex`.
/// This service is conditionally wired by `mount_if` in the pipeline — if
/// `ENABLE_METRICS` is not set, this struct is never placed in a `Router`.
pub struct MetricsService {
    requests_total: AtomicU64,
    started_at: std::time::Instant,
}

impl MetricsService {
    pub fn new() -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            started_at: std::time::Instant::now(),
        }
    }

    /// Increment the request counter and return a snapshot of current metrics.
    pub fn record_and_snapshot(&self) -> MetricsResponse {
        let requests_total = self.requests_total.fetch_add(1, Ordering::Relaxed) + 1;
        let uptime_seconds = self.started_at.elapsed().as_secs();
        MetricsResponse {
            requests_total,
            uptime_seconds,
        }
    }
}
