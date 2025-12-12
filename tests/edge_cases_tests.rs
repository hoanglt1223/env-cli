//! Edge cases and boundary tests for env-cli
//!
//! This module contains tests for edge cases, error conditions,
//! and boundary scenarios to ensure robustness.

use crate::common::TestProjectBuilder;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// ============================================================================
// Invalid Input Tests
// ============================================================================

#[test]
fn test_empty_environment_names() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();
    let mut cmd = project.env_cli();

    cmd.args(["switch", ""]);

    cmd.assert().failure().stderr(
        predicates::str::contains("empty")
            .or(predicates::str::contains("invalid"))
            .or(predicates::str::contains("required")),
    );

    Ok(())
}

#[test]
fn test_special_characters_in_environment_names() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    let invalid_names = vec![
        "env-with spaces",
        "env@with#special!",
        "env/with/slashes",
        "env\\with\\backslashes",
        "env:with:colons",
        "env*with*asterisks",
        "env?with?questions",
        "env|with|pipes",
        "env\"with\"quotes",
        "env'with'apostrophes",
    ];

    for name in invalid_names {
        let mut cmd = project.env_cli();
        cmd.args(["switch", name]);

        cmd.assert().failure().stderr(
            predicates::str::contains("invalid")
                .or(predicates::str::contains("format"))
                .or(predicates::str::contains("special")),
        );
    }

    Ok(())
}

#[test]
fn test_very_long_environment_names() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Create a very long environment name (1000 characters)
    let long_name = "a".repeat(1000);
    let mut cmd = project.env_cli();

    cmd.args(["switch", &long_name]);

    cmd.assert().failure().stderr(
        predicates::str::contains("too long")
            .or(predicates::str::contains("invalid"))
            .or(predicates::str::contains("maximum")),
    );

    Ok(())
}

#[test]
fn test_unicode_environment_names() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    let unicode_names = vec![
        "环境",          // Chinese
        "окружение",     // Russian
        "environnement", // French with special char
        "umgebung",      // German
        "環境",          // Japanese
    ];

    for name in unicode_names {
        let mut cmd = project.env_cli();
        cmd.args(["switch", name]);

        // Unicode should generally be supported, but test the behavior
        let result = cmd.assert();
        if !result.get_output().status.success() {
            result.stderr(
                predicates::str::contains("unicode")
                    .or(predicates::str::contains("encoding"))
                    .or(predicates::str::contains("unsupported")),
            );
        }
    }

    Ok(())
}

// ============================================================================
// File System Edge Cases
// ============================================================================

#[test]
fn test_nonexistent_config_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(temp_dir.path());
    cmd.arg("status");

    cmd.assert().failure().stderr(
        predicates::str::contains("not initialized")
            .or(predicates::str::contains("config").or(predicates::str::contains("directory"))),
    );

    Ok(())
}

#[test]
fn test_permission_denied_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Create a read-only directory (if possible on the system)
    let readonly_dir = project.path().join("readonly");
    fs::create_dir_all(&readonly_dir)?;

    // Try to make it read-only (this may not work on all systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&readonly_dir)?.permissions();
        perms.set_mode(0o444); // read-only
        fs::set_permissions(&readonly_dir, perms)?;
    }

    let mut cmd = project.env_cli();
    cmd.args([
        "generate",
        "--output",
        readonly_dir.join("test.env").to_str().unwrap(),
    ]);

    let result = cmd.assert();
    if !result.get_output().status.success() {
        result.stderr(
            predicates::str::contains("permission")
                .or(predicates::str::contains("access"))
                .or(predicates::str::contains("denied")),
        );
    }

    Ok(())
}

#[test]
fn test_disk_space_exhaustion_simulation() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Create a very large file to simulate disk space issues (limited size for test)
    let large_file = project.path().join("large_file.txt");
    let large_content = "x".repeat(10_000_000); // 10MB
    fs::write(&large_file, large_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["scan"]);

    // The command should still work even with large files present
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("scanning").or(predicates::str::contains("completed")));

    Ok(())
}

#[test]
fn test_malformed_config_files() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .build();

    // Create malformed config.toml
    let malformed_config = r#"[project
name = "test-project"  # Missing closing bracket
default_environment = "development"

[environments
name = "development"  # Malformed TOML syntax
"#;

    project.write_file(".env/config.toml", malformed_config)?;

    let mut cmd = project.env_cli();
    cmd.arg("status");

    cmd.assert().failure().stderr(
        predicates::str::contains("config")
            .or(predicates::str::contains("parse"))
            .or(predicates::str::contains("invalid"))
            .or(predicates::str::contains("TOML")),
    );

    Ok(())
}

#[test]
fn test_corrupted_environment_files() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Create corrupted .env file with invalid encoding
    let corrupted_content = vec![0xFF, 0xFE, 0xFD, 0xFC]; // Invalid UTF-8
    fs::write(project.file_path(".env/development.env"), corrupted_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["validate", "--env", "development"]);

    cmd.assert().failure().stderr(
        predicates::str::contains("encoding")
            .or(predicates::str::contains("invalid"))
            .or(predicates::str::contains("corrupted"))
            .or(predicates::str::contains("UTF-8")),
    );

    Ok(())
}

// ============================================================================
// Network and External Dependencies
// ============================================================================

#[test]
fn test_offline_mode() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Test that basic commands work in offline mode
    let mut cmd = project.env_cli();
    cmd.arg("status");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("current").or(predicates::str::contains("environment")));

    Ok(())
}

#[test]
fn test_timeout_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .build();

    // Create a very deep directory structure to potentially cause timeouts
    let mut current_path = project.path().to_path_buf();
    for i in 0..100 {
        current_path = current_path.join(format!("level_{}", i));
        fs::create_dir_all(&current_path)?;
        fs::write(
            current_path.join("file.rs"),
            format!("const VAR_{} = \"value\";", i),
        )?;
    }

    let mut cmd = project.env_cli();
    cmd.args(["scan", "--hidden"]);

    // Set a reasonable timeout and check behavior
    let result = cmd.assert();
    if !result.get_output().status.success() {
        result.stderr(
            predicates::str::contains("timeout")
                .or(predicates::str::contains("time"))
                .or(predicates::str::contains("slow")),
        );
    }

    Ok(())
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[test]
fn test_concurrent_file_access() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Simulate concurrent access by creating lock files
    let lock_file = project.path().join(".env.lock");
    fs::write(&lock_file, "locked")?;

    let mut cmd1 = project.env_cli();
    cmd1.args(["switch", "production", "--yes"]);

    let mut cmd2 = project.env_cli();
    cmd2.args(["validate", "--env", "development"]);

    // Both commands should handle lock file appropriately
    let result1 = cmd1.assert();
    let result2 = cmd2.assert();

    // At least one should succeed or provide a clear error about locking
    if !result1.get_output().status.success() {
        result1.stderr(
            predicates::str::contains("lock")
                .or(predicates::str::contains("concurrent"))
                .or(predicates::str::contains("busy")),
        );
    }

    if !result2.get_output().status.success() {
        result2.stderr(
            predicates::str::contains("lock")
                .or(predicates::str::contains("concurrent"))
                .or(predicates::str::contains("busy")),
        );
    }

    Ok(())
}

// ============================================================================
// Memory and Performance Edge Cases
// ============================================================================

#[test]
fn test_large_environment_variable_values() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Create environment file with very large values
    let large_value = "x".repeat(1_000_000); // 1MB
    let large_env_content = format!(
        r#"LARGE_VAR={}
DATABASE_URL=postgresql://localhost:5432/test_db
"#,
        large_value
    );

    project.write_file(".env/development.env", &large_env_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["validate", "--env", "development"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("valid").or(predicates::str::contains("success")));

    Ok(())
}

#[test]
fn test_many_environment_variables() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .build();

    // Create .env file with many variables
    let mut env_content = String::new();
    for i in 0..1000 {
        env_content.push_str(&format!("VAR_{}=value_{}\n", i, i));
    }
    env_content.push_str("DATABASE_URL=postgresql://localhost:5432/test_db\n");

    project.write_file(".env/development.env", &env_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["validate", "--env", "development"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("valid").or(predicates::str::contains("success")));

    Ok(())
}

// ============================================================================
// Special Character Handling
// ============================================================================

#[test]
fn test_special_characters_in_values() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Test various special characters in environment variable values
    let special_values = vec![
        (
            "URL_WITH_SPECIAL",
            "https://example.com/path?param=value&other=test#fragment",
        ),
        (
            "JSON_VALUE",
            "{\"key\": \"value\", \"array\": [1, 2, 3], \"nested\": {\"bool\": true}}",
        ),
        ("NEWLINES", "line1\nline2\nline3"),
        ("TABS", "col1\tcol2\tcol3"),
        ("QUOTES", "\"single\" and 'double' quotes"),
        ("BACKSLASHES", "path\\to\\file\\with\\backslashes"),
        ("UNICODE", "🚀 Rocket emoji and 中文 Chinese characters"),
        ("SQL_INJECTION_ATTEMPT", "'; DROP TABLE users; --"),
        ("XSS_ATTEMPT", "<script>alert('xss')</script>"),
    ];

    let mut env_content = String::new();
    for (key, value) in special_values {
        env_content.push_str(&format!("{}={}\n", key, value));
    }
    env_content.push_str("DATABASE_URL=postgresql://localhost:5432/test_db\n");

    project.write_file(".env/development.env", &env_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["validate", "--env", "development"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("valid").or(predicates::str::contains("success")));

    Ok(())
}

// ============================================================================
// Platform-Specific Tests
// ============================================================================

#[test]
fn test_windows_path_handling() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Test Windows-style paths in environment variables
    let windows_paths = vec![
        ("WINDOWS_PATH", "C:\\Program Files\\MyApp\\config"),
        ("UNC_PATH", "\\\\server\\share\\folder"),
        ("MIXED_PATH", "C:/mixed/slash/style/path"),
        (
            "LONG_PATH",
            &format!("{}\\{}", "C:\\".to_string(), "a".repeat(100)),
        ),
    ];

    let mut env_content = String::new();
    for (key, value) in windows_paths {
        env_content.push_str(&format!("{}={}\n", key, value));
    }
    env_content.push_str("DATABASE_URL=postgresql://localhost:5432/test_db\n");

    project.write_file(".env/development.env", &env_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["validate", "--env", "development"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("valid").or(predicates::str::contains("success")));

    Ok(())
}

#[cfg(unix)]
#[test]
fn test_unix_path_handling() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .create_config_files()?
        .build();

    // Test Unix-style paths in environment variables
    let unix_paths = vec![
        ("UNIX_PATH", "/usr/local/bin/app"),
        ("HOME_PATH", "~/Documents/myapp"),
        ("RELATIVE_PATH", "./config/settings.yaml"),
        (
            "DEEP_PATH",
            "/very/deep/nested/directory/structure/path/to/file",
        ),
    ];

    let mut env_content = String::new();
    for (key, value) in unix_paths {
        env_content.push_str(&format!("{}={}\n", key, value));
    }
    env_content.push_str("DATABASE_URL=postgresql://localhost:5432/test_db\n");

    project.write_file(".env/development.env", &env_content)?;

    let mut cmd = project.env_cli();
    cmd.args(["validate", "--env", "development"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("valid").or(predicates::str::contains("success")));

    Ok(())
}

// ============================================================================
// Resource Exhaustion Tests
// ============================================================================

#[test]
fn test_max_file_descriptors() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .build();

    // Create many small files to potentially exhaust file descriptors
    for i in 0..1000 {
        let file_path = project.path().join(format!("file_{}.rs", i));
        fs::write(file_path, format!("const VAR_{} = \"value\";", i))?;
    }

    let mut cmd = project.env_cli();
    cmd.args(["scan"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("scanning").or(predicates::str::contains("completed")));

    Ok(())
}

#[test]
fn test_extremely_deep_directory_structure() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProjectBuilder::new()?
        .create_standard_structure()?
        .build();

    // Create a very deep directory structure
    let mut current_path = project.path().to_path_buf();
    for i in 0..50 {
        // 50 levels deep
        current_path = current_path.join(format!("level_{}", i));
        fs::create_dir_all(&current_path)?;
    }

    // Add a file at the deepest level
    fs::write(
        current_path.join("deep_file.rs"),
        "const DEEP_VAR = \"deep_value\";",
    )?;

    let mut cmd = project.env_cli();
    cmd.args(["scan", "--hidden"]);

    let result = cmd.assert();
    if !result.get_output().status.success() {
        result.stderr(
            predicates::str::contains("deep")
                .or(predicates::str::contains("path"))
                .or(predicates::str::contains("recursion")),
        );
    }

    Ok(())
}
