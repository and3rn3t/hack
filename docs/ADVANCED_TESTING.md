# Advanced Testing Infrastructure

**Date**: October 21, 2025
**Status**: ✅ Complete

---

## Overview

This document covers advanced testing tools beyond standard unit and integration tests: **benchmarking**, **mutation testing**, and **fuzzing**.

---

## 1. Performance Benchmarking 🏃

### Purpose

Measure and track performance of critical code paths to prevent regressions.

### Tool: Criterion

-   **Industry standard** for Rust benchmarking
-   **Statistical analysis** with confidence intervals
-   **HTML reports** with graphs and comparisons
-   **Baseline comparison** to detect regressions

### Benchmarks Created

#### Challenge Benchmarks (`benches/challenge_benchmarks.rs`)

-   **Validation Performance**: Test challenge answer validation speed
-   **Retrieval Operations**: Measure challenge fetching and filtering
-   **Cryptography Operations**: Benchmark encoding/decoding
-   **Complexity Analysis**: Compare simple vs. complex validators

**Key Metrics**:

-   Simple validation: ~10-50 ns
-   Medium complexity: ~100-500 ns
-   Complex operations: ~1-5 μs

#### State Benchmarks (`benches/state_benchmarks.rs`)

-   **State Creation**: New game state initialization
-   **Modifications**: Challenge completion, sanity changes
-   **Queries**: has_completed checks
-   **Serialization**: JSON encoding at various sizes
-   **File I/O**: Save/load operations
-   **Level Progression**: XP calculations

**Key Metrics**:

-   State creation: ~100-200 ns
-   Serialization (small): ~1-5 μs
-   Serialization (large): ~10-50 μs
-   File I/O: ~100-500 μs

### Running Benchmarks

**All Benchmarks**:

```bash
# PowerShell
pwsh scripts/run-benchmarks.ps1

# Bash
./scripts/run-benchmarks.sh

# VS Code
Ctrl+Shift+P → "Run Task" → "Benchmark: Run All"
```

**Specific Benchmark**:

```bash
cargo bench --bench challenge_benchmarks
cargo bench --bench state_benchmarks
```

**Single Test**:

```bash
cargo bench base64_decode_validation
```

### Viewing Results

1. **Terminal Output**: Immediate results with statistical analysis
2. **HTML Report**: `target/criterion/report/index.html`
3. **Comparison**: Automatically compares to previous baseline

**Example Output**:

```
validate_correct_answer time:   [42.123 ns 42.456 ns 42.789 ns]
                        change: [-2.3456% -1.2345% +0.1234%] (p = 0.12 > 0.05)
                        No change in performance detected.
```

### Interpreting Results

-   **Green**: Performance improved
-   **Red**: Performance regressed (may need investigation)
-   **Gray**: No significant change

**Confidence Intervals**:

-   Shows measurement uncertainty
-   Narrow intervals = more reliable
-   p-value indicates statistical significance

### CI Integration

Benchmarks are **not run in CI** by default (too time-consuming). Options:

1. **Manual Trigger**: Run via workflow_dispatch
2. **Scheduled**: Nightly/weekly performance testing
3. **Release**: Run before major releases

### Best Practices

-   **Warmup**: Criterion handles this automatically
-   **Multiple Runs**: 100+ iterations for statistical validity
-   **Consistent Environment**: Close other applications
-   **Baseline**: Commit baselines to track over time
-   **Regression Alerts**: Set performance budgets

---

## 2. Mutation Testing 🧬

### Purpose

Test the quality of your tests by introducing bugs (mutations) and checking if tests catch them.

### Tool: cargo-mutants

-   **Smart mutations**: Realistic code changes
-   **Fast execution**: Parallel testing
-   **Detailed reports**: Which mutations were caught/missed
-   **Rust-aware**: Understands Rust semantics

### How It Works

1. **Generate Mutants**: Modify code (e.g., `>` becomes `>=`)
2. **Run Tests**: Execute test suite against each mutant
3. **Analyze**: Did tests catch the bug?

**Outcomes**:

-   ✅ **Caught**: Tests failed (good!)
-   ⏱️ **Timeout**: Tests ran too long (probably caught)
-   ❌ **Missed**: Tests passed (bad! Missing coverage)
-   🚫 **Unviable**: Mutant didn't compile (expected)

### Mutations Applied

-   **Comparison operators**: `<` ↔ `<=`, `==` ↔ `!=`
-   **Arithmetic**: `+` ↔ `-`, `*` ↔ `/`
-   **Boolean logic**: `&&` ↔ `||`, `true` ↔ `false`
-   **Return values**: Early returns, changed values
-   **Boundary conditions**: Off-by-one errors

### Running Mutation Tests

**Quick Test** (limited mutants):

```bash
# PowerShell
pwsh scripts/run-mutation-tests.ps1 -Quick

# Bash
./scripts/run-mutation-tests.sh --quick

# VS Code
Ctrl+Shift+P → "Run Task" → "Advanced: Mutation Testing"
```

**Full Test** (all mutants, may take 10-30 minutes):

```bash
pwsh scripts/run-mutation-tests.ps1
```

**Specific File**:

```bash
pwsh scripts/run-mutation-tests.ps1 -File src/state.rs
```

**Installation**:

```bash
pwsh scripts/run-mutation-tests.ps1 -Install
```

### Analyzing Results

**Output Location**: `mutants.out/outcomes.txt`

**Example**:

```
Caught: 45/50 (90%)
Missed: 3/50 (6%)
Timeout: 2/50 (4%)
Unviable: 5 (excluded from score)
```

**Mutation Score**: (Caught + Timeout) / (Total - Unviable)

**Target Scores**:

-   ✅ **>90%**: Excellent test coverage
-   👍 **80-90%**: Good coverage
-   ⚠️ **70-80%**: Needs improvement
-   ❌ **<70%**: Poor coverage

### Fixing Missed Mutations

1. **Review**: Check `mutants.out/missed.txt`
2. **Understand**: Why didn't tests fail?
3. **Add Tests**: Cover the missed case
4. **Re-run**: Verify mutation is now caught

**Example**:

```rust
// Mutation: `>` changed to `>=`
if sanity > 100 { ... }

// Missing test: sanity == 100 edge case
// Add: test_sanity_at_maximum()
```

### Best Practices

-   **Start Small**: Use `-Quick` flag initially
-   **Iterate**: Fix missed mutations incrementally
-   **Focus**: Target critical modules first
-   **Automate**: Run periodically (weekly/monthly)

---

## 3. Fuzzing 🎲

### Purpose

Find crashes, panics, and edge cases by testing with random/malformed input.

### Tool: cargo-fuzz

-   **Coverage-guided**: Learns which inputs are interesting
-   **Mutation-based**: Evolves inputs to find bugs
-   **Crash detection**: Finds panics, overflows, assertion failures
-   **Corpus management**: Saves interesting inputs

### Fuzz Targets Created

#### 1. Challenge Validators (`fuzz_challenge_validators`)

Tests all 26 challenge validators with random input.

**What It Tests**:

-   No panics on arbitrary input
-   Handles invalid UTF-8
-   Manages very long strings
-   Deals with special characters

#### 2. State Deserialization (`fuzz_state_deserialization`)

Tests JSON deserialization with malformed data.

**What It Tests**:

-   Invalid JSON structures
-   Missing fields
-   Wrong types
-   Nested corruption

#### 3. State Operations (`fuzz_state_operations`)

Tests game state modifications with random inputs.

**What It Tests**:

-   Random player names
-   Arbitrary challenge IDs
-   Extreme sanity values
-   Edge case XP values

### Requirements

**Nightly Rust Toolchain**:

```bash
rustup install nightly
```

**cargo-fuzz**:

```bash
pwsh scripts/run-fuzz.ps1 -Install
```

### Running Fuzz Tests

**Quick Test** (60 seconds):

```bash
# PowerShell
pwsh scripts/run-fuzz.ps1 -Seconds 60

# Bash
./scripts/run-fuzz.sh --seconds 60

# VS Code
Ctrl+Shift+P → "Run Task" → "Advanced: Fuzzing (60s)"
```

**Specific Target**:

```bash
pwsh scripts/run-fuzz.ps1 -Target fuzz_state_deserialization -Seconds 300
```

**Continuous Fuzzing** (until crash found):

```bash
cargo +nightly fuzz run fuzz_challenge_validators
```

### Understanding Results

**Normal Output**:

```
#12345: cov: 234 ft: 567 corp: 89 exec/s: 1234 ...
```

-   **cov**: Code coverage (unique edges)
-   **ft**: Features (interesting behaviors)
-   **corp**: Corpus size (saved inputs)
-   **exec/s**: Executions per second

**Crash Found**:

```
==12345== ERROR: libFuzzer: deadly signal
```

Crash input saved to: `fuzz/artifacts/<target>/crash-<hash>`

### Fixing Crashes

1. **Reproduce**: Run with crash input

    ```bash
    cargo +nightly fuzz run fuzz_challenge_validators fuzz/artifacts/.../crash-abc123
    ```

2. **Debug**: Use backtrace

    ```bash
    RUST_BACKTRACE=1 cargo +nightly fuzz run ...
    ```

3. **Fix**: Handle the edge case properly

4. **Verify**: Re-run fuzzer

### Corpus Management

**Interesting Inputs** saved to:

```
fuzz/corpus/<target>/
```

These inputs found new code paths or behaviors.

**Minimize Corpus**:

```bash
cargo +nightly fuzz cmin <target>
```

Reduces corpus to minimal set covering same code paths.

### CI Integration

Fuzzing is **not run in CI** by default (too time-consuming). Options:

1. **OSS-Fuzz**: Continuous fuzzing service (for public projects)
2. **Scheduled**: Weekly fuzzing runs
3. **Pre-release**: Fuzz before major releases

### Best Practices

-   **Start Short**: 60 seconds to verify setup
-   **Run Long**: 1+ hours to find real bugs
-   **Multiple Targets**: Fuzz different modules
-   **Save Corpus**: Commit interesting inputs
-   **Continuous**: Run overnight or on spare machines

---

## Comparison Matrix

| Feature        | Benchmarking         | Mutation Testing | Fuzzing          |
| -------------- | -------------------- | ---------------- | ---------------- |
| **Purpose**    | Performance          | Test Quality     | Bug Finding      |
| **Speed**      | Fast (minutes)       | Slow (10-30 min) | Variable         |
| **Automation** | Manual/Scheduled     | Periodic         | Continuous       |
| **Output**     | Metrics              | Coverage Score   | Crashes          |
| **CI Ready**   | No (too slow)        | No (too slow)    | No (too slow)    |
| **Value**      | Prevents regressions | Improves tests   | Finds edge cases |

---

## Recommended Workflow

### Weekly Development

1. **Quick Check**: `pwsh scripts/quick-check.ps1` (before every commit)
2. **Benchmarks**: Run if changing performance-critical code
3. **Mutation Test**: Quick mode on modified files
4. **Fuzzing**: 5-10 minutes during breaks

### Before Release

1. **Full Test Suite**: `cargo test`
2. **Full Benchmarks**: All benchmarks, compare to baseline
3. **Full Mutation Test**: Target >85% mutation score
4. **Extended Fuzzing**: 1+ hour per target

### Monthly Maintenance

1. **Review Benchmarks**: Check for performance drift
2. **Full Mutation Test**: Ensure test quality maintained
3. **Corpus Review**: Check saved fuzz inputs
4. **Documentation**: Update benchmarks in docs

---

## Metrics & Goals

### Benchmarking Targets

-   **Challenge Validation**: <100 ns (simple), <1 μs (complex)
-   **State Serialization**: <10 μs (typical state)
-   **File I/O**: <500 μs (save/load)

### Mutation Testing Targets

-   **Overall Score**: >85%
-   **Critical Modules**: >90% (state.rs, challenges.rs)
-   **UI Modules**: >75% (ui.rs, narrative.rs)

### Fuzzing Targets

-   **Duration**: 1+ hour per target before release
-   **Corpus Size**: 50-200 inputs per target
-   **Crashes**: Zero unhandled panics

---

## Tools Installation

### All Tools

```bash
# Benchmarking (already in Cargo.toml)
cargo install cargo-criterion

# Mutation Testing
cargo install cargo-mutants

# Fuzzing (requires nightly)
rustup install nightly
cargo install cargo-fuzz
```

### Verification

```bash
cargo bench --version
cargo mutants --version
cargo +nightly fuzz --version
```

---

## Troubleshooting

### Benchmarks Unstable

**Problem**: Large variance in results

**Solutions**:

-   Close other applications
-   Disable CPU throttling
-   Run multiple times
-   Increase sample size

### Mutation Tests Timeout

**Problem**: Tests take too long per mutant

**Solutions**:

-   Use `--timeout-multiplier 2`
-   Skip slow integration tests: `--skip-calls-in-trees tests/`
-   Focus on specific files

### Fuzzing Not Finding Issues

**Problem**: No crashes found

**Solutions**:

-   Run longer (hours, not minutes)
-   Try different seed: `--seed 42`
-   Check corpus diversity
-   May indicate robust code! ✅

---

## Resources

-   [Criterion Docs](https://bheisler.github.io/criterion.rs/book/)
-   [cargo-mutants Guide](https://github.com/sourcefrog/cargo-mutants)
-   [Rust Fuzz Book](https://rust-fuzz.github.io/book/)
-   [OSS-Fuzz](https://github.com/google/oss-fuzz)

---

## Questions?

See also:

-   [TESTING_STRATEGY.md](TESTING_STRATEGY.md) - Overall testing approach
-   [CI_CD_PIPELINE.md](CI_CD_PIPELINE.md) - CI/CD configuration
-   [DEV_WORKFLOW.md](DEV_WORKFLOW.md) - Development workflows

---

**Last Updated**: October 21, 2025
**Version**: 1.0
