//! Switch command implementation.

use crate::config::{load_config, default_config_path};
use crate::env::EnvManager;
use crate::error::{EnvCliError, Result};
use std::path::PathBuf;
use chrono::Utc;

/// Switch to a different environment.
pub async fn execute(environment: String, yes: bool) -> Result<()> {
    println!("Switching to environment: {}", environment);

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

    // Validate target environment exists
    let target_env = config.environments.iter()
        .find(|e| e.name == environment)
        .ok_or_else(|| EnvCliError::Environment(format!(
            "Environment '{}' not found. Available environments: {}",
            environment,
            config.environments.iter().map(|e| &e.name).collect::<Vec<_>>().join(", ")
        )))?;

    // Show current environment if it exists
    if let Ok(current) = get_current_environment() {
        if current == environment {
            println!("Already using '{}' environment.", environment);
            return Ok(());
        }

        if !yes {
            println!("Current environment: {}", current);
            println!("Target environment: {}", target_env.name.clone());

            if let Some(desc) = &target_env.description {
                println!("Description: {}", desc);
            }

            println!("\nThis will create a backup of the current environment and switch to '{}'.", environment);
            print!("Continue? [y/N] ");
            use std::io::Write;
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if !input.trim().to_lowercase().starts_with('y') {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
    } else if !yes {
        println!("No current environment set. Will set '{}' as the active environment.", environment);
        if let Some(desc) = &target_env.description {
            println!("Description: {}", desc);
        }

        print!("Continue? [y/N] ");
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // Perform the switch
    perform_environment_switch(&environment, &config).await?;

    println!("\nSuccessfully switched to '{}' environment!", environment);
    println!("Run 'env status' to verify the switch.");

    Ok(())
}

/// Perform the actual environment switch.
async fn perform_environment_switch(environment: &str, config: &crate::config::Config) -> Result<()> {
    let env_dir = PathBuf::from(".env");

    // Create backup of current environment if it exists
    if let Ok(current_env) = get_current_environment() {
        create_backup(&current_env, &env_dir)?;
        println!("✓ Created backup of current environment");
    }

    // Validate target environment file exists
    let target_file = if let Some(file) = config.environments.iter()
        .find(|e| e.name == environment)
        .and_then(|e| e.file.as_ref()) {
        file.clone()
    } else {
        env_dir.join("environments").join(format!("{}.env", environment))
    };

    if !target_file.exists() {
        return Err(EnvCliError::FileSystem(format!(
            "Environment file not found: {}",
            target_file.display()
        )));
    }

    // Validate environment file can be parsed
    let mut env_manager = EnvManager::new();
    env_manager.load_from_file(&target_file)?;
    println!("✓ Validated environment configuration");

    // Check for required variables
    if !config.validation.required.is_empty() {
        let missing = env_manager.validate(&config.validation.required);
        if !missing.is_empty() {
            println!("Warning: Missing required environment variables: {}", missing.join(", "));
            print!("Continue anyway? [y/N] ");
            use std::io::Write;
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if !input.trim().to_lowercase().starts_with('y') {
                return Err(EnvCliError::Validation(
                    "Missing required environment variables".to_string()
                ));
            }
        }
    }

    // Update .current symlink
    update_current_symlink(&env_dir, environment)?;
    println!("✓ Updated current environment link");

    // Load environment variables into current process (optional)
    // Note: This only affects the current process, not the shell
    for (key, value) in env_manager.list() {
        std::env::set_var(key, value);
    }

    println!("✓ Switched to environment '{}'", environment);

    Ok(())
}

/// Get the current environment from the .current symlink.
fn get_current_environment() -> Result<String> {
    let current_path = PathBuf::from(".env/.current");

    if !current_path.exists() {
        return Err(EnvCliError::Environment("No current environment set".to_string()));
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

/// Create a backup of the current environment.
fn create_backup(env_name: &str, env_dir: &PathBuf) -> Result<()> {
    let current_path = env_dir.join(".current");
    let backups_dir = env_dir.join("backups");

    if !current_path.exists() {
        return Ok(()); // No current environment to backup
    }

    // Create backups directory if it doesn't exist
    std::fs::create_dir_all(&backups_dir)?;

    // Generate backup filename with timestamp
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("{}_{}.env", env_name, timestamp);
    let backup_path = backups_dir.join(backup_filename);

    // Copy current environment file to backup location
    #[cfg(unix)]
    {
        use std::os::unix::fs;
        let target_path = fs::read_link(&current_path)?;
        let absolute_target = env_dir.join(target_path);
        std::fs::copy(&absolute_target, &backup_path)?;
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs;
        let target_path = fs::read_link(&current_path)?;
        let absolute_target = env_dir.join(target_path);
        std::fs::copy(&absolute_target, &backup_path)?;
    }

    Ok(())
}

/// Update the .current symlink to point to the target environment.
fn update_current_symlink(env_dir: &PathBuf, environment: &str) -> Result<()> {
    let current_path = env_dir.join(".current");
    let target_path = PathBuf::from("environments").join(format!("{}.env", environment));

    // Remove existing .current if it exists
    if current_path.exists() {
        std::fs::remove_file(&current_path)?;
    }

    // Create new symlink
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&target_path, &current_path)?;
    }

    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(&target_path, &current_path)?;
    }

    Ok(())
}