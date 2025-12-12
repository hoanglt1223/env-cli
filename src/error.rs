//! Error types for the env-cli application.
//!
//! This module defines custom error types and provides error handling utilities.

use std::fmt;

/// Custom error type for env-cli operations.
#[derive(Debug)]
pub enum EnvCliError {
    /// Configuration-related errors
    Config(String),
    /// Environment variable errors
    Environment(String),
    /// File system errors
    FileSystem(String),
    /// Command execution errors
    Command(String),
    /// Scanning errors
    Scan(String),
    /// Validation errors
    Validation(String),
    /// Invalid format errors
    InvalidFormat(String),
    /// Already initialized error
    AlreadyInitialized(String),
    /// Invalid argument errors
    InvalidArgument(String),
    /// IO errors
    Io(std::io::Error),
    /// Serialization errors
    Serialization(String),
    /// Authentication errors
    AuthenticationError(String),
    /// Collaboration errors
    CollaborationError(String),
    /// Config errors (enterprise)
    ConfigError(String),
    /// Encryption errors
    EncryptionError(String),
    /// RBAC errors
    RbacError(String),
    /// Audit errors
    AuditError(String),
    /// Compliance errors
    ComplianceError(String),
}

impl fmt::Display for EnvCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvCliError::Config(msg) => write!(f, "Configuration error: {}", msg),
            EnvCliError::Environment(msg) => write!(f, "Environment error: {}", msg),
            EnvCliError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            EnvCliError::Command(msg) => write!(f, "Command error: {}", msg),
            EnvCliError::Scan(msg) => write!(f, "Scanning error: {}", msg),
            EnvCliError::Validation(msg) => write!(f, "Validation error: {}", msg),
            EnvCliError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            EnvCliError::AlreadyInitialized(msg) => write!(f, "Already initialized: {}", msg),
            EnvCliError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            EnvCliError::Io(err) => write!(f, "IO error: {}", err),
            EnvCliError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            EnvCliError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            EnvCliError::CollaborationError(msg) => write!(f, "Collaboration error: {}", msg),
            EnvCliError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            EnvCliError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            EnvCliError::RbacError(msg) => write!(f, "RBAC error: {}", msg),
            EnvCliError::AuditError(msg) => write!(f, "Audit error: {}", msg),
            EnvCliError::ComplianceError(msg) => write!(f, "Compliance error: {}", msg),
        }
    }
}

impl std::error::Error for EnvCliError {}

impl From<std::io::Error> for EnvCliError {
    fn from(err: std::io::Error) -> Self {
        EnvCliError::Io(err)
    }
}

impl From<serde_json::Error> for EnvCliError {
    fn from(err: serde_json::Error) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for EnvCliError {
    fn from(err: toml::de::Error) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<toml::ser::Error> for EnvCliError {
    fn from(err: toml::ser::Error) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<std::fmt::Error> for EnvCliError {
    fn from(err: std::fmt::Error) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<serde_yaml::Error> for EnvCliError {
    fn from(err: serde_yaml::Error) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for EnvCliError {
    fn from(err: std::time::SystemTimeError) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<anyhow::Error> for EnvCliError {
    fn from(err: anyhow::Error) -> Self {
        EnvCliError::Serialization(err.to_string())
    }
}

impl From<uuid::Error> for EnvCliError {
    fn from(err: uuid::Error) -> Self {
        EnvCliError::InvalidArgument(format!("UUID error: {}", err))
    }
}

impl From<bcrypt::BcryptError> for EnvCliError {
    fn from(err: bcrypt::BcryptError) -> Self {
        EnvCliError::AuthenticationError(format!("Bcrypt error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for EnvCliError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        EnvCliError::AuthenticationError(format!("JWT error: {}", err))
    }
}

impl From<sha2::digest::InvalidLength> for EnvCliError {
    fn from(err: sha2::digest::InvalidLength) -> Self {
        EnvCliError::EncryptionError(format!("Invalid key length: {}", err))
    }
}

/// Result type alias for env-cli operations.
pub type Result<T> = std::result::Result<T, EnvCliError>;

/// Error context extension trait for better error messages.
pub trait ErrorContext<T> {
    /// Add context to an error result.
    fn context(self, msg: &str) -> Result<T>;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<EnvCliError>,
{
    fn context(self, msg: &str) -> Result<T> {
        self.map_err(|e| EnvCliError::Config(format!("{}: {}", msg, e.into())))
    }
}
