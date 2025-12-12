# env-cli - Project Structure and Architecture

## Overview

**env-cli** is a professional Rust CLI tool for environment variable management, built following best practices from successful Rust CLI projects like fnm. This document outlines the project structure, architecture, and development practices.

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

1. **CLI Layer** (`src/cli/`):
   - Uses `clap` for argument parsing
   - Defines command structure and options
   - Handles help generation and version info

2. **Command Layer** (`src/commands/`):
   - Business logic for each command
   - Async execution using `tokio`
   - Error handling and user feedback

3. **Configuration Layer** (`src/config/`):
   - TOML-based configuration files
   - Environment and scan settings
   - Validation rules and security policies

4. **Environment Layer** (`src/env/`):
   - Environment variable management
   - .env file parsing and generation
   - Usage tracking and analysis

5. **Scanning Layer** (`src/scan/`):
   - Multi-language source code analysis
   - Pattern-based env usage detection
   - Parallel processing for large codebases

6. **Utilities Layer** (`src/utils/`):
   - File system operations
   - Security utilities (secret generation)
   - Helper functions and common patterns

### Error Handling

- Custom error types in `src/error.rs`
- Contextual error messages using `anyhow`
- Proper error propagation and user-friendly output

### Testing Strategy

- **Unit tests**: Module-level tests for core functionality
- **Integration tests**: End-to-end command testing
- **Property-based testing**: Where applicable
- **Cross-platform testing**: Windows, macOS, Linux

## Development Practices

### Code Quality

1. **Formatting**: `rustfmt` with default settings
2. **Linting**: `clippy` with strict warnings
3. **Documentation**: Public items have `///` doc comments
4. **Error handling**: Comprehensive error types and messages

### Dependencies

- Minimal, well-vetted dependencies
- Regular security audits using `cargo deny`
- MSRV (Minimum Supported Rust Version) of 1.70.0

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

### Toolchain Configuration

- **Rust 1.70.0+** (MSRV)
- **rust-toolchain.toml**: Ensures consistent toolchain across environments
- **nextest.toml**: Optimized test runner configuration

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

## Development Workflow

1. **Setup**: `make setup` - Install development dependencies
2. **Development**: `make build` - Build the project
3. **Testing**: `make test` - Run all tests
4. **Quality Checks**: `make check` - Run formatting, linting, and security checks
5. **Release**: `make release` - Build release binaries

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