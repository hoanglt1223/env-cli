//! Completion command implementation.
//!
//! This module handles shell completion generation and installation.

use crate::cli::completion::{install_completion, uninstall_completion, Shell, CompletionGenerator};
use crate::error::Result;

/// Execute the completion command.
pub async fn execute(shell: String, install: bool, uninstall: bool) -> Result<()> {
    // Parse shell type
    let shell: Shell = shell.parse()?;

    // Handle install/uninstall actions
    if install && uninstall {
        return Err(crate::error::EnvCliError::InvalidArgument(
            "Cannot use --install and --uninstall together".to_string(),
        ));
    }

    if install {
        install_completion(shell)?;
        return Ok(());
    }

    if uninstall {
        uninstall_completion(shell)?;
        return Ok(());
    }

    // Default: generate completion script
    let generator = CompletionGenerator::new();
    let script = generator.generate(shell)?;

    // Print completion script to stdout
    println!("{}", script);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_parsing() {
        assert!("bash".parse::<Shell>().is_ok());
        assert!("zsh".parse::<Shell>().is_ok());
        assert!("fish".parse::<Shell>().is_ok());
        assert!("powershell".parse::<Shell>().is_ok());
        assert!("pwsh".parse::<Shell>().is_ok());
        assert!("invalid".parse::<Shell>().is_err());
    }
}