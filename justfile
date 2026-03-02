# Run all tests across the workspace
test:
    cargo test --workspace

# Run tests with HTML coverage report (opens in browser)
# Requires: cargo install cargo-llvm-cov
cov:
    cargo llvm-cov --open

# Generate LCOV coverage report (for CI)
cov-lcov:
    cargo llvm-cov --lcov --output-path target/lcov.info

# Run the example app (admin routes disabled — no ADMIN_API_KEY set)
run:
    cargo run -p basic-api

# Run with admin routes enabled
run-admin:
    ADMIN_API_KEY=secret cargo run -p basic-api

# Run with metrics endpoint enabled
run-metrics:
    ENABLE_METRICS=1 cargo run -p basic-api

# Run with all features enabled
run-full:
    ADMIN_API_KEY=secret ENABLE_METRICS=1 cargo run -p basic-api

# Check for compile errors without producing a binary
check:
    cargo check --workspace

# Format all source files
fmt:
    cargo fmt --all

# Run Clippy lints (warnings treated as errors)
lint:
    cargo clippy --workspace -- -D warnings

# Build release binary
build:
    cargo build --release -p basic-api

# Remove build artifacts
clean:
    cargo clean
