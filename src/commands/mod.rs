//! Command implementations.
//!
//! This module contains the business logic for each CLI command.

pub mod completion;
pub mod generate;
pub mod init;
pub mod scan;
pub mod status;
pub mod switch;
pub mod sync;
pub mod validate;

use crate::cli::Commands;
use crate::error::Result;

/// Result type for command execution
pub type CommandResult = Result<()>;

/// Execute the given command.
pub async fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Init { force } => init::execute(force).await,
        Commands::Switch { environment, yes } => switch::execute(environment, yes).await,
        Commands::Scan {
            path,
            format,
            hidden,
        } => scan::execute(path, format, hidden).await,
        Commands::Validate { env, check_unused } => validate::execute(env, check_unused).await,
        Commands::Sync {
            source,
            target,
            yes,
        } => sync::execute(source, target, yes).await,
        Commands::Generate {
            output,
            comments,
            docs,
            scan_dir,
        } => generate::execute(output, comments, docs, Some(scan_dir)).await,
        Commands::Status { verbose } => status::execute(verbose).await,
        Commands::Completion {
            shell,
            install,
            uninstall,
        } => completion::execute(shell, install, uninstall).await,
    }
}
