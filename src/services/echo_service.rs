use serde::{Deserialize, Serialize};
use axum::{extract::State };
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Response type for the health check endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub data: String,
    pub count: u64,
}

/// Echo Service implementation
pub struct EchoService {
    call_count: AtomicU64,
}

impl EchoService {
    pub fn new() -> Self {
        Self {
            call_count: AtomicU64::new(0)
        }
    }

    pub fn echo(&self, value: &str) -> EchoResponse {
        // Atomically increment and get the new value
        let count = self.call_count.fetch_add(1, Ordering::SeqCst) + 1;

        EchoResponse {
            data: format!("{}: {}", count, value),
            count
        }
    }
}

