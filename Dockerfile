# Multi-stage build — final image contains only the binary, not the Rust toolchain.
#
# Build:  docker build -t rust-api .
# Run:    docker run -p 3000:3000 -e ADMIN_API_KEY=secret rust-api

# ---------------------------------------------------------------------------
# Stage 1: builder
# ---------------------------------------------------------------------------
FROM rust:slim AS builder

WORKDIR /app

# Copy manifests first so dependency layers are cached separately from source.
COPY Cargo.toml Cargo.lock ./
COPY crates/rust-api/Cargo.toml         crates/rust-api/Cargo.toml
COPY crates/rust-api-macros/Cargo.toml  crates/rust-api-macros/Cargo.toml
COPY examples/basic-api/Cargo.toml      examples/basic-api/Cargo.toml

# Stub out source so Cargo can resolve and cache dependencies without full source.
RUN mkdir -p crates/rust-api/src \
             crates/rust-api-macros/src \
             examples/basic-api/src \
    && echo "fn main() {}" > examples/basic-api/src/main.rs \
    && echo "" > crates/rust-api/src/lib.rs \
    && echo "" > crates/rust-api-macros/src/lib.rs

RUN cargo build --release -p basic-api 2>/dev/null || true

# Now copy the real source and build for real.
COPY crates/ crates/
COPY examples/ examples/

# Touch all real source files to bust the cached stub build artifacts.
# Using find so new crates are automatically included without Dockerfile changes.
RUN find crates/ examples/ -name "*.rs" -exec touch {} + \
    && cargo build --release -p basic-api

# ---------------------------------------------------------------------------
# Stage 2: runtime
# ---------------------------------------------------------------------------
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/basic-api /usr/local/bin/basic-api

EXPOSE 3000

ENV RUST_LOG=info

CMD ["basic-api"]
