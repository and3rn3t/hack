# Changelog

All notable changes to The Hack: Ghost Protocol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

-   **Automated Test Suite**: Comprehensive testing for all 26 challenges
    -   34 test cases covering challenge validation, metadata, and edge cases
    -   Answer validation tests for all correct/incorrect inputs
    -   Challenge metadata tests (IDs, titles, descriptions, hints)
    -   Balance tests ensuring XP and sanity scale with difficulty
    -   Level distribution tests verifying proper challenge organization
    -   Edge case tests for empty inputs, whitespace, and special characters
    -   All tests passing with `cargo test`
-   **14 New Challenges** for expanded gameplay (total: 26 challenges):
    -   `rot13_ghost` (Level 0): ROT13 cipher decoding challenge
    -   `binary_basics` (Level 0): Binary to ASCII conversion
    -   `url_decode` (Level 0): URL encoding and decoding
    -   `jwt_token` (Level 1): JWT authentication vulnerability (Algorithm Confusion)
    -   `path_traversal` (Level 1): Directory traversal attacks
    -   `md5_collision` (Level 1): MD5 hash cracking with rainbow tables
    -   `command_injection` (Level 1): Shell command injection
    -   `dns_tunneling` (Level 2): DNS tunneling detection and hex decoding
    -   `xss_attack` (Level 2): Cross-Site Scripting (XSS) basics
    -   `api_key_leak` (Level 2): Secret exposure in Git history
    -   `session_hijack` (Level 2): Session hijacking and cookie theft
    -   `cors_bypass` (Level 2): CORS misconfiguration vulnerabilities
    -   `format_string` (Level 3): Format string vulnerability exploitation
    -   `race_condition` (Level 3): TOCTOU race condition attacks
    -   `integer_overflow` (Level 3): Integer overflow exploits
-   **Major UI Improvements**:
    -   Enhanced challenge selection menu with better visual hierarchy
    -   Progress bars for sanity, XP, and challenge completion
    -   Color-coded status indicators (✓ completed, ○ available)
    -   Improved statistics screen with per-level progress breakdown
    -   Beautiful challenge headers and completion animations
    -   Enhanced hint display with bordered boxes
    -   Better level transition screens with atmospheric colors
    -   Menu option formatting with icons and descriptions
-   **New UI Functions** for future use:
    -   `print_info()` - Info messages with icon
    -   `print_progress_bar()` - Customizable progress bars
    -   `print_box()` - Bordered content boxes
    -   `print_menu_option()` - Formatted menu items
    -   `print_challenge_header()` - Challenge intro headers
    -   `typewriter_effect()` - Typewriter text animation
-   Complete terminal configuration system:
    -   `docs/TERMINAL_SETUP.md`: Comprehensive setup guide for all platforms
    -   `scripts/verify-terminal.sh`: Bash verification script (Linux/macOS)
    -   `scripts/verify-terminal.ps1`: PowerShell verification script (Windows)
    -   `scripts/setup-terminal.ps1`: Automated Windows terminal setup
    -   `scripts/README.md`: Documentation for terminal scripts
-   Comprehensive documentation system:
    -   `.github/instructions/copilot-instructions.md`: AI coding assistant guidance
    -   `docs/ROADMAP.md`: Multi-year feature roadmap (v1.1-5.0)
    -   `docs/SETUP.md`: Development environment setup
    -   `docs/CONFIGURATION.md`: Detailed configuration reference
    -   `docs/CONFIG_SUMMARY.md`: Quick configuration overview
    -   `docs/INDEX.md`: Documentation navigation index
    -   `docs/TERMINAL_OPTIMIZATION_SUMMARY.md`: Terminal implementation summary
-   GitHub workflow templates:
    -   `.github/workflows/ci.yml`: CI/CD pipeline with cross-platform testing
    -   `.github/ISSUE_TEMPLATE/bug_report.md`: Bug report template
    -   `.github/ISSUE_TEMPLATE/feature_request.md`: Feature request template
    -   `.github/ISSUE_TEMPLATE/challenge_proposal.md`: Challenge proposal template
    -   `.github/pull_request_template.md`: PR template
    -   `.github/CHECKLIST.md`: Setup verification checklist
-   Development tooling configuration:
    -   `rustfmt.toml`: Code formatting rules
    -   `clippy.toml`: Linting configuration
    -   `.cargo/config.toml`: Cargo build settings
    -   `.editorconfig`: Cross-editor formatting
    -   `.vscode/`: VS Code workspace settings, extensions, tasks, launch configs
-   `CONTRIBUTING.md`: Contribution guidelines and workflow
-   `CHANGELOG.md`: This file

### Changed

-   `README.md`: Added terminal requirements and setup links
-   `docs/SETUP.md`: Added terminal setup section with verification steps
-   `docs/INDEX.md`: Added terminal setup to player guides
-   `docs/CONFIG_SUMMARY.md`: Added terminal verification to setup process

## [1.0.0] - 2025-10-21

### Initial Release

-   Horror-themed hacking simulator and CTF game
-   11 original challenges across 5 difficulty levels
-   Sanity mechanic adding urgency to gameplay
-   Terminal-based UI with ANSI colors and ASCII art
-   Save/load system for game progress
-   Progressive hint system
-   Educational cybersecurity content:
    -   Base64 encoding/decoding
    -   Caesar cipher cryptography
    -   SQL injection basics
    -   Hexadecimal decoding
    -   HTTP headers
    -   Mobile deep linking
    -   Buffer overflow concepts
    -   Reverse engineering
    -   XOR operations
-   Immersive horror narrative with glitch effects
-   Cross-platform support (Windows, Linux, macOS)

[Unreleased]: https://github.com/and3rn3t/hack/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/and3rn3t/hack/releases/tag/v1.0.0
