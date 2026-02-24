use std::sync::atomic::{AtomicU64, Ordering};

use rust_api::prelude::*;

/// Response type for the echo endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub data: String,
    pub count: u64,
}

/// Echo service — uses an atomic counter for lock-free call tracking.
/// Immutable interface: `&self` only. State changes via AtomicU64, not Mutex.
pub struct EchoService {
    call_count: AtomicU64,
}

impl EchoService {
    pub fn new() -> Self {
        Self {
            call_count: AtomicU64::new(0),
        }
    }

    pub fn echo(&self, value: &str) -> EchoResponse {
        // atomically increment and get the new value
        let count = self.increment_counter();

        self.create_response(value, count)
    }

    // increment the call counter atomically
    fn increment_counter(&self) -> u64 {
        self.call_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    // create an echo response with the given value and count
    fn create_response(&self, value: &str, count: u64) -> EchoResponse {
        EchoResponse {
            data: format!("{}: {}", count, value),
            count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_returns_message_with_count() {
        let svc = EchoService::new();
        let resp = svc.echo("hello");
        assert_eq!(resp.count, 1);
        assert_eq!(resp.data, "1: hello");
    }

    #[test]
    fn echo_increments_counter_on_each_call() {
        let svc = EchoService::new();
        let first = svc.echo("a");
        let second = svc.echo("b");
        let third = svc.echo("c");
        assert_eq!(first.count, 1);
        assert_eq!(second.count, 2);
        assert_eq!(third.count, 3);
    }

    #[test]
    fn echo_data_format_includes_count_prefix() {
        let svc = EchoService::new();
        let resp = svc.echo("world");
        assert!(resp.data.starts_with("1: "), "data should start with '<count>: '");
        assert!(resp.data.ends_with("world"));
    }

    #[test]
    fn echo_counter_is_independent_per_instance() {
        let svc_a = EchoService::new();
        let svc_b = EchoService::new();
        svc_a.echo("x");
        svc_a.echo("x");
        let b_resp = svc_b.echo("x");
        assert_eq!(b_resp.count, 1, "each service instance has its own counter");
    }
}
