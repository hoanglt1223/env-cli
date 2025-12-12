//! Configuration management.
//!
//! This module handles loading, parsing, and managing configuration files.

use crate::error::{EnvCliError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for env-cli.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project name
    pub project: String,
    /// Default environment
    pub default_environment: String,
    /// Available environments
    pub environments: Vec<Environment>,
    /// Scan configuration
    pub scan: ScanConfig,
    /// Validation rules
    pub validation: ValidationConfig,
}

/// Environment configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Environment name
    pub name: String,
    /// Environment description
    pub description: Option<String>,
    /// Configuration file path
    pub file: Option<PathBuf>,
    /// Environment variables
    pub variables: std::collections::HashMap<String, String>,
}

/// Scan configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Directories to include
    pub include_dirs: Vec<String>,
    /// Directories to exclude
    pub exclude_dirs: Vec<String>,
    /// File patterns to include
    pub include_patterns: Vec<String>,
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
}

/// Validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Required variables
    pub required: Vec<String>,
    /// Variable format rules
    pub formats: std::collections::HashMap<String, String>,
    /// Security rules
    pub security: SecurityConfig,
}

/// Security configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Sensitive variable patterns
    pub sensitive_patterns: Vec<String>,
    /// Minimum password length
    pub min_secret_length: Option<usize>,
    /// Require special characters in secrets
    pub require_special_chars: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: "env-cli project".to_string(),
            default_environment: "development".to_string(),
            environments: vec![],
            scan: ScanConfig::default(),
            validation: ValidationConfig::default(),
        }
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            include_dirs: vec!["src".to_string()],
            exclude_dirs: vec!["target".to_string(), "node_modules".to_string()],
            include_patterns: vec![
                "*.rs".to_string(),
                "*.js".to_string(),
                "*.ts".to_string(),
                "*.py".to_string(),
            ],
            exclude_patterns: vec!["*.min.js".to_string()],
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            required: vec![],
            formats: std::collections::HashMap::new(),
            security: SecurityConfig::default(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sensitive_patterns: vec![
                ".*KEY.*".to_string(),
                ".*SECRET.*".to_string(),
                ".*PASSWORD.*".to_string(),
                ".*TOKEN.*".to_string(),
            ],
            min_secret_length: Some(16),
            require_special_chars: Some(true),
        }
    }
}

/// Load configuration from file.
pub fn load_config(path: &PathBuf) -> Result<Config> {
    if !path.exists() {
        return Err(EnvCliError::Config(format!(
            "Configuration file not found: {}",
            path.display()
        )));
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Save configuration to file.
pub fn save_config(config: &Config, path: &PathBuf) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Get default configuration file path.
pub fn default_config_path() -> PathBuf {
    PathBuf::from(".env/config.toml")
}