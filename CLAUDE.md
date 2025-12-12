# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# env-cli - Project Structure and Architecture

## Overview

**env-cli** is a professional Rust CLI tool for environment variable management, built following best practices from successful Rust CLI projects like fnm. This document outlines the project structure, architecture, and development practices.
Use docs/vibe-kanban-api.md to self create task, start task in concurrent

## Project Structure

```
env-cli/
├── .github/
│   └── workflows/           # CI/CD workflows
│       ├── CI.yml          # Continuous integration
│       ├── Release.yml     # Release automation
│       └── Docs.yml        # Documentation generation
├── docs/                   # Documentation
│   ├── tasks/             # Task specifications (EC-01, EC-02, etc.)
│   │   ├── EC-01.md       # Completed: Rust project initialization
│   │   ├── EC-02.md       # TODO: Core functionality implementation
│   │   ├── EC-03.md       # TODO: Advanced features
│   │   └── planning.md    # Project planning and roadmap
│   ├── ARCHITECTURE.md    # Technical architecture
│   ├── CLI_SPEC.md        # CLI command specifications
│   ├── DEVELOPMENT.md     # Development guidelines
│   └── README.md          # User documentation
├── src/                   # Source code
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library interface
│   ├── error.rs          # Error types and handling
│   ├── cli/              # CLI argument definitions
│   │   └── mod.rs        # Command-line interface using clap
│   ├── commands/         # Command implementations
│   │   ├── mod.rs        # Command dispatcher
│   │   ├── init.rs       # Initialize command
│   │   ├── switch.rs     # Environment switching
│   │   ├── scan.rs       # Code scanning
│   │   ├── validate.rs   # Environment validation
│   │   ├── sync.rs       # Environment synchronization
│   │   ├── generate.rs   # .env.example generation
│   │   └── status.rs     # Status reporting
│   ├── config/           # Configuration management
│   │   └── mod.rs        # Configuration structures and loading
│   ├── env/              # Environment variable handling
│   │   └── mod.rs        # Environment variable utilities
│   ├── scan/             # Code scanning functionality
│   │   └── mod.rs        # Source code analysis for env usage
│   └── utils/            # Utility functions
│       └── mod.rs        # Helper functions and tools
├── tests/                # Integration tests
│   └── integration_tests.rs
├── Cargo.toml           # Cargo configuration
├── Cargo.lock           # Lock file (gitignored)
├── rust-toolchain.toml  # Toolchain specification
├── nextest.toml         # Test runner configuration
├── deny.toml            # Dependency security policy
├── Makefile             # Build automation
├── LICENSE.md           # MIT License
├── CONTRIBUTING.md      # Contribution guidelines
├── README.md            # Project README
├── CLAUDE.md            # This file
├── todo.md              # Task tracking
└── .gitignore           # Git ignore patterns
```

## Architecture

### Core Components

1. **CLI Layer** (`src/cli/mod.rs`):
   - Uses `clap` with derive macros for argument parsing
   - Defines `Cli` struct and `Commands` enum with subcommands
   - Supports multiple output formats (text, json, yaml) for scan results
   - Entry point: `Cli::parse()` in `main.rs:12`

2. **Command Layer** (`src/commands/`):
   - Dispatcher in `mod.rs:execute_command()` routes commands to handlers
   - Each command in separate file (init.rs, switch.rs, scan.rs, etc.)
   - All commands are async functions using `tokio`
   - Returns `Result<()>` for consistent error handling

3. **Configuration Layer** (`src/config/mod.rs`):
   - TOML-based configuration management using serde
   - Project settings in `.env/config.toml`
   - Environment definitions and validation rules
   - Scan patterns and security policies

4. **Environment Layer** (`src/env/mod.rs`):
   - Environment variable parsing and manipulation
   - .env file handling with validation
   - Environment switching logic
   - Secret generation and management

5. **Scanning Layer** (`src/scan/mod.rs`):
   - Multi-language source code analysis
   - Regex-based pattern matching for env usage
   - Parallel processing capabilities for large codebases
   - Support for various file types and exclusion patterns

6. **Utilities Layer** (`src/utils/mod.rs`):
   - File system operations and path handling
   - Security utilities (random secret generation)
   - Common helper functions and error context

### Error Handling Architecture

- Custom error type `EnvCliError` in `src/error.rs` with variants for different domains
- `Result<T>` type alias for convenience
- Error context trait `ErrorContext` for better error messages
- Automatic conversions from `std::io::Error` and serialization errors
- All error messages are user-friendly and actionable

### Testing Strategy

- **Unit tests**: Module-level tests for core functionality
- **Integration tests**: End-to-end command testing
- **Property-based testing**: Where applicable
- **Cross-platform testing**: Windows, macOS, Linux

## Development Practices

### Code Quality Standards

- **Formatting**: `rustfmt` with default settings (enforced in CI)
- **Linting**: `clippy` with `-D warnings` (treat warnings as errors)
- **Documentation**: All public items require `///` doc comments (enforced with `#![deny(missing_docs)]`)
- **Error handling**: Comprehensive error types with `anyhow` for context
- **Dependencies**: Minimal, well-vetted dependencies with regular security audits

### Toolchain and Dependencies

- **Rust 1.70.0+** MSRV defined in `rust-toolchain.toml`
- **Async runtime**: `tokio` with full features for async operations
- **CLI parsing**: `clap` 4.5 with derive macros
- **Serialization**: `serde` + `serde_json` + `toml` for config handling
- **Security**: `cargo-deny` for dependency vulnerability scanning
- **Testing**: `cargo-nextest` as primary test runner, `cargo-llvm-cov` for coverage

### CI/CD Pipeline

1. **Continuous Integration**:
   - Multi-platform testing (Linux, Windows, macOS)
   - Multiple Rust versions (stable, beta, MSRV)
   - Code formatting and linting checks
   - Security vulnerability scanning
   - Code coverage reporting

2. **Release Automation**:
   - Automated binary builds for all platforms
   - GitHub release creation
   - crates.io publishing
   - Documentation deployment

### Command Structure

The CLI follows a hierarchical command structure:

```
env <COMMAND> [OPTIONS]

Commands:
  init         Initialize project with env structure
  switch       Switch between environments
  scan         Scan code for env usage
  validate     Validate environment configuration
  sync         Sync environments safely
  generate     Generate .env.example file
  status       Show current environment status
```

## Configuration

### Project Configuration (`.env/config.toml`)

```toml
[project]
name = "my-project"
default_environment = "development"

[[environments]]
name = "development"
description = "Development environment"
file = ".env/development.env"

[scan]
include_dirs = ["src", "lib"]
exclude_dirs = ["target", "node_modules"]
include_patterns = ["*.rs", "*.js", "*.ts"]
exclude_patterns = ["*.min.js"]

[validation]
required = ["DATABASE_URL", "API_KEY"]

[validation.security]
sensitive_patterns = [".*KEY.*", ".*SECRET.*"]
min_secret_length = 16
```

### Toolchain Configuration Files

- **rust-toolchain.toml**: Ensures consistent Rust version across environments
- **nextest.toml**: Optimized test runner configuration
- **deny.toml**: Security policy for dependency auditing
- **Makefile**: Comprehensive build automation with all development tasks

## Security Considerations

1. **Secret Management**: Secure generation and storage of sensitive variables
2. **Environment Isolation**: Safe switching between environments
3. **Input Validation**: Comprehensive validation of all inputs
4. **Dependency Security**: Regular audits and vulnerability scanning

## Performance Optimizations

1. **Parallel Processing**: Uses `rayon` for CPU-intensive operations
2. **Async Operations**: Non-blocking I/O for file operations
3. **Efficient Scanning**: Regex patterns optimized for performance
4. **Memory Management**: Careful handling of large codebases

## Development Workflow and Commands

### Essential Commands

**Setup and Building:**
- `make setup` - Install development dependencies (rustfmt, clippy, cargo-nextest, etc.)
- `make build` - Build the project in debug mode
- `make build-release` - Build optimized release binaries

**Testing:**
- `make test` - Run tests using cargo-nextest (optimized test runner)
- `make test-coverage` - Run tests with coverage report (cargo llvm-cov)
- `cargo test <test_name>` - Run a specific test
- `make test-all` - Run tests + linting + formatting checks

**Code Quality:**
- `make check` - Run formatting check, clippy linting, and security audit
- `make check-format` - Check code formatting without modifying files
- `make format` - Format code with rustfmt
- `make lint` - Run clippy lints with strict warnings
- `make audit` - Run security audit with cargo-deny

**Development Workflow:**
- `make ci` - Run all CI checks (formatting, linting, tests, security audit)
- `make clean` - Clean build artifacts and documentation
- `make install` - Build and install the binary locally
- `make docs` - Generate documentation
- `make docs-open` - Generate and open documentation in browser

**Release:**
- `make release` - Build release binaries for all target platforms
- `make release-patch` - Create a patch release with version bump and git tag

### Testing Individual Components

Run tests for specific modules:
```bash
cargo test -p env-cli cli::tests        # Test CLI parsing
cargo test -p env-cli commands::tests   # Test command logic
cargo test -p env-cli config::tests     # Test configuration
cargo test -p env-cli scan::tests       # Test scanning functionality
```

### Development Notes

- Uses `cargo-nextest` as the primary test runner (faster and better UX than cargo test)
- Security auditing is enforced via `cargo-deny` with policy in `deny.toml`
- Pre-commit hooks can be installed with `make pre-commit` to run quality checks automatically
- The binary name is `env` (defined in Cargo.toml), not `env-cli`
- All commands are async and must be `.await`ed in tests

## Future Enhancements

### Phase 2 (EC-02)
- Core functionality implementation
- Advanced environment switching
- Comprehensive scanning algorithms
- Integration with popular frameworks

### Phase 3 (EC-03)
- Plugin system
- Advanced configuration options
- Performance optimizations
- Shell completion and integration

## Contributing

1. Follow the contribution guidelines in `CONTRIBUTING.md`
2. Use the provided development tools and workflows
3. Ensure all tests pass and quality checks are successful
4. Update documentation for new features

## Inspiration and References

This project draws inspiration from successful Rust CLI tools:

- **fnm**: CI/CD patterns and project structure
- **ripgrep**: Performance optimization techniques
- **bat**: User experience and output formatting
- **exa**: Modern CLI design principles

The project follows the [Rust CLI Guidelines](https://rust-cli.github.io/book/) and incorporates best practices from the broader Rust ecosystem.

Vibe-kanban
