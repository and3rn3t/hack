# Contributing to The Hack: Ghost Protocol

First off, thank you for considering contributing to The Hack: Ghost Protocol! It's people like you that make this project a great learning tool for the cybersecurity community.

## Code of Conduct

By participating in this project, you are expected to uphold our standards:

-   Be respectful and inclusive
-   Focus on educational value
-   Provide constructive feedback
-   Help maintain the horror theme and game balance
-   Follow ethical hacking principles

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

-   **Use a clear and descriptive title**
-   **Describe the exact steps to reproduce the problem**
-   **Provide specific examples** (terminal output, save file snippets)
-   **Describe the behavior you observed** and what you expected
-   **Include your environment details** (OS, Rust version, terminal emulator)
-   **Add screenshots** if relevant

Example bug report template:

```markdown
## Bug Description

[Clear description of the bug]

## Steps to Reproduce

1. Start game
2. Select challenge X
3. Enter answer Y
4. Observe behavior Z

## Expected Behavior

[What should happen]

## Actual Behavior

[What actually happens]

## Environment

-   OS: Windows 11 / Ubuntu 22.04 / macOS 14
-   Rust version: 1.75.0
-   Terminal: Windows Terminal / GNOME Terminal / iTerm2
-   Game version: 0.1.0
```

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

-   **Use a clear and descriptive title**
-   **Provide a detailed description** of the suggested enhancement
-   **Explain why this enhancement would be useful**
-   **Consider the educational value**
-   **Think about how it fits the horror theme**
-   **Suggest implementation approach** if you have ideas

### Creating New Challenges

We're always looking for new educational challenges! Here's how to contribute:

1. **Check the Roadmap** - See if your challenge fits a planned category
2. **Open an issue first** - Discuss your challenge idea before coding
3. **Follow the challenge template**:

    - Educational objective (what will players learn?)
    - Difficulty level (0-4)
    - Category (Crypto, Web, Network, etc.)
    - Hints progression (3-5 hints)
    - Unique identifier
    - Sanity cost (balanced for difficulty)
    - XP reward (balanced for difficulty)

4. **Test thoroughly** - Ensure hints are helpful but not spoilers
5. **Update documentation** - Add to WALKTHROUGH.md if needed

Example challenge proposal:

```markdown
## Challenge Proposal: DNS Tunneling Detection

**Category**: Network Security
**Level**: 3
**Objective**: Teach players about DNS tunneling for data exfiltration

**Description**:
Players analyze DNS query logs to identify suspicious patterns indicating
DNS tunneling. They'll learn about normal vs abnormal DNS query behavior.

**Solution Approach**:
Look for unusually long subdomain names, high query frequency to single domain,
and encoded data patterns in DNS requests.

**Hints**:

1. DNS queries usually look for domain names, not data
2. Check the length and character patterns of subdomains
3. Look for base64-like patterns in the DNS queries

**Sanity Cost**: 10
**XP Reward**: 150
```

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Follow the coding standards** (see `.github/copilot-instructions.md`)
3. **Add tests** if applicable
4. **Update documentation** to reflect changes
5. **Ensure the game compiles** with `cargo build`
6. **Test on multiple platforms** if possible
7. **Keep commits atomic** and well-described

#### Pull Request Template

```markdown
## Description

[Brief description of changes]

## Type of Change

-   [ ] Bug fix (non-breaking change which fixes an issue)
-   [ ] New feature (non-breaking change which adds functionality)
-   [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
-   [ ] Documentation update
-   [ ] New challenge

## Testing Performed

-   [ ] Tested on Windows
-   [ ] Tested on Linux
-   [ ] Tested on macOS
-   [ ] Save/load compatibility verified
-   [ ] All challenges still solvable
-   [ ] Game balance maintained

## Checklist

-   [ ] My code follows the project's style guidelines
-   [ ] I have performed a self-review
-   [ ] I have commented complex code sections
-   [ ] I have updated relevant documentation
-   [ ] My changes generate no new warnings
-   [ ] I have tested save file compatibility
-   [ ] Horror theme consistency maintained
-   [ ] Educational value preserved

## Related Issues

Closes #[issue_number]
```

## Development Setup

### Prerequisites

-   Rust 1.70.0 or later
-   Git
-   A terminal emulator with ANSI color support

### Setup Steps

1. **Clone the repository**

    ```bash
    git clone https://github.com/and3rn3t/hack.git
    cd hack
    ```

2. **Build the project**

    ```bash
    cargo build
    ```

3. **Run tests** (when available)

    ```bash
    cargo test
    ```

4. **Run the game**

    ```bash
    cargo run
    ```

5. **Build release version**

    ```bash
    cargo build --release
    ./target/release/hack_simulator
    ```

### Development Workflow

1. **Create a feature branch**

    ```bash
    git checkout -b feature/your-feature-name
    ```

2. **Make your changes**

    - Edit code in `src/`
    - Follow Rust conventions and project style
    - Test frequently with `cargo run`

3. **Format your code**

    ```bash
    cargo fmt
    ```

4. **Check for issues**

    ```bash
    cargo clippy -- -D warnings
    ```

5. **Commit your changes**

    ```bash
    git add .
    git commit -m "feat: Add new steganography challenge"
    ```

6. **Push to your fork**

    ```bash
    git push origin feature/your-feature-name
    ```

7. **Create a Pull Request** on GitHub

### Commit Message Convention

We follow conventional commits for clarity:

-   `feat:` - New feature
-   `fix:` - Bug fix
-   `docs:` - Documentation changes
-   `style:` - Code style changes (formatting)
-   `refactor:` - Code refactoring
-   `test:` - Adding tests
-   `chore:` - Maintenance tasks

Examples:

```
feat: Add OSINT challenge for social media analysis
fix: Correct Base64 validation in encoding challenge
docs: Update WALKTHROUGH.md with new challenges
refactor: Simplify challenge validation logic
```

## Code Style Guidelines

### Rust Style

-   Follow `rustfmt` defaults
-   Use `cargo clippy` and fix all warnings
-   Keep functions under 50 lines when possible
-   Use descriptive variable names
-   Add doc comments for public APIs

### Challenge Code Style

```rust
Challenge {
    id: "descriptive_id",              // snake_case, unique
    level: 2,                           // 0-4
    title: "Challenge Title",           // Title Case
    category: ChallengeCategory::Web,   // Appropriate category
    description: "Full context...",     // Multi-line OK
    prompt: "What the user sees...",    // Clear question
    solution: "exact_answer",           // Case-sensitive unless noted
    hints: vec![
        "First hint - general direction",
        "Second hint - more specific",
        "Third hint - almost there",
    ],
    sanity_cost: 10,                    // 5-15 based on difficulty
    xp_reward: 100,                     // 50-200 based on difficulty
}
```

### UI Code Style

```rust
// Always clean up terminal state
execute!(stdout(), LeaveAlternateScreen)?;
terminal::disable_raw_mode()?;

// Use project color constants
print_colored("Success!", Color::Green)?;
print_colored("Warning!", Color::Yellow)?;
print_colored("Error!", Color::Red)?;

// Handle user input safely
let input = get_user_input()?.trim().to_lowercase();
```

## Testing Guidelines

### Manual Testing Checklist

Before submitting a PR, verify:

-   [ ] Game starts without errors
-   [ ] All new challenges are solvable
-   [ ] Hints are progressive and helpful
-   [ ] Save/load works with new changes
-   [ ] No terminal corruption on exit
-   [ ] Colors display correctly
-   [ ] Horror theme is maintained
-   [ ] Game balance feels right
-   [ ] Cross-platform compatibility (if possible)

### Automated Testing (Future)

We plan to add:

-   Unit tests for challenge validators
-   Integration tests for save/load
-   Property-based tests for encoding challenges
-   CI/CD pipeline

## Documentation Standards

### Code Documentation

````rust
/// Validates a challenge answer against the expected solution.
///
/// # Arguments
/// * `answer` - The user's answer (trimmed and lowercased)
/// * `solution` - The expected correct answer
///
/// # Returns
/// `true` if the answer matches, `false` otherwise
///
/// # Examples
/// ```
/// assert_eq!(validate_answer("hello", "hello"), true);
/// assert_eq!(validate_answer("HELLO", "hello"), true);
/// ```
fn validate_answer(answer: &str, solution: &str) -> bool {
    answer.trim().to_lowercase() == solution.to_lowercase()
}
````

### User-Facing Documentation

-   Update `README.md` for user-facing changes
-   Update `WALKTHROUGH.md` for new challenges
-   Update `SECURITY.md` for security-related changes
-   Update `docs/ROADMAP.md` for major features

## Community

### Getting Help

-   **GitHub Discussions**: Ask questions and discuss ideas
-   **Discord Server**: Real-time chat (coming soon)
-   **GitHub Issues**: Report bugs and request features

### Recognition

Contributors will be:

-   Listed in `CONTRIBUTORS.md`
-   Mentioned in release notes
-   Credited in challenge descriptions (if applicable)
-   Invited to the core team (for consistent contributors)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (check LICENSE file).

## Questions?

Don't hesitate to ask questions! Create a GitHub Discussion or reach out to the maintainers.

Thank you for contributing to making cybersecurity education more accessible and engaging! 👻🔒
