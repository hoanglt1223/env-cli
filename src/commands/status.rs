//! Status command implementation.

use crate::config::{load_config, default_config_path};
use crate::env::EnvManager;
use crate::error::{EnvCliError, Result};
use crate::scan::CodeScanner;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Show current environment status.
pub async fn execute(verbose: bool) -> Result<()> {
    println!("Environment Status");

    // Check if we're in an env-cli project
    let env_dir = PathBuf::from(".env");
    if !env_dir.exists() {
        println!("  ✗ Not an env-cli project");
        println!("  Run 'env init' to initialize.");
        return Ok(());
    }

    println!("  ✓ Project initialized");

    // Load configuration
    let config_path = default_config_path();
    let config = load_config(&config_path)?;
    println!("  ✓ Configuration loaded");

    // Get current environment
    let current_env = match get_current_environment() {
        Ok(env) => env,
        Err(_) => {
            println!("  ⚠ No current environment set");
            println!("    Use 'env switch <environment>' to set one.");
            return Ok(());
        }
    };
    println!("  ✓ Current environment: {}", current_env);

    // Load environment variables
    let env_file = if let Some(env_config) = config.environments.iter()
        .find(|e| e.name == current_env) {
        if let Some(file) = &env_config.file {
            file.clone()
        } else {
            env_dir.join("environments").join(format!("{}.env", current_env))
        }
    } else {
        env_dir.join("environments").join(format!("{}.env", current_env))
    };

    let mut env_manager = EnvManager::new();
    let var_count = if env_file.exists() {
        env_manager.load_from_file(&env_file)?;
        env_manager.list().count()
    } else {
        0
    };

    println!("  ✓ Environment file: {} ({} variables)",
             env_file.file_name().unwrap_or_default().to_string_lossy(),
             var_count);

    // Show available environments
    println!("  Available environments: {}",
             config.environments.iter().map(|e| &e.name).collect::<Vec<_>>().join(", "));

    // Check for backups
    let backups_dir = env_dir.join("backups");
    let backup_count = if backups_dir.exists() {
        std::fs::read_dir(&backups_dir)?.count()
    } else {
        0
    };

    if backup_count > 0 {
        println!("  ✓ Backup files: {}", backup_count);
    }

    if verbose {
        print_detailed_status(&config, &current_env, &env_manager, &env_file).await?;
    } else {
        println!("\nRun with --verbose for detailed information.");
    }

    Ok(())
}

/// Print detailed status information.
async fn print_detailed_status(
    config: &crate::config::Config,
    current_env: &str,
    env_manager: &EnvManager,
    env_file: &PathBuf,
) -> Result<()> {
    println!("\nDetailed Information:");
    println!("  Project: {}", config.project);
    println!("  Configuration file: .env/config.toml");
    println!("  Environment file: {}", env_file.display());
    println!("  Default environment: {}", config.default_environment);
    println!("  Total environments: {}", config.environments.len());
    println!("  Current environment variables: {}", env_manager.list().count());

    // Show environment details
    if let Some(env_config) = config.environments.iter().find(|e| e.name == current_env) {
        if let Some(description) = &env_config.description {
            println!("  Environment description: {}", description);
        }
    }

    // Show file modification time
    if let Ok(metadata) = std::fs::metadata(env_file) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(datetime) = DateTime::from_timestamp(
                modified.duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                0
            ) {
                println!("  Last modified: {}", datetime.format("%Y-%m-%d %H:%M:%S UTC"));
            }
        }
    }

    // Variable status
    println!("\nVariable Status:");
    let mut variables: Vec<_> = env_manager.list().collect();
    variables.sort_by_key(|(k, _)| *k);

    if variables.is_empty() {
        println!("  No environment variables set");
    } else {
        // Show required variables first
        if !config.validation.required.is_empty() {
            for required in &config.validation.required {
                if let Some(value) = env_manager.get(required) {
                    let masked_value = mask_sensitive_value(required, value);
                    println!("  ✓ {} - {}", required, masked_value);
                } else {
                    println!("  ✗ {} - Missing (required)", required);
                }
            }
        }

        // Show other variables
        for (key, value) in variables {
            if !config.validation.required.contains(key) {
                let masked_value = mask_sensitive_value(key, value);
                println!("  ✓ {} - {}", key, masked_value);
            }
        }
    }

    // Security status
    println!("\nSecurity Status:");
    let mut security_issues = 0;
    let mut security_warnings = 0;

    for (key, value) in env_manager.list() {
        for pattern in &config.validation.security.sensitive_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(key) {
                    if is_insecure_value(value, &config.validation.security) {
                        security_issues += 1;
                    } else {
                        security_warnings += 1;
                    }
                }
            }
        }
    }

    if security_issues == 0 {
        println!("  ✓ No critical security issues");
    } else {
        println!("  ✗ {} critical security issues found", security_issues);
    }

    if security_warnings > 0 {
        println!("  ⚠ {} security warnings", security_warnings);
    }

    // Recent activity
    println!("\nRecent Activity:");
    show_recent_activity().await?;

    // Project structure
    println!("\nProject Structure:");
    let env_dir = PathBuf::from(".env");
    show_directory_tree(&env_dir, 2)?;

    Ok(())
}

/// Get the current environment name.
fn get_current_environment() -> Result<String> {
    let current_path = PathBuf::from(".env/.current");

    if !current_path.exists() {
        return Err(EnvCliError::Environment("No current environment set".to_string()));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs;
        let target = fs::read_link(&current_path)?;

        if let Some(file_name) = target.file_name().and_then(|n| n.to_str()) {
            if let Some(env_name) = file_name.strip_suffix(".env") {
                return Ok(env_name.to_string());
            }
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs;
        let target = fs::read_link(&current_path)?;

        if let Some(file_name) = target.file_name().and_then(|n| n.to_str()) {
            if let Some(env_name) = file_name.strip_suffix(".env") {
                return Ok(env_name.to_string());
            }
        }
    }

    Err(EnvCliError::Environment("Unable to determine current environment".to_string()))
}

/// Mask sensitive values for display.
fn mask_sensitive_value(key: &str, value: &str) -> String {
    let sensitive_keywords = ["password", "secret", "key", "token", "auth"];
    let key_lower = key.to_lowercase();

    if sensitive_keywords.iter().any(|&kw| key_lower.contains(kw)) {
        if value.len() <= 4 {
            "****".to_string()
        } else {
            format!("{}****", &value[..4])
        }
    } else {
        value.to_string()
    }
}

/// Check if a value is insecure.
fn is_insecure_value(value: &str, security_config: &crate::config::SecurityConfig) -> bool {
    let placeholders = [
        "change_me", "placeholder", "your_api_key", "your_secret",
        "change_this", "todo", "xxx", "111111", "123456",
    ];

    let lower_value = value.to_lowercase();
    for placeholder in &placeholders {
        if lower_value.contains(placeholder) {
            return true;
        }
    }

    if let Some(min_length) = security_config.min_secret_length {
        if value.len() < min_length {
            return true;
        }
    }

    false
}

/// Show recent activity logs.
async fn show_recent_activity() -> Result<()> {
    let env_dir = PathBuf::from(".env");
    let backups_dir = env_dir.join("backups");

    if !backups_dir.exists() {
        println!("  No recent activity found");
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&backups_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_name().to_string_lossy().ends_with(".env")
        })
        .collect();

    // Sort by modification time (newest first)
    entries.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
        let b_time = b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
        b_time.cmp(&a_time)
    });

    let mut activity_count = 0;
    for entry in entries.iter().take(5) {
        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                if let Ok(datetime) = DateTime::from_timestamp(
                    modified.duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                    0
                ) {
                    let filename = entry.file_name().to_string_lossy();
                    if let Some(env_name) = filename.strip_suffix(".env") {
                        if let Some(timestamp) = env_name.split('_').last() {
                            println!("  [{}] Environment backup created",
                                   datetime.format("%Y-%m-%d %H:%M"));
                            activity_count += 1;
                        }
                    }
                }
            }
        }
    }

    if activity_count == 0 {
        println!("  No recent activity found");
    }
}

/// Show directory tree structure.
fn show_directory_tree(dir: &PathBuf, indent: usize) -> Result<()> {
    let indent_str = " ".repeat(indent);

    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        for entry in entries {
            let name = entry.file_name().to_string_lossy();
            let path = entry.path();

            if path.is_dir() {
                println!("{}{}/", indent_str, name);
                if name != "." && name != ".." {
                    show_directory_tree(&path, indent + 2)?;
                }
            } else {
                println!("{}{}", indent_str, name);
            }
        }
    }

    Ok(())
}