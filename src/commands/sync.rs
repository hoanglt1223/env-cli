//! Advanced sync command implementation for EC-03.

use crate::error::Result;
use crate::sync::{ConflictResolution, EnvironmentSync, SyncConfig};
use std::path::PathBuf;

/// Sync environments safely with advanced conflict detection and resolution.
pub async fn execute(source: String, target: String, yes: bool) -> Result<()> {
    println!("🔄 Synchronizing environments...");
    println!("📂 Source: {}", source);
    println!("📂 Target: {}", target);

    let source_path = PathBuf::from(source);
    let target_path = PathBuf::from(target);

    // Configure synchronization
    let config = SyncConfig {
        conflict_resolution: ConflictResolution::ManualReview,
        backup_before_sync: true,
        audit_log_path: PathBuf::from(".env/sync-audit.json"),
        security_check: true,
        dry_run: false,
        selective_variables: None,
    };

    let mut sync_engine = EnvironmentSync::new(config);

    println!("🔍 Analyzing environments for conflicts...");

    // Perform dry run first
    let dry_run_result = sync_engine.dry_run_sync(&source_path, &target_path).await?;

    // Display dry run results
    if !dry_run_result.proposed_changes.is_empty() {
        println!("\n📋 Proposed Changes:");
        for change in &dry_run_result.proposed_changes {
            let change_icon = match change.change_type {
                crate::sync::ChangeType::Add => "➕",
                crate::sync::ChangeType::Update => "🔄",
                crate::sync::ChangeType::Remove => "➖",
                crate::sync::ChangeType::Conflict => "⚠️",
            };
            println!(
                "  {} {}: '{}' -> '{}'",
                change_icon,
                change.variable,
                if change.old_value.is_empty() {
                    "(empty)"
                } else {
                    &change.old_value
                },
                if change.new_value.is_empty() {
                    "(empty)"
                } else {
                    &change.new_value
                }
            );
        }
    }

    // Display conflicts
    if !dry_run_result.conflicts.is_empty() {
        println!("\n⚠️  Conflicts Detected:");
        for conflict in &dry_run_result.conflicts {
            let conflict_icon = match conflict.conflict_type {
                crate::sync::ConflictType::ValueMismatch => "💥",
                crate::sync::ConflictType::MissingInTarget => "➕",
                crate::sync::ConflictType::MissingInSource => "➖",
                crate::sync::ConflictType::TypeMismatch => "🔄",
                crate::sync::ConflictType::SecurityViolation => "🚨",
            };
            println!(
                "  {} {}: Source='{}' vs Target='{}'",
                conflict_icon,
                conflict.variable,
                if conflict.source_value.is_empty() {
                    "(empty)"
                } else {
                    &conflict.source_value
                },
                if conflict.target_value.is_empty() {
                    "(empty)"
                } else {
                    &conflict.target_value
                }
            );
        }
    }

    // Display security violations
    if !dry_run_result.security_violations.is_empty() {
        println!("\n🚨 Security Violations:");
        for violation in &dry_run_result.security_violations {
            let severity_icon = match violation.severity {
                crate::sync::SecuritySeverity::Critical => "🚨",
                crate::sync::SecuritySeverity::High => "❌",
                crate::sync::SecuritySeverity::Medium => "⚠️",
                crate::sync::SecuritySeverity::Low => "ℹ️",
            };
            println!(
                "  {} {}: {}",
                severity_icon, violation.variable, violation.description
            );
        }
    }

    if dry_run_result.proposed_changes.is_empty() && dry_run_result.conflicts.is_empty() {
        println!("\n✅ No changes needed. Environments are already in sync.");
        return Ok(());
    }

    // Confirmation prompt
    if !yes {
        println!(
            "\n🤔 Estimated sync duration: {:?}",
            dry_run_result.estimated_duration
        );
        println!("💡 A backup will be created before sync.");

        print!("Proceed with synchronization? [y/N]: ");
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("❌ Synchronization cancelled by user.");
            return Ok(());
        }
    }

    println!("\n🚀 Starting synchronization...");

    // Perform actual synchronization
    let sync_result = sync_engine
        .sync_environments(&source_path, &target_path)
        .await?;

    // Display results
    println!("✅ Synchronization completed successfully!");
    println!("📊 Sync Results:");
    println!(
        "  - Variables synced: {}",
        sync_result.synced_variables.len()
    );
    println!(
        "  - Conflicts resolved: {}",
        sync_result.conflicts_resolved.len()
    );
    println!("  - Duration: {:?}", sync_result.duration);
    println!("  - Backup created: {}", sync_result.backup_created);

    if !sync_result.synced_variables.is_empty() {
        println!("\n🔄 Synced Variables:");
        for variable in &sync_result.synced_variables {
            println!("  - {}", variable);
        }
    }

    println!("\n📝 Audit log updated: .env/sync-audit.json");
    println!("💡 Run 'env status --verbose' to see detailed changes.");

    Ok(())
}
