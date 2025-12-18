# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Plugin system for custom validators
- IDE integrations (VSCode, IntelliJ)
- Cloud secret manager integration (AWS Secrets Manager, HashiCorp Vault)
- Environment variable encryption

## [0.1.0] - 2025-12-14

### Added
- Initial release of env-cli
- Core CLI commands: `init`, `switch`, `scan`, `validate`, `sync`, `generate`, `status`
- Multi-language code scanning support (Rust, JavaScript, TypeScript, Python, Go, PHP, etc.)
- Environment variable validation with configurable rules
- Safe environment switching with confirmation prompts
- Auto-generation of `.env.example` files from code usage
- Shell completion support (bash, zsh, fish, PowerShell)
- Comprehensive error handling and user-friendly messages
- Cross-platform support (Windows, macOS, Linux)
- Parallel processing for fast code scanning
- TOML-based configuration system
- Security features:
  - Cryptographically secure secret generation
  - Sensitive pattern detection
  - Production environment protection
- Extensive test coverage (unit and integration tests)
- Complete documentation

### Technical Details
- Built with Rust 2021 edition
- Uses clap for CLI parsing
- Tokio for async operations
- Serde for serialization
- Regex for pattern matching
- Rayon for parallel processing

[Unreleased]: https://github.com/env-cli/env-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/env-cli/env-cli/releases/tag/v0.1.0

