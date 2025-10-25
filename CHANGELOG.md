# Changelog

All notable changes to The Hack: Ghost Protocol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - New OSINT Challenges

-   **🔍 Five New OSINT (Open Source Intelligence) Challenges** (Level 2):

    -   **Username Enumeration** (`osint_username_recon`): Track digital identities across platforms using username correlation techniques
    -   **Wayback Machine** (`osint_wayback_machine`): Discover historical website data using Internet Archive to uncover deleted content
    -   **Metadata Extraction** (`osint_metadata_extraction`): Extract hidden information from document metadata (author, creation date, company info)
    -   **Shodan Reconnaissance** (`osint_shodan_recon`): Learn about the internet-connected device search engine for finding exposed systems
    -   **Pastebin Data Dumps** (`osint_pastebin_leak`): Identify credential leaks on public text-sharing sites

    All challenges include educational context, progressive hints, and teach real-world OSINT techniques used in cybersecurity investigations.

## [1.2.0] - 2025-10-25

### Added - Advanced User Experience & Challenge Variants

-   **🎮 Advanced Command System**:

    -   **User Aliases**: Create custom shortcuts for frequently used commands
    -   **Enhanced Tab Completion**: Expanded completion contexts for settings, aliases, save slots, and difficulty options
    -   **Command Resolution**: Automatic alias resolution with circular reference protection
    -   **Smart Auto-completion**: Context-aware suggestions based on current menu

-   **⚙️ Comprehensive Settings System**:

    -   **Difficulty Scaling**: Choose between Adaptive, Static, or Custom difficulty progression
    -   **Hint Verbosity**: Select Minimal, Moderate, or Detailed hint levels
    -   **Font Size Options**: Small, Medium, Large, and Extra Large for accessibility
    -   **Audio Preferences**: Toggle atmospheric audio effects on/off
    -   **Animation Speed**: Control animation speeds (None, Slow, Normal, Fast) for accessibility
    -   **Theme Preferences**: Persistent color theme selection with user preference storage

-   **💾 Enhanced Save System**:

    -   **Multiple Save Slots**: 5 save slots (0-4) for different playthroughs
    -   **Save Slot Management**: Visual interface showing player name, level, progress, and file size
    -   **Export/Import**: Export save data as JSON for backup and sharing
    -   **Save Metadata**: Detailed information about each save slot including last modified time
    -   **Auto-save Preferences**: Automatic saving after settings changes

-   **📊 Progress Analytics**:

    -   **Detailed Statistics**: Track total playtime, challenge attempts, hint usage, and completion times
    -   **Performance Metrics**: Completion rate calculations and average completion times
    -   **Learning Streaks**: Track consecutive successful challenges
    -   **Difficulty Preferences**: Per-challenge difficulty preference tracking
    -   **Adaptive Recommendations**: Smart difficulty suggestions based on performance

-   **🎯 Challenge Difficulty Variants**:

    -   **Four Difficulty Levels**: Beginner (Tutorial), Standard (Default), Advanced (Fewer hints), Expert (Minimal help)
    -   **Dynamic Scaling**: XP and sanity cost multipliers based on difficulty selection
    -   **Variant System**: Challenges can have multiple versions with different content and complexity
    -   **Adaptive Selection**: Automatic difficulty selection based on player performance and preferences
    -   **Challenge Replay**: Completed challenges can be attempted at different difficulty levels

-   **🔄 Dynamic Challenge Generation**:

    -   **Randomized Content**: Procedurally generated challenges for practice
    -   **Practice Mode**: Dedicated practice area with dynamic challenges that don't affect main progress
    -   **Random Challenge Generator**: Dynamic Base64, ROT cipher, and hex decoding challenges
    -   **Flexible Validation**: Smart answer checking for procedurally generated content
    -   **Practice Bonuses**: Bonus XP rewards for completing practice challenges

### Enhanced

-   **Menu System**: Expanded main menu with new options for settings, aliases, and practice mode
-   **User Interface**: Improved settings menus with comprehensive customization options
-   **Progress Tracking**: Enhanced statistics with detailed analytics and performance metrics
-   **Challenge Selection**: Interactive difficulty selection for challenges with variants
-   **Save System**: Robust multi-slot saving with metadata and import/export capabilities

### Technical

-   **State Management**: Extended GameState with user preferences and progress analytics
-   **Compatibility**: Backward-compatible save file system with automatic field initialization
-   **Performance**: Efficient caching and analytics tracking without performance impact
-   **Architecture**: Modular design for easy extension of preferences and challenge variants

## [1.0.0] - 2025-10-24

### Added - Web Version & Major Enhancements

-   **🌐 Web Application**: Complete browser-based version with WebAssembly

    -   Full HTML5 frontend with xterm.js terminal emulator
    -   WebAssembly compilation with wasm-bindgen
    -   Production deployment on Cloudflare Workers at <https://hack.andernet.dev>
    -   Cross-platform save system compatible with browser storage
    -   Responsive design with horror theme styling

-   **🧪 Comprehensive Testing Infrastructure**:

    -   Web testing suite with Jest and Playwright
    -   End-to-end browser compatibility testing
    -   WebAssembly integration tests
    -   Performance and accessibility testing

-   **🎨 Achievement & Social System**:

    -   Digital achievement tracking and display
    -   Progress sharing capabilities
    -   Social media integration
    -   Clipboard API integration for sharing

-   **🔧 Development & Deployment**:
    -   Cloudflare Workers deployment with global CDN
    -   Automated CI/CD pipeline with GitHub Actions
    -   Cross-platform build system (native + web)
    -   Production-ready infrastructure

### Added

-   **Better Feedback System**:

    -   Progressive feedback based on attempt number
    -   Intelligent answer analysis (empty, too short, too long)
    -   Contextual guidance that improves with each attempt
    -   Category-specific learning tips after failed challenges
    -   Helpful resource suggestions for different challenge types
    -   Enhanced `provide_feedback()` method analyzing user input
    -   `show_learning_resources()` provides study guidance
    -   `get_category_tip()` offers challenge-specific advice

-   **Command History with Arrow Key Navigation**:

    -   Press ↑/↓ to navigate through previously entered commands
    -   Full line editing support with Left/Right arrows, Home/End keys
    -   Maintains up to 50 most recent commands
    -   Avoids duplicate consecutive entries
    -   Cursor positioning and in-place editing
    -   Backspace/Delete support at any cursor position
    -   Enhanced `read_input()` function using crossterm's event system
    -   Helper functions: `clear_command_history()`, `get_history_size()`
    -   Unit tests for history management

-   **Comprehensive Testing Framework** (88+ automated tests):
    -   **Challenge Tests** (34 tests):
        -   Answer validation tests for all 26 challenges (correct/incorrect inputs)
        -   Challenge metadata tests (IDs, titles, descriptions, hints)
        -   Balance tests ensuring XP and sanity scale with difficulty
        -   Level distribution tests verifying proper challenge organization
        -   Edge case tests for empty inputs, whitespace, and special characters
    -   **GameState Tests** (30 tests):
        -   State creation, modification, and persistence
        -   Challenge completion and level progression
        -   Sanity bounds and experience tracking
        -   Secret discovery mechanics
        -   Save/load functionality
        -   Serialization edge cases (unicode, long strings, extreme values)
    -   **Property-Based Tests** (17 tests using proptest):
        -   Challenge validators never panic on any input
        -   Sanity always bounded [0-100] regardless of operations
        -   Experience never decreases
        -   State invariants hold across random inputs
        -   Serialization is lossless
    -   **Integration Tests** (14 tests):
        -   Save/load round-trip preservation
        -   Multi-cycle save/load operations
        -   Backward compatibility
        -   File format validation
    -   **Test Infrastructure**:
        -   `tests/common/mod.rs`: Reusable test fixtures and helpers
        -   `TempSaveFile`: RAII temporary file manager
        -   State factory functions for various game scenarios
        -   Test data constants for common inputs
    -   **Testing Documentation**:
        -   `docs/TESTING_STRATEGY.md`: 680+ line comprehensive testing guide
        -   `docs/TESTING_IMPLEMENTATION_SUMMARY.md`: Implementation overview
        -   Updated `docs/TESTING.md` with new test information
    -   **Development Dependencies**:
        -   `proptest 1.4`: Property-based testing framework
        -   `criterion 0.5`: Benchmarking framework (ready for future use)
    -   **Library Interface**: `src/lib.rs` for integration testing
    -   All tests passing with `cargo test` in <0.5 seconds
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

[Unreleased]: https://github.com/and3rn3t/hack/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/and3rn3t/hack/compare/v1.0.0...v1.2.0
[1.0.0]: https://github.com/and3rn3t/hack/releases/tag/v1.0.0
