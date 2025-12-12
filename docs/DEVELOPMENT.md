# env-cli - Development & Contributing Guide

## Development Environment Setup

### Prerequisites

- **Rust 1.70+** - Latest stable version recommended
- **Git** - For version control
- **Make** - For build automation (optional)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/your-org/env-cli.git
cd env-cli

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development dependencies
cargo install cargo-watch cargo-flamegraph cargo-audit

# Run initial build
cargo build

# Run tests
cargo test
```

### Development Workflow

```bash
# Watch for changes and run tests
cargo watch -x test

# Run with debug logging
RUST_LOG=debug cargo run -- scan

# Build with optimizations
cargo build --release

# Run integration tests
cargo test --test integration

# Check code formatting
cargo fmt --check

# Run clippy lints
cargo clippy -- -D warnings

# Audit dependencies
cargo audit
```

## Project Structure

```
env-cli/
├── src/                     # Source code
│   ├── main.rs             # CLI entry point
│   ├── cli/                # Command-line interface
│   ├── core/               # Core business logic
│   ├── parsers/            # Language parsers
│   ├── utils/              # Utility modules
│   ├── types/              # Type definitions
│   └── tests/              # Test modules
├── tests/                  # Integration tests
├── benches/                # Performance benchmarks
├── docs/                   # Documentation
├── scripts/                # Build and utility scripts
├── .github/                # GitHub workflows
├── Cargo.toml              # Dependencies and metadata
├── Makefile               # Build automation
└── README.md              # Project documentation
```

## Code Style & Guidelines

### Rust Code Style

We follow the official Rust style guide with these additional conventions:

```rust
// Use descriptive names
pub struct EnvironmentVariableScanner {
    // Prefer explicit types over inference for public APIs
    scan_patterns: Vec<Regex>,
    excluded_dirs: HashSet<PathBuf>,
}

// Document all public APIs
/// Scans a directory for environment variable usage
///
/// # Arguments
///
/// * `root_dir` - Root directory to scan
/// * `config` - Scanning configuration
///
/// # Returns
///
/// Returns a `ScanResult` containing all found variables
///
/// # Examples
///
/// ```
/// let result = scan_directory(Path::new("."), &config)?;
/// println!("Found {} variables", result.variables.len());
/// ```
pub fn scan_directory(
    root_dir: &Path,
    config: &ScanConfig,
) -> Result<ScanResult, ScanError> {
    // Implementation
}

// Use Result for error handling
pub type EnvResult<T> = Result<T, EnvError>;

#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Invalid environment format: {0}")]
    InvalidFormat(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Error Handling

Use `thiserror` for structured error handling:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Failed to read file: {path}")]
    FileReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid regex pattern: {pattern}")]
    InvalidRegex {
        pattern: String,
        #[source]
        source: regex::Error,
    },

    #[error("Parse error in {file}:{line}")]
    ParseError {
        file: PathBuf,
        line: usize,
        message: String,
    },
}
```

### Logging

Use the `tracing` crate for structured logging:

```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(self))]
pub fn scan_file(&self, path: &Path) -> Result<Vec<EnvUsage>, ScanError> {
    debug!("Starting scan of file: {:?}", path);

    let content = std::fs::read_to_string(path)
        .map_err(|e| ScanError::FileReadError {
            path: path.to_path_buf(),
            source: e
        })?;

    info!("Successfully read file, size: {} bytes", content.len());

    let usages = self.parse_content(&content)?;

    debug!("Found {} environment variable usages", usages.len());
    Ok(usages)
}
```

## Testing Strategy

### Unit Tests

Test individual components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_environment_variable_parsing() {
        let input = r#"
API_URL=https://api.example.com
DATABASE_URL=postgresql://user:pass@localhost/db
DEBUG=true
        "#;

        let env = Environment::from_str(input).unwrap();

        assert_eq!(env.variables.len(), 3);
        assert_eq!(env.variables["API_URL"].value, "https://api.example.com");
        assert_eq!(env.variables["API_URL"].var_type, VariableType::Url);
        assert_eq!(env.variables["DEBUG"].var_type, VariableType::Boolean);
    }

    #[test]
    fn test_secret_detection() {
        let secret_patterns = SecretDetector::default();

        assert!(secret_patterns.is_secret("JWT_SECRET"));
        assert!(secret_patterns.is_secret("DATABASE_PASSWORD"));
        assert!(secret_patterns.is_secret("API_KEY"));
        assert!(!secret_patterns.is_secret("API_URL"));
        assert!(!secret_patterns.is_secret("DEBUG_MODE"));
    }
}
```

### Integration Tests

Test end-to-end workflows:

```rust
// tests/integration/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_env_init_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("env").unwrap();

    cmd.current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // Verify files were created
    assert!(temp_dir.path().join(".env.example").exists());
    assert!(temp_dir.path().join(".env.local").exists());
    assert!(temp_dir.path().join(".env.dev").exists());
    assert!(temp_dir.path().join(".env.test").exists());
}

#[test]
fn test_env_scan_with_project() {
    let project_dir = PathBuf::from("tests/fixtures/sample_project");
    let mut cmd = Command::cargo_bin("env").unwrap();

    cmd.current_dir(&project_dir)
        .args(&["scan", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DATABASE_URL"))
        .stdout(predicate::str::contains("API_KEY"));
}
```

### Performance Benchmarks

Use Criterion for performance testing:

```rust
// benches/scanning_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use env_cli::core::scanner::Scanner;

fn bench_scan_small_project(c: &mut Criterion) {
    c.bench_function("scan_small_project", |b| {
        b.iter(|| {
            let scanner = Scanner::default();
            let result = scanner.scan_directory(black_box(Path::new("tests/fixtures/small_project")));
            black_box(result)
        })
    });
}

fn bench_scan_large_project(c: &mut Criterion) {
    c.bench_function("scan_large_project", |b| {
        b.iter(|| {
            let scanner = Scanner::default();
            let result = scanner.scan_directory(black_box(Path::new("tests/fixtures/large_project")));
            black_box(result)
        })
    });
}

criterion_group!(benches, bench_scan_small_project, bench_scan_large_project);
criterion_main!(benches);
```

## Adding New Features

### 1. Adding a New Command

1. **Define the command in `cli/args.rs`:**

```rust
#[derive(Subcommand)]
pub enum Command {
    /// Existing commands...

    /// Generate environment documentation
    Docs {
        /// Output format [default: markdown]
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,

        /// Output file [default: stdout]
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
```

2. **Implement the command in `cli/commands/mod.rs`:**

```rust
pub mod docs;

// Use it in the command matcher
Command::Docs(args) => commands::docs::execute(args, &config)?,
```

3. **Create the implementation in `cli/commands/docs.rs`:**

```rust
use crate::cli::args::DocsCommand;
use crate::core::config::Config;
use anyhow::Result;

pub fn execute(args: DocsCommand, _config: &Config) -> Result<()> {
    let env = Environment::load_current()?;
    let docs = DocumentationGenerator::new();

    let content = match args.format.unwrap_or_default() {
        OutputFormat::Markdown => docs.generate_markdown(&env),
        OutputFormat::Json => docs.generate_json(&env),
        OutputFormat::Html => docs.generate_html(&env),
    };

    match args.output {
        Some(path) => std::fs::write(path, content)?,
        None => println!("{}", content),
    }

    Ok(())
}
```

### 2. Adding a New Language Parser

1. **Create the parser in `parsers/`:**

```rust
// parsers/php.rs
use crate::parsers::{LanguageParser, EnvUsage, ParseError};
use regex::Regex;
use std::path::Path;

pub struct PhpParser {
    patterns: Vec<Regex>,
}

impl PhpParser {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                Regex::new(r#"\$_ENV\['([^']+)'\]"#).unwrap(),
                Regex::new(r#"getenv\('([^']+)'\)"#).unwrap(),
                Regex::new(r#"\$([A-Z_][A-Z0-9_]*)\s*="#).unwrap(),
            ],
        }
    }
}

impl LanguageParser for PhpParser {
    fn detect(&self, file_path: &Path) -> bool {
        matches!(file_path.extension().and_then(|s| s.to_str()), Some("php"))
    }

    fn parse(&self, content: &str, file_path: &Path) -> Result<Vec<EnvUsage>, ParseError> {
        let mut usages = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                for cap in pattern.captures_iter(line) {
                    if let Some(var_match) = cap.get(1) {
                        usages.push(EnvUsage {
                            variable: var_match.as_str().to_string(),
                            line: line_num + 1,
                            context: line.to_string(),
                            file_path: file_path.to_path_buf(),
                        });
                    }
                }
            }
        }

        Ok(usages)
    }
}
```

2. **Register it in `parsers/mod.rs`:**

```rust
pub mod php;

// In the ParserFactory
pub struct ParserFactory {
    parsers: Vec<Box<dyn LanguageParser>>,
}

impl ParserFactory {
    pub fn new() -> Self {
        Self {
            parsers: vec![
                Box::new(JavaScriptParser::new()),
                Box::new(PythonParser::new()),
                Box::new(PhpParser::new()),
                // ... other parsers
            ],
        }
    }
}
```

### 3. Adding a New Validation Rule

```rust
// core/validation/rules.rs
use crate::core::validation::{ValidationRule, ValidationIssue, Environment};
use crate::types::SecretLevel;

pub struct RequiredVariablesRule {
    required_vars: HashSet<String>,
}

impl RequiredVariablesRule {
    pub fn new(required_vars: Vec<String>) -> Self {
        Self {
            required_vars: required_vars.into_iter().collect(),
        }
    }
}

impl ValidationRule for RequiredVariablesRule {
    fn validate(&self, env: &Environment) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for required_var in &self.required_vars {
            if !env.variables.contains_key(required_var) {
                issues.push(ValidationIssue {
                    level: ValidationLevel::Error,
                    variable: required_var.clone(),
                    message: format!("Required variable '{}' is missing", required_var),
                    suggestion: Some(format!("Add {} to your environment file", required_var)),
                });
            }
        }

        issues
    }

    fn name(&self) -> &str {
        "required_variables"
    }
}
```

## Release Process

### Pre-release Checklist

1. **Update version numbers:**
   ```bash
   # Update Cargo.toml
   cargo edit set-version 1.2.3

   # Update version in CLI help
   # Update version in documentation
   ```

2. **Run full test suite:**
   ```bash
   cargo test
   cargo test --release
   cargo test --test integration
   ```

3. **Run security audit:**
   ```bash
   cargo audit
   ```

4. **Check documentation:**
   ```bash
   cargo doc --no-deps
   # Check that all public APIs are documented
   ```

5. **Verify benchmarks:**
   ```bash
   cargo bench
   # Ensure performance hasn't regressed
   ```

6. **Update changelog:**
   ```markdown
   ## [1.2.3] - 2024-01-15

   ### Added
   - Support for PHP environment variable parsing
   - New `env docs` command for generating documentation

   ### Changed
   - Improved error messages for validation failures
   - Better performance for large codebase scanning

   ### Fixed
   - Fixed issue with Windows path handling
   - Corrected secret detection edge cases
   ```

### Building Release Binaries

```bash
# Install cross for cross-compilation
cargo install cross

# Build for all targets
make build-release

# Individual targets
cross build --target x86_64-unknown-linux-gnu --release
cross build --target x86_64-pc-windows-gnu --release
cross build --target x86_64-apple-darwin --release
cross build --target aarch64-apple-darwin --release
```

### Publishing

```bash
# Publish to crates.io
cargo publish

# Create GitHub release
gh release create 1.2.3 \
  --title "env-cli 1.2.3" \
  --notes-file CHANGELOG.md \
  target/x86_64-unknown-linux-gnu/release/env-cli \
  target/x86_64-pc-windows-gnu/release/env-cli.exe \
  target/x86_64-apple-darwin/release/env-cli \
  target/aarch64-apple-darwin/release/env-cli
```

## Performance Optimization

### Profiling

Use `cargo-flamegraph` for profiling:

```bash
# Install flamegraph tool
cargo install flamegraph

# Profile scanning performance
cargo flamegraph --bin env -- scan

# Profile with specific project
cargo flamegraph --bin env -- scan tests/fixtures/large_project
```

### Memory Optimization

Monitor memory usage:

```bash
# Use memory profiler
cargo install cargo-memory

cargo memory --bin env -- scan

# Check for memory leaks
valgrind --tool=memcheck target/release/env scan
```

### Benchmarking

Track performance over time:

```bash
# Run benchmarks
cargo bench

# Save benchmark results
cargo bench -- --save-baseline baseline

# Compare with previous
cargo bench -- --baseline baseline
```

## Contributing Guidelines

### Before Contributing

1. **Check existing issues** - Look for related issues or PRs
2. **Create an issue** - For significant changes, create an issue first
3. **Discuss the approach** - Get feedback on your proposed solution

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch:** `git checkout -b feature/amazing-feature`
3. **Make your changes:**
   - Write clean, idiomatic Rust code
   - Add tests for new functionality
   - Update documentation
   - Follow the code style guidelines
4. **Run the test suite:** `cargo test`
5. **Run formatting:** `cargo fmt`
6. **Run lints:** `cargo clippy`
7. **Commit your changes:** `git commit -m 'Add amazing feature'`
8. **Push to your branch:** `git push origin feature/amazing-feature`
9. **Create a Pull Request**

### Code Review Guidelines

When reviewing PRs:

1. **Functionality:** Does the code work as intended?
2. **Testing:** Are there adequate tests?
3. **Documentation:** Is the code well-documented?
4. **Performance:** Will this affect performance?
5. **Security:** Are there any security implications?
6. **Style:** Does it follow project conventions?

### Bug Reports

When reporting bugs:

1. **Use the bug report template**
2. **Provide minimal reproduction case**
3. **Include environment details:**
   - OS version
   - Rust version
   - env-cli version
4. **Add relevant logs or error messages**

## Community & Support

### Getting Help

- **GitHub Discussions:** For questions and general discussion
- **GitHub Issues:** For bug reports and feature requests
- **Discord:** Real-time chat with maintainers and users

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

### License

By contributing to env-cli, you agree that your contributions will be licensed under the MIT License.