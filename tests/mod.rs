//! Integration tests for env-cli
//!
//! This module contains happy path tests for all CLI commands.

pub mod common;

mod integration_tests;

// Re-export commonly used test utilities for convenience
pub use common::{assertions, EnvVarGenerator, TestConfig, TestProject, TestProjectBuilder};
