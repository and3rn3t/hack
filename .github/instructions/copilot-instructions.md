# GitHub Copilot Instructions for The Hack: Ghost Protocol

## Project Overview
This is a horror-themed hacking simulator and CTF challenge game built in Rust. It combines educational cybersecurity concepts with an immersive horror narrative and sanity mechanic. The game is designed for both beginners and enthusiasts, featuring progressive difficulty and terminal-based UI.

## Core Architecture

### Module Structure
- `main.rs` - Entry point with terminal setup and initialization
- `game.rs` - Main game loop, menu system, and user interaction
- `challenges.rs` - Challenge definitions, validation, and hints (11 challenges across 5 levels)
- `narrative.rs` - Horror story elements, atmospheric text, and glitch effects
- `state.rs` - Game state management, save/load system, player statistics
- `ui.rs` - Terminal UI rendering, colors, ASCII art, and visual effects

### Key Dependencies
- `crossterm` - Cross-platform terminal manipulation
- `serde`/`serde_json` - State serialization and persistence
- `rand` - Random elements for horror effects
- `chrono` - Timestamps and game tracking

## Coding Standards & Best Practices

### Rust Idioms
- Use Rust's ownership system and borrowing rules correctly
- Prefer `Result<T, E>` for error handling over panics
- Use pattern matching for control flow
- Implement `Display` and `Debug` traits where appropriate
- Leverage the type system for compile-time safety

### Error Handling
- Use `crossterm::Result` for terminal operations
- Use custom error types or `anyhow` for complex error scenarios
- Always handle user input errors gracefully
- Provide helpful error messages to users

### Code Style
- Follow Rust standard naming conventions (snake_case for functions, PascalCase for types)
- Keep functions focused and single-purpose
- Use descriptive variable names
- Add doc comments (`///`) for public functions and structs
- Keep line length under 100 characters where reasonable

### Terminal UI Guidelines
- Always use color constants from `ui.rs` for consistency
- Test terminal output on both Windows and Unix-like systems
- Use `crossterm::execute!` and `queue!` appropriately
- Ensure proper terminal cleanup on exit
- Handle terminal resize events gracefully

## Challenge Design Principles

### Educational Focus
- Each challenge should teach a real cybersecurity concept
- Provide progressive hints that guide learning without giving away answers
- Include educational descriptions explaining the vulnerability or technique
- Balance authenticity with accessibility for beginners

### Difficulty Progression
- Level 0: Basic encoding and simple concepts
- Level 1: Intermediate cryptography and basic exploits
- Level 2: Web and mobile security
- Level 3+: Advanced topics like binary exploitation and reverse engineering

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
- Maintain the dark, mysterious atmosphere throughout
- Use glitch effects sparingly for impact
- Keep the twist ending in mind (player becomes the ghost)
- Balance horror with gameplay - don't overwhelm the player

### Sanity Mechanic
- Starting sanity: 100
- Each challenge costs 5-15 sanity
- Game over at 0 sanity
- Use sanity as a pacing mechanism
- Display warnings as sanity drops

### Visual Effects
- Use ASCII art for atmosphere
- Apply color coding: Red for danger, Green for success, Yellow for warnings
- Implement glitch effects for critical moments
- Keep effects terminal-friendly (ANSI compatible)

## State Management

### Save System
- Auto-save after each completed challenge
- Save location: `save_game.json` in project directory
- Serialize: player stats, completed challenges, current level, sanity, xp
- Handle corrupted save files gracefully
- Consider adding backup saves for important milestones

### Player Statistics
Track and persist:
- Challenges completed (by ID)
- Current level and XP
- Sanity level
- Hints used
- Attempts per challenge
- Total play time

## Testing Considerations

### Manual Testing Checklist
- All challenges have valid solutions
- Hints are helpful without spoiling
- Terminal UI renders correctly on target platforms
- Save/load works across sessions
- Edge cases in user input are handled
- Sanity mechanic works as intended
- Color coding is consistent and accessible

### Future Automated Testing
- Unit tests for challenge validation logic
- Integration tests for save/load system
- Property-based tests for encoding/decoding challenges
- Mock terminal tests for UI components

## Performance Guidelines

### Optimization Priorities
1. Terminal rendering (minimize unnecessary redraws)
2. Challenge validation (should be instant)
3. Save/load operations (keep files small)
4. Memory usage (avoid allocations in hot paths)

### Scalability
- Challenge system should support 50+ challenges easily
- Save file should remain under 100KB
- Startup time should be under 1 second
- UI should be responsive on modest hardware

## Accessibility Considerations

### Color Blindness
- Don't rely solely on color to convey information
- Use symbols and text alongside colors
- Consider adding a color-blind mode in future

### Difficulty Accessibility
- Always provide hints
- Allow challenge skipping
- Save progress frequently
- Clear instructions for all challenges

## Security Notes

### Input Validation
- Sanitize all user input
- Prevent directory traversal in file-related challenges
- Validate challenge solutions safely
- Don't execute arbitrary code from user input

### Educational Security
- Teach responsible disclosure
- Include ethical hacking notes
- Don't provide actual exploit code that could be misused
- Focus on concepts over specific vulnerable software

## Common Patterns

### Adding a New Challenge
1. Define the challenge struct in `challenges.rs`
2. Add to the appropriate level in `get_challenges()`
3. Implement validation logic
4. Create progressive hints
5. Test with various inputs
6. Update documentation

### Adding New Visual Effects
1. Define color constants in `ui.rs`
2. Create rendering function
3. Test on multiple terminals
4. Document ANSI codes used
5. Ensure cleanup on exit

### Extending the Narrative
1. Maintain consistency with existing story
2. Add to `narrative.rs`
3. Consider sanity level for timing
4. Test message flow in gameplay
5. Keep the horror theme consistent

## Future Feature Considerations

When implementing new features, consider:
- Backward compatibility with existing save files
- Impact on game balance and pacing
- Terminal compatibility across platforms
- Educational value vs entertainment value
- Consistency with horror theme
- Extensibility for future enhancements

## Documentation Standards

### Code Comments
- Explain *why*, not *what*
- Document complex algorithms
- Note any workarounds or hacks
- Reference external resources for CTF techniques

### User-Facing Documentation
- Keep README.md updated
- Document new challenges in walkthrough
- Update security notes as needed
- Maintain changelog for versions

## Contribution Guidelines

### Before Adding Features
- Check if it fits the horror theme
- Ensure it's educational
- Verify terminal compatibility
- Test on Windows and Linux
- Update relevant documentation

### Code Review Focus
- Rust best practices
- Error handling completeness
- Terminal UI consistency
- Challenge quality and hints
- Save file compatibility

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
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### CTF Resources
- [CTF101](https://ctf101.org/)
- [OverTheWire](https://overthewire.org/)
- [picoCTF](https://picoctf.org/)

### Terminal Programming
- [crossterm documentation](https://docs.rs/crossterm/)
- [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)

---

*This file helps GitHub Copilot provide better suggestions when working on The Hack: Ghost Protocol project.*
