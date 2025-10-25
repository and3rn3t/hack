# GitHub Copilot Instructions for The Hack: Ghost Protocol

## Project Overview

This is a **production-ready** horror-themed hacking simulator and CTF challenge game built in Rust. It combines educational cybersecurity concepts with an immersive horror narrative and sanity mechanic. The game is designed for both beginners and enthusiasts, featuring progressive difficulty and a rich terminal-based UI.

**Current Status**: Production-ready with comprehensive testing, documentation, and professional development infrastructure.

## Core Architecture

### Module Structure

-   `main.rs` - Entry point with terminal setup and initialization
-   `game.rs` - Main game loop, menu system, user interaction, and theme management
-   `challenges.rs` - Challenge definitions, validation, and hints (11 challenges across 5 levels)
-   `narrative.rs` - Horror story elements, atmospheric text, and glitch effects
-   `state.rs` - Game state management, save/load system, player statistics
-   `ui.rs` - Terminal UI rendering, colors, ASCII art, visual effects, and theming system
-   `tutorial.rs` - Interactive tutorial system for new users

### Key Dependencies

-   `crossterm` - Cross-platform terminal manipulation
-   `serde`/`serde_json` - State serialization and persistence
-   `rand` - Random elements for horror effects
-   `chrono` - Timestamps and game tracking
-   `criterion` - Performance benchmarking (dev dependency)

## Coding Standards & Best Practices

### Rust Idioms

-   Use Rust's ownership system and borrowing rules correctly
-   Prefer `Result<T, E>` for error handling over panics
-   Use pattern matching for control flow
-   Implement `Display` and `Debug` traits where appropriate
-   Leverage the type system for compile-time safety
-   Use `Arc<Mutex<T>>` for thread-safe shared state (e.g., color themes)

### Error Handling

-   Use `crossterm::Result` for terminal operations
-   Use custom error types or `anyhow` for complex error scenarios
-   Always handle user input errors gracefully
-   Provide helpful error messages to users
-   Log errors appropriately for debugging
-   Never panic in production code

### Code Style

-   Follow Rust standard naming conventions (snake_case for functions, PascalCase for types)
-   Keep functions focused and single-purpose
-   Use descriptive variable names
-   Add doc comments (`///`) for public functions and structs
-   Keep line length under 100 characters where reasonable
-   Run `cargo fmt` and `cargo clippy` before committing
-   Maintain consistent error handling patterns

### Terminal UI Guidelines

-   Always use the theming system from `ui.rs` instead of hardcoded colors
-   Test terminal output on both Windows and Unix-like systems
-   Use `crossterm::execute!` and `queue!` appropriately
-   Ensure proper terminal cleanup on exit with Drop trait
-   Handle terminal resize events gracefully
-   Support multiple color themes for accessibility
-   Batch terminal operations for better performance

## Challenge Design Principles

### Educational Focus

-   Each challenge should teach a real cybersecurity concept
-   Provide progressive hints that guide learning without giving away answers
-   Include educational descriptions explaining the vulnerability or technique
-   Balance authenticity with accessibility for beginners

### Difficulty Progression

-   Level 0: Basic encoding and simple concepts
-   Level 1: Intermediate cryptography and basic exploits
-   Level 2: Web and mobile security
-   Level 3+: Advanced topics like binary exploitation and reverse engineering

### Challenge Structure

When creating new challenges:

```rust
Challenge {
    id: unique_identifier,
    level: 0-4,
    title: "Descriptive Title",
    category: ChallengeCategory,
    description: "Educational context and scenario",
    prompt: "What the user sees",
    solution: "Correct answer",
    hints: vec!["Progressive", "Helpful", "Hints"],
    sanity_cost: 5-15, // Higher for harder challenges
    xp_reward: 50-200, // Scaled to difficulty
}
```

## Horror Theme Integration

### Narrative Consistency

-   Maintain the dark, mysterious atmosphere throughout
-   Use glitch effects sparingly for impact
-   Keep the twist ending in mind (player becomes the ghost)
-   Balance horror with gameplay - don't overwhelm the player

### Sanity Mechanic

-   Starting sanity: 100
-   Each challenge costs 5-15 sanity
-   Game over at 0 sanity
-   Use sanity as a pacing mechanism
-   Display warnings as sanity drops

### Visual Effects

-   Use ASCII art for atmosphere
-   Apply color coding: Red for danger, Green for success, Yellow for warnings
-   Implement glitch effects for critical moments
-   Keep effects terminal-friendly (ANSI compatible)

## State Management

### Save System

-   Auto-save after each completed challenge
-   Save location: `save_game.json` in project directory
-   Serialize: player stats, completed challenges, current level, sanity, xp
-   Handle corrupted save files gracefully
-   Consider adding backup saves for important milestones

### Player Statistics

Track and persist:

-   Challenges completed (by ID)
-   Current level and XP
-   Sanity level
-   Hints used
-   Attempts per challenge
-   Total play time

## Testing Infrastructure

### Comprehensive Testing Strategy

The project now includes a complete testing ecosystem with multiple testing approaches:

#### Unit and Integration Testing

-   **Unit tests**: Challenge validation, state management, UI components
-   **Integration tests**: End-to-end gameplay, save/load, cross-platform compatibility
-   **Property-based tests**: Input fuzzing, state serialization round-trips
-   **Color theme tests**: Theme switching, accessibility, visual consistency

#### Advanced Testing Tools

-   **Benchmarking**: Performance regression testing with Criterion
-   **Mutation testing**: Test quality validation with cargo-mutants
-   **Fuzzing**: Edge case discovery with cargo-fuzz
-   **Coverage tracking**: Codecov integration with 85%+ target coverage

#### Testing Scripts and Automation

Use the provided scripts for consistent testing:

```bash
# Quick development validation
scripts/quick-check.ps1

# Comprehensive test suite
scripts/test-verbose.ps1
scripts/test-coverage.ps1
scripts/test-watch.ps1

# Advanced testing (when needed)
scripts/run-benchmarks.ps1
scripts/run-mutation-tests.ps1 -Quick
scripts/run-fuzz.ps1 -Seconds 60
```

### Manual Testing Checklist

-   All challenges have valid solutions and educational value
-   Hints are progressive and helpful without spoiling
-   Terminal UI renders correctly on all target platforms (Windows/macOS/Linux)
-   Color themes work properly and provide accessibility benefits
-   Save/load works across sessions and handles corruption gracefully
-   Edge cases in user input are handled safely
-   Sanity mechanic creates appropriate game pacing
-   Tutorial system guides new users effectively
-   Tab completion works for all commands
-   Performance remains responsive during extended play

## Performance Guidelines

### Optimization Priorities

1. Terminal rendering (minimize unnecessary redraws)
2. Challenge validation (should be instant)
3. Save/load operations (keep files small)
4. Memory usage (avoid allocations in hot paths)

### Scalability

-   Challenge system should support 50+ challenges easily
-   Save file should remain under 100KB
-   Startup time should be under 1 second
-   UI should be responsive on modest hardware

## Accessibility Considerations

### Color Blindness

-   Don't rely solely on color to convey information
-   Use symbols and text alongside colors
-   Consider adding a color-blind mode in future

### Difficulty Accessibility

-   Always provide hints
-   Allow challenge skipping
-   Save progress frequently
-   Clear instructions for all challenges

## Security Notes

### Input Validation

-   Sanitize all user input
-   Prevent directory traversal in file-related challenges
-   Validate challenge solutions safely
-   Don't execute arbitrary code from user input

### Educational Security

-   Teach responsible disclosure
-   Include ethical hacking notes
-   Don't provide actual exploit code that could be misused
-   Focus on concepts over specific vulnerable software

## Development Workflow

### Development Scripts and Tools

The project includes comprehensive development automation:

#### VS Code Integration

-   **25+ VS Code tasks** for common operations
-   **Build tasks**: `cargo build`, `cargo build --release`, `cargo check`
-   **Testing tasks**: `cargo test`, `Test: Watch Mode`, `Test: Coverage Report`
-   **Quality tasks**: `cargo fmt`, `cargo clippy`, `Format and Check`
-   **Advanced tasks**: `Benchmark: Run All`, `Advanced: Mutation Testing`

#### Cross-Platform Scripts

All scripts have both PowerShell (.ps1) and Bash (.sh) versions:

-   `scripts/quick-check.*` - Fast pre-commit validation
-   `scripts/build-release.*` - Optimized production builds
-   `scripts/test-coverage.*` - Generate coverage reports
-   `scripts/clean-all.*` - Deep clean of build artifacts

#### CI/CD Pipeline

-   **GitHub Actions** with comprehensive checks
-   **Multi-platform testing** (Windows, macOS, Linux)
-   **Automated dependency updates** via Dependabot
-   **Security scanning** with CodeQL and cargo audit
-   **Coverage reporting** to Codecov with PR comments

### Common Development Patterns

#### Adding a New Challenge

1. Define the challenge struct in `challenges.rs`
2. Add to the appropriate level in `get_challenges()`
3. Implement validation logic with comprehensive error handling
4. Create progressive hints that teach concepts
5. Write unit tests for validation logic
6. Test with various inputs including edge cases
7. Add integration test for end-to-end challenge flow
8. Update documentation and walkthrough
9. Run `scripts/quick-check.ps1` before committing

#### Adding New Visual Effects

1. Use the theming system from `ui.rs` instead of hardcoded colors
2. Create rendering function with proper error handling
3. Test on multiple terminals and color themes
4. Ensure accessibility with high contrast themes
5. Document any new ANSI codes used
6. Ensure proper cleanup on exit
7. Add unit tests for visual components

#### Extending Color Themes

1. Add new theme to `ColorTheme` enum in `ui.rs`
2. Implement theme colors following accessibility guidelines
3. Add theme to selection menu in `get_color_theme_menu()`
4. Test theme with all UI components
5. Add unit test for new theme
6. Update documentation

#### Extending the Narrative

1. Maintain consistency with existing horror story
2. Add new narrative elements to `narrative.rs`
3. Consider sanity level for appropriate timing
4. Test message flow in actual gameplay
5. Keep horror elements atmospheric, not distracting
6. Integrate with challenge educational objectives

## Future Feature Considerations

When implementing new features, consider:

-   Backward compatibility with existing save files
-   Impact on game balance and pacing
-   Terminal compatibility across platforms
-   Educational value vs entertainment value
-   Consistency with horror theme
-   Extensibility for future enhancements

## Documentation Standards

### Code Comments

-   Explain _why_, not _what_
-   Document complex algorithms
-   Note any workarounds or hacks
-   Reference external resources for CTF techniques

### User-Facing Documentation

-   Keep README.md updated
-   Document new challenges in walkthrough
-   Update security notes as needed
-   Maintain changelog for versions

## Contribution Guidelines

### Before Adding Features

-   Check if it fits the horror theme
-   Ensure it's educational
-   Verify terminal compatibility
-   Test on Windows and Linux
-   Update relevant documentation

### Code Review Focus

-   Rust best practices
-   Error handling completeness
-   Terminal UI consistency
-   Challenge quality and hints
-   Save file compatibility

## Common Pitfalls to Avoid

1. **Breaking Save Compatibility**: Always test with existing save files
2. **Terminal State Corruption**: Always clean up terminal state on exit
3. **Platform-Specific Code**: Test on both Windows and Unix-like systems
4. **Spoilers in Hints**: Keep hints progressive and educational
5. **Color Overuse**: Too many colors reduce impact
6. **Sanity Balance**: Don't make game too short or too long
7. **Input Validation**: Always validate and sanitize user input
8. **Error Messages**: Make them helpful, not just technical

## Quick Reference

### Adding a Challenge

```rust
// In challenges.rs
Challenge {
    id: "new_challenge",
    level: 1,
    title: "Challenge Title",
    category: ChallengeCategory::Cryptography,
    description: "Educational context...",
    prompt: "User-facing prompt...",
    solution: "correct_answer",
    hints: vec!["Hint 1", "Hint 2", "Hint 3"],
    sanity_cost: 10,
    xp_reward: 100,
}
```

### Displaying Colored Text

```rust
// In any module
use crate::ui::{print_colored, Color};
print_colored("Warning!", Color::Red)?;
```

### Accessing Game State

```rust
// In game.rs
state.add_completed_challenge("challenge_id");
state.add_xp(100);
state.decrease_sanity(10);
state.check_level_up();
```

## Resources

### Rust Learning

-   [The Rust Book](https://doc.rust-lang.org/book/)
-   [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
-   [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### CTF Resources

-   [CTF101](https://ctf101.org/)
-   [OverTheWire](https://overthewire.org/)
-   [picoCTF](https://picoctf.org/)

### Terminal Programming

-   [crossterm documentation](https://docs.rs/crossterm/)
-   [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)

---

_This file helps GitHub Copilot provide better suggestions when working on The Hack: Ghost Protocol project._
