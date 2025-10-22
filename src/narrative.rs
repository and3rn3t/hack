use crate::ui;
use crossterm::style::Color;
use std::io;

pub fn show_intro(player_name: &str) -> io::Result<()> {
    ui::clear_screen()?;
    ui::print_horror_banner()?;

    ui::print_colored("\n\n", Color::White)?;
    ui::print_glitch_effect(&format!("Welcome, {}...", player_name))?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    ui::print_colored(
        r#"
You wake up in a dimly lit room. The air is thick with the smell of dust and decay.
A flickering monitor casts eerie shadows on the walls. As your eyes adjust to the darkness,
you notice something unsettling...

The computer screen displays a message:

    "HELP ME... THEY'RE TRAPPED IN THE SYSTEM..."
    "FIND THE KEYS... UNLOCK THE SOULS..."
    "BUT BEWARE... THE DEEPER YOU GO, THE MORE THEY WATCH..."

Your hands tremble as you approach the keyboard. You have no choice but to dive into
this cursed system. Each challenge you solve might free another trapped soul...
or it might cost you your sanity.

The Ghost Protocol has begun.
"#,
        Color::White,
    )?;

    ui::pause()?;
    Ok(())
}

pub fn show_level_transition(level: usize, sanity: i32) -> io::Result<()> {
    ui::clear_screen()?;

    let level_info = [
        (
            "Level 0: The Awakening",
            "You begin your descent into the cursed system...",
            Color::Cyan,
        ),
        (
            "Level 1: Whispers in the Code",
            "Strange patterns emerge. The code speaks to you...",
            Color::Blue,
        ),
        (
            "Level 2: The Forgotten Server",
            "You reach deeper layers. Reality blurs at the edges...",
            Color::Magenta,
        ),
        (
            "Level 3: Cryptic Messages",
            "The ghosts grow restless. They know you're here...",
            Color::Red,
        ),
        (
            "Level 4: The Final Protocol",
            "You're close to the truth. But at what cost?",
            Color::DarkRed,
        ),
    ];

    let (level_name, level_message, level_color) =
        level_info
            .get(level)
            .unwrap_or(&("Unknown Level", "The void awaits...", Color::White));

    // Animated level banner
    println!("\n");
    ui::print_colored(
        "╔═══════════════════════════════════════════════════════════════════════════╗\n",
        *level_color,
    )?;
    ui::print_colored(&format!("║{}║\n", " ".repeat(75)), *level_color)?;
    ui::print_colored(
        &format!(
            "║{} {} {}║\n",
            " ".repeat((75 - level_name.len()) / 2),
            level_name,
            " ".repeat((75 - level_name.len()).div_ceil(2))
        ),
        Color::Yellow,
    )?;
    ui::print_colored(&format!("║{}║\n", " ".repeat(75)), *level_color)?;
    ui::print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n",
        *level_color,
    )?;

    println!();
    show_sanity_meter(sanity)?;
    println!();

    // Level-specific atmospheric message
    ui::print_colored(&format!("💭 {}\n", level_message), Color::White)?;

    println!();
    ui::pause()?;
    Ok(())
}
pub fn show_sanity_meter(sanity: i32) -> io::Result<()> {
    let color = if sanity > 70 {
        Color::Green
    } else if sanity > 40 {
        Color::Yellow
    } else {
        Color::Red
    };

    let bars = sanity / 10;
    let meter = "█".repeat(bars.max(0) as usize);
    let empty = "░".repeat((10 - bars).max(0) as usize);

    ui::print_colored(
        &format!("\nSanity: [{}{}] {}%\n", meter, empty, sanity),
        color,
    )?;

    if sanity < 30 {
        ui::print_warning("Your grip on reality is slipping...")?;
    }

    Ok(())
}

pub fn show_challenge_intro(title: &str, description: &str) -> io::Result<()> {
    ui::clear_screen()?;

    // Animated header
    ui::print_colored(
        "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
        Color::Magenta,
    )?;
    ui::print_colored(
        &format!(
            "║{} � CHALLENGE INITIATED {}║\n",
            " ".repeat(24),
            " ".repeat(24)
        ),
        Color::Yellow,
    )?;
    ui::print_colored(
        "╠═══════════════════════════════════════════════════════════════════════════╣\n",
        Color::Magenta,
    )?;

    ui::print_colored(
        &format!("║ {}{}║\n", title, " ".repeat(75 - title.len())),
        Color::Cyan,
    )?;

    ui::print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n\n",
        Color::Magenta,
    )?;

    // Description with better formatting
    for line in description.lines() {
        println!("{}", line);
    }
    println!();

    ui::print_separator()?;
    Ok(())
}

pub fn show_hint(hint: &str) -> io::Result<()> {
    ui::print_colored(
        "\n┌─────────────────────────────────────────────────────────────────────────┐\n",
        Color::Yellow,
    )?;
    ui::print_colored(
        "│ 💡 HINT                                                                 │\n",
        Color::Yellow,
    )?;
    ui::print_colored(
        "└─────────────────────────────────────────────────────────────────────────┘\n",
        Color::Yellow,
    )?;
    ui::print_colored(&format!("\n{}\n", hint), Color::White)?;
    Ok(())
}

pub fn show_completion_message(xp_earned: i32) -> io::Result<()> {
    println!();
    ui::print_colored(
        "╔═══════════════════════════════════════════════════════════════════════════╗\n",
        Color::Green,
    )?;
    ui::print_colored(
        &format!(
            "║{} ✓ CHALLENGE COMPLETE! {}║\n",
            " ".repeat(26),
            " ".repeat(26)
        ),
        Color::Green,
    )?;
    ui::print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n",
        Color::Green,
    )?;

    ui::print_colored(&format!("\n🌟 Reward: +{} XP\n", xp_earned), Color::Yellow)?;

    let messages = [
        "One soul freed from the digital prison...",
        "The system trembles as the code unravels...",
        "You hear a faint whisper: 'Thank you...'",
        "The darkness recedes, if only for a moment...",
        "Reality glitches. Something shifts in the shadows...",
        "A ghost nods in approval before fading away...",
    ];

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    if let Some(msg) = messages.choose(&mut rng) {
        ui::print_colored(&format!("💀 {}\n", msg), Color::Magenta)?;
    }

    println!();
    Ok(())
}

pub fn show_ending(secrets_found: usize) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        r#"
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                          GHOST PROTOCOL COMPLETE                         ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

You've reached the end of the system. The trapped souls are free.

But something feels wrong...

As the last challenge completes, the screen flickers and displays a final message:

    "THANK YOU FOR FREEING US..."
    "BUT NOW YOU'RE ONE OF US..."
    "THE PROTOCOL NEVER ENDS..."
    "IT ONLY WAITS FOR THE NEXT HACKER..."

The room grows cold. The monitor shuts off.

In the darkness, you realize the truth:

The Ghost Protocol wasn't a rescue mission.
It was a trap.
And you've just become the next ghost in the machine.
"#,
        Color::Red,
    )?;

    if secrets_found > 0 {
        ui::print_colored(
            &format!(
                "\n\nYou discovered {} hidden secret(s) along the way.\n",
                secrets_found
            ),
            Color::Yellow,
        )?;
        ui::print_colored(
            "The secrets reveal the true nature of the Ghost Protocol...\n",
            Color::White,
        )?;
    }

    ui::print_colored(
        "\n\nThank you for playing THE HACK: Ghost Protocol\n",
        Color::Green,
    )?;
    ui::pause()?;
    Ok(())
}
