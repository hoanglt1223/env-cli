//! Environment synchronization functionality for EC-03.
//!
//! This module provides comprehensive environment variable synchronization
//! with conflict detection, resolution strategies, and comprehensive audit logging.

use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a synchronization conflict between environments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub variable: String,
    pub source_value: String,
    pub target_value: String,
    pub conflict_type: ConflictType,
    pub recommendation: ConflictResolution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    ValueMismatch,
    MissingInTarget,
    MissingInSource,
    TypeMismatch,
    SecurityViolation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    KeepSource,
    KeepTarget,
    Merge,
    Skip,
    ManualReview,
}

/// Represents an audit log entry for synchronization operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub operation: SyncOperation,
    pub source_env: String,
    pub target_env: String,
    pub variables_synced: Vec<String>,
    pub conflicts_resolved: Vec<String>,
    pub errors: Vec<String>,
    pub duration_ms: u64,
    pub user: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOperation {
    FullSync,
    IncrementalSync,
    ConflictResolution,
    Rollback,
}

/// Configuration for synchronization operations
#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub conflict_resolution: ConflictResolution,
    pub backup_before_sync: bool,
    pub audit_log_path: PathBuf,
    pub security_check: bool,
    pub dry_run: bool,
    pub selective_variables: Option<Vec<String>>,
}

/// Environment synchronization engine with advanced conflict resolution
pub struct EnvironmentSync {
    config: SyncConfig,
    audit_log: Vec<AuditLogEntry>,
}

impl EnvironmentSync {
    /// Create a new environment sync engine.
    pub fn new(config: SyncConfig) -> Self {
        Self {
            config,
            audit_log: Vec::new(),
        }
    }

    /// Synchronize environments with conflict detection and resolution.
    pub async fn sync_environments(
        &mut self,
        source_path: &PathBuf,
        target_path: &PathBuf,
    ) -> Result<SyncResult> {
        let start_time = std::time::Instant::now();

        // Load source and target environments
        let source_env = self.load_environment(source_path).await?;
        let target_env = self.load_environment(target_path).await?;

        // Detect conflicts
        let conflicts = self.detect_conflicts(&source_env, &target_env);

        // Create backup if configured
        if self.config.backup_before_sync {
            self.create_backup(target_path).await?;
        }

        // Resolve conflicts
        let resolved_conflicts = self.resolve_conflicts(conflicts).await?;

        // Perform synchronization
        let synced_variables = self
            .perform_sync(&source_env, &target_path, &resolved_conflicts)
            .await?;

        let duration = start_time.elapsed();

        // Log the operation
        self.log_operation(
            SyncOperation::FullSync,
            &source_env,
            &target_env,
            synced_variables.clone(),
            resolved_conflicts
                .iter()
                .map(|c| c.variable.clone())
                .collect(),
            Vec::new(),
            duration.as_millis() as u64,
        )
        .await?;

        Ok(SyncResult {
            synced_variables,
            conflicts_resolved: resolved_conflicts,
            duration,
            success: true,
            backup_created: self.config.backup_before_sync,
        })
    }

    /// Dry run synchronization without making changes.
    pub async fn dry_run_sync(
        &self,
        source_path: &PathBuf,
        target_path: &PathBuf,
    ) -> Result<DryRunResult> {
        let source_env = self.load_environment(source_path).await?;
        let target_env = self.load_environment(target_path).await?;

        let conflicts = self.detect_conflicts(&source_env, &target_env);
        let proposed_changes =
            self.calculate_proposed_changes(&source_env, &target_env, &conflicts);

        Ok(DryRunResult {
            conflicts,
            proposed_changes,
            estimated_duration: self.estimate_sync_duration(&source_env, &target_env),
            security_violations: self.check_security_violations(&source_env, &target_env)?,
        })
    }

    /// Load environment from file.
    async fn load_environment(&self, path: &PathBuf) -> Result<HashMap<String, String>> {
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let content = std::fs::read_to_string(path)?;
        let mut env_vars = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                env_vars.insert(key, value);
            }
        }

        Ok(env_vars)
    }

    /// Detect conflicts between source and target environments.
    fn detect_conflicts(
        &self,
        source: &HashMap<String, String>,
        target: &HashMap<String, String>,
    ) -> Vec<SyncConflict> {
        let mut conflicts = Vec::new();

        // Check for value mismatches
        for (key, source_value) in source {
            if let Some(target_value) = target.get(key) {
                if source_value != target_value {
                    conflicts.push(SyncConflict {
                        variable: key.clone(),
                        source_value: source_value.clone(),
                        target_value: target_value.clone(),
                        conflict_type: ConflictType::ValueMismatch,
                        recommendation: self.recommend_resolution(key, source_value, target_value),
                    });
                }
            } else {
                conflicts.push(SyncConflict {
                    variable: key.clone(),
                    source_value: source_value.clone(),
                    target_value: "".to_string(),
                    conflict_type: ConflictType::MissingInTarget,
                    recommendation: ConflictResolution::KeepSource,
                });
            }
        }

        // Check for variables that exist only in target
        for key in target.keys() {
            if !source.contains_key(key) {
                conflicts.push(SyncConflict {
                    variable: key.clone(),
                    source_value: "".to_string(),
                    target_value: target[key].clone(),
                    conflict_type: ConflictType::MissingInSource,
                    recommendation: ConflictResolution::KeepTarget,
                });
            }
        }

        conflicts
    }

    /// Recommend resolution strategy for a conflict.
    fn recommend_resolution(
        &self,
        variable: &str,
        _source_value: &str,
        target_value: &str,
    ) -> ConflictResolution {
        // Security-sensitive variables should be manually reviewed
        if self.is_security_sensitive(variable) {
            return ConflictResolution::ManualReview;
        }

        // Empty values suggest the target should be updated
        if target_value.is_empty() {
            return ConflictResolution::KeepSource;
        }

        // Default to manual review for conflicting non-empty values
        ConflictResolution::ManualReview
    }

    /// Check if a variable is security-sensitive.
    fn is_security_sensitive(&self, variable: &str) -> bool {
        let variable_lower = variable.to_lowercase();
        variable_lower.contains("password")
            || variable_lower.contains("secret")
            || variable_lower.contains("key")
            || variable_lower.contains("token")
            || variable_lower.contains("credential")
            || variable_lower.contains("private")
    }

    /// Resolve conflicts based on configuration.
    async fn resolve_conflicts(&self, conflicts: Vec<SyncConflict>) -> Result<Vec<SyncConflict>> {
        let mut resolved = Vec::new();

        for conflict in conflicts {
            let resolution = match self.config.conflict_resolution {
                ConflictResolution::ManualReview => {
                    // In a real implementation, this would prompt the user
                    // For now, use the recommended resolution
                    conflict.recommendation.clone()
                }
                _ => self.config.conflict_resolution.clone(),
            };

            resolved.push(SyncConflict {
                recommendation: resolution,
                ..conflict
            });
        }

        Ok(resolved)
    }

    /// Perform the actual synchronization.
    async fn perform_sync(
        &self,
        _source_env: &HashMap<String, String>,
        target_path: &PathBuf,
        resolved_conflicts: &[SyncConflict],
    ) -> Result<Vec<String>> {
        let mut synced_vars = Vec::new();

        for conflict in resolved_conflicts {
            match conflict.recommendation {
                ConflictResolution::KeepSource | ConflictResolution::Merge => {
                    // Update target with source value
                    self.update_variable(target_path, &conflict.variable, &conflict.source_value)
                        .await?;
                    synced_vars.push(conflict.variable.clone());
                }
                ConflictResolution::KeepTarget | ConflictResolution::Skip => {
                    // Do nothing
                }
                ConflictResolution::ManualReview => {
                    // Skip for now - in real implementation would prompt user
                }
            }
        }

        Ok(synced_vars)
    }

    /// Update a single variable in the target environment file.
    async fn update_variable(
        &self,
        target_path: &PathBuf,
        variable: &str,
        value: &str,
    ) -> Result<()> {
        let content = if target_path.exists() {
            std::fs::read_to_string(target_path)?
        } else {
            String::new()
        };

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;

        // Try to update existing variable
        for line in &mut lines {
            if line.trim().starts_with(&format!("{}=", variable)) {
                *line = format!("{}={}", variable, value);
                found = true;
                break;
            }
        }

        // Add new variable if not found
        if !found {
            lines.push(format!("{}={}", variable, value));
        }

        // Write back to file
        std::fs::write(target_path, lines.join("\n") + "\n")?;

        Ok(())
    }

    /// Create backup of target environment.
    async fn create_backup(&self, target_path: &PathBuf) -> Result<PathBuf> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = target_path.with_extension(format!("env.backup.{}", timestamp));

        if target_path.exists() {
            std::fs::copy(target_path, &backup_path)?;
        }

        Ok(backup_path)
    }

    /// Calculate proposed changes for dry run.
    fn calculate_proposed_changes(
        &self,
        _source: &HashMap<String, String>,
        _target: &HashMap<String, String>,
        conflicts: &[SyncConflict],
    ) -> Vec<ProposedChange> {
        let mut changes = Vec::new();

        for conflict in conflicts {
            let change_type = match conflict.conflict_type {
                ConflictType::MissingInTarget => ChangeType::Add,
                ConflictType::ValueMismatch => ChangeType::Update,
                ConflictType::MissingInSource => ChangeType::Remove,
                _ => ChangeType::Conflict,
            };

            changes.push(ProposedChange {
                variable: conflict.variable.clone(),
                change_type,
                old_value: conflict.target_value.clone(),
                new_value: conflict.source_value.clone(),
            });
        }

        changes
    }

    /// Estimate sync duration.
    fn estimate_sync_duration(
        &self,
        source: &HashMap<String, String>,
        _target: &HashMap<String, String>,
    ) -> std::time::Duration {
        // Rough estimation: 1ms per variable
        std::time::Duration::from_millis(source.len() as u64)
    }

    /// Check for security violations.
    fn check_security_violations(
        &self,
        source: &HashMap<String, String>,
        target: &HashMap<String, String>,
    ) -> Result<Vec<SecurityViolation>> {
        let mut violations = Vec::new();

        // Check for potential secrets being exposed
        for (key, value) in source {
            if self.is_security_sensitive(key) && !value.is_empty() {
                if let Some(target_value) = target.get(key) {
                    if target_value != value {
                        violations.push(SecurityViolation {
                            variable: key.clone(),
                            violation_type: SecurityViolationType::SecretModification,
                            severity: SecuritySeverity::High,
                            description: format!(
                                "Security-sensitive variable '{}' is being modified",
                                key
                            ),
                        });
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Log synchronization operation.
    async fn log_operation(
        &mut self,
        operation: SyncOperation,
        source_env: &HashMap<String, String>,
        target_env: &HashMap<String, String>,
        variables_synced: Vec<String>,
        conflicts_resolved: Vec<String>,
        errors: Vec<String>,
        duration_ms: u64,
    ) -> Result<()> {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            operation,
            source_env: format!("{} variables", source_env.len()),
            target_env: format!("{} variables", target_env.len()),
            variables_synced,
            conflicts_resolved,
            errors,
            duration_ms,
            user: std::env::var("USER")
                .ok()
                .or_else(|| std::env::var("USERNAME").ok()),
        };

        self.audit_log.push(entry);

        // Write audit log to file if configured
        if self
            .config
            .audit_log_path
            .parent()
            .map(|p| p.exists())
            .unwrap_or(true)
        {
            self.write_audit_log().await?;
        }

        Ok(())
    }

    /// Write audit log to file.
    async fn write_audit_log(&self) -> Result<()> {
        let log_content = serde_json::to_string_pretty(&self.audit_log)?;
        std::fs::write(&self.config.audit_log_path, log_content)?;
        Ok(())
    }
}

/// Result of a synchronization operation
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub synced_variables: Vec<String>,
    pub conflicts_resolved: Vec<SyncConflict>,
    pub duration: std::time::Duration,
    pub success: bool,
    pub backup_created: bool,
}

/// Result of a dry run operation
#[derive(Debug, Clone)]
pub struct DryRunResult {
    pub conflicts: Vec<SyncConflict>,
    pub proposed_changes: Vec<ProposedChange>,
    pub estimated_duration: std::time::Duration,
    pub security_violations: Vec<SecurityViolation>,
}

/// A proposed change during dry run
#[derive(Debug, Clone)]
pub struct ProposedChange {
    pub variable: String,
    pub change_type: ChangeType,
    pub old_value: String,
    pub new_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Add,
    Update,
    Remove,
    Conflict,
}

/// Security violation detected during sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityViolation {
    pub variable: String,
    pub violation_type: SecurityViolationType,
    pub severity: SecuritySeverity,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityViolationType {
    SecretModification,
    InsecureValue,
    UnencryptedTransmission,
    PrivilegeEscalation,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}
