//! Common utilities and helpers for integration tests

use assert_cmd::Command as AssertCommand;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Test configuration structure
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub project_name: String,
    pub environments: Vec<String>,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            project_name: "test-project".to_string(),
            environments: vec![
                "development".to_string(),
                "staging".to_string(),
                "production".to_string(),
            ],
            include_patterns: vec!["*.rs".to_string(), "*.js".to_string()],
            exclude_patterns: vec!["*.min.js".to_string()],
        }
    }
}

/// A comprehensive test project builder
pub struct TestProjectBuilder {
    temp_dir: TempDir,
    config: TestConfig,
}

impl TestProjectBuilder {
    /// Create a new test project builder
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        Ok(Self {
            temp_dir,
            config: TestConfig::default(),
        })
    }

    /// Set custom configuration
    pub fn with_config(mut self, config: TestConfig) -> Self {
        self.config = config;
        self
    }

    /// Create standard project structure
    pub fn create_standard_structure(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let project_dir = self.temp_dir.path();

        // Create basic directories
        fs::create_dir_all(project_dir.join("src"))?;
        fs::create_dir_all(project_dir.join("tests"))?;
        fs::create_dir_all(project_dir.join("docs"))?;
        fs::create_dir_all(project_dir.join("scripts"))?;

        // Create main source file
        fs::write(
            project_dir.join("src/main.rs"),
            r#"//! Main application entry point

use std::env;

fn main() {
    println!("Hello, {}!", get_app_name());

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string());
    if debug == "true" {
        println!("Debug mode enabled");
    }
}

fn get_app_name() -> String {
    env::var("APP_NAME").unwrap_or_else(|_| "Unknown App".to_string())
}

fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
"#,
        )?;

        // Create library file with more environment variables
        fs::write(
            project_dir.join("src/lib.rs"),
            r#"//! Library module

use std::env;

pub struct AppConfig {
    pub app_name: String,
    pub app_secret: String,
    pub api_port: u16,
    pub log_level: String,
    pub environment: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "MyApp".to_string()),
            app_secret: env::var("APP_SECRET")
                .expect("APP_SECRET must be set"),
            api_port: env::var("API_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            environment: env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string()),
        })
    }
}

pub mod database {
    use std::env;

    pub fn get_connection_string() -> String {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }

    pub fn get_pool_size() -> u32 {
        env::var("DB_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10)
    }
}

pub mod auth {
    use std::env;

    pub fn get_jwt_secret() -> String {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    }

    pub fn get_token_expiry() -> u64 {
        env::var("TOKEN_EXPIRY")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap_or(3600)
    }
}
"#,
        )?;

        // Create test file
        fs::write(
            project_dir.join("tests/integration_tests.rs"),
            r#"#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_environment_setup() {
        env::set_var("TEST_MODE", "true");
        assert_eq!(env::var("TEST_MODE").unwrap(), "true");
    }

    #[test]
    fn test_database_connection() {
        // This would normally connect to a test database
        let db_url = env::var("TEST_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        assert!(!db_url.is_empty());
    }
}
"#,
        )?;

        Ok(self)
    }

    /// Create frontend files for testing cross-language support
    pub fn create_frontend_files(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let project_dir = self.temp_dir.path();

        fs::create_dir_all(project_dir.join("frontend/src"))?;
        fs::create_dir_all(project_dir.join("frontend/public"))?;

        // JavaScript files
        fs::write(
            project_dir.join("frontend/src/app.js"),
            r#"// Main frontend application
const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3001';
const APP_VERSION = process.env.REACT_APP_VERSION || '1.0.0';
const NODE_ENV = process.env.NODE_ENV || 'development';

class App {
    constructor() {
        this.apiUrl = API_BASE_URL;
        this.environment = NODE_ENV;
    }

    async fetchConfig() {
        const response = await fetch(`${this.apiUrl}/config`);
        return response.json();
    }

    isDevelopment() {
        return this.environment === 'development';
    }
}

module.exports = App;
"#,
        )?;

        // TypeScript file
        fs::write(
            project_dir.join("frontend/src/types.ts"),
            r#"// Type definitions
interface AppConfig {
    apiUrl: string;
    environment: string;
    debugMode: boolean;
    version: string;
}

class ConfigLoader {
    private static instance: ConfigLoader;

    public static getInstance(): ConfigLoader {
        if (!ConfigLoader.instance) {
            ConfigLoader.instance = new ConfigLoader();
        }
        return ConfigLoader.instance;
    }

    public loadConfig(): AppConfig {
        return {
            apiUrl: process.env.REACT_APP_API_URL || 'http://localhost:3001',
            environment: process.env.NODE_ENV || 'development',
            debugMode: process.env.REACT_APP_DEBUG === 'true',
            version: process.env.REACT_APP_VERSION || '1.0.0'
        };
    }
}

export { AppConfig, ConfigLoader };
"#,
        )?;

        // Package.json with environment variables
        fs::write(
            project_dir.join("frontend/package.json"),
            r#"{
    "name": "test-frontend",
    "version": "1.0.0",
    "scripts": {
        "start": "NODE_ENV=development react-scripts start",
        "build": "NODE_ENV=production react-scripts build",
        "test": "NODE_ENV=test jest"
    },
    "proxy": "${REACT_APP_API_URL}",
    "homepage": "${PUBLIC_URL}"
}"#,
        )?;

        Ok(self)
    }

    /// Create configuration files
    pub fn create_config_files(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let project_dir = self.temp_dir.path();

        // Create .env directory and files
        fs::create_dir_all(project_dir.join(".env"))?;

        for env in &self.config.environments {
            let env_file = format!(
                r#"# {} Environment Configuration

# Database Configuration
DATABASE_URL=postgresql://localhost:5432/{}_db
DB_POOL_SIZE=10

# Application Configuration
APP_NAME={}
APP_SECRET={}secret_key_{}
API_PORT=8080
LOG_LEVEL={}
NODE_ENV={}

# Security
JWT_SECRET=jwt_secret_key_{}
TOKEN_EXPIRY=3600

# Features
DEBUG={}
CORS_ORIGIN=http://localhost:3000
"#,
                env.capitalize(),
                env,
                self.config.project_name,
                env,
                env,
                match env.as_str() {
                    "development" => "debug",
                    "test" => "error",
                    _ => "info",
                },
                env,
                env,
                match env.as_str() {
                    "development" => "true",
                    "test" => "false",
                    _ => "false",
                }
            );

            fs::write(project_dir.join(format!(".env/{}.env", env)), env_file)?;
        }

        // Create config.toml
        let config_toml = format!(
            r#"[project]
name = "{}"
default_environment = "development"

"#,
            self.config.project_name
        );

        let mut environments_section = String::new();
        for env in &self.config.environments {
            environments_section.push_str(&format!(
                r#"[[environments]]
name = "{}"
description = "{} environment"
file = ".env/{}.env"

"#,
                env,
                env.capitalize(),
                env
            ));
        }

        let full_config = format!(
            "{}{}

[scan]
include_dirs = [\"src\", \"frontend/src\", \"tests\"]
exclude_dirs = [\"target\", \"node_modules\", \".git\"]
include_patterns = [{:?}]
exclude_patterns = [{:?}]

[validation]
required = [\"DATABASE_URL\", \"APP_SECRET\", \"JWT_SECRET\"]

[validation.security]
sensitive_patterns = [\".*KEY.*\", \".*SECRET.*\", \".*PASSWORD.*\"]
min_secret_length = 16
",
            config_toml,
            environments_section,
            self.config.include_patterns,
            self.config.exclude_patterns
        );

        fs::write(project_dir.join(".env/config.toml"), full_config)?;

        Ok(self)
    }

    /// Create CI/CD configuration files
    pub fn create_cicd_files(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let project_dir = self.temp_dir.path();

        fs::create_dir_all(project_dir.join(".github/workflows"))?;

        // GitHub Actions workflow
        fs::write(
            project_dir.join(".github/workflows/ci.yml"),
            r#"name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  NODE_ENV: test
  DATABASE_URL: postgresql://localhost:5432/test_db
  APP_NAME: ${{ github.event.repository.name }}

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:13
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: cargo test
      env:
        DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test_db

  frontend:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
    - name: Install dependencies
      working-directory: ./frontend
      run: npm ci
    - name: Run tests
      working-directory: ./frontend
      run: npm test
      env:
        NODE_ENV: test
        REACT_APP_API_URL: http://localhost:3001
"#,
        )?;

        Ok(self)
    }

    /// Build the test project
    pub fn build(self) -> Result<TestProject, Box<dyn std::error::Error>> {
        Ok(TestProject {
            temp_dir: self.temp_dir,
            config: self.config,
        })
    }
}

/// A test project instance
pub struct TestProject {
    temp_dir: TempDir,
    config: TestConfig,
}

impl TestProject {
    /// Get the project directory path
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Get a path to a file in the project
    pub fn file_path(&self, relative_path: impl AsRef<Path>) -> PathBuf {
        self.temp_dir.path().join(relative_path)
    }

    /// Check if a file exists in the project
    pub fn file_exists(&self, relative_path: impl AsRef<Path>) -> bool {
        self.file_path(relative_path).exists()
    }

    /// Read file content
    pub fn read_file(&self, relative_path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        fs::read_to_string(self.file_path(relative_path))
    }

    /// Write file content
    pub fn write_file(
        &self,
        relative_path: impl AsRef<Path>,
        content: &str,
    ) -> Result<(), std::io::Error> {
        if let Some(parent) = self.file_path(relative_path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(self.file_path(relative_path), content)
    }

    /// Get the project configuration
    pub fn config(&self) -> &TestConfig {
        &self.config
    }

    /// Create an env-cli command in this project directory
    pub fn env_cli(&self) -> AssertCommand {
        let mut cmd = AssertCommand::cargo_bin("env").expect("Failed to create env command");
        cmd.current_dir(self.path());
        cmd
    }

    /// Run an env-cli command and return the result
    pub fn run_env_command(
        &self,
        args: &[&str],
    ) -> Result<assert_cmd::assert::Assert, Box<dyn std::error::Error>> {
        let mut cmd = self.env_cli();
        for arg in args {
            cmd.arg(arg);
        }
        Ok(cmd.assert())
    }
}

/// Utility functions for test assertions
pub mod assertions {
    use predicates::prelude::*;

    /// Assert that command output contains environment variable
    pub fn assert_contains_env_var(output: &str, var_name: &str) -> bool {
        output.contains(var_name)
    }

    /// Assert that command output contains multiple environment variables
    pub fn assert_contains_env_vars(output: &str, var_names: &[&str]) -> bool {
        var_names.iter().all(|&name| output.contains(name))
    }

    /// Assert that command output is valid JSON
    pub fn assert_valid_json(output: &str) -> bool {
        serde_json::from_str::<serde_json::Value>(output).is_ok()
    }

    /// Assert that command output is valid YAML
    pub fn assert_valid_yaml(output: &str) -> bool {
        serde_yaml::from_str::<serde_yaml::Value>(output).is_ok()
    }
}

/// Environment variable test data generator
pub struct EnvVarGenerator;

impl EnvVarGenerator {
    /// Generate a set of common environment variables
    pub fn generate_common_vars() -> Vec<(&'static str, String)> {
        vec![
            (
                "DATABASE_URL",
                "postgresql://localhost:5432/test_db".to_string(),
            ),
            ("API_KEY", "test_api_key_12345".to_string()),
            ("APP_SECRET", "super_secret_key_16_chars".to_string()),
            ("DEBUG", "true".to_string()),
            ("PORT", "8080".to_string()),
            ("LOG_LEVEL", "info".to_string()),
            ("NODE_ENV", "development".to_string()),
            ("JWT_SECRET", "jwt_secret_key_32_chars_long".to_string()),
        ]
    }

    /// Generate environment-specific variables
    pub fn generate_env_specific_vars(env: &str) -> Vec<(&'static str, String)> {
        match env {
            "development" => vec![
                ("DEBUG", "true".to_string()),
                ("LOG_LEVEL", "debug".to_string()),
                ("HOT_RELOAD", "true".to_string()),
            ],
            "test" => vec![
                ("DEBUG", "false".to_string()),
                ("LOG_LEVEL", "error".to_string()),
                ("TEST_MODE", "true".to_string()),
            ],
            "production" => vec![
                ("DEBUG", "false".to_string()),
                ("LOG_LEVEL", "warn".to_string()),
                ("MINIMIZE", "true".to_string()),
            ],
            _ => vec![],
        }
    }
}
