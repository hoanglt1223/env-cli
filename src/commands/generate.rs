//! Advanced generate command implementation for EC-03.

use crate::env::EnvUsage;
use crate::error::Result;
use crate::scan::CodeScanner;
use std::path::PathBuf;

/// Generate .env.example file with advanced automated documentation.
pub async fn execute(
    output: PathBuf,
    comments: bool,
    docs: bool,
    scan_dir: Option<PathBuf>,
) -> Result<()> {
    println!("🔧 Generating .env.example file with advanced features...");
    println!("📄 Output: {}", output.display());
    println!("💬 Include comments: {}", comments);
    println!("📚 Generate documentation: {}", docs);

    let scan_path = scan_dir.unwrap_or_else(|| PathBuf::from("."));

    // Initialize advanced scanner
    let scanner = CodeScanner::new()?;

    println!("🔍 Scanning codebase for environment variables...");
    println!("📁 Scan directory: {}", scan_path.display());

    // Perform comprehensive scan
    let scan_result = scanner.scan_directory_advanced(&scan_path).await?;

    println!("✅ Scan completed!");
    println!("📊 Scan Statistics:");
    println!("  - Files scanned: {}", scan_result.files_scanned);
    println!("  - Variables found: {}", scan_result.variables.len());
    println!(
        "  - Languages detected: {}",
        scan_result.languages_detected.len()
    );

    // Extract variables for generation
    let variables: Vec<_> = scan_result.variables.values().collect();

    if variables.is_empty() {
        println!("\n⚠️  No environment variables found in the codebase.");
        println!("💡 Make sure you're scanning the correct directory.");
        return Ok(());
    }

    // Generate .env.example content
    let usages: Vec<EnvUsage> = variables.iter().map(|v| (*v).clone()).collect();
    let env_example_content = scanner.generate_env_example(&usages, comments)?;

    // Create output directory if it doesn't exist
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write .env.example file
    std::fs::write(&output, env_example_content)?;

    println!(
        "\n📋 Generated .env.example with {} variables:",
        variables.len()
    );

    // Group variables by type for display
    let mut database_vars = Vec::new();
    let mut api_vars = Vec::new();
    let mut security_vars = Vec::new();
    let mut network_vars = Vec::new();
    let mut logging_vars = Vec::new();
    let mut general_vars = Vec::new();

    for usage in variables {
        if usage.name.contains("DATABASE") || usage.name.contains("DB") {
            database_vars.push(usage);
        } else if usage.name.contains("API") || usage.name.contains("TOKEN") {
            api_vars.push(usage);
        } else if usage.name.to_uppercase().contains("SECRET")
            || usage.name.to_uppercase().contains("KEY")
        {
            security_vars.push(usage);
        } else if usage.name.contains("PORT") || usage.name.contains("HOST") {
            network_vars.push(usage);
        } else if usage.name.contains("LOG") || usage.name.contains("DEBUG") {
            logging_vars.push(usage);
        } else {
            general_vars.push(usage);
        }
    }

    let display_category = |name: &str, vars: Vec<&crate::env::EnvUsage>| {
        if !vars.is_empty() {
            println!("  🔧 {} ({}):", name, vars.len());
            for usage in vars {
                println!(
                    "    - {} (used in {} location(s))",
                    usage.name,
                    usage.files.len()
                );
            }
        }
    };

    display_category("Database", database_vars);
    display_category("API & Authentication", api_vars);
    display_category("Security", security_vars);
    display_category("Network", network_vars);
    display_category("Logging & Debugging", logging_vars);
    display_category("General", general_vars);

    // Generate documentation if requested
    if docs {
        println!("\n📚 Generating comprehensive documentation...");

        let documentation = scanner.generate_documentation(&scan_result)?;
        let docs_path = output.with_extension("md");

        std::fs::write(&docs_path, documentation)?;

        println!("✅ Documentation generated: {}", docs_path.display());
    }

    // Display security summary
    if !scan_result.security_issues.is_empty() {
        println!("\n⚠️  Security Summary:");
        println!(
            "  - Total security issues: {}",
            scan_result.security_issues.len()
        );

        let critical_count = scan_result
            .security_issues
            .iter()
            .filter(|i| matches!(i.severity, crate::scan::SecuritySeverity::Critical))
            .count();
        let high_count = scan_result
            .security_issues
            .iter()
            .filter(|i| matches!(i.severity, crate::scan::SecuritySeverity::High))
            .count();

        if critical_count > 0 {
            println!("  - {} Critical issues", critical_count);
        }
        if high_count > 0 {
            println!("  - {} High severity issues", high_count);
        }

        println!("💡 Review the generated documentation for detailed security analysis.");
    }

    println!("\n✨ .env.example generated successfully!");
    println!("📁 File saved to: {}", output.display());

    if docs {
        println!(
            "📚 Documentation saved to: {}",
            output.with_extension("md").display()
        );
    }

    println!("💡 Tips:");
    println!("  - Review the generated file and add example values");
    println!("  - Add descriptions for complex variables");
    println!("  - Consider security implications of each variable");
    println!("  - Test the example file with your application");

    Ok(())
}
