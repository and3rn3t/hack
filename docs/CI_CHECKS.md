# CI/CD Check Configuration

This document describes the exact checks that run in CI/CD and how to run them locally.

## CI/CD Pipeline Checks

Our GitHub Actions CI/CD pipeline (`.github/workflows/ci.yml`) runs the following checks on every push and PR:

### 1. Format Check

```bash
cargo fmt --all -- --check
```

**Fails if**: Code is not formatted according to `rustfmt.toml`

### 2. Clippy Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Fails if**: Any clippy warnings are found
**Configuration**: `clippy.toml`

### 3. Tests

```bash
cargo test --all-features --verbose
```

**Fails if**: Any tests fail
**Current**: 34 tests, all passing

### 4. Cargo Check

```bash
cargo check --all-features
```

**Fails if**: Code doesn't compile

### 5. Security Audit

```bash
cargo audit
```

**Fails if**: Security vulnerabilities found in dependencies

### 6. Dependency Check

```bash
cargo deny check
```

**Fails if**: License violations, banned crates, or security issues
**Configuration**: `deny.toml`

### 7. Doc Tests

```bash
cargo test --doc --all-features
```

**Fails if**: Documentation examples fail

## Running CI Checks Locally

### Quick Method (using just)

```bash
just ci
```

### Quick Method (using make)

```bash
make ci
```

### Manual Method

```bash
# Run all checks in order
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features --verbose && \
cargo audit && \
cargo deny check
```

### Individual Checks

```bash
# Format check
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all-features --verbose

# Security
cargo audit

# Dependencies
cargo deny check
```

## IDE Configuration

### VS Code (rust-analyzer)

The `.vscode/settings.json` is configured to match CI/CD:

```jsonc
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.check.extraArgs": ["--all-targets", "--all-features", "--", "-D", "warnings"],
    "rust-analyzer.checkOnSave": true
}
```

This means:

-   **On save**: rust-analyzer runs clippy with the same flags as CI
-   **In editor**: You see the same errors/warnings that CI will catch
-   **Real-time**: Problems highlighted as you type

### Pre-commit Hooks

Install pre-commit hooks to catch issues before committing:

```bash
# Install pre-commit (one time)
pip install pre-commit
# or: brew install pre-commit

# Install hooks for this repo
pre-commit install

# Now fmt, clippy, and tests run automatically on commit
```

## Troubleshooting

### "My code passes locally but fails in CI"

**Common causes**:

1. Not running with `--all-features` flag
2. Not running with `--all-targets` flag
3. Different Rust version (use `rust-toolchain.toml`)
4. Cached build artifacts (`cargo clean`)

**Solution**:

```bash
cargo clean
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

### "CI is slow"

CI runs on multiple platforms (Ubuntu, Windows, macOS) and Rust versions (stable, nightly).
This is intentional for cross-platform compatibility.

Local checks are much faster:

```bash
just ci  # ~5-10 seconds
```

### "Failed to resolve patches"

Update dependencies:

```bash
cargo update
cargo check
```

## Platform Matrix

CI tests on:

-   **OS**: Ubuntu, Windows, macOS
-   **Rust**: stable, nightly
-   **Total**: 6 combinations

We ensure the game works everywhere!

## Configuration Files Reference

| File                  | Purpose             | CI Check            |
| --------------------- | ------------------- | ------------------- |
| `rustfmt.toml`        | Format rules        | `cargo fmt --check` |
| `clippy.toml`         | Lint rules          | `cargo clippy`      |
| `deny.toml`           | Dependency policies | `cargo deny check`  |
| `.cargo/config.toml`  | Cargo behavior      | -                   |
| `rust-toolchain.toml` | Rust version        | Ensures consistency |
| `.editorconfig`       | Editor settings     | -                   |

## Quick Reference Card

```bash
# Before committing
just pre-commit

# Before pushing
just ci

# To match CI exactly
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features --verbose && \
cargo audit && \
cargo deny check
```

---

**Remember**: If it passes `just ci` locally, it will pass in CI/CD! 🎉
