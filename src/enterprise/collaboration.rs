//! # Team Collaboration Module
//!
//! This module implements multi-user environment sharing, conflict resolution,
//! and collaborative features for enterprise teams.

use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;
use crate::enterprise::{UserId, WorkspaceId, EnvironmentId, EncryptedValue, Outcome};
use crate::error::EnvCliError;

/// Team workspace for collaborative environment management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamWorkspace {
    /// Unique workspace identifier
    pub id: WorkspaceId,
    /// Workspace name
    pub name: String,
    /// Workspace description
    pub description: String,
    /// Workspace members
    pub members: Vec<TeamMember>,
    /// Shared environments
    pub environments: Vec<SharedEnvironment>,
    /// Workspace settings
    pub settings: WorkspaceSettings,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Created by user
    pub created_by: UserId,
}

impl TeamWorkspace {
    /// Create a new team workspace
    pub fn new(
        name: String,
        description: String,
        created_by: UserId,
    ) -> Self {
        let workspace_id = Uuid::new_v4();
        let now = Utc::now();

        Self {
            id: workspace_id,
            name,
            description,
            members: vec![],
            environments: vec![],
            settings: WorkspaceSettings::default(),
            created_at: now,
            updated_at: now,
            created_by,
        }
    }

    /// Add a member to the workspace
    pub fn add_member(&mut self, member: TeamMember) {
        self.members.push(member);
        self.updated_at = Utc::now();
    }

    /// Remove a member from the workspace
    pub fn remove_member(&mut self, user_id: UserId) -> Result<()> {
        let initial_len = self.members.len();
        self.members.retain(|m| m.user_id != user_id);

        if self.members.len() == initial_len {
            return Err(EnvCliError::CollaborationError(
                format!("User {} is not a member of this workspace", user_id)
            ).into());
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get member by user ID
    pub fn get_member(&self, user_id: UserId) -> Option<&TeamMember> {
        self.members.iter().find(|m| m.user_id == user_id)
    }

    /// Check if a user is a member of the workspace
    pub fn is_member(&self, user_id: UserId) -> bool {
        self.members.iter().any(|m| m.user_id == user_id)
    }

    /// Add an environment to the workspace
    pub fn add_environment(&mut self, environment: SharedEnvironment) {
        self.environments.push(environment);
        self.updated_at = Utc::now();
    }

    /// Remove an environment from the workspace
    pub fn remove_environment(&mut self, environment_id: EnvironmentId) -> Result<()> {
        let initial_len = self.environments.len();
        self.environments.retain(|e| e.id != environment_id);

        if self.environments.len() == initial_len {
            return Err(EnvCliError::CollaborationError(
                format!("Environment {} is not in this workspace", environment_id)
            ).into());
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get environment by ID
    pub fn get_environment(&self, environment_id: EnvironmentId) -> Option<&SharedEnvironment> {
        self.environments.iter().find(|e| e.id == environment_id)
    }

    /// Update workspace settings
    pub fn update_settings(&mut self, settings: WorkspaceSettings) {
        self.settings = settings;
        self.updated_at = Utc::now();
    }
}

/// Team member information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    /// User ID
    pub user_id: UserId,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// Display name
    pub display_name: String,
    /// Member role in the workspace
    pub role: WorkspaceRole,
    /// Permissions for this member
    pub permissions: Vec<WorkspacePermission>,
    /// Joined timestamp
    pub joined_at: DateTime<Utc>,
    /// Last active timestamp
    pub last_active_at: DateTime<Utc>,
}

impl TeamMember {
    /// Create a new team member
    pub fn new(
        user_id: UserId,
        username: String,
        email: String,
        display_name: String,
        role: WorkspaceRole,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            username,
            email,
            display_name,
            role,
            permissions: role.default_permissions(),
            joined_at: now,
            last_active_at: now,
        }
    }

    /// Update last active timestamp
    pub fn update_last_active(&mut self) {
        self.last_active_at = Utc::now();
    }

    /// Check if the member has a specific permission
    pub fn has_permission(&self, permission: WorkspacePermission) -> bool {
        self.permissions.contains(&permission)
    }

    /// Add a permission to the member
    pub fn add_permission(&mut self, permission: WorkspacePermission) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    /// Remove a permission from the member
    pub fn remove_permission(&mut self, permission: &WorkspacePermission) {
        self.permissions.retain(|p| p != permission);
    }
}

/// Workspace role for team members
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkspaceRole {
    /// Owner with full control
    Owner,
    /// Admin with management permissions
    Admin,
    /// Editor who can modify environments
    Editor,
    /// Viewer with read-only access
    Viewer,
    /// Guest with limited access
    Guest,
}

impl WorkspaceRole {
    /// Get default permissions for this role
    pub fn default_permissions(&self) -> Vec<WorkspacePermission> {
        match self {
            WorkspaceRole::Owner => vec![
                WorkspacePermission::ManageWorkspace,
                WorkspacePermission::ManageMembers,
                WorkspacePermission::ManageEnvironments,
                WorkspacePermission::DeleteEnvironment,
                WorkspacePermission::ViewAuditLogs,
                WorkspacePermission::ManageSettings,
            ],
            WorkspaceRole::Admin => vec![
                WorkspacePermission::ManageMembers,
                WorkspacePermission::ManageEnvironments,
                WorkspacePermission::DeleteEnvironment,
                WorkspacePermission::ViewAuditLogs,
            ],
            WorkspaceRole::Editor => vec![
                WorkspacePermission::ManageEnvironments,
                WorkspacePermission::ViewAuditLogs,
            ],
            WorkspaceRole::Viewer => vec![
                WorkspacePermission::ViewEnvironments,
            ],
            WorkspaceRole::Guest => vec![
                WorkspacePermission::ViewEnvironments,
            ],
        }
    }

    /// Get the hierarchy level for the role
    pub fn hierarchy_level(&self) -> u8 {
        match self {
            WorkspaceRole::Owner => 100,
            WorkspaceRole::Admin => 80,
            WorkspaceRole::Editor => 60,
            WorkspaceRole::Viewer => 40,
            WorkspaceRole::Guest => 20,
        }
    }
}

/// Workspace permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkspacePermission {
    /// Manage workspace settings
    ManageWorkspace,
    /// Manage workspace members
    ManageMembers,
    /// Manage environments
    ManageEnvironments,
    /// Delete environments
    DeleteEnvironment,
    /// View environments
    ViewEnvironments,
    /// View audit logs
    ViewAuditLogs,
    /// Manage workspace settings
    ManageSettings,
}

/// Shared environment for team collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedEnvironment {
    /// Unique environment identifier
    pub id: EnvironmentId,
    /// Environment name
    pub name: String,
    /// Environment description
    pub description: String,
    /// Encrypted environment variables
    pub encrypted_variables: HashMap<String, EncryptedValue>,
    /// Environment owners
    pub owners: Vec<UserId>,
    /// Environment editors
    pub editors: Vec<UserId>,
    /// Environment viewers
    pub viewers: Vec<UserId>,
    /// Environment metadata
    pub metadata: EnvironmentMetadata,
    /// Change history
    pub change_history: Vec<EnvironmentChange>,
    /// Pending changes requiring approval
    pub pending_changes: Vec<PendingChange>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Created by user
    pub created_by: UserId,
}

impl SharedEnvironment {
    /// Create a new shared environment
    pub fn new(
        name: String,
        description: String,
        created_by: UserId,
    ) -> Self {
        let env_id = Uuid::new_v4();
        let now = Utc::now();

        Self {
            id: env_id,
            name,
            description,
            encrypted_variables: HashMap::new(),
            owners: vec![created_by],
            editors: vec![],
            viewers: vec![],
            metadata: EnvironmentMetadata::default(),
            change_history: vec![],
            pending_changes: vec![],
            created_at: now,
            updated_at: now,
            created_by,
        }
    }

    /// Add an owner to the environment
    pub fn add_owner(&mut self, user_id: UserId) {
        if !self.owners.contains(&user_id) {
            self.owners.push(user_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove an owner from the environment
    pub fn remove_owner(&mut self, user_id: UserId) {
        self.owners.retain(|id| id != user_id);
        self.updated_at = Utc::now();
    }

    /// Add an editor to the environment
    pub fn add_editor(&mut self, user_id: UserId) {
        if !self.editors.contains(&user_id) {
            self.editors.push(user_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove an editor from the environment
    pub fn remove_editor(&mut self, user_id: UserId) {
        self.editors.retain(|id| id != user_id);
        self.updated_at = Utc::now();
    }

    /// Add a viewer to the environment
    pub fn add_viewer(&mut self, user_id: UserId) {
        if !self.viewers.contains(&user_id) {
            self.viewers.push(user_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove a viewer from the environment
    pub fn remove_viewer(&mut self, user_id: UserId) {
        self.viewers.retain(|id| id != user_id);
        self.updated_at = Utc::now();
    }

    /// Check if a user has write access to the environment
    pub fn can_write(&self, user_id: UserId) -> bool {
        self.owners.contains(&user_id) || self.editors.contains(&user_id)
    }

    /// Check if a user has read access to the environment
    pub fn can_read(&self, user_id: UserId) -> bool {
        self.owners.contains(&user_id) ||
        self.editors.contains(&user_id) ||
        self.viewers.contains(&user_id)
    }

    /// Add a variable change to the history
    pub fn add_change(&mut self, change: EnvironmentChange) {
        self.change_history.push(change);
        self.updated_at = Utc::now();
    }

    /// Add a pending change requiring approval
    pub fn add_pending_change(&mut self, pending_change: PendingChange) {
        self.pending_changes.push(pending_change);
        self.updated_at = Utc::now();
    }

    /// Approve a pending change
    pub fn approve_pending_change(&mut self, change_id: Uuid, approved_by: UserId) -> Result<()> {
        let change_index = self.pending_changes.iter()
            .position(|c| c.id == change_id)
            .ok_or_else(|| EnvCliError::CollaborationError(
                format!("Pending change {} not found", change_id)
            ))?;

        let mut pending_change = self.pending_changes.remove(change_index);
        pending_change.status = ChangeStatus::Approved;
        pending_change.approved_by = Some(approved_by);
        pending_change.approved_at = Some(Utc::now());

        // Apply the change
        self.apply_change(&pending_change.change)?;

        // Add to history
        let history_change = EnvironmentChange {
            id: Uuid::new_v4(),
            environment_id: self.id,
            timestamp: Utc::now(),
            author: pending_change.requested_by,
            change: pending_change.change.clone(),
            approval: Some(ApprovalInfo {
                approved_by,
                approved_at: Utc::now(),
                comments: pending_change.comments.clone(),
            }),
        };
        self.add_change(history_change);

        Ok(())
    }

    /// Reject a pending change
    pub fn reject_pending_change(&mut self, change_id: Uuid, rejected_by: UserId, reason: String) -> Result<()> {
        let change_index = self.pending_changes.iter()
            .position(|c| c.id == change_id)
            .ok_or_else(|| EnvCliError::CollaborationError(
                format!("Pending change {} not found", change_id)
            ))?;

        let mut pending_change = self.pending_changes.remove(change_index);
        pending_change.status = ChangeStatus::Rejected;
        pending_change.comments = Some(reason);

        Ok(())
    }

    /// Apply a change to the environment
    fn apply_change(&mut self, change: &VariableChange) -> Result<()> {
        match change {
            VariableChange::Add { key, value } => {
                // In a real implementation, this would encrypt the value
                // For now, we'll store it as-is (this is a placeholder)
                self.encrypted_variables.insert(key.clone(), EncryptedValue {
                    ciphertext: base64::encode(value.as_bytes()),
                    nonce: "dummy_nonce".to_string(),
                    tag: "dummy_tag".to_string(),
                    key_id: "dummy_key".to_string(),
                    algorithm: "AES-256-GCM".to_string(),
                    encrypted_at: Utc::now(),
                });
            }
            VariableChange::Update { key, new_value, .. } => {
                // Update encrypted value
                self.encrypted_variables.insert(key.clone(), EncryptedValue {
                    ciphertext: base64::encode(new_value.as_bytes()),
                    nonce: "dummy_nonce".to_string(),
                    tag: "dummy_tag".to_string(),
                    key_id: "dummy_key".to_string(),
                    algorithm: "AES-256-GCM".to_string(),
                    encrypted_at: Utc::now(),
                });
            }
            VariableChange::Delete { key } => {
                self.encrypted_variables.remove(key);
            }
        }
        Ok(())
    }
}

/// Environment metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMetadata {
    /// Environment type
    pub environment_type: String,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Is this environment locked for changes
    pub is_locked: bool,
    /// Lock reason (if locked)
    pub lock_reason: Option<String>,
    /// Locked by user
    pub locked_by: Option<UserId>,
    /// Lock timestamp
    pub locked_at: Option<DateTime<Utc>>,
    /// Environment version
    pub version: u32,
}

impl Default for EnvironmentMetadata {
    fn default() -> Self {
        Self {
            environment_type: "development".to_string(),
            tags: vec![],
            is_locked: false,
            lock_reason: None,
            locked_by: None,
            locked_at: None,
            version: 1,
        }
    }
}

/// Variable change definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableChange {
    /// Add a new variable
    Add {
        key: String,
        value: String,
    },
    /// Update an existing variable
    Update {
        key: String,
        old_value: String,
        new_value: String,
    },
    /// Delete a variable
    Delete {
        key: String,
    },
}

impl VariableChange {
    /// Get the key for this change
    pub fn key(&self) -> &str {
        match self {
            VariableChange::Add { key, .. } => key,
            VariableChange::Update { key, .. } => key,
            VariableChange::Delete { key } => key,
        }
    }
}

/// Environment change with approval information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentChange {
    /// Unique change identifier
    pub id: Uuid,
    /// Environment ID
    pub environment_id: EnvironmentId,
    /// Change timestamp
    pub timestamp: DateTime<Utc>,
    /// Author of the change
    pub author: UserId,
    /// The actual change
    pub change: VariableChange,
    /// Approval information
    pub approval: Option<ApprovalInfo>,
}

/// Approval information for changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalInfo {
    /// Approved by user
    pub approved_by: UserId,
    /// Approval timestamp
    pub approved_at: DateTime<Utc>,
    /// Approval comments
    pub comments: Option<String>,
}

/// Pending change requiring approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingChange {
    /// Unique change identifier
    pub id: Uuid,
    /// Requested by user
    pub requested_by: UserId,
    /// The requested change
    pub change: VariableChange,
    /// Change status
    pub status: ChangeStatus,
    /// Comments or reason
    pub comments: Option<String>,
    /// Request timestamp
    pub requested_at: DateTime<Utc>,
    /// When was it approved/rejected
    pub approved_at: Option<DateTime<Utc>>,
    /// Who approved/rejected it
    pub approved_by: Option<UserId>,
}

/// Change status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeStatus {
    /// Pending approval
    Pending,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
}

/// Workspace settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    /// Require approval for environment changes
    pub require_approval: bool,
    /// Minimum number of approvers
    pub min_approvers: u32,
    /// Auto-approval for certain roles
    pub auto_approval_roles: Vec<WorkspaceRole>,
    /// Conflict resolution strategy
    pub conflict_resolution: ConflictResolutionStrategy,
    /// Environment backup settings
    pub backup_settings: BackupSettings,
    /// Notification settings
    pub notification_settings: NotificationSettings,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            require_approval: false,
            min_approvers: 1,
            auto_approval_roles: vec![WorkspaceRole::Owner, WorkspaceRole::Admin],
            conflict_resolution: ConflictResolutionStrategy::LastWriterWins,
            backup_settings: BackupSettings::default(),
            notification_settings: NotificationSettings::default(),
        }
    }
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// Last writer wins
    LastWriterWins,
    /// Manual resolution required
    Manual,
    /// Merge with conflict markers
    MergeWithConflictMarkers,
    /// Automatic based on timestamp
    AutomaticBasedOnTimestamp,
}

/// Backup settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSettings {
    /// Enable automatic backups
    pub enabled: bool,
    /// Backup frequency in hours
    pub frequency_hours: u32,
    /// Number of backups to retain
    pub retain_count: u32,
}

impl Default for BackupSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency_hours: 24,
            retain_count: 30,
        }
    }
}

/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    /// Enable notifications
    pub enabled: bool,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
    /// Notification events
    pub events: Vec<NotificationEvent>,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: vec![NotificationChannel::Email],
            events: vec![NotificationEvent::EnvironmentChanged, NotificationEvent::MemberAdded],
        }
    }
}

/// Notification channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notifications
    Email,
    /// Slack notifications
    Slack,
    /// Webhook notifications
    Webhook(String),
    /// In-app notifications
    InApp,
}

/// Notification events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationEvent {
    /// Environment was changed
    EnvironmentChanged,
    /// Member was added to workspace
    MemberAdded,
    /// Member was removed from workspace
    MemberRemoved,
    /// Pending change requires approval
    PendingChange,
    /// Backup completed
    BackupCompleted,
}

/// Conflict resolution engine
pub struct ConflictResolver {
    /// Resolution strategy
    strategy: ConflictResolutionStrategy,
}

impl ConflictResolver {
    /// Create a new conflict resolver
    pub fn new(strategy: ConflictResolutionStrategy) -> Self {
        Self { strategy }
    }

    /// Resolve conflicts between changes
    pub async fn resolve_conflicts(
        &self,
        local_changes: &[EnvironmentChange],
        remote_changes: &[EnvironmentChange],
    ) -> Result<Vec<EnvironmentChange>> {
        match self.strategy {
            ConflictResolutionStrategy::LastWriterWins => {
                self.resolve_by_timestamp(local_changes, remote_changes).await
            }
            ConflictResolutionStrategy::Manual => {
                // Return conflicts for manual resolution
                let mut conflicts = Vec::new();
                conflicts.extend_from_slice(local_changes);
                conflicts.extend_from_slice(remote_changes);
                Ok(conflicts)
            }
            ConflictResolutionStrategy::MergeWithConflictMarkers => {
                self.resolve_with_markers(local_changes, remote_changes).await
            }
            ConflictResolutionStrategy::AutomaticBasedOnTimestamp => {
                self.resolve_by_timestamp(local_changes, remote_changes).await
            }
        }
    }

    /// Resolve conflicts by timestamp (last writer wins)
    async fn resolve_by_timestamp(
        &self,
        local_changes: &[EnvironmentChange],
        remote_changes: &[EnvironmentChange],
    ) -> Result<Vec<EnvironmentChange>> {
        let mut all_changes = Vec::new();
        all_changes.extend_from_slice(local_changes);
        all_changes.extend_from_slice(remote_changes);

        // Sort by timestamp (newest first)
        all_changes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Keep only the latest change for each variable
        let mut resolved_changes = Vec::new();
        let mut seen_keys = HashSet::new();

        for change in all_changes {
            let key = change.change.key();
            if !seen_keys.contains(key) {
                seen_keys.insert(key.to_string());
                resolved_changes.push(change.clone());
            }
        }

        Ok(resolved_changes)
    }

    /// Resolve conflicts with conflict markers
    async fn resolve_with_markers(
        &self,
        local_changes: &[EnvironmentChange],
        remote_changes: &[EnvironmentChange],
    ) -> Result<Vec<EnvironmentChange>> {
        // For now, just return local changes (placeholder implementation)
        // In a real implementation, this would create conflict markers
        Ok(local_changes.to_vec())
    }
}