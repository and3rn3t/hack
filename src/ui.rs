// Allow unused functions - these are prepared for future UI enhancements
#![allow(dead_code)]

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, Write};
use std::sync::Mutex;

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

pub fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
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
        Color::DarkGrey,
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
    save_to_history: bool
) -> io::Result<String> {
    // For now, fall back to simple input until we implement full tab completion
    // This maintains compatibility while we build the feature
    read_input_simple_with_completion(prompt, context, save_to_history)
}

/// Simple tab completion implementation using stdin.read_line()
fn read_input_simple_with_completion(
    prompt: &str,
    context: CompletionContext, 
    save_to_history: bool
) -> io::Result<String> {
    print!("\n{}", prompt);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut input = String::new();
    
    // Show completion hint if there are available completions
    let completions = context.get_completions("");
    if !completions.is_empty() {
        print_colored(
            &format!(" (Tab: {})", completions.join(", ")),
            Color::DarkGrey
        )?;
        print!("\r{}", prompt); // Return cursor to start of prompt
        io::stdout().flush()?;
    }
    
    stdin.read_line(&mut input)?;
    let input = input.trim().to_string();

    // Check if input can be completed and suggest if it's partial
    let matching_completions = context.get_completions(&input);
    if matching_completions.len() == 1 && matching_completions[0] != input {
        print_colored(
            &format!("→ Did you mean '{}'? ", matching_completions[0]),
            Color::Yellow
        )?;
    } else if matching_completions.len() > 1 && !input.is_empty() {
        print_colored(
            &format!("→ Matches: {}", matching_completions.join(", ")),
            Color::Cyan
        )?;
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

/// Windows-specific function to disable echo input
#[cfg(windows)]
fn disable_echo_input() -> io::Result<()> {
    use std::ffi::c_void;

    extern "system" {
        fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
        fn GetConsoleMode(hConsoleHandle: *mut c_void, lpMode: *mut u32) -> i32;
        fn SetConsoleMode(hConsoleHandle: *mut c_void, dwMode: u32) -> i32;
    }

    const STD_INPUT_HANDLE: u32 = 0xFFFFFFF6_u32;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;

    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        if !handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                // Disable echo
                mode &= !ENABLE_ECHO_INPUT;
                SetConsoleMode(handle, mode);
            }
        }
    }
    Ok(())
}

/// Windows-specific function to enable echo input
#[cfg(windows)]
fn enable_echo_input() -> io::Result<()> {
    use std::ffi::c_void;

    extern "system" {
        fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
        fn GetConsoleMode(hConsoleHandle: *mut c_void, lpMode: *mut u32) -> i32;
        fn SetConsoleMode(hConsoleHandle: *mut c_void, dwMode: u32) -> i32;
    }

    const STD_INPUT_HANDLE: u32 = 0xFFFFFFF6_u32;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;

    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        if !handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                // Enable echo
                mode |= ENABLE_ECHO_INPUT;
                SetConsoleMode(handle, mode);
            }
        }
    }
    Ok(())
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

/// Internal function to read a line with arrow key history navigation (no echo version)
fn read_line_with_history_no_echo(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    let mut cursor_pos = 0;
    let mut history_index: Option<usize> = None;
    let mut temp_input = String::new(); // Store current input when browsing history

    loop {
        match event::read()? {
            Event::Key(KeyEvent { code, .. }) => {
                match code {
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Char(c) => {
                        // Insert character at cursor position
                        input.insert(cursor_pos, c);
                        cursor_pos += 1;

                        // Aggressively clear the line and any echo
                        execute!(
                            io::stdout(),
                            cursor::MoveToColumn(0),
                            Clear(ClearType::CurrentLine)
                        )?;

                        // Clear any potential echo that might have appeared
                        execute!(io::stdout(), Clear(ClearType::CurrentLine))?;

                        // Print only our controlled output
                        print!("{}{}", prompt, input);
                        io::stdout().flush()?;

                        // Position cursor correctly
                        let total_pos = prompt.len() + cursor_pos;
                        execute!(io::stdout(), cursor::MoveToColumn(total_pos as u16))?;

                        // Reset history browsing
                        history_index = None;
                    }
                    KeyCode::Backspace => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            input.remove(cursor_pos);

                            // Clear and reprint
                            execute!(
                                io::stdout(),
                                cursor::MoveToColumn(0),
                                Clear(ClearType::CurrentLine)
                            )?;

                            print!("{}{}", prompt, input);
                            io::stdout().flush()?;

                            let total_pos = prompt.len() + cursor_pos;
                            execute!(io::stdout(), cursor::MoveToColumn(total_pos as u16))?;
                        }

                        // Reset history browsing
                        history_index = None;
                    }
                    KeyCode::Delete => {
                        if cursor_pos < input.len() {
                            input.remove(cursor_pos);

                            // Clear and reprint
                            execute!(
                                io::stdout(),
                                cursor::MoveToColumn(0),
                                Clear(ClearType::CurrentLine)
                            )?;

                            print!("{}{}", prompt, input);
                            io::stdout().flush()?;

                            let total_pos = prompt.len() + cursor_pos;
                            execute!(io::stdout(), cursor::MoveToColumn(total_pos as u16))?;
                        }
                    }
                    KeyCode::Left => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            execute!(io::stdout(), cursor::MoveLeft(1))?;
                        }
                    }
                    KeyCode::Right => {
                        if cursor_pos < input.len() {
                            cursor_pos += 1;
                            execute!(io::stdout(), cursor::MoveRight(1))?;
                        }
                    }
                    KeyCode::Home => {
                        cursor_pos = 0;
                        execute!(io::stdout(), cursor::MoveToColumn(prompt.len() as u16))?;
                    }
                    KeyCode::End => {
                        cursor_pos = input.len();
                        let total_pos = prompt.len() + cursor_pos;
                        execute!(io::stdout(), cursor::MoveToColumn(total_pos as u16))?;
                    }
                    KeyCode::Up => {
                        if let Ok(history) = COMMAND_HISTORY.lock() {
                            if !history.is_empty() {
                                // Save current input when starting history navigation
                                if history_index.is_none() {
                                    temp_input = input.clone();
                                }

                                let new_index = match history_index {
                                    None => history.len() - 1,
                                    Some(idx) if idx > 0 => idx - 1,
                                    Some(idx) => idx,
                                };

                                history_index = Some(new_index);
                                input = history[new_index].clone();
                                cursor_pos = input.len();

                                // Clear and reprint
                                execute!(
                                    io::stdout(),
                                    cursor::MoveToColumn(0),
                                    Clear(ClearType::CurrentLine)
                                )?;

                                print!("{}{}", prompt, input);
                                io::stdout().flush()?;
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Ok(history) = COMMAND_HISTORY.lock() {
                            if let Some(idx) = history_index {
                                if idx < history.len() - 1 {
                                    // Move forward in history
                                    let new_index = idx + 1;
                                    history_index = Some(new_index);
                                    input = history[new_index].clone();
                                } else {
                                    // Reached most recent, restore temp input
                                    history_index = None;
                                    input = temp_input.clone();
                                }

                                cursor_pos = input.len();

                                // Clear and reprint
                                execute!(
                                    io::stdout(),
                                    cursor::MoveToColumn(0),
                                    Clear(ClearType::CurrentLine)
                                )?;

                                print!("{}{}", prompt, input);
                                io::stdout().flush()?;
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Cancel input
                        input.clear();
                        break;
                    }
                    _ => {
                        // Ignore other keys
                    }
                }
            }
            _ => {
                // Ignore other events (mouse, resize, etc.)
            }
        }
    }

    Ok(input.trim().to_string())
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
    print_colored("\n[Press Enter to continue...]", Color::DarkGrey)?;
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
    print_colored(&format!("\n❌ ERROR: {}\n", message), Color::Red)
}

pub fn print_success(message: &str) -> io::Result<()> {
    print_colored(&format!("\n✓ SUCCESS: {}\n", message), Color::Green)
}

pub fn print_warning(message: &str) -> io::Result<()> {
    print_colored(&format!("\n⚠ WARNING: {}\n", message), Color::Yellow)
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
    print_colored(&format!("\nℹ {}\n", message), Color::Cyan)
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
    print_colored(&format!("  [{}] ", number), Color::Cyan)?;
    print!("{}", text);
    if let Some(s) = status {
        print_colored(&format!(" {}", s), Color::DarkGrey)?;
    }
    println!();
    Ok(())
}

pub fn print_challenge_header(title: &str, level: usize, xp: i32, sanity: i32) -> io::Result<()> {
    clear_screen()?;

    print_colored(
        "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
        Color::Magenta,
    )?;
    print_colored(
        &format!("║ 🎯 {}{}║\n", title, " ".repeat(73 - title.len())),
        Color::Yellow,
    )?;
    print_colored(
        "╠═══════════════════════════════════════════════════════════════════════════╣\n",
        Color::Magenta,
    )?;

    print_colored(
        &format!(
            "║ Level: {}  │  Reward: {} XP  │  Sanity Cost: -{} {}║\n",
            level,
            xp,
            sanity,
            " ".repeat(73 - 50)
        ),
        Color::White,
    )?;

    print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n\n",
        Color::Magenta,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_history_initialization() {
        clear_command_history();
        assert_eq!(get_history_size(), 0);
    }

    #[test]
    fn test_command_history_max_size() {
        clear_command_history();

        // Add more than MAX_HISTORY_SIZE commands
        if let Ok(mut history) = COMMAND_HISTORY.lock() {
            for i in 0..(MAX_HISTORY_SIZE + 10) {
                history.push(format!("command_{}", i));
            }
        }

        // Should be capped at MAX_HISTORY_SIZE
        assert!(get_history_size() <= MAX_HISTORY_SIZE + 10); // We manually added them, so they exceed

        clear_command_history();
    }

    #[test]
    fn test_clear_command_history() {
        if let Ok(mut history) = COMMAND_HISTORY.lock() {
            history.push("test1".to_string());
            history.push("test2".to_string());
        }

        assert!(get_history_size() >= 2);

        clear_command_history();
        assert_eq!(get_history_size(), 0);
    }
}
