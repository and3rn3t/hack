# Project Configuration Checklist ✅

Use this checklist to ensure your development environment is properly configured.

## Initial Setup

### Prerequisites

-   [ ] Rust 1.70.0+ installed (`rustc --version`)
-   [ ] Git installed (`git --version`)
-   [ ] Terminal with ANSI color support
-   [ ] VS Code installed (recommended)

### Repository Setup

-   [ ] Repository cloned
-   [ ] Can build project (`cargo build`)
-   [ ] Can run game (`cargo run`)
-   [ ] Save file generates on first run

## Editor Configuration (VS Code)

### Extensions Installed

-   [ ] `rust-lang.rust-analyzer` - Rust language support
-   [ ] `serayuzgur.crates` - Cargo.toml management
-   [ ] `tamasfe.even-better-toml` - TOML support
-   [ ] `vadimcn.vscode-lldb` - Debugging support
-   [ ] `eamodio.gitlens` - Git integration
-   [ ] `github.copilot` - AI assistance (optional)

### Settings Verified

-   [ ] Auto-format on save working
-   [ ] Clippy integration active
-   [ ] Inlay hints visible
-   [ ] Proper indentation (4 spaces for Rust)
-   [ ] Line ruler at 100 characters

### Tasks Working

-   [ ] Can run `cargo build` task
-   [ ] Can run `cargo run` task
-   [ ] Can run `cargo test` task
-   [ ] Can run `cargo clippy` task
-   [ ] Can run `cargo fmt` task

## Development Workflow

### Code Quality

-   [ ] `cargo fmt` runs without errors
-   [ ] `cargo clippy` shows no warnings
-   [ ] `cargo check` passes
-   [ ] `cargo build` succeeds
-   [ ] `cargo test` passes (when tests exist)

### Git Configuration

-   [ ] Git user name configured
-   [ ] Git email configured
-   [ ] `.gitignore` excludes build artifacts
-   [ ] No `game_save.json` in commits
-   [ ] No `target/` directory in commits

### Commit Standards

-   [ ] Using conventional commits (feat:, fix:, docs:, etc.)
-   [ ] Commit messages are descriptive
-   [ ] Commits are atomic (one logical change)
-   [ ] Branches follow naming convention

## CI/CD Verification

### GitHub Actions

-   [ ] CI pipeline runs on push
-   [ ] Format check passes
-   [ ] Clippy check passes
-   [ ] Tests pass on all platforms
-   [ ] Build succeeds for all targets

### Pre-Commit Checks

Before every commit:

-   [ ] Run `cargo fmt`
-   [ ] Run `cargo clippy -- -D warnings`
-   [ ] Run `cargo test` (if tests exist)
-   [ ] Review changed files
-   [ ] No debug code or commented-out sections

## Documentation

### Read

-   [ ] README.md - Project overview
-   [ ] CONTRIBUTING.md - How to contribute
-   [ ] docs/SETUP.md - Development setup
-   [ ] docs/CONFIGURATION.md - Config reference
-   [ ] docs/ROADMAP.md - Future plans
-   [ ] SECURITY.md - Security policy

### Maintain

-   [ ] Update WALKTHROUGH.md for new challenges
-   [ ] Update README.md for user-facing changes
-   [ ] Update CONTRIBUTING.md for process changes
-   [ ] Update docs/ROADMAP.md for completed items

## Contribution Workflow

### Before Starting

-   [ ] Issue exists for the work
-   [ ] Issue is assigned to you
-   [ ] You've commented on the issue
-   [ ] You understand the requirements

### During Development

-   [ ] Working in a feature branch
-   [ ] Branch name is descriptive
-   [ ] Committing regularly
-   [ ] Following code style guidelines
-   [ ] Adding/updating tests as needed
-   [ ] Documenting new features

### Before Pull Request

-   [ ] All commits are properly formatted
-   [ ] Code is formatted (`cargo fmt`)
-   [ ] No clippy warnings (`cargo clippy`)
-   [ ] Tests pass (`cargo test`)
-   [ ] Documentation updated
-   [ ] CHANGELOG.md updated (if applicable)

### Pull Request

-   [ ] PR title is descriptive
-   [ ] PR description follows template
-   [ ] Related issues linked
-   [ ] Screenshots/output included (if applicable)
-   [ ] All checkboxes in template completed
-   [ ] Ready for review

## Testing

### Manual Testing

-   [ ] Game starts without errors
-   [ ] New features work as expected
-   [ ] Existing features still work
-   [ ] Save/load works correctly
-   [ ] Terminal UI displays properly
-   [ ] Colors render correctly
-   [ ] No terminal corruption on exit

### Cross-Platform (if possible)

-   [ ] Tested on Windows
-   [ ] Tested on Linux
-   [ ] Tested on macOS

### Challenge Testing (if applicable)

-   [ ] Challenge is solvable
-   [ ] Hints are helpful but not spoilers
-   [ ] Answer validation works
-   [ ] XP and sanity costs are balanced
-   [ ] Educational value is clear

## Performance

### Build Times

-   [ ] Debug build completes in reasonable time
-   [ ] Release build completes successfully
-   [ ] No unnecessary dependencies

### Runtime Performance

-   [ ] Game starts quickly (<2 seconds)
-   [ ] No noticeable lag during gameplay
-   [ ] Terminal rendering is smooth
-   [ ] Save/load is fast

## Security

### Code Security

-   [ ] No hardcoded secrets or credentials
-   [ ] User input is validated
-   [ ] File operations are safe
-   [ ] No unsafe code without justification
-   [ ] Dependencies are up to date

### Reporting

-   [ ] Know how to report security issues (SECURITY.md)
-   [ ] Don't commit sensitive information
-   [ ] Review changes before pushing

## Community

### Communication

-   [ ] GitHub Discussions for questions
-   [ ] GitHub Issues for bugs/features
-   [ ] Follow code of conduct
-   [ ] Be respectful and inclusive

### Collaboration

-   [ ] Review others' PRs when possible
-   [ ] Help answer questions
-   [ ] Share knowledge
-   [ ] Provide constructive feedback

## Advanced Setup (Optional)

### Additional Tools

-   [ ] `cargo-watch` - Auto-rebuild on changes
-   [ ] `cargo-edit` - Manage dependencies
-   [ ] `cargo-audit` - Security audits
-   [ ] `cargo-tarpaulin` - Code coverage
-   [ ] `sccache` - Compile cache

### Aliases Configured

-   [ ] `cargo b` for build
-   [ ] `cargo r` for run
-   [ ] `cargo t` for test
-   [ ] `cargo cl` for clippy
-   [ ] Custom aliases in `.cargo/config.toml`

## Troubleshooting

If something doesn't work, check:

-   [ ] Rust version is 1.70.0 or later
-   [ ] All dependencies are installed
-   [ ] VS Code extensions are enabled
-   [ ] `.vscode/settings.json` is not overridden
-   [ ] Terminal supports ANSI colors
-   [ ] No conflicting global Git configs
-   [ ] Cargo cache is not corrupted (`cargo clean`)

## Regular Maintenance

### Weekly

-   [ ] Update dependencies (`cargo update`)
-   [ ] Run security audit (`cargo audit`)
-   [ ] Review open issues
-   [ ] Check CI/CD status

### Monthly

-   [ ] Review and update roadmap
-   [ ] Archive completed milestones
-   [ ] Celebrate contributions
-   [ ] Plan next version

## Getting Help

### Resources

-   **Setup Issues:** docs/SETUP.md
-   **Config Questions:** docs/CONFIGURATION.md
-   **Contribution Help:** CONTRIBUTING.md
-   **General Questions:** GitHub Discussions
-   **Bug Reports:** GitHub Issues

### Contacts

-   **Maintainers:** See CONTRIBUTING.md
-   **Security:** See SECURITY.md
-   **Community:** GitHub Discussions

---

## Status: [ ] Ready to Contribute

Once all essential items are checked:

1. ✅ Environment is configured
2. ✅ Documentation is read
3. ✅ First build succeeds
4. ✅ You understand the workflow
5. ✅ You're ready to make your first contribution!

**Welcome to The Hack: Ghost Protocol development team! 👻🔒**

---

_Save this checklist and refer to it regularly to ensure your setup stays current._
