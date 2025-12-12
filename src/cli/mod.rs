//! CLI argument definitions and parsing.
//!
//! This module contains the command-line interface definitions using clap.

use clap::Parser;

/// The missing CLI for environment variable management.
#[derive(Parser)]
#[command(name = "env")]
#[command(
    about = "The missing CLI for environment variable management",
    long_about = "A powerful, open-source tool that manages the entire lifecycle of environment variables for full-stack projects."
)]
#[command(version = "0.1.0")]
#[command(author = "env-cli contributors")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    /// Initialize project with env structure
    Init {
        /// Force initialization even if already initialized
        #[arg(long)]
        force: bool,
    },
    /// Switch between environments
    Switch {
        /// Environment name to switch to
        environment: String,
        /// Skip confirmation before switching
        #[arg(long)]
        yes: bool,
    },
    /// Scan code for env usage
    Scan {
        /// Directory to scan (default: current directory)
        #[arg(default_value = ".")]
        path: std::path::PathBuf,
        /// Output format (default: text)
        #[arg(long, default_value = "text")]
        format: OutputFormat,
        /// Include hidden files and directories
        #[arg(long)]
        hidden: bool,
    },
    /// Validate environment configuration
    Validate {
        /// Environment to validate (default: current)
        #[arg(long, default_value = "current")]
        env: String,
        /// Check for unused environment variables
        #[arg(long)]
        check_unused: bool,
    },
    /// Sync environments safely
    Sync {
        /// Source environment
        source: String,
        /// Target environment
        target: String,
        /// Skip confirmation before syncing
        #[arg(long)]
        yes: bool,
    },
    /// Generate .env.example file
    Generate {
        /// Output file path (default: .env.example)
        #[arg(long, default_value = ".env.example")]
        output: std::path::PathBuf,
        /// Include comments describing each variable
        #[arg(long)]
        comments: bool,
    },
    /// Show current environment status
    Status {
        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
}