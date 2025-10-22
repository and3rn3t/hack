# Justfile for The Hack: Ghost Protocol
# Modern command runner - install with: cargo install just
# Run `just` or `just --list` to see all commands

# Default recipe (runs when you just type `just`)
default:
    @just --list

# Build the project in debug mode
build:
    cargo build

# Build optimized release binary
release:
    cargo build --release

# Run the game in debug mode
run:
    cargo run

# Run the game in release mode
run-release:
    cargo run --release

# Run all tests
test:
    cargo test --verbose

# Run tests with output
test-nocapture:
    cargo test -- --nocapture

# Run cargo check
check:
    cargo check --all-features

# Format code with rustfmt
fmt:
    cargo fmt --all

# Check code formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Check for security vulnerabilities
audit:
    cargo audit

# Check dependencies with cargo-deny
deny:
    cargo deny check

# Check for outdated dependencies
outdated:
    cargo outdated

# Find unused dependencies (requires nightly)
udeps:
    cargo +nightly udeps

# Analyze binary size
bloat:
    cargo bloat --release

# Show dependency tree
tree:
    cargo tree

# Generate and open documentation
doc:
    cargo doc --open --no-deps

# Generate test coverage report (requires tarpaulin)
coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Clean build artifacts
clean:
    cargo clean

# Install recommended development tools
install-tools:
    cargo install cargo-audit
    cargo install cargo-deny
    cargo install cargo-outdated
    cargo install cargo-udeps
    cargo install cargo-bloat
    cargo install cargo-tarpaulin
    cargo install cargo-watch
    cargo install just

# Run all checks (format, lint, test)
full-check: fmt-check clippy test

# Run CI checks locally
ci: fmt-check clippy test audit deny

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch and run the game
watch-run:
    cargo watch -x run

# Update all dependencies
update:
    cargo update

# Fix clippy warnings automatically
fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Check compilation on all platforms (requires targets installed)
check-all:
    cargo check --target x86_64-pc-windows-msvc
    cargo check --target x86_64-unknown-linux-gnu
    cargo check --target x86_64-apple-darwin

# Build release for Windows
release-windows:
    cargo build --release --target x86_64-pc-windows-msvc

# Build release for Linux
release-linux:
    cargo build --release --target x86_64-unknown-linux-gnu

# Build release for macOS
release-macos:
    cargo build --release --target x86_64-apple-darwin

# Build for all platforms
release-all: release-windows release-linux release-macos

# Run the game with specific challenge (for testing)
test-challenge challenge:
    cargo run --release

# Quick dev workflow: format, check, test
dev: fmt check test

# Pre-commit checks
pre-commit: fmt clippy test
    @echo "✅ Pre-commit checks passed!"

# Verify terminal setup
verify-terminal:
    @powershell -File scripts/verify-terminal.ps1

# Verify CI configuration matches local setup
verify-ci:
    @powershell -File scripts/verify-ci-config.ps1
