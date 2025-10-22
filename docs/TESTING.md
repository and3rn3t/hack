# Testing Guide

This document provides comprehensive information about testing The Hack: Ghost Protocol.

## Quick Start

```bash
# Run all tests
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run a specific test
cargo test test_welcome_challenge

# Run tests for the challenges module
cargo test challenges::tests
```

## Test Suite Overview

The project includes 34 automated tests that validate all aspects of the challenge system:

### Test Categories

#### 1. Metadata Validation Tests (8 tests)

-   `test_all_challenges_have_valid_ids` - Ensures IDs are non-empty and alphanumeric
-   `test_all_challenges_have_unique_ids` - Prevents duplicate challenge IDs
-   `test_all_challenges_have_titles` - Verifies all challenges have titles
-   `test_all_challenges_have_descriptions` - Ensures descriptions exist and are meaningful (>20 chars)
-   `test_all_challenges_have_hints` - Validates each challenge has at least 2 hints
-   `test_no_empty_hints` - Ensures no hint is an empty string
-   `test_challenge_ids_follow_naming_convention` - Enforces snake_case naming
-   `test_challenge_levels_are_valid` - Ensures levels are 0-4

#### 2. Balance Tests (2 tests)

-   `test_challenge_rewards_are_positive` - Validates XP and sanity costs are positive
-   `test_rewards_scale_with_difficulty` - Ensures higher levels have higher average XP

#### 3. Challenge Organization Tests (3 tests)

-   `test_get_challenges_for_level` - Validates level filtering
-   `test_total_challenge_count` - Ensures exactly 26 challenges exist
-   `test_level_distribution` - Verifies distribution: L0(6), L1(7), L2(7), L3(5), L4(1)

#### 4. Answer Validation Tests (21 tests)

Individual tests for each challenge's answer validation:

-   `test_welcome_challenge` - Base64 decoding
-   `test_file_discovery_challenge` - Hidden file discovery
-   `test_port_scan_challenge` - Port scanning
-   `test_rot13_challenge` - ROT13 cipher
-   `test_binary_challenge` - Binary to ASCII
-   `test_url_decode_challenge` - URL decoding
-   `test_caesar_cipher_challenge` - Caesar cipher
-   `test_sql_injection_challenge` - SQL injection payloads
-   `test_hex_decode_challenge` - Hexadecimal decoding
-   `test_jwt_token_challenge` - JWT algorithm confusion
-   `test_path_traversal_challenge` - Directory traversal
-   `test_md5_collision_challenge` - MD5 hash cracking
-   `test_command_injection_challenge` - Shell command injection
-   `test_xss_attack_challenge` - XSS basics
-   `test_session_hijack_challenge` - Session hijacking
-   `test_cors_bypass_challenge` - CORS vulnerabilities
-   `test_buffer_overflow_challenge` - Buffer overflow
-   `test_reverse_engineering_challenge` - XOR operations
-   `test_format_string_challenge` - Format string vulnerabilities
-   `test_race_condition_challenge` - TOCTOU attacks
-   `test_integer_overflow_challenge` - Integer overflow

## Test Philosophy

### Comprehensive Coverage

Every challenge must have:

-   ✅ Correct answer validation
-   ✅ Case-insensitive handling (where applicable)
-   ✅ Alternative valid answers (for challenges with multiple solutions)
-   ✅ Invalid answer rejection
-   ✅ Edge case handling (empty strings, whitespace)

### Example Test Structure

```rust
#[test]
fn test_your_challenge() {
    let challenges = get_all_challenges();
    let challenge = challenges.iter()
        .find(|c| c.id == "your_challenge_id")
        .expect("Challenge not found");

    // Test correct answers (including variations)
    assert!((challenge.check_answer)("correct_answer"));
    assert!((challenge.check_answer)("CORRECT_ANSWER")); // uppercase
    assert!((challenge.check_answer)("Correct Answer")); // mixed case

    // Test incorrect answers
    assert!(!(challenge.check_answer)("wrong_answer"));
    assert!(!(challenge.check_answer)("")); // empty string
    assert!(!(challenge.check_answer)("partial")); // partial answer
}
```

## Running Tests in CI/CD

Tests are automatically run on every push and pull request via GitHub Actions:

```yaml
- name: Run tests
  run: cargo test --verbose
```

## Test Results

Current status: **All 34 tests passing** ✅

```
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured
```

## Adding New Tests

### When to Add Tests

Add tests when:

1. Creating a new challenge
2. Modifying answer validation logic
3. Adding new features to the challenge system
4. Fixing bugs in existing challenges

### Test Template for New Challenges

```rust
#[test]
fn test_new_challenge_name() {
    let challenges = get_all_challenges();
    let challenge = challenges.iter()
        .find(|c| c.id == "new_challenge_id")
        .expect("Challenge not found");

    // Test all valid answers
    assert!((challenge.check_answer)("answer1"));
    assert!((challenge.check_answer)("answer2"));

    // Test invalid answers
    assert!(!(challenge.check_answer)("wrong"));

    // Don't forget to update:
    // - test_total_challenge_count
    // - test_level_distribution
}
```

### Checklist for New Challenge Tests

-   [ ] Test at least one correct answer
-   [ ] Test case variations (uppercase, lowercase, mixed)
-   [ ] Test alternative valid answers (if applicable)
-   [ ] Test at least one incorrect answer
-   [ ] Test empty string
-   [ ] Update `test_total_challenge_count` with new count
-   [ ] Update `test_level_distribution` with new level counts
-   [ ] Ensure the challenge exists in `get_all_challenges()`

## Test Coverage Goals

-   **Current**: 34 tests covering 26 challenges
-   **Target**: 100% challenge coverage
-   **Future**: Integration tests for game state, save/load system

## Debugging Failed Tests

If a test fails:

1. **Read the error message carefully** - it shows which assertion failed
2. **Check the challenge ID** - ensure it matches exactly (case-sensitive)
3. **Verify the answer format** - check for extra spaces, case sensitivity
4. **Run the specific test** with `cargo test test_name -- --nocapture`
5. **Check the challenge implementation** in `src/challenges.rs`

### Common Test Failures

#### Challenge Not Found

```
thread 'challenges::tests::test_xyz' panicked at 'Challenge not found'
```

**Solution**: Verify the challenge ID exists in `get_all_challenges()`

#### Answer Validation Failed

```
assertion failed: (challenge.check_answer)("expected_answer")
```

**Solution**: Check the `check_answer` closure implementation for that challenge

#### Count Mismatch

```
assertion `left == right` failed
  left: 25
 right: 26
```

**Solution**: Update the test to match the current challenge count

## Performance

Test suite performance:

-   **Total runtime**: ~10ms
-   **Average per test**: <1ms
-   **Build time**: ~1s (with tests)

## Best Practices

1. **Keep tests focused** - One test per challenge
2. **Use descriptive names** - `test_challenge_id_description`
3. **Test edge cases** - Empty strings, special characters, whitespace
4. **Maintain coverage** - Don't delete tests without replacement
5. **Run tests before committing** - Catch issues early
6. **Document complex tests** - Add comments for non-obvious logic

## Resources

-   [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
-   [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
-   [Project Contributing Guide](../CONTRIBUTING.md)

---

**Last Updated**: December 2024
**Test Count**: 34 tests
**Coverage**: 26/26 challenges (100%)
