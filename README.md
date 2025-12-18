# env-cli

[![CI](https://github.com/env-cli/env-cli/workflows/CI/badge.svg)](https://github.com/env-cli/env-cli/actions)
[![Crates.io](https://img.shields.io/crates/v/env-cli.svg)](https://crates.io/crates/env-cli)
[![Documentation](https://docs.rs/env-cli/badge.svg)](https://docs.rs/env-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/env-cli.svg)](https://crates.io/crates/env-cli)

**The missing CLI for environment variable management** — A powerful, open-source tool that manages the entire lifecycle of environment variables for full-stack projects.

> 🚀 **Status**: Production-ready. Built with Rust for performance and reliability.

## 🚀 Quick Start

### Installation

**From crates.io (Recommended)**
```bash
cargo install env-cli
```

**From source**
```bash
git clone https://github.com/env-cli/env-cli.git
cd env-cli
cargo install --path .
```

**Pre-built binaries**
Download from [GitHub Releases](https://github.com/env-cli/env-cli/releases)

### Basic Usage

```bash
# Initialize your project
env init

# Scan code to find environment variable usage
env scan

# Switch between environments
env switch development
env switch production

# Validate configuration
env validate --env production

# Generate .env.example from actual code usage
env generate

# Check current status
env status
```

## 📚 Documentation

- **[User Guide & Features](docs/README.md)** - Complete user documentation
- **[CLI Reference](docs/CLI_SPEC.md)** - Detailed command specifications
- **[Architecture](docs/ARCHITECTURE.md)** - Technical design and architecture
- **[Development Guide](docs/DEVELOPMENT.md)** - Contributing and development setup

## ✨ Key Features

- 🔍 **Smart Code Scanning** - Automatically detect environment variable usage across your codebase
- 🔄 **Safe Environment Switching** - Switch between development, staging, and production with confidence
- ✅ **Pre-deployment Validation** - Catch missing or misconfigured variables before they cause issues
- 🔒 **Security First** - Generate cryptographically secure secrets and validate sensitive patterns
- 📝 **Auto Documentation** - Generate `.env.example` files from actual code usage
- 🚀 **Zero Dependencies** - Single static binary, works offline
- ⚡ **Blazingly Fast** - Parallel processing for large codebases
- 🌍 **Cross-platform** - Windows, macOS, and Linux support

## 🔧 Core Commands

| Command | Description | Example |
|---------|-------------|---------|
| `env init` | Initialize project with env structure | `env init` |
| `env switch` | Switch between environments | `env switch production` |
| `env scan` | Scan code for env variable usage | `env scan --format json` |
| `env validate` | Validate environment configuration | `env validate --env production` |
| `env sync` | Safely sync variables between environments | `env sync dev staging` |
| `env generate` | Generate .env.example from code | `env generate --comments` |
| `env status` | Show current environment status | `env status --verbose` |
| `env completion` | Generate shell completions | `env completion bash` |

Run `env --help` or `env <command> --help` for detailed usage information.

## 📖 Use Cases

### Development Teams
- **Onboarding**: New developers get correct environment setup instantly
- **Consistency**: Everyone uses the same environment variable names
- **Documentation**: Auto-generated .env.example stays up-to-date

### DevOps & CI/CD
- **Pre-deployment Checks**: Validate environments before deploying
- **Environment Promotion**: Safely sync variables from dev → staging → prod
- **Security Audits**: Detect weak secrets and missing sensitive variables

### Full-stack Projects
- **Multi-service**: Manage env vars across frontend, backend, and services
- **Language Agnostic**: Works with Node.js, Python, Go, Rust, PHP, etc.
- **Framework Integration**: Compatible with Next.js, Django, Rails, Laravel, etc.

## 🏗️ Technical Architecture

Built with **Rust** for maximum performance and reliability:

- ⚡ **Single static binary** - No runtime dependencies, ~5MB executable
- 🔒 **Memory safe** - Rust's ownership system prevents common bugs
- 🌍 **Cross-platform** - Native support for Windows, macOS, Linux
- 💾 **Local-first** - Works completely offline, no telemetry
- 🚀 **Parallel processing** - Multi-threaded scanning for large codebases
- 📦 **Modern tooling** - Uses clap, tokio, serde, and other battle-tested crates

## 🤝 Contributing

We welcome contributions of all kinds! Whether you're:

- 🐛 Reporting bugs
- 💡 Suggesting features
- 📝 Improving documentation
- 🔧 Submitting code changes

Please see our [Contributing Guide](CONTRIBUTING.md) for:

- Development environment setup
- Code style and conventions
- Testing requirements
- Pull request process

### Quick Development Setup

```bash
git clone https://github.com/env-cli/env-cli.git
cd env-cli
cargo build
cargo test
```

## 🛣️ Roadmap

### Current (v0.1.x)
- ✅ Core CLI commands
- ✅ Multi-language code scanning
- ✅ Environment validation
- ✅ Shell completions

### Upcoming (v0.2.x)
- 🔄 Plugin system for custom validators
- 🔄 IDE integrations (VSCode, IntelliJ)
- 🔄 Cloud secret manager integration (AWS Secrets Manager, HashiCorp Vault)
- 🔄 Environment variable encryption

### Future (v0.3.x+)
- Advanced diff and merge capabilities
- Team collaboration features
- Environment variable usage analytics
- Integration with more frameworks

See [GitHub Issues](https://github.com/env-cli/env-cli/issues) for detailed planning.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🔗 Links

- **📚 Documentation**: [Complete docs](docs/)
- **🐛 Issue Tracker**: [Report bugs](https://github.com/env-cli/env-cli/issues)
- **💬 Discussions**: [Community forum](https://github.com/env-cli/env-cli/discussions)
- **📦 Crates.io**: [Published package](https://crates.io/crates/env-cli)
- **📖 docs.rs**: [API documentation](https://docs.rs/env-cli)

## 🙏 Acknowledgments

Inspired by excellent Rust CLI tools:
- [fnm](https://github.com/Schniz/fnm) - Fast Node Manager
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast grep alternative
- [bat](https://github.com/sharkdp/bat) - Cat with wings
- [exa](https://github.com/ogham/exa) - Modern ls replacement

---

**Made with ❤️ for developers who care about their environment variables**

**Star ⭐ this repo if you find it useful!**