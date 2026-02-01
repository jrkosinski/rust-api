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
- `GET /health` - Health check endpoint
- `POST /echo` - Echo service that returns your message with a counter

## Testing

```bash
# Root endpoint
curl http://localhost:3000/

# Health check
curl http://localhost:3000/health

# Echo endpoint
curl -X POST -H "Content-Type: application/json" \
  -d '{"message":"Hello RustAPI"}' \
  http://localhost:3000/echo
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

1. **Dependency Injection**: Services are registered in a DI container and resolved at runtime
2. **State Management**: Each router can have its own state (service instance)
3. **Route Merging**: Separate routers are merged into a single application
4. **Middleware Layers**: CORS and tracing are added as middleware layers
