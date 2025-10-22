# Command History Feature

## Overview

The Hack: Ghost Protocol now includes full command history support with arrow key navigation, making the terminal experience more user-friendly and efficient.

## Features

### Arrow Key Navigation

-   **↑ (Up Arrow)**: Navigate backward through command history
-   **↓ (Down Arrow)**: Navigate forward through command history
-   Automatically cycles through your last 50 commands
-   Duplicates of the last command are not saved (prevents redundant history)

### Line Editing

-   **← (Left Arrow)**: Move cursor left within current input
-   **→ (Right Arrow)**: Move cursor right within current input
-   **Home**: Jump to beginning of input line
-   **End**: Jump to end of input line
-   **Backspace**: Delete character before cursor
-   **Delete**: Delete character at cursor
-   **Type anywhere**: Insert characters at cursor position

### History Management

-   **Capacity**: Stores up to 50 most recent commands
-   **Deduplication**: Consecutive identical commands are not duplicated
-   **Session-based**: History is maintained during your game session
-   **Privacy**: History is cleared when the game exits

## Technical Implementation

### Architecture

```
read_input() -> read_input_with_history() -> read_line_with_history()
```

1. **`read_input(prompt: &str)`**: Public API, enables history by default
2. **`read_input_with_history(prompt, save_to_history)`**: Controls whether input is saved
3. **`read_line_with_history()`**: Core implementation using crossterm events

### Key Components

#### Event Handling

Uses `crossterm::event` system to capture keyboard events in raw mode:

-   Processes each key press individually
-   Maintains cursor position and display
-   Handles special keys (arrows, home, end, etc.)

#### History Buffer

```rust
static mut COMMAND_HISTORY: Vec<String> = Vec::new();
const MAX_HISTORY_SIZE: usize = 50;
```

#### Helper Functions

-   `clear_command_history()`: Clears all history (useful for testing)
-   `get_history_size()`: Returns current number of commands in history

### Code Flow

```
User presses key
    ↓
crossterm::event::read() captures it
    ↓
Match on KeyCode
    ↓
Handle action (insert char, move cursor, navigate history)
    ↓
Update display
    ↓
Repeat until Enter is pressed
```

## Usage Examples

### Basic Usage

```rust
// Simple input with history
let command = ui::read_input("> Enter command: ")?;

// Input without saving to history (e.g., passwords)
let password = ui::read_input_with_history("Password: ", false)?;
```

### Typical User Workflow

1. User types: `stats`
2. Presses Enter
3. Later types: `sa` and presses ↑
4. Previous command `stats` appears
5. User presses Enter to execute again

### Managing History in Tests

```rust
#[test]
fn test_something() {
    ui::clear_command_history(); // Start with clean slate

    // Your test code here

    assert_eq!(ui::get_history_size(), expected_count);
}
```

## Benefits

### For Players

1. **Efficiency**: Quickly re-execute previous commands
2. **Convenience**: Fix typos without retyping entire command
3. **Exploration**: Navigate through command options easily
4. **Learning**: Review what you've tried before

### For Development

1. **Testing**: Easily repeat test commands
2. **Debugging**: Reproduce issues with command sequences
3. **User Experience**: Modern terminal behavior expected by users

## Limitations & Future Enhancements

### Current Limitations

-   History is session-only (not persisted to disk)
-   No history search (Ctrl+R)
-   No tab completion yet
-   Fixed max size of 50 commands

### Planned Enhancements

-   [ ] Persistent history across sessions
-   [ ] Reverse search (Ctrl+R)
-   [ ] Tab completion for commands
-   [ ] Configurable history size
-   [ ] History statistics and analytics
-   [ ] Smart history (frequency-based suggestions)

## Testing

### Unit Tests

Located in `src/ui.rs`:

```rust
#[test]
fn test_command_history_initialization()
#[test]
fn test_command_history_max_size()
#[test]
fn test_clear_command_history()
```

### Manual Testing

1. Run the game: `cargo run`
2. Enter several commands (e.g., `stats`, `1`, `hint`, `quit`)
3. Press ↑ to verify history navigation
4. Edit a recalled command and verify cursor movement
5. Test Home/End keys
6. Test Backspace/Delete at various positions

## Implementation Details

### Terminal Mode Management

The implementation uses crossterm's raw mode for direct key capture:

```rust
enable_raw_mode()?;
let result = read_line_with_history();
disable_raw_mode()?;
```

**Important**: Always ensure `disable_raw_mode()` is called, even on errors, to prevent terminal corruption.

### Display Updates

When user navigates history or edits:

1. Move cursor to column 0
2. Clear current line
3. Print updated text
4. Reposition cursor

This ensures clean, flicker-free updates.

### Safety Considerations

Uses `unsafe` block for static mutable `COMMAND_HISTORY`:

-   Justified because: Single-threaded game with no concurrent access
-   Alternative: Could use `RefCell` or `Mutex` for thread-safety
-   Current approach: Simple, fast, appropriate for use case

## Platform Compatibility

### Tested Platforms

-   ✅ Windows (PowerShell, Windows Terminal)
-   ✅ Linux (GNOME Terminal, Konsole)
-   ✅ macOS (Terminal.app, iTerm2)

### Known Issues

None currently. All arrow key functionality works cross-platform thanks to crossterm.

## Performance

-   **Memory**: ~1KB for 50 commands (negligible)
-   **CPU**: Event processing is instant (microseconds)
-   **Latency**: No perceptible delay in typing

## Contributing

To extend command history functionality:

1. Maintain crossterm event-driven architecture
2. Keep history management simple and fast
3. Add tests for new features
4. Update this documentation
5. Ensure cross-platform compatibility

## References

-   [crossterm documentation](https://docs.rs/crossterm/)
-   [Terminal input handling best practices](https://viewsourcecode.org/snaptoken/kilo/)
-   [GNU Readline](https://tiswww.case.edu/php/chet/readline/rltop.html) (inspiration)

---

_Last updated: October 22, 2025_
