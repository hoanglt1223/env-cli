.PHONY: help build test clean install lint format check docs release

# Default target
help:
	@echo "Available commands:"
	@echo "  build      - Build the project"
	@echo "  test       - Run tests"
	@echo "  test-all   - Run all checks (tests, lint, format)"
	@echo "  clean      - Clean build artifacts"
	@echo "  install    - Install the binary"
	@echo "  lint       - Run clippy lints"
	@echo "  format     - Format code"
	@echo "  check      - Check formatting and run lints"
	@echo "  docs       - Generate documentation"
	@echo "  release    - Build release binaries"

# Build the project
build:
	cargo build

# Build release binaries
build-release:
	cargo build --release

# Run tests
test:
	cargo nextest run

# Run tests with coverage
test-coverage:
	cargo llvm-cov --all-features --workspace --html

# Run all checks
test-all: test lint format check

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/doc
	rm -f lcov.info

# Install the binary
install: build-release
	cargo install --path .

# Run clippy lints
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
format:
	cargo fmt

# Check formatting without modifying files
check-format:
	cargo fmt --all --check

# Run security audit
audit:
	cargo deny check

# Run all quality checks
check: check-format lint audit

# Generate documentation
docs:
	cargo doc --all-features --no-deps

# Open documentation in browser
docs-open: docs
	cargo doc --open

# Run development checks (used in CI)
ci:
	cargo fmt --all --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo nextest run
	cargo deny check

# Build release binaries for all targets
release:
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target x86_64-pc-windows-msvc

# Install development dependencies
setup:
	rustup component add rustfmt clippy rust-src
	cargo install cargo-nextest
	cargo install cargo-llvm-cov
	cargo install cargo-deny

# Create a new release
release-patch:
	@echo "Creating patch release..."
	@cargo bump patch
	@git add Cargo.toml
	@git commit -m "chore: bump version to $$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version')"
	@cargo tag

# Install pre-commit hooks
pre-commit:
	@echo "Setting up pre-commit hooks..."
	@echo '#!/bin/sh' > .git/hooks/pre-commit
	@echo 'make check' >> .git/hooks/pre-commit
	@echo 'cargo test' >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hooks installed!"