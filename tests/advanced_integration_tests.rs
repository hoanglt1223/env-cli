//! Advanced integration tests for env-cli enterprise and workspace features
//!
//! This module contains tests for advanced features like enterprise commands,
//! workspace management, and complex scenarios.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper function to create a team workspace setup
fn create_team_workspace() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let workspace_dir = temp_dir.path();

    // Create workspace structure
    fs::create_dir_all(workspace_dir.join("projects/frontend")).unwrap();
    fs::create_dir_all(workspace_dir.join("projects/backend")).unwrap();
    fs::create_dir_all(workspace_dir.join("projects/shared")).unwrap();
    fs::create_dir_all(workspace_dir.join("config")).unwrap();

    // Create workspace configuration
    fs::write(
        workspace_dir.join("workspace.toml"),
        r#"[workspace]
name = "team-workspace"
description = "Team workspace for environment variable management"
default_environment = "development"

[projects.frontend]
path = "projects/frontend"
type = "web"
environments = ["development", "staging", "production"]

[projects.backend]
path = "projects/backend"
type = "api"
environments = ["development", "staging", "production"]

[projects.shared]
path = "projects/shared"
type = "library"
environments = ["development", "test"]

[security]
encryption_enabled = true
audit_log = true
access_control = true

[environments.development]
description = "Development environment"
auto_refresh = true

[environments.staging]
description = "Staging environment"
auto_refresh = false

[environments.production]
description = "Production environment"
auto_refresh = false
approval_required = true
"#,
    ).unwrap();

    // Create sample project files
    fs::write(
        workspace_dir.join("projects/frontend/.env/development.env"),
        r#"# Frontend Development
REACT_APP_API_URL=http://localhost:3001
REACT_APP_ENV=development
REACT_APP_DEBUG=true
"#,
    ).unwrap();

    fs::write(
        workspace_dir.join("projects/backend/.env/development.env"),
        r#"# Backend Development
DATABASE_URL=postgresql://localhost:5432/dev_db
API_PORT=3001
DEBUG=true
CORS_ORIGIN=http://localhost:3000
"#,
    ).unwrap();

    temp_dir
}

/// Helper function to create enterprise setup with security features
fn create_enterprise_setup() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let enterprise_dir = temp_dir.path();

    // Create enterprise structure
    fs::create_dir_all(enterprise_dir.join("config")).unwrap();
    fs::create_dir_all(enterprise_dir.join("secrets")).unwrap();
    fs::create_dir_all(enterprise_dir.join("audit")).unwrap();

    // Create enterprise configuration
    fs::write(
        enterprise_dir.join("enterprise.toml"),
        r#"[enterprise]
organization = "acme-corp"
version = "1.0.0"

[security]
encryption_algorithm = "AES-256-GCM"
key_rotation_days = 90
min_password_length = 12
mfa_required = true

[audit]
log_level = "info"
retention_days = 365
real_time_monitoring = true

[compliance]
standards = ["SOC2", "ISO27001", "GDPR"]
auto_scan = true
scan_frequency = "daily"

[environments.shared]
description = "Shared enterprise environment"
access_level = "team"
audit_access = true

[environments.production]
description = "Production enterprise environment"
access_level = "restricted"
audit_access = true
approval_workflow = true

[integrations]
vault_provider = "hashicorp"
ldap_enabled = true
saml_sso = true
"#,
    ).unwrap();

    temp_dir
}

// ============================================================================
// Enterprise Command Tests
// ============================================================================

#[test]
fn test_enterprise_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "init"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("enterprise"))
        .stdout(predicates::str::contains("initializing"));

    Ok(())
}

#[test]
fn test_enterprise_status_command() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "status"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("enterprise"))
        .stdout(predicates::str::contains("status"));

    Ok(())
}

#[test]
fn test_enterprise_security_scan() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "security", "scan"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("security"))
        .stdout(predicates::str::contains("scan"));

    Ok(())
}

#[test]
fn test_enterprise_audit_logs() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "audit", "logs"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("audit"))
        .stdout(predicates::str::contains("logs"));

    Ok(())
}

#[test]
fn test_enterprise_user_management() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "users", "list"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("users"));

    Ok(())
}

#[test]
fn test_enterprise_encryption_status() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "encryption", "status"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("encryption"))
        .stdout(predicates::str::contains("status"));

    Ok(())
}

// ============================================================================
// Workspace Command Tests
// ============================================================================

#[test]
fn test_workspace_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("workspace"))
        .stdout(predicates::str::contains("initializing"));

    Ok(())
}

#[test]
fn test_workspace_status_command() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "status"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("workspace"))
        .stdout(predicates::str::contains("status"));

    Ok(())
}

#[test]
fn test_workspace_list_projects() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "projects", "list"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("projects"))
        .stdout(predicates::str::contains("frontend"))
        .stdout(predicates::str::contains("backend"));

    Ok(())
}

#[test]
fn test_workspace_sync_all() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "sync", "--all", "--yes"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("syncing"))
        .stdout(predicates::str::contains("workspace"));

    Ok(())
}

#[test]
fn test_workspace_add_project() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "add", "mobile", "projects/mobile", "--yes"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("adding"))
        .stdout(predicates::str::contains("mobile"));

    Ok(())
}

#[test]
fn test_workspace_remove_project() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "remove", "shared", "--yes"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("removing"))
        .stdout(predicates::str::contains("shared"));

    Ok(())
}

// ============================================================================
// Complex Integration Scenarios
// ============================================================================

#[test]
fn test_enterprise_workspace_integration() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let enterprise_dir = create_enterprise_setup();

    // Initialize enterprise
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "init"]);
    cmd.assert().success();

    // Initialize workspace
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Link workspace to enterprise
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "link", "--enterprise", enterprise_dir.path().to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("linking"))
        .stdout(predicates::str::contains("enterprise"));

    Ok(())
}

#[test]
fn test_multi_environment_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Set development environment for frontend
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "env", "set", "frontend", "development", "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("frontend"))
        .stdout(predicates::str::contains("development"));

    // Set staging environment for backend
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "env", "set", "backend", "staging", "--yes"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("backend"))
        .stdout(predicates::str::contains("staging"));

    // Sync all projects
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "sync", "--all", "--yes"]);
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_security_compliance_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let enterprise_dir = create_enterprise_setup();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "init"]);
    cmd.assert().success();

    // Run security scan
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "security", "scan", "--compliance"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("compliance"));

    // Generate security report
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "security", "report", "--format", "json"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("security"))
        .stdout(predicates::str::contains("report"));

    // Check audit logs
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(enterprise_dir.path());
    cmd.args(["enterprise", "audit", "recent", "--limit", "10"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("audit"));

    Ok(())
}

#[test]
fn test_team_collaboration_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Add team member
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "members", "add", "alice@example.com", "--role", "developer"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("alice@example.com"))
        .stdout(predicates::str::contains("developer"));

    // Set permissions
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "permissions", "set", "alice@example.com", "frontend", "read-write"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("permissions"))
        .stdout(predicates::str::contains("frontend"));

    // List members
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "members", "list"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("alice@example.com"));

    Ok(())
}

// ============================================================================
// Error Handling and Edge Cases
// ============================================================================

#[test]
fn test_workspace_invalid_project_path() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "add", "invalid", "/nonexistent/path", "--yes"]);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("not found")
            .or(predicates::str::contains("invalid path")));

    Ok(())
}

#[test]
fn test_enterprise_missing_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(temp_dir.path());
    cmd.args(["enterprise", "status"]);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("not initialized")
            .or(predicates::str::contains("configuration")));

    Ok(())
}

#[test]
fn test_workspace_duplicate_project() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Try to add duplicate project
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "add", "frontend", "projects/frontend", "--yes"]);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("already exists")
            .or(predicates::str::contains("duplicate")));

    Ok(())
}

#[test]
fn test_permission_denied_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Try to access production without permissions
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "env", "set", "frontend", "production"]);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("permission")
            .or(predicates::str::contains("access denied"))
            .or(predicates::str::contains("approval")));

    Ok(())
}

// ============================================================================
// Performance and Scalability Tests
// ============================================================================

#[test]
fn test_large_workspace_performance() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let workspace_dir = temp_dir.path();

    // Create a large workspace with many projects
    fs::create_dir_all(workspace_dir.join("config")).unwrap();

    for i in 0..20 {
        let project_path = format!("projects/project_{}", i);
        fs::create_dir_all(workspace_dir.join(&project_path).join(".env")).unwrap();

        // Create environment file for each project
        fs::write(
            workspace_dir.join(&project_path).join(".env/development.env"),
            format!(r#"# Project {} Development
PROJECT_ID={}
API_URL=http://localhost:300{}
DEBUG=true
"#, i, i, 8000 + i),
        ).unwrap();
    }

    // Initialize large workspace
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Test workspace status performance
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "status"]);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("workspace"));

    Ok(())
}

#[test]
fn test_concurrent_operations() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = create_team_workspace();
    let mut cmd = Command::cargo_bin("env")?;

    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "init"]);
    cmd.assert().success();

    // Add multiple projects in sequence
    for i in 0..5 {
        let project_name = format!("service_{}", i);
        let project_path = format!("services/{}", project_name);
        fs::create_dir_all(workspace_dir.join(&project_path)).unwrap();

        let mut cmd = Command::cargo_bin("env")?;
        cmd.current_dir(workspace_dir.path());
        cmd.args(["workspace", "add", &project_name, &project_path, "--yes"]);
        cmd.assert().success();
    }

    // Sync all projects
    let mut cmd = Command::cargo_bin("env")?;
    cmd.current_dir(workspace_dir.path());
    cmd.args(["workspace", "sync", "--all", "--yes"]);
    cmd.assert().success();

    Ok(())
}