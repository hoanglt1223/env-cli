# env-cli

[![Crates.io](https://img.shields.io/crates/v/env-cli.svg)](https://crates.io/crates/env-cli)
[![Documentation](https://docs.rs/env-cli/badge.svg)](https://docs.rs/env-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**The missing CLI for environment variable management** — A powerful, open-source tool that manages the entire lifecycle of environment variables for full-stack projects.

## 🚀 Quick Start

```bash
# Install
cargo install env-cli

# Initialize your project
env init

# Scan code to find env usage
env scan

# Switch environments
env switch prod

# Validate before deployment
env validate --env prod
```

## 📚 Documentation

- **[User Guide & Features](docs/README.md)** - Complete user documentation
- **[CLI Reference](docs/CLI_SPEC.md)** - Detailed command specifications
- **[Architecture](docs/ARCHITECTURE.md)** - Technical design and architecture
- **[Development Guide](docs/DEVELOPMENT.md)** - Contributing and development setup

## ✨ Key Features

- 🔍 **Code Intelligence** - Scan code to detect actual env usage
- 🔄 **Environment Switching** - Safe switching between local/dev/prod
- ✅ **Validation** - Validate env before running or deploying
- 🔒 **Security** - Generate strong secrets, protect production
- 📝 **Documentation** - Auto-generate .env.example from code
- 🔗 **Integration** - Works with existing tools and workflows

## 🔧 Core Commands

| Command | Description |
|---------|-------------|
| `env init` | Initialize project with env structure |
| `env switch <env>` | Switch between environments |
| `env scan` | Scan code for env usage |
| `env validate` | Validate environment configuration |
| `env sync` | Sync environments safely |

## 🏗️ Architecture

Built with **Rust** for performance and security:

- **Single static binary** - Zero runtime dependencies
- **Cross-platform** - Windows, macOS, Linux support
- **Local-first** - Works offline, no cloud dependency
- **Fast scanning** - Parallel processing for large codebases

## 🤝 Contributing

We welcome contributions! See the [Development Guide](docs/DEVELOPMENT.md) for details on:

- Setting up the development environment
- Code style and guidelines
- Testing and contribution process

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **GitHub Issues**: [Report bugs & request features](https://github.com/your-org/env-cli/issues)
- **Discussions**: [Community discussions](https://github.com/your-org/env-cli/discussions)

---

**Made with ❤️ for developers who care about their environment variables**