# RustAPI

> **FastAPI-inspired REST framework for Rust**

[![Crates.io](https://img.shields.io/crates/v/rust-api.svg)](https://crates.io/crates/rust-api)
[![Documentation](https://docs.rs/rust-api/badge.svg)](https://docs.rs/rust-api)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/jrkosinski/rustapi/workflows/CI/badge.svg)](https://github.com/jrkosinski/rustapi/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

**Motivation**: to make it as easy as possible to spin up a quick REST API in Rust with minimal plumbing code. 

FastAPI in Python, and NestJS in JS/TS, make it easy to spin up a REST API. There are plenty of good reasons in which you might need a REST API defined in Rust, providing access (perhaps internal) to code that is best done in Rust. What I want is a FastAPI-like experience in Rust. This crate attempts to give that, as much as possible. The class-first definition of FastAPI, the composable middleware of NestJS. It offers:

- **Route Macros** - FastAPI-style endpoint definitions with enforced HTTP verbs
- **Composable Pipeline** - Type-safe, monadic router composition (Kleisli arrows)
- **Controller Pattern** - Clean separation of routes and business logic
- **Performance** - Built on Axum + Tokio with zero runtime overhead
- **Type Safety** - Compile-time route verification, no runtime panics
- **Future: Auto OpenAPI** - Documentation that stays in sync (coming soon)

**Status**: Active Development | Not yet production-ready

## Quick Start

```rust
use rust_api::prelude::*;

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

// Service layer
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_user(&self, id: &str) -> User {
        User {
            id: id.to_string(),
            name: format!("User {}", id)
        }
    }
}

// Controller layer
pub struct UserController;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, rust-api!"
}

#[get("/users/{id}")]
async fn get_user(
    Path(id): Path<String>,
    State(svc): State<Arc<UserService>>
) -> Json<User> {
    Json(svc.get_user(&id))
}

mount_handlers!(UserController, UserService, [
    (__get_user_route, get_user)
]);

#[tokio::main]
async fn main() {
    let user_svc = Arc::new(UserService::new());

    let app = RouterPipeline::new()
        .mount::<UserController>(user_svc)
        .route(__hello_route, hello)
        .build()
        .unwrap();

    RustAPI::new(app)
        .port(3000)
        .serve()
        .await
        .unwrap();
}
```

## Features

### ✅ Implemented

- **Route Macros**: `#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[patch]` with enforced HTTP verbs
- **RouterPipeline**: Composable, type-safe route building with Kleisli arrows
- **Controller Pattern**: Clean separation with `mount_handlers!` macro
- **Conditional Mounting**: `mount_if` and `mount_guarded` for feature flags and validation
- **Scoped Middleware**: Group-level auth and path prefixes
- **Prelude Module**: One import for everything you need

### Coming Soon

- **`Inject<T>` Extractor**: Automatic dependency injection in handlers
- **Validation**: `#[derive(Validate)]` with automatic error responses
- **OpenAPI Generation**: Auto-generated Swagger docs
- **Request-Scoped Services**: Per-request service instances
- **Testing Utilities**: Easy integration testing

## Examples

Run the working example to see the framework in action:

```bash
# Full-featured example with RouterPipeline, controllers, and middleware
cargo run --package basic-api
```

Then test the endpoints:

```bash
# Root endpoint
curl http://localhost:3000/

# Health check
curl http://localhost:3000/api/v1/health

# Echo endpoint
curl -X POST http://localhost:3000/api/v1/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello RustAPI"}'

# Metrics (requires ENABLE_METRICS env var)
ENABLE_METRICS=1 cargo run --package basic-api
curl http://localhost:3000/metrics

# Admin endpoint (requires ADMIN_API_KEY env var)
ADMIN_API_KEY=secret cargo run --package basic-api
curl -X POST http://localhost:3000/admin/reset \
  -H "Authorization: Bearer secret"
```

## Architecture

```
rust-api/
├── crates/
│   ├── rust-api/           # Main crate (RouterPipeline, Controller trait, middleware)
│   └── rust-api-macros/    # Route macros (#[get], #[post], etc.)
├── examples/
│   └── basic-api/         # Complete app demonstrating RouterPipeline with controllers
└── Cargo.toml             # Workspace configuration
```

## Comparison

| Feature         | rust-api    | axum | actix-web | poem | rocket |
| --------------- | ----------- | ---- | --------- | ---- | ------ |
| Route Macros    | ✅          | ❌   | ❌        | ❌   | ✅     |
| Built-in DI     | ✅          | ❌   | ✅        | ❌   | ❌     |
| Auto OpenAPI    | In Progress | ❌   | ❌        | ✅   | ❌     |
| FastAPI-like DX | ✅          | ❌   | ❌        | ~    | ~      |
| Performance     | High        | High | High      | High | High   |

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Complete architectural vision
- [CompositionalRefactor.md](docs/CompositionalRefactor.md) - Deep dive into the Kleisli Pipeline refactor
- [PROGRESS.md](PROGRESS.md) - Development progress
- [TODO.md](TODO.md) - Detailed roadmap
- [examples/](examples/) - Working code examples

## Roadmap

**Phase 1: Core** ✅

- [x] Route Macros with enforced HTTP verbs
- [x] RouterPipeline with Kleisli composition
- [x] Controller trait and mount_handlers! macro
- [x] Conditional and guarded mounting
- [x] Scoped middleware and route groups
- [x] Working example (basic-api)

**Phase 2: DX Improvements** (In Progress)

- [x] Better route registration (RouterPipeline)
- [x] Compile-time type safety (no runtime panics)
- [ ] `Inject<T>` extractor (alternative to State)
- [ ] Macro-generated app builder

**Phase 3: Validation** (Planned)

- [ ] `#[derive(Validate)]`
- [ ] Automatic validation
- [ ] Structured error responses

**Phase 4: OpenAPI** (Planned)

- [ ] Schema generation
- [ ] Swagger UI
- [ ] ReDoc support

## Why RustAPI?

**Python/FastAPI developers** get Rust performance with familiar patterns.

**TypeScript/NestJS developers** get dependency injection in Rust.

**Rust developers** get FastAPI-level developer experience.

## Contributing

This is currently in active development. Contributions welcome!

## License

This project is licensed under either of:

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Inspiration

- **FastAPI** (Python) - Amazing DX, automatic docs
- **NestJS** (TypeScript) - Dependency injection, modules
- **Axum** (Rust) - Performance, type safety

---

Built using Rust, Axum, and Tokio.
