# env-cli - CLI Specification

## env --help (Main Help)

```
env-cli 1.0.0
The missing CLI for environment variable management

USAGE:
    env [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

DESCRIPTION:
    env-cli manages the entire lifecycle of environment variables for full-stack projects.

    Key capabilities:
    • Create and switch between multiple environments
    • Scan code to detect actual env usage
    • Generate and validate .env.example files
    • Sync and compare environments safely
    • Generate strong secrets and validate configurations

    Works locally first with optional cloud integrations.

EXAMPLES:
    env init                    # Initialize project with env structure
    env switch prod             # Switch to production environment
    env scan                    # Scan code for env variable usage
    env validate --env prod     # Validate production environment

GET STARTED:
    1. Run 'env init' in your project directory
    2. Run 'env scan' to detect env usage
    3. Run 'env example' to generate .env.example
    4. Use 'env switch <env>' to change environments

For detailed help on any command, run: env <command> --help
```

## Command Specifications

### env init --help

```
env-init
Initialize project with environment structure

USAGE:
    env init [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -f, --force      Overwrite existing env files
        --template   Use predefined template (basic|fullstack|microservice)

OPTIONS:
        --envs <ENVIRONMENTS>    Comma-separated environment names [default: local,dev,test,uat,prod]

DESCRIPTION:
    Creates the initial environment structure for your project.

    Creates these files:
    • .env.example      # Generated from code scanning
    • .env.local        # Your local development environment
    • .env.<env>        # Environment-specific files
    • .env/config.yaml  # Optional configuration file

EXAMPLES:
    env init                           # Create default structure
    env init --envs dev,staging,prod   # Custom environment names
    env init --template fullstack      # Fullstack template
    env init --force                   # Overwrite existing files

TEMPLATES:
    basic           # Simple web application
    fullstack       # Backend + frontend project
    microservice    # Microservice architecture
```

### env switch --help

```
env-switch
Switch between environments

USAGE:
    env switch <ENVIRONMENT> [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -y, --yes        Skip confirmation prompts

ARGS:
    <ENVIRONMENT>    Target environment name

DESCRIPTION:
    Switches the active environment by copying .env.<ENV> to .env.

    • Backs up current .env to .env.backup
    • Validates target environment before switching
    • Prevents accidental production switches without confirmation

EXAMPLES:
    env switch dev              # Switch to development
    env switch prod -y          # Switch to production (skip confirm)
    env switch staging          # Switch to staging
```

### env scan --help

```
env-scan
Scan code for environment variable usage

USAGE:
    env scan [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Detailed output with file locations

OPTIONS:
        --include <PATTERNS>    File patterns to include [default: **/*.{js,ts,jsx,tsx,py,go,rs,java,php,rb}]
        --exclude <PATTERNS>    File patterns to exclude [default: node_modules,target,dist,build,.git]
        --max-depth <DEPTH>     Maximum directory depth to scan [default: 10]

DESCRIPTION:
    Scans your codebase to find all environment variable usage patterns.

    Detection patterns:
    • process.env.VAR (Node.js)
    • os.getenv('VAR') (Python)
    • os.Getenv("VAR") (Go)
    • env::var("VAR") (Rust)
    • System.getenv("VAR") (Java)
    • $_ENV['VAR'] (PHP)
    • ENV['VAR'] (Ruby)

OUTPUT:
    • List of all used environment variables
    • Count of usage per variable
    • Files where each variable is used
    • Unused variables in .env files

EXAMPLES:
    env scan                              # Scan all files
    env scan --verbose                    # Show file locations
    env scan --include "**/*.py"          # Python files only
    env scan --exclude "test/**"          # Exclude test files
```

### env example --help

```
env-example
Generate .env.example from code scanning

USAGE:
    env example [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -f, --force      Overwrite existing .env.example
        --no-secrets Exclude secret-like variables
        --comments   Add descriptive comments

DESCRIPTION:
    Generates .env.example based on actual code usage.

    Features:
    • Scans code to find used variables
    • Excludes detected secrets automatically
    • Adds type hints and descriptions when available
    • Sorts variables alphabetically
    • Groups related variables together

SECRET DETECTION:
    Variables containing these patterns are flagged as secrets:
    • secret, key, token, password, pass
    • private, auth, credential
    • jwt, session, cookie
    • api_key, access_key, secret_key

EXAMPLES:
    env example                          # Generate .env.example
    env example --force                  # Overwrite existing
    env example --comments               # Add helpful comments
    env example --no-secrets             # Exclude all secret-like vars
```

### env dead --help

```
env-dead
Find unused environment variables

USAGE:
    env dead [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --remove     Remove unused variables from .env files

OPTIONS:
        --env <ENV>  Check specific environment [default: .env]

DESCRIPTION:
    Identifies environment variables that are defined but not used in code.

    Process:
    1. Scan code for all used environment variables
    2. Compare with variables defined in .env files
    3. Report unused variables

OUTPUT:
    • List of unused variables per environment
    • Confidence score for each detection
    • Suggested actions (safe to remove, review needed)

EXAMPLES:
    env dead                     # List unused variables
    env dead --remove           # Remove unused vars from .env
    env dead --env .env.prod    # Check production env only
```

### env lint --help

```
env-lint
Lint environment files for issues

USAGE:
    env lint [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --strict     Use strict rules (fail on warnings)
        --fix        Auto-fix minor issues

OPTIONS:
        --env <ENV>  Lint specific environment [default: all]

DESCRIPTION:
    Analyzes .env files for common issues and best practices.

CHECKS:
    • Missing comments for variables
    • Weak secrets (short length, simple patterns)
    • Insecure default values
    • Malformed variable names
    • Duplicate variables
    • Required vs optional variables
    • Type mismatches
    • Deprecated variable names

EXAMPLES:
    env lint                    # Lint all env files
    env lint --strict          # Strict mode (warnings = errors)
    env lint --fix             # Auto-fix simple issues
    env lint --env .env.prod   # Lint production only
```

### env format --help

```
env-format
Format and standardize environment files

USAGE:
    env format [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --check      Check formatting without making changes

OPTIONS:
        --env <ENV>  Format specific environment [default: all]

DESCRIPTION:
    Standardizes the format of .env files for consistency.

FORMATTING RULES:
    • Alphabetical ordering
    • Consistent spacing around =
    • Remove duplicate variables
    • Standardize line endings
    • Remove empty lines at start/end
    • Group related variables

EXAMPLES:
    env format                   # Format all env files
    env format --check          # Check if formatting needed
    env format --env .env.dev   # Format development env
```

### env comment --help

```
env-comment
Add descriptive comments to environment variables

USAGE:
    env comment [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --auto       Generate automatic comments
        --overwrite  Overwrite existing comments

OPTIONS:
        --env <ENV>  Comment specific environment [default: all]

DESCRIPTION:
    Adds helpful comments to explain environment variables.

COMMENT TYPES:
    • Type hints (string, number, boolean, url, email)
    • Required/optional status
    • Default values
    • Security level (public, sensitive, secret)
    • Purpose descriptions

EXAMPLES:
    env comment                      # Add comments to all vars
    env comment --auto              # Auto-generate comments
    env comment --overwrite         # Replace existing comments
    env comment --env .env.prod     # Comment production only
```

### env explain --help

```
env-explain
Explain environment variables

USAGE:
    env explain [VARIABLE] [FLAGS]

FLAGS:
    -h, --help       Prints help information
        --all        Show all variables
        --json       Output in JSON format

ARGS:
    <VARIABLE>       Specific variable to explain

DESCRIPTION:
    Provides detailed information about environment variables.

INFORMATION PROVIDED:
    • Usage locations in code
    • Variable type and format
    • Security classification
    • Default values
    • Related variables
    • Migration history (if available)

EXAMPLES:
    env explain DATABASE_URL       # Explain specific variable
    env explain --all             # Explain all variables
    env explain --json API_KEY    # JSON output for automation
```

### env secrets generate --help

```
env-secrets-generate
Generate strong secrets

USAGE:
    env secrets generate [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --all        Generate all missing secrets

OPTIONS:
        --type <TYPE>        Type of secret [jwt|session|api|db|encryption|webhook]
        --length <LENGTH>    Secret length [default: varies by type]
        --var <VAR_NAME>     Specific variable name

DESCRIPTION:
    Generates cryptographically secure secrets for environment variables.

SECRET TYPES:
    jwt           # JWT secret (64 chars, base64url)
    session       # Session key (64 chars, hex)
    api           # API key (32 chars, alphanumeric)
    db            # Database password (32 chars, printable)
    encryption    # Encryption key (64 chars, hex)
    webhook       # Webhook secret (32 chars, hex)

EXAMPLES:
    env secrets generate                    # Interactive mode
    env secrets generate --all             # Generate all missing
    env secrets generate --type jwt        # Generate JWT secret
    env secrets generate --var SECRET_KEY  # Generate for specific var
    env secrets generate --length 128      # Custom length
```

### env validate --help

```
env-validate
Validate environment configuration

USAGE:
    env validate [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --strict     Fail on warnings as well as errors

OPTIONS:
        --env <ENV>  Validate specific environment [default: .env]

DESCRIPTION:
    Validates environment configuration against code requirements.

VALIDATION CHECKS:
    • All required variables present
    • Variable formats correct (URLs, emails, numbers)
    • Secret strength requirements
    • Database connection strings format
    • Port numbers in valid ranges
    • File paths exist and accessible
    • No production values in development

EXIT CODES:
    0    # Validation passed
    1    # Validation failed (errors)
    2    # Validation passed with warnings

EXAMPLES:
    env validate                    # Validate .env
    env validate --env .env.prod   # Validate production
    env validate --strict          # Treat warnings as errors
```

### env run --help

```
env-run
Run command with environment validation

USAGE:
    env run <COMMAND> [ARGS]... [FLAGS]

FLAGS:
    -h, --help       Prints help information

ARGS:
    <COMMAND>        Command to execute
    <ARGS>...        Command arguments

DESCRIPTION:
    Executes a command after validating the environment.

PROCESS:
    1. Validate current environment
    2. Run the provided command if validation passes
    3. Return command exit code

EXAMPLES:
    env run npm start                    # Start Node.js app after validation
    env run python manage.py runserver  # Start Django server
    env run cargo run                   # Run Rust application
    env run "npm run build && npm run deploy"  # Complex command
```

### env sync --help

```
env-sync
Sync environments

USAGE:
    env sync <FROM> -> <TO> [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --dry-run    Show what would be synced without making changes
        --safe       Skip secret-like variables (default)

OPTIONS:
        --include <VARS>    Only sync specific variables
        --exclude <VARS>    Exclude specific variables

ARGS:
    <FROM>           Source environment
    <TO>             Target environment

DESCRIPTION:
    Synchronizes variables between environments safely.

SAFETY RULES:
    • Skip secret-like variables by default
    • Require confirmation for production syncs
    • Create backup before overwriting
    • Validate format before applying

EXAMPLES:
    env sync dev -> test           # Sync dev to test (safe mode)
    env sync dev -> prod --dry-run  # Preview prod sync
    env sync .env.dev -> test     # Sync from file to environment
    env sync staging -> prod --exclude "DATABASE_*"  # Exclude DB vars
```

### env snapshot --help

```
env-snapshot
Save environment state snapshot

USAGE:
    env snapshot <ENVIRONMENT> [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information

OPTIONS:
        --name <NAME>    Custom snapshot name [default: timestamp]

ARGS:
    <ENVIRONMENT>        Environment to snapshot

DESCRIPTION:
    Creates a timestamped snapshot of environment state.

SNAPSHOT CONTENT:
    • All variable values
    • Metadata (timestamp, environment, git commit)
    • Validation status
    • File hash for integrity

SNAPSHOTS SAVED TO:
    .env/snapshots/<env>/<name>.env

EXAMPLES:
    env snapshot prod               # Create production snapshot
    env snapshot dev --name backup  # Named snapshot
    env snapshot .env               # Snapshot current env
```

### env diff --help

```
env-diff
Compare environments

USAGE:
    env diff <ENV1> <ENV2> [FLAGS]

FLAGS:
    -h, --help       Prints help information
        --json       Output in JSON format

ARGS:
    <ENV1>           First environment
    <ENV2>           Second environment

DESCRIPTION:
    Compares two environments and shows differences.

COMPARISON TYPES:
    • Variables only in ENV1
    • Variables only in ENV2
    • Variables with different values
    • Variables with different types
    • Secret strength differences

EXAMPLES:
    env diff dev prod             # Compare dev vs prod
    env diff .env.dev .env.prod   # Compare specific files
    env diff staging prod --json  # JSON output
```

### env detect services --help

```
env-detect-services
Detect local development services

USAGE:
    env detect services [FLAGS]

FLAGS:
    -h, --help       Prints help information
        --auto       Auto-configure detected services
        --list       List detected services only

DESCRIPTION:
    Scans for common development services running locally.

DETECTABLE SERVICES:
    • Databases: PostgreSQL, MySQL, MongoDB, Redis
    • Message queues: RabbitMQ, Kafka
    • Search: Elasticsearch, OpenSearch
    • Cache: Redis, Memcached
    • Storage: MinIO, LocalStack

OUTPUT:
    • Service type and version
    • Connection details (host, port)
    • Suggested environment variables

EXAMPLES:
    env detect services          # Detect and report
    env detect services --auto   # Auto-configure env
    env detect services --list   # List only
```

### env link --help

```
env-link
Link backend and frontend configurations

USAGE:
    env link <SOURCE> <TARGET> [FLAGS]

FLAGS:
    -h, --help       Prints help information
        --dry-run    Show changes without applying

ARGS:
    <SOURCE>         Backend directory or file
    <TARGET>         Frontend directory or file

DESCRIPTION:
    Links backend environment configuration to frontend.

LINKING RULES:
    • API_URL → VITE_API_URL / REACT_APP_API_URL
    • PORT → Frontend proxy settings
    • CORS configuration
    • WebSocket endpoints
    • Authentication endpoints

EXAMPLES:
    env link backend frontend       # Link directories
    env link .env ../frontend      # Link specific files
    env link backend ../frontend --dry-run  # Preview
```

### env doctor --help

```
env-doctor
Health check and diagnostics

USAGE:
    env doctor [FLAGS]

FLAGS:
    -h, --help       Prints help information
        --fix        Attempt to fix found issues
        --verbose    Detailed diagnostic output

DESCRIPTION:
    Performs comprehensive health check of environment setup.

HEALTH CHECKS:
    • File permissions on .env files
    • .env files in .gitignore
    • Environment consistency
    • Secret strength assessment
    • Code coverage analysis
    • Integration status

EXAMPLES:
    env doctor                # Run health check
    env doctor --fix         # Fix issues automatically
    env doctor --verbose     # Detailed output
```

## Global Options

All commands support these global options:
- `-h, --help`: Show help
- `-V, --version`: Show version
- `--config <FILE>`: Use custom config file
- `--verbose`: Verbose output
- `--quiet`: Minimal output

## Configuration File

Optional `.env/config.yaml`:

```yaml
# env-cli configuration
environments:
  - local
  - dev
  - test
  - uat
  - prod

scan:
  include:
    - "**/*.{js,ts,jsx,tsx}"
    - "**/*.{py,go,rs}"
  exclude:
    - node_modules
    - target
    - .git

secrets:
  min_length:
    jwt: 64
    session: 64
    api_key: 32

sync:
  safe_mode: true
  require_confirmation:
    - prod
    - production

validation:
  strict_mode: false
  fail_on_warnings: false

formats:
  sort_variables: true
  group_by_type: true
  add_comments: true
```