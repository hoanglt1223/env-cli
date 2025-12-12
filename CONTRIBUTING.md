# Contributing to env-cli

Thank you for your interest in contributing to env-cli! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust 1.70.0 or later (see `rust-toolchain.toml`)
- Git
- Make (optional, for using Makefile commands)

### Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/env-cli.git
   cd env-cli
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/original-org/env-cli.git
   ```
4. Install the pinned Rust toolchain:
   ```bash
   rustup toolchain install
   ```

### Development Commands

- `cargo build` - Build the project
- `cargo test` - Run tests
- `cargo nextest run` - Run tests with nextest (recommended)
- `cargo fmt` - Format code
- `cargo clippy` - Run lints
- `cargo deny check` - Check dependencies
- `cargo doc` - Generate documentation

## Code Style

We follow standard Rust conventions:

1. **Formatting**: Use `rustfmt` with default settings
2. **Linting**: Use `clippy` with default settings
3. **Documentation**: Public items should have `///` doc comments
4. **Testing**: Include tests for new functionality

### Code Organization

```
src/
├── main.rs           # CLI entry point
├── cli/              # CLI argument definitions
├── commands/         # Command implementations
├── config/           # Configuration management
├── env/              # Environment variable handling
├── scan/             # Code scanning functionality
├── utils/            # Utility functions
└── error.rs          # Error types
```

## Submitting Changes

### Branch Naming

Use descriptive branch names:
- `feature/description` - New features
- `bugfix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Refactoring

### Commit Messages

Follow conventional commits:
- `feat: add new command for environment validation`
- `fix: handle missing .env files gracefully`
- `docs: update installation instructions`
- `refactor: extract configuration logic into module`

### Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the documentation
3. Ensure your code follows the project's style guidelines
4. Ensure all tests pass
5. Ensure CI checks pass
6. Fill out the pull request template

### Testing

Run the full test suite before submitting:

```bash
cargo nextest run
cargo clippy --all-targets --all-features
cargo fmt --all --check
cargo deny check
```

## Architecture

### Core Components

1. **CLI Layer** (`cli/`): Argument parsing and command routing
2. **Command Layer** (`commands/`): Business logic for each command
3. **Configuration Layer** (`config/`): Configuration file handling
4. **Environment Layer** (`env/`): Environment variable operations
5. **Scanning Layer** (`scan/`): Code analysis and scanning
6. **Utilities** (`utils/`): Shared utility functions

### Error Handling

Use the `anyhow` crate for error handling:
```rust
use anyhow::{Context, Result};

fn example_function() -> Result<()> {
    some_operation()
        .context("Failed to perform some operation")?;
    Ok(())
}
```

### Configuration

Configuration is handled through:
- Command-line arguments (highest priority)
- Configuration files
- Environment variables
- Default values (lowest priority)

## Release Process

Releases are automated through GitHub Actions:

1. Create a new tag: `git tag v0.1.0`
2. Push the tag: `git push origin v0.1.0`
3. The release workflow will:
   - Build binaries for all platforms
   - Create a GitHub release
   - Publish to crates.io

## Getting Help

- Create an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues and discussions

## License

By contributing to env-cli, you agree that your contributions will be licensed under the MIT License.