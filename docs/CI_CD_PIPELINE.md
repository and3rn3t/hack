# CI/CD Pipeline Documentation

## Overview

The Hack: Ghost Protocol uses GitHub Actions for continuous integration and deployment. Our CI/CD pipeline ensures code quality, runs comprehensive tests, and automates releases.

## Workflows

### 1. Quick Check (`quick-check.yml`)

**Purpose**: Fast validation for every commit
**Runs on**: Every push and PR
**Duration**: ~1-2 minutes

**Steps**:

1. ✅ Code formatting check (`cargo fmt`)
2. ✅ Linting with Clippy (`cargo clippy`)
3. ✅ Run all tests (`cargo test`)

**Badge**: [![Quick Check](https://github.com/and3rn3t/hack/workflows/Quick%20Check/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/quick-check.yml)

This workflow mirrors the `scripts/quick-check.ps1` script and provides the fastest feedback loop.

### 2. Test Suite (`test-suite.yml`)

**Purpose**: Comprehensive testing across all platforms
**Runs on**: Push to main/develop, PRs
**Duration**: ~5-10 minutes

**Test Categories**:

-   **Unit Tests**: Tests for individual modules (74 tests)

    -   Runs on: Ubuntu, Windows, macOS
    -   Tests: `challenges`, `state`, `narrative`, `ui`

-   **Integration Tests**: End-to-end testing (14 tests)

    -   Save/load functionality
    -   State persistence
    -   Cross-platform compatibility

-   **Property-Based Tests**: Fuzzing with proptest (10 tests)

    -   Tests invariants and edge cases
    -   Validates no panics on arbitrary input

-   **Documentation Tests**: Code examples in docs
    -   Ensures documentation is accurate

**Badge**: [![Test Suite](https://github.com/and3rn3t/hack/workflows/Test%20Suite/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/test-suite.yml)

### 3. Full CI/CD Pipeline (`ci.yml`)

**Purpose**: Complete validation and release automation
**Runs on**: Push to main/develop, PRs, releases
**Duration**: ~15-30 minutes

**Jobs**:

#### Code Quality

-   **check**: `cargo check` for compilation errors
-   **fmt**: Format validation
-   **clippy**: Linting (with `-D warnings`)

#### Testing

-   **test**: Full test suite on multiple platforms
    -   Matrix: [Ubuntu, Windows, macOS] × [stable, nightly]
    -   88+ tests total

#### Security

-   **security-audit**: `cargo audit` for vulnerabilities
-   **deny**: `cargo deny` for dependency checks

#### Build & Release

-   **build**: Release binaries for all platforms

    -   Linux (x86_64)
    -   Windows (x86_64)
    -   macOS (x86_64, ARM64)
    -   Stripped and optimized

-   **release**: Create GitHub releases
    -   Automated on version tags
    -   Uploads platform-specific archives

#### Coverage & Docs

-   **coverage**: Code coverage with Tarpaulin

    -   Generates XML for Codecov
    -   HTML report as artifact
    -   Runs on main branch only

-   **docs**: Build and deploy rustdoc
    -   Deploys to GitHub Pages
    -   Updates on main branch

**Badge**: [![CI/CD](https://github.com/and3rn3t/hack/workflows/CI%2FCD/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/ci.yml)

## Test Statistics

| Metric            | Count       |
| ----------------- | ----------- |
| **Total Tests**   | 88+         |
| Unit Tests        | 74          |
| Integration Tests | 14          |
| Property Tests    | 10          |
| Doc Tests         | 0 (planned) |

**Test Isolation**: ✅ Fixed! All tests use unique temp files.

## Local Development

### Run What CI Runs

**Quick Check** (same as CI):

```bash
pwsh scripts/quick-check.ps1
```

**Full Test Suite**:

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Property tests
cargo test proptests

# All tests
cargo test
```

**Security Checks**:

```bash
cargo audit
cargo deny check
```

**Coverage Report**:

```bash
# Linux/macOS only
./scripts/test-coverage.sh
```

### VS Code Tasks

All CI checks are available as VS Code tasks:

-   `Dev: Quick Check` - Fastest validation
-   `Test: All Tests` - Full test suite
-   `Test: Coverage Report` - Generate coverage
-   `Build: Release` - Build optimized binary

Access via: `Ctrl+Shift+P` → "Run Task"

## CI Configuration

### Caching Strategy

We cache:

-   `~/.cargo/registry` - Downloaded crates
-   `~/.cargo/git` - Git dependencies
-   `target/` - Build artifacts

**Cache Keys**: Include `Cargo.lock` hash for invalidation on dependency changes.

### Environment Variables

```yaml
CARGO_TERM_COLOR: always # Colored output
RUST_BACKTRACE: 1 # Stack traces on panic
```

### Matrix Testing

**OS Matrix**: `[ubuntu-latest, windows-latest, macos-latest]`
**Rust Matrix**: `[stable, nightly]`

This ensures:

-   Cross-platform compatibility
-   Future Rust version compatibility
-   No platform-specific bugs

## Triggering Workflows

### Automatic Triggers

-   **On Push**: `main`, `develop` branches
-   **On PR**: Targeting `main` or `develop`
-   **On Release**: When creating a release tag

### Manual Trigger

All workflows support `workflow_dispatch` for manual runs:

1. Go to GitHub Actions
2. Select workflow
3. Click "Run workflow"
4. Choose branch

## Badges

Add these to your PR descriptions or README:

```markdown
[![Quick Check](https://github.com/and3rn3t/hack/workflows/Quick%20Check/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/quick-check.yml)
[![Test Suite](https://github.com/and3rn3t/hack/workflows/Test%20Suite/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/test-suite.yml)
[![CI/CD](https://github.com/and3rn3t/hack/workflows/CI%2FCD/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/ci.yml)
```

## Release Process

### Creating a Release

1. **Update Version**: In `Cargo.toml`

    ```toml
    [package]
    version = "0.2.0"
    ```

2. **Update CHANGELOG.md**: Document changes

3. **Commit and Tag**:

    ```bash
    git commit -am "Release v0.2.0"
    git tag v0.2.0
    git push origin main --tags
    ```

4. **Create GitHub Release**:

    - Go to Releases → New Release
    - Choose tag `v0.2.0`
    - Add release notes
    - Click "Publish release"

5. **CI Automation**:
    - Builds release binaries for all platforms
    - Creates `.zip` (Windows) and `.tar.gz` (Unix) archives
    - Uploads to GitHub release automatically

### Release Artifacts

-   `hack_simulator-linux-amd64.tar.gz`
-   `hack_simulator-windows-amd64.zip`
-   `hack_simulator-macos-amd64.tar.gz`
-   `hack_simulator-macos-arm64.tar.gz`

## Troubleshooting CI Failures

### Format Check Failed

**Error**: "code is not formatted correctly"

**Fix**:

```bash
cargo fmt
git commit -am "Format code"
git push
```

### Clippy Failed

**Error**: "clippy warnings found"

**Fix**:

```bash
cargo clippy --fix --allow-dirty
git commit -am "Fix clippy warnings"
git push
```

### Tests Failed

**Error**: "test result: FAILED"

**Fix**:

1. Run locally: `cargo test`
2. Check specific test: `cargo test test_name -- --nocapture`
3. Fix the issue
4. Commit and push

### Build Failed on Specific Platform

**Error**: Build fails on Windows but passes on Linux

**Fix**:

1. Check platform-specific code paths
2. Test locally if possible
3. Use conditional compilation: `#[cfg(target_os = "windows")]`

### Coverage Upload Failed

**Error**: Codecov token missing

**Note**: Coverage upload requires `CODECOV_TOKEN` secret. This is optional and won't fail the build.

## Performance Optimization

### Speed Up CI Runs

1. **Use Caching**: Already configured
2. **Parallel Jobs**: Matrix testing runs in parallel
3. **Fail Fast**: Disabled to see all failures
4. **Incremental Builds**: Cached via `target/`

### Current Timings

| Workflow    | Duration   |
| ----------- | ---------- |
| Quick Check | ~1-2 min   |
| Test Suite  | ~5-10 min  |
| Full CI/CD  | ~15-30 min |

## Future Enhancements

### Planned

-   [ ] **Benchmarking**: Add performance regression tests
-   [ ] **Mutation Testing**: Test the tests with `cargo-mutants`
-   [ ] **Security Scanning**: Add CodeQL or similar
-   [ ] **Dependency Updates**: Renovate or Dependabot
-   [ ] **Pre-commit Hooks**: Local validation before push

### Under Consideration

-   [ ] **Nightly Builds**: Scheduled runs for stability
-   [ ] **Beta Releases**: Automated pre-releases
-   [ ] **Container Images**: Docker builds
-   [ ] **Fuzzing**: Continuous fuzzing with cargo-fuzz

## Best Practices

### For Contributors

1. ✅ Run `pwsh scripts/quick-check.ps1` before pushing
2. ✅ Write tests for new features
3. ✅ Keep tests isolated (use unique temp files)
4. ✅ Update documentation with code changes
5. ✅ Check CI results before requesting review

### For Maintainers

1. ✅ Review CI logs on PRs
2. ✅ Keep workflows up to date
3. ✅ Monitor security advisories
4. ✅ Update dependencies regularly
5. ✅ Tag releases properly

## Resources

-   [GitHub Actions Documentation](https://docs.github.com/en/actions)
-   [Rust CI with GitHub Actions](https://github.com/actions-rs)
-   [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
-   [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
-   [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)

## Questions?

See also:

-   [TESTING.md](TESTING.md) - Testing strategy and guidelines
-   [DEV_WORKFLOW.md](DEV_WORKFLOW.md) - Development workflow guide
-   [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines

---

**Last Updated**: October 21, 2025
**Workflows Version**: v2.0
