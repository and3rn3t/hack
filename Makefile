# Makefile for The Hack: Ghost Protocol
# Convenient commands for common development tasks

.PHONY: help build run test check fmt clippy audit deny coverage clean install-tools

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in debug mode
	cargo build

release: ## Build optimized release binary
	cargo build --release

run: ## Run the game in debug mode
	cargo run

run-release: ## Run the game in release mode
	cargo run --release

test: ## Run all tests
	cargo test --verbose

test-nocapture: ## Run tests with output
	cargo test -- --nocapture

check: ## Run cargo check
	cargo check --all-features

fmt: ## Format code with rustfmt
	cargo fmt --all

fmt-check: ## Check code formatting without modifying
	cargo fmt --all -- --check

clippy: ## Run clippy lints
	cargo clippy --all-targets --all-features -- -D warnings

audit: ## Check for security vulnerabilities
	cargo audit

deny: ## Check dependencies with cargo-deny
	cargo deny check

outdated: ## Check for outdated dependencies
	cargo outdated

udeps: ## Find unused dependencies (requires nightly)
	cargo +nightly udeps

bloat: ## Analyze binary size
	cargo bloat --release

tree: ## Show dependency tree
	cargo tree

doc: ## Generate and open documentation
	cargo doc --open --no-deps

coverage: ## Generate test coverage report (requires tarpaulin)
	cargo tarpaulin --out Html --output-dir coverage

clean: ## Clean build artifacts
	cargo clean

install-tools: ## Install recommended development tools
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-outdated
	cargo install cargo-udeps
	cargo install cargo-bloat
	cargo install cargo-tarpaulin

full-check: fmt-check clippy test ## Run all checks (format, lint, test)

ci: fmt-check clippy test audit deny ## Run CI checks locally

watch: ## Watch for changes and run tests
	cargo watch -x test

# Platform-specific release builds
release-windows: ## Build Windows release
	cargo build --release --target x86_64-pc-windows-msvc

release-linux: ## Build Linux release
	cargo build --release --target x86_64-unknown-linux-gnu

release-macos: ## Build macOS release
	cargo build --release --target x86_64-apple-darwin
