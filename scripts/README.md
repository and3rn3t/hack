# Terminal Setup Scripts

This directory contains scripts to verify and configure your terminal for The Hack: Ghost Protocol.

## Scripts Overview

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
