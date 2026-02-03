# RustAPI Framework Architecture

**Vision**: FastAPI-inspired REST framework for Rust with automatic OpenAPI, validation, and DI.

## Core Principles

1. **Type-Driven Development** - Types define validation, serialization, and documentation
2. **Minimal Boilerplate** - Macros handle repetitive code
3. **Developer Experience First** - Great errors, automatic docs, easy testing
4. **Rust-Idiomatic** - Embrace Rust patterns, don't fight them
5. **Performance** - Built on tokio/axum for production-grade speed

## Architecture Layers

```
┌─────────────────────────────────────┐
│   User Application Code             │
│   (Routes, Services, Models)        │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│   RustAPI Macros                │
│   #[get], #[post], #[injectable]    │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│   RustAPI Core                  │
│   - DI Container                    │
│   - Validation Engine               │
│   - OpenAPI Generator               │
│   - Testing Utilities               │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│   Axum + Tokio                      │
│   (HTTP Server Foundation)          │
└─────────────────────────────────────┘
```

## Key Components

### 1. Route Macros (`rust-api-macros`)

Procedural macros for route definitions:

```rust
#[get("/users/{id}")]
async fn get_user(
    path: Path<String>,
    db: Inject<Database>,
) -> Result<Json<User>> {
    // Implementation
}
```

**Generates:**

- Axum handler registration
- OpenAPI schema for this route
- Parameter validation
- Dependency injection setup

### 2. Dependency Injection (`rust-api::di`)

Simple, type-safe DI container:

```rust
#[injectable]
pub struct UserService {
    db: Inject<Database>,
}

impl UserService {
    pub async fn get_user(&self, id: &str) -> Result<User> {
        // Implementation
    }
}
```

**Features:**

- Constructor injection
- Singleton and scoped lifetimes
- Trait-based services for testing
- Automatic Arc wrapping

### 3. Validation (`rust-api-validate`) [Planned]

Derive-based validation with great errors:

```rust
#[derive(Deserialize, Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 50))]
    password: String,

    #[validate(range(min = 18, max = 120))]
    age: u8,
}
```

**Returns:**

- Structured validation errors
- HTTP 422 with field-level errors
- Automatic OpenAPI validation schema

### 4. OpenAPI Generation (`rust-api-openapi`) [Planned]

Automatic OpenAPI 3.0 spec generation:

```rust
let app = App::new()
    .register(get_user)
    .register(create_user)
    .openapi("/docs")  // Swagger UI at /docs
    .build();
```

**Generates:**

- Complete OpenAPI 3.0 JSON/YAML
- Swagger UI integration
- ReDoc support
- Type-safe schemas from Rust types

### 5. Application Builder (`rust-api::app`)

Ergonomic app construction:

```rust
#[tokio::main]
async fn main() {
    App::new()
        .service::<Database>()
        .service::<UserService>()
        .route(get_user)
        .route(create_user)
        .middleware(cors())
        .openapi("/docs")
        .serve("0.0.0.0:3000")
        .await
        .unwrap();
}
```

## Project Structure

### Current Structure

```
rust-api/
├── crates/
│   ├── rust-api/           # Main crate (DI, app, server, router, error)
│   └── rust-api-macros/    # Proc macros (#[get], #[post], etc.)
├── examples/
│   ├── hello_world.rs     # Minimal example
│   ├── with_macros.rs     # Full-featured example
│   └── basic-api/         # Complete app with DI
└── Cargo.toml             # Workspace configuration
```

### Future Structure (Post v1.0)

```
rust-apii/
├── crates/
│   ├── rust-api/           # Main facade crate
│   ├── rust-api-core/      # Core runtime (DI, app builder)
│   ├── rust-api-macros/    # Proc macros
│   ├── rust-api-validate/  # Validation system (planned)
│   └── rust-api-openapi/   # OpenAPI generation (planned)
├── examples/
│   ├── hello-world/
│   ├── todo-app/
│   └── microservices/
└── tests/
    └── integration/
```

## Implementation Phases

### Phase 1: Core Foundation ✅

- [x] Basic controller-service pattern
- [x] Define macro API surface
- [x] Build DI container
- [x] Create app builder

### Phase 2: Macro System ✅

- [x] Route macros (#[get], #[post], etc.)
- [x] Integration with axum
- [ ] Injectable macro (future)
- [ ] Validation derive macro (future)

### Phase 3: Validation & Errors

- [ ] Validation engine
- [ ] Error handling & formatting
- [ ] HTTP error responses
- [ ] Testing utilities

### Phase 4: OpenAPI

- [ ] Schema generation from types
- [ ] Route documentation
- [ ] Swagger UI integration
- [ ] Export to JSON/YAML

### Phase 5: Polish & Docs

- [ ] Comprehensive examples
- [ ] Documentation site
- [ ] Performance benchmarks
- [ ] Community feedback

## Success Metrics

1. **Developer Experience**
   - Lines of code: 50% less than plain axum
   - Time to first endpoint: < 5 minutes
   - Learning curve: Familiar to FastAPI/NestJS devs

2. **Performance**
   - Overhead: < 5% vs raw axum
   - Compile time: Reasonable (< 30s for medium project)
   - Runtime: Production-ready

3. **Adoption**
   - Clear value proposition vs existing frameworks
   - Good documentation
   - Active examples and community

## Comparison to Existing Solutions

| Feature         | rust-api | axum   | actix-web | poem | rocket |
| --------------- | -------- | ------ | --------- | ---- | ------ |
| Route Macros    | ✅       | ❌     | ❌        | ❌   | ✅     |
| Built-in DI     | ✅       | ❌     | ✅        | ❌   | ❌     |
| Auto OpenAPI    | Planned  | ❌     | ❌        | ✅   | ❌     |
| Validation      | Planned  | Manual | Manual    | ✅   | ✅     |
| FastAPI-like DX | ✅       | ❌     | ❌        | ~    | ~      |
| Maturity        | 🚧       | ✅     | ✅        | ~    | ✅     |

## Next Steps

1. ✅ ~~Prototype DI container~~
2. ✅ ~~Design macro syntax~~
3. ✅ ~~Build minimal working example~~
4. ✅ ~~Validate architecture with real use case~~
5. Improve route registration ergonomics (Inject<T> extractor)
6. Add validation system (rust-api-validate crate)
7. Implement OpenAPI generation (rust-api-openapi crate)
8. Iterate based on community feedback
