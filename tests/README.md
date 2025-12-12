# Integration Tests for env-cli

This directory contains comprehensive integration and end-to-end tests for the env-cli CLI tool.

## Test Structure

```
tests/
├── mod.rs                     # Test module declarations
├── integration_tests.rs       # Main integration tests for all commands
├── advanced_integration_tests.rs  # Enterprise and workspace feature tests
├── edge_cases_tests.rs        # Edge cases and boundary condition tests
├── common/
│   └── mod.rs                 # Common utilities and test helpers
└── README.md                  # This file
```

## Test Categories

### 1. Basic Integration Tests (`integration_tests.rs`)

These tests cover the core functionality of all CLI commands:

- **CLI Basic Functionality**: Help, version, error handling
- **Init Command**: Project initialization, force reinitialization
- **Scan Command**: Code scanning, output formats (text, JSON, YAML)
- **Status Command**: Environment status display, verbose output
- **Generate Command**: .env.example generation, with/without comments
- **Validate Command**: Environment validation, unused variable checking
- **Switch Command**: Environment switching, confirmation prompts
- **Sync Command**: Environment synchronization
- **Completion Command**: Shell completion generation
- **Error Handling**: Invalid commands, missing directories, permission issues
- **Workflow Integration**: Complete end-to-end workflows
- **Performance**: Large project scanning

### 2. Advanced Integration Tests (`advanced_integration_tests.rs`)

These tests cover enterprise and workspace features:

- **Enterprise Commands**: Init, status, security scanning, audit logs
- **Workspace Commands**: Init, project management, team collaboration
- **Complex Scenarios**: Enterprise-workspace integration, multi-environment workflows
- **Security & Compliance**: Security scans, compliance checking, audit trails
- **Team Collaboration**: User management, permissions, role-based access
- **Error Handling**: Invalid configurations, permission denials, duplicate resources
- **Performance**: Large workspaces, concurrent operations

### 3. Edge Cases Tests (`edge_cases_tests.rs`)

These tests cover edge cases and boundary conditions:

- **Invalid Input**: Empty names, special characters, very long names, Unicode
- **File System Edge Cases**: Nonexistent directories, permission denied, corrupted files
- **Network & External Dependencies**: Offline mode, timeout scenarios
- **Concurrent Access**: File locking, simultaneous operations
- **Memory & Performance**: Large values, many variables, resource exhaustion
- **Special Character Handling**: Paths, URLs, JSON, Unicode in values
- **Platform-Specific**: Windows vs Unix path handling
- **Resource Exhaustion**: Maximum file descriptors, deep directory structures

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Modules

```bash
# Basic integration tests
cargo test integration_tests

# Advanced integration tests
cargo test advanced_integration_tests

# Edge cases tests
cargo test edge_cases_tests
```

### Run Specific Test Cases

```bash
# Run a specific test
cargo test test_init_command

# Run tests matching a pattern
cargo test test_scan_command

# Run tests with specific output
cargo test -- --nocapture
```

### Run Tests in Parallel

```bash
# Use nextest for faster test execution
cargo nextest run

# Run tests with specific number of threads
cargo test -- --test-threads=4
```

## Test Utilities

### TestProjectBuilder

The `TestProjectBuilder` utility creates isolated test environments:

```rust
use crate::common::TestProjectBuilder;

let project = TestProjectBuilder::new()?
    .create_standard_structure()?
    .create_config_files()?
    .build();

// Use the project for testing
let mut cmd = project.env_cli();
cmd.arg("status");
cmd.assert().success();
```

### Environment Variable Generation

The `EnvVarGenerator` creates test environment variables:

```rust
use crate::common::EnvVarGenerator;

let common_vars = EnvVarGenerator::generate_common_vars();
let dev_vars = EnvVarGenerator::generate_env_specific_vars("development");
```

### Test Assertions

Helper functions for common assertions:

```rust
use crate::common::assertions;

// Check if output contains environment variables
assert!(assertions::assert_contains_env_vars(output, &["DATABASE_URL", "API_KEY"]));

// Check if output is valid JSON
assert!(assertions::assert_valid_json(json_output));

// Check if output is valid YAML
assert!(assertions::assert_valid_yaml(yaml_output));
```

## Test Data

### Sample Project Structure

Tests create a standardized project structure:

```
test_project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integration_tests.rs
├── config/
│   └── app.json
├── frontend/
│   ├── src/
│   │   ├── app.js
│   │   └── types.ts
│   └── package.json
├── .env/
│   ├── development.env
│   ├── production.env
│   └── test.env
└── .env/
    └── config.toml
```

### Environment Variables

Tests use a consistent set of environment variables:

- **Database**: `DATABASE_URL`, `DB_POOL_SIZE`
- **Application**: `APP_NAME`, `APP_SECRET`, `API_PORT`, `LOG_LEVEL`
- **Environment**: `NODE_ENV`, `DEBUG`, `PORT`
- **Security**: `JWT_SECRET`, `TOKEN_EXPIRY`
- **Features**: `CORS_ORIGIN`, `HOT_RELOAD`

## Best Practices

### Test Isolation

- Each test uses temporary directories for isolation
- Tests clean up automatically via `TempDir`
- No tests rely on external state or files

### Comprehensive Coverage

- Test both success and failure scenarios
- Include edge cases and boundary conditions
- Test with different input formats and values
- Verify output content and format

### Performance Considerations

- Use realistic but manageable data sizes
- Include performance tests for large-scale scenarios
- Test memory usage and resource management

### Platform Compatibility

- Test on Windows, macOS, and Linux when possible
- Handle platform-specific differences (paths, permissions)
- Test Unicode and special character handling

## Debugging Failed Tests

### Enable Debug Output

```bash
# Show test output
cargo test -- --nocapture

# Show command output
cargo test -- --exact test_name --nocapture
```

### Inspect Test Environment

```bash
# Run test and keep temporary directory
RUST_TEST_KEEP_TEMP=1 cargo test test_name
```

### Manual Testing

After a test fails, you can manually reproduce the issue:

```rust
// In test, before assertion:
eprintln!("Project directory: {}", project.path().display());
eprintln!("Command: {:?}", cmd.get_args());

// After assertion failure:
let output = cmd.get_output();
eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
```

## Continuous Integration

These tests are designed to run in CI/CD environments:

- No external dependencies required
- Tests complete quickly
- Clear pass/fail criteria
- Comprehensive coverage for confidence

### CI Test Matrix

Consider testing on:

- **Operating Systems**: Windows, macOS, Linux
- **Rust Versions**: Stable, Beta, MSRV
- **Architecture**: x86_64, ARM64 (if supported)

## Contributing

When adding new tests:

1. **Use TestProjectBuilder** for creating test environments
2. **Test both success and failure cases**
3. **Include edge cases** where relevant
4. **Document complex scenarios** with comments
5. **Follow existing naming conventions**
6. **Add appropriate assertions** for validation

### Test Naming

- Use descriptive names: `test_command_specific_scenario`
- Group related tests together
- Include the command name in test names
- Use snake_case for test function names

### Test Organization

- Add basic command tests to `integration_tests.rs`
- Add enterprise/workspace tests to `advanced_integration_tests.rs`
- Add edge cases to `edge_cases_tests.rs`
- Add reusable utilities to `common/mod.rs`