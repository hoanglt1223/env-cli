# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of env-cli seriously. If you believe you have found a security vulnerability, please report it to us as described below.

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: [security@env-cli.org](mailto:security@env-cli.org)

If you prefer encrypted communication, please use our PGP key (available on request).

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information in your report:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Disclosure Policy

When we receive a security bug report, we will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any similar problems
3. Prepare fixes for all supported releases
4. Release new versions as soon as possible

## Comments on this Policy

If you have suggestions on how this process could be improved, please submit a pull request.

## Security Best Practices for Users

When using env-cli:

1. **Never commit `.env` files** to version control
2. **Use strong secrets** - env-cli can generate cryptographically secure secrets
3. **Validate production environments** before deployment
4. **Review sensitive patterns** in your configuration
5. **Keep env-cli updated** to the latest version
6. **Use appropriate file permissions** for `.env` files (e.g., `chmod 600 .env`)
7. **Rotate secrets regularly**, especially after team member changes
8. **Use different values** for development and production environments

## Known Security Considerations

- env-cli stores environment variables in plain text files by design
- Secret generation uses the operating system's cryptographically secure random number generator
- Environment variable validation helps prevent common security mistakes
- Production environment protection features help prevent accidental changes

