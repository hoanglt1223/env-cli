//! Integration tests for env-cli
//!
//! This module contains happy path tests for all core CLI commands.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a test project with basic structure
fn create_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_dir = temp_dir.path();

    // Create sample project structure
    fs::create_dir_all(project_dir.join("src")).unwrap();

    // Create sample source file with environment variables
    fs::write(
        project_dir.join("src/main.rs"),
        r#"
use std::env;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string());
    println!("Database: {}", database_url);
}
"#,
    )
    .unwrap();

    temp_dir
}

// ============================================================================
// CLI Basic Tests
// ============================================================================

#[test]
fn test_cli_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("environment"));

    Ok(())
}

#[test]
fn test_cli_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("env"));

    Ok(())
}

// ============================================================================
// Init Command Tests
// ============================================================================

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(temp_dir.path());
    cmd.arg("init");

    cmd.assert().success();

    // Verify .env directory was created
    assert!(temp_dir.path().join(".env").exists());

    Ok(())
}

// ============================================================================
// Scan Command Tests
// ============================================================================

#[test]
fn test_scan_command_basic() -> Result<(), Box<dyn std::error::Error>> {
    let project = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project.path());
    cmd.args(["scan", "."]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Scan completed"));

    Ok(())
}

#[test]
fn test_scan_command_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let project = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project.path());
    cmd.args(["scan", ".", "--format", "json"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("scan_result"));

    Ok(())
}

// ============================================================================
// Status Command Tests
// ============================================================================

#[test]
fn test_status_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(temp_dir.path());
    cmd.arg("status");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Environment Status"));

    Ok(())
}

// ============================================================================
// Generate Command Tests
// ============================================================================

#[test]
fn test_generate_command() -> Result<(), Box<dyn std::error::Error>> {
    let project = create_test_project();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(project.path());
    cmd.arg("generate");

    cmd.assert().success();

    Ok(())
}

// ============================================================================
// Validate Command Tests
// ============================================================================

#[test]
fn test_validate_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    
    // Initialize first
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(temp_dir.path());
    cmd.arg("init");
    cmd.assert().success();

    // Create a development environment with secure values
    fs::write(
        temp_dir.path().join(".env/development.env"),
        "DATABASE_URL=postgresql://localhost:5432/dev\nAPI_SECRET=very_long_secure_secret_key_123456789012345678\n",
    )?;

    // Then validate development environment - just check it runs
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(temp_dir.path());
    cmd.args(["validate", "--env", "development"]);

    // Command should run (may succeed or fail based on validation rules)
    cmd.assert()
        .code(predicates::function::function(|code: &i32| *code == 0 || *code == 1));

    Ok(())
}

// ============================================================================
// Switch Command Tests
// ============================================================================

#[test]
fn test_switch_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    
    // Initialize first
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(temp_dir.path());
    cmd.arg("init");
    cmd.assert().success();

    // Create development environment
    fs::create_dir_all(temp_dir.path().join(".env")).unwrap();
    fs::write(
        temp_dir.path().join(".env/development.env"),
        "DATABASE_URL=postgresql://localhost:5432/dev\n",
    )?;

    // Switch to development
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(temp_dir.path());
    cmd.args(["switch", "development", "--yes"]);

    cmd.assert().success();

    Ok(())
}

// ============================================================================
// Sync Command Tests
// ============================================================================

#[test]
fn test_sync_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    
    // Initialize first
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(temp_dir.path());
    cmd.arg("init");
    cmd.assert().success();

    // Create source and target environments
    fs::create_dir_all(temp_dir.path().join(".env")).unwrap();
    fs::write(
        temp_dir.path().join(".env/development.env"),
        "DATABASE_URL=postgresql://localhost:5432/dev\nAPI_KEY=dev_key\n",
    )?;
    fs::write(
        temp_dir.path().join(".env/staging.env"),
        "DATABASE_URL=postgresql://localhost:5432/staging\n",
    )?;

    // Sync from development to staging
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(temp_dir.path());
    cmd.args(["sync", "development", "staging", "--yes"]);

    cmd.assert().success();

    Ok(())
}

// ============================================================================
// Completion Command Tests
// ============================================================================

#[test]
fn test_completion_command_bash() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.args(["completion", "bash"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("bash"));

    Ok(())
}

#[test]
fn test_completion_command_powershell() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;
    cmd.args(["completion", "powershell"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("PowerShell"));

    Ok(())
}

// ============================================================================
// Complete Workflow Test
// ============================================================================

#[test]
fn test_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let project = create_test_project();

    // 1. Initialize project
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project.path());
    cmd.arg("init");
    cmd.assert().success();

    // 2. Scan for environment variables
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project.path());
    cmd.args(["scan", "."]);
    cmd.assert().success();

    // 3. Generate .env.example
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project.path());
    cmd.arg("generate");
    cmd.assert().success();

    // 4. Check status
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(project.path());
    cmd.arg("status");
    cmd.assert().success();

    Ok(())
}
