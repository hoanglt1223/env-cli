//! Integration tests for env-cli
//!
//! This module contains comprehensive end-to-end tests for all CLI commands.
//! Tests use temporary directories to ensure isolation and proper cleanup.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper function to create a temporary project directory with sample files
fn create_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_dir = temp_dir.path();

    // Create sample project structure
    fs::create_dir_all(project_dir.join("src")).unwrap();
    fs::create_dir_all(project_dir.join("tests")).unwrap();
    fs::create_dir_all(project_dir.join("config")).unwrap();

    // Create sample source files with environment variables
    fs::write(
        project_dir.join("src/main.rs"),
        r#"
use std::env;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string());
    println!("Database: {}", database_url);

    // Additional environment variables
    let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
}
"#,
    ).unwrap();

    fs::write(
        project_dir.join("src/lib.rs"),
        r#"
use std::env;

pub fn get_app_config() -> AppConfig {
    AppConfig {
        app_name: env::var("APP_NAME").unwrap_or_else(|_| "MyApp".to_string()),
        app_secret: env::var("APP_SECRET").expect("APP_SECRET must be set"),
        log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
    }
}

pub struct AppConfig {
    pub app_name: String,
    pub app_secret: String,
    pub log_level: String,
}
"#,
    ).unwrap();

    fs::write(
        project_dir.join("tests/integration_tests.rs"),
        r#"
use std::env;

#[test]
fn test_with_test_env() {
    env::set_var("TEST_ENV", "test");
    assert_eq!(env::var("TEST_ENV").unwrap(), "test");
}
"#,
    ).unwrap();

    fs::write(
        project_dir.join("config/app.json"),
        r#"{
    "database": "${DATABASE_URL}",
    "apiKey": "${API_KEY}",
    "debug": "${DEBUG:false}",
    "port": "${PORT:8080}"
}"#,
    ).unwrap();

    // Create a JavaScript/TypeScript file to demonstrate cross-language support
    fs::write(
        project_dir.join("frontend/app.js"),
        r#"
const databaseUrl = process.env.DATABASE_URL;
const apiKey = process.env.API_KEY || 'default-api-key';
const nodeEnv = process.env.NODE_ENV || 'development';

module.exports = {
    databaseUrl,
    apiKey,
    nodeEnv,
    port: process.env.PORT || 3000
};
"#,
    ).unwrap();

    temp_dir
}

/// Helper function to create sample environment files
fn create_sample_env_files(project_dir: &Path) {
    fs::create_dir_all(project_dir.join(".env")).unwrap();

    // Development environment
    fs::write(
        project_dir.join(".env/development.env"),
        r#"# Development Environment Configuration
DATABASE_URL=postgresql://localhost/dev_db
API_KEY=dev_api_key_123
DEBUG=true
PORT=8080
APP_NAME=MyApp-Dev
APP_SECRET=dev_secret_key
LOG_LEVEL=debug
NODE_ENV=development
"#,
    ).unwrap();

    // Production environment
    fs::write(
        project_dir.join(".env/production.env"),
        r#"# Production Environment Configuration
DATABASE_URL=postgresql://prod-host/prod_db
API_KEY=prod_secure_api_key_456
DEBUG=false
PORT=80
APP_NAME=MyApp
APP_SECRET=prod_super_secret_key
LOG_LEVEL=info
NODE_ENV=production
"#,
    ).unwrap();

    // Test environment
    fs::write(
        project_dir.join(".env/test.env"),
        r#"# Test Environment Configuration
DATABASE_URL=postgresql://localhost/test_db
API_KEY=test_api_key_789
DEBUG=false
PORT=8080
APP_NAME=MyApp-Test
APP_SECRET=test_secret_key
LOG_LEVEL=error
NODE_ENV=test
"#,
    ).unwrap();
}

/// Helper function to create a configuration file
fn create_config_file(project_dir: &Path) {
    fs::write(
        project_dir.join(".env/config.toml"),
        r#"[project]
name = "test-project"
default_environment = "development"

[[environments]]
name = "development"
description = "Development environment"
file = ".env/development.env"

[[environments]]
name = "production"
description = "Production environment"
file = ".env/production.env"

[[environments]]
name = "test"
description = "Test environment"
file = ".env/test.env"

[scan]
include_dirs = ["src", "tests", "frontend"]
exclude_dirs = ["target", "node_modules", ".git"]
include_patterns = ["*.rs", "*.js", "*.ts", "*.json"]
exclude_patterns = ["*.min.js", "bundle.*"]

[validation]
required = ["DATABASE_URL", "API_KEY", "APP_SECRET"]

[validation.security]
sensitive_patterns = [".*KEY.*", ".*SECRET.*", ".*PASSWORD.*"]
min_secret_length = 16
"#,
    ).unwrap();
}

// ============================================================================
// CLI Basic Functionality Tests
// ============================================================================

#[test]
fn test_cli_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("env-cli"))
        .stdout(predicates::str::contains("environment variable management"))
        .stdout(predicates::str::contains("Usage: env"));

    Ok(())
}

#[test]
fn test_cli_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("0.1.0"));

    Ok(())
}

#[test]
fn test_cli_no_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.assert()
        .failure()  // Should fail when no command provided
        .stderr(predicates::str::contains("required")));

    Ok(())
}

// ============================================================================
// Init Command Tests
// ============================================================================

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("init");

    let assert = cmd.assert()
        .success()
        .stdout(predicates::str::contains("Initializing"))
        .stdout(predicates::str::contains("project"));

    // Verify that initialization creates expected files
    assert!(project_dir.path().join(".env").exists());
    assert!(project_dir.path().join(".env/config.toml").exists());

    Ok(())
}

#[test]
fn test_init_command_force() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();

    // Create initial .env directory
    fs::create_dir_all(project_dir.path().join(".env")).unwrap();

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("init").arg("--force");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("force"))
        .stdout(predicates::str::contains("reinitializing"));

    Ok(())
}

#[test]
fn test_init_command_already_initialized() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();

    // Pre-initialize the project
    fs::create_dir_all(project_dir.path().join(".env")).unwrap();
    fs::write(project_dir.path().join(".env/config.toml"), "# Existing config").unwrap();

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("init");

    // Should fail if already initialized without --force
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("already initialized"));

    Ok(())
}

// ============================================================================
// Scan Command Tests
// ============================================================================

#[test]
fn test_scan_command_basic() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("scan");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("DATABASE_URL"))
        .stdout(predicates::str::contains("API_KEY"))
        .stdout(predicates::str::contains("APP_SECRET"));

    Ok(())
}

#[test]
fn test_scan_command_with_path() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("scan").arg("src");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("DATABASE_URL"))
        .stdout(predicates::str::contains("APP_SECRET"));

    Ok(())
}

#[test]
fn test_scan_command_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("scan").args(["--format", "json"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("\"format\""))
        .stdout(predicates::str::contains("\"variables\""))
        .stdout(predicates::str::contains("DATABASE_URL"));

    Ok(())
}

#[test]
fn test_scan_command_yaml_output() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("scan").args(["--format", "yaml"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("format:"))
        .stdout(predicates::str::contains("variables:"))
        .stdout(predicates::str::contains("DATABASE_URL"));

    Ok(())
}

#[test]
fn test_scan_command_include_hidden() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();

    // Create a hidden file with environment variables
    fs::write(project_dir.path().join(".hidden_config"), "SECRET_KEY=hidden_value").unwrap();

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("scan").arg("--hidden");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("SECRET_KEY"));

    Ok(())
}

// ============================================================================
// Status Command Tests
// ============================================================================

#[test]
fn test_status_command() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("status");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("current"))
        .stdout(predicates::str::contains("environment"));

    Ok(())
}

#[test]
fn test_status_command_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("status").arg("--verbose");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("detailed"))
        .stdout(predicates::str::contains("environments"));

    Ok(())
}

// ============================================================================
// Generate Command Tests
// ============================================================================

#[test]
fn test_generate_command() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("generate");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("generating"))
        .stdout(predicates::str::contains(".env.example"));

    // Verify that the .env.example file was created
    assert!(project_dir.path().join(".env.example").exists());

    Ok(())
}

#[test]
fn test_generate_command_with_comments() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("generate").arg("--comments");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("comments"));

    // Check that the generated file contains comments
    let content = fs::read_to_string(project_dir.path().join(".env.example")).unwrap();
    assert!(content.contains("#") || content.contains("Description"));

    Ok(())
}

#[test]
fn test_generate_command_with_docs() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("generate").arg("--docs");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("documentation"));

    Ok(())
}

#[test]
fn test_generate_command_custom_output() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project_dir.path());
    cmd.arg("generate").args(["--output", "custom.env.example"]);

    cmd.assert()
        .success();

    // Verify that the custom output file was created
    assert!(project_dir.path().join("custom.env.example").exists());
    assert!(!project_dir.path().join(".env.example").exists());

    Ok(())
}

// ============================================================================
// Validate Command Tests
// ============================================================================

#[test]
fn test_validate_command() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("validate");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("validation"))
        .stdout(predicates::str::contains("valid"));

    Ok(())
}

#[test]
fn test_validate_command_specific_env() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("validate").args(["--env", "development"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("development"));

    Ok(())
}

#[test]
fn test_validate_command_check_unused() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("validate").arg("--check-unused");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("unused"));

    Ok(())
}

// ============================================================================
// Switch Command Tests
// ============================================================================

#[test]
fn test_switch_command() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("switch").args(["production", "--yes"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("switching"))
        .stdout(predicates::str::contains("production"));

    Ok(())
}

#[test]
fn test_switch_command_nonexistent_env() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("switch").args(["nonexistent", "--yes"]);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("not found"))
        .stderr(predicates::str::contains("nonexistent"));

    Ok(())
}

// ============================================================================
// Sync Command Tests
// ============================================================================

#[test]
fn test_sync_command() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("sync").args(["development", "test", "--yes"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("syncing"))
        .stdout(predicates::str::contains("development"))
        .stdout(predicates::str::contains("test"));

    Ok(())
}

#[test]
fn test_sync_command_invalid_source() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("sync").args(["nonexistent", "test", "--yes"]);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("not found"));

    Ok(())
}

// ============================================================================
// Completion Command Tests
// ============================================================================

#[test]
fn test_completion_command_bash() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("completion").arg("bash");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("completion"))
        .stdout(predicates::str::contains("bash"));

    Ok(())
}

#[test]
fn test_completion_command_zsh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("completion").arg("zsh");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("zsh"));

    Ok(())
}

#[test]
fn test_completion_command_fish() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("completion").arg("fish");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("fish"));

    Ok(())
}

#[test]
fn test_completion_command_powershell() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("completion").arg("powershell");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("powershell"));

    Ok(())
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("invalid-command");

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("unrecognized"));

    Ok(())
}

#[test]
fn test_scan_nonexistent_directory() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("scan").arg("/nonexistent/directory");

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("not found")
            .or(predicates::str::contains("No such file")));

    Ok(())
}

#[test]
fn test_generate_to_readonly_directory() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();

    // Try to generate to a path that doesn't exist and can't be created
    let readonly_path = project_dir.path().join("readonly");

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("generate").args(["--output", readonly_path.join("test.env.example").to_str().unwrap()]);

    // This might succeed or fail depending on the system, but we expect some reasonable behavior
    let assert = cmd.assert();

    // If the command fails, it should provide a helpful error message
    if !assert.get_output().status.success() {
        assert.stderr(predicates::str::contains("permission")
            .or(predicates::str::contains("access"))
            .or(predicates::str::contains("directory")));
    }

    Ok(())
}

// ============================================================================
// Workflow Integration Tests
// ============================================================================

#[test]
fn test_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();

    // 1. Initialize the project
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("init").arg("--force");
    cmd.assert().success();

    // 2. Create environment files manually (simulating user setup)
    create_sample_env_files(project_dir.path());
    create_config_file(project_dir.path());

    // 3. Scan for environment variables
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("scan").args(["--format", "json"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("DATABASE_URL"));

    // 4. Generate .env.example
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("generate").arg("--comments");
    cmd.assert().success();
    assert!(project_dir.path().join(".env.example").exists());

    // 5. Validate configuration
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("validate");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("valid"));

    // 6. Check status
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("status").arg("--verbose");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("current"));

    // 7. Switch environment
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("switch").args(["production", "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("production"));

    // 8. Sync environments
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("sync").args(["production", "test", "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("syncing"));

    Ok(())
}

// ============================================================================
// Performance Tests
// ============================================================================

#[test]
fn test_scan_large_project() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = create_test_project();

    // Create many files to simulate a large project
    for i in 0..50 {
        let file_content = format!(r#"
use std::env;

fn function_{}() {{
    let var = env::var("TEST_VAR_{}").unwrap_or_else(|_| "default".to_string());
    println!("{{}}", var);
}}
"#, i, i);

        fs::write(
            project_dir.path().join(format!("src/module_{}.rs", i)),
            file_content,
        ).unwrap();
    }

    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project_dir.path());
    cmd.arg("scan");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("TEST_VAR_0"))
        .stdout(predicates::str::contains("DATABASE_URL"));

    Ok(())
}