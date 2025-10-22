# Project Scripts

This directory contains utility scripts for development, testing, and building The Hack: Ghost Protocol.

## Quick Reference

| Script               | Purpose                            | Platform |
| -------------------- | ---------------------------------- | -------- |
| `quick-check.*`      | Fast dev check (fmt, clippy, test) | All      |
| `test-watch.*`       | Continuous testing on file changes | All      |
| `test-verbose.*`     | Run tests with detailed output     | All      |
| `test-coverage.*`    | Generate coverage reports          | All      |
| `build-release.*`    | Build optimized release binary     | All      |
| `clean-all.*`        | Clean all build artifacts          | All      |
| `verify-terminal.*`  | Check terminal compatibility       | All      |
| `setup-terminal.ps1` | Configure Windows terminal         | Windows  |

## Development Scripts

### `quick-check` - Fast Development Check

Runs format, clippy, and tests in sequence. Perfect for pre-commit checks.

**Usage:**

```bash
# Windows
pwsh scripts/quick-check.ps1

# Linux/macOS
./scripts/quick-check.sh
```

**What it does:**

1. Formats code with `cargo fmt`
2. Runs `cargo clippy` with warnings as errors
3. Runs all tests
4. Shows total execution time

### `clean-all` - Deep Clean

Removes all build artifacts, coverage reports, and optionally save files.

**Usage:**

```bash
# Windows
pwsh scripts/clean-all.ps1

# Linux/macOS
./scripts/clean-all.sh
```

## Testing Scripts

### `test-watch` - Continuous Testing

Automatically re-runs tests when files change. Great for TDD workflow.

**Requirements:** `cargo install cargo-watch`

**Usage:**

```bash
# Watch all tests (Windows)
pwsh scripts/test-watch.ps1

# Watch specific test pattern
pwsh scripts/test-watch.ps1 "challenges"

# Linux/macOS
./scripts/test-watch.sh
./scripts/test-watch.sh "challenges"
```

### `test-verbose` - Detailed Test Output

Runs tests with `--nocapture` to see all output including print statements.

**Usage:**

```bash
# Run all tests verbosely (Windows)
pwsh scripts/test-verbose.ps1

# Run specific test
pwsh scripts/test-verbose.ps1 "test_welcome_challenge"

# Linux/macOS
./scripts/test-verbose.sh
./scripts/test-verbose.sh "test_welcome_challenge"
```

### `test-coverage` - Coverage Reports

Generates HTML coverage report using `cargo-tarpaulin`.

**Requirements:** `cargo install cargo-tarpaulin` (Linux/macOS/WSL only)

**Usage:**

```bash
# Windows (requires WSL)
wsl ./scripts/test-coverage.sh

# Linux/macOS
./scripts/test-coverage.sh
```

**Output:** `coverage/index.html`

## Build Scripts

### `build-release` - Optimized Build

Builds release binary with optimizations and shows detailed stats.

**Usage:**

```bash
# Windows
pwsh scripts/build-release.ps1

# Linux/macOS
./scripts/build-release.sh
```

**Output:** `target/release/hack_simulator[.exe]`

## Terminal Verification Scripts

### `verify-terminal.sh` (Linux/macOS)

Checks your terminal configuration and capabilities.

**Usage:**

```bash
bash scripts/verify-terminal.sh
```

**Checks:**

-   Terminal size (minimum 80x24, recommended 100x30)
-   Color support (256-color or better)
-   UTF-8 encoding
-   Unicode box-drawing character rendering
-   ANSI color code support
-   Required tools (cargo, rustc)

### `verify-terminal.ps1` (Windows)

PowerShell version of the terminal verification script.

**Usage:**

```powershell
pwsh scripts/verify-terminal.ps1
```

**Checks:**

-   PowerShell version
-   Terminal size
-   UTF-8 encoding configuration
-   Windows Terminal detection
-   Unicode rendering
-   ANSI color support
-   Rust toolchain

### `setup-terminal.ps1` (Windows)

Interactive setup script for Windows users.

**Usage:**

```powershell
pwsh scripts/setup-terminal.ps1
```

**Features:**

-   Install Windows Terminal, PowerShell 7, and fonts
-   Configure UTF-8 encoding
-   Set up PowerShell profile
-   Resize terminal window
-   Verify configuration

**Options:**

```powershell
# Install components only
pwsh scripts/setup-terminal.ps1 -Install

# Configure settings only
pwsh scripts/setup-terminal.ps1 -Configure

# Verify only
pwsh scripts/setup-terminal.ps1 -Verify
```

## Quick Start

### First Time Setup (Windows)

```powershell
# Run interactive setup
pwsh scripts/setup-terminal.ps1

# Follow prompts to install and configure everything
```

### First Time Setup (Linux/macOS)

```bash
# Verify your current terminal
bash scripts/verify-terminal.sh

# If issues found, see docs/TERMINAL_SETUP.md for configuration help
```

### Before Playing

Always verify your terminal is properly configured:

```bash
# Windows
pwsh scripts/verify-terminal.ps1

# Linux/macOS
bash scripts/verify-terminal.sh
```

## Common Issues

### Colors Not Showing

**Windows:** Use Windows Terminal instead of cmd.exe
**Linux/macOS:** Ensure `TERM=xterm-256color` and UTF-8 locale

### Box Characters Broken

Install a proper monospace font with Unicode support:

-   **Cascadia Code** (Windows Terminal default)
-   **Fira Code** (Cross-platform)
-   **JetBrains Mono** (Cross-platform)

### Terminal Too Small

Resize your terminal window to at least 80x24 characters, preferably 100x30.

## Detailed Configuration

For detailed terminal configuration instructions, see:

-   **[Terminal Setup Guide](../docs/TERMINAL_SETUP.md)** - Complete configuration guide
-   **[Setup Guide](../docs/SETUP.md)** - Development environment setup
-   **[Configuration Guide](../docs/CONFIGURATION.md)** - All configuration options

## Making Scripts Executable (Linux/macOS)

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Run without 'bash' prefix
./scripts/verify-terminal.sh
```

## Troubleshooting

### Script Won't Run (Windows)

```powershell
# Check execution policy
Get-ExecutionPolicy

# If restricted, run as administrator:
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Script Won't Run (Linux/macOS)

```bash
# Make executable
chmod +x scripts/verify-terminal.sh

# Run with bash explicitly
bash scripts/verify-terminal.sh
```

### UTF-8 Errors

**Windows:**

```powershell
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
```

**Linux/macOS:**

```bash
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
```

## Contributing

When adding new scripts:

1. Use appropriate shell (bash for Unix, PowerShell for Windows)
2. Include error handling
3. Provide helpful output with colors
4. Update this README
5. Test on target platforms

## Related Documentation

-   [Terminal Setup Guide](../docs/TERMINAL_SETUP.md)
-   [Development Setup](../docs/SETUP.md)
-   [Contributing Guide](../CONTRIBUTING.md)

---

**Need help?** Open an issue or see the [Terminal Setup Guide](../docs/TERMINAL_SETUP.md)
