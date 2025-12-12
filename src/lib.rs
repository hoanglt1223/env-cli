//! # env-cli
//!
//! The missing CLI for environment variable management.
//!
//! This crate provides a powerful, open-source tool that manages the entire
//! lifecycle of environment variables for full-stack projects.

#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod cli;
pub mod commands;
pub mod config;
pub mod env;
pub mod error;
pub mod scan;
pub mod utils;

// Re-export commonly used types
pub use error::{EnvCliError, Result};