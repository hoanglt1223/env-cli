//! Integration tests for env-cli
//!
//! This module contains comprehensive end-to-end tests for all CLI commands
//! and functionality of the env-cli tool.

pub mod common;

mod advanced_integration_tests;
mod integration_tests;

// Re-export commonly used test utilities for convenience
pub use common::{assertions, EnvVarGenerator, TestConfig, TestProject, TestProjectBuilder};
