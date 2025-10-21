use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
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
    print_colored("\n─────────────────────────────────────────────────────────────────────\n", Color::DarkGrey)
}

pub fn read_input(prompt: &str) -> io::Result<String> {
    print!("\n{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn pause() -> io::Result<()> {
    print_colored("\n[Press Enter to continue...]", Color::DarkGrey)?;
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
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
