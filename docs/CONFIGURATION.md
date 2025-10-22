# Configuration Guide for The Hack: Ghost Protocol

This document explains all configuration files and options available in the project.

## Table of Contents

-   [Project Configuration](#project-configuration)
-   [Development Tools Configuration](#development-tools-configuration)
-   [Editor Configuration](#editor-configuration)
-   [CI/CD Configuration](#cicd-configuration)
-   [Runtime Configuration](#runtime-configuration)

## Project Configuration

### Cargo.toml

Main project configuration file for Rust.

```toml
[package]
name = "hack_simulator"              # Binary name
version = "0.1.0"                    # Semantic versioning
edition = "2021"                     # Rust edition
authors = ["The Hack Team"]          # Author information
description = "..."                  # Package description

[dependencies]
# Core dependencies with version requirements
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
crossterm = "0.27"
rand = "0.8"
chrono = "0.4"

[profile.dev]
# Debug profile (default for `cargo build`)
opt-level = 0    # No optimization
debug = true     # Include debug info

[profile.release]
# Release profile (for `cargo build --release`)
opt-level = 3    # Maximum optimization
debug = false    # No debug info
lto = true       # Link-time optimization
```

**Customization:**

-   Add new dependencies in `[dependencies]`
-   Adjust optimization with `opt-level` (0-3)
-   Enable/disable features per dependency

### rustfmt.toml

Code formatting configuration.

**Key Settings:**

-   `max_width = 100` - Maximum line length
-   `edition = "2021"` - Rust edition
-   `reorder_imports = true` - Auto-sort imports
-   `format_strings = true` - Format string literals

**Usage:**

```bash
cargo fmt              # Format all files
cargo fmt -- --check   # Check formatting without changes
```

### clippy.toml

Linting configuration for Clippy.

**Enabled Lint Groups:**

-   `clippy::all` - All default lints
-   `clippy::pedantic` - Stricter style lints
-   `clippy::nursery` - Experimental lints

**Allowed Lints:**

-   `module_name_repetitions` - Allow module prefixes
-   `must_use_candidate` - Don't require #[must_use]
-   `missing_errors_doc` - Error docs optional
-   `missing_panics_doc` - Panic docs optional

**Usage:**

```bash
cargo clippy                      # Run clippy
cargo clippy -- -D warnings       # Treat warnings as errors
cargo clippy --fix               # Auto-fix issues
```

### .cargo/config.toml

Cargo behavior configuration.

**Features:**

-   Command aliases (`cargo b` for `cargo build`)
-   Target-specific settings
-   Build parallelism
-   Color output configuration

**Usage:**

```bash
cargo b     # Alias for 'cargo build'
cargo rr    # Alias for 'cargo run --release'
cargo cl    # Alias for 'cargo clippy -- -D warnings'
```

## Development Tools Configuration

### .editorconfig

Cross-editor configuration.

**Settings:**

-   UTF-8 encoding for all files
-   LF line endings (auto-converted on Windows)
-   Trailing whitespace trimming
-   4 spaces for Rust, 2 for TOML/YAML

**Supported Editors:**

-   VS Code (with EditorConfig extension)
-   IntelliJ/CLion
-   Vim/Neovim
-   Atom
-   Sublime Text

### .gitignore

Git exclusions.

**Excluded:**

```
/target              # Build artifacts
game_save.json       # User save files
*.swp, *.swo        # Editor temp files
.DS_Store           # macOS metadata
Thumbs.db           # Windows metadata
```

**Note:** Add sensitive files (API keys, credentials) here.

## Editor Configuration

### VS Code Settings (.vscode/settings.json)

**Rust Settings:**

```json
{
    "editor.formatOnSave": true,
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.inlayHints.enable": true
}
```

**Key Features:**

-   Auto-format with rustfmt on save
-   Clippy integration for real-time linting
-   Type hints inline
-   Proper tab/space handling

**Customization:**
Edit `.vscode/settings.json` to:

-   Change color theme: `workbench.colorTheme`
-   Adjust font size: `editor.fontSize`
-   Modify rulers: `editor.rulers`

### VS Code Tasks (.vscode/tasks.json)

**Available Tasks:**

-   `cargo build` - Compile debug version
-   `cargo run` - Run the game
-   `cargo test` - Execute tests
-   `cargo clippy` - Lint code
-   `cargo fmt` - Format code
-   `Format and Check` - Combined task

**Usage:**

-   Press `Ctrl+Shift+B` for default build task
-   Press `Ctrl+Shift+P` → "Tasks: Run Task"

**Adding Custom Tasks:**

```json
{
    "label": "My Task",
    "type": "shell",
    "command": "cargo",
    "args": ["custom-command"],
    "group": "build"
}
```

### VS Code Launch (.vscode/launch.json)

**Debug Configurations:**

1. Debug executable - Run with debugger
2. Debug unit tests - Debug tests
3. Debug release build - Debug optimized build

**Requirements:**

-   Install `vadimcn.vscode-lldb` extension
-   Rust debug info enabled (default in dev profile)

**Usage:**

-   Press `F5` to start debugging
-   Set breakpoints with `F9`
-   Step through code with `F10` (over) / `F11` (into)

### VS Code Extensions (.vscode/extensions.json)

**Essential Extensions:**

-   `rust-lang.rust-analyzer` - Rust support
-   `serayuzgur.crates` - Dependency management
-   `tamasfe.even-better-toml` - TOML support

**Install All:**
VS Code will prompt to install recommended extensions on first open.

## CI/CD Configuration

### GitHub Actions (.github/workflows/ci.yml)

**Pipeline Stages:**

1. **Check** - Verify compilation

    ```yaml
    cargo check --all-features
    ```

2. **Format** - Ensure code is formatted

    ```yaml
    cargo fmt --all -- --check
    ```

3. **Clippy** - Lint for issues

    ```yaml
    cargo clippy --all-targets -- -D warnings
    ```

4. **Test** - Run test suite (cross-platform)

    - Matrix: [Windows, Linux, macOS] × [stable, nightly]

5. **Build** - Create release binaries

    - Artifacts for all platforms
    - Optimized release builds

6. **Release** - Publish on GitHub releases
    - Triggered on release tags
    - Uploads platform-specific binaries

**Customization:**

**Add a Job:**

```yaml
new-job:
    name: Custom Job
    runs-on: ubuntu-latest
    steps:
        - uses: actions/checkout@v4
        - name: Run custom command
          run: cargo custom
```

**Modify Matrix:**

```yaml
strategy:
    matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
```

**Secrets Required:**

-   `GITHUB_TOKEN` - Auto-provided by GitHub
-   `CODECOV_TOKEN` - For coverage (optional)

## Runtime Configuration

### Game Save File (game_save.json)

**Location:** Project root directory

**Structure:**

```json
{
    "player_name": "Ghost",
    "level": 2,
    "xp": 350,
    "sanity": 75,
    "completed_challenges": ["base64_basics", "port_scan"],
    "hints_used": { "base64_basics": 1 },
    "attempts": { "base64_basics": 2 }
}
```

**Management:**

-   Auto-created on first run
-   Auto-saved after each challenge
-   Manual save with `save` command
-   Delete to reset progress

**Backup:**

```bash
# Windows
copy game_save.json game_save.backup.json

# Linux/macOS
cp game_save.json game_save.backup.json
```

### Environment Variables

**Optional Variables:**

| Variable         | Purpose                       | Default       |
| ---------------- | ----------------------------- | ------------- |
| `RUST_BACKTRACE` | Show full backtraces on panic | `0`           |
| `RUST_LOG`       | Logging level                 | Not set       |
| `TERM`           | Terminal type                 | Auto-detected |

**Usage:**

```bash
# Windows PowerShell
$env:RUST_BACKTRACE=1
cargo run

# Linux/macOS
RUST_BACKTRACE=1 cargo run
```

### Terminal Configuration

**Required Features:**

-   ANSI color code support
-   UTF-8 encoding
-   Minimum 80x24 character size

**Recommended Terminals:**

-   **Windows:** Windows Terminal, PowerShell 7+
-   **Linux:** GNOME Terminal, Konsole, Alacritty
-   **macOS:** iTerm2, Terminal.app

**Color Scheme:**
Game uses standard ANSI colors:

-   Red: Errors, danger
-   Green: Success, hints
-   Yellow: Warnings, important info
-   Blue: Information
-   Cyan: Commands, prompts

## Configuration Best Practices

### For Contributors

1. **Don't Commit:**

    - `game_save.json` - User-specific
    - `.cargo/` directories - Local cache
    - IDE-specific files not in `.vscode/`

2. **Do Commit:**

    - Changes to `.vscode/*.json` if improving DX
    - Updates to `Cargo.toml` dependencies
    - CI/CD improvements

3. **Before Committing:**

    ```bash
    cargo fmt
    cargo clippy -- -D warnings
    cargo test
    ```

### For Maintainers

1. **Version Updates:**

    - Bump version in `Cargo.toml`
    - Update `CHANGELOG.md`
    - Tag release: `git tag v0.2.0`

2. **Dependency Updates:**

    ```bash
    cargo update
    cargo test  # Verify still works
    ```

3. **Security Audits:**

    ```bash
    cargo install cargo-audit
    cargo audit
    ```

## Troubleshooting Configuration

### Cargo Not Found

**Problem:** `cargo: command not found`
**Solution:**

```bash
# Add to PATH
# Windows: During Rust installation
# Linux/macOS: source ~/.cargo/env
```

### Formatting Issues

**Problem:** Code not auto-formatting
**Solution:**

1. Check `.vscode/settings.json` has `formatOnSave: true`
2. Verify `rust-analyzer` extension installed
3. Run manually: `cargo fmt`

### Clippy Warnings

**Problem:** Too many warnings
**Solution:**

1. Fix warnings: `cargo clippy --fix`
2. Allow specific lints in `clippy.toml`
3. Use `#[allow(clippy::lint_name)]` in code

### CI Failures

**Problem:** Tests pass locally but fail in CI
**Solution:**

1. Check matrix (OS, Rust version)
2. Review CI logs for differences
3. Test with `cargo test --release`

## Further Reading

-   [Cargo Book](https://doc.rust-lang.org/cargo/)
-   [Rustfmt Documentation](https://rust-lang.github.io/rustfmt/)
-   [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
-   [GitHub Actions Docs](https://docs.github.com/actions)

---

For questions about configuration, open an issue or discussion on GitHub.
