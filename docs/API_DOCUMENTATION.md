# API Documentation for Extensibility

**Target Audience**: Developers extending and integrating with The Hack: Ghost Protocol
**Last Updated**: October 24, 2025

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Data Structures](#core-data-structures)
3. [Challenge System API](#challenge-system-api)
4. [State Management API](#state-management-api)
5. [UI System API](#ui-system-api)
6. [Narrative System API](#narrative-system-api)
7. [Extension Points](#extension-points)
8. [Plugin Architecture](#plugin-architecture)
9. [Integration Examples](#integration-examples)
10. [Testing APIs](#testing-apis)
11. [Performance Considerations](#performance-considerations)
12. [Future API Changes](#future-api-changes)

---

## Architecture Overview

The Hack: Ghost Protocol follows a modular architecture designed for extensibility while maintaining the core horror-themed experience.

### Module Dependencies

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    main     │───▶│    game     │───▶│    ui       │
└─────────────┘    └─────────────┘    └─────────────┘
                            │                   │
                            ▼                   ▼
                   ┌─────────────┐    ┌─────────────┐
                   │  challenges │    │  narrative  │
                   └─────────────┘    └─────────────┘
                            │                   │
                            ▼                   │
                   ┌─────────────┐              │
                   │    state    │◀─────────────┘
                   └─────────────┘
```

### Key Principles

1. **Separation of Concerns**: Each module has a clear responsibility
2. **Loose Coupling**: Modules communicate through well-defined interfaces
3. **Extensibility**: New challenges, themes, and features can be added
4. **Testability**: Each component can be tested in isolation
5. **Performance**: Lazy loading and caching where appropriate

---

## Core Data Structures

### Challenge Structure

```rust
/// Represents a single CTF challenge with all associated metadata and logic
#[derive(Debug, Clone)]
pub struct Challenge {
    /// Unique identifier for this challenge (snake_case)
    pub id: &'static str,

    /// Human-readable title displayed to players
    pub title: &'static str,

    /// Educational context and horror narrative setup
    pub description: &'static str,

    /// The actual challenge prompt shown to the player
    pub prompt: &'static str,

    /// Difficulty level (0-4, where 0 is beginner)
    pub level: u32,

    /// Experience points awarded for completion
    pub xp_reward: u32,

    /// Sanity points lost when attempting
    pub sanity_cost: u32,

    /// Validation function that checks if an answer is correct
    pub check_answer: fn(&str) -> bool,

    /// Progressive hints to help players learn
    pub hints: Vec<String>,

    /// Category classification for organization
    pub category: ChallengeCategory,
}

/// Categories for organizing challenges by type
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeCategory {
    /// Basic encoding/decoding challenges
    Encoding,
    /// Cryptography and cipher challenges
    Cryptography,
    /// Web application security
    WebSecurity,
    /// Mobile application security
    MobileSecurity,
    /// Network analysis and forensics
    NetworkSecurity,
    /// Binary exploitation concepts
    BinaryExploitation,
    /// Reverse engineering challenges
    ReverseEngineering,
    /// Steganography and hidden data
    Steganography,
    /// Open Source Intelligence gathering
    OSINT,
    /// Malware analysis concepts
    MalwareAnalysis,
    /// General forensics challenges
    Forensics,
}
```

### Game State Structure

```rust
/// Complete game state including player progress and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Player's chosen name
    pub player_name: String,

    /// Current difficulty level (0-4)
    pub current_level: u32,

    /// Total experience points earned
    pub experience: u32,

    /// Current sanity level (0-100)
    pub sanity: u32,

    /// Set of completed challenge IDs
    pub completed_challenges: HashSet<String>,

    /// Set of discovered secrets and easter eggs
    pub discovered_secrets: HashSet<String>,

    /// Detailed statistics for analysis
    pub stats: PlayerStats,

    /// Timestamps for progress tracking
    pub timestamps: GameTimestamps,
}

/// Detailed player statistics for analytics and progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Total number of challenge attempts
    pub total_attempts: u32,

    /// Number of hints used across all challenges
    pub hints_used: u32,

    /// Challenges skipped
    pub challenges_skipped: u32,

    /// Per-category completion rates
    pub category_progress: HashMap<String, CategoryProgress>,

    /// Time spent in game (seconds)
    pub total_playtime_seconds: u64,
}

/// Progress tracking for specific challenge categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryProgress {
    pub completed: u32,
    pub attempted: u32,
    pub total_available: u32,
}

/// Timestamp tracking for various game events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTimestamps {
    pub game_started: DateTime<Utc>,
    pub last_played: DateTime<Utc>,
    pub first_challenge_completed: Option<DateTime<Utc>>,
    pub last_level_up: Option<DateTime<Utc>>,
}
```

### Color Theme System

```rust
/// Customizable color theme for terminal UI
#[derive(Debug, Clone, PartialEq)]
pub struct ColorTheme {
    pub name: String,
    pub primary: Color,      // Main text, titles
    pub secondary: Color,    // Subtitles, descriptions
    pub accent: Color,       // Highlights, selections
    pub success: Color,      // Success messages
    pub warning: Color,      // Warnings, hints
    pub error: Color,        // Errors, dangerous actions
    pub info: Color,         // Information, help text
    pub muted: Color,        // Less important text
    pub border: Color,       // Borders, separators
    pub background: Color,   // Background color
}
```

---

## Challenge System API

### Core Functions

#### Challenge Retrieval

```rust
/// Get all challenges for a specific difficulty level
pub fn get_challenges_for_level(level: u32) -> &'static Vec<Challenge> {
    // Returns cached challenges for the specified level
}

/// Get all challenges across all levels (expensive, use sparingly)
pub fn get_all_challenges() -> &'static Vec<Challenge> {
    // Returns all challenges with lazy loading and caching
}

/// Find a specific challenge by ID
pub fn find_challenge_by_id(id: &str) -> Option<&'static Challenge> {
    // Efficient lookup of challenges by unique identifier
}

/// Get challenges filtered by category
pub fn get_challenges_by_category(category: ChallengeCategory) -> Vec<&'static Challenge> {
    // Returns all challenges matching the specified category
}
```

#### Challenge Validation

```rust
/// Validate a player's answer against a challenge
pub fn validate_challenge_answer(challenge_id: &str, answer: &str) -> ChallengeResult {
    // Comprehensive answer validation with detailed feedback
}

/// Result of challenge validation
#[derive(Debug)]
pub enum ChallengeResult {
    /// Answer is correct
    Correct,
    /// Answer is incorrect with optional feedback
    Incorrect(Option<String>),
    /// Challenge not found
    NotFound,
    /// Validation error (e.g., challenge function panicked)
    ValidationError(String),
}
```

#### Challenge Creation API

````rust
/// Builder pattern for creating new challenges
pub struct ChallengeBuilder {
    id: Option<&'static str>,
    title: Option<&'static str>,
    description: Option<&'static str>,
    prompt: Option<&'static str>,
    level: Option<u32>,
    xp_reward: Option<u32>,
    sanity_cost: Option<u32>,
    check_answer: Option<fn(&str) -> bool>,
    hints: Vec<String>,
    category: Option<ChallengeCategory>,
}

impl ChallengeBuilder {
    pub fn new() -> Self { /* ... */ }

    pub fn id(mut self, id: &'static str) -> Self { /* ... */ }

    pub fn title(mut self, title: &'static str) -> Self { /* ... */ }

    pub fn description(mut self, description: &'static str) -> Self { /* ... */ }

    pub fn prompt(mut self, prompt: &'static str) -> Self { /* ... */ }

    pub fn level(mut self, level: u32) -> Self { /* ... */ }

    pub fn rewards(mut self, xp: u32, sanity_cost: u32) -> Self { /* ... */ }

    pub fn validator<F>(mut self, validator: F) -> Self
    where F: Fn(&str) -> bool + 'static { /* ... */ }

    pub fn hint(mut self, hint: String) -> Self { /* ... */ }

    pub fn hints(mut self, hints: Vec<String>) -> Self { /* ... */ }

    pub fn category(mut self, category: ChallengeCategory) -> Self { /* ... */ }

    pub fn build(self) -> Result<Challenge, ChallengeBuilderError> { /* ... */ }
}

/// Example usage:
/// ```rust
/// let challenge = ChallengeBuilder::new()
///     .id("my_challenge")
///     .title("My Custom Challenge")
///     .description("A description...")
///     .prompt("What is the answer?")
///     .level(1)
///     .rewards(100, 10)
///     .validator(|answer| answer.to_lowercase() == "correct")
///     .hint("This is a hint".to_string())
///     .category(ChallengeCategory::Cryptography)
///     .build()?;
/// ```
````

#### Challenge Metadata API

```rust
/// Get challenge statistics and metadata
pub fn get_challenge_stats() -> ChallengeStats {
    // Returns comprehensive statistics about all challenges
}

#[derive(Debug)]
pub struct ChallengeStats {
    pub total_challenges: usize,
    pub challenges_by_level: HashMap<u32, usize>,
    pub challenges_by_category: HashMap<ChallengeCategory, usize>,
    pub total_xp_available: u32,
    pub average_difficulty: f32,
}

/// Validate challenge completeness and balance
pub fn validate_challenge_set() -> ValidationReport {
    // Analyzes all challenges for consistency, balance, and completeness
}

#[derive(Debug)]
pub struct ValidationReport {
    pub total_challenges: usize,
    pub issues: Vec<ValidationIssue>,
    pub balance_analysis: BalanceAnalysis,
    pub coverage_report: CoverageReport,
}

#[derive(Debug)]
pub enum ValidationIssue {
    DuplicateId { id: String },
    EmptyField { challenge_id: String, field: String },
    InvalidLevel { challenge_id: String, level: u32 },
    UnbalancedRewards { challenge_id: String, issue: String },
    MissingHints { challenge_id: String },
    ValidatorIssue { challenge_id: String, error: String },
}
```

---

## State Management API

### Core State Operations

```rust
/// Create a new game state with default values
pub fn create_new_state(player_name: String) -> GameState {
    // Initializes a fresh game state for a new player
}

/// Load game state from the default save file
pub fn load_state() -> Result<GameState, StateError> {
    // Loads persisted state with error handling
}

/// Save game state to the default save file
pub fn save_state(state: &GameState) -> Result<(), StateError> {
    // Persists current state with atomic write operations
}

/// Load game state from a specific file path
pub fn load_state_from_file<P: AsRef<Path>>(path: P) -> Result<GameState, StateError> {
    // Flexible loading from custom locations
}

/// Save game state to a specific file path
pub fn save_state_to_file<P: AsRef<Path>>(state: &GameState, path: P) -> Result<(), StateError> {
    // Flexible saving to custom locations
}
```

### State Modification API

```rust
impl GameState {
    /// Create a new game state
    pub fn new(player_name: String) -> Self { /* ... */ }

    /// Complete a challenge and update all related state
    pub fn complete_challenge(&mut self, challenge_id: &str, xp_reward: u32) {
        // Updates completed challenges, XP, level progression
    }

    /// Decrease sanity with bounds checking
    pub fn decrease_sanity(&mut self, amount: u32) {
        // Safe sanity modification with minimum bounds
    }

    /// Add experience points and check for level up
    pub fn add_experience(&mut self, xp: u32) -> bool {
        // Returns true if player leveled up
    }

    /// Check if player has completed a specific challenge
    pub fn has_completed(&self, challenge_id: &str) -> bool {
        // Efficient lookup in completed challenges set
    }

    /// Get current level based on experience points
    pub fn get_current_level(&self) -> u32 {
        // Calculates level from XP using progression formula
    }

    /// Get XP required for next level
    pub fn xp_to_next_level(&self) -> u32 {
        // Shows progress toward next level milestone
    }

    /// Add a discovered secret to the collection
    pub fn discover_secret(&mut self, secret_id: &str) {
        // Tracks easter eggs and hidden content discovery
    }

    /// Get completion percentage for a specific category
    pub fn get_category_completion(&self, category: ChallengeCategory) -> f32 {
        // Calculates completion rate for challenge categories
    }

    /// Update playtime statistics
    pub fn update_playtime(&mut self, additional_seconds: u64) {
        // Tracks total time spent in game
    }
}
```

### State Query API

```rust
impl GameState {
    /// Get available challenges for current level
    pub fn get_available_challenges(&self) -> Vec<&'static Challenge> {
        // Returns challenges appropriate for current progression
    }

    /// Get completion statistics
    pub fn get_completion_stats(&self) -> CompletionStats {
        // Comprehensive progress analysis
    }

    /// Get detailed progress report
    pub fn get_progress_report(&self) -> ProgressReport {
        // Formatted report suitable for display
    }

    /// Check if game is over (sanity <= 0)
    pub fn is_game_over(&self) -> bool {
        // Horror mechanic: lose when sanity depleted
    }

    /// Get estimated time to completion
    pub fn estimated_completion_time(&self) -> Duration {
        // Based on current progress and average challenge time
    }
}

#[derive(Debug)]
pub struct CompletionStats {
    pub total_challenges: usize,
    pub completed_challenges: usize,
    pub completion_percentage: f32,
    pub current_level: u32,
    pub max_level: u32,
    pub sanity_remaining: u32,
    pub categories: HashMap<ChallengeCategory, CategoryProgress>,
}

#[derive(Debug)]
pub struct ProgressReport {
    pub player_name: String,
    pub session_summary: String,
    pub achievements: Vec<String>,
    pub recommendations: Vec<String>,
    pub formatted_stats: String,
}
```

### State Serialization API

```rust
/// Custom serialization options for different use cases
pub mod serialization {
    use serde::{Serialize, Deserialize};

    /// Serialize state with compression
    pub fn serialize_compressed(state: &GameState) -> Result<Vec<u8>, SerializationError> {
        // Compressed binary format for efficient storage
    }

    /// Deserialize compressed state
    pub fn deserialize_compressed(data: &[u8]) -> Result<GameState, SerializationError> {
        // Decompress and validate state data
    }

    /// Export state for external analysis
    pub fn export_for_analysis(state: &GameState) -> AnalysisExport {
        // Structured data suitable for analytics
    }

    /// Import state from external format
    pub fn import_from_analysis(export: AnalysisExport) -> Result<GameState, ImportError> {
        // Convert analysis format back to game state
    }
}
```

---

## UI System API

### Core Rendering Functions

```rust
/// Terminal output and rendering functions
pub mod ui {
    use crossterm::style::Color;
    use std::io;

    /// Print text with specified color
    pub fn print_colored(text: &str, color: Color) -> io::Result<()> {
        // Efficient colored text output with proper cleanup
    }

    /// Print text using current theme colors
    pub fn print_themed(text: &str, theme_element: ThemeElement) -> io::Result<()> {
        // Uses active theme for consistent styling
    }

    /// Clear the terminal screen
    pub fn clear_screen() -> io::Result<()> {
        // Cross-platform screen clearing
    }

    /// Move cursor to specific position
    pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
        // Absolute cursor positioning
    }

    /// Get terminal size
    pub fn get_terminal_size() -> io::Result<(u16, u16)> {
        // Returns (width, height) of current terminal
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ThemeElement {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
    Muted,
    Border,
    Background,
}
```

### Advanced UI Components

```rust
/// High-level UI components for common patterns
pub mod components {
    /// Display a formatted challenge
    pub fn render_challenge(challenge: &Challenge) -> io::Result<()> {
        // Comprehensive challenge display with styling
    }

    /// Display player statistics
    pub fn render_stats(state: &GameState) -> io::Result<()> {
        // Formatted statistics display
    }

    /// Display a menu with numbered options
    pub fn render_menu(title: &str, options: &[String]) -> io::Result<()> {
        // Consistent menu formatting
    }

    /// Display ASCII art with optional effects
    pub fn render_ascii_art(art: &str, effects: AsciiEffects) -> io::Result<()> {
        // Horror-themed ASCII art with glitch effects
    }

    /// Display a progress bar
    pub fn render_progress_bar(
        current: u32,
        max: u32,
        width: u16,
        style: ProgressStyle
    ) -> io::Result<()> {
        // Customizable progress visualization
    }
}

#[derive(Debug, Clone)]
pub struct AsciiEffects {
    pub glitch: bool,
    pub flicker: bool,
    pub color_shift: bool,
    pub delay_ms: u64,
}

#[derive(Debug, Clone)]
pub enum ProgressStyle {
    Simple,
    Detailed,
    Horror, // Special effects for horror theme
}
```

### Theme Management API

```rust
/// Color theme management functions
pub mod themes {
    /// Get the currently active theme
    pub fn get_theme() -> ColorTheme {
        // Returns current theme (thread-safe)
    }

    /// Set a new active theme
    pub fn set_theme(theme: ColorTheme) -> io::Result<()> {
        // Updates global theme state
    }

    /// Get all available themes
    pub fn get_available_themes() -> Vec<ColorTheme> {
        // Returns predefined themes
    }

    /// Create a custom theme
    pub fn create_custom_theme(name: String, colors: ThemeColors) -> ColorTheme {
        // Theme factory function
    }

    /// Validate theme accessibility
    pub fn validate_theme_accessibility(theme: &ColorTheme) -> AccessibilityReport {
        // Checks contrast ratios and accessibility guidelines
    }
}

/// Convenience functions for theme-based coloring
pub fn theme_primary() -> Color { /* ... */ }
pub fn theme_secondary() -> Color { /* ... */ }
pub fn theme_accent() -> Color { /* ... */ }
pub fn theme_success() -> Color { /* ... */ }
pub fn theme_warning() -> Color { /* ... */ }
pub fn theme_error() -> Color { /* ... */ }
pub fn theme_info() -> Color { /* ... */ }
pub fn theme_muted() -> Color { /* ... */ }
pub fn theme_border() -> Color { /* ... */ }
pub fn theme_background() -> Color { /* ... */ }
```

### Input Handling API

```rust
/// Input processing and validation
pub mod input {
    use crossterm::event::{KeyEvent, KeyCode};

    /// Read a line of input with editing support
    pub fn read_line_with_history() -> io::Result<String> {
        // Full-featured input with arrow key navigation
    }

    /// Read a single key press
    pub fn read_key() -> io::Result<KeyEvent> {
        // Raw key event capture
    }

    /// Read input with timeout
    pub fn read_input_timeout(timeout_ms: u64) -> io::Result<Option<KeyEvent>> {
        // Non-blocking input with timeout
    }

    /// Validate menu selection
    pub fn validate_menu_choice(input: &str, max_options: usize) -> Option<usize> {
        // Parse and validate menu selections
    }
}
```

---

## Narrative System API

### Story Management

```rust
/// Narrative and atmospheric elements
pub mod narrative {
    /// Display the main story introduction
    pub fn show_introduction(player_name: &str) -> io::Result<()> {
        // Atmospheric game introduction with personalization
    }

    /// Display level transition story
    pub fn show_level_transition(from_level: u32, to_level: u32) -> io::Result<()> {
        // Story progression between difficulty levels
    }

    /// Display challenge completion narrative
    pub fn show_completion_message(
        challenge: &Challenge,
        sanity_remaining: u32
    ) -> io::Result<()> {
        // Contextual story elements after challenge completion
    }

    /// Display game over sequence
    pub fn show_game_over(state: &GameState) -> io::Result<()> {
        // Horror-themed ending when sanity reaches zero
    }

    /// Display victory sequence
    pub fn show_victory(state: &GameState) -> io::Result<()> {
        // Final narrative when all challenges completed
    }

    /// Get random atmospheric message
    pub fn get_atmospheric_message(context: AtmosphereContext) -> String {
        // Context-appropriate horror elements
    }
}

#[derive(Debug, Clone)]
pub enum AtmosphereContext {
    MenuIdle,
    LowSanity,
    ChallengeStart,
    ChallengeFailure,
    LevelUp,
    Discovery,
}
```

### Dynamic Content Generation

```rust
/// Content generation for replayability
pub mod generation {
    /// Generate contextual flavor text
    pub fn generate_challenge_flavor(challenge: &Challenge) -> String {
        // Dynamic atmospheric descriptions
    }

    /// Generate personalized messages
    pub fn generate_personal_message(state: &GameState, context: MessageContext) -> String {
        // Player-specific narrative elements
    }

    /// Generate hint with horror elements
    pub fn generate_horror_hint(base_hint: &str, sanity_level: u32) -> String {
        // Atmospheric hint delivery based on sanity
    }
}

#[derive(Debug, Clone)]
pub enum MessageContext {
    Welcome,
    Progress,
    Warning,
    Encouragement,
    Challenge,
}
```

---

## Extension Points

### Plugin Architecture Foundation

```rust
/// Plugin system for extensibility (Future API)
pub mod plugins {
    /// Plugin trait for challenge providers
    pub trait ChallengePlugin: Send + Sync {
        /// Get plugin metadata
        fn metadata(&self) -> PluginMetadata;

        /// Provide challenges for integration
        fn challenges(&self) -> Vec<Challenge>;

        /// Initialize plugin with game context
        fn initialize(&mut self, context: &GameContext) -> Result<(), PluginError>;

        /// Clean up plugin resources
        fn cleanup(&mut self);
    }

    /// Plugin trait for theme providers
    pub trait ThemePlugin: Send + Sync {
        fn metadata(&self) -> PluginMetadata;
        fn themes(&self) -> Vec<ColorTheme>;
        fn custom_ui_elements(&self) -> Option<CustomUI>;
    }

    /// Plugin trait for narrative extensions
    pub trait NarrativePlugin: Send + Sync {
        fn metadata(&self) -> PluginMetadata;
        fn story_elements(&self) -> Vec<StoryElement>;
        fn atmospheric_content(&self) -> Vec<AtmosphericContent>;
    }
}

#[derive(Debug)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub compatibility_version: String,
}
```

### Custom Validators

```rust
/// Framework for custom challenge validators
pub mod validators {
    /// Trait for advanced validation logic
    pub trait CustomValidator: Send + Sync {
        /// Validate answer with detailed feedback
        fn validate(&self, answer: &str) -> ValidationResult;

        /// Get hints for current attempt number
        fn get_contextual_hint(&self, attempt: u32) -> Option<String>;

        /// Check if answer is partially correct
        fn partial_credit(&self, answer: &str) -> PartialCreditResult;
    }

    /// Result with detailed validation information
    #[derive(Debug)]
    pub struct ValidationResult {
        pub correct: bool,
        pub feedback: Option<String>,
        pub confidence: f32,
        pub suggestions: Vec<String>,
    }

    /// Partial credit for learning-focused challenges
    #[derive(Debug)]
    pub struct PartialCreditResult {
        pub percentage: f32,
        pub recognized_elements: Vec<String>,
        pub missing_elements: Vec<String>,
    }
}
```

### Event System

```rust
/// Event-driven architecture for extensibility
pub mod events {
    /// Game events that plugins can listen to
    #[derive(Debug, Clone)]
    pub enum GameEvent {
        ChallengeStarted { challenge_id: String },
        ChallengeCompleted { challenge_id: String, attempts: u32 },
        ChallengeFailed { challenge_id: String, attempts: u32 },
        ChallengeSkipped { challenge_id: String },
        HintUsed { challenge_id: String, hint_number: u32 },
        LevelUp { from_level: u32, to_level: u32 },
        SanityChanged { old_sanity: u32, new_sanity: u32 },
        SecretDiscovered { secret_id: String },
        GameOver { reason: GameOverReason },
        GameCompleted { total_time: Duration },
    }

    #[derive(Debug, Clone)]
    pub enum GameOverReason {
        SanityDepleted,
        PlayerQuit,
        AllChallengesCompleted,
    }

    /// Event listener trait
    pub trait EventListener: Send + Sync {
        fn handle_event(&self, event: &GameEvent);
    }

    /// Event system functions
    pub fn register_listener(listener: Box<dyn EventListener>);
    pub fn emit_event(event: GameEvent);
    pub fn clear_listeners();
}
```

---

## Integration Examples

### Adding a New Challenge Category

```rust
// 1. Extend the category enum
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeCategory {
    // ... existing categories
    IoTSecurity,  // New category
}

// 2. Create challenges for the new category
fn iot_default_credentials() -> Challenge {
    Challenge {
        id: "iot_default_creds",
        title: "The Unsecured Device",
        description: "You discover an IoT device on the network with concerning security...",
        prompt: "What are the default credentials for this router model RT-AC68U?",
        level: 2,
        xp_reward: 125,
        sanity_cost: 12,
        check_answer: |answer| {
            let lower = answer.to_lowercase().replace([':', ' ', '/'], "");
            lower.contains("admin") && lower.contains("admin")
        },
        hints: vec![
            "IoT devices often ship with default credentials".to_string(),
            "Check manufacturer documentation or credential databases".to_string(),
            "Common format is username:password = admin:admin".to_string(),
        ],
        category: ChallengeCategory::IoTSecurity,
    }
}

// 3. Add to challenge lists
fn get_challenges_for_level(level: u32) -> &'static Vec<Challenge> {
    match level {
        2 => vec![
            // ... existing challenges
            iot_default_credentials(),
        ],
        // ...
    }
}
```

### Custom Theme Creation

```rust
// Create a custom accessibility theme
fn create_high_contrast_theme() -> ColorTheme {
    ColorTheme {
        name: "High Contrast".to_string(),
        primary: Color::White,
        secondary: Color::Yellow,
        accent: Color::Cyan,
        success: Color::Green,
        warning: Color::Yellow,
        error: Color::Red,
        info: Color::Cyan,
        muted: Color::Grey,
        border: Color::White,
        background: Color::Black,
    }
}

// Integration with theme system
pub fn setup_custom_themes() {
    let themes = vec![
        ColorTheme::horror(),
        ColorTheme::high_contrast(),
        create_high_contrast_theme(),
        // ... other themes
    ];

    // Make available to theme selector
}
```

### External Integration Example

```rust
// Example: Integrating with external learning management system
pub mod lms_integration {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct LearningRecord {
        pub student_id: String,
        pub challenge_id: String,
        pub completed: bool,
        pub attempts: u32,
        pub time_spent: Duration,
        pub xp_earned: u32,
    }

    /// Export progress for LMS integration
    pub fn export_learning_records(state: &GameState) -> Vec<LearningRecord> {
        state.completed_challenges
            .iter()
            .map(|challenge_id| {
                LearningRecord {
                    student_id: state.player_name.clone(),
                    challenge_id: challenge_id.clone(),
                    completed: true,
                    attempts: 1, // Would need to track this
                    time_spent: Duration::from_secs(600), // Estimated
                    xp_earned: find_challenge_by_id(challenge_id)
                        .map(|c| c.xp_reward)
                        .unwrap_or(0),
                }
            })
            .collect()
    }

    /// Import progress from LMS
    pub fn import_learning_records(
        records: Vec<LearningRecord>
    ) -> Result<GameState, ImportError> {
        // Convert LMS records back to game state
        let mut state = GameState::new(records[0].student_id.clone());

        for record in records {
            if record.completed {
                state.complete_challenge(&record.challenge_id, record.xp_earned);
            }
        }

        Ok(state)
    }
}
```

---

## Testing APIs

### Test Utilities

```rust
/// Testing utilities for challenge developers
pub mod testing {
    /// Create a test game state with specific configuration
    pub fn create_test_state() -> GameState {
        GameState::new("test_player".to_string())
    }

    /// Create an advanced test state with progress
    pub fn create_advanced_test_state() -> GameState {
        let mut state = create_test_state();
        state.experience = 500;
        state.current_level = 2;
        state.complete_challenge("welcome", 50);
        state.complete_challenge("basic_base64", 75);
        state
    }

    /// Test challenge validation comprehensively
    pub fn test_challenge_validation(challenge: &Challenge) -> ValidationTestResult {
        ValidationTestResult {
            basic_tests: run_basic_validation_tests(challenge),
            edge_case_tests: run_edge_case_tests(challenge),
            performance_tests: run_performance_tests(challenge),
            security_tests: run_security_tests(challenge),
        }
    }

    /// Temporary save file for testing
    pub struct TempSaveFile {
        path: PathBuf,
    }

    impl TempSaveFile {
        pub fn new(filename: &str) -> Self {
            // Creates temporary file that's automatically cleaned up
        }

        pub fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempSaveFile {
        fn drop(&mut self) {
            // Automatic cleanup
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[derive(Debug)]
pub struct ValidationTestResult {
    pub basic_tests: TestResults,
    pub edge_case_tests: TestResults,
    pub performance_tests: PerformanceResults,
    pub security_tests: SecurityResults,
}
```

### Property Testing Integration

```rust
/// Property-based testing support
pub mod property_testing {
    use proptest::prelude::*;

    /// Generate random challenge inputs for testing
    pub fn challenge_input_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            // Normal text inputs
            "[a-zA-Z0-9 ]{1,50}",
            // Special characters
            "[!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?`~]{1,20}",
            // Unicode and emojis
            "[\\u{0000}-\\u{FFFF}]{1,10}",
            // Very long inputs
            prop::collection::vec(any::<char>(), 100..1000)
                .prop_map(|chars| chars.into_iter().collect::<String>()),
        ]
    }

    /// Test that challenge validators never panic
    pub fn test_validator_robustness(validator: fn(&str) -> bool) {
        proptest!(|(input in challenge_input_strategy())| {
            // Should never panic regardless of input
            let _ = validator(&input);
        });
    }
}
```

---

## Performance Considerations

### Caching and Optimization

```rust
/// Performance optimization APIs
pub mod performance {
    use std::sync::OnceLock;

    /// Challenge cache for fast retrieval
    static CHALLENGE_CACHE: OnceLock<Vec<Challenge>> = OnceLock::new();

    /// Initialize challenge cache (called once at startup)
    pub fn init_challenge_cache() {
        CHALLENGE_CACHE.get_or_init(|| {
            // Expensive initialization done once
            compute_all_challenges()
        });
    }

    /// Performance monitoring for validators
    pub struct PerformanceTimer {
        start: std::time::Instant,
        operation: String,
    }

    impl PerformanceTimer {
        pub fn new(operation: String) -> Self {
            Self {
                start: std::time::Instant::now(),
                operation,
            }
        }
    }

    impl Drop for PerformanceTimer {
        fn drop(&mut self) {
            let duration = self.start.elapsed();
            if duration > std::time::Duration::from_millis(100) {
                eprintln!("Slow operation detected: {} took {:?}", self.operation, duration);
            }
        }
    }

    /// Macro for easy performance monitoring
    macro_rules! timed {
        ($operation:expr, $code:block) => {{
            let _timer = PerformanceTimer::new($operation.to_string());
            $code
        }};
    }
}
```

### Memory Management

```rust
/// Memory-efficient operations
pub mod memory {
    /// Estimate memory usage of game state
    pub fn estimate_state_size(state: &GameState) -> usize {
        // Calculate approximate memory usage
        std::mem::size_of_val(state)
            + state.completed_challenges.capacity() * std::mem::size_of::<String>()
            + state.discovered_secrets.capacity() * std::mem::size_of::<String>()
            // ... other dynamic allocations
    }

    /// Optimize state for memory efficiency
    pub fn optimize_state_memory(state: &mut GameState) {
        // Shrink collections to fit
        state.completed_challenges.shrink_to_fit();
        state.discovered_secrets.shrink_to_fit();
    }
}
```

---

## Future API Changes

### Planned Additions (Version 2.0)

```rust
/// Future APIs (subject to change)
pub mod future {
    /// Multiplayer support structures (planned)
    #[derive(Debug)]
    pub struct MultiplayerSession {
        pub session_id: String,
        pub players: Vec<Player>,
        pub shared_challenges: Vec<Challenge>,
        pub competitive_mode: bool,
    }

    /// Real-time collaboration (planned)
    pub trait CollaborativeChallenge {
        fn supports_collaboration(&self) -> bool;
        fn merge_solutions(&self, solutions: Vec<&str>) -> CombinedSolution;
    }

    /// AI-powered hint system (planned)
    pub trait AdaptiveHints {
        fn generate_hint(&self, attempt_count: u32, previous_answers: &[String]) -> String;
        fn analyze_common_mistakes(&self, wrong_answers: &[String]) -> MistakeAnalysis;
    }
}
```

### Deprecation Policy

The API follows semantic versioning:

-   **Patch versions** (x.x.X): Bug fixes, no API changes
-   **Minor versions** (x.X.x): New features, backward compatible
-   **Major versions** (X.x.x): Breaking changes, migration guide provided

### Migration Guides

When breaking changes occur, detailed migration guides will be provided showing:

1. **What Changed**: Specific API modifications
2. **Why**: Rationale for the breaking change
3. **How to Update**: Step-by-step migration instructions
4. **Timeline**: Deprecation schedule and support timeline

---

## Error Handling

### Error Types

```rust
/// Comprehensive error handling for all APIs
#[derive(Debug, thiserror::Error)]
pub enum HackError {
    #[error("State error: {0}")]
    State(#[from] StateError),

    #[error("Challenge error: {0}")]
    Challenge(#[from] ChallengeError),

    #[error("UI error: {0}")]
    Ui(#[from] UiError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Failed to load state from {path}: {source}")]
    LoadFailed { path: String, source: std::io::Error },

    #[error("Failed to save state to {path}: {source}")]
    SaveFailed { path: String, source: std::io::Error },

    #[error("Invalid state format: {reason}")]
    InvalidFormat { reason: String },

    #[error("State migration required from version {from} to {to}")]
    MigrationRequired { from: String, to: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ChallengeError {
    #[error("Challenge '{id}' not found")]
    NotFound { id: String },

    #[error("Validation failed for challenge '{id}': {reason}")]
    ValidationFailed { id: String, reason: String },

    #[error("Challenge '{id}' has invalid configuration: {issue}")]
    InvalidConfiguration { id: String, issue: String },
}
```

---

## Examples and Tutorials

### Complete Integration Example

```rust
// Example: Creating a complete custom challenge pack
mod my_challenge_pack {
    use super::*;

    /// IoT Security Challenge Pack
    pub fn get_iot_challenges() -> Vec<Challenge> {
        vec![
            iot_default_credentials(),
            iot_firmware_analysis(),
            iot_communication_interception(),
            iot_device_enumeration(),
        ]
    }

    fn iot_default_credentials() -> Challenge {
        ChallengeBuilder::new()
            .id("iot_default_creds")
            .title("The Forgotten Password")
            .description(r#"
                You've discovered an old IoT camera on the network. The ghost's
                digital presence seems to emanate from these connected devices,
                each one a gateway to understanding what happened here...
            "#)
            .prompt("What are common default credentials for IoT devices? (format: username:password)")
            .level(2)
            .rewards(150, 12)
            .category(ChallengeCategory::IoTSecurity)
            .validator(|answer| {
                let normalized = answer.to_lowercase().replace(' ', "");
                normalized.contains("admin:admin") ||
                normalized.contains("admin:password") ||
                normalized.contains("admin:")
            })
            .hint("Many IoT devices ship with default credentials for initial setup".to_string())
            .hint("Common combinations include admin:admin, admin:password, admin:123456".to_string())
            .hint("The format is typically username:password".to_string())
            .build()
            .unwrap()
    }

    // Additional challenges...
}

// Integration with main game
fn main() {
    // Initialize the game
    let mut game_state = GameState::new("Player".to_string());

    // Add custom challenges (future plugin system)
    // register_challenge_pack(my_challenge_pack::get_iot_challenges());

    // Start game loop
    game_loop(&mut game_state);
}
```

---

## Getting Help

### API Documentation Resources

-   **This Document**: Complete API reference
-   **Source Code**: `src/` directory with inline documentation
-   **Examples**: `examples/` directory (planned)
-   **Tests**: `tests/` directory showing usage patterns

### Community Support

-   **GitHub Issues**: Bug reports and feature requests
-   **GitHub Discussions**: API questions and design discussions
-   **Documentation Issues**: Report unclear or missing documentation

### Contributing to API

1. **Read**: [Contributing Guide](../CONTRIBUTING.md)
2. **Discuss**: Open issue for API changes
3. **Implement**: Follow existing patterns
4. **Document**: Update this documentation
5. **Test**: Comprehensive test coverage

---

**The API is designed for extensibility while maintaining the core horror experience. Build responsibly!** 👻

_"In the Ghost Protocol, code is poetry... dark, terrifying poetry."_
