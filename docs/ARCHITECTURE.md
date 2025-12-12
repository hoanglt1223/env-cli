# env-cli - Architecture & Technical Design

## Overview

env-cli is a **single binary CLI tool** built with Rust that provides comprehensive environment variable management for full-stack development projects. The architecture prioritizes **performance, security, and developer experience** while maintaining simplicity and determinism.

## Core Design Principles

1. **Single Static Binary**: Zero runtime dependencies, works everywhere
2. **Local-First**: Full functionality without cloud dependencies
3. **Security by Default**: Never expose secrets, production protection
4. **Deterministic Output**: Same input produces same output (CI/CD friendly)
5. **Unix Philosophy**: Each command does one thing well
6. **Performance**: Fast scanning and processing of large codebases

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    env-cli (Rust Binary)                    │
├─────────────────────────────────────────────────────────────┤
│  CLI Layer (Commands, Flags, Help)                         │
├─────────────────────────────────────────────────────────────┤
│  Core Engine                                                │
│  ├── Environment Manager                                    │
│  ├── Scanner Engine                                         │
│  ├── Validator Engine                                       │
│  ├── Secret Generator                                       │
│  └── Sync Engine                                            │
├─────────────────────────────────────────────────────────────┤
│  Language Parsers                                           │
│  ├── JavaScript/TypeScript                                  │
│  ├── Python                                                 │
│  ├── Go                                                     │
│  ├── Rust                                                   │
│  ├── Java                                                   │
│  └── PHP                                                    │
├─────────────────────────────────────────────────────────────┤
│  Utilities                                                  │
│  ├── File System Operations                                 │
│  ├── Pattern Matching (Regex)                               │
│  ├── Cryptographic Operations                               │
│  └── Configuration Management                               │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer                                              │
│  ├── .env Files                                             │
│  ├── Snapshots                                              │
│  ├── Config Files                                           │
│  └── Cache Files                                            │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. CLI Layer

**Responsibility**: Command parsing, validation, help generation, user interaction

**Key Modules**:
```rust
// Main CLI structure
pub struct Cli {
    pub command: Command,
    pub verbose: bool,
    pub quiet: bool,
    pub config: Option<PathBuf>,
}

// Command enum
pub enum Command {
    Init(InitCommand),
    Switch(SwitchCommand),
    Scan(ScanCommand),
    Validate(ValidateCommand),
    // ... other commands
}
```

**Features**:
- Structured argument parsing with `clap`
- Auto-generated help documentation
- Interactive prompts for dangerous operations
- Progress indicators for long-running operations
- Colorized output for better UX

### 2. Environment Manager

**Responsibility**: Managing multiple environments, switching, snapshots

**Key Structures**:
```rust
pub struct EnvironmentManager {
    project_root: PathBuf,
    config: Config,
}

pub struct Environment {
    name: String,
    path: PathBuf,
    variables: HashMap<String, EnvVariable>,
    metadata: EnvironmentMetadata,
}

pub struct EnvVariable {
    name: String,
    value: String,
    var_type: VariableType,
    comment: Option<String>,
    secret_level: SecretLevel,
}
```

**Core Operations**:
- **Switch**: Safe environment switching with backups
- **Snapshot**: Timestamped environment state saving
- **Diff**: Environment comparison with intelligent merging
- **Sync**: Secure environment synchronization with safety rules

### 3. Scanner Engine

**Responsibility**: Fast, accurate code scanning for environment variable usage

**Architecture**:
```
Scanner Engine
├── File Walker (parallel, configurable depth)
├── Language Detectors
├── Pattern Matchers (Regex + AST when possible)
├── Variable Collector
└── Usage Analyzer
```

**Language Detection & Parsing**:
```rust
pub trait LanguageParser {
    fn detect(&self, file_path: &Path) -> bool;
    fn parse(&self, content: &str) -> Result<Vec<EnvUsage>, ScanError>;
}

// Example: JavaScript/TypeScript Parser
pub struct JavaScriptParser;

impl LanguageParser for JavaScriptParser {
    fn detect(&self, file_path: &Path) -> bool {
        matches!(file_path.extension().and_then(|s| s.to_str()),
                 Some("js" | "jsx" | "ts" | "tsx"))
    }

    fn parse(&self, content: &str) -> Result<Vec<EnvUsage>, ScanError> {
        // Regex patterns + SWC AST for accurate parsing
        let patterns = vec![
            r#"process\.env\.([A-Z_][A-Z0-9_]*)"#,
            r#"import\.meta\.env\.([A-Z_][A-Z0-9_]*)"#,
        ];
        // Parse and return EnvUsage objects
    }
}
```

**Performance Optimizations**:
- Parallel file processing using Rayon
- Memory-efficient streaming for large files
- Cached results for repeated scans
- Intelligent exclusion patterns (node_modules, target, etc.)

**Detection Patterns**:
- **JavaScript/TypeScript**: `process.env.VAR`, `import.meta.env.VAR`
- **Python**: `os.getenv('VAR')`, `os.environ['VAR']`
- **Go**: `os.Getenv("VAR")`
- **Rust**: `env::var("VAR")`, `std::env::var("VAR")`
- **Java**: `System.getenv("VAR")`
- **PHP**: `$_ENV['VAR']`, `getenv('VAR')`
- **Ruby**: `ENV['VAR']`

### 4. Validator Engine

**Responsibility**: Environment validation, format checking, security validation

**Validation Rules**:
```rust
pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

pub trait ValidationRule {
    fn validate(&self, env: &Environment) -> Vec<ValidationIssue>;
}

// Example: URL validation
pub struct UrlValidationRule;

impl ValidationRule for UrlValidationRule {
    fn validate(&self, env: &Environment) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        for (name, var) in &env.variables {
            if name.contains("URL") && !is_valid_url(&var.value) {
                issues.push(ValidationIssue::new(
                    ValidationLevel::Error,
                    format!("{} has invalid URL format", name),
                ));
            }
        }
        issues
    }
}
```

**Built-in Validation Rules**:
- **Format validation**: URLs, emails, numbers, ports
- **Security validation**: Secret strength, insecure defaults
- **Required variables**: Based on code analysis
- **Consistency validation**: Variable naming, format consistency
- **Environment validation**: Dev values in production, etc.

### 5. Secret Generator

**Responsibility**: Cryptographically secure secret generation

**Design**:
```rust
pub struct SecretGenerator {
    rng: ChaCha20Rng,
}

impl SecretGenerator {
    pub fn generate_jwt_secret(&mut self) -> String {
        // Base64URL encoding, 64 bytes
    }

    pub fn generate_session_key(&mut self) -> String {
        // Hex encoding, 64 bytes
    }

    pub fn generate_api_key(&mut self) -> String {
        // Alphanumeric, 32 bytes, prefix-based
    }
}
```

**Security Features**:
- Uses `ChaCha20` CSPRNG for randomness
- Multiple encoding formats (hex, base64, base64url)
- Entropy requirements based on secret type
- Avoids ambiguous characters in user-facing secrets

### 6. Sync Engine

**Responsibility**: Safe environment synchronization

**Safety Mechanisms**:
```rust
pub struct SyncEngine {
    safety_rules: Vec<Box<dyn SafetyRule>>,
}

pub trait SafetyRule {
    fn should_sync(&self, var: &EnvVariable, from: &str, to: &str) -> SyncDecision;
}

// Example: Production protection
pub struct ProductionSafetyRule;

impl SafetyRule for ProductionSafetyRule {
    fn should_sync(&self, var: &EnvVariable, from: &str, to: &str) -> SyncDecision {
        if to == "prod" && var.secret_level == SecretLevel::Secret {
            SyncDecision::Deny("Cannot sync secrets to production".to_string())
        } else {
            SyncDecision::Allow
        }
    }
}
```

## Data Structures

### Environment Variable

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
    pub var_type: VariableType,
    pub secret_level: SecretLevel,
    pub comment: Option<String>,
    pub required: bool,
    pub source: VariableSource,
    pub last_modified: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Url,
    Email,
    Path,
    Json,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretLevel {
    Public,      // Safe to share
    Sensitive,   // Should be protected
    Secret,      // Must be encrypted/handled carefully
}
```

### Scan Results

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub variables: HashMap<String, VariableUsage>,
    pub files_scanned: usize,
    pub total_files: usize,
    pub scan_duration: Duration,
    pub errors: Vec<ScanError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableUsage {
    pub name: String,
    pub usage_count: usize,
    pub files: Vec<FileUsage>,
    pub contexts: Vec<UsageContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUsage {
    pub path: PathBuf,
    pub line_numbers: Vec<usize>,
    pub language: String,
}
```

## File Organization

```
src/
├── main.rs                 # CLI entry point
├── cli/                    # Command-line interface
│   ├── mod.rs
│   ├── commands/           # Individual command implementations
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── scan.rs
│   │   └── ...
│   └── args.rs             # Command-line argument definitions
├── core/                   # Core business logic
│   ├── mod.rs
│   ├── env_manager.rs      # Environment management
│   ├── scanner.rs          # Code scanning engine
│   ├── validator.rs        # Validation engine
│   ├── secrets.rs          # Secret generation
│   ├── sync.rs             # Environment synchronization
│   └── config.rs           # Configuration management
├── parsers/                # Language-specific parsers
│   ├── mod.rs
│   ├── javascript.rs
│   ├── python.rs
│   ├── go.rs
│   ├── rust.rs
│   └── java.rs
├── utils/                  # Utility modules
│   ├── mod.rs
│   ├── file_ops.rs         # File system operations
│   ├── crypto.rs           # Cryptographic utilities
│   ├── patterns.rs         # Regex patterns
│   └── output.rs           # Output formatting
├── types/                  # Common types
│   ├── mod.rs
│   ├── env_var.rs          # Environment variable types
│   ├── scan_result.rs      # Scan result types
│   └── validation.rs       # Validation types
└── tests/                  # Integration tests
    ├── mod.rs
    ├── fixtures/           # Test fixtures
    └── integration/        # Integration test modules
```

## Performance Characteristics

### Scanning Performance

- **Small projects** (< 100 files): < 100ms
- **Medium projects** (< 1000 files): < 500ms
- **Large projects** (< 10000 files): < 2s
- **Enterprise projects** (> 10000 files): < 10s

**Optimizations**:
- Parallel file processing
- Memory-efficient streaming
- Intelligent caching
- Configurable depth limits
- Efficient regex patterns

### Memory Usage

- **Base memory**: ~10MB
- **Large codebases**: Additional 1-5MB depending on file count
- **No memory leaks**: All operations are bounded and cleaned up

## Security Considerations

### Secret Handling

1. **Never log secrets**: All secret values are masked in logs
2. **Secure random generation**: Uses OS CSPRNG (ChaCha20)
3. **Memory cleanup**: Secrets are zeroed from memory after use
4. **File permissions**: .env files get restricted permissions (600)

### Production Safety

1. **Confirmation prompts**: Dangerous operations require confirmation
2. **Backup creation**: Automatic backups before destructive operations
3. **Read-only mode**: Optional read-only mode for production environments
4. **Audit logging**: All operations logged for security auditing

### Code Security

1. **Memory safety**: Rust's ownership system prevents buffer overflows
2. **Input validation**: All user inputs validated and sanitized
3. **Dependency management**: Minimal dependencies, regularly audited
4. **Static analysis**: Automated security scanning in CI/CD

## Extension Points

### Plugin Architecture

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, context: &PluginContext) -> Result<PluginResult, PluginError>;
}

// Example: Secret manager plugin
pub struct InfisicalPlugin;

impl Plugin for InfisicalPlugin {
    fn name(&self) -> &str { "infisical" }
    fn version(&self) -> &str { "1.0.0" }
    fn execute(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        // Implementation for Infisical integration
    }
}
```

### Custom Validation Rules

Users can add custom validation rules through configuration:

```yaml
custom_rules:
  - name: "company_api_key_format"
    pattern: "^[A-Z]{3}_[a-f0-9]{32}$"
    variables: ["API_KEY", "SECRET_KEY"]
    error_message: "API keys must follow COMPANY_32hex format"
```

## Testing Strategy

### Unit Tests
- Core logic validation
- Parser accuracy tests
- Secret generation verification
- Configuration parsing

### Integration Tests
- End-to-end command workflows
- Real project scanning
- Multi-language support
- File system interactions

### Performance Tests
- Large codebase scanning benchmarks
- Memory usage profiling
- Concurrent operation testing

### Security Tests
- Secret handling verification
- Input validation testing
- Permission checking
- Attack surface analysis

## Deployment & Distribution

### Binary Distribution
- **Single static binary** - no dependencies required
- **Cross-platform** - Windows, macOS, Linux (x64, ARM)
- **Package managers** - Cargo, Homebrew, Scoop, apt, yum
- **CI/CD integration** - GitHub Actions, GitLab CI, Jenkins

### Installation Methods

```bash
# Cargo (crates.io)
cargo install env-cli

# Direct download
curl -L https://github.com/your-org/env-cli/releases/latest/download/env-cli.tar.gz | tar xz

# Package managers
brew install env-cli         # macOS
scoop install env-cli        # Windows
apt install env-cli          # Debian/Ubuntu
```

### Release Process
1. **Automated testing** - Full test suite passes
2. **Security audit** - Dependencies scanned
3. **Performance testing** - Benchmarks verified
4. **Multi-platform builds** - All targets compiled
5. **Asset signing** - Binaries signed for security
6. **Release notes** - Comprehensive changelog

This architecture provides a solid foundation for a reliable, performant, and secure environment variable management tool that can scale from small personal projects to large enterprise codebases.