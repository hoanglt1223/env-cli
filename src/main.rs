//! env-cli - The missing CLI for environment variable management
//!
//! This is the main entry point for the env-cli application.

use env_cli::cli::Cli;
use env_cli::commands::execute_command;
use env_cli::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the appropriate command
    execute_command(cli.command).await
}
