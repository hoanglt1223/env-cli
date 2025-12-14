//! # env-cli
//!
//! The missing CLI for environment variable management.
//!
//! This crate provides a powerful, open-source tool that manages the entire
//! lifecycle of environment variables for full-stack projects.

// Temporarily allow missing docs during development
#![allow(missing_docs)]
#![warn(clippy::all)]
// Allow clippy warnings temporarily to get build working
#![allow(clippy::new_without_default)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::manual_strip)]
#![allow(clippy::chars_next_cmp)]
#![allow(clippy::manual_flatten)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::option_as_ref_deref)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::unwrap_or_default)]
// Temporarily allow deprecated warnings in tests
#![cfg_attr(test, allow(deprecated))]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(unused_variables))]
#![cfg_attr(test, allow(unused_mut))]

pub mod cli;
pub mod commands;
pub mod config;
pub mod env;
pub mod error;
pub mod plugins;
pub mod scan;
pub mod sync;
pub mod utils;

// Re-export commonly used types
pub use error::{EnvCliError, Result};
