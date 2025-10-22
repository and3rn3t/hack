# Development Setup Guide

## Table of Contents

-   [Prerequisites](#prerequisites)
-   [Initial Setup](#initial-setup)
-   [Development Environment](#development-environment)
-   [Configuration Files](#configuration-files)
-   [Running the Project](#running-the-project)
-   [Testing](#testing)
-   [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Software

1. **Rust** (1.70.0 or later)

    - Install from: <https://rustup.rs/>
    - Verify: `rustc --version`

2. **Git**

    - Download from: <https://git-scm.com/>
    - Verify: `git --version`

3. **Terminal Emulator with ANSI Support**
    - **Windows**: Windows Terminal (recommended) or PowerShell 7+
    - **Linux**: GNOME Terminal, Konsole, or any modern terminal
    - **macOS**: iTerm2 or built-in Terminal.app

### Optional Tools

-   **VS Code** with Rust extensions (recommended)
-   **cargo-watch** for auto-rebuild: `cargo install cargo-watch`
-   **cargo-edit** for dependency management: `cargo install cargo-edit`
-   **cargo-audit** for security audits: `cargo install cargo-audit`

## Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/and3rn3t/hack.git
cd hack
```

### 2. Verify Rust Installation

```bash
rustc --version
cargo --version
```

Should show Rust 1.70.0 or later.

### 3. Build the Project

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### 4. Run the Game

```bash
# Debug mode
cargo run

# Release mode (better performance)
cargo run --release
```

### Terminal Setup

#### Importance

This is a **terminal-based game** with ANSI colors and Unicode box-drawing characters. A properly configured terminal is essential for the best experience.

#### Quick Check

Run the terminal verification script:

```bash
# Windows
pwsh scripts/verify-terminal.ps1

# Linux/macOS
bash scripts/verify-terminal.sh
```

#### Recommended Terminals

-   **Windows**: Windows Terminal (install: `winget install Microsoft.WindowsTerminal`)
-   **Linux**: GNOME Terminal, Alacritty, or Kitty
-   **macOS**: iTerm2 or Terminal.app

#### Quick Setup

For automatic setup on Windows:

```powershell
pwsh scripts/setup-terminal.ps1
```

**For detailed terminal configuration, see [TERMINAL_SETUP.md](TERMINAL_SETUP.md)**

## Development Environment

### VS Code Setup

#### 1. Install Recommended Extensions

VS Code will prompt you to install recommended extensions. Accept the prompt or manually install:

-   `rust-lang.rust-analyzer` - Rust language support
-   `serayuzgur.crates` - Cargo.toml dependency management
-   `tamasfe.even-better-toml` - TOML language support
-   `eamodio.gitlens` - Enhanced Git integration
-   `github.copilot` - AI pair programming (if available)

#### 2. Configure Settings

The project includes `.vscode/settings.json` with optimal settings for Rust development.

Key settings:

-   Auto-format on save with `rustfmt`
-   Clippy for linting
-   Inlay hints for type information
-   Proper tab/space configuration

#### 3. Available Tasks

Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) and type "Tasks: Run Task" to see:

-   `cargo build` - Build debug version
-   `cargo run` - Run the game
-   `cargo test` - Run tests
-   `cargo clippy` - Lint code
-   `cargo fmt` - Format code

### Terminal Setup

#### Windows (PowerShell)

```powershell
# Set UTF-8 encoding
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# Add to your PowerShell profile for persistence:
# notepad $PROFILE
# Add the line above
```

#### Linux/macOS (Bash/Zsh)

```bash
# Ensure UTF-8 locale
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Add to ~/.bashrc or ~/.zshrc for persistence
```

## Configuration Files

### Project Configuration Files

| File                 | Purpose                           |
| -------------------- | --------------------------------- |
| `Cargo.toml`         | Project metadata and dependencies |
| `rustfmt.toml`       | Code formatting rules             |
| `clippy.toml`        | Linting configuration             |
| `.cargo/config.toml` | Cargo build configuration         |
| `.editorconfig`      | Editor-agnostic formatting rules  |
| `.gitignore`         | Git exclusions                    |

### VS Code Configuration Files

| File                      | Purpose                  |
| ------------------------- | ------------------------ |
| `.vscode/settings.json`   | Editor settings for Rust |
| `.vscode/extensions.json` | Recommended extensions   |
| `.vscode/tasks.json`      | Build and run tasks      |
| `.vscode/launch.json`     | Debug configurations     |

### GitHub Configuration Files

| File                                           | Purpose          |
| ---------------------------------------------- | ---------------- |
| `.github/workflows/ci.yml`                     | CI/CD pipeline   |
| `.github/ISSUE_TEMPLATE/`                      | Issue templates  |
| `.github/pull_request_template.md`             | PR template      |
| `.github/instructions/copilot-instructions.md` | Copilot guidance |

## Running the Project

### Development Workflow

1. **Start Development**

    ```bash
    # Run with auto-reload (if cargo-watch installed)
    cargo watch -x run
    ```

2. **Make Changes**

    - Edit files in `src/`
    - Save (auto-format applies)
    - Reload happens automatically with cargo-watch

3. **Check Code Quality**

    ```bash
    # Format code
    cargo fmt

    # Check for issues
    cargo clippy

    # Run all checks
    cargo fmt && cargo clippy -- -D warnings
    ```

4. **Test Changes**

    ```bash
    # Run tests (when available)
    cargo test

    # Run with verbose output
    cargo test -- --nocapture
    ```

### Common Commands

```bash
# Build
cargo build                     # Debug build
cargo build --release          # Release build

# Run
cargo run                       # Run debug build
cargo run --release            # Run release build

# Check (faster than build)
cargo check                     # Check compilation without building

# Clean
cargo clean                     # Remove target directory

# Update dependencies
cargo update                    # Update Cargo.lock

# View documentation
cargo doc --open               # Build and open docs
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Writing Tests

Place tests in:

-   `src/*.rs` - Inline unit tests with `#[cfg(test)]`
-   `tests/` - Integration tests (when created)

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_validation() {
        assert_eq!(validate_answer("test", "test"), true);
    }
}
```

## Troubleshooting

### Common Issues

#### 1. "error: could not compile"

**Solution**: Check for syntax errors, run `cargo check` for details

#### 2. Colors Not Displaying

**Windows**: Use Windows Terminal instead of old cmd.exe
**Linux/Mac**: Ensure `TERM` environment variable is set (e.g., `xterm-256color`)

#### 3. "error: linker failed"

**Windows**: Install Visual Studio Build Tools
**Linux**: Install `build-essential` package
**macOS**: Install Xcode Command Line Tools

#### 4. Slow Compilation

**Solution**:

-   Use `cargo check` instead of `cargo build` during development
-   Enable incremental compilation (default in debug mode)
-   Consider using `sccache`: `cargo install sccache`

#### 5. Save File Not Found

**Solution**: Run the game at least once to generate `game_save.json`

#### 6. Terminal State Corrupted

**Solution**:

-   Run `reset` command (Linux/Mac) or restart terminal (Windows)
-   Ensure proper terminal cleanup in code

### Getting Help

1. **Check Documentation**

    - Read `README.md` for project overview
    - Check `CONTRIBUTING.md` for contribution guidelines
    - Review `WALKTHROUGH.md` for gameplay help

2. **GitHub Resources**

    - Search existing issues: <https://github.com/and3rn3t/hack/issues>
    - Start a discussion: <https://github.com/and3rn3t/hack/discussions>

3. **Community**
    - Discord server (coming soon)
    - GitHub Discussions for questions

### Development Tips

1. **Use Rust Analyzer**

    - Hover over types for information
    - Use auto-complete (Ctrl+Space)
    - Quick fixes with Ctrl+. (or Cmd+.)

2. **Format Regularly**

    - Enable format on save
    - Or run `cargo fmt` before commits

3. **Run Clippy Often**

    - Catches common mistakes
    - Suggests improvements
    - Run with `cargo clippy`

4. **Read Compiler Messages**

    - Rust compiler gives helpful errors
    - Follow suggestions
    - Use `rustc --explain E0123` for error codes

5. **Use Debug Builds for Development**
    - Faster compilation
    - More debug info
    - Save release builds for testing performance

## Next Steps

After setup:

1. Read `CONTRIBUTING.md` to understand contribution guidelines
2. Check `docs/ROADMAP.md` for planned features
3. Try solving challenges in the game
4. Explore the codebase starting with `src/main.rs`
5. Pick an issue tagged `good-first-issue` to contribute

Happy hacking! 👻🔒
