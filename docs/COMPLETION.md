# Shell Completion for env-cli

This document describes how to use shell completion with env-cli.

## Overview

env-cli provides comprehensive shell completion support for all major shells, including:

- **Bash**: Traditional bash shell completion
- **Zsh**: Advanced zsh completion with descriptions
- **Fish**: Modern fish shell completion
- **PowerShell**: Windows PowerShell completion

## Installation

### Generate Completion Script

To generate a completion script for your shell:

```bash
# Bash completion
env completion bash > ~/.local/share/bash-completion/completions/env

# Zsh completion
env completion zsh > ~/.zsh/completions/_env

# Fish completion
env completion fish > ~/.config/fish/completions/env.fish

# PowerShell completion
env completion powershell > $PROFILE
```

### Automatic Installation

You can also use the built-in installation command:

```bash
# Install completion for bash
env completion bash --install

# Install completion for zsh
env completion zsh --install

# Install completion for fish
env completion fish --install

# Install completion for PowerShell
env completion powershell --install
```

### Uninstallation

To uninstall completion:

```bash
# Uninstall completion for bash
env completion bash --uninstall

# Uninstall completion for zsh
env completion zsh --uninstall

# Uninstall completion for fish
env completion fish --uninstall

# Uninstall completion for PowerShell
env completion powershell --uninstall
```

## Usage

Once completion is installed and your shell is reloaded, you can use tab completion with env-cli:

### Command Completion

```bash
env <TAB>
# Shows: init, switch, scan, validate, sync, generate, status, completion
```

### Option Completion

```bash
env scan --<TAB>
# Shows: --format, --hidden, --help
```

### Environment Name Completion

```bash
env switch <TAB>
# Shows available environments from your project
```

### File and Directory Completion

```bash
env scan <TAB>
# Shows directories for scanning
```

## Shell-Specific Setup

### Bash

Add the following to your `~/.bashrc` or `~/.bash_profile`:

```bash
source ~/.local/share/bash-completion/completions/env
```

### Zsh

Add the following to your `~/.zshrc`:

```bash
fpath=(~/.zsh/completions $fpath)
autoload -U compinit
compinit
```

### Fish

Fish completion should work automatically after restarting your shell.

### PowerShell

PowerShell completion should work automatically after restarting your PowerShell session.

## Features

### Context-Aware Completion

The completion system provides context-aware suggestions:

- **Switch command**: Completes environment names from your project
- **Validate command**: Completes environment names including "current"
- **Scan command**: Completes directories and file paths
- **Generate command**: Completes file paths and options

### Option Descriptions

Zsh completion provides helpful descriptions for commands and options.

### Dynamic Environment Detection

Completion scripts automatically detect available environments in your current project.

## Troubleshooting

### Completion Not Working

1. Ensure the completion script is in the correct directory
2. Reload your shell or source your shell configuration
3. Check that the completion script is executable
4. Verify the script syntax for your shell

### Installation Issues

1. Check that the completion directory exists
2. Ensure you have write permissions
3. Manually create the directory if needed
4. Use the `--help` option for more information

### Environment Names Not Showing

1. Ensure you're in a project directory with env-cli initialized
2. Check that `.env/` directory exists
3. Verify environment files exist in `.env/`
4. Run `env status` to check available environments

## Examples

### Basic Usage

```bash
# Get command help with completion
env completion --help

# Generate and install bash completion
env completion bash --install

# Generate completion script for manual installation
env completion zsh > ~/.zsh/completions/_env
```

### Advanced Usage

```bash
# Install all completion scripts (run for each shell)
for shell in bash zsh fish powershell; do
    env completion $shell --install
done
```

## Development

If you're developing env-cli and want to test completion:

```bash
# Build the project
cargo build

# Test completion generation
./target/debug/env completion bash
./target/debug/env completion zsh
./target/debug/env completion fish
./target/debug/env completion powershell
```

## Contributing

To add support for a new shell:

1. Add the shell enum variant to `src/cli/completion.rs`
2. Implement the `generate_<shell>()` method
3. Update the `get_completion_dir()` function
4. Add tests for the new shell
5. Update this documentation

## Support

For issues with shell completion:

1. Check this documentation for common solutions
2. Open an issue on the GitHub repository
3. Include your shell type, OS, and error messages
4. Provide the output of `env --version`