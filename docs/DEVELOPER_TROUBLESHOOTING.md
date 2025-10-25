# Developer Troubleshooting Guide

**Target Audience**: Developers working on and extending The Hack: Ghost Protocol
**Last Updated**: October 24, 2025

---

## Table of Contents

1. [Quick Diagnostics](#quick-diagnostics)
2. [Build and Compilation Issues](#build-and-compilation-issues)
3. [Runtime Errors](#runtime-errors)
4. [Terminal and UI Problems](#terminal-and-ui-problems)
5. [Challenge Development Issues](#challenge-development-issues)
6. [State Management Problems](#state-management-problems)
7. [Testing Framework Issues](#testing-framework-issues)
8. [Performance Problems](#performance-problems)
9. [Cross-Platform Issues](#cross-platform-issues)
10. [CI/CD Pipeline Troubleshooting](#cicd-pipeline-troubleshooting)
11. [Advanced Debugging](#advanced-debugging)
12. [Common Error Messages](#common-error-messages)
13. [Environment Setup Issues](#environment-setup-issues)
14. [Integration Problems](#integration-problems)

---

## Quick Diagnostics

### First Steps Checklist

When encountering any issue, start here:

```bash
# 1. Verify environment
rustc --version
cargo --version

# 2. Clean build
cargo clean
cargo build

# 3. Run basic tests
cargo test

# 4. Check for configuration issues
cargo check
```

### Environment Verification Script

```powershell
# PowerShell script to check development environment
Write-Host "=== The Hack: Ghost Protocol - Environment Check ===" -ForegroundColor Cyan

# Check Rust version
$rustVersion = rustc --version 2>$null
if ($rustVersion) {
    Write-Host "✅ Rust: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Rust not found or not in PATH" -ForegroundColor Red
    exit 1
}

# Check Cargo version
$cargoVersion = cargo --version 2>$null
if ($cargoVersion) {
    Write-Host "✅ Cargo: $cargoVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Cargo not found" -ForegroundColor Red
    exit 1
}

# Check project structure
if (Test-Path "Cargo.toml") {
    Write-Host "✅ Found Cargo.toml" -ForegroundColor Green
} else {
    Write-Host "❌ Not in project root - Cargo.toml not found" -ForegroundColor Red
    exit 1
}

# Check source files
$requiredFiles = @("src/main.rs", "src/lib.rs", "src/game.rs", "src/challenges.rs")
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "✅ Found $file" -ForegroundColor Green
    } else {
        Write-Host "❌ Missing $file" -ForegroundColor Red
    }
}

# Try compilation
Write-Host "`n=== Compilation Test ===" -ForegroundColor Cyan
$compileResult = cargo check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Compilation successful" -ForegroundColor Green
} else {
    Write-Host "❌ Compilation failed:" -ForegroundColor Red
    Write-Host $compileResult -ForegroundColor Red
}

Write-Host "`n=== Environment Check Complete ===" -ForegroundColor Cyan
```

### Quick Fix Commands

```bash
# Common quick fixes
cargo clean && cargo build          # Clean build
cargo update                       # Update dependencies
cargo fmt                         # Fix formatting
cargo clippy --fix                # Auto-fix lints
rm Cargo.lock && cargo build      # Reset lock file
```

---

## Build and Compilation Issues

### Dependency Resolution Problems

**Problem**: Cargo fails to resolve dependencies

```
error: failed to select a version for the requirement `crossterm = "^0.27"`
```

**Solutions**:

```bash
# 1. Clear cargo registry and try again
rm -rf ~/.cargo/registry
cargo build

# 2. Update Cargo.lock
rm Cargo.lock
cargo build

# 3. Check for conflicting versions
cargo tree --duplicates

# 4. Force update specific dependency
cargo update --package crossterm
```

### Compilation Errors in Rust Code

**Problem**: Static initialization errors

```rust
error[E0015]: calls in statics are limited to constant functions
```

**Solution**: Use `OnceLock` for lazy initialization

```rust
// ❌ Wrong - not const-compatible
static THEME: Mutex<Option<ColorTheme>> = Mutex::new(None);

// ✅ Correct - lazy initialization
static THEME: OnceLock<ColorTheme> = OnceLock::new();

pub fn get_theme() -> &'static ColorTheme {
    THEME.get_or_init(|| ColorTheme::horror())
}
```

**Problem**: Iterator type mismatches

```rust
error[E0277]: `&&Vec<Challenge>` is not an iterator
```

**Solution**: Remove extra reference

```rust
// ❌ Wrong - double reference
for challenge in &challenges { }

// ✅ Correct - single reference or owned
for challenge in challenges { }
// OR
for challenge in &*challenges { }
```

**Problem**: Missing structure fields

```rust
error[E0063]: missing field `background` in initializer of `ColorTheme`
```

**Solution**: Add missing fields to struct initialization

```rust
ColorTheme {
    name: "Theme".to_string(),
    primary: Color::White,
    // ... other fields
    background: Color::Black,  // Add missing field
}
```

### Linker Errors

**Problem**: Linker fails on Windows

```
error: linking with `link.exe` failed: exit code: 1120
```

**Solutions**:

```bash
# 1. Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

# 2. Alternative: Use GNU toolchain
rustup default stable-x86_64-pc-windows-gnu

# 3. Check Windows SDK installation
# Ensure Windows 10/11 SDK is installed
```

**Problem**: Missing system libraries on Linux

```
error: failed to run custom build command for `openssl-sys`
```

**Solutions**:

```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# CentOS/RHEL/Fedora
sudo yum install gcc openssl-devel pkgconfig
# OR
sudo dnf install gcc openssl-devel pkgconfig

# Arch Linux
sudo pacman -S base-devel openssl pkg-config
```

### Cross-Compilation Issues

**Problem**: Cross-compilation target not available

```
error: toolchain 'stable-x86_64-pc-windows-msvc' does not have target 'x86_64-unknown-linux-gnu'
```

**Solutions**:

```bash
# 1. Add target
rustup target add x86_64-unknown-linux-gnu

# 2. List available targets
rustup target list

# 3. Cross-compile
cargo build --target x86_64-unknown-linux-gnu
```

---

## Runtime Errors

### Terminal Initialization Failures

**Problem**: Terminal features not available

```
Error: "Raw mode not supported"
thread 'main' panicked at 'Failed to enable raw mode'
```

**Solutions**:

```rust
// Check terminal capabilities before using features
use crossterm::terminal;

fn safe_terminal_init() -> Result<(), Box<dyn std::error::Error>> {
    if terminal::is_raw_mode_supported()? {
        terminal::enable_raw_mode()?;
        Ok(())
    } else {
        eprintln!("Warning: Raw mode not supported, using basic input");
        // Fallback to basic input
        Ok(())
    }
}
```

**Problem**: Terminal size detection fails

```
thread 'main' panicked at 'Failed to get terminal size'
```

**Solution**: Add fallback for terminal size

```rust
use crossterm::terminal;

fn get_safe_terminal_size() -> (u16, u16) {
    terminal::size().unwrap_or((80, 24))  // Fallback to 80x24
}
```

### Save/Load System Errors

**Problem**: Permission denied when saving

```
Error: Permission denied (os error 13)
```

**Solutions**:

1. **Check file permissions**:

    ```bash
    # Linux/macOS
    ls -la save_game.json
    chmod 644 save_game.json

    # Windows (PowerShell)
    Get-Acl save_game.json
    ```

2. **Use user-specific save location**:

    ```rust
    use std::env;
    use std::path::PathBuf;

    fn get_save_path() -> PathBuf {
        let mut path = env::var("APPDATA")
            .or_else(|_| env::var("HOME"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));

        path.push("hack_ghost_protocol");
        std::fs::create_dir_all(&path).ok();
        path.push("save_game.json");
        path
    }
    ```

**Problem**: Corrupted save file

```
Error: EOF while parsing a value at line 1 column 0
```

**Solutions**:

1. **Backup and recovery system**:

    ```rust
    pub fn save_with_backup(state: &GameState) -> Result<(), StateError> {
        let save_path = get_save_path();
        let backup_path = save_path.with_extension("json.bak");

        // Create backup if save exists
        if save_path.exists() {
            std::fs::copy(&save_path, &backup_path)?;
        }

        // Save new state
        match save_state_to_file(state, &save_path) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Restore backup on failure
                if backup_path.exists() {
                    std::fs::copy(&backup_path, &save_path)?;
                }
                Err(e)
            }
        }
    }
    ```

2. **Validate save file format**:

    ```rust
    pub fn validate_save_file<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        // Try to parse as JSON first
        let _: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        // Try to parse as GameState
        let _: GameState = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid GameState format: {}", e))?;

        Ok(())
    }
    ```

### Memory and Performance Issues

**Problem**: Stack overflow in recursive functions

```
thread 'main' has overflowed its stack
```

**Solutions**:

1. **Increase stack size**:

    ```rust
    use std::thread;

    fn main() {
        let builder = thread::Builder::new()
            .name("main".into())
            .stack_size(8 * 1024 * 1024); // 8MB stack

        let handler = builder.spawn(|| {
            // Your main logic here
            actual_main();
        }).unwrap();

        handler.join().unwrap();
    }
    ```

2. **Convert to iterative approach**:

    ```rust
    // ❌ Recursive (can stack overflow)
    fn process_challenges_recursive(challenges: &[Challenge], index: usize) {
        if index >= challenges.len() { return; }
        process_challenge(&challenges[index]);
        process_challenges_recursive(challenges, index + 1);
    }

    // ✅ Iterative (safe)
    fn process_challenges_iterative(challenges: &[Challenge]) {
        for challenge in challenges {
            process_challenge(challenge);
        }
    }
    ```

**Problem**: Excessive memory usage

```
Error: memory allocation of 2147483648 bytes failed
```

**Solutions**:

1. **Use streaming for large data**:

    ```rust
    // ❌ Load everything at once
    let all_data: Vec<String> = huge_data_source.collect();

    // ✅ Process incrementally
    for item in huge_data_source.take(1000) {
        process_item(item);
    }
    ```

2. **Monitor memory usage**:
    ```rust
    fn monitor_memory_usage(operation: &str) {
        #[cfg(debug_assertions)]
        {
            let usage = get_memory_usage(); // Platform-specific implementation
            if usage > 100_000_000 { // 100MB threshold
                eprintln!("High memory usage in {}: {} bytes", operation, usage);
            }
        }
    }
    ```

---

## Terminal and UI Problems

### Color Display Issues

**Problem**: Colors not displaying correctly

**Diagnosis**:

```rust
fn diagnose_color_support() {
    use crossterm::style::{Color, SetForegroundColor};
    use crossterm::{execute, style::Print};
    use std::io::stdout;

    println!("Terminal Color Support Test:");

    let colors = [
        (Color::Red, "Red"),
        (Color::Green, "Green"),
        (Color::Blue, "Blue"),
        (Color::Yellow, "Yellow"),
        (Color::Magenta, "Magenta"),
        (Color::Cyan, "Cyan"),
    ];

    for (color, name) in colors {
        if execute!(stdout(), SetForegroundColor(color), Print(name), Print(" ")).is_err() {
            println!("Failed to display {}", name);
        }
    }

    // Reset color
    execute!(stdout(), crossterm::style::ResetColor).ok();
}
```

**Solutions**:

1. **Force color support**:

    ```bash
    # Force color in terminal
    export FORCE_COLOR=1
    export CLICOLOR_FORCE=1
    ```

2. **Graceful fallback**:
    ```rust
    fn print_with_fallback(text: &str, color: Color) -> io::Result<()> {
        match execute!(stdout(), SetForegroundColor(color), Print(text)) {
            Ok(_) => {
                execute!(stdout(), crossterm::style::ResetColor)?;
                Ok(())
            }
            Err(_) => {
                // Fallback to plain text
                print!("{}", text);
                Ok(())
            }
        }
    }
    ```

### Terminal Size Issues

**Problem**: UI layout breaks on small terminals

**Solutions**:

1. **Dynamic layout adjustment**:

    ```rust
    fn render_adaptive_ui() -> io::Result<()> {
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));

        if width < 80 {
            render_compact_ui(width, height)?;
        } else if width < 120 {
            render_normal_ui(width, height)?;
        } else {
            render_wide_ui(width, height)?;
        }

        Ok(())
    }
    ```

2. **Minimum size requirements**:

    ```rust
    fn check_terminal_requirements() -> Result<(), String> {
        let (width, height) = crossterm::terminal::size()
            .map_err(|_| "Cannot determine terminal size")?;

        if width < 60 {
            return Err(format!("Terminal too narrow: {} columns (minimum 60)", width));
        }

        if height < 20 {
            return Err(format!("Terminal too short: {} rows (minimum 20)", height));
        }

        Ok(())
    }
    ```

### Input Handling Problems

**Problem**: Arrow keys not working in input

**Solution**: Proper event handling setup

```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

fn handle_input_with_history() -> io::Result<String> {
    let mut input = String::new();
    let mut cursor_pos = 0;
    let mut history_index = 0;
    let history = get_command_history();

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event {
                KeyEvent { code: KeyCode::Enter, .. } => {
                    break;
                }
                KeyEvent { code: KeyCode::Up, .. } => {
                    if history_index > 0 {
                        history_index -= 1;
                        input = history[history_index].clone();
                        cursor_pos = input.len();
                        redraw_input_line(&input, cursor_pos)?;
                    }
                }
                KeyEvent { code: KeyCode::Down, .. } => {
                    if history_index < history.len() - 1 {
                        history_index += 1;
                        input = history[history_index].clone();
                        cursor_pos = input.len();
                        redraw_input_line(&input, cursor_pos)?;
                    }
                }
                KeyEvent { code: KeyCode::Left, .. } => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        move_cursor_to_position(cursor_pos)?;
                    }
                }
                KeyEvent { code: KeyCode::Right, .. } => {
                    if cursor_pos < input.len() {
                        cursor_pos += 1;
                        move_cursor_to_position(cursor_pos)?;
                    }
                }
                KeyEvent { code: KeyCode::Backspace, .. } => {
                    if cursor_pos > 0 {
                        input.remove(cursor_pos - 1);
                        cursor_pos -= 1;
                        redraw_input_line(&input, cursor_pos)?;
                    }
                }
                KeyEvent { code: KeyCode::Char(c), .. } => {
                    input.insert(cursor_pos, c);
                    cursor_pos += 1;
                    redraw_input_line(&input, cursor_pos)?;
                }
                _ => {}
            }
        }
    }

    Ok(input)
}
```

---

## Challenge Development Issues

### Validation Function Problems

**Problem**: Validator panics on certain inputs

```rust
// ❌ Dangerous - can panic
|answer| {
    let number: u32 = answer.parse().unwrap();  // Panics on non-numbers
    number == 42
}
```

**Solution**: Robust error handling

```rust
// ✅ Safe - handles all inputs
|answer| {
    answer.trim()
        .parse::<u32>()
        .map(|n| n == 42)
        .unwrap_or(false)
}
```

**Problem**: Case sensitivity issues

```rust
// ❌ Too strict
|answer| answer == "Hello World"

// ✅ Flexible
|answer| answer.to_lowercase().trim() == "hello world"
```

**Problem**: Whitespace handling inconsistency

```rust
// ❌ Inconsistent whitespace handling
|answer| {
    answer == "hello world" ||  // Sensitive to spaces
    answer == "helloworld"      // No spaces allowed
}

// ✅ Consistent normalization
|answer| {
    let normalized = answer.to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    normalized == "helloworld"
}
```

### Challenge Balance Issues

**Problem**: Inappropriate difficulty rewards

**Solution**: Use difficulty calculator

```rust
fn calculate_balanced_rewards(level: u32, complexity: ChallengeComplexity) -> (u32, u32) {
    let base_xp = match level {
        0 => 50,   // Beginner
        1 => 100,  // Intermediate
        2 => 150,  // Advanced
        3 => 200,  // Expert
        _ => 250,  // Master
    };

    let complexity_multiplier = match complexity {
        ChallengeComplexity::Simple => 1.0,
        ChallengeComplexity::Moderate => 1.5,
        ChallengeComplexity::Complex => 2.0,
        ChallengeComplexity::Expert => 2.5,
    };

    let xp_reward = (base_xp as f32 * complexity_multiplier) as u32;
    let sanity_cost = (level + 1) * 3 + match complexity {
        ChallengeComplexity::Simple => 2,
        ChallengeComplexity::Moderate => 5,
        ChallengeComplexity::Complex => 8,
        ChallengeComplexity::Expert => 12,
    };

    (xp_reward, sanity_cost)
}
```

### Testing Challenge Validation

**Problem**: Incomplete test coverage for validators

**Solution**: Comprehensive test template

```rust
#[cfg(test)]
mod challenge_validation_tests {
    use super::*;

    fn test_challenge_validator(
        challenge_fn: fn() -> Challenge,
        correct_answers: &[&str],
        incorrect_answers: &[&str]
    ) {
        let challenge = challenge_fn();

        // Test correct answers
        for &answer in correct_answers {
            assert!(
                (challenge.check_answer)(answer),
                "Should accept correct answer: '{}'", answer
            );
        }

        // Test incorrect answers
        for &answer in incorrect_answers {
            assert!(
                !(challenge.check_answer)(answer),
                "Should reject incorrect answer: '{}'", answer
            );
        }

        // Test edge cases
        let edge_cases = ["", " ", "\n", "\t", "🎃", "very_long_string".repeat(100)];
        for edge_case in edge_cases {
            // Should not panic
            let _ = (challenge.check_answer)(&edge_case);
        }
    }

    #[test]
    fn test_base64_challenge() {
        test_challenge_validator(
            base64_challenge,
            &["hello world", "Hello World", " hello world ", "HELLO WORLD"],
            &["wrong", "hello", "world", "123456", ""]
        );
    }
}
```

---

## State Management Problems

### Serialization Issues

**Problem**: Version compatibility between saves

**Solution**: Implement migration system

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(default = "default_version")]
    pub version: String,
    // ... other fields
}

fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn load_state_with_migration() -> Result<GameState, StateError> {
    let raw_json = std::fs::read_to_string("save_game.json")?;

    // Try current format first
    match serde_json::from_str::<GameState>(&raw_json) {
        Ok(state) => Ok(state),
        Err(_) => {
            // Try legacy formats
            if let Ok(legacy_state) = migrate_from_v1(&raw_json) {
                // Save in new format
                save_state(&legacy_state)?;
                Ok(legacy_state)
            } else {
                Err(StateError::UnsupportedVersion)
            }
        }
    }
}

fn migrate_from_v1(json: &str) -> Result<GameState, serde_json::Error> {
    #[derive(Deserialize)]
    struct GameStateV1 {
        player_name: String,
        level: u32,
        xp: u32,
        // ... other v1 fields
    }

    let v1_state: GameStateV1 = serde_json::from_str(json)?;

    // Convert to current format
    Ok(GameState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        player_name: v1_state.player_name,
        current_level: v1_state.level,
        experience: v1_state.xp,
        // ... set defaults for new fields
        completed_challenges: HashSet::new(),
        sanity: 100,
        // ...
    })
}
```

**Problem**: Concurrent save file access

**Solution**: File locking mechanism

```rust
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn safe_save_state(state: &GameState) -> Result<(), StateError> {
    let save_path = get_save_path();
    let lock_path = save_path.with_extension("lock");

    // Create lock file
    if lock_path.exists() {
        return Err(StateError::SaveInProgress);
    }

    let _lock_file = File::create(&lock_path)?;

    // Perform save with error handling
    let result = perform_atomic_save(state, &save_path);

    // Clean up lock file
    std::fs::remove_file(&lock_path).ok();

    result
}

fn perform_atomic_save(state: &GameState, path: &Path) -> Result<(), StateError> {
    let temp_path = path.with_extension("tmp");

    // Write to temporary file first
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;

        let json = serde_json::to_string_pretty(state)?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?; // Ensure data is written to disk
    }

    // Atomic move to final location
    std::fs::rename(&temp_path, path)?;

    Ok(())
}
```

---

## Testing Framework Issues

### Test Isolation Problems

**Problem**: Tests interfere with each other

**Solution**: Proper test isolation

```rust
use std::sync::Once;
use tempfile::TempDir;

static INIT: Once = Once::new();

fn setup_test_environment() -> TempDir {
    INIT.call_once(|| {
        // One-time test setup
        env_logger::init();
    });

    // Create isolated temp directory for each test
    tempfile::tempdir().expect("Failed to create temp dir")
}

#[test]
fn test_save_load_isolation() {
    let temp_dir = setup_test_environment();
    let save_path = temp_dir.path().join("test_save.json");

    // Test runs in isolation
    let original_state = create_test_state();
    save_state_to_file(&original_state, &save_path).unwrap();
    let loaded_state = load_state_from_file(&save_path).unwrap();

    assert_eq!(original_state.player_name, loaded_state.player_name);
}
```

**Problem**: Flaky tests due to timing

**Solution**: Deterministic testing approach

```rust
use std::time::{SystemTime, UNIX_EPOCH};

// ❌ Flaky - depends on system time
fn create_state_with_current_time() -> GameState {
    let mut state = GameState::new("test".to_string());
    state.timestamps.game_started = SystemTime::now();
    state
}

// ✅ Deterministic - uses fixed time
fn create_state_with_fixed_time() -> GameState {
    let mut state = GameState::new("test".to_string());
    let fixed_time = UNIX_EPOCH + std::time::Duration::from_secs(1609459200); // 2021-01-01
    state.timestamps.game_started = fixed_time;
    state
}

#[test]
fn test_timestamp_serialization() {
    let state = create_state_with_fixed_time();
    let json = serde_json::to_string(&state).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(state.timestamps.game_started, loaded.timestamps.game_started);
}
```

### Property Test Issues

**Problem**: Property tests finding false positives

**Solution**: More specific property constraints

```rust
use proptest::prelude::*;

// ❌ Too broad - may generate invalid test cases
proptest! {
    #[test]
    fn test_answer_validation(answer in ".*") {
        let challenge = example_challenge();
        let _ = (challenge.check_answer)(&answer); // Only tests it doesn't panic
    }
}

// ✅ More targeted - tests specific properties
proptest! {
    #[test]
    fn test_correct_answer_variations(
        base_answer in "correct",
        case_variation in prop::bool::ANY,
        whitespace_prefix in "[ \t]*",
        whitespace_suffix in "[ \t]*"
    ) {
        let challenge = example_challenge();
        let test_answer = format!(
            "{}{}{}",
            whitespace_prefix,
            if case_variation { base_answer.to_uppercase() } else { base_answer.to_string() },
            whitespace_suffix
        );

        // Should always accept variations of correct answer
        assert!((challenge.check_answer)(&test_answer));
    }

    #[test]
    fn test_wrong_answers_rejected(
        wrong_answer in prop::string::string_regex("[a-z]{1,20}")
            .prop_filter("Filter out correct answers", |s| s != "correct")
    ) {
        let challenge = example_challenge();

        // Should reject clearly wrong answers
        assert!(!(challenge.check_answer)(&wrong_answer));
    }
}
```

---

## Performance Problems

### Slow Challenge Loading

**Problem**: Challenges take too long to initialize

**Diagnosis**:

```rust
use std::time::Instant;

fn diagnose_challenge_loading() {
    let start = Instant::now();
    let challenges = get_all_challenges();
    let load_time = start.elapsed();

    println!("Loaded {} challenges in {:?}", challenges.len(), load_time);

    if load_time > std::time::Duration::from_millis(100) {
        println!("Warning: Challenge loading is slow");

        // Profile individual challenges
        for (i, challenge) in challenges.iter().enumerate() {
            let challenge_start = Instant::now();
            let _ = (challenge.check_answer)("test");
            let challenge_time = challenge_start.elapsed();

            if challenge_time > std::time::Duration::from_millis(10) {
                println!("Slow challenge {}: {} ({:?})", i, challenge.id, challenge_time);
            }
        }
    }
}
```

**Solution**: Implement lazy loading and caching

```rust
use std::sync::OnceLock;

static CHALLENGE_CACHE: OnceLock<Vec<Challenge>> = OnceLock::new();

pub fn get_all_challenges() -> &'static Vec<Challenge> {
    CHALLENGE_CACHE.get_or_init(|| {
        // This expensive computation happens only once
        compute_all_challenges()
    })
}

fn compute_all_challenges() -> Vec<Challenge> {
    let mut all_challenges = Vec::new();

    // Load challenges level by level to spread cost
    for level in 0..=4 {
        all_challenges.extend(compute_challenges_for_level(level));
    }

    all_challenges
}
```

**Problem**: Slow terminal rendering

**Solution**: Optimize UI rendering

```rust
use crossterm::{queue, execute};
use std::io::{stdout, Write};

// ❌ Slow - many execute! calls
fn render_slow() -> io::Result<()> {
    execute!(stdout(), crossterm::cursor::MoveTo(0, 0))?;
    execute!(stdout(), crossterm::style::Print("Line 1"))?;
    execute!(stdout(), crossterm::cursor::MoveTo(0, 1))?;
    execute!(stdout(), crossterm::style::Print("Line 2"))?;
    // ... many more execute! calls
    Ok(())
}

// ✅ Fast - batch with queue!
fn render_fast() -> io::Result<()> {
    queue!(stdout(), crossterm::cursor::MoveTo(0, 0))?;
    queue!(stdout(), crossterm::style::Print("Line 1"))?;
    queue!(stdout(), crossterm::cursor::MoveTo(0, 1))?;
    queue!(stdout(), crossterm::style::Print("Line 2"))?;
    // ... queue more operations

    // Flush all at once
    stdout().flush()?;
    Ok(())
}
```

### Memory Leaks

**Problem**: Memory usage grows over time

**Diagnosis**:

```rust
fn monitor_memory_usage() {
    #[cfg(debug_assertions)]
    {
        let initial_memory = get_process_memory();

        // Run game for a while...
        run_game_session();

        let final_memory = get_process_memory();
        let growth = final_memory - initial_memory;

        if growth > 10_000_000 { // 10MB growth threshold
            eprintln!("Potential memory leak: grew by {} bytes", growth);
        }
    }
}

#[cfg(target_os = "linux")]
fn get_process_memory() -> usize {
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1].parse::<usize>().unwrap_or(0) * 1024; // Convert KB to bytes
            }
        }
    }
    0
}
```

**Solution**: Proper resource management

```rust
// Use RAII and proper cleanup
struct GameSession {
    state: GameState,
    _terminal_guard: TerminalGuard,
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Ensure terminal is restored on drop
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::Show,
            crossterm::style::ResetColor
        );
    }
}

impl GameSession {
    fn new(player_name: String) -> io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        let _terminal_guard = TerminalGuard;

        Ok(Self {
            state: GameState::new(player_name),
            _terminal_guard,
        })
    }
}
```

---

## Cross-Platform Issues

### Windows-Specific Problems

**Problem**: Path separator issues

```rust
// ❌ Hard-coded separators
let save_path = "saves/game.json";

// ✅ Cross-platform paths
let save_path = Path::new("saves").join("game.json");
```

**Problem**: Windows Terminal color support

**Solution**: Enable ANSI support

```rust
#[cfg(windows)]
fn enable_ansi_support() -> io::Result<()> {
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        let mut mode = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return Err(io::Error::last_os_error());
        }

        mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if SetConsoleMode(handle, mode) == 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(not(windows))]
fn enable_ansi_support() -> io::Result<()> {
    // Unix systems support ANSI by default
    Ok(())
}
```

### macOS-Specific Problems

**Problem**: Terminal capabilities detection

**Solution**: Feature detection

```rust
fn detect_terminal_features() -> TerminalFeatures {
    let term = std::env::var("TERM").unwrap_or_default();
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default();

    TerminalFeatures {
        colors_256: term.contains("256") || term_program == "iTerm.app",
        true_color: term_program == "iTerm.app" || term == "xterm-kitty",
        unicode: true, // Most modern terminals support Unicode
        mouse: !matches!(term.as_str(), "dumb" | "unknown"),
    }
}

struct TerminalFeatures {
    colors_256: bool,
    true_color: bool,
    unicode: bool,
    mouse: bool,
}
```

### Linux Distribution Differences

**Problem**: Different package managers and dependencies

**Solution**: Runtime dependency detection

```bash
# Create setup script that detects distro
#!/bin/bash

detect_package_manager() {
    if command -v apt-get &> /dev/null; then
        echo "apt"
    elif command -v yum &> /dev/null; then
        echo "yum"
    elif command -v dnf &> /dev/null; then
        echo "dnf"
    elif command -v pacman &> /dev/null; then
        echo "pacman"
    else
        echo "unknown"
    fi
}

install_dependencies() {
    local pm=$(detect_package_manager)

    case $pm in
        "apt")
            sudo apt-get update
            sudo apt-get install -y build-essential pkg-config libssl-dev
            ;;
        "yum"|"dnf")
            sudo $pm install -y gcc openssl-devel pkgconfig
            ;;
        "pacman")
            sudo pacman -S --noconfirm base-devel openssl pkg-config
            ;;
        *)
            echo "Unknown package manager. Please install build tools manually."
            ;;
    esac
}
```

---

## CI/CD Pipeline Troubleshooting

### GitHub Actions Issues

**Problem**: CI fails on specific platforms

**Solution**: Platform-specific troubleshooting

```yaml
# In .github/workflows/ci.yml
jobs:
    test:
        strategy:
            matrix:
                os: [ubuntu-latest, windows-latest, macos-latest]
        runs-on: ${{ matrix.os }}
        steps:
            - uses: actions/checkout@v4

            # Platform-specific setup
            - name: Install Linux dependencies
              if: runner.os == 'Linux'
              run: |
                  sudo apt-get update
                  sudo apt-get install -y build-essential pkg-config libssl-dev

            - name: Install macOS dependencies
              if: runner.os == 'macOS'
              run: |
                  brew install pkg-config openssl

            - name: Setup Rust toolchain
              uses: actions-rs/toolchain@v1
              with:
                  toolchain: stable
                  override: true

            # Debug information
            - name: Debug environment
              run: |
                  rustc --version
                  cargo --version
                  echo "OS: ${{ runner.os }}"
                  echo "Runner: ${{ runner.arch }}"

            - name: Run tests with verbose output
              run: cargo test --verbose
              env:
                  RUST_BACKTRACE: 1
```

**Problem**: Flaky CI tests

**Solution**: Retry and isolation strategies

```yaml
- name: Run tests with retry
  uses: nick-invision/retry@v2
  with:
      timeout_minutes: 10
      max_attempts: 3
      command: cargo test --verbose

- name: Run tests in single-threaded mode
  run: cargo test --verbose -- --test-threads=1
  if: failure()
```

### Dependency Update Issues

**Problem**: Dependabot creates breaking changes

**Solution**: Automated testing with fallback

```yaml
# In .github/dependabot.yml
version: 2
updates:
    - package-ecosystem: "cargo"
      directory: "/"
      schedule:
          interval: "weekly"
      # Group minor updates together
      groups:
          minor-updates:
              patterns:
                  - "*"
              update-types:
                  - "minor"
                  - "patch"
      # Separate major updates
      ignore:
          - dependency-name: "*"
            update-types: ["version-update:semver-major"]
```

### Coverage Reporting Issues

**Problem**: Codecov upload fails

**Diagnosis and solution**:

```yaml
- name: Generate coverage report
  run: |
      cargo install cargo-tarpaulin
      cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Xml

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v3
  with:
      files: ./cobertura.xml
      fail_ci_if_error: true
      verbose: true
      token: ${{ secrets.CODECOV_TOKEN }}

- name: Debug coverage files
  if: failure()
  run: |
      ls -la *.xml || echo "No XML files found"
      ls -la target/tarpaulin/ || echo "No tarpaulin directory"
```

---

## Advanced Debugging

### Debug Builds vs Release Builds

**Problem**: Bug appears only in release mode

**Solution**: Debugging release builds

```toml
# In Cargo.toml
[profile.release]
debug = true      # Include debug info in release
opt-level = 2     # Reduce optimization level
lto = false      # Disable link-time optimization for debugging

# Or create a custom profile
[profile.release-debug]
inherits = "release"
debug = true
opt-level = 1
```

**Problem**: Performance issue only in debug mode

**Solution**: Profile-specific optimizations

```toml
[profile.dev]
opt-level = 1    # Some optimization in debug mode

# Or use release mode for dependencies only
[profile.dev.package."*"]
opt-level = 3
```

### Memory Debugging

**Problem**: Investigating memory issues

**Tools and techniques**:

```bash
# 1. Valgrind (Linux/macOS)
cargo build
valgrind --tool=memcheck --leak-check=full ./target/debug/hack_simulator

# 2. AddressSanitizer
RUSTFLAGS="-Z sanitizer=address" cargo run --target x86_64-unknown-linux-gnu

# 3. Memory profiling with heaptrack (Linux)
heaptrack cargo run
heaptrack_gui heaptrack.hack_simulator.*.gz
```

**Rust-specific memory debugging**:

```rust
// Enable allocation tracking in debug builds
#[cfg(debug_assertions)]
use std::alloc::{GlobalAlloc, Layout, System};

#[cfg(debug_assertions)]
struct TrackingAllocator;

#[cfg(debug_assertions)]
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        eprintln!("Allocated {} bytes at {:p}", layout.size(), ptr);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        eprintln!("Deallocated {} bytes at {:p}", layout.size(), ptr);
        System.dealloc(ptr, layout);
    }
}

#[cfg(debug_assertions)]
#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;
```

### Concurrency Debugging

**Problem**: Investigating race conditions or deadlocks

**Solution**: Thread sanitizer and debugging

```bash
# Thread sanitizer
RUSTFLAGS="-Z sanitizer=thread" cargo run --target x86_64-unknown-linux-gnu

# Deadlock detection
cargo add parking_lot --features=deadlock_detection
```

```rust
// Deadlock detection setup
#[cfg(debug_assertions)]
fn setup_deadlock_detection() {
    use parking_lot::deadlock;
    use std::thread;
    use std::time::Duration;

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        }
    });
}
```

---

## Common Error Messages

### Rust Compiler Errors

| Error Code | Description                          | Common Solution                               |
| ---------- | ------------------------------------ | --------------------------------------------- |
| E0277      | Trait bound not satisfied            | Implement required trait or add constraint    |
| E0502      | Cannot borrow as mutable             | Restructure code to avoid conflicting borrows |
| E0515      | Cannot return reference to temporary | Change return type or extend lifetime         |
| E0308      | Mismatched types                     | Convert between types or fix type annotations |
| E0382      | Use of moved value                   | Clone value or use references                 |
| E0425      | Cannot find value in scope           | Import missing items or fix typos             |
| E0433      | Failed to resolve use                | Add dependency or fix import path             |

### Runtime Errors

| Error Pattern               | Likely Cause                        | Solution                                          |
| --------------------------- | ----------------------------------- | ------------------------------------------------- |
| "No such file or directory" | Missing save file or wrong path     | Check file permissions and paths                  |
| "Permission denied"         | Insufficient file permissions       | Change file permissions or use different location |
| "Address already in use"    | Port conflict (if using networking) | Use different port or kill existing process       |
| "Broken pipe"               | Terminal disconnection              | Add signal handling                               |
| "Invalid UTF-8"             | Character encoding issues           | Validate input encoding                           |

### Custom Game Errors

```rust
// Provide helpful context in error messages
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Challenge '{id}' not found. Available challenges: {available:?}")]
    ChallengeNotFound {
        id: String,
        available: Vec<String>,
    },

    #[error("Save file corrupted at line {line}: {details}")]
    CorruptedSaveFile {
        line: usize,
        details: String,
    },

    #[error("Terminal too small: {width}x{height} (minimum: 80x24)")]
    TerminalTooSmall {
        width: u16,
        height: u16,
    },
}
```

---

## Environment Setup Issues

### Rust Installation Problems

**Problem**: Rustup installation fails

**Solutions**:

```bash
# Windows: Run as administrator
# Download from: https://rustup.rs/

# Linux/macOS: Fix permissions
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

**Problem**: Wrong Rust version

```bash
# Update to latest stable
rustup update stable
rustup default stable

# Or use specific version
rustup install 1.75.0
rustup default 1.75.0
```

**Problem**: Missing components

```bash
# Add required components
rustup component add clippy rustfmt
rustup component add rust-src  # For IDE support
```

### VS Code Setup Issues

**Problem**: Rust-analyzer not working

**Solutions**:

1. **Install rust-analyzer extension**
2. **Configure workspace settings**:
    ```json
    {
        "rust-analyzer.cargo.buildScripts.enable": true,
        "rust-analyzer.checkOnSave.command": "clippy",
        "rust-analyzer.procMacro.enable": true
    }
    ```
3. **Restart language server**: Ctrl+Shift+P → "Rust Analyzer: Restart Server"

**Problem**: Tasks not working

**Solution**: Verify tasks.json configuration:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo run",
            "type": "shell",
            "command": "cargo",
            "args": ["run"],
            "group": {
                "kind": "build",
                "isDefault": true
            }
        }
    ]
}
```

### Git Configuration Issues

**Problem**: Line ending conflicts

**Solution**: Configure Git properly

```bash
# Windows
git config --global core.autocrlf true

# Linux/macOS
git config --global core.autocrlf input

# Project-specific (recommended)
echo "* text=auto" > .gitattributes
echo "*.rs text eol=lf" >> .gitattributes
```

---

## Integration Problems

### External Tool Integration

**Problem**: Command-line tools not found

**Solution**: Path verification and fallbacks

```rust
use std::process::Command;

fn check_external_tools() -> Vec<String> {
    let required_tools = ["git", "cargo", "rustc"];
    let mut missing = Vec::new();

    for tool in required_tools {
        match Command::new(tool).arg("--version").output() {
            Ok(output) if output.status.success() => {
                println!("✅ {} available", tool);
            }
            _ => {
                println!("❌ {} not found or not working", tool);
                missing.push(tool.to_string());
            }
        }
    }

    missing
}
```

**Problem**: JSON parsing from external commands

**Solution**: Robust command execution

```rust
use serde_json::Value;

fn run_cargo_metadata() -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Cargo metadata failed: {}", stderr).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let json: Value = serde_json::from_str(&stdout)?;

    Ok(json)
}
```

### Library Integration Issues

**Problem**: Version conflicts between dependencies

**Solution**: Dependency resolution strategies

```toml
# In Cargo.toml - pin specific versions
[dependencies]
crossterm = "=0.27.0"
serde = "=1.0.193"

# Or use compatible versions
crossterm = "~0.27.0"  # 0.27.x

# For development dependencies
[dev-dependencies]
proptest = "1.4"
criterion = "0.5"
```

**Problem**: Feature flag conflicts

**Solution**: Careful feature management

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
crossterm = { version = "0.27", default-features = false, features = ["event-stream"] }

# Resolve conflicts explicitly
tokio = { version = "1.0", features = ["rt-multi-thread"], default-features = false }
```

---

## Prevention Strategies

### Code Review Checklist

Before submitting code, verify:

-   [ ] **All tests pass**: `cargo test`
-   [ ] **No clippy warnings**: `cargo clippy`
-   [ ] **Code is formatted**: `cargo fmt`
-   [ ] **Documentation updated**: Relevant docs reflect changes
-   [ ] **Error handling**: All error paths are handled
-   [ ] **Cross-platform compatibility**: Tested on target platforms
-   [ ] **Performance**: No obvious performance regressions
-   [ ] **Security**: No security vulnerabilities introduced

### Automated Prevention

```bash
# Pre-commit hook (place in .git/hooks/pre-commit)
#!/bin/bash

echo "Running pre-commit checks..."

# Check formatting
if ! cargo fmt -- --check; then
    echo "❌ Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi

# Check for clippy warnings
if ! cargo clippy -- -D warnings; then
    echo "❌ Clippy warnings found. Please fix before committing."
    exit 1
fi

# Run tests
if ! cargo test; then
    echo "❌ Tests failed. Please fix before committing."
    exit 1
fi

echo "✅ All pre-commit checks passed!"
```

### Monitoring and Alerting

```rust
// Add runtime monitoring for production-like testing
#[cfg(debug_assertions)]
mod monitoring {
    use std::time::{Duration, Instant};

    pub struct PerformanceMonitor {
        start_time: Instant,
        operation: String,
    }

    impl PerformanceMonitor {
        pub fn new(operation: &str) -> Self {
            Self {
                start_time: Instant::now(),
                operation: operation.to_string(),
            }
        }
    }

    impl Drop for PerformanceMonitor {
        fn drop(&mut self) {
            let elapsed = self.start_time.elapsed();
            if elapsed > Duration::from_millis(100) {
                eprintln!("⚠️  Slow operation: {} took {:?}", self.operation, elapsed);
            }
        }
    }

    // Usage: let _monitor = PerformanceMonitor::new("challenge_validation");
}
```

---

## Getting Help

### Internal Resources

1. **Code Documentation**: Use `cargo doc --open` to view generated docs
2. **Test Examples**: Look at existing tests for usage patterns
3. **Error Messages**: Read error messages carefully - Rust provides excellent diagnostics
4. **Source Code**: The source is the ultimate documentation

### External Resources

1. **Rust Language**:

    - [The Rust Book](https://doc.rust-lang.org/book/)
    - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    - [Rust Error Index](https://doc.rust-lang.org/error-index.html)

2. **Libraries**:

    - [Crossterm Documentation](https://docs.rs/crossterm/)
    - [Serde Guide](https://serde.rs/)
    - [Tokio Documentation](https://tokio.rs/) (if using async)

3. **Community**:
    - [Rust Users Forum](https://users.rust-lang.org/)
    - [Rust Reddit](https://www.reddit.com/r/rust/)
    - [Project Discussions](https://github.com/and3rn3t/hack/discussions)

### Debugging Workflow

1. **Reproduce**: Create minimal reproduction case
2. **Isolate**: Use binary search to narrow down the problem
3. **Instrument**: Add logging/println! to understand execution
4. **Test**: Write tests that demonstrate the issue
5. **Fix**: Apply smallest possible fix
6. **Verify**: Ensure fix works and doesn't break other things
7. **Document**: Update docs to prevent future issues

---

**Remember**: Most issues have been encountered before. Check existing issues, documentation, and community resources before diving deep into debugging.

**Pro Tip**: When asking for help, provide:

-   Complete error messages
-   Minimal reproduction code
-   Environment details (OS, Rust version, etc.)
-   What you've already tried

_"In debugging, as in hacking, patience and methodical thinking are your greatest tools."_ 🔧👻
