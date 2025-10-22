# Testing Strategy for The Hack: Ghost Protocol

## Overview

This document outlines the comprehensive testing strategy for The Hack: Ghost Protocol, a horror-themed cybersecurity education game. Our testing approach ensures code quality, prevents regressions, and maintains the educational integrity of the challenges.

---

## Testing Philosophy

### Core Principles

1. **Educational Integrity First**: Every challenge must validate correct answers accurately and reject incorrect ones
2. **Comprehensive Coverage**: Test all critical paths, edge cases, and error conditions
3. **Fast Feedback**: Tests should run quickly to encourage frequent execution
4. **Clear Failures**: Test failures should clearly indicate what broke and why
5. **Maintainable Tests**: Tests should be easy to understand, modify, and extend

### Test-Driven Development

We encourage TDD for new features:

1. Write failing tests that describe desired behavior
2. Implement the minimum code to make tests pass
3. Refactor while keeping tests green
4. Document the feature

---

## Test Types & Coverage

### 1. Unit Tests ✅ (Current: 34 tests)

**Purpose**: Test individual functions and components in isolation

**Current Coverage**:

-   ✅ Challenge validation (21 tests)
-   ✅ Challenge metadata (8 tests)
-   ✅ Balance and scaling (2 tests)
-   ✅ Level organization (3 tests)

**To Add**:

-   [ ] GameState methods (save, load, modify)
-   [ ] UI helper functions (color formatting, menu rendering)
-   [ ] Narrative text processing
-   [ ] Input sanitization and validation

**Example**:

```rust
#[test]
fn test_welcome_challenge() {
    let challenges = get_all_challenges();
    let welcome = challenges.iter()
        .find(|c| c.id == "welcome")
        .expect("Welcome challenge not found");

    // Correct answers
    assert!((welcome.check_answer)("welcome to the ghost protocol"));

    // Incorrect answers
    assert!(!(welcome.check_answer)("wrong answer"));
}
```

### 2. Integration Tests 🚧 (Planned)

**Purpose**: Test how components work together

**Target Areas**:

-   Save/load system with real files
-   Game loop transitions between levels
-   Challenge completion flow
-   State persistence across sessions
-   Menu navigation and user flow

**Structure**:

```
tests/
  ├── integration/
  │   ├── save_load_tests.rs
  │   ├── game_flow_tests.rs
  │   ├── challenge_completion_tests.rs
  │   └── mod.rs
  └── common/
      └── test_helpers.rs
```

**Example**:

```rust
#[test]
fn test_save_and_load_preserves_state() {
    let mut state = GameState::new("TestPlayer".to_string());
    state.complete_challenge("welcome", 50);
    state.save().unwrap();

    let loaded = GameState::load().unwrap();
    assert_eq!(loaded.player_name, "TestPlayer");
    assert!(loaded.has_completed("welcome"));
}
```

### 3. Property-Based Tests 🎯 (Recommended)

**Purpose**: Test properties that should hold for any input

**Use Cases**:

-   Challenge validators with random input
-   State transitions are always valid
-   Sanity and XP never go negative (unless intended)
-   Save/load is idempotent

**Library**: `proptest` or `quickcheck`

**Example**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_challenge_validator_never_panics(input in ".*") {
        let challenges = get_all_challenges();
        for challenge in challenges {
            // Should not panic on any input
            let _ = (challenge.check_answer)(&input);
        }
    }

    #[test]
    fn test_sanity_always_valid(initial in 0..100, cost in 0..50) {
        let mut state = GameState::new("Test".to_string());
        state.sanity = initial;
        state.modify_sanity(-cost);
        assert!(state.sanity >= 0 && state.sanity <= 100);
    }
}
```

### 4. UI/Display Tests 🎨 (Planned)

**Purpose**: Verify terminal output and rendering

**Challenges**:

-   Terminal I/O is inherently difficult to test
-   Need to mock or capture output

**Approach**:

-   Extract display logic from I/O
-   Test formatting functions return correct strings
-   Use `assert_cmd` for full binary testing

**Example**:

```rust
#[test]
fn test_format_sanity_bar() {
    let bar = format_sanity_bar(75, 100);
    assert!(bar.contains("75"));
    assert!(bar.contains("100"));
    assert!(bar.len() > 0);
}

#[test]
fn test_color_code_generation() {
    let red_text = get_colored_string("Error", Color::Red);
    assert!(red_text.contains("\x1b[31m")); // ANSI red
}
```

### 5. End-to-End Tests 🔄 (Future)

**Purpose**: Test complete user workflows

**Approach**:

-   Use `assert_cmd` to spawn game binary
-   Simulate user input
-   Capture and verify output

**Example**:

```rust
#[test]
fn test_complete_first_challenge() {
    let mut cmd = Command::cargo_bin("hack_simulator").unwrap();
    cmd.write_stdin("NewPlayer\n")
        .write_stdin("1\n") // Select challenge
        .write_stdin("welcome to the ghost protocol\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Challenge completed!"));
}
```

### 6. Benchmark Tests ⚡ (Optional)

**Purpose**: Measure and track performance

**Metrics**:

-   Challenge validation time
-   Save/load speed
-   UI rendering performance
-   Memory usage

**Library**: `criterion`

**Example**:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_challenge_validation(c: &mut Criterion) {
    let challenges = get_all_challenges();
    c.bench_function("validate_all_challenges", |b| {
        b.iter(|| {
            for challenge in &challenges {
                black_box((challenge.check_answer)("test"));
            }
        });
    });
}

criterion_group!(benches, benchmark_challenge_validation);
criterion_main!(benches);
```

---

## Test Organization

### Directory Structure

```
hack/
├── src/
│   ├── challenges.rs        # Unit tests at bottom
│   ├── state.rs             # Unit tests at bottom
│   ├── ui.rs                # Unit tests at bottom
│   ├── narrative.rs         # Unit tests at bottom
│   └── game.rs              # Unit tests at bottom
├── tests/
│   ├── integration/         # Integration tests
│   │   ├── save_load.rs
│   │   ├── game_flow.rs
│   │   └── challenge_completion.rs
│   ├── common/              # Test utilities
│   │   ├── mod.rs
│   │   └── fixtures.rs
│   └── end_to_end.rs        # E2E tests
└── benches/
    └── performance.rs       # Benchmarks
```

### Naming Conventions

-   Test functions: `test_<what_is_being_tested>`
-   Integration tests: `integration_<feature>`
-   Benchmark functions: `benchmark_<operation>`

---

## Running Tests

### Quick Commands

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_welcome_challenge

# Run tests in a specific module
cargo test challenges::tests

# Run integration tests only
cargo test --test '*'

# Run with coverage (requires tarpaulin)
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

### Watch Mode (with cargo-watch)

```bash
# Install cargo-watch
cargo install cargo-watch

# Run tests on file changes
cargo watch -x test

# Run specific tests on changes
cargo watch -x 'test challenges::tests'
```

---

## Test Data & Fixtures

### Common Test Helpers

Create reusable test utilities in `tests/common/`:

```rust
// tests/common/fixtures.rs
pub fn create_test_state() -> GameState {
    let mut state = GameState::new("TestPlayer".to_string());
    state.sanity = 100;
    state.experience = 0;
    state
}

pub fn create_completed_state() -> GameState {
    let mut state = create_test_state();
    state.complete_challenge("welcome", 50);
    state.complete_challenge("file_discovery", 50);
    state
}

pub fn temp_save_file() -> TempFile {
    // Create temporary save file for testing
    TempFile::new("test_save.json").unwrap()
}
```

### Test Data Sets

```rust
// Common test inputs
const VALID_INPUTS: &[&str] = &[
    "welcome to the ghost protocol",
    "WELCOME TO THE GHOST PROTOCOL",
    "Welcome To The Ghost Protocol",
];

const INVALID_INPUTS: &[&str] = &[
    "",
    "   ",
    "wrong answer",
    "welcome",
    "protocol",
];
```

---

## Edge Cases to Test

### Input Validation

-   ✅ Empty strings
-   ✅ Whitespace only
-   ✅ Very long inputs (10000+ chars)
-   ✅ Special characters (@#$%^&\*)
-   ✅ Unicode characters (emoji, non-ASCII)
-   ✅ Case variations (upper, lower, mixed)
-   [ ] Null bytes
-   [ ] Control characters

### State Management

-   [ ] Corrupted save files
-   [ ] Missing save files
-   [ ] Save file with old schema
-   [ ] Concurrent save/load operations
-   [ ] Invalid state transitions
-   [ ] Negative sanity
-   [ ] Extreme XP values

### Game Logic

-   [ ] Completing same challenge twice
-   [ ] Skipping required challenges
-   [ ] Attempting locked challenges
-   [ ] Level transitions
-   [ ] Game completion conditions
-   [ ] Sanity reaching 0
-   [ ] Maximum hints used

---

## Coverage Goals

### Current Coverage

-   **Challenges**: ~100% (all validators tested)
-   **State**: ~20% (basic structure only)
-   **UI**: 0% (no tests yet)
-   **Game Loop**: 0% (no tests yet)
-   **Narrative**: 0% (no tests yet)

### Target Coverage

-   **Overall**: 80%+ line coverage
-   **Challenge Validators**: 100%
-   **State Management**: 90%+
-   **Game Logic**: 80%+
-   **UI Functions**: 60%+ (harder to test)
-   **Narrative**: 50%+ (mostly text)

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/index.html
```

---

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: Tests

on: [push, pull_request]

jobs:
    test:
        runs-on: ${{ matrix.os }}
        strategy:
            matrix:
                os: [ubuntu-latest, windows-latest, macos-latest]
                rust: [stable, beta, nightly]

        steps:
            - uses: actions/checkout@v2
            - uses: actions-rs/toolchain@v1
              with:
                  toolchain: ${{ matrix.rust }}

            - name: Run tests
              run: cargo test --verbose

            - name: Run clippy
              run: cargo clippy -- -D warnings

            - name: Check formatting
              run: cargo fmt -- --check
```

### Pre-commit Hooks

```bash
# Install git hooks
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
cargo test || exit 1
cargo clippy -- -D warnings || exit 1
cargo fmt -- --check || exit 1
EOF

chmod +x .git/hooks/pre-commit
```

---

## Test Quality Guidelines

### Good Test Practices

✅ **DO**:

-   Test one thing per test
-   Use descriptive test names
-   Include positive and negative cases
-   Test edge cases and boundaries
-   Use assertions with helpful messages
-   Keep tests fast and independent
-   Clean up test resources (files, state)

❌ **DON'T**:

-   Test implementation details
-   Have tests depend on each other
-   Use arbitrary sleeps/waits
-   Hard-code paths or system-specific values
-   Ignore test failures
-   Write tests that are flaky
-   Test external services directly

### Test Structure (AAA Pattern)

```rust
#[test]
fn test_feature() {
    // Arrange: Set up test data
    let mut state = GameState::new("Test".to_string());

    // Act: Perform the operation
    state.complete_challenge("welcome", 50);

    // Assert: Verify the results
    assert!(state.has_completed("welcome"));
    assert_eq!(state.experience, 50);
}
```

---

## Debugging Failed Tests

### Common Failure Patterns

1. **Assertion Failure**: Expected vs actual mismatch

    ```bash
    cargo test -- --nocapture
    ```

2. **Panic in Test**: Unwrap on None or Err

    ```rust
    // Add better error messages
    .expect("Descriptive error message here")
    ```

3. **Flaky Test**: Passes sometimes, fails others

    - Remove timing dependencies
    - Ensure test isolation
    - Check for shared mutable state

4. **Slow Tests**: Taking too long

    ```bash
    cargo test -- --show-output --test-threads=1
    ```

---

## Adding New Tests

### Checklist for New Challenges

When adding a new challenge:

1. [ ] Write test for correct answer(s)
2. [ ] Test case variations (upper/lower/mixed)
3. [ ] Test incorrect/similar answers
4. [ ] Test empty input
5. [ ] Test whitespace handling
6. [ ] Add to metadata tests (ID, title, hints)
7. [ ] Update challenge count assertion
8. [ ] Update level distribution test
9. [ ] Document special validation rules

### Checklist for New Features

When adding a new feature:

1. [ ] Write unit tests for new functions
2. [ ] Add integration test for feature workflow
3. [ ] Test edge cases and error conditions
4. [ ] Update documentation
5. [ ] Run full test suite
6. [ ] Check coverage didn't decrease
7. [ ] Update CHANGELOG.md

---

## Future Testing Enhancements

### Phase 1 (Q4 2025)

-   [ ] Complete GameState integration tests
-   [ ] Add property-based tests for validators
-   [ ] Set up GitHub Actions CI
-   [ ] Achieve 80% code coverage
-   [ ] Add test fixtures and helpers

### Phase 2 (Q1 2026)

-   [ ] UI component testing framework
-   [ ] End-to-end test suite
-   [ ] Performance benchmarks
-   [ ] Mutation testing (cargo-mutants)
-   [ ] Fuzz testing for input validation

### Phase 3 (Q2 2026)

-   [ ] Visual regression testing
-   [ ] Load testing for multiplayer features
-   [ ] Security testing automation
-   [ ] Test data generation tools
-   [ ] Automated test report generation

---

## Resources

### Rust Testing

-   [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
-   [Rust By Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
-   [cargo test documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

### Testing Tools

-   [proptest](https://github.com/proptest-rs/proptest) - Property-based testing
-   [quickcheck](https://github.com/BurntSushi/quickcheck) - QuickCheck for Rust
-   [criterion](https://github.com/bheisler/criterion.rs) - Benchmarking
-   [tarpaulin](https://github.com/xd009642/tarpaulin) - Code coverage
-   [cargo-watch](https://github.com/watchexec/cargo-watch) - Watch mode
-   [assert_cmd](https://github.com/assert-rs/assert_cmd) - CLI testing

### Best Practices

-   [Rust API Guidelines - Testing](https://rust-lang.github.io/api-guidelines/)
-   [Test Driven Development in Rust](https://testdriven.io/blog/rust-tdd/)

---

## Questions & Support

For questions about testing:

1. Check this document first
2. Review existing tests in `src/challenges.rs`
3. Consult CONTRIBUTING.md
4. Open an issue on GitHub

---

**Last Updated**: October 21, 2025
**Version**: 1.0
**Status**: Active Development
