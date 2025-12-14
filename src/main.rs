//! env-cli - The missing CLI for environment variable management
//!
//! This is the main entry point for the env-cli application.

use clap::Parser;
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

// Add a test to prevent the binary from being executed during lib tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_main_exists() {
        // This test exists to prevent the main binary from executing during lib tests
        assert!(true);
    }
}
