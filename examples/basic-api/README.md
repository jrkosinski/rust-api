# Basic API Example

This example demonstrates a complete REST API using the RustAPI framework with:

- **Route Macros**: Using `#[get]`, `#[post]` for clean endpoint definitions
- **Dependency Injection**: Type-safe DI container for services
- **FastAPI-style Routing**: Decorator-based routing with auto-generated path constants
- **Middleware**: CORS and request tracing

## Running the Example

```bash
cargo run --package basic-api
```

The server will start on `http://localhost:3000`.

## Available Endpoints

- `GET /` - Welcome message
- `GET /api/v1/health` - Health check endpoint
- `POST /api/v1/echo` - Echo service that returns your message with a counter
- `GET /metrics` - Metrics endpoint (when ENABLE_METRICS is set)
- `POST /admin/reset` - Admin endpoint (requires ADMIN_API_KEY)

## Testing

```bash
# Root endpoint
curl http://localhost:3000/

# Health check
curl http://localhost:3000/api/v1/health

# Echo endpoint
curl -X POST http://localhost:3000/api/v1/echo \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello RustAPI"}'

# Metrics (requires ENABLE_METRICS env var)
ENABLE_METRICS=1 cargo run --package basic-api
curl http://localhost:3000/metrics

# Admin endpoint (requires ADMIN_API_KEY env var)
ADMIN_API_KEY=secret cargo run --package basic-api
curl -X POST http://localhost:3000/admin/reset \
  -H "Authorization: Bearer secret"
```

## Project Structure

```
basic-api/
├── src/
│   ├── main.rs              # Application entry point
│   ├── controllers/         # HTTP request handlers
│   │   ├── health_controller.rs
│   │   └── echo_controller.rs
│   └── services/            # Business logic layer
│       ├── health_service.rs
│       └── echo_service.rs
└── Cargo.toml
```

## Key Features Demonstrated

1. **RouterPipeline Composition**: Services are passed directly as `Arc<Service>` to `mount::<Controller>()`
2. **Kleisli Arrows**: Controllers are pure functions that compose via `and_then` (`>>=`)
3. **Conditional Mounting**: `mount_if` and `mount_guarded` for feature flags and startup validation
4. **Scoped Middleware**: `group()` with path prefixes and scoped auth layers
5. **Verb Enforcement**: HTTP methods from `#[get]`/`#[post]` annotations are binding contracts
6. **Zero Axum Imports**: User code only imports `rust_api::prelude::*`
