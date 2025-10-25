//! Cross-platform UI module for The Hack: Ghost Protocol
//! 
//! This module provides UI functionality that works both in native terminals
//! and in web browsers via WebAssembly.

use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

// Re-export crossterm Color for native builds
#[cfg(feature = "native")]
pub use crossterm::style::Color;

// Define our own Color enum for web builds
#[cfg(feature = "web")]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Grey,
    DarkGrey,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    White,
}

#[cfg(feature = "web")]
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Color theme system for customizable UI appearance
#[derive(Debug, Clone, PartialEq)]
pub struct ColorTheme {
    pub name: String,
    pub primary: Color,    // Main text, titles
    pub secondary: Color,  // Subtitles, descriptions
    pub accent: Color,     // Highlights, selections
    pub success: Color,    // Success messages, completed items
    pub warning: Color,    // Warnings, hints
    pub error: Color,      // Errors, dangerous actions
    pub info: Color,       // Information, help text
    pub muted: Color,      // Less important text, placeholders
    pub border: Color,     // Borders, separators
    pub background: Color, // Background color
}

impl ColorTheme {
    /// Default horror theme (current colors)
    pub fn horror() -> Self {
        Self {
            name: "Horror".to_string(),
            primary: Color::White,
            secondary: Color::Yellow,
            accent: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            muted: Color::DarkGrey,
            border: Color::Cyan,
            background: Color::Black,
        }
    }

    /// High contrast theme for accessibility
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            primary: Color::White,
            secondary: Color::White,
            accent: Color::Yellow,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::White,
            muted: Color::Grey,
            border: Color::White,
            background: Color::Black,
        }
    }

    /// Dark theme for reduced eye strain
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            primary: Color::Grey,
            secondary: Color::DarkGrey,
            accent: Color::Blue,
            success: Color::DarkGreen,
            warning: Color::DarkYellow,
            error: Color::DarkRed,
            info: Color::DarkBlue,
            muted: Color::DarkGrey,
            border: Color::DarkBlue,
            background: Color::Black,
        }
    }

    /// Colorblind-friendly theme (deuteranopia/protanopia safe)
    pub fn colorblind_friendly() -> Self {
        Self {
            name: "Colorblind Friendly".to_string(),
            primary: Color::White,
            secondary: Color::Grey,
            accent: Color::Blue,
            success: Color::Blue, // Blue instead of green
            warning: Color::Yellow,
            error: Color::Magenta, // Magenta instead of red
            info: Color::Cyan,
            muted: Color::DarkGrey,
            border: Color::Blue,
            background: Color::Black,
        }
    }

    /// Retro green monochrome theme
    pub fn retro() -> Self {
        Self {
            name: "Retro Green".to_string(),
            primary: Color::Green,
            secondary: Color::DarkGreen,
            accent: Color::Green,
            success: Color::Green,
            warning: Color::Green,
            error: Color::Green,
            info: Color::Green,
            muted: Color::DarkGreen,
            border: Color::Green,
            background: Color::Black,
        }
    }

    /// Get all available themes
    pub fn all_themes() -> Vec<Self> {
        vec![
            Self::horror(),
            Self::high_contrast(),
            Self::dark(),
            Self::colorblind_friendly(),
            Self::retro(),
        ]
    }
}

/// Context for tab completion - determines what commands are available
#[derive(Debug, Clone)]
pub enum CompletionContext {
    /// Main menu context (stats, help, save, quit, challenge numbers)
    MainMenu { challenge_count: usize },
    /// During a challenge (hint, skip)
    Challenge,
    /// Help menu context (1-5, back)
    HelpMenu,
    /// No completion available
    None,
}

impl CompletionContext {
    /// Get all valid completions for this context
    pub fn get_completions(&self, partial: &str) -> Vec<String> {
        let candidates = match self {
            CompletionContext::MainMenu { challenge_count } => {
                let mut commands = vec![
                    "stats".to_string(),
                    "help".to_string(),
                    "tutorial".to_string(),
                    "theme".to_string(),
                    "themes".to_string(),
                    "save".to_string(),
                    "quit".to_string(),
                ];
                // Add challenge numbers
                for i in 1..=*challenge_count {
                    commands.push(i.to_string());
                }
                commands
            }
            CompletionContext::Challenge => {
                vec!["hint".to_string(), "skip".to_string()]
            }
            CompletionContext::HelpMenu => {
                vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                    "back".to_string(),
                ]
            }
            CompletionContext::None => vec![],
        };

        // Filter candidates that start with the partial input (case insensitive)
        let partial_lower = partial.to_lowercase();
        candidates
            .into_iter()
            .filter(|cmd| cmd.to_lowercase().starts_with(&partial_lower))
            .collect()
    }
}

/// Current active theme (thread-safe)
static CURRENT_THEME: OnceLock<Mutex<ColorTheme>> = OnceLock::new();

/// Initialize the theme system (call once)
fn init_theme() -> &'static Mutex<ColorTheme> {
    CURRENT_THEME.get_or_init(|| Mutex::new(ColorTheme::horror()))
}

/// Set the active color theme
#[cfg(feature = "native")]
pub fn set_theme(theme: ColorTheme) -> std::io::Result<()> {
    if let Ok(mut current) = init_theme().lock() {
        *current = theme;
    }
    Ok(())
}

#[cfg(feature = "web")]
pub fn set_theme(theme: ColorTheme) -> Result<(), String> {
    if let Ok(mut current) = init_theme().lock() {
        *current = theme;
    }
    Ok(())
}

/// Get the current color theme
pub fn get_theme() -> ColorTheme {
    init_theme()
        .lock()
        .unwrap_or_else(|_| panic!("Theme mutex poisoned"))
        .clone()
}

/// Themed color accessors for consistent UI
pub fn theme_primary() -> Color {
    get_theme().primary
}

pub fn theme_secondary() -> Color {
    get_theme().secondary
}

pub fn theme_accent() -> Color {
    get_theme().accent
}

pub fn theme_success() -> Color {
    get_theme().success
}

pub fn theme_warning() -> Color {
    get_theme().warning
}

pub fn theme_error() -> Color {
    get_theme().error
}

pub fn theme_info() -> Color {
    get_theme().info
}

pub fn theme_muted() -> Color {
    get_theme().muted
}

pub fn theme_border() -> Color {
    get_theme().border
}

// Conditional compilation - use native or web implementations
#[cfg(feature = "native")]
mod ui_native;

#[cfg(feature = "native")]
pub use ui_native::*;

#[cfg(feature = "web")]
mod web_stubs {
    use super::*;

    pub type IoResult<T> = Result<T, String>;

    // Web stub implementations - all functions return success or empty values
    pub fn clear_screen() -> IoResult<()> { Ok(()) }
    pub fn clear_screen_buffered() -> IoResult<()> { Ok(()) }
    pub fn begin_frame() -> IoResult<()> { Ok(()) }
    pub fn end_frame() -> IoResult<()> { Ok(()) }
    pub fn print_colored_buffered(_text: &str, _color: Color) -> IoResult<()> { Ok(()) }
    pub fn render_screen<F>(_render_fn: F) -> IoResult<()> 
    where F: FnOnce() -> IoResult<()> { Ok(()) }
    pub fn render_menu_buffered(_title: &str, _items: &[(String, String)], _footer: Option<&str>) -> IoResult<()> { Ok(()) }
    pub fn print_separator_buffered() -> IoResult<()> { Ok(()) }
    pub fn print_colored(_text: &str, _color: Color) -> IoResult<()> { Ok(()) }
    pub fn print_horror_banner() -> IoResult<()> { Ok(()) }
    pub fn print_separator() -> IoResult<()> { Ok(()) }
    pub fn read_input(_prompt: &str) -> IoResult<String> { Ok(String::new()) }
    pub fn read_input_with_history(_prompt: &str, _save_to_history: bool) -> IoResult<String> { Ok(String::new()) }
    pub fn read_input_with_completion(_prompt: &str, _context: CompletionContext, _save_to_history: bool) -> IoResult<String> { Ok(String::new()) }
    pub fn clear_command_history() {}
    pub fn get_history_size() -> usize { 0 }
    pub fn pause() -> IoResult<()> { Ok(()) }
    pub fn print_error(_message: &str) -> IoResult<()> { Ok(()) }
    pub fn print_success(_message: &str) -> IoResult<()> { Ok(()) }
    pub fn print_warning(_message: &str) -> IoResult<()> { Ok(()) }
    pub fn print_glitch_effect(_text: &str) -> IoResult<()> { Ok(()) }
    pub fn print_info(_message: &str) -> IoResult<()> { Ok(()) }
    pub fn print_progress_bar(_current: i32, _max: i32, _label: &str) -> IoResult<()> { Ok(()) }
    pub fn print_box(_title: &str, _content: &str, _color: Color) -> IoResult<()> { Ok(()) }
    pub fn print_menu_option(_number: &str, _text: &str, _status: Option<&str>) -> IoResult<()> { Ok(()) }
    pub fn print_challenge_header(_title: &str, _level: usize, _xp: i32, _sanity: i32) -> IoResult<()> { Ok(()) }
    pub fn typewriter_effect(_text: &str, _delay_ms: u64) -> IoResult<()> { Ok(()) }
    pub fn random_jumpscare(_probability: f64) -> IoResult<()> { Ok(()) }
    pub fn subtle_jumpscare() -> IoResult<()> { Ok(()) }
    pub fn show_theme_selection() -> IoResult<()> { Ok(()) }
    pub fn print_memory_usage(_label: &str) {}

    pub struct PerformanceTimer {
        _name: String,
    }

    impl PerformanceTimer {
        pub fn new(name: &str) -> Self {
            Self { _name: name.to_string() }
        }

        pub fn elapsed(&self) -> std::time::Duration {
            std::time::Duration::from_millis(0)
        }
    }

    impl Drop for PerformanceTimer {
        fn drop(&mut self) {}
    }
}

#[cfg(feature = "web")]
pub use web_stubs::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = ColorTheme::horror();
        assert_eq!(theme.name, "Horror");
        assert_eq!(theme.primary, Color::White);
    }

    #[test]
    fn test_all_themes() {
        let themes = ColorTheme::all_themes();
        assert!(themes.len() >= 5);
        
        let theme_names: Vec<_> = themes.iter().map(|t| &t.name).collect();
        assert!(theme_names.contains(&&"Horror".to_string()));
        assert!(theme_names.contains(&&"High Contrast".to_string()));
    }

    #[test]
    fn test_completion_context() {
        let context = CompletionContext::MainMenu { challenge_count: 5 };
        let completions = context.get_completions("he");
        assert!(completions.contains(&"help".to_string()));
    }

    #[test]
    fn test_theme_system() {
        let original_theme = get_theme();
        let new_theme = ColorTheme::dark();
        
        set_theme(new_theme.clone()).unwrap();
        
        let current_theme = get_theme();
        assert_eq!(current_theme.name, "Dark");
        
        // Restore original theme
        set_theme(original_theme).unwrap();
    }
}