use rustapi::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Response type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub data: String,
    pub count: u64,
}

/// Echo Service implementation
pub struct EchoService {
    call_count: AtomicU64,
}

impl Injectable for EchoService {}

impl EchoService {
    pub fn new() -> Self {
        Self {
            call_count: AtomicU64::new(0)
        }
    }

    pub fn echo(&self, value: &str) -> EchoResponse {
        //atomically increment and get the new value
        let count = self.increment_counter();

        self.create_response(value, count)
    }

    //increment the call counter atomically
    fn increment_counter(&self) -> u64 {
        self.call_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    //create an echo response with the given value and count
    fn create_response(&self, value: &str, count: u64) -> EchoResponse {
        EchoResponse {
            data: format!("{}: {}", count, value),
            count
        }
    }
}

