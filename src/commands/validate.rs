//! Validate command implementation.

use crate::config::{load_config, default_config_path};
use crate::env::EnvManager;
use crate::error::{EnvCliError, Result};
use crate::scan;
use std::path::PathBuf;
use regex::Regex;

/// Validation result information.
#[derive(Debug)]
struct ValidationResult {
    required_passed: bool,
    format_errors: Vec<String>,
    security_warnings: Vec<String>,
    security_errors: Vec<String>,
    unused_variables: Vec<String>,
    total_variables: usize,
}

/// Validate environment configuration.
pub async fn execute(env: String, check_unused: bool) -> Result<()> {
    println!("Validating environment configuration for: {}", env);

    // Check if we're in an env-cli project
    let env_dir = PathBuf::from(".env");
    if !env_dir.exists() {
        return Err(EnvCliError::Config(
            "Not an env-cli project. Run 'env init' first.".to_string()
        ));
    }

    // Load configuration
    let config_path = default_config_path();
    let config = load_config(&config_path)?;

    // Determine which environment to validate
    let env_name = if env.is_empty() {
        get_current_environment()?
    } else {
        env.clone()
    };

    // Validate environment exists
    let target_env = config.environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| EnvCliError::Environment(format!(
            "Environment '{}' not found. Available environments: {}",
            env_name,
            config.environments.iter().map(|e| &e.name).collect::<Vec<_>>().join(", ")
        )))?;

    println!("Loading environment: {}", target_env.name);

    // Load environment variables
    let env_file = if let Some(file) = &target_env.file {
        file.clone()
    } else {
        env_dir.join("environments").join(format!("{}.env", env_name))
    };

    if !env_file.exists() {
        return Err(EnvCliError::FileSystem(format!(
            "Environment file not found: {}",
            env_file.display()
        )));
    }

    let mut env_manager = EnvManager::new();
    env_manager.load_from_file(&env_file)?;

    // Perform validation
    let mut result = ValidationResult {
        required_passed: true,
        format_errors: Vec::new(),
        security_warnings: Vec::new(),
        security_errors: Vec::new(),
        unused_variables: Vec::new(),
        total_variables: 0,
    };

    // Count total variables
    result.total_variables = env_manager.list().count();

    // Validate required variables
    if !config.validation.required.is_empty() {
        println!("Validating required variables...");
        let missing = env_manager.validate(&config.validation.required);
        if !missing.is_empty() {
            result.required_passed = false;
            for var in &missing {
                result.format_errors.push(format!("Missing required variable: {}", var));
            }
        }
    }

    // Validate variable formats
    if !config.validation.formats.is_empty() {
        println!("Checking variable formats...");
        for (var_name, pattern) in &config.validation.formats {
            if let Some(value) = env_manager.get(var_name) {
                if let Ok(regex) = Regex::new(pattern) {
                    if !regex.is_match(value) {
                        result.format_errors.push(format!(
                            "Variable '{}' has invalid format. Expected pattern: {}",
                            var_name, pattern
                        ));
                    }
                } else {
                    result.format_errors.push(format!(
                        "Invalid regex pattern for variable '{}': {}",
                        var_name, pattern
                    ));
                }
            }
        }
    }

    // Security validation
    println!("Verifying security constraints...");
    validate_security(&env_manager, &config.validation.security, &mut result)?;

    // Check for unused variables (if requested)
    if check_unused {
        println!("Scanning for unused variables...");
        match crate::scan::CodeScanner::new() {
            Ok(scanner) => {
                match scanner.scan_directory(&PathBuf::from(".")).await {
                    Ok(usage_info) => {
                        let used_vars: std::collections::HashSet<String> = usage_info
                            .into_iter()
                            .map(|usage| usage.name)
                            .collect();

                        for (var_name, _) in env_manager.list() {
                            if !used_vars.contains(var_name) {
                                result.unused_variables.push(var_name.clone());
                            }
                        }
                    }
                    Err(e) => {
                        result.security_warnings.push(format!(
                            "Could not scan for unused variables: {}",
                            e
                        ));
                    }
                }
            }
            Err(e) => {
                result.security_warnings.push(format!(
                    "Could not initialize code scanner: {}",
                    e
                ));
            }
        }
    }

    // Print results
    print_validation_results(&env_name, &result);

    // Return error if validation failed
    if !result.required_passed || !result.format_errors.is_empty() || !result.security_errors.is_empty() {
        return Err(EnvCliError::Validation("Environment validation failed".to_string()));
    }

    Ok(())
}

/// Get the current environment name.
fn get_current_environment() -> Result<String> {
    let current_path = PathBuf::from(".env/.current");

    if !current_path.exists() {
        return Err(EnvCliError::Environment("No current environment set. Use 'env switch <environment>' first.".to_string()));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs;
        let target = fs::read_link(&current_path)?;

        // Extract environment name from path like "environments/development.env"
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

        // Extract environment name from path like "environments\development.env"
        if let Some(file_name) = target.file_name().and_then(|n| n.to_str()) {
            if let Some(env_name) = file_name.strip_suffix(".env") {
                return Ok(env_name.to_string());
            }
        }
    }

    Err(EnvCliError::Environment("Unable to determine current environment".to_string()))
}

/// Validate security constraints.
fn validate_security(
    env_manager: &EnvManager,
    security_config: &crate::config::SecurityConfig,
    result: &mut ValidationResult,
) -> Result<()> {
    for (var_name, var_value) in env_manager.list() {
        // Check for sensitive patterns
        for pattern in &security_config.sensitive_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(var_name) {
                    // Check if value looks like a placeholder or is too short
                    if is_insecure_value(var_value, security_config) {
                        result.security_errors.push(format!(
                            "Sensitive variable '{}' has an insecure value",
                            var_name
                        ));
                    } else {
                        result.security_warnings.push(format!(
                            "Sensitive variable '{}' detected, ensure value is properly secured",
                            var_name
                        ));
                    }
                }
            }
        }

        // Additional security checks
        if var_name.to_lowercase().contains("password") ||
           var_name.to_lowercase().contains("secret") ||
           var_name.to_lowercase().contains("key") {
            if var_value.is_empty() {
                result.security_errors.push(format!(
                    "Secret variable '{}' cannot be empty",
                    var_name
                ));
            }
        }
    }

    Ok(())
}

/// Check if a value is insecure for a sensitive variable.
fn is_insecure_value(value: &str, security_config: &crate::config::SecurityConfig) -> bool {
    // Check for placeholder values
    let placeholders = [
        "change_me",
        "placeholder",
        "your_api_key",
        "your_secret",
        "change_this",
        "todo",
        "xxx",
        "111111",
        "123456",
    ];

    let lower_value = value.to_lowercase();
    for placeholder in &placeholders {
        if lower_value.contains(placeholder) {
            return true;
        }
    }

    // Check minimum length
    if let Some(min_length) = security_config.min_secret_length {
        if value.len() < min_length {
            return true;
        }
    }

    // Check for special characters (if required)
    if security_config.require_special_chars.unwrap_or(false) {
        let has_special = value.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        if !has_special {
            return true;
        }
    }

    false
}

/// Print validation results in a user-friendly format.
fn print_validation_results(env_name: &str, result: &ValidationResult) {
    println!("\nValidation Results for '{}':", env_name);
    println!("  Total variables: {}", result.total_variables);

    // Required variables
    if result.required_passed {
        println!("  ✓ Required variables: All present");
    } else {
        println!("  ✗ Required variables: Missing");
    }

    // Format validation
    if result.format_errors.is_empty() {
        println!("  ✓ Variable formats: All valid");
    } else {
        println!("  ✗ Variable formats: {} errors", result.format_errors.len());
        for error in &result.format_errors {
            println!("    - {}", error);
        }
    }

    // Security validation
    if result.security_errors.is_empty() && result.security_warnings.is_empty() {
        println!("  ✓ Security check: Passed");
    } else {
        if !result.security_errors.is_empty() {
            println!("  ✗ Security check: {} errors", result.security_errors.len());
            for error in &result.security_errors {
                println!("    - {}", error);
            }
        }

        if !result.security_warnings.is_empty() {
            println!("  ⚠ Security warnings: {} warnings", result.security_warnings.len());
            for warning in &result.security_warnings {
                println!("    - {}", warning);
            }
        }
    }

    // Unused variables
    if !result.unused_variables.is_empty() {
        println!("  ⚠ Unused variables: {} found", result.unused_variables.len());
        for var in &result.unused_variables {
            println!("    - {} (consider removing if not needed)", var);
        }
    }

    // Overall status
    let is_valid = result.required_passed &&
        result.format_errors.is_empty() &&
        result.security_errors.is_empty();

    if is_valid {
        println!("\n✓ Environment '{}' is valid!", env_name);
    } else {
        println!("\n✗ Environment '{}' validation failed!", env_name);
        println!("Fix the errors above and run 'env validate' again.");
    }
}