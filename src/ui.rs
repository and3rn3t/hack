//! Cross-platform UI module for The Hack: Ghost Protocol
//!
//! This module provides UI functionality that works both in native terminals
//! and in web browsers via WebAssembly.

use std::sync::{Mutex, OnceLock};

#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};

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
pub fn set_theme(theme: ColorTheme) -> std::io::Result<()> {
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

// Native terminal UI implementation
#[cfg(feature = "native")]
mod native_ui_impl {
    use crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEvent},
        execute, queue,
        style::{Color, Print, ResetColor, SetForegroundColor},
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    };
    use std::io::{self, Write};
    use std::sync::Mutex;

    use super::{ColorTheme, CompletionContext};

    pub fn clear_screen() -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
    }

    /// Optimized clear screen with reduced flicker
    pub fn clear_screen_buffered() -> io::Result<()> {
        // Use queued commands for less flicker
        queue!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        io::stdout().flush()
    }

    /// Begin a buffered rendering session to reduce flicker
    pub fn begin_frame() -> io::Result<()> {
        // Hide cursor during rendering to reduce flicker
        queue!(io::stdout(), cursor::Hide)?;
        Ok(())
    }

    /// End a buffered rendering session and flush all output
    pub fn end_frame() -> io::Result<()> {
        // Show cursor and flush all queued output
        queue!(io::stdout(), cursor::Show)?;
        io::stdout().flush()
    }

    /// Optimized print_colored using queued output
    pub fn print_colored_buffered(text: &str, color: Color) -> io::Result<()> {
        queue!(
            io::stdout(),
            SetForegroundColor(color),
            Print(text),
            ResetColor
        )?;
        Ok(())
    }

    /// Render a complete screen with minimal flicker
    pub fn render_screen<F>(render_fn: F) -> io::Result<()>
    where
        F: FnOnce() -> io::Result<()>,
    {
        begin_frame()?;
        clear_screen_buffered()?;
        render_fn()?;
        end_frame()
    }

    /// Optimized menu rendering with buffering
    pub fn render_menu_buffered(
        title: &str,
        items: &[(String, String)],
        footer: Option<&str>,
    ) -> io::Result<()> {
        begin_frame()?;

        // Title
        print_colored_buffered(&format!("\n{}\n", title), super::theme_accent())?;
        print_separator_buffered()?;

        // Menu items
        for (key, desc) in items {
            print_colored_buffered(&format!("  [{}] ", key), super::theme_accent())?;
            print_colored_buffered(desc, super::theme_primary())?;
            queue!(io::stdout(), Print("\n"))?;
        }

        // Footer
        if let Some(footer_text) = footer {
            print_separator_buffered()?;
            print_colored_buffered(footer_text, super::theme_muted())?;
        }

        end_frame()
    }

    /// Buffered separator
    pub fn print_separator_buffered() -> io::Result<()> {
        print_colored_buffered(&format!("{}\n", "═".repeat(75)), super::theme_border())
    }

    /// Performance measurement utility
    pub struct PerformanceTimer {
        start: std::time::Instant,
        name: String,
    }

    impl PerformanceTimer {
        pub fn new(name: &str) -> Self {
            Self {
                start: std::time::Instant::now(),
                name: name.to_string(),
            }
        }

        pub fn elapsed(&self) -> std::time::Duration {
            self.start.elapsed()
        }
    }

    impl Drop for PerformanceTimer {
        fn drop(&mut self) {
            #[cfg(debug_assertions)]
            {
                let elapsed = self.elapsed();
                if elapsed.as_millis() > 10 {
                    eprintln!("[PERF] {}: {}ms", self.name, elapsed.as_millis());
                }
            }
        }
    }

    /// Memory usage tracking (debug builds only)
    #[cfg(debug_assertions)]
    pub fn print_memory_usage(label: &str) {
        // Simple memory usage reporting for development
        if let Ok(usage) = std::fs::read_to_string("/proc/self/status") {
            for line in usage.lines() {
                if line.starts_with("VmRSS:") {
                    eprintln!("[MEM] {}: {}", label, line);
                    break;
                }
            }
        }
        // On Windows, we can't easily get memory usage without additional crates
        // For now, just print the label to show where we're tracking
        #[cfg(windows)]
        eprintln!("[MEM] {} - memory tracking not available on Windows", label);
    }

    #[cfg(not(debug_assertions))]
    pub fn print_memory_usage(_label: &str) {
        // No-op in release builds
    }

    pub fn print_colored(text: &str, color: Color) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(color),
            Print(text),
            ResetColor
        )
    }

    pub fn print_horror_banner() -> io::Result<()> {
        clear_screen()?;
        print_colored(
            r#"
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║     ████████╗██╗  ██╗███████╗    ██╗  ██╗ █████╗  ██████╗██╗  ██╗      ║
║     ╚══██╔══╝██║  ██║██╔════╝    ██║  ██║██╔══██╗██╔════╝██║ ██╔╝      ║
║        ██║   ███████║█████╗      ███████║███████║██║     █████╔╝       ║
║        ██║   ██╔══██║██╔══╝      ██╔══██║██╔══██║██║     ██╔═██╗       ║
║        ██║   ██║  ██║███████╗    ██║  ██║██║  ██║╚██████╗██║  ██╗      ║
║        ╚═╝   ╚═╝  ╚═╝╚══════╝    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝      ║
║                                                                           ║
║                    ═══ GHOST PROTOCOL INITIATED ═══                      ║
║                                                                           ║
║            A Horror-Themed Hacking Simulator & CTF Challenge             ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
"#,
            Color::Red,
        )?;
        Ok(())
    }

    pub fn print_separator() -> io::Result<()> {
        print_colored(
            "\n─────────────────────────────────────────────────────────────────────\n",
            super::theme_border(),
        )
    }

    /// Command history buffer for input recall
    static COMMAND_HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());
    const MAX_HISTORY_SIZE: usize = 50;

    /// Read input with command history support (up/down arrows)
    pub fn read_input(prompt: &str) -> io::Result<String> {
        read_input_with_history(prompt, true)
    }

    /// Read input with command history support using reliable standard input
    pub fn read_input_with_history(prompt: &str, save_to_history: bool) -> io::Result<String> {
        // Using simple, reliable stdin.read_line() for consistent cross-platform behavior
        //
        // ✅ Features: Normal typing, no character doubling, command history saving
        // ✅ Reliable: Works consistently across all terminal types and platforms
        // ✅ Simple: Standard library input - no complex terminal state management
        read_input_simple(prompt, save_to_history)
    }

    /// Read input with tab completion support
    pub fn read_input_with_completion(
        prompt: &str,
        context: CompletionContext,
        save_to_history: bool,
    ) -> io::Result<String> {
        // For now, fall back to simple input until we implement full tab completion
        // This maintains compatibility while we build the feature
        read_input_simple_with_completion(prompt, context, save_to_history)
    }

    /// Simple tab completion implementation using stdin.read_line()
    fn read_input_simple_with_completion(
        prompt: &str,
        context: CompletionContext,
        save_to_history: bool,
    ) -> io::Result<String> {
        // Show available commands as helpful context
        let all_completions = context.get_completions("");
        if !all_completions.is_empty() && all_completions.len() <= 8 {
            print_colored(
                &format!("  💡 Available: {}\n", all_completions.join(", ")),
                Color::DarkGrey,
            )?;
        }

        print!("{}", prompt);
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let mut input = input.trim().to_string();

        // Smart completion and correction logic
        let matching_completions = context.get_completions(&input);

        match matching_completions.len() {
            0 => {
                // No matches - check if it's a typo we can fix
                if !input.is_empty() {
                    let close_matches = find_close_matches(&input, &all_completions);
                    if !close_matches.is_empty() {
                        print_colored(
                            &format!("❓ Did you mean: {}? [Y/n] ", close_matches.join(", ")),
                            Color::Yellow,
                        )?;
                        io::stdout().flush()?;

                        let mut correction = String::new();
                        stdin.read_line(&mut correction)?;
                        let correction = correction.trim().to_lowercase();

                        if correction.is_empty() || correction.starts_with('y') {
                            if close_matches.len() == 1 {
                                input = close_matches[0].clone();
                                print_colored(&format!("→ Using: {}", input), Color::Green)?;
                            } else {
                                // Multiple close matches, let user choose
                                print_colored("Choose:", Color::Cyan)?;
                                for (i, option) in close_matches.iter().enumerate() {
                                    print!(" [{}] {}", i + 1, option);
                                }
                                print!(" : ");
                                io::stdout().flush()?;

                                let mut choice = String::new();
                                stdin.read_line(&mut choice)?;
                                if let Ok(idx) = choice.trim().parse::<usize>() {
                                    if idx > 0 && idx <= close_matches.len() {
                                        input = close_matches[idx - 1].clone();
                                        print_colored(
                                            &format!("→ Using: {}", input),
                                            Color::Green,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            1 => {
                // Single match - auto-complete if it's clearly what they want
                let completion = &matching_completions[0];
                if completion.len() > input.len() {
                    print_colored(&format!("→ Auto-completed: {}", completion), Color::Cyan)?;
                    input = completion.clone();
                }
            }
            _ => {
                // Multiple matches - show them
                if !input.is_empty() {
                    print_colored(
                        &format!(
                            "→ {} matches: {}",
                            matching_completions.len(),
                            matching_completions.join(", ")
                        ),
                        Color::Cyan,
                    )?;
                }
            }
        }

        // Save to history if requested and not empty
        if save_to_history && !input.trim().is_empty() {
            if let Ok(mut history) = COMMAND_HISTORY.lock() {
                // Don't add duplicate of last command
                if history.is_empty() || history.last() != Some(&input) {
                    history.push(input.clone());

                    // Keep history size manageable
                    if history.len() > MAX_HISTORY_SIZE {
                        history.remove(0);
                    }
                }
            }
        }

        Ok(input)
    }

    /// Find close matches using simple edit distance for typo correction
    fn find_close_matches(input: &str, candidates: &[String]) -> Vec<String> {
        let mut matches = Vec::new();
        let input_lower = input.to_lowercase();

        for candidate in candidates {
            let candidate_lower = candidate.to_lowercase();

            // Check for substring matches
            if candidate_lower.contains(&input_lower) || input_lower.contains(&candidate_lower) {
                matches.push(candidate.clone());
                continue;
            }

            // Simple typo detection: single character difference
            if (input_lower.len() as i32 - candidate_lower.len() as i32).abs() <= 1 {
                let distance = simple_edit_distance(&input_lower, &candidate_lower);
                if distance <= 1 {
                    matches.push(candidate.clone());
                }
            }
        }

        matches
    }

    /// Simple edit distance calculation for typo correction
    fn simple_edit_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();

        if a_len == 0 {
            return b_len;
        }
        if b_len == 0 {
            return a_len;
        }

        let mut prev_row: Vec<usize> = (0..=b_len).collect();
        let mut curr_row = vec![0; b_len + 1];

        for i in 1..=a_len {
            curr_row[0] = i;

            for j in 1..=b_len {
                let cost = if a_chars[i - 1] == b_chars[j - 1] {
                    0
                } else {
                    1
                };
                curr_row[j] = (prev_row[j] + 1) // deletion
                    .min(curr_row[j - 1] + 1) // insertion
                    .min(prev_row[j - 1] + cost); // substitution
            }

            prev_row.clone_from(&curr_row);
        }

        curr_row[b_len]
    }

    /// Simple input fallback for terminals that don't support advanced features
    fn read_input_simple(prompt: &str, save_to_history: bool) -> io::Result<String> {
        print!("\n{}", prompt);
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let input = input.trim().to_string();

        // Save to history if requested and not empty
        if save_to_history && !input.trim().is_empty() {
            if let Ok(mut history) = COMMAND_HISTORY.lock() {
                // Don't add duplicate of last command
                if history.is_empty() || history.last() != Some(&input) {
                    history.push(input.clone());

                    // Keep history size manageable
                    if history.len() > MAX_HISTORY_SIZE {
                        history.remove(0);
                    }
                }
            }
        }

        Ok(input)
    }

    /// Clear command history
    pub fn clear_command_history() {
        if let Ok(mut history) = COMMAND_HISTORY.lock() {
            history.clear();
        }
    }

    /// Get command history size
    pub fn get_history_size() -> usize {
        COMMAND_HISTORY.lock().map(|h| h.len()).unwrap_or(0)
    }

    pub fn pause() -> io::Result<()> {
        print_colored("\n[Press Enter to continue...]", super::theme_muted())?;
        io::stdout().flush()?;

        // Use simple read for pause (no history needed)
        enable_raw_mode()?;
        loop {
            if let Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) = event::read()?
            {
                break;
            }
        }
        disable_raw_mode()?;

        println!(); // Move to next line
        Ok(())
    }

    pub fn print_error(message: &str) -> io::Result<()> {
        print_colored(&format!("\n❌ ERROR: {}\n", message), super::theme_error())
    }

    pub fn print_success(message: &str) -> io::Result<()> {
        print_colored(
            &format!("\n✓ SUCCESS: {}\n", message),
            super::theme_success(),
        )
    }

    pub fn print_warning(message: &str) -> io::Result<()> {
        print_colored(
            &format!("\n⚠ WARNING: {}\n", message),
            super::theme_warning(),
        )
    }

    pub fn print_glitch_effect(text: &str) -> io::Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for ch in text.chars() {
            if rng.gen_bool(0.95) {
                print!("{}", ch);
            } else {
                print!("{}", rng.gen_range('!'..='~'));
            }
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        println!();
        Ok(())
    }

    pub fn print_info(message: &str) -> io::Result<()> {
        print_colored(&format!("\nℹ {}\n", message), super::theme_info())
    }

    pub fn print_progress_bar(current: i32, max: i32, label: &str) -> io::Result<()> {
        let percentage = (current as f32 / max as f32 * 100.0) as i32;
        let filled = (current as f32 / max as f32 * 20.0) as usize;
        let empty = 20 - filled;

        let color = if percentage > 70 {
            Color::Green
        } else if percentage > 40 {
            Color::Yellow
        } else {
            Color::Red
        };

        let bar = "█".repeat(filled) + &"░".repeat(empty);
        print_colored(
            &format!(
                "{}: [{}] {}/{} ({}%)\n",
                label, bar, current, max, percentage
            ),
            color,
        )
    }

    pub fn print_box(title: &str, content: &str, color: Color) -> io::Result<()> {
        let width = 75;
        let title_padding = (width - title.len() - 4) / 2;

        print_colored(&format!("\n╔{}╗\n", "═".repeat(width)), color)?;
        print_colored(
            &format!(
                "║{}{} {}{}║\n",
                " ".repeat(title_padding),
                title,
                " ".repeat(width - title.len() - title_padding - 2),
                " "
            ),
            color,
        )?;
        print_colored(&format!("╠{}╣\n", "═".repeat(width)), color)?;

        for line in content.lines() {
            let padding = width - line.len() - 2;
            print_colored(
                &format!("║ {}{}║\n", line, " ".repeat(padding)),
                Color::White,
            )?;
        }

        print_colored(&format!("╚{}╝\n", "═".repeat(width)), color)?;
        Ok(())
    }

    pub fn print_menu_option(number: &str, text: &str, status: Option<&str>) -> io::Result<()> {
        print_colored(&format!("  [{}] ", number), super::theme_accent())?;
        print!("{}", text);
        if let Some(s) = status {
            print_colored(&format!(" {}", s), super::theme_muted())?;
        }
        println!();
        Ok(())
    }

    pub fn print_challenge_header(
        title: &str,
        level: usize,
        xp: i32,
        sanity: i32,
    ) -> io::Result<()> {
        clear_screen()?;

        print_colored(
            "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
            super::theme_border(),
        )?;
        print_colored(
            &format!("║ 🎯 {}{}║\n", title, " ".repeat(73 - title.len())),
            super::theme_accent(),
        )?;
        print_colored(
            "╠═══════════════════════════════════════════════════════════════════════════╣\n",
            super::theme_border(),
        )?;

        print_colored(
            &format!(
                "║ Level: {}  │  Reward: {} XP  │  Sanity Cost: -{} {}║\n",
                level,
                xp,
                sanity,
                " ".repeat(73 - 50)
            ),
            super::theme_primary(),
        )?;

        print_colored(
            "╚═══════════════════════════════════════════════════════════════════════════╝\n\n",
            super::theme_border(),
        )?;

        Ok(())
    }

    pub fn typewriter_effect(text: &str, delay_ms: u64) -> io::Result<()> {
        for ch in text.chars() {
            print!("{}", ch);
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        }
        println!();
        Ok(())
    }

    // ============================================================================
    // JUMPSCARE SYSTEM
    // ============================================================================

    /// Trigger a random jumpscare with configurable probability
    pub fn random_jumpscare(probability: f64) -> io::Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if rng.gen_bool(probability) {
            let jumpscare_type = rng.gen_range(0..8);
            match jumpscare_type {
                0 => jumpscare_ghost_face()?,
                1 => jumpscare_screen_shake()?,
                2 => jumpscare_glitch_text()?,
                3 => jumpscare_sudden_message()?,
                4 => jumpscare_red_eyes()?,
                5 => jumpscare_static()?,
                6 => jumpscare_whisper()?,
                _ => jumpscare_flash()?,
            }
        }
        Ok(())
    }

    /// Ghost face ASCII art jumpscare
    fn jumpscare_ghost_face() -> io::Result<()> {
        clear_screen()?;

        // Flash effect
        for _ in 0..3 {
            execute!(io::stdout(), SetForegroundColor(Color::White))?;
            print!("\n\n\n");
            execute!(io::stdout(), SetForegroundColor(Color::Red))?;

            print!(
                r#"
                        ░░░░░░░░░░░░░░░░░░░░░░░░░░
                        ░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░
                        ░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░
                        ░░▓▓░░░░░░░░░░░░░░░░▓▓░░
                        ░░▓▓░░██░░░░░░██░░░░▓▓░░
                        ░░▓▓░░██░░░░░░██░░░░▓▓░░
                        ░░▓▓░░░░░░░░░░░░░░░░▓▓░░
                        ░░▓▓░░░░░██████░░░░░▓▓░░
                        ░░▓▓░░░██░░░░░░██░░░▓▓░░
                        ░░▓▓░░░░░░░░░░░░░░░░▓▓░░
                        ░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░
                        ░░░░░░░░░░░░░░░░░░░░░░░░

                        I   S E E   Y O U . . .
            "#
            );

            execute!(io::stdout(), ResetColor)?;
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(150));
            clear_screen()?;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        std::thread::sleep(std::time::Duration::from_millis(800));
        Ok(())
    }

    /// Screen shake effect with distorted text
    fn jumpscare_screen_shake() -> io::Result<()> {
        let messages = [
            "ERROR ERROR ERROR ERROR",
            "THEY ARE COMING",
            "YOU CANNOT ESCAPE",
            "HELP US",
            "IT'S TOO LATE",
        ];

        use rand::{seq::SliceRandom, Rng};
        let mut rng = rand::thread_rng();
        let message = messages.choose(&mut rng).unwrap();

        for _ in 0..10 {
            clear_screen()?;
            let offset = rng.gen_range(0..5);
            println!("{}", "\n".repeat(offset));

            for _ in 0..rng.gen_range(0..3) {
                print!("{}", " ".repeat(rng.gen_range(0..20)));
            }

            print_colored(message, Color::Red)?;
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(80));
        }

        clear_screen()?;
        std::thread::sleep(std::time::Duration::from_millis(300));
        Ok(())
    }

    /// Glitch text effect with corrupted characters
    fn jumpscare_glitch_text() -> io::Result<()> {
        let text = "SYSTEM COMPROMISED - ENTITY DETECTED - RUN";

        clear_screen()?;
        println!("\n\n\n");

        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..5 {
            print!("        ");
            for ch in text.chars() {
                if rng.gen_bool(0.3) {
                    let glitch_char = rng.gen_range('!'..='~');
                    print_colored(&glitch_char.to_string(), Color::Red)?;
                } else {
                    print_colored(&ch.to_string(), Color::White)?;
                }
            }
            println!();
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Move cursor back up
            execute!(io::stdout(), cursor::MoveToPreviousLine(1))?;
        }

        println!();
        std::thread::sleep(std::time::Duration::from_millis(500));
        Ok(())
    }

    /// Sudden message popup
    fn jumpscare_sudden_message() -> io::Result<()> {
        clear_screen()?;

        std::thread::sleep(std::time::Duration::from_millis(100));

        execute!(io::stdout(), SetForegroundColor(Color::Red))?;
        println!("\n\n\n");
        println!("        ╔════════════════════════════════════════════╗");
        println!("        ║                                            ║");
        println!("        ║     W H Y   A R E   Y O U   H E R E ?     ║");
        println!("        ║                                            ║");
        println!("        ╚════════════════════════════════════════════╝");
        execute!(io::stdout(), ResetColor)?;

        io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(1500));

        clear_screen()?;
        Ok(())
    }

    /// Red eyes appearing in the darkness
    fn jumpscare_red_eyes() -> io::Result<()> {
        clear_screen()?;

        println!("\n\n\n\n\n\n\n");

        // Slowly reveal the eyes
        for i in 0..3 {
            if i > 0 {
                execute!(io::stdout(), cursor::MoveToPreviousLine(1))?;
            }

            let eyes = match i {
                0 => "                         ●           ●",
                1 => "                        ●●          ●●",
                _ => "                        ◉●          ●◉",
            };

            print_colored(eyes, Color::Red)?;
            println!();
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(400));
        }

        std::thread::sleep(std::time::Duration::from_millis(800));

        // Quick flash of text
        println!();
        print_colored(
            "                    I ' M   W A T C H I N G",
            Color::DarkRed,
        )?;
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(1000));
        clear_screen()?;
        Ok(())
    }

    /// Static interference effect
    fn jumpscare_static() -> io::Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        clear_screen()?;

        for _ in 0..15 {
            for _ in 0..20 {
                let chars = ['░', '▒', '▓', '█', ' ', '·', '∴'];
                let ch = chars[rng.gen_range(0..chars.len())];

                let color = if rng.gen_bool(0.1) {
                    Color::Red
                } else {
                    Color::DarkGrey
                };

                print_colored(&ch.to_string(), color)?;
            }
            println!();
        }

        io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(400));
        clear_screen()?;
        Ok(())
    }

    /// Whisper effect with fading text
    fn jumpscare_whisper() -> io::Result<()> {
        let whispers = [
            "help me...",
            "you're next...",
            "don't look behind you...",
            "they know...",
            "it's watching...",
            "join us...",
        ];

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let whisper = whispers.choose(&mut rng).unwrap();

        clear_screen()?;
        println!("\n\n\n\n\n");

        // Fade in
        for _ in 0..3 {
            print!("                    ");
            print_colored(whisper, Color::DarkGrey)?;
            println!();
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(200));
            execute!(io::stdout(), cursor::MoveToPreviousLine(1))?;
        }

        print!("                    ");
        print_colored(whisper, Color::Red)?;
        println!();
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(1200));
        clear_screen()?;
        Ok(())
    }

    /// Flash effect with color burst
    fn jumpscare_flash() -> io::Result<()> {
        for _ in 0..4 {
            clear_screen()?;

            // Full screen red
            execute!(io::stdout(), SetForegroundColor(Color::Red))?;
            for _ in 0..30 {
                println!("{}", "█".repeat(80));
            }
            execute!(io::stdout(), ResetColor)?;

            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(50));

            clear_screen()?;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        Ok(())
    }

    /// Subtle jumpscare - just a brief distortion (lower intensity)
    pub fn subtle_jumpscare() -> io::Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if rng.gen_bool(0.15) {
            match rng.gen_range(0..3) {
                0 => {
                    // Brief text corruption
                    print_colored("█", Color::Red)?;
                    io::stdout().flush()?;
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                1 => {
                    // Whispered word
                    print_colored(" [̲̅w̲̅a̲̅t̲̅c̲̅h̲̅i̲̅n̲̅g̲̅] ", Color::DarkGrey)?;
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
                _ => {
                    // Brief screen flicker
                    execute!(io::stdout(), SetForegroundColor(Color::DarkRed))?;
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    execute!(io::stdout(), ResetColor)?;
                }
            }
        }

        Ok(())
    }

    /// Theme selection interface
    pub fn show_theme_selection() -> io::Result<()> {
        clear_screen()?;

        print_colored("\n🎨 COLOR THEME SELECTION\n", super::theme_accent())?;
        print_separator()?;

        print_colored(
            "\nChoose a color theme for better accessibility and personalization:\n",
            super::theme_primary(),
        )?;

        let themes = ColorTheme::all_themes();
        let current_theme = super::get_theme();

        for (idx, theme) in themes.iter().enumerate() {
            let marker = if theme.name == current_theme.name {
                "● ACTIVE"
            } else {
                "○"
            };

            print_colored(&format!("  [{}] ", idx + 1), super::theme_accent())?;
            print_colored(&theme.name, super::theme_primary())?;
            print_colored(
                &format!(" {}", marker),
                if theme.name == current_theme.name {
                    super::theme_success()
                } else {
                    super::theme_muted()
                },
            )?;

            // Show preview of theme colors
            print!(" │ ");
            print_colored("Sample", theme.primary)?;
            print!(" ");
            print_colored("Text", theme.accent)?;
            print!(" ");
            print_colored("Colors", theme.success)?;
            println!();
        }

        print_separator()?;
        print_colored("  [0] Return to main menu\n", super::theme_muted())?;

        loop {
            let input = read_input_with_completion(
                "\n> Select theme (1-5) or 0 to return: ",
                CompletionContext::None,
                true,
            )?;

            match input.as_str() {
                "0" => {
                    print_info("Returning to main menu...")?;
                    pause()?;
                    return Ok(());
                }
                "1" | "2" | "3" | "4" | "5" => {
                    if let Ok(idx) = input.parse::<usize>() {
                        if idx > 0 && idx <= themes.len() {
                            let selected_theme = &themes[idx - 1];
                            super::set_theme(selected_theme.clone())?;

                            print_success(&format!("Theme changed to '{}'!", selected_theme.name))?;

                            // Show preview with new theme
                            print_colored("\nPreview of new theme:\n", super::theme_primary())?;
                            print_colored("✓ Success message", super::theme_success())?;
                            print_colored("⚠ Warning message", super::theme_warning())?;
                            print_colored("❌ Error message", super::theme_error())?;
                            print_colored("ℹ Info message", super::theme_info())?;
                            print_colored("Regular text", super::theme_primary())?;
                            print_colored("Muted text", super::theme_muted())?;

                            pause()?;
                            return Ok(());
                        }
                    }
                    print_warning("Please enter a number between 1-5 or 0 to return.")?;
                }
                _ => {
                    print_warning("Please enter a number between 1-5 or 0 to return.")?;
                }
            }
        }
    }
}

#[cfg(feature = "native")]
pub use native_ui_impl::*;

#[cfg(feature = "web")]
mod web_stubs {
    use super::*;
    use std::io;

    pub type IoResult<T> = io::Result<T>;

    // Web stub implementations - all functions return success or empty values
    pub fn clear_screen() -> io::Result<()> {
        Ok(())
    }
    pub fn clear_screen_buffered() -> io::Result<()> {
        Ok(())
    }
    pub fn begin_frame() -> io::Result<()> {
        Ok(())
    }
    pub fn end_frame() -> io::Result<()> {
        Ok(())
    }
    pub fn print_colored_buffered(_text: &str, _color: Color) -> io::Result<()> {
        Ok(())
    }
    pub fn render_screen<F>(_render_fn: F) -> io::Result<()>
    where
        F: FnOnce() -> io::Result<()>,
    {
        Ok(())
    }
    pub fn render_menu_buffered(
        _title: &str,
        _items: &[(String, String)],
        _footer: Option<&str>,
    ) -> io::Result<()> {
        Ok(())
    }
    pub fn print_separator_buffered() -> io::Result<()> {
        Ok(())
    }
    pub fn print_colored(_text: &str, _color: Color) -> io::Result<()> {
        Ok(())
    }
    pub fn print_horror_banner() -> io::Result<()> {
        Ok(())
    }
    pub fn print_separator() -> io::Result<()> {
        Ok(())
    }
    pub fn read_input(_prompt: &str) -> io::Result<String> {
        Ok(String::new())
    }
    pub fn read_input_with_history(_prompt: &str, _save_to_history: bool) -> io::Result<String> {
        Ok(String::new())
    }
    pub fn read_input_with_completion(
        _prompt: &str,
        _context: CompletionContext,
        _save_to_history: bool,
    ) -> io::Result<String> {
        Ok(String::new())
    }
    pub fn clear_command_history() {}
    pub fn get_history_size() -> usize {
        0
    }
    pub fn pause() -> io::Result<()> {
        Ok(())
    }
    pub fn print_error(_message: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn print_success(_message: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn print_warning(_message: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn print_glitch_effect(_text: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn print_info(_message: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn print_progress_bar(_current: i32, _max: i32, _label: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn print_box(_title: &str, _content: &str, _color: Color) -> io::Result<()> {
        Ok(())
    }
    pub fn print_menu_option(_number: &str, _text: &str, _status: Option<&str>) -> io::Result<()> {
        Ok(())
    }
    pub fn print_challenge_header(
        _title: &str,
        _level: usize,
        _xp: i32,
        _sanity: i32,
    ) -> io::Result<()> {
        Ok(())
    }
    pub fn typewriter_effect(_text: &str, _delay_ms: u64) -> io::Result<()> {
        Ok(())
    }
    pub fn random_jumpscare(_probability: f64) -> io::Result<()> {
        Ok(())
    }
    pub fn subtle_jumpscare() -> io::Result<()> {
        Ok(())
    }
    pub fn show_theme_selection() -> io::Result<()> {
        Ok(())
    }
    pub fn print_memory_usage(_label: &str) {}

    pub struct PerformanceTimer {
        _name: String,
    }

    impl PerformanceTimer {
        pub fn new(name: &str) -> Self {
            Self {
                _name: name.to_string(),
            }
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
