//! Code scanning functionality.
//!
//! This module provides utilities for scanning source code to detect environment variable usage.

use crate::env::EnvUsage;
use crate::error::Result;
use std::path::PathBuf;

/// Code scanner for environment variables.
pub struct CodeScanner {
    /// Patterns to detect environment variable usage
    patterns: Vec<regex::Regex>,
    /// File patterns to include
    include_patterns: Vec<glob::Pattern>,
    /// File patterns to exclude
    exclude_patterns: Vec<glob::Pattern>,
    /// Directories to exclude
    exclude_dirs: Vec<String>,
}

impl CodeScanner {
    /// Create a new code scanner.
    pub fn new() -> Result<Self> {
        let mut scanner = Self {
            patterns: Vec::new(),
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            exclude_dirs: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "vendor".to_string(),
            ],
        };

        scanner.init_patterns();
        Ok(scanner)
    }

    /// Initialize regex patterns for detecting environment variable usage.
    fn init_patterns(&mut self) {
        let patterns = vec![
            r#"(?i)env\.\s*get\s*\(\s*["']([^"']+)["']"#,            // env::get("VAR")
            r#"(?i)std::env::var\s*\(\s*["']([^"']+)["']"#,          // std::env::var("VAR")
            r#"(?i)process\.env\.([a-zA-Z_][a-zA-Z0-9_]*)"#,        // process.env.VAR (Node.js)
            r#"(?i)os\.getenv\s*\(\s*["']([^"']+)["']"#,            // os.getenv("VAR") (Python)
            r#"(?i)\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}"#,               // ${VAR} (shell)
            r#"(?i)\$([a-zA-Z_][a-zA-Z0-9_]*)"#,                    // $VAR (shell)
            r#"(?i)dotenv\s*\(\s*["']([^"']+)["']"#,                // dotenv("VAR")
        ];

        for pattern in patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                self.patterns.push(regex);
            }
        }

        // Initialize file patterns
        let include_patterns = vec!["*.rs", "*.js", "*.ts", "*.jsx", "*.tsx", "*.py"];
        let exclude_patterns = vec!["*.min.js", "*.min.css", "*.lock"];

        for pattern in include_patterns {
            if let Ok(glob) = glob::Pattern::new(pattern) {
                self.include_patterns.push(glob);
            }
        }

        for pattern in exclude_patterns {
            if let Ok(glob) = glob::Pattern::new(pattern) {
                self.exclude_patterns.push(glob);
            }
        }
    }

    /// Scan a directory for environment variable usage.
    pub async fn scan_directory(&self, path: &PathBuf) -> Result<Vec<EnvUsage>> {
        let mut usages = std::collections::HashMap::new();

        self.scan_recursive(path, &mut usages).await?;

        Ok(usages.into_values().collect())
    }

    /// Recursively scan directories.
    async fn scan_recursive(
        &self,
        path: &PathBuf,
        usages: &mut std::collections::HashMap<String, EnvUsage>,
    ) -> Result<()> {
        if !path.exists() {
            return Err(crate::error::EnvCliError::FileSystem(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }

        let entries = std::fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Check if directory should be excluded
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if self.exclude_dirs.contains(&dir_name.to_string()) {
                        continue;
                    }
                }

                // Recursively scan subdirectory
                Box::pin(self.scan_recursive(&path, usages)).await?;
            } else if path.is_file() {
                // Check if file should be included
                if self.should_include_file(&path) {
                    self.scan_file(&path, usages).await?;
                }
            }
        }

        Ok(())
    }

    /// Check if a file should be included in the scan.
    fn should_include_file(&self, path: &PathBuf) -> bool {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Check include patterns
        let included = self.include_patterns.iter().any(|pattern| pattern.matches(file_name));

        // Check exclude patterns
        let excluded = self.exclude_patterns.iter().any(|pattern| pattern.matches(file_name));

        included && !excluded
    }

    /// Scan a single file for environment variable usage.
    async fn scan_file(
        &self,
        path: &PathBuf,
        usages: &mut std::collections::HashMap<String, EnvUsage>,
    ) -> Result<()> {
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Ok(()), // Skip files that can't be read
        };

        let path_str = path.to_string_lossy();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                for captures in pattern.captures_iter(line) {
                    if let Some(var_match) = captures.get(1) {
                        let var_name = var_match.as_str().to_string();

                        let usage = usages
                            .entry(var_name.clone())
                            .or_insert_with(|| EnvUsage::new(var_name));

                        usage.add_usage(path_str.to_string(), line_num + 1);
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate .env.example file from scan results.
    pub fn generate_env_example(&self, usages: &[EnvUsage], comments: bool) -> Result<String> {
        let mut content = String::new();
        content.push_str("# Environment variables\n");
        content.push_str("# Generated by env-cli scan\n\n");

        for usage in usages {
            if comments {
                if let Some(desc) = &usage.description {
                    content.push_str(&format!("# {}\n", desc));
                } else {
                    content.push_str(&format!("# Used in: {}\n", usage.files.join(", ")));
                }
            }

            content.push_str(&format!("{}=\n", usage.name));

            if comments {
                content.push('\n');
            }
        }

        Ok(content)
    }
}

impl Default for CodeScanner {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            patterns: Vec::new(),
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            exclude_dirs: Vec::new(),
        })
    }
}