# env-cli

[![Crates.io](https://img.shields.io/crates/v/env-cli.svg)](https://crates.io/crates/env-cli)
[![Documentation](https://docs.rs/env-cli/badge.svg)](https://docs.rs/env-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**The missing CLI for environment variable management** — A powerful, open-source tool that manages the entire lifecycle of environment variables for full-stack projects.

- **Create** env structures for multiple environments
- **Switch** between environments safely with one command
- **Scan** code to detect which env vars are actually used
- **Validate** env before running or deploying
- **Sync** env between environments securely

> ⚡ **env-cli isn't just another `.env` loader** — it's a complete environment management system for developers who care about DX, security, and reliability.

---

## 🎯 The Problem

Managing environment variables across development teams and deployment stages is a pain:

- ❌ `.env.example` gets out of sync with actual usage
- ❌ Developers copy `.env` files and accidentally commit secrets
- ❌ No validation → production crashes from missing env vars
- ❌ Switching between local/dev/prod requires manual file shuffling
- ❌ Dead/unused env vars accumulate over time
- ❌ Secrets are weak, inconsistent, or reused

## ✨ The Solution

**env-cli** treats environment variables as a **first-class, managed system** rather than loose files:

```bash
# Initialize your project with env structure
env init

# Switch to production environment
env switch prod

# Scan code to find what env vars you actually use
env scan

# Generate always-up-to-date .env.example
env example

# Validate before deployment
env validate --env prod
```

---

## 🚀 Quick Start

### Installation

```bash
# Install with cargo
cargo install env-cli

# Or download the binary from releases
curl -L https://github.com/your-org/env-cli/releases/latest/download/env-cli.tar.gz | tar xz
```

### Basic Usage

```bash
# Initialize in your project
env init

# This creates:
# ├── .env.example      # Generated from actual usage
# ├── .env.local        # Your local development env
# ├── .env.dev          # Development environment
# ├── .env.test         # Testing environment
# └── .env.prod         # Production template
```

---

## 🔧 Core Features

### Environment Management

```bash
# Create new environment
env switch <env-name>

# Switch active environment
env switch prod    # .env → .env.prod

# Take snapshots
env snapshot prod  # Save current state
```

### Code Intelligence

```bash
# Scan your codebase to find env usage
env scan
# ✅ Found 15 env variables in use
# ✅ Scanned 124 files (JS, TS, Python, Go, Rust, etc.)

# Generate clean .env.example (no secrets!)
env example
# Generated .env.example with 15 variables
# ❌ 3 variables excluded (secrets detected)
```

### Quality & DX

```bash
# Lint your .env files
env lint
# ⚠️  API_BASE_URL missing comment
# ⚠️  JWT_SECRET is too short (12 < 32 chars)

# Format and standardize
env format

# Add helpful comments
env comment
```

### Security & Validation

```bash
# Generate strong secrets
env secrets generate
# 🔐 JWT_SECRET: generated 64-char secure random string
# 🔐 SESSION_KEY: generated hex secret

# Validate before deployment
env validate --env prod
# ✅ All required variables present
# ❌ DATABASE_URL is missing
# ⚠️  REDIS_HOST uses dev value 'localhost'
```

### Multi-Environment Sync

```bash
# Sync between environments (safe defaults)
env sync dev -> test
# ✅ Synced 12 variables from dev to test
# 🔒 3 secrets excluded from sync

# Compare environments
env diff dev prod
# 🔍 API_URL differs: http://localhost:3000 → https://api.prod.com
```

### Full-Stack Integration

```bash
# Auto-detect local services
env detect services
# 🔍 Found Redis on localhost:6379
# 🔍 Found PostgreSQL on localhost:5432

# Link backend to frontend
env link backend frontend
# ✅ Generated frontend .env from backend config
```

---

## 📚 Command Reference

### Core Commands

| Command | Description | Example |
|---------|-------------|---------|
| `env init` | Initialize project structure | `env init` |
| `env switch <env>` | Switch active environment | `env switch prod` |
| `env scan` | Scan code for env usage | `env scan --exclude node_modules` |
| `env example` | Generate .env.example | `env example --force` |
| `env dead` | Find unused env vars | `env dead --remove` |

### Quality & DX

| Command | Description | Example |
|---------|-------------|---------|
| `env lint` | Lint env files | `env lint --strict` |
| `env format` | Format env files | `env format` |
| `env comment` | Add descriptive comments | `env comment --auto` |
| `env explain [KEY]` | Explain env variable | `env explain DATABASE_URL` |

### Security & Validation

| Command | Description | Example |
|---------|-------------|---------|
| `env secrets generate` | Generate strong secrets | `env secrets generate --type jwt` |
| `env validate [--env <name>]` | Validate environment | `env validate --env prod` |
| `env run <command>` | Run command with env validation | `env run npm start` |

### Multi-Environment

| Command | Description | Example |
|---------|-------------|---------|
| `env sync <from> -> <to>` | Sync environments | `env sync dev -> test --safe` |
| `env snapshot <env>` | Save environment state | `env snapshot prod --backup` |
| `env diff <env1> <env2>` | Compare environments | `env diff dev prod` |

### Integration

| Command | Description | Example |
|---------|-------------|---------|
| `env detect services` | Detect local services | `env detect services --auto` |
| `env link <src> <dst>` | Link project parts | `env link backend frontend` |
| `env doctor` | Health check & diagnostics | `env doctor --fix` |

---

## 🏗️ Architecture

**Built with Rust for performance and safety:**

- **Core Engine**: Rust-based scanning, parsing, validation
- **AST Analysis**: Deep code understanding for accurate detection
- **Regex Patterns**: Fast pattern matching for common env usage
- **Zero Dependencies**: Single static binary, works everywhere
- **Platform Agnostic**: Windows, macOS, Linux support

**Security First:**

- 🔒 Never expose secrets in generated files
- 🔒 Safe defaults for production environments
- 🔒 Local-first operation (no cloud dependency)
- 🔒 Deterministic output for CI/CD pipelines

---

## 🔒 Design Principles

1. **`.env` is active, `.env.<name>` are sources** — Clear separation of concerns
2. **Production protection by default** — Requires confirmation for prod changes
3. **Local-first operation** — Works offline, no SaaS dependency
4. **One command, one purpose** — Unix philosophy applied to env management
5. **Deterministic output** — Perfect for automation and CI/CD
6. **Optional integrations** — Secret manager plugins are opt-in

---

## 🔌 Integrations

**Secret Managers (Optional Plugins):**

```bash
# Install Infisical plugin
env plugin install infisical

# Pull secrets from remote
env sync infisical:prod -> .env.prod
```

**CI/CD Examples:**

```yaml
# GitHub Actions
- name: Validate environment
  run: env validate --env prod

# Dockerfile
RUN env validate --env prod && npm run build

# Pre-deployment hook
npm run build && env validate --env prod && npm run deploy
```

---

## 🛠️ Development

### Building from source

```bash
git clone https://github.com/your-org/env-cli.git
cd env-cli
cargo build --release
```

### Running tests

```bash
cargo test

# Integration tests
cargo test --test integration
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🤝 Community

- **GitHub Issues**: [Report bugs & request features](https://github.com/your-org/env-cli/issues)
- **Discussions**: [Community discussions & questions](https://github.com/your-org/env-cli/discussions)
- **Discord**: [Join our Discord server](https://discord.gg/env-cli)

---

**Made with ❤️ for developers who care about their environment variables**
