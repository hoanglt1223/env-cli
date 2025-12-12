//! # Role-Based Access Control (RBAC) Module
//!
//! This module implements a comprehensive RBAC system for enterprise
//! access control with granular permissions and roles.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;
use crate::enterprise::{UserId, Resource, Action, Effect};
use crate::error::EnvCliError;

/// User roles with hierarchical permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Role {
    /// Super administrator with all permissions
    Owner,
    /// Administrative user with workspace management
    Admin,
    /// Editor who can modify environments
    Editor,
    /// Viewer with read-only access
    Viewer,
    /// Auditor with audit log access
    Auditor,
    /// Custom role with specific permissions
    Custom(String),
}

impl Role {
    /// Get the hierarchy level for the role (higher = more privileges)
    pub fn hierarchy_level(&self) -> u8 {
        match self {
            Role::Owner => 100,
            Role::Admin => 80,
            Role::Editor => 60,
            Role::Auditor => 50,
            Role::Viewer => 30,
            Role::Custom(_) => 40,
        }
    }

    /// Check if this role has higher or equal privileges than another
    pub fn has_higher_or_equal_privileges(&self, other: &Role) -> bool {
        self.hierarchy_level() >= other.hierarchy_level()
    }

    /// Get default permissions for this role
    pub fn default_permissions(&self) -> Vec<Permission> {
        match self {
            Role::Owner => vec![
                Permission::new(Resource::Workspace, Action::Admin),
                Permission::new(Resource::User, Action::Admin),
                Permission::new(Resource::Environment, Action::Admin),
                Permission::new(Resource::Secret, Action::Admin),
                Permission::new(Resource::AuditLog, Action::Audit),
                Permission::new(Resource::Config, Action::Admin),
            ],
            Role::Admin => vec![
                Permission::new(Resource::Workspace, Action::Write),
                Permission::new(Resource::User, Action::Write),
                Permission::new(Resource::Environment, Action::Write),
                Permission::new(Resource::Secret, Action::Write),
                Permission::new(Resource::AuditLog, Action::Read),
                Permission::new(Resource::Config, Action::Write),
            ],
            Role::Editor => vec![
                Permission::new(Resource::Environment, Action::Write),
                Permission::new(Resource::Secret, Action::Write),
                Permission::new(Resource::Workspace, Action::Read),
                Permission::new(Resource::Config, Action::Read),
            ],
            Role::Viewer => vec![
                Permission::new(Resource::Environment, Action::Read),
                Permission::new(Resource::Workspace, Action::Read),
                Permission::new(Resource::Config, Action::Read),
            ],
            Role::Auditor => vec![
                Permission::new(Resource::AuditLog, Action::Audit),
                Permission::new(Resource::Workspace, Action::Read),
                Permission::new(Resource::Environment, Action::Read),
            ],
            Role::Custom(_) => vec![], // Custom roles need explicit permissions
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Owner => write!(f, "owner"),
            Role::Admin => write!(f, "admin"),
            Role::Editor => write!(f, "editor"),
            Role::Viewer => write!(f, "viewer"),
            Role::Auditor => write!(f, "auditor"),
            Role::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = EnvCliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owner" => Ok(Role::Owner),
            "admin" => Ok(Role::Admin),
            "editor" => Ok(Role::Editor),
            "viewer" => Ok(Role::Viewer),
            "auditor" => Ok(Role::Auditor),
            custom => Ok(Role::Custom(custom.to_string())),
        }
    }
}

/// Permission definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission {
    /// Resource the permission applies to
    pub resource: Resource,
    /// Action that can be performed
    pub action: Action,
    /// Effect of the permission (allow/deny)
    pub effect: Effect,
    /// Optional conditions for the permission
    pub conditions: Vec<String>,
}

impl Permission {
    /// Create a new permission with allow effect
    pub fn new(resource: Resource, action: Action) -> Self {
        Self {
            resource,
            action,
            effect: Effect::Allow,
            conditions: vec![],
        }
    }

    /// Create a permission with custom effect
    pub fn with_effect(resource: Resource, action: Action, effect: Effect) -> Self {
        Self {
            resource,
            action,
            effect,
            conditions: vec![],
        }
    }

    /// Add a condition to the permission
    pub fn with_condition(mut self, condition: String) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Get permission key for storage
    pub fn key(&self) -> String {
        format!("{:?}:{:?}", self.resource, self.action)
    }
}

/// Permission matrix that maps roles to permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMatrix {
    /// Matrix of (role, resource) -> set of actions
    matrix: HashMap<(Role, Resource), HashSet<Action>>,
    /// Role hierarchy for inheritance
    hierarchy: HashMap<Role, Vec<Role>>,
    /// Deny rules that override allow rules
    deny_rules: HashSet<Permission>,
}

impl PermissionMatrix {
    /// Create a new permission matrix with default roles
    pub fn new() -> Self {
        let mut matrix = Self {
            matrix: HashMap::new(),
            hierarchy: HashMap::new(),
            deny_rules: HashSet::new(),
        };

        // Initialize default permissions
        matrix.initialize_default_permissions();
        matrix.setup_role_hierarchy();
        matrix
    }

    /// Initialize default permissions for standard roles
    fn initialize_default_permissions(&mut self) {
        for role in [Role::Owner, Role::Admin, Role::Editor, Role::Viewer, Role::Auditor] {
            for permission in role.default_permissions() {
                self.add_permission(&role, permission);
            }
        }
    }

    /// Setup role hierarchy for permission inheritance
    fn setup_role_hierarchy(&mut self) {
        self.hierarchy.insert(Role::Owner, vec![Role::Admin, Role::Editor, Role::Viewer, Role::Auditor]);
        self.hierarchy.insert(Role::Admin, vec![Role::Editor, Role::Viewer]);
        self.hierarchy.insert(Role::Editor, vec![Role::Viewer]);
        self.hierarchy.insert(Role::Viewer, vec![]);
        self.hierarchy.insert(Role::Auditor, vec![]);
    }

    /// Add a permission for a role
    pub fn add_permission(&mut self, role: &Role, permission: Permission) {
        let key = (role.clone(), permission.resource.clone());
        let actions = self.matrix.entry(key).or_insert_with(HashSet::new);

        match permission.effect {
            Effect::Allow => {
                actions.insert(permission.action);
            }
            Effect::Deny => {
                self.deny_rules.insert(permission);
            }
        }
    }

    /// Check if a role has permission for a specific action on a resource
    pub fn can_access(&self, role: &Role, resource: &Resource, action: &Action) -> bool {
        // First check deny rules (they have highest priority)
        if self.is_denied(role, resource, action) {
            return false;
        }

        // Check direct permission
        if self.has_direct_permission(role, resource, action) {
            return true;
        }

        // Check inherited permissions
        self.has_inherited_permission(role, resource, action)
    }

    /// Check if a permission is explicitly denied
    fn is_denied(&self, role: &Role, resource: &Resource, action: &Action) -> bool {
        self.deny_rules.iter().any(|perm| {
            perm.resource == *resource && perm.action == *action
        })
    }

    /// Check if a role has direct permission
    fn has_direct_permission(&self, role: &Role, resource: &Resource, action: &Action) -> bool {
        self.matrix
            .get(&(role.clone(), resource.clone()))
            .map(|actions| actions.contains(action))
            .unwrap_or(false)
    }

    /// Check if a role has inherited permission from higher roles
    fn has_inherited_permission(&self, role: &Role, resource: &Resource, action: &Action) -> bool {
        if let Some(lower_roles) = self.hierarchy.get(role) {
            for lower_role in lower_roles {
                if self.has_direct_permission(lower_role, resource, action) {
                    return true;
                }
            }
        }
        false
    }

    /// Get all permissions for a role
    pub fn get_permissions(&self, role: &Role) -> Vec<Permission> {
        let mut permissions = Vec::new();

        for ((r, resource), actions) in &self.matrix {
            if r == role {
                for action in actions {
                    permissions.push(Permission::new(resource.clone(), action.clone()));
                }
            }
        }

        permissions
    }

    /// Remove a permission from a role
    pub fn remove_permission(&mut self, role: &Role, resource: &Resource, action: &Action) {
        let key = (role.clone(), resource.clone());
        if let Some(actions) = self.matrix.get_mut(&key) {
            actions.remove(action);
        }
    }

    /// Add a deny rule
    pub fn add_deny_rule(&mut self, permission: Permission) {
        self.deny_rules.insert(permission);
    }

    /// Remove a deny rule
    pub fn remove_deny_rule(&mut self, permission: &Permission) {
        self.deny_rules.remove(permission);
    }
}

impl Default for PermissionMatrix {
    fn default() -> Self {
        Self::new()
    }
}

/// User assignment with roles and permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAssignment {
    /// User ID
    pub user_id: UserId,
    /// Assigned roles
    pub roles: HashSet<Role>,
    /// Additional permissions (beyond role-based)
    pub additional_permissions: HashSet<Permission>,
    /// Permissions that are explicitly denied
    pub denied_permissions: HashSet<Permission>,
    /// Assignment timestamp
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    /// Expiration timestamp (optional)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserAssignment {
    /// Create a new user assignment
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            roles: HashSet::new(),
            additional_permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
            assigned_at: chrono::Utc::now(),
            expires_at: None,
        }
    }

    /// Add a role to the user
    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role);
    }

    /// Remove a role from the user
    pub fn remove_role(&mut self, role: &Role) {
        self.roles.remove(role);
    }

    /// Add an additional permission
    pub fn add_permission(&mut self, permission: Permission) {
        self.additional_permissions.insert(permission);
    }

    /// Remove an additional permission
    pub fn remove_permission(&mut self, permission: &Permission) {
        self.additional_permissions.remove(permission);
    }

    /// Add a denied permission
    pub fn deny_permission(&mut self, permission: Permission) {
        self.denied_permissions.insert(permission);
    }

    /// Remove a denied permission
    pub fn undeny_permission(&mut self, permission: &Permission) {
        self.denied_permissions.remove(permission);
    }

    /// Check if the assignment is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }
}

/// RBAC engine that manages permissions and access control
pub struct RbacEngine {
    /// Permission matrix
    permission_matrix: PermissionMatrix,
    /// User assignments
    user_assignments: HashMap<UserId, UserAssignment>,
    /// Resource-specific permissions
    resource_permissions: HashMap<String, HashMap<UserId, HashSet<Permission>>>,
}

impl RbacEngine {
    /// Create a new RBAC engine
    pub fn new() -> Self {
        Self {
            permission_matrix: PermissionMatrix::new(),
            user_assignments: HashMap::new(),
            resource_permissions: HashMap::new(),
        }
    }

    /// Create an RBAC engine with custom permission matrix
    pub fn with_matrix(permission_matrix: PermissionMatrix) -> Self {
        Self {
            permission_matrix,
            user_assignments: HashMap::new(),
            resource_permissions: HashMap::new(),
        }
    }

    /// Assign a role to a user
    pub fn assign_role(&mut self, user_id: UserId, role: Role) {
        let assignment = self.user_assignments
            .entry(user_id)
            .or_insert_with(UserAssignment::new);
        assignment.add_role(role);
    }

    /// Remove a role from a user
    pub fn remove_role(&mut self, user_id: UserId, role: &Role) {
        if let Some(assignment) = self.user_assignments.get_mut(&user_id) {
            assignment.remove_role(role);
        }
    }

    /// Check if a user has permission for an action on a resource
    pub fn check_permission(&self, user_id: UserId, resource: &Resource, action: &Action) -> bool {
        let assignment = match self.user_assignments.get(&user_id) {
            Some(assignment) => assignment,
            None => return false,
        };

        // Check if assignment is expired
        if assignment.is_expired() {
            return false;
        }

        // Check explicitly denied permissions first
        if assignment.denied_permissions.iter().any(|perm| {
            perm.resource == *resource && perm.action == *action
        }) {
            return false;
        }

        // Check additional permissions
        if assignment.additional_permissions.iter().any(|perm| {
            perm.resource == *resource && perm.action == *action
        }) {
            return true;
        }

        // Check role-based permissions
        for role in &assignment.roles {
            if self.permission_matrix.can_access(role, resource, action) {
                return true;
            }
        }

        // Check resource-specific permissions
        if let Some(resource_perms) = self.resource_permissions.get(&self.resource_key(resource)) {
            if let Some(user_perms) = resource_perms.get(&user_id) {
                if user_perms.iter().any(|perm| {
                    perm.resource == *resource && perm.action == *action
                }) {
                    return true;
                }
            }
        }

        false
    }

    /// Grant a specific permission to a user on a resource
    pub fn grant_permission(&mut self, user_id: UserId, resource_id: &str, permission: Permission) {
        let resource_perms = self.resource_permissions
            .entry(resource_id.to_string())
            .or_insert_with(HashMap::new);
        let user_perms = resource_perms.entry(user_id).or_insert_with(HashSet::new);
        user_perms.insert(permission);
    }

    /// Revoke a specific permission from a user on a resource
    pub fn revoke_permission(&mut self, user_id: UserId, resource_id: &str, permission: &Permission) {
        if let Some(resource_perms) = self.resource_permissions.get_mut(resource_id) {
            if let Some(user_perms) = resource_perms.get_mut(&user_id) {
                user_perms.remove(permission);
            }
        }
    }

    /// Get user roles
    pub fn get_user_roles(&self, user_id: UserId) -> Vec<Role> {
        self.user_assignments
            .get(&user_id)
            .map(|assignment| assignment.roles.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all users with a specific role
    pub fn get_users_with_role(&self, role: &Role) -> Vec<UserId> {
        self.user_assignments
            .iter()
            .filter(|(_, assignment)| assignment.roles.contains(role))
            .map(|(user_id, _)| *user_id)
            .collect()
    }

    /// Get the permission matrix reference
    pub fn permission_matrix(&self) -> &PermissionMatrix {
        &self.permission_matrix
    }

    /// Get mutable permission matrix reference
    pub fn permission_matrix_mut(&mut self) -> &mut PermissionMatrix {
        &mut self.permission_matrix
    }

    /// Convert resource to key string
    fn resource_key(&self, resource: &Resource) -> String {
        match resource {
            Resource::Environment => "environment".to_string(),
            Resource::Workspace => "workspace".to_string(),
            Resource::User => "user".to_string(),
            Resource::AuditLog => "audit_log".to_string(),
            Resource::Config => "config".to_string(),
            Resource::Secret => "secret".to_string(),
        }
    }

    /// Validate user assignment
    pub fn validate_assignment(&self, user_id: UserId) -> Result<()> {
        let assignment = self.user_assignments.get(&user_id)
            .ok_or_else(|| EnvCliError::RbacError("User assignment not found".to_string()))?;

        if assignment.is_expired() {
            return Err(EnvCliError::RbacError("User assignment has expired".to_string()).into());
        }

        Ok(())
    }
}

impl Default for RbacEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        assert!(Role::Owner.has_higher_or_equal_privileges(&Role::Admin));
        assert!(Role::Admin.has_higher_or_equal_privileges(&Role::Editor));
        assert!(Role::Editor.has_higher_or_equal_privileges(&Role::Viewer));
        assert!(!Role::Viewer.has_higher_or_equal_privileges(&Role::Editor));
    }

    #[test]
    fn test_permission_matrix() {
        let matrix = PermissionMatrix::new();

        // Test that owner can do admin actions
        assert!(matrix.can_access(&Role::Owner, &Resource::Workspace, &Action::Admin));
        assert!(matrix.can_access(&Role::Owner, &Resource::Environment, &Action::Write));

        // Test that viewer can only read
        assert!(matrix.can_access(&Role::Viewer, &Resource::Environment, &Action::Read));
        assert!(!matrix.can_access(&Role::Viewer, &Resource::Environment, &Action::Write));

        // Test that auditor can access audit logs
        assert!(matrix.can_access(&Role::Auditor, &Resource::AuditLog, &Action::Audit));
    }

    #[test]
    fn test_rbac_engine() {
        let mut engine = RbacEngine::new();
        let user_id = Uuid::new_v4();

        // Assign editor role
        engine.assign_role(user_id, Role::Editor);

        // Check permissions
        assert!(engine.check_permission(user_id, &Resource::Environment, &Action::Write));
        assert!(engine.check_permission(user_id, &Resource::Environment, &Action::Read));
        assert!(!engine.check_permission(user_id, &Resource::User, &Action::Write));

        // Get user roles
        let roles = engine.get_user_roles(user_id);
        assert!(roles.contains(&Role::Editor));
        assert_eq!(roles.len(), 1);
    }

    #[test]
    fn test_user_assignment() {
        let user_id = Uuid::new_v4();
        let mut assignment = UserAssignment::new(user_id);

        assignment.add_role(Role::Admin);
        assignment.add_permission(Permission::new(Resource::Secret, Action::Write));

        assert!(assignment.roles.contains(&Role::Admin));
        assert_eq!(assignment.additional_permissions.len(), 1);
        assert!(!assignment.is_expired());
    }

    #[test]
    fn test_role_from_str() {
        assert_eq!("owner".parse::<Role>().unwrap(), Role::Owner);
        assert_eq!("admin".parse::<Role>().unwrap(), Role::Admin);
        assert_eq!("custom_role".parse::<Role>().unwrap(), Role::Custom("custom_role".to_string()));
    }
}