//! # Audit Logging Module
//!
//! This module implements comprehensive audit logging for all enterprise
//! operations with configurable retention and forensic capabilities.

use crate::enterprise::config::AuditConfig;
use crate::enterprise::{EnvironmentId, Outcome, UserId, WorkspaceId};
use crate::error::EnvCliError;
use crate::error::Result;
use chrono::{DateTime, Duration, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Comprehensive audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event identifier
    pub id: Uuid,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// User ID who performed the action
    pub user_id: UserId,
    /// Workspace ID (if applicable)
    pub workspace_id: Option<WorkspaceId>,
    /// Environment ID (if applicable)
    pub environment_id: Option<EnvironmentId>,
    /// Action performed
    pub action: String,
    /// Resource type
    pub resource_type: String,
    /// Resource identifier
    pub resource_id: Option<String>,
    /// Outcome of the action
    pub outcome: Outcome,
    /// Event details
    pub details: HashMap<String, serde_json::Value>,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// Severity level
    pub severity: AuditSeverity,
    /// Category of the event
    pub category: AuditCategory,
    /// Tags for filtering and analysis
    pub tags: Vec<String>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(user_id: UserId, action: String, resource_type: String, outcome: Outcome) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id,
            workspace_id: None,
            environment_id: None,
            action,
            resource_type,
            resource_id: None,
            outcome,
            details: HashMap::new(),
            ip_address: None,
            user_agent: None,
            session_id: None,
            request_id: None,
            severity: AuditSeverity::Info,
            category: AuditCategory::General,
            tags: vec![],
        }
    }

    /// Add workspace context
    pub fn with_workspace(mut self, workspace_id: WorkspaceId) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }

    /// Add environment context
    pub fn with_environment(mut self, environment_id: EnvironmentId) -> Self {
        self.environment_id = Some(environment_id);
        self
    }

    /// Add resource ID
    pub fn with_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Add detail
    pub fn with_detail(mut self, key: String, value: serde_json::Value) -> Self {
        self.details.insert(key, value);
        self
    }

    /// Add IP address
    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    /// Add user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Add session ID
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Add request ID
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Set severity level
    pub fn with_severity(mut self, severity: AuditSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set category
    pub fn with_category(mut self, category: AuditCategory) -> Self {
        self.category = category;
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Audit severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditSeverity {
    /// Debug level information
    Debug,
    /// General informational events
    Info,
    /// Warning events that should be investigated
    Warning,
    /// Error events requiring attention
    Error,
    /// Critical events requiring immediate action
    Critical,
}

impl AuditSeverity {
    /// Get numeric severity level for filtering
    pub fn level(&self) -> u8 {
        match self {
            AuditSeverity::Debug => 10,
            AuditSeverity::Info => 20,
            AuditSeverity::Warning => 30,
            AuditSeverity::Error => 40,
            AuditSeverity::Critical => 50,
        }
    }

    /// Check if this severity is higher than another
    pub fn is_higher_than(&self, other: &AuditSeverity) -> bool {
        self.level() > other.level()
    }
}

/// Audit event categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditCategory {
    /// General uncategorized events
    General,
    /// Authentication events
    Authentication,
    /// Authorization events
    Authorization,
    /// Environment management events
    Environment,
    /// Workspace management events
    Workspace,
    /// User management events
    User,
    /// Configuration events
    Configuration,
    /// Security events
    Security,
    /// Compliance events
    Compliance,
    /// System events
    System,
    /// Data events
    Data,
}

/// Audit storage interface
#[async_trait::async_trait]
pub trait AuditStorage: Send + Sync {
    /// Store an audit event
    async fn store_event(&self, event: &AuditEvent) -> Result<()>;

    /// Query audit events
    async fn query_events(&self, query: &AuditQuery) -> Result<Vec<AuditEvent>>;

    /// Delete old events based on retention policy
    async fn cleanup_old_events(&self, retention_days: u32) -> Result<usize>;

    /// Get event count for statistics
    async fn get_event_count(&self, query: &AuditQuery) -> Result<u64>;
}

/// In-memory audit storage for development
pub struct InMemoryAuditStorage {
    events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl InMemoryAuditStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl AuditStorage for InMemoryAuditStorage {
    async fn store_event(&self, event: &AuditEvent) -> Result<()> {
        let mut events = self.events.write().await;
        events.push(event.clone());
        Ok(())
    }

    async fn query_events(&self, query: &AuditQuery) -> Result<Vec<AuditEvent>> {
        let events = self.events.read().await;
        let mut filtered_events = events.clone();

        // Apply filters
        if let Some(user_id) = &query.user_id {
            filtered_events.retain(|e| e.user_id == *user_id);
        }

        if let Some(workspace_id) = &query.workspace_id {
            filtered_events.retain(|e| e.workspace_id == Some(*workspace_id));
        }

        if let Some(severity) = &query.severity {
            filtered_events.retain(|e| &e.severity == severity);
        }

        if let Some(category) = &query.category {
            filtered_events.retain(|e| &e.category == category);
        }

        if let Some(start_time) = query.start_time {
            filtered_events.retain(|e| e.timestamp >= start_time);
        }

        if let Some(end_time) = query.end_time {
            filtered_events.retain(|e| e.timestamp <= end_time);
        }

        // Sort by timestamp (newest first)
        filtered_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = query.limit {
            filtered_events.truncate(limit);
        }

        Ok(filtered_events)
    }

    async fn cleanup_old_events(&self, retention_days: u32) -> Result<usize> {
        let mut events = self.events.write().await;
        let cutoff_time = Utc::now() - Duration::days(retention_days as i64);
        let initial_count = events.len();

        events.retain(|e| e.timestamp >= cutoff_time);

        Ok(initial_count - events.len())
    }

    async fn get_event_count(&self, query: &AuditQuery) -> Result<u64> {
        let events = self.query_events(query).await?;
        Ok(events.len() as u64)
    }
}

/// Audit query for filtering events
#[derive(Debug, Clone)]
pub struct AuditQuery {
    /// Filter by user ID
    pub user_id: Option<UserId>,
    /// Filter by workspace ID
    pub workspace_id: Option<WorkspaceId>,
    /// Filter by environment ID
    pub environment_id: Option<EnvironmentId>,
    /// Filter by action
    pub action: Option<String>,
    /// Filter by resource type
    pub resource_type: Option<String>,
    /// Filter by outcome
    pub outcome: Option<Outcome>,
    /// Filter by severity level
    pub severity: Option<AuditSeverity>,
    /// Filter by category
    pub category: Option<AuditCategory>,
    /// Filter by start time
    pub start_time: Option<DateTime<Utc>>,
    /// Filter by end time
    pub end_time: Option<DateTime<Utc>>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
    /// Text search in details
    pub search: Option<String>,
}

impl Default for AuditQuery {
    fn default() -> Self {
        Self {
            user_id: None,
            workspace_id: None,
            environment_id: None,
            action: None,
            resource_type: None,
            outcome: None,
            severity: None,
            category: None,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
            search: None,
        }
    }
}

/// Retention policy for audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Retention period in days for general events
    pub general_retention_days: u32,
    /// Retention period in days for security events
    pub security_retention_days: u32,
    /// Retention period in days for compliance events
    pub compliance_retention_days: u32,
    /// Enable archival for old events
    pub enable_archival: bool,
    /// Archival storage location
    pub archival_location: Option<String>,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            general_retention_days: 90,
            security_retention_days: 2555,   // 7 years
            compliance_retention_days: 2555, // 7 years
            enable_archival: false,
            archival_location: None,
        }
    }
}

/// Main audit logger
pub struct AuditLogger {
    /// Audit storage backend
    storage: Arc<dyn AuditStorage>,
    /// Audit configuration
    config: AuditConfig,
    /// Retention policy
    retention_policy: RetentionPolicy,
    /// Event formatter for different output formats
    formatter: AuditFormatter,
    /// Metrics for audit statistics
    metrics: Arc<Mutex<AuditMetrics>>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub async fn new(config: &AuditConfig) -> Result<Self> {
        let storage: Arc<dyn AuditStorage> = if config.forensic_mode {
            // In a real implementation, this would use a database or file storage
            Arc::new(InMemoryAuditStorage::new())
        } else {
            Arc::new(InMemoryAuditStorage::new())
        };

        Ok(Self {
            storage,
            config: config.clone(),
            retention_policy: RetentionPolicy::default(),
            formatter: AuditFormatter::new(),
            metrics: Arc::new(Mutex::new(AuditMetrics::new())),
        })
    }

    /// Log an audit event
    pub async fn log_event(&self, event: AuditEvent) -> Result<()> {
        // Update metrics
        {
            let mut metrics = self.metrics.lock();
            metrics.total_events += 1;
            metrics
                .events_by_category
                .entry(event.category.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            metrics
                .events_by_severity
                .entry(event.severity.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // Store the event
        self.storage.store_event(&event).await?;

        // Check for critical events that need immediate attention
        if event.severity == AuditSeverity::Critical {
            self.handle_critical_event(&event).await?;
        }

        Ok(())
    }

    /// Log a successful action
    pub async fn log_success(
        &self,
        user_id: UserId,
        action: String,
        resource_type: String,
        details: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<()> {
        let mut event = AuditEvent::new(user_id, action, resource_type, Outcome::Success);

        if let Some(details) = details {
            event.details = details;
        }

        self.log_event(event).await
    }

    /// Log a failed action
    pub async fn log_failure(
        &self,
        user_id: UserId,
        action: String,
        resource_type: String,
        error: String,
    ) -> Result<()> {
        let event = AuditEvent::new(user_id, action, resource_type, Outcome::Failure)
            .with_severity(AuditSeverity::Error)
            .with_detail("error".to_string(), serde_json::Value::String(error));

        self.log_event(event).await
    }

    /// Log a denied action
    pub async fn log_denied(
        &self,
        user_id: UserId,
        action: String,
        resource_type: String,
        reason: String,
    ) -> Result<()> {
        let event = AuditEvent::new(user_id, action, resource_type, Outcome::Denied)
            .with_category(AuditCategory::Authorization)
            .with_severity(AuditSeverity::Warning)
            .with_detail("reason".to_string(), serde_json::Value::String(reason));

        self.log_event(event).await
    }

    /// Query audit events
    pub async fn query_events(&self, query: &AuditQuery) -> Result<Vec<AuditEvent>> {
        self.storage.query_events(query).await
    }

    /// Get audit metrics
    pub async fn get_metrics(&self) -> AuditMetrics {
        self.metrics.lock().clone()
    }

    /// Clean up old events based on retention policy
    pub async fn cleanup_old_events(&self) -> Result<usize> {
        self.storage
            .cleanup_old_events(self.config.retention_days)
            .await
    }

    /// Generate audit report
    pub async fn generate_report(
        &self,
        query: &AuditQuery,
        format: ReportFormat,
    ) -> Result<String> {
        let events = self.query_events(query).await?;
        self.formatter.format_events(&events, format)
    }

    /// Handle critical audit events
    async fn handle_critical_event(&self, event: &AuditEvent) -> Result<()> {
        // In a real implementation, this would send alerts, notifications, etc.
        eprintln!(
            "CRITICAL AUDIT EVENT: {} by {}",
            event.action, event.user_id
        );
        Ok(())
    }
}

/// Audit metrics and statistics
#[derive(Debug, Clone, Default)]
pub struct AuditMetrics {
    /// Total number of events logged
    pub total_events: u64,
    /// Events by category
    pub events_by_category: HashMap<AuditCategory, u64>,
    /// Events by severity
    pub events_by_severity: HashMap<AuditSeverity, u64>,
    /// Events by user
    pub events_by_user: HashMap<UserId, u64>,
    /// Events by action
    pub events_by_action: HashMap<String, u64>,
}

impl AuditMetrics {
    /// Create new audit metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Get percentage of events by severity
    pub fn get_severity_percentage(&self, severity: &AuditSeverity) -> f64 {
        if self.total_events == 0 {
            return 0.0;
        }

        let count = self.events_by_severity.get(severity).unwrap_or(&0);
        (*count as f64 / self.total_events as f64) * 100.0
    }
}

/// Audit event formatter for different output formats
pub struct AuditFormatter {
    json_serializer: serde_json::Serializer<Vec<u8>>,
}

impl AuditFormatter {
    /// Create a new audit formatter
    pub fn new() -> Self {
        Self {
            json_serializer: serde_json::Serializer::new(Vec::new()),
        }
    }

    /// Format events in the specified format
    pub fn format_events(&self, events: &[AuditEvent], format: ReportFormat) -> Result<String> {
        match format {
            ReportFormat::Json => self.format_json(events),
            ReportFormat::Csv => self.format_csv(events),
            ReportFormat::Text => self.format_text(events),
            ReportFormat::Summary => self.format_summary(events),
        }
    }

    /// Format events as JSON
    fn format_json(&self, events: &[AuditEvent]) -> Result<String> {
        serde_json::to_string_pretty(events).map_err(|e| {
            EnvCliError::AuditError(format!("JSON serialization failed: {}", e)).into()
        })
    }

    /// Format events as CSV
    fn format_csv(&self, events: &[AuditEvent]) -> Result<String> {
        let mut output = String::new();
        output.push_str("timestamp,user_id,action,resource_type,outcome,severity,category\n");

        for event in events {
            output.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                event.user_id,
                event.action,
                event.resource_type,
                match event.outcome {
                    Outcome::Success => "success",
                    Outcome::Failure => "failure",
                    Outcome::Denied => "denied",
                    Outcome::Error(_) => "error",
                },
                format!("{:?}", event.severity).to_lowercase(),
                format!("{:?}", event.category).to_lowercase(),
            ));
        }

        Ok(output)
    }

    /// Format events as readable text
    fn format_text(&self, events: &[AuditEvent]) -> Result<String> {
        let mut output = String::new();
        output.push_str("Audit Log Report\n");
        output.push_str("================\n\n");

        for event in events {
            output.push_str(&format!(
                "Time: {}\nUser: {}\nAction: {}\nResource: {}\nOutcome: {}\nSeverity: {}\nCategory: {}\n\n",
                event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                event.user_id,
                event.action,
                event.resource_type,
                match event.outcome {
                    Outcome::Success => "success",
                    Outcome::Failure => "failure",
                    Outcome::Denied => "denied",
                    Outcome::Error(_) => "error",
                },
                format!("{:?}", event.severity).to_lowercase(),
                format!("{:?}", event.category).to_lowercase(),
            ));
        }

        Ok(output)
    }

    /// Format events as summary report
    fn format_summary(&self, events: &[AuditEvent]) -> Result<String> {
        let mut output = String::new();
        output.push_str("Audit Summary Report\n");
        output.push_str("===================\n\n");

        output.push_str(&format!("Total Events: {}\n\n", events.len()));

        // Count by category
        let mut category_counts = HashMap::new();
        for event in events {
            *category_counts.entry(event.category.clone()).or_insert(0) += 1;
        }

        output.push_str("Events by Category:\n");
        for (category, count) in &category_counts {
            output.push_str(&format!("  {:?}: {}\n", category, count));
        }

        output.push_str("\nEvents by Severity:\n");
        let mut severity_counts = HashMap::new();
        for event in events {
            *severity_counts.entry(event.severity.clone()).or_insert(0) += 1;
        }

        for (severity, count) in &severity_counts {
            output.push_str(&format!("  {:?}: {}\n", severity, count));
        }

        Ok(output)
    }
}

/// Report format options
#[derive(Debug, Clone)]
pub enum ReportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Human-readable text format
    Text,
    /// Summary format
    Summary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_event_creation() {
        let user_id = Uuid::new_v4();
        let event = AuditEvent::new(
            user_id,
            "create_environment".to_string(),
            "environment".to_string(),
            Outcome::Success,
        )
        .with_workspace(Uuid::new_v4())
        .with_severity(AuditSeverity::Info)
        .with_category(AuditCategory::Environment);

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.action, "create_environment");
        assert_eq!(event.resource_type, "environment");
        assert_eq!(event.severity, AuditSeverity::Info);
        assert_eq!(event.category, AuditCategory::Environment);
    }

    #[tokio::test]
    async fn test_audit_logger() {
        let config = AuditConfig::default();
        let logger = AuditLogger::new(&config).await.unwrap();

        let user_id = Uuid::new_v4();
        logger
            .log_success(
                user_id,
                "create_environment".to_string(),
                "environment".to_string(),
                None,
            )
            .await
            .unwrap();

        let query = AuditQuery::default();
        let events = logger.query_events(&query).await.unwrap();
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn test_audit_storage() {
        let storage = InMemoryAuditStorage::new();
        let user_id = Uuid::new_v4();
        let event = AuditEvent::new(
            user_id,
            "test_action".to_string(),
            "test_resource".to_string(),
            Outcome::Success,
        );

        storage.store_event(&event).await.unwrap();

        let query = AuditQuery::default();
        let events = storage.query_events(&query).await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].action, "test_action");
    }

    #[test]
    fn test_severity_levels() {
        assert!(AuditSeverity::Critical.is_higher_than(&AuditSeverity::Error));
        assert!(AuditSeverity::Error.is_higher_than(&AuditSeverity::Warning));
        assert!(AuditSeverity::Warning.is_higher_than(&AuditSeverity::Info));
        assert!(AuditSeverity::Info.is_higher_than(&AuditSeverity::Debug));
        assert!(!AuditSeverity::Debug.is_higher_than(&AuditSeverity::Info));
    }
}
