# Development Tools & Linting

This document covers all the linting, checking, and quality assurance tools configured for The Hack: Ghost Protocol.

## 📋 Quick Reference

```bash
# Format code
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run linter
cargo clippy

# Run tests
cargo test

# Security audit
cargo audit

# Dependency checks
cargo deny check

# Run ALL checks (what CI runs)
just ci
# or
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test && cargo audit && cargo deny check
```

## 🛠️ Installed Tools

### Core Rust Tools (Built-in)

#### rustfmt

**Purpose**: Code formatting
**Config**: `rustfmt.toml`
**Usage**:

```bash
cargo fmt              # Format all code
cargo fmt --check      # Check if formatted
```

#### Clippy

**Purpose**: Linting and best practices
**Config**: `clippy.toml`, `.cargo/config.toml`
**Usage**:

```bash
cargo clippy                                                     # Basic linting
cargo clippy --all-targets --all-features -- -D warnings        # Strict mode (what CI uses)
cargo clippy --fix                                              # Auto-fix warnings
```

**Enabled Lints**:

-   `clippy::all` - All basic lints
-   `clippy::pedantic` - Extra pedantic lints
-   `clippy::nursery` - Experimental lints

**Disabled Lints** (too noisy):

-   `module_name_repetitions`
-   `must_use_candidate`
-   `missing_errors_doc`
-   `missing_panics_doc`

### Additional Tools (Require Installation)

#### cargo-audit

**Purpose**: Security vulnerability scanning
**Install**: `cargo install cargo-audit`
**Usage**:

```bash
cargo audit                    # Check for known vulnerabilities
cargo audit --deny warnings    # Fail on any advisory
```

#### cargo-deny

**Purpose**: Dependency policy enforcement
**Config**: `deny.toml`
**Install**: `cargo install cargo-deny`
**Usage**:

```bash
cargo deny check              # Run all checks
cargo deny check advisories   # Only security advisories
cargo deny check licenses     # Only license compliance
cargo deny check bans         # Only banned crates
cargo deny check sources      # Only source verification
```

**Checks**:

-   ✅ Security vulnerabilities
-   ✅ License compliance (allows MIT, Apache-2.0, BSD)
-   ✅ Banned/duplicate dependencies
-   ✅ Source registry verification

#### cargo-outdated (Optional)

**Purpose**: Find outdated dependencies
**Install**: `cargo install cargo-outdated`
**Usage**:

```bash
cargo outdated              # Check for updates
cargo outdated -R           # Include root dependencies only
```

#### cargo-udeps (Optional)

**Purpose**: Find unused dependencies
**Requires**: Nightly Rust
**Install**: `cargo install cargo-udeps`
**Usage**:

```bash
cargo +nightly udeps        # Find unused deps
```

#### cargo-tarpaulin (Optional)

**Purpose**: Code coverage
**Install**: `cargo install cargo-tarpaulin`
**Usage**:

```bash
cargo tarpaulin --out Html --output-dir coverage
```

#### cargo-bloat (Optional)

**Purpose**: Binary size analysis
**Install**: `cargo install cargo-bloat`
**Usage**:

```bash
cargo bloat --release       # What takes up space
```

#### cargo-watch (Optional)

**Purpose**: Auto-run commands on file changes
**Install**: `cargo install cargo-watch`
**Usage**:

```bash
cargo watch -x test         # Re-run tests on changes
cargo watch -x run          # Re-run game on changes
```

## 📝 Configuration Files

| File                 | Purpose               | Documentation                                                         |
| -------------------- | --------------------- | --------------------------------------------------------------------- |
| `rustfmt.toml`       | Code formatting rules | [rustfmt docs](https://rust-lang.github.io/rustfmt/)                  |
| `clippy.toml`        | Clippy configuration  | [clippy docs](https://doc.rust-lang.org/clippy/)                      |
| `deny.toml`          | Dependency policies   | [cargo-deny docs](https://embarkstudios.github.io/cargo-deny/)        |
| `.editorconfig`      | Editor settings       | [EditorConfig](https://editorconfig.org/)                             |
| `.cargo/config.toml` | Cargo behavior        | [Cargo config](https://doc.rust-lang.org/cargo/reference/config.html) |

## 🚀 Command Runners

We provide two ways to run common commands easily:

### justfile (Recommended for Windows)

Install `just`: `cargo install just`

```bash
just              # List all commands
just build        # Build project
just test         # Run tests
just ci           # Run all CI checks locally
just dev          # Quick dev workflow (fmt, check, test)
just install-tools # Install all recommended tools
```

### Makefile (Unix-style)

```bash
make              # Show help
make build        # Build project
make test         # Run tests
make ci           # Run all CI checks locally
make install-tools # Install all recommended tools
```

## 🔄 Pre-commit Hooks

Install pre-commit: `pip install pre-commit` or `brew install pre-commit`

```bash
pre-commit install              # Install hooks
pre-commit run --all-files      # Run manually
```

**Hooks configured** (`.pre-commit-config.yaml`):

-   ✅ `cargo fmt --check`
-   ✅ `cargo clippy`
-   ✅ `cargo test`
-   ✅ YAML/TOML/JSON validation
-   ✅ Trailing whitespace removal
-   ✅ Markdown linting
-   ✅ Secret detection

## 🤖 CI/CD Checks

GitHub Actions runs these checks on every push/PR (`.github/workflows/ci.yml`):

1. **Format Check**: `cargo fmt --check`
2. **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
3. **Tests**: `cargo test --all-features --verbose`
4. **Security Audit**: `cargo audit`
5. **Dependency Check**: `cargo deny check`
6. **Multi-platform**: Ubuntu, Windows, macOS
7. **Multi-version**: Stable & Nightly Rust

## 🎯 Workflows

### Before Committing

```bash
just pre-commit
# or
cargo fmt && cargo clippy && cargo test
```

### Before Pull Request

```bash
just ci
# or
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test && cargo audit && cargo deny check
```

### Weekly Maintenance

```bash
cargo outdated              # Check for updates
cargo update                # Update dependencies
cargo deny check            # Verify new versions
cargo test                  # Ensure still works
```

## 🔍 VS Code Integration

Tasks are configured in `.vscode/tasks.json`:

-   **Ctrl+Shift+B**: Run default build task
-   **Terminal → Run Task**: Access all configured tasks
    -   Build, Run, Test
    -   Format, Clippy
    -   Audit, Deny Check
    -   Full CI Check

## 🚨 Common Issues

### Clippy Warnings

```bash
# Fix automatically (safe changes)
cargo clippy --fix

# Fix with staged changes
cargo clippy --fix --allow-dirty --allow-staged
```

### Formatting Issues

```bash
# Format everything
cargo fmt --all

# Check what would change
cargo fmt --all -- --check
```

### Security Vulnerabilities

```bash
# Update dependencies
cargo update

# Check again
cargo audit

# If persistent, see advisory details and add exception in deny.toml if necessary
```

### License Issues

```bash
# Check licenses
cargo deny check licenses

# View dependency tree to find problematic crate
cargo tree

# Add to allow list in deny.toml if acceptable
```

## 📊 Recommended Workflow

**Daily Development**:

```bash
1. Make changes
2. cargo fmt
3. cargo clippy --fix
4. cargo test
5. git commit
```

**Before Push**:

```bash
just ci
# Review output
git push
```

**Weekly**:

```bash
cargo outdated
cargo update
cargo test
cargo audit
cargo deny check
```

## 🎓 Learning Resources

-   [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
-   [Clippy Lint List](https://rust-lang.github.io/rust-clippy/master/)
-   [cargo-deny Guide](https://embarkstudios.github.io/cargo-deny/)
-   [Rust Security Advisories](https://rustsec.org/)

---

**Last Updated**: October 21, 2025
