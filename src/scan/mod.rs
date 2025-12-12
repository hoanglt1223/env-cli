//! Advanced code scanning functionality for EC-03.
//!
//! This module provides comprehensive multi-language source code analysis
//! for environment variable usage with parallel processing capabilities.

use crate::env::EnvUsage;
use crate::error::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use ignore::{Walk, WalkBuilder};

/// Represents a security issue found during scanning
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: SecuritySeverity,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub variable: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Language-specific configuration for scanning
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub extensions: Vec<String>,
    pub patterns: Vec<String>,
    pub frameworks: Vec<String>,
    pub comment_patterns: Vec<String>,
}

/// Scan result containing comprehensive information
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub variables: HashMap<String, EnvUsage>,
    pub files_scanned: usize,
    pub patterns_matched: usize,
    pub security_issues: Vec<SecurityIssue>,
    pub scan_duration: std::time::Duration,
    pub languages_detected: HashMap<String, usize>,
}

/// Advanced code scanner with multi-language support and parallel processing
pub struct CodeScanner {
    /// Language-specific configurations
    languages: HashMap<String, LanguageConfig>,
    /// Compiled regex patterns for each language
    patterns: HashMap<String, Vec<regex::Regex>>,
    /// Security scanning patterns
    security_patterns: Vec<regex::Regex>,
    /// File inclusion patterns
    include_patterns: Vec<glob::Pattern>,
    /// File exclusion patterns
    exclude_patterns: Vec<glob::Pattern>,
    /// Directories to exclude
    exclude_dirs: Vec<String>,
    /// Whether to enable parallel scanning
    parallel: bool,
    /// Number of worker threads for parallel processing
    worker_threads: Option<usize>,
}

impl CodeScanner {
    /// Create a new advanced code scanner.
    pub fn new() -> Result<Self> {
        let mut scanner = Self {
            languages: HashMap::new(),
            patterns: HashMap::new(),
            security_patterns: Vec::new(),
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            exclude_dirs: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "vendor".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".next".to_string(),
                ".nuxt".to_string(),
                "coverage".to_string(),
            ],
            parallel: true,
            worker_threads: None, // Use rayon's default
        };

        scanner.init_languages();
        scanner.init_patterns();
        Ok(scanner)
    }

    /// Create a scanner with custom configuration.
    pub fn with_config(parallel: bool, worker_threads: Option<usize>) -> Result<Self> {
        let mut scanner = Self::new()?;
        scanner.parallel = parallel;
        scanner.worker_threads = worker_threads;
        Ok(scanner)
    }

    /// Initialize language-specific configurations.
    fn init_languages(&mut self) {
        // Rust
        self.languages.insert("rust".to_string(), LanguageConfig {
            extensions: vec!["rs".to_string()],
            patterns: vec![
                r#"std::env::var\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"env!\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"dotenv\s*\(\)\.ok\(\)"#.to_string(),
                r#"dotenvy\s*\(\)\.ok\(\)"#.to_string(),
            ],
            frameworks: vec!["actix".to_string(), "rocket".to_string(), "tokio".to_string()],
            comment_patterns: vec![r#"//.*"#.to_string(), r#"/\*[\s\S]*?\*/"#.to_string()],
        });

        // JavaScript/TypeScript
        self.languages.insert("javascript".to_string(), LanguageConfig {
            extensions: vec!["js".to_string(), "jsx".to_string(), "mjs".to_string()],
            patterns: vec![
                r#"process\.env\.([a-zA-Z_][a-zA-Z0-9_]*)"#.to_string(),
                r#"import\.meta\.env\.([a-zA-Z_][a-zA-Z0-9_]*)"#.to_string(),
                r#"process\.env\["([^"]+)"\]"#.to_string(),
                r#"process\.env\['([^']+)'\]"#.to_string(),
            ],
            frameworks: vec!["react".to_string(), "vue".to_string(), "express".to_string(), "next".to_string()],
            comment_patterns: vec![r#"//.*"#.to_string(), r#"/\*[\s\S]*?\*/"#.to_string()],
        });

        // TypeScript
        self.languages.insert("typescript".to_string(), LanguageConfig {
            extensions: vec!["ts".to_string(), "tsx".to_string()],
            patterns: vec![
                r#"process\.env\.([a-zA-Z_][a-zA-Z0-9_]*)"#.to_string(),
                r#"import\.meta\.env\.([a-zA-Z_][a-zA-Z0-9_]*)"#.to_string(),
                r#"process\.env\["([^"]+)"\]"#.to_string(),
                r#"process\.env\['([^']+)'\]"#.to_string(),
            ],
            frameworks: vec!["angular".to_string(), "nestjs".to_string(), "react".to_string()],
            comment_patterns: vec![r#"//.*"#.to_string(), r#"/\*[\s\S]*?\*/"#.to_string()],
        });

        // Python
        self.languages.insert("python".to_string(), LanguageConfig {
            extensions: vec!["py".to_string(), "pyx".to_string(), "pyi".to_string()],
            patterns: vec![
                r#"os\.getenv\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"os\.environ\["([^"]+)"\]"#.to_string(),
                r#"os\.environ\.get\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"dotenv\.load_dotenv\(\)"#.to_string(),
            ],
            frameworks: vec!["django".to_string(), "flask".to_string(), "fastapi".to_string()],
            comment_patterns: vec![r#"#.*"#.to_string(), r#"'''[\s\S]*?'''".to_string(), r#"""[\s\S]*?"""".to_string()],
        });

        // Go
        self.languages.insert("go".to_string(), LanguageConfig {
            extensions: vec!["go".to_string()],
            patterns: vec![
                r#"os\.Getenv\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"viper\.Get\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"godotenv\.Load\(\)"#.to_string(),
            ],
            frameworks: vec!["gin".to_string(), "echo".to_string(), "cobra".to_string()],
            comment_patterns: vec![r#"//.*"#.to_string(), r#"/\*[\s\S]*?\*/"#.to_string()],
        });

        // Java
        self.languages.insert("java".to_string(), LanguageConfig {
            extensions: vec!["java".to_string(), "kt".to_string(), "scala".to_string()],
            patterns: vec![
                r#"System\.getenv\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"System\.getProperty\s*\(\s*["']([^"']+)["']"#.to_string(),
            ],
            frameworks: vec!["spring".to_string(), "maven".to_string(), "gradle".to_string()],
            comment_patterns: vec![r#"//.*"#.to_string(), r#"/\*[\s\S]*?\*/"#.to_string()],
        });

        // PHP
        self.languages.insert("php".to_string(), LanguageConfig {
            extensions: vec!["php".to_string()],
            patterns: vec![
                r#"\$_ENV\["([^"]+)"\]"#.to_string(),
                r#"\$_SERVER\["([^"]+)"\]"#.to_string(),
                r#"getenv\s*\(\s*["']([^"']+)["']"#.to_string(),
                r#"dotenv\s*\(\s*["']([^"']+)["']"#.to_string(),
            ],
            frameworks: vec!["laravel".to_string(), "symfony".to_string()],
            comment_patterns: vec![r#"//.*"#.to_string(), r#"#.*"#.to_string(), r#"/\*[\s\S]*?\*/"#.to_string()],
        });

        // Shell/Bash
        self.languages.insert("shell".to_string(), LanguageConfig {
            extensions: vec!["sh".to_string(), "bash".to_string(), "zsh".to_string(), "fish".to_string()],
            patterns: vec![
                r#"\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}"#.to_string(),
                r#"\$([a-zA-Z_][a-zA-Z0-9_]*)"#.to_string(),
                r#"export\s+([a-zA-Z_][a-zA-Z0-9_]*)="#.to_string(),
            ],
            frameworks: vec![],
            comment_patterns: vec![r#"#.*"#.to_string()],
        });

        // Configuration files
        self.languages.insert("config".to_string(), LanguageConfig {
            extensions: vec!["json".to_string(), "yaml".to_string(), "yml".to_string(), "toml".to_string(), "ini".to_string()],
            patterns: vec![
                r#"["']?([a-zA-Z_][a-zA-Z0-9_]*)["']?\s*:\s*["']?\$\{([^}]+)\}["']?"#.to_string(),
                r#"["']?([a-zA-Z_][a-zA-Z0-9_]*)["']?\s*=\s*["']?\$\{([^}]+)\}["']?"#.to_string(),
            ],
            frameworks: vec!["docker".to_string(), "kubernetes".to_string()],
            comment_patterns: vec![r#"#.*"#.to_string(), r#"//.*"#.to_string()],
        });
    }

    /// Initialize regex patterns for detecting environment variable usage.
    fn init_patterns(&mut self) {
        // Compile language-specific patterns
        for (language, config) in &self.languages {
            let mut compiled_patterns = Vec::new();
            for pattern in &config.patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    compiled_patterns.push(regex);
                }
            }
            self.patterns.insert(language.clone(), compiled_patterns);
        }

        // Initialize security patterns
        let security_patterns = vec![
            (r#"(?i)password\s*=\s*["']?[^"'\s]+["']?"#, SecuritySeverity::High),
            (r#"(?i)secret.*=\s*["']?[^"'\s]{8,}"#, SecuritySeverity::High),
            (r#"(?i)api[_-]?key.*=\s*["']?[^"'\s]{16,}"#, SecuritySeverity::Medium),
            (r#"(?i)token.*=\s*["']?[^"'\s]{16,}"#, SecuritySeverity::Medium),
            (r#"(?i)private[_-]?key"#, SecuritySeverity::Critical),
            (r#"(?i)aws[_-]?secret"#, SecuritySeverity::Critical),
            (r#"(?i)database[_-]?url.*=.*://.*:"#, SecuritySeverity::High),
            (r#"(?i)connection[_-]?string.*=.*password"#, SecuritySeverity::High),
        ];

        for (pattern, _) in security_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                self.security_patterns.push(regex);
            }
        }

        // Initialize file patterns
        let include_patterns = vec![
            "*.rs", "*.js", "*.ts", "*.jsx", "*.tsx", "*.py", "*.go", "*.java", "*.kt", "*.scala",
            "*.php", "*.sh", "*.bash", "*.zsh", "*.fish", "*.json", "*.yaml", "*.yml", "*.toml", "*.ini",
            "*.env", "*.env.example", "*.env.template", "*.env.sample"
        ];
        let exclude_patterns = vec![
            "*.min.js", "*.min.css", "*.lock", "*.sum", "*.mod", "*.pb.go",
            "*.generated.*", "*_pb2.py", "*.mock.*", "*.test.*", "*.spec.*"
        ];

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

    /// Scan a directory for environment variable usage with comprehensive results.
    pub async fn scan_directory_advanced(&self, path: &PathBuf) -> Result<ScanResult> {
        let start_time = std::time::Instant::now();
        let mut variables = HashMap::new();
        let mut security_issues = Vec::new();
        let mut languages_detected = HashMap::new();
        let mut files_scanned = 0;
        let mut patterns_matched = 0;

        // Configure rayon thread pool if custom threads specified
        if let Some(threads) = self.worker_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .map_err(|e| crate::error::EnvCliError::Config(format!("Failed to configure thread pool: {}", e)))?;
        }

        // Build walk iterator with ignore support
        let walk = WalkBuilder::new(path)
            .hidden(false)
            .git_ignore(false)
            .git_exclude(false)
            .parallel(self.parallel)
            .build_parallel();

        let scan_results = std::sync::Mutex::new(Vec::new());

        walk.run(|| {
            Box::new(|entry_result| {
                if let Ok(entry) = entry_result {
                    if self.should_include_file(&entry.path()) {
                        if let Some(result) = self.scan_file_advanced(&entry.path()) {
                            scan_results.lock().unwrap().push(result);
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });

        // Process results
        let results = scan_results.into_inner().unwrap();
        for result in results {
            files_scanned += 1;

            // Update language detection
            if let Some(lang) = self.detect_language(&result.path) {
                *languages_detected.entry(lang).or_insert(0) += 1;
            }

            // Process variable usages
            for (var_name, locations) in result.variables {
                let usage = variables
                    .entry(var_name.clone())
                    .or_insert_with(|| EnvUsage::new(var_name));

                for (file, line) in locations {
                    usage.add_usage(file, line);
                    patterns_matched += 1;
                }
            }

            // Process security issues
            security_issues.extend(result.security_issues);
        }

        let scan_duration = start_time.elapsed();

        Ok(ScanResult {
            variables,
            files_scanned,
            patterns_matched,
            security_issues,
            scan_duration,
            languages_detected,
        })
    }

    /// Legacy scan method for backward compatibility
    pub async fn scan_directory(&self, path: &PathBuf) -> Result<Vec<EnvUsage>> {
        let result = self.scan_directory_advanced(path).await?;
        Ok(result.variables.into_values().collect())
    }

    /// Scan result from processing a single file
    #[derive(Debug)]
    struct FileScanResult {
        path: String,
        variables: HashMap<String, Vec<(String, usize)>>,
        security_issues: Vec<SecurityIssue>,
    }

    /// Advanced file scanning with security analysis
    fn scan_file_advanced(&self, path: &PathBuf) -> Option<FileScanResult> {
        let content = std::fs::read_to_string(path).ok()?;
        let path_str = path.to_string_lossy().to_string();
        let mut variables = HashMap::new();
        let mut security_issues = Vec::new();

        // Detect language for this file
        let language = self.detect_language(path);

        if let Some(lang) = language {
            if let Some(patterns) = self.patterns.get(&lang) {
                for (line_num, line) in content.lines().enumerate() {
                    // Scan for environment variable usage
                    for pattern in patterns {
                        for captures in pattern.captures_iter(line) {
                            if let Some(var_match) = captures.get(1) {
                                let var_name = var_match.as_str().to_string();
                                variables
                                    .entry(var_name.clone())
                                    .or_insert_with(Vec::new)
                                    .push((path_str.clone(), line_num + 1));
                            } else if let Some(var_match) = captures.get(2) {
                                // Some patterns capture in group 2
                                let var_name = var_match.as_str().to_string();
                                variables
                                    .entry(var_name.clone())
                                    .or_insert_with(Vec::new)
                                    .push((path_str.clone(), line_num + 1));
                            }
                        }
                    }

                    // Scan for security issues
                    for (i, security_pattern) in self.security_patterns.iter().enumerate() {
                        if security_pattern.is_match(line) {
                            let severity = match i {
                                0 | 1 | 6 | 7 => SecuritySeverity::High,
                                2 | 3 => SecuritySeverity::Medium,
                                4 | 5 => SecuritySeverity::Critical,
                                _ => SecuritySeverity::Low,
                            };

                            security_issues.push(SecurityIssue {
                                severity,
                                message: format!("Potential security issue detected: {}", line.trim()),
                                file: path_str.clone(),
                                line: line_num + 1,
                                variable: "unknown".to_string(), // Could be extracted with more complex parsing
                            });
                        }
                    }
                }
            }
        }

        Some(FileScanResult {
            path: path_str,
            variables,
            security_issues,
        })
    }

    /// Detect the programming language of a file
    fn detect_language(&self, path: &PathBuf) -> Option<String> {
        let extension = path.extension()?.to_str()?;

        for (lang, config) in &self.languages {
            if config.extensions.contains(&extension.to_string()) {
                return Some(lang.clone());
            }
        }

        None
    }

    /// Check if a file should be included in the scan.
    fn should_include_file(&self, path: &PathBuf) -> bool {
        if !path.is_file() {
            return false;
        }

        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Check include patterns
        let included = self.include_patterns.iter().any(|pattern| pattern.matches(file_name));

        // Check exclude patterns
        let excluded = self.exclude_patterns.iter().any(|pattern| pattern.matches(file_name));

        included && !excluded
    }

    /// Generate enhanced .env.example file from scan results.
    pub fn generate_env_example(&self, usages: &[EnvUsage], comments: bool) -> Result<String> {
        let mut content = String::new();
        content.push_str("# Environment variables\n");
        content.push_str("# Generated by env-cli advanced scan\n");
        content.push_str(&format!("# Generated on: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Group variables by type and usage frequency
        let mut grouped_vars = HashMap::new();
        for usage in usages {
            let category = if usage.name.contains("DATABASE") || usage.name.contains("DB") {
                "Database"
            } else if usage.name.contains("API") || usage.name.contains("TOKEN") {
                "API & Authentication"
            } else if usage.name.contains("LOG") || usage.name.contains("DEBUG") {
                "Logging & Debugging"
            } else if usage.name.contains("PORT") || usage.name.contains("HOST") {
                "Network"
            } else if usage.name.to_uppercase().contains("SECRET") || usage.name.to_uppercase().contains("KEY") {
                "Security"
            } else {
                "General"
            };

            grouped_vars
                .entry(category)
                .or_insert_with(Vec::new)
                .push(usage);
        }

        for (category, vars) in grouped_vars {
            content.push_str(&format!("# {}\n", category));
            content.push_str("# ----------------------------\n");

            for usage in vars {
                if comments {
                    content.push_str(&format!("# Used in {} location(s)\n", usage.files.len()));
                    if usage.files.len() <= 3 {
                        content.push_str(&format!("# Locations: {}\n", usage.files.join(", ")));
                    } else {
                        content.push_str(&format!("# Locations: {} and {} more\n",
                            usage.files[0..2].join(", "),
                            usage.files.len() - 2));
                    }
                }

                content.push_str(&format!("{}=\n", usage.name));

                if comments {
                    content.push('\n');
                }
            }

            content.push('\n');
        }

        Ok(content)
    }

    /// Generate comprehensive documentation from scan results
    pub fn generate_documentation(&self, scan_result: &ScanResult) -> Result<String> {
        let mut content = String::new();
        content.push_str("# Environment Variables Documentation\n\n");
        content.push_str(&format!("Generated on: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        content.push_str(&format!("Scan completed in: {:?}\n", scan_result.scan_duration));
        content.push_str(&format!("Files scanned: {}\n", scan_result.files_scanned));
        content.push_str(&format!("Patterns matched: {}\n", scan_result.patterns_matched));
        content.push_str(&format!("Variables found: {}\n\n", scan_result.variables.len()));

        // Language distribution
        content.push_str("## Language Distribution\n\n");
        for (language, count) in &scan_result.languages_detected {
            content.push_str(&format!("- {}: {} files\n", language, count));
        }
        content.push('\n');

        // Variables section
        content.push_str("## Environment Variables\n\n");
        for (name, usage) in &scan_result.variables {
            content.push_str(&format!("### {}\n", name));
            content.push_str(&format!("- Used in {} files\n", usage.files.len()));
            content.push_str(&format!("- First seen in: {}\n", usage.files.first().unwrap_or(&"unknown".to_string())));

            if usage.files.len() > 1 {
                content.push_str("- Usage locations:\n");
                for file in &usage.files {
                    content.push_str(&format!("  - {}\n", file));
                }
            }
            content.push('\n');
        }

        // Security issues section
        if !scan_result.security_issues.is_empty() {
            content.push_str("## Security Issues\n\n");

            let mut critical_issues = Vec::new();
            let mut high_issues = Vec::new();
            let mut medium_issues = Vec::new();
            let mut low_issues = Vec::new();

            for issue in &scan_result.security_issues {
                match issue.severity {
                    SecuritySeverity::Critical => critical_issues.push(issue),
                    SecuritySeverity::High => high_issues.push(issue),
                    SecuritySeverity::Medium => medium_issues.push(issue),
                    SecuritySeverity::Low => low_issues.push(issue),
                }
            }

            if !critical_issues.is_empty() {
                content.push_str("### Critical Issues\n\n");
                for issue in &critical_issues {
                    content.push_str(&format!("- {}: {}:{}\n",
                        issue.message, issue.file, issue.line));
                }
                content.push('\n');
            }

            if !high_issues.is_empty() {
                content.push_str("### High Severity Issues\n\n");
                for issue in &high_issues {
                    content.push_str(&format!("- {}: {}:{}\n",
                        issue.message, issue.file, issue.line));
                }
                content.push('\n');
            }

            if !medium_issues.is_empty() {
                content.push_str("### Medium Severity Issues\n\n");
                for issue in &medium_issues {
                    content.push_str(&format!("- {}: {}:{}\n",
                        issue.message, issue.file, issue.line));
                }
                content.push('\n');
            }

            if !low_issues.is_empty() {
                content.push_str("### Low Severity Issues\n\n");
                for issue in &low_issues {
                    content.push_str(&format!("- {}: {}:{}\n",
                        issue.message, issue.file, issue.line));
                }
                content.push('\n');
            }
        }

        Ok(content)
    }
}

impl Default for CodeScanner {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            languages: HashMap::new(),
            patterns: HashMap::new(),
            security_patterns: Vec::new(),
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            exclude_dirs: Vec::new(),
            parallel: true,
            worker_threads: None,
        })
    }
}