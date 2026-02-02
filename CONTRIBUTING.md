# Contributing to rust-api

Thank you for your interest in contributing to rust-api! We welcome contributions from everyone.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and constructive in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- A clear and descriptive title
- Steps to reproduce the behavior
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Any relevant logs or error messages

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear and descriptive title
- A detailed description of the proposed functionality
- Why this enhancement would be useful
- Possible implementation approaches (optional)

### Pull Requests

1. Fork the repository and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure the test suite passes
4. Make sure your code lints (see below)
5. Write a clear commit message

## Development Process

### Setting Up Your Development Environment

```bash
# Clone your fork
git clone https://github.com/yourusername/rust-api.git
cd rust-api

# Build the project
cargo build

# Run tests
cargo test
```

### Code Style

We use `rustfmt` for code formatting and `clippy` for linting.

```bash
# Format your code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

All code must pass `cargo fmt --check` and `cargo clippy` before being merged.

### Testing

- Write unit tests for new functionality
- Ensure all tests pass with `cargo test`
- Add integration tests when appropriate
- Aim for high code coverage

### Commit Messages

- Use clear and meaningful commit messages
- Start with a verb in the imperative mood (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Add a detailed description if needed, separated by a blank line

Example:

```
Add user authentication endpoint

- Implement JWT token generation
- Add password hashing with bcrypt
- Include tests for auth flow
```

### Documentation

- Document all public APIs
- Update the README.md if you change functionality
- Add inline comments for complex logic
- Update CHANGELOG.md following Keep a Changelog format

## Pull Request Process

1. Update the README.md with details of changes to the interface, if applicable
2. Update the CHANGELOG.md with a note describing your changes
3. The PR will be merged once you have the sign-off of at least one maintainer

## Development Commands

```bash
# Run tests
cargo test

# Run tests with coverage
cargo tarpaulin --verbose --all-features

# Build documentation
cargo doc --open

# Run benchmarks
cargo bench

# Check for security vulnerabilities
cargo audit

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

## Questions?

Feel free to open an issue for any questions about contributing!

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
