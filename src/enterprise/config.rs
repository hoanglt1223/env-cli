//! # Enterprise Configuration Module
//!
//! This module handles enterprise-specific configuration management
//! including workspace settings, security policies, and integration configs.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::error::EnvCliError;

/// Re-export the audit config from the main enterprise module
pub use super::AuditConfig;

/// Enterprise-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    /// Whether enterprise features are enabled
    pub enabled: bool,
    /// Workspace name
    pub workspace_name: String,
    /// Workspace description
    pub workspace_description: String,
    /// Security configuration
    pub security: SecurityConfig,
    /// Audit configuration
    pub audit: AuditConfig,
    /// Compliance configuration
    pub compliance: ComplianceConfig,
    /// SSO configuration
    pub sso: Option<SsoConfig>,
    /// LDAP configuration
    pub ldap: Option<LdapConfig>,
    /// Secrets management configuration
    pub secrets: SecretsConfig,
    /// RBAC configuration
    pub rbac: RbacConfig,
    /// Backup configuration
    pub backup: BackupConfig,
    /// Notification configuration
    pub notifications: NotificationConfig,
}

impl Default for EnterpriseConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            workspace_name: "default".to_string(),
            workspace_description: "Default workspace".to_string(),
            security: SecurityConfig::default(),
            audit: AuditConfig::default(),
            compliance: ComplianceConfig::default(),
            sso: None,
            ldap: None,
            secrets: SecretsConfig::default(),
            rbac: RbacConfig::default(),
            backup: BackupConfig::default(),
            notifications: NotificationConfig::default(),
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable encryption at rest
    pub encryption_at_rest: bool,
    /// Encryption algorithm
    pub encryption_algorithm: String,
    /// Secret rotation period in days
    pub secret_rotation_days: u32,
    /// Minimum secret length
    pub min_secret_length: usize,
    /// Enable zero-knowledge encryption
    pub zero_knowledge: bool,
    /// Password policy
    pub password_policy: PasswordPolicy,
    /// Session timeout in minutes
    pub session_timeout_minutes: u32,
    /// Maximum login attempts
    pub max_login_attempts: u32,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Enable MFA
    pub mfa_enabled: bool,
    /// MFA methods
    pub mfa_methods: Vec<MfaMethod>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encryption_at_rest: true,
            encryption_algorithm: "AES-256-GCM".to_string(),
            secret_rotation_days: 90,
            min_secret_length: 16,
            zero_knowledge: true,
            password_policy: PasswordPolicy::default(),
            session_timeout_minutes: 480, // 8 hours
            max_login_attempts: 5,
            lockout_duration_minutes: 30,
            mfa_enabled: false,
            mfa_methods: vec![],
        }
    }
}

/// Password policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Minimum password length
    pub min_length: usize,
    /// Require uppercase letters
    pub require_uppercase: bool,
    /// Require lowercase letters
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special_chars: bool,
    /// Special characters allowed
    pub special_chars: String,
    /// Password history size
    pub history_size: usize,
    /// Maximum password age in days
    pub max_age_days: u32,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
            history_size: 12,
            max_age_days: 90,
        }
    }
}

/// MFA methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// Time-based One-Time Password (TOTP)
    TOTP,
    /// SMS verification
    SMS,
    /// Email verification
    Email,
    /// Hardware token
    HardwareToken,
    /// Push notification
    Push,
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Compliance frameworks to enforce
    pub frameworks: Vec<String>,
    /// Enable automatic reporting
    pub auto_reports: bool,
    /// Report schedule
    pub report_schedule: String,
    /// Data retention policies
    pub data_retention_policies: HashMap<String, u32>,
    /// Compliance rules
    pub rules: Vec<ComplianceRule>,
    /// Remediation settings
    pub remediation: RemediationConfig,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            frameworks: vec!["SOC2".to_string(), "ISO27001".to_string()],
            auto_reports: true,
            report_schedule: "monthly".to_string(),
            data_retention_policies: HashMap::new(),
            rules: vec![],
            remediation: RemediationConfig::default(),
        }
    }
}

/// Compliance rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Framework this rule belongs to
    pub framework: String,
    /// Rule category
    pub category: String,
    /// Rule severity
    pub severity: String,
    /// Enabled status
    pub enabled: bool,
    /// Rule conditions
    pub conditions: Vec<RuleCondition>,
    /// Actions to take when rule is violated
    pub actions: Vec<RuleAction>,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    /// Field to check
    pub field: String,
    /// Condition operator
    pub operator: String,
    /// Expected value
    pub value: String,
}

/// Rule action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    /// Log warning
    Log,
    /// Block action
    Block,
    /// Require approval
    RequireApproval,
    /// Send alert
    Alert,
    /// Create ticket
    CreateTicket,
}

/// Remediation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationConfig {
    /// Enable automatic remediation
    pub auto_remediate: bool,
    /// Remediation strategies
    pub strategies: Vec<RemediationStrategy>,
    /// Approval required for remediation
    pub require_approval: bool,
}

impl Default for RemediationConfig {
    fn default() -> Self {
        Self {
            auto_remediate: false,
            strategies: vec![],
            require_approval: true,
        }
    }
}

/// Remediation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationStrategy {
    /// Strategy name
    pub name: String,
    /// Strategy description
    pub description: String,
    /// Conditions for applying this strategy
    pub conditions: Vec<String>,
    /// Actions to take
    pub actions: Vec<String>,
}

/// SSO configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    /// SSO provider type
    pub provider: String,
    /// Provider name
    pub name: String,
    /// SSO provider specific configuration
    pub provider_config: HashMap<String, String>,
    /// Attribute mapping
    pub attribute_mapping: AttributeMapping,
    /// Role mapping
    pub role_mapping: HashMap<String, String>,
    /// Group mapping
    pub group_mapping: HashMap<String, String>,
}

/// Attribute mapping for SSO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeMapping {
    /// User ID attribute
    pub user_id: String,
    /// Username attribute
    pub username: String,
    /// Email attribute
    pub email: String,
    /// Display name attribute
    pub display_name: String,
    /// Groups attribute
    pub groups: String,
}

impl Default for AttributeMapping {
    fn default() -> Self {
        Self {
            user_id: "sub".to_string(),
            username: "preferred_username".to_string(),
            email: "email".to_string(),
            display_name: "name".to_string(),
            groups: "groups".to_string(),
        }
    }
}

/// LDAP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapConfig {
    /// LDAP server URL
    pub server_url: String,
    /// LDAP port
    pub port: u16,
    /// Use SSL/TLS
    pub use_ssl: bool,
    /// Base DN
    pub base_dn: String,
    /// Bind DN
    pub bind_dn: String,
    /// Bind password
    pub bind_password: String,
    /// User search base
    pub user_search_base: String,
    /// User search filter
    pub user_search_filter: String,
    /// Group search base
    pub group_search_base: String,
    /// Group search filter
    pub group_search_filter: String,
    /// User attributes
    pub user_attributes: UserAttributes,
}

/// LDAP user attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    /// User ID attribute
    pub user_id: String,
    /// Username attribute
    pub username: String,
    /// Email attribute
    pub email: String,
    /// Display name attribute
    pub display_name: String,
    /// First name attribute
    pub first_name: String,
    /// Last name attribute
    pub last_name: String,
    /// Groups attribute
    pub groups: String,
}

impl Default for UserAttributes {
    fn default() -> Self {
        Self {
            user_id: "objectGUID".to_string(),
            username: "sAMAccountName".to_string(),
            email: "mail".to_string(),
            display_name: "displayName".to_string(),
            first_name: "givenName".to_string(),
            last_name: "sn".to_string(),
            groups: "memberOf".to_string(),
        }
    }
}

/// Secrets management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsConfig {
    /// Secrets provider
    pub provider: String,
    /// Provider-specific configuration
    pub provider_config: HashMap<String, String>,
    /// Enable caching
    pub cache_enabled: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u32,
    /// Auto-rotation enabled
    pub auto_rotation: bool,
    /// Rotation interval in days
    pub rotation_interval_days: u32,
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            provider: "internal".to_string(),
            provider_config: HashMap::new(),
            cache_enabled: true,
            cache_ttl_seconds: 3600, // 1 hour
            auto_rotation: false,
            rotation_interval_days: 90,
        }
    }
}

/// RBAC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfig {
    /// Default role for new users
    pub default_role: String,
    /// Admin users list
    pub admin_users: Vec<String>,
    /// Role hierarchy
    pub role_hierarchy: HashMap<String, Vec<String>>,
    /// Permission matrix
    pub permission_matrix: HashMap<String, Vec<String>>,
    /// Custom roles
    pub custom_roles: Vec<CustomRole>,
}

impl Default for RbacConfig {
    fn default() -> Self {
        Self {
            default_role: "viewer".to_string(),
            admin_users: vec![],
            role_hierarchy: HashMap::new(),
            permission_matrix: HashMap::new(),
            custom_roles: vec![],
        }
    }
}

/// Custom role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRole {
    /// Role name
    pub name: String,
    /// Role description
    pub description: String,
    /// Role permissions
    pub permissions: Vec<String>,
    /// Inherited roles
    pub inherits: Vec<String>,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    pub enabled: bool,
    /// Backup frequency in hours
    pub frequency_hours: u32,
    /// Number of backups to retain
    pub retain_count: u32,
    /// Backup location
    pub location: BackupLocation,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Compression enabled
    pub compression_enabled: bool,
    /// Include environment files
    pub include_environments: bool,
    /// Include audit logs
    pub include_audit_logs: bool,
    /// Include user data
    pub include_user_data: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency_hours: 24,
            retain_count: 30,
            location: BackupLocation::Local { path: "./backups".to_string() },
            encryption_enabled: true,
            compression_enabled: true,
            include_environments: true,
            include_audit_logs: true,
            include_user_data: false,
        }
    }
}

/// Backup location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupLocation {
    /// Local filesystem
    Local { path: String },
    /// S3 bucket
    S3 { bucket: String, prefix: String, region: String },
    /// Azure Blob Storage
    Azure { container: String, prefix: String },
    /// Google Cloud Storage
    GCS { bucket: String, prefix: String },
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Enable notifications
    pub enabled: bool,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
    /// Notification events
    pub events: NotificationEvents,
    /// Rate limiting
    pub rate_limit: RateLimit,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: vec![],
            events: NotificationEvents::default(),
            rate_limit: RateLimit::default(),
        }
    }
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: String,
    /// Channel configuration
    pub config: HashMap<String, String>,
    /// Enabled status
    pub enabled: bool,
}

/// Notification events configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvents {
    /// Environment changes
    pub environment_changes: bool,
    /// User management events
    pub user_management: bool,
    /// Security events
    pub security_events: bool,
    /// Compliance events
    pub compliance_events: bool,
    /// Backup events
    pub backup_events: bool,
    /// System events
    pub system_events: bool,
}

impl Default for NotificationEvents {
    fn default() -> Self {
        Self {
            environment_changes: true,
            user_management: true,
            security_events: true,
            compliance_events: true,
            backup_events: false,
            system_events: false,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Max notifications per minute
    pub max_per_minute: u32,
    /// Max notifications per hour
    pub max_per_hour: u32,
    /// Max notifications per day
    pub max_per_day: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            max_per_minute: 10,
            max_per_hour: 100,
            max_per_day: 1000,
        }
    }
}

/// Enterprise configuration manager
pub struct EnterpriseConfigManager {
    config: EnterpriseConfig,
    config_path: Option<PathBuf>,
}

impl EnterpriseConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            config: EnterpriseConfig::default(),
            config_path: None,
        }
    }

    /// Load configuration from file
    pub async fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = tokio::fs::read_to_string(&path).await
            .map_err(|e| EnvCliError::ConfigError(format!("Failed to read config file: {}", e)))?;

        self.config = toml::from_str(&content)
            .map_err(|e| EnvCliError::ConfigError(format!("Failed to parse config file: {}", e)))?;

        self.config_path = Some(PathBuf::from(path.as_ref()));
        Ok(())
    }

    /// Save configuration to file
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| EnvCliError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        tokio::fs::write(&path, content).await
            .map_err(|e| EnvCliError::ConfigError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    /// Get configuration reference
    pub fn config(&self) -> &EnterpriseConfig {
        &self.config
    }

    /// Get mutable configuration reference
    pub fn config_mut(&mut self) -> &mut EnterpriseConfig {
        &mut self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: EnterpriseConfig) {
        self.config = config;
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.config.enabled {
            // Validate workspace name
            if self.config.workspace_name.is_empty() {
                return Err(EnvCliError::ConfigError("Workspace name cannot be empty".to_string()).into());
            }

            // Validate security settings
            if self.config.security.encryption_at_rest {
                if self.config.security.encryption_algorithm.is_empty() {
                    return Err(EnvCliError::ConfigError("Encryption algorithm cannot be empty".to_string()).into());
                }
            }

            // Validate SSO configuration if provided
            if let Some(sso_config) = &self.config.sso {
                if sso_config.provider.is_empty() {
                    return Err(EnvCliError::ConfigError("SSO provider cannot be empty".to_string()).into());
                }
            }

            // Validate LDAP configuration if provided
            if let Some(ldap_config) = &self.config.ldap {
                if ldap_config.server_url.is_empty() {
                    return Err(EnvCliError::ConfigError("LDAP server URL cannot be empty".to_string()).into());
                }
                if ldap_config.base_dn.is_empty() {
                    return Err(EnvCliError::ConfigError("LDAP base DN cannot be empty".to_string()).into());
                }
            }
        }

        Ok(())
    }

    /// Get configuration summary
    pub fn get_summary(&self) -> ConfigSummary {
        ConfigSummary {
            enterprise_enabled: self.config.enabled,
            workspace_name: self.config.workspace_name.clone(),
            encryption_enabled: self.config.security.encryption_at_rest,
            audit_enabled: self.config.audit.log_all_actions,
            sso_configured: self.config.sso.is_some(),
            ldap_configured: self.config.ldap.is_some(),
            backup_enabled: self.config.backup.enabled,
            notifications_enabled: self.config.notifications.enabled,
            compliance_frameworks: self.config.compliance.frameworks.clone(),
        }
    }
}

impl Default for EnterpriseConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSummary {
    /// Enterprise features enabled
    pub enterprise_enabled: bool,
    /// Workspace name
    pub workspace_name: String,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Audit enabled
    pub audit_enabled: bool,
    /// SSO configured
    pub sso_configured: bool,
    /// LDAP configured
    pub ldap_configured: bool,
    /// Backup enabled
    pub backup_enabled: bool,
    /// Notifications enabled
    pub notifications_enabled: bool,
    /// Compliance frameworks
    pub compliance_frameworks: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enterprise_config_default() {
        let config = EnterpriseConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.workspace_name, "default");
        assert!(config.security.encryption_at_rest);
        assert!(config.audit.log_all_actions);
    }

    #[test]
    fn test_password_policy() {
        let policy = PasswordPolicy::default();
        assert_eq!(policy.min_length, 12);
        assert!(policy.require_uppercase);
        assert!(policy.require_lowercase);
        assert!(policy.require_numbers);
        assert!(policy.require_special_chars);
    }

    #[test]
    fn test_config_validation() {
        let mut manager = EnterpriseConfigManager::new();
        let mut config = EnterpriseConfig::default();
        config.enabled = true;
        config.workspace_name = "".to_string(); // Invalid empty name

        manager.update_config(config);
        let result = manager.validate();
        assert!(result.is_err());

        // Fix the validation error
        manager.config_mut().workspace_name = "test-workspace".to_string();
        let result = manager.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_attribute_mapping_default() {
        let mapping = AttributeMapping::default();
        assert_eq!(mapping.user_id, "sub");
        assert_eq!(mapping.email, "email");
        assert_eq!(mapping.username, "preferred_username");
    }

    #[test]
    fn test_backup_location() {
        let local = BackupLocation::Local { path: "/tmp/backups".to_string() };
        let s3 = BackupLocation::S3 {
            bucket: "my-bucket".to_string(),
            prefix: "backups".to_string(),
            region: "us-west-2".to_string(),
        };

        match local {
            BackupLocation::Local { path } => assert_eq!(path, "/tmp/backups"),
            _ => panic!("Expected Local backup location"),
        }

        match s3 {
            BackupLocation::S3 { bucket, prefix, region } => {
                assert_eq!(bucket, "my-bucket");
                assert_eq!(prefix, "backups");
                assert_eq!(region, "us-west-2");
            },
            _ => panic!("Expected S3 backup location"),
        }
    }
}