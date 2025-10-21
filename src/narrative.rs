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
    
    let level_names = [
        "Level 0: The Awakening",
        "Level 1: Whispers in the Code",
        "Level 2: The Forgotten Server",
        "Level 3: Cryptic Messages",
        "Level 4: The Dark Network",
        "Level 5: Phantom Protocols",
        "Level 6: Data Corruption",
        "Level 7: The Haunted Firewall",
        "Level 8: Soul Extraction",
        "Level 9: The Final Breach",
        "Level 10: Ghost Protocol Complete",
    ];
    
    let level_name = level_names.get(level).unwrap_or(&"Unknown Level");
    
    ui::print_colored(&format!("\n\n╔═══════════════════════════════════════════╗\n"), Color::DarkRed)?;
    ui::print_colored(&format!("║   {}   ║\n", level_name), Color::Red)?;
    ui::print_colored(&format!("╚═══════════════════════════════════════════╝\n"), Color::DarkRed)?;
    
    // Show sanity meter
    show_sanity_meter(sanity)?;
    
    if level == 0 {
        ui::print_colored("\nYou begin your descent into the cursed system...\n", Color::White)?;
    } else if level < 5 {
        ui::print_colored("\nThe shadows grow darker. You feel watched...\n", Color::White)?;
    } else if level < 8 {
        ui::print_colored("\nThe system fights back. Reality blurs...\n", Color::Yellow)?;
    } else {
        ui::print_colored("\nYou're close to the truth. But at what cost?\n", Color::Red)?;
    }
    
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
    
    ui::print_colored(&format!("\nSanity: [{}{}] {}%\n", meter, empty, sanity), color)?;
    
    if sanity < 30 {
        ui::print_warning("Your grip on reality is slipping...")?;
    }
    
    Ok(())
}

pub fn show_challenge_intro(title: &str, description: &str) -> io::Result<()> {
    ui::print_separator()?;
    ui::print_colored(&format!("\n🔒 CHALLENGE: {}\n", title), Color::Cyan)?;
    ui::print_separator()?;
    ui::print_colored(&format!("\n{}\n", description), Color::White)?;
    Ok(())
}

pub fn show_hint(hint: &str) -> io::Result<()> {
    ui::print_colored(&format!("\n💡 HINT: {}\n", hint), Color::Yellow)?;
    Ok(())
}

pub fn show_completion_message(xp_earned: i32) -> io::Result<()> {
    ui::print_success(&format!("Challenge completed! +{} XP", xp_earned))?;
    
    let messages = [
        "One soul freed from the digital prison...",
        "The system trembles...",
        "You hear a faint whisper: 'Thank you...'",
        "The darkness recedes, if only for a moment...",
    ];
    
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    if let Some(msg) = messages.choose(&mut rng) {
        ui::print_colored(&format!("\n{}\n", msg), Color::Magenta)?;
    }
    
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
        ui::print_colored(&format!("\n\nYou discovered {} hidden secret(s) along the way.\n", secrets_found), Color::Yellow)?;
        ui::print_colored("The secrets reveal the true nature of the Ghost Protocol...\n", Color::White)?;
    }
    
    ui::print_colored("\n\nThank you for playing THE HACK: Ghost Protocol\n", Color::Green)?;
    ui::pause()?;
    Ok(())
}
