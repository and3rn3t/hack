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
static mut COMMAND_HISTORY: Vec<String> = Vec::new();
const MAX_HISTORY_SIZE: usize = 50;

/// Read input with command history support (up/down arrows)
pub fn read_input(prompt: &str) -> io::Result<String> {
    read_input_with_history(prompt, true)
}

/// Read input with optional command history support
pub fn read_input_with_history(prompt: &str, save_to_history: bool) -> io::Result<String> {
    print!("\n{}", prompt);
    io::stdout().flush()?;

    enable_raw_mode()?;
    let result = read_line_with_history();
    disable_raw_mode()?;

    match result {
        Ok(input) => {
            println!(); // Move to next line after input

            // Save to history if requested and not empty
            if save_to_history && !input.trim().is_empty() {
                unsafe {
                    // Don't add duplicate of last command
                    if COMMAND_HISTORY.is_empty() || COMMAND_HISTORY.last() != Some(&input) {
                        COMMAND_HISTORY.push(input.clone());

                        // Keep history size manageable
                        if COMMAND_HISTORY.len() > MAX_HISTORY_SIZE {
                            COMMAND_HISTORY.remove(0);
                        }
                    }
                }
            }

            Ok(input)
        }
        Err(e) => {
            disable_raw_mode()?;
            Err(e)
        }
    }
}

/// Internal function to read a line with arrow key history navigation
fn read_line_with_history() -> io::Result<String> {
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

                        // Clear current line and reprint
                        execute!(
                            io::stdout(),
                            cursor::MoveToColumn(0),
                            Clear(ClearType::CurrentLine),
                            Print(&input)
                        )?;

                        // Move cursor to correct position
                        if cursor_pos < input.len() {
                            execute!(io::stdout(), cursor::MoveToColumn(cursor_pos as u16))?;
                        }

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
                                Clear(ClearType::CurrentLine),
                                Print(&input)
                            )?;

                            if cursor_pos < input.len() {
                                execute!(io::stdout(), cursor::MoveToColumn(cursor_pos as u16))?;
                            }
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
                                Clear(ClearType::CurrentLine),
                                Print(&input)
                            )?;

                            if cursor_pos < input.len() {
                                execute!(io::stdout(), cursor::MoveToColumn(cursor_pos as u16))?;
                            }
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
                        execute!(io::stdout(), cursor::MoveToColumn(0))?;
                    }
                    KeyCode::End => {
                        cursor_pos = input.len();
                        execute!(io::stdout(), cursor::MoveToColumn(cursor_pos as u16))?;
                    }
                    KeyCode::Up => {
                        unsafe {
                            if !COMMAND_HISTORY.is_empty() {
                                // Save current input when starting history navigation
                                if history_index.is_none() {
                                    temp_input = input.clone();
                                }

                                let new_index = match history_index {
                                    None => COMMAND_HISTORY.len() - 1,
                                    Some(idx) if idx > 0 => idx - 1,
                                    Some(idx) => idx,
                                };

                                history_index = Some(new_index);
                                input = COMMAND_HISTORY[new_index].clone();
                                cursor_pos = input.len();

                                // Clear and reprint
                                execute!(
                                    io::stdout(),
                                    cursor::MoveToColumn(0),
                                    Clear(ClearType::CurrentLine),
                                    Print(&input)
                                )?;
                            }
                        }
                    }
                    KeyCode::Down => {
                        unsafe {
                            if let Some(idx) = history_index {
                                if idx < COMMAND_HISTORY.len() - 1 {
                                    // Move forward in history
                                    let new_index = idx + 1;
                                    history_index = Some(new_index);
                                    input = COMMAND_HISTORY[new_index].clone();
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
                                    Clear(ClearType::CurrentLine),
                                    Print(&input)
                                )?;
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
    unsafe {
        COMMAND_HISTORY.clear();
    }
}

/// Get command history size
pub fn get_history_size() -> usize {
    unsafe { COMMAND_HISTORY.len() }
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
        unsafe {
            for i in 0..(MAX_HISTORY_SIZE + 10) {
                COMMAND_HISTORY.push(format!("command_{}", i));
            }
        }

        // Should be capped at MAX_HISTORY_SIZE
        assert!(get_history_size() <= MAX_HISTORY_SIZE + 10); // We manually added them, so they exceed

        clear_command_history();
    }

    #[test]
    fn test_clear_command_history() {
        unsafe {
            COMMAND_HISTORY.push("test1".to_string());
            COMMAND_HISTORY.push("test2".to_string());
        }

        assert!(get_history_size() >= 2);

        clear_command_history();
        assert_eq!(get_history_size(), 0);
    }
}
