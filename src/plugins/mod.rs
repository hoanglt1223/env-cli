//! Plugin system architecture for EC-03 extensibility.
//!
//! This module provides a foundation for plugin-based extensibility
//! that allows users to add custom scanning and processing capabilities.

use crate::error::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Plugin trait for extensible functionality
pub trait Plugin {
    /// Get the plugin name
    fn name(&self) -> &str;

    /// Get the plugin version
    fn version(&self) -> &str;

    /// Get the plugin description
    fn description(&self) -> &str;

    /// Scan files with custom logic
    fn scan(&self, context: &ScanContext) -> Result<ScanResult>;

    /// Process environment variables with custom logic
    fn process_variables(&self, context: &ProcessContext) -> Result<ProcessResult>;

    /// Validate environment variables with custom rules
    fn validate(&self, context: &ValidationContext) -> Result<ValidationResult>;
}

/// Context provided to plugins during scanning
#[derive(Debug, Clone)]
pub struct ScanContext {
    pub file_path: PathBuf,
    pub content: String,
    pub language: Option<String>,
    pub scan_config: ScanConfig,
}

/// Scan configuration
#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub parallel: bool,
    pub security_scan: bool,
}

/// Result of plugin scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub variables: HashMap<String, VariableInfo>,
    pub issues: Vec<PluginIssue>,
    pub metadata: HashMap<String, String>,
}

/// Information about a discovered variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub usage_count: usize,
    pub locations: Vec<Location>,
    pub suggested_value: Option<String>,
    pub security_level: SecurityLevel,
}

/// Location of variable usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub context: String,
}

/// Security level for variables
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Issues found by plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    pub suggestion: Option<String>,
}

/// Issue severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Context for variable processing
#[derive(Debug, Clone)]
pub struct ProcessContext {
    pub variables: HashMap<String, String>,
    pub operation: ProcessOperation,
}

/// Processing operations
#[derive(Debug, Clone)]
pub enum ProcessOperation {
    Validate,
    Transform,
    Generate,
    Export,
}

/// Result of variable processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub variables: HashMap<String, String>,
    pub transformations: Vec<TransformOperation>,
    pub warnings: Vec<String>,
}

/// Transform operation applied to variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformOperation {
    pub operation: String,
    pub variable: String,
    pub old_value: String,
    pub new_value: String,
}

/// Context for validation
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub variables: HashMap<String, String>,
    pub validation_rules: Vec<ValidationRule>,
    pub environment: String,
}

/// Validation rule
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub name: String,
    pub pattern: String,
    pub required: bool,
    pub validator: Box<dyn ValidationFn>,
}

/// Trait for validation functions
pub trait ValidationFn: Send + Sync {
    fn validate(&self, value: &str) -> ValidationResult;
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub variable: String,
    pub message: String,
    pub rule: String,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub variable: String,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Plugin registry for managing loaded plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    security_policy: SecurityPolicy,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            security_policy: SecurityPolicy::default(),
        }
    }

    /// Register a plugin
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) -> Result<()> {
        let name = plugin.name().to_string();

        // Check security policy
        if !self.security_policy.allow_plugin(&name) {
            return Err(crate::error::EnvCliError::Config(format!(
                "Plugin '{}' is not allowed by security policy", name
            )));
        }

        self.plugins.insert(name, Box::new(plugin));
        Ok(())
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    /// Run scan with all applicable plugins
    pub fn run_scan(&self, context: &ScanContext) -> Result<Vec<ScanResult>> {
        let mut results = Vec::new();

        for plugin in self.plugins.values() {
            // Only run plugins that are applicable to this file type
            if self.is_plugin_applicable(plugin, context) {
                let result = plugin.scan(context)?;
                results.push(result);
            }
        }

        Ok(results)
    }

    /// Check if a plugin is applicable for the given context
    fn is_plugin_applicable(&self, plugin: &dyn Plugin, context: &ScanContext) -> bool {
        // Simple implementation - in a real system this would be more sophisticated
        true
    }
}

/// Security policy for plugin execution
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub allowed_plugins: Vec<String>,
    pub blocked_plugins: Vec<String>,
    pub require_signature: bool,
    pub sandbox_enabled: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            allowed_plugins: vec![
                "rust-scanner".to_string(),
                "javascript-scanner".to_string(),
                "python-scanner".to_string(),
                "security-analyzer".to_string(),
            ],
            blocked_plugins: Vec::new(),
            require_signature: false,
            sandbox_enabled: true,
        }
    }
}

impl SecurityPolicy {
    /// Check if a plugin is allowed
    pub fn allow_plugin(&self, name: &str) -> bool {
        if self.blocked_plugins.contains(&name.to_string()) {
            return false;
        }

        if self.allowed_plugins.is_empty() {
            return true; // No restrictions
        }

        self.allowed_plugins.contains(&name.to_string())
    }
}

/// Built-in plugins for common languages
pub mod builtin {
    use super::*;

    /// Rust language plugin
    pub struct RustPlugin;

    impl Plugin for RustPlugin {
        fn name(&self) -> &str {
            "rust-scanner"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn description(&self) -> &str {
            "Rust environment variable scanner"
        }

        fn scan(&self, context: &ScanContext) -> Result<ScanResult> {
            // Implementation for Rust-specific scanning
            Ok(ScanResult {
                variables: HashMap::new(),
                issues: Vec::new(),
                metadata: HashMap::new(),
            })
        }

        fn process_variables(&self, _context: &ProcessContext) -> Result<ProcessResult> {
            Ok(ProcessResult {
                variables: HashMap::new(),
                transformations: Vec::new(),
                warnings: Vec::new(),
            })
        }

        fn validate(&self, _context: &ValidationContext) -> Result<ValidationResult> {
            Ok(ValidationResult {
                is_valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            })
        }
    }

    /// JavaScript/TypeScript plugin
    pub struct JavaScriptPlugin;

    impl Plugin for JavaScriptPlugin {
        fn name(&self) -> &str {
            "javascript-scanner"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn description(&self) -> &str {
            "JavaScript/TypeScript environment variable scanner"
        }

        fn scan(&self, context: &ScanContext) -> Result<ScanResult> {
            // Implementation for JavaScript-specific scanning
            Ok(ScanResult {
                variables: HashMap::new(),
                issues: Vec::new(),
                metadata: HashMap::new(),
            })
        }

        fn process_variables(&self, _context: &ProcessContext) -> Result<ProcessResult> {
            Ok(ProcessResult {
                variables: HashMap::new(),
                transformations: Vec::new(),
                warnings: Vec::new(),
            })
        }

        fn validate(&self, _context: &ValidationContext) -> Result<ValidationResult> {
            Ok(ValidationResult {
                is_valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            })
        }
    }

    /// Security analysis plugin
    pub struct SecurityPlugin;

    impl Plugin for SecurityPlugin {
        fn name(&self) -> &str {
            "security-analyzer"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn description(&self) -> &str {
            "Security vulnerability scanner for environment variables"
        }

        fn scan(&self, context: &ScanContext) -> Result<ScanResult> {
            // Implementation for security analysis
            Ok(ScanResult {
                variables: HashMap::new(),
                issues: Vec::new(),
                metadata: HashMap::new(),
            })
        }

        fn process_variables(&self, _context: &ProcessContext) -> Result<ProcessResult> {
            Ok(ProcessResult {
                variables: HashMap::new(),
                transformations: Vec::new(),
                warnings: Vec::new(),
            })
        }

        fn validate(&self, _context: &ValidationContext) -> Result<ValidationResult> {
            Ok(ValidationResult {
                is_valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            })
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}