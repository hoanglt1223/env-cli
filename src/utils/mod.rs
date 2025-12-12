//! Utility functions.

use crate::error::Result;
use std::path::PathBuf;

/// Get the current working directory.
pub fn current_dir() -> Result<PathBuf> {
    std::env::current_dir().map_err(|e| crate::error::EnvCliError::FileSystem(e.to_string()))
}

/// Ensure a directory exists, creating it if necessary.
pub fn ensure_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Read file contents as string.
pub fn read_file(path: &PathBuf) -> Result<String> {
    std::fs::read_to_string(path).map_err(|e| crate::error::EnvCliError::FileSystem(e.to_string()))
}

/// Write string to file.
pub fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_dir(&parent.to_path_buf())?;
    }

    std::fs::write(path, content).map_err(|e| crate::error::EnvCliError::FileSystem(e.to_string()))
}

/// Check if a path exists.
pub fn path_exists(path: &PathBuf) -> bool {
    path.exists()
}

/// Generate a random secure string.
pub fn generate_secret(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Format bytes as human readable string.
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Validate environment variable name.
pub fn validate_env_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && name
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_alphabetic())
}

/// Get user home directory.
pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

/// Get user config directory.
pub fn config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}

/// Prompt user for confirmation.
pub fn confirm(message: &str) -> Result<bool> {
    println!("{} [y/N]", message);

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| crate::error::EnvCliError::Io(e))?;

    let input = input.trim().to_lowercase();
    Ok(matches!(input.as_str(), "y" | "yes"))
}

/// Create a backup of a file.
pub fn backup_file(path: &PathBuf) -> Result<PathBuf> {
    if !path.exists() {
        return Err(crate::error::EnvCliError::FileSystem(format!(
            "Cannot backup non-existent file: {}",
            path.display()
        )));
    }

    let backup_path = path.with_extension(format!(
        "{}.backup",
        path.extension().and_then(|s| s.to_str()).unwrap_or("bak")
    ));

    std::fs::copy(path, &backup_path)?;
    Ok(backup_path)
}

/// Get timestamp string for current time.
pub fn timestamp() -> String {
    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_env_name() {
        assert!(validate_env_name("VALID_NAME"));
        assert!(validate_env_name("VALID_NAME_123"));
        assert!(!validate_env_name(""));
        assert!(!validate_env_name("123_INVALID"));
        assert!(!validate_env_name("INVALID-NAME"));
        assert!(!validate_env_name("INVALID NAME"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
    }

    #[test]
    fn test_generate_secret() {
        let secret = generate_secret(32);
        assert_eq!(secret.len(), 32);

        // Ensure it contains only valid characters
        assert!(secret
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "!@#$%^&*".contains(c)));
    }
}
