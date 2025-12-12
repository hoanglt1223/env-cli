//! Command implementations.
//!
//! This module contains the business logic for each CLI command.

pub mod completion;
pub mod enterprise;
pub mod generate;
pub mod init;
pub mod scan;
pub mod status;
pub mod switch;
pub mod sync;
pub mod validate;
pub mod workspace;

use crate::cli::Commands;
use crate::error::Result;
use uuid::Uuid;

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
        Commands::Enterprise { command } => enterprise::execute_enterprise_command(command).await,
        Commands::Workspace { command } => {
            // Create a placeholder auth context for workspace commands
            let auth_context = crate::enterprise::auth::AuthContext {
                user_id: Uuid::new_v4(),
                username: "test_user".to_string(),
                email: "test@example.com".to_string(),
                display_name: "Test User".to_string(),
                roles: vec!["admin".to_string()],
                authenticated_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
                auth_method: crate::enterprise::auth::AuthMethod::Local,
                claims: std::collections::HashMap::new(),
            };
            workspace::execute_workspace_command(command, &auth_context).await
        }
    }
}
