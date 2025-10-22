# Development Workflow Guide

This guide explains how to use the scripts and VS Code tasks for efficient development of The Hack: Ghost Protocol.

## Quick Start

### Using VS Code Tasks (Recommended)

Press `Ctrl+Shift+P` (Windows/Linux) or `Cmd+Shift+P` (macOS), then type "Run Task" to see all available tasks.

**Common Tasks:**

-   `Dev: Quick Check` - Run fmt, clippy, and tests
-   `Test: Watch Mode` - Continuous testing
-   `cargo test` - Run all tests once
-   `Build: Release with Stats` - Build optimized binary

### Using Scripts Directly

All scripts are in the `scripts/` directory with both PowerShell (`.ps1`) and Bash (`.sh`) versions.

```bash
# Windows PowerShell
pwsh scripts/quick-check.ps1

# Linux/macOS Bash
./scripts/quick-check.sh
```

---

## Development Workflows

### 1. Before Starting Work

**Clean build to ensure fresh start:**

```bash
# VS Code Task
Ctrl+Shift+P → "Run Task" → "Dev: Clean All"

# Or via script
pwsh scripts/clean-all.ps1
```

### 2. During Development (TDD Approach)

**Terminal 1: Watch mode for continuous testing**

```bash
# VS Code Task
Ctrl+Shift+P → "Run Task" → "Test: Watch Mode"

# Or via script
pwsh scripts/test-watch.ps1
```

**Terminal 2: Your editor**

Edit code normally. Tests will automatically re-run when you save files.

**Filter tests while developing:**

```bash
# Watch only challenge tests
pwsh scripts/test-watch.ps1 "challenges"

# Watch only state tests
pwsh scripts/test-watch.ps1 "state"

# Watch specific test
pwsh scripts/test-watch.ps1 "test_welcome_challenge"
```

### 3. Before Committing

**Run quick validation:**

```bash
# VS Code Task
Ctrl+Shift+P → "Run Task" → "Dev: Quick Check"

# Or via script
pwsh scripts/quick-check.ps1
```

This runs in sequence:

1. ✅ Format code (`cargo fmt`)
2. ✅ Lint code (`cargo clippy`)
3. ✅ Run all tests

If all pass, you're ready to commit!

### 4. Deep Testing

**Run tests with full output:**

```bash
# VS Code Task
Ctrl+Shift+P → "Run Task" → "Test: Verbose Output"

# Or via script
pwsh scripts/test-verbose.ps1
```

**Run specific test categories:**

```bash
# Property-based tests only
Ctrl+Shift+P → "Run Task" → "Test: Property-Based Only"

# Integration tests only
Ctrl+Shift+P → "Run Task" → "Test: Integration Tests Only"

# Challenge validation tests
Ctrl+Shift+P → "Run Task" → "Test: Challenges Only"

# State management tests
Ctrl+Shift+P → "Run Task" → "Test: State Only"
```

### 5. Coverage Analysis

**Generate and view coverage report:**

```bash
# VS Code Task (Windows - requires WSL)
Ctrl+Shift+P → "Run Task" → "Test: Coverage Report"

# Or via script
# Linux/macOS
./scripts/test-coverage.sh

# Windows (via WSL)
wsl ./scripts/test-coverage.sh
```

Opens `coverage/index.html` showing:

-   Line coverage percentages
-   Uncovered code sections
-   Per-module breakdown

### 6. Building Release Version

**Build optimized binary:**

```bash
# VS Code Task
Ctrl+Shift+P → "Run Task" → "Build: Release with Stats"

# Or via script
pwsh scripts/build-release.ps1
```

Shows:

-   Build time
-   Binary size
-   Binary location

**Run the release build:**

```bash
# VS Code Task
Ctrl+Shift+P → "Run Task" → "Build: Run Release"

# Or directly
.\target\release\hack_simulator.exe  # Windows
./target/release/hack_simulator      # Linux/macOS
```

---

## Workflow Examples

### Example 1: Adding a New Challenge

```bash
# 1. Start watch mode
pwsh scripts/test-watch.ps1 "challenges"

# 2. Create the challenge in src/challenges.rs
# 3. Add test for the challenge
# 4. Watch tests auto-run as you save

# 5. Once tests pass, run quick check
pwsh scripts/quick-check.ps1

# 6. Commit!
```

### Example 2: Debugging a Failing Test

```bash
# 1. Run test with verbose output
pwsh scripts/test-verbose.ps1 "test_name"

# 2. See full output including print statements

# 3. If needed, run with single thread for clarity
cargo test test_name -- --nocapture --test-threads=1
```

### Example 3: Preparing for Release

```bash
# 1. Clean everything
pwsh scripts/clean-all.ps1

# 2. Run full check
Ctrl+Shift+P → "Run Task" → "Full Check (CI Locally)"

# 3. Generate coverage report
./scripts/test-coverage.sh

# 4. Build release
pwsh scripts/build-release.ps1

# 5. Test the release binary
.\target\release\hack_simulator.exe
```

### Example 4: Working on Multiple Features

**Terminal 1: Watch main tests**

```bash
pwsh scripts/test-watch.ps1
```

**Terminal 2: Run specific tests**

```bash
cargo test test_specific_feature -- --nocapture
```

**Terminal 3: Run the game**

```bash
cargo run
```

---

## VS Code Keyboard Shortcuts

Add these to your `keybindings.json` for faster workflows:

```json
[
    {
        "key": "ctrl+shift+t",
        "command": "workbench.action.tasks.runTask",
        "args": "cargo test"
    },
    {
        "key": "ctrl+shift+b",
        "command": "workbench.action.tasks.build"
    },
    {
        "key": "ctrl+shift+r",
        "command": "workbench.action.tasks.runTask",
        "args": "cargo run"
    },
    {
        "key": "ctrl+shift+c",
        "command": "workbench.action.tasks.runTask",
        "args": "Dev: Quick Check"
    }
]
```

---

## Troubleshooting

### Tests Failing Locally But Should Pass

```bash
# Clean and rebuild
pwsh scripts/clean-all.ps1
cargo build
cargo test
```

### cargo-watch Not Found

```bash
cargo install cargo-watch
```

### cargo-tarpaulin Not Found (Coverage)

```bash
# Linux/macOS
cargo install cargo-tarpaulin

# Windows: Use WSL
wsl sudo apt install libssl-dev pkg-config
wsl cargo install cargo-tarpaulin
```

### Test Isolation Issues

If tests fail when run together but pass individually:

```bash
# Run with single thread
cargo test -- --test-threads=1

# Or run specific test alone
cargo test test_name --exact
```

### Terminal Compatibility Issues

```bash
# Verify terminal setup
pwsh scripts/verify-terminal.ps1

# Windows: Use setup script
pwsh scripts/setup-terminal.ps1
```

---

## CI/CD Integration

These scripts are designed to work in CI pipelines:

```yaml
# GitHub Actions example
- name: Quick Check
  run: pwsh scripts/quick-check.ps1

- name: Full CI Check
  run: cargo test && cargo clippy && cargo audit
```

---

## Performance Tips

### Fast Iteration

```bash
# Use check instead of build during development
cargo check  # Much faster than cargo build

# Use watch mode for instant feedback
pwsh scripts/test-watch.ps1
```

### Faster Tests

```bash
# Run only changed tests (requires cargo-nextest)
cargo nextest run

# Skip slow tests during development
cargo test --lib  # Skip integration tests
```

### Faster Builds

```bash
# Use release mode with faster linking
cargo build --release

# Or adjust profile in Cargo.toml
[profile.dev]
opt-level = 1  # Some optimization, faster builds
```

---

## Best Practices

### ✅ DO

-   Run `quick-check.ps1` before committing
-   Use watch mode during active development
-   Run full test suite before pushing
-   Generate coverage reports for new features
-   Clean build artifacts occasionally

### ❌ DON'T

-   Commit without running tests
-   Skip clippy warnings
-   Ignore failing tests
-   Push broken builds
-   Leave debug print statements

---

## Script Maintenance

### Adding New Scripts

1. Create both `.ps1` (Windows) and `.sh` (Linux/macOS) versions
2. Add to `scripts/` directory
3. Make bash script executable: `chmod +x scripts/new-script.sh`
4. Add task to `.vscode/tasks.json`
5. Update this guide
6. Update `scripts/README.md`

### Script Template

```powershell
#!/usr/bin/env pwsh
# Brief description
# Usage: ./script-name.ps1 [args]

param(
    [string]$Arg1 = ""
)

Write-Host "🔍 Starting..." -ForegroundColor Cyan
Write-Host ""

# Your script logic here

Write-Host ""
Write-Host "✅ Complete!" -ForegroundColor Green
```

---

## Additional Resources

-   [Cargo Book](https://doc.rust-lang.org/cargo/)
-   [cargo-watch Documentation](https://github.com/watchexec/cargo-watch)
-   [VS Code Tasks Documentation](https://code.visualstudio.com/docs/editor/tasks)
-   [Testing Guide](../docs/TESTING.md)
-   [Testing Strategy](../docs/TESTING_STRATEGY.md)

---

**Questions?** Check `docs/` or open an issue on GitHub.
