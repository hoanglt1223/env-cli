//! Advanced scan command implementation for EC-03.

use crate::cli::OutputFormat;
use crate::error::Result;
use crate::scan::CodeScanner;
use std::path::PathBuf;

/// Scan code for environment variable usage with advanced features.
pub async fn execute(path: PathBuf, format: OutputFormat, hidden: bool) -> Result<()> {
    println!("🔍 Scanning code for environment variable usage...");
    println!("📁 Path: {}", path.display());
    println!("⚙️  Format: {:?}", format);

    // Initialize advanced scanner
    let scanner = CodeScanner::with_config(true, None)?;

    println!("🚀 Starting advanced scan with parallel processing...");

    // Perform advanced scan
    let scan_result = scanner.scan_directory_advanced(&path).await?;

    println!("✅ Scan completed successfully!");
    println!("📊 Scan Statistics:");
    println!("  - Files scanned: {}", scan_result.files_scanned);
    println!("  - Variables found: {}", scan_result.variables.len());
    println!("  - Patterns matched: {}", scan_result.patterns_matched);
    println!("  - Security issues: {}", scan_result.security_issues.len());
    println!("  - Scan duration: {:?}", scan_result.scan_duration);

    // Language distribution
    if !scan_result.languages_detected.is_empty() {
        println!("\n🏷️  Languages detected:");
        for (language, count) in &scan_result.languages_detected {
            println!("  - {}: {} files", language, count);
        }
    }

    // Display environment variables
    if !scan_result.variables.is_empty() {
        println!("\n🔧 Environment variables found:");

        let mut variables: Vec<_> = scan_result.variables.values().collect();
        variables.sort_by(|a, b| a.name.cmp(&b.name));

        for usage in variables {
            let _display_value = if hidden { "***HIDDEN***" } else { "empty" };

            if usage.files.len() == 1 {
                println!("  {} - Used in {}", usage.name, usage.files[0]);
            } else {
                println!("  {} - Used in {} locations", usage.name, usage.files.len());
                if !hidden && usage.files.len() <= 3 {
                    for file in &usage.files {
                        println!("    - {}", file);
                    }
                } else if !hidden {
                    println!(
                        "    - {} and {} more",
                        usage.files[0..2].join(", "),
                        usage.files.len() - 2
                    );
                }
            }
        }
    }

    // Display security issues
    if !scan_result.security_issues.is_empty() {
        println!("\n⚠️  Security Issues:");
        for issue in &scan_result.security_issues {
            let severity_icon = match issue.severity {
                crate::scan::SecuritySeverity::Critical => "🚨",
                crate::scan::SecuritySeverity::High => "❌",
                crate::scan::SecuritySeverity::Medium => "⚠️",
                crate::scan::SecuritySeverity::Low => "ℹ️",
            };
            println!(
                "  {} {}: {}:{}",
                severity_icon, issue.message, issue.file, issue.line
            );
        }
    }

    // Output in different formats
    match format {
        OutputFormat::Text => {
            println!("\n✨ Scan completed successfully!");
        }
        OutputFormat::Json => {
            println!("\n📄 JSON Output:");
            let json_output = serde_json::json!({
                "scan_result": {
                    "files_scanned": scan_result.files_scanned,
                    "variables_found": scan_result.variables.len(),
                    "patterns_matched": scan_result.patterns_matched,
                    "security_issues_count": scan_result.security_issues.len(),
                    "scan_duration_ms": scan_result.scan_duration.as_millis(),
                    "languages_detected": scan_result.languages_detected,
                    "variables": scan_result.variables.values().map(|usage| {
                        serde_json::json!({
                            "name": usage.name,
                            "files": usage.files,
                            "usage_count": usage.files.len()
                        })
                    }).collect::<Vec<_>>(),
                    "security_issues": scan_result.security_issues.iter().map(|issue| {
                        serde_json::json!({
                            "severity": format!("{:?}", issue.severity),
                            "message": issue.message,
                            "file": issue.file,
                            "line": issue.line
                        })
                    }).collect::<Vec<_>>()
                }
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        OutputFormat::Yaml => {
            println!("\n📄 YAML Output:");
            let yaml_output = serde_json::json!({
                "scan_result": {
                    "files_scanned": scan_result.files_scanned,
                    "variables_found": scan_result.variables.len(),
                    "patterns_matched": scan_result.patterns_matched,
                    "security_issues_count": scan_result.security_issues.len(),
                    "scan_duration_ms": scan_result.scan_duration.as_millis(),
                    "languages_detected": scan_result.languages_detected,
                    "variables": scan_result.variables.values().map(|usage| {
                        serde_json::json!({
                            "name": usage.name,
                            "files": usage.files,
                            "usage_count": usage.files.len()
                        })
                    }).collect::<Vec<_>>(),
                    "security_issues": scan_result.security_issues.iter().map(|issue| {
                        serde_json::json!({
                            "severity": format!("{:?}", issue.severity),
                            "message": issue.message,
                            "file": issue.file,
                            "line": issue.line
                        })
                    }).collect::<Vec<_>>()
                }
            });
            println!("{}", serde_yaml::to_string(&yaml_output)?);
        }
    }

    Ok(())
}
