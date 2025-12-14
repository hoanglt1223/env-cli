//! env-cli - The missing CLI for environment variable management
//!
//! This is the main entry point for the env-cli application.

#[cfg(not(test))]
use clap::Parser;
#[cfg(not(test))]
use env_cli::cli::Cli;
#[cfg(not(test))]
use env_cli::commands::execute_command;
#[cfg(not(test))]
use env_cli::error::Result;

#[cfg(not(test))]
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the appropriate command
    execute_command(cli.command).await
}

// Stub main for test mode to prevent binary execution
#[cfg(test)]
fn main() {
    // No-op in test mode
}
