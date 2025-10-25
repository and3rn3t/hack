use crate::{state::GameState, ui};
use std::io;

/// Convert UI result to io::Result for compatibility
#[cfg(feature = "web")]
fn ui_to_io_result<T>(result: Result<T, String>) -> std::io::Result<T> {
    result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[cfg(feature = "native")]
fn ui_to_io_result<T>(result: std::io::Result<T>) -> std::io::Result<T> {
    result
}

/// Run the interactive tutorial for new players
pub fn run_tutorial(state: &mut GameState) -> io::Result<()> {
    ui::clear_screen()?;

    show_welcome()?;
    explain_game_mechanics()?;
    explain_sanity_system()?;
    explain_challenges()?;

    // Practice challenges section
    practice_challenge_intro()?;
    practice_challenge_basic(state)?;
    practice_challenge_encoding(state)?;
    practice_challenge_web(state)?;
    practice_challenge_file_analysis(state)?;

    explain_hints_and_commands()?;
    show_tutorial_complete()?;

    state.mark_tutorial_completed();
    state.save()?;

    Ok(())
}

fn show_welcome() -> io::Result<()> {
    ui::print_colored(
        r#"
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                     🎓 WELCOME TO THE GHOST PROTOCOL 🎓                  ║
║                                                                           ║
║                        Interactive Tutorial System                        ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
"#,
        crossterm::style::Color::Cyan,
    )?;

    ui::print_colored(
        "\nWelcome, new recruit! This tutorial will guide you through the basics\n",
        crossterm::style::Color::White,
    )?;
    ui::print_colored(
        "of the Ghost Protocol challenge system.\n",
        crossterm::style::Color::White,
    )?;

    ui::pause()?;
    Ok(())
}

fn explain_game_mechanics() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored("\n📖 GAME MECHANICS\n", crossterm::style::Color::Yellow)?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
The Ghost Protocol is a horror-themed hacking simulator where you'll:

  1. 🎯 SOLVE CHALLENGES - Complete cybersecurity puzzles
  2. ⭐ EARN EXPERIENCE - Gain XP to unlock higher levels
  3. 🧠 MANAGE SANITY - Your mental state affects gameplay
  4. 📚 LEARN SKILLS - Master real hacking concepts

Each challenge teaches you actual cybersecurity techniques used in
Capture The Flag (CTF) competitions and real-world security.
"#,
        crossterm::style::Color::White,
    )?;

    ui::print_colored(
        "\n💡 TIP: Don't worry if you're new to hacking - we start with basics!\n",
        crossterm::style::Color::Cyan,
    )?;

    ui::pause()?;
    Ok(())
}

fn explain_sanity_system() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored("\n🧠 THE SANITY SYSTEM\n", crossterm::style::Color::Red)?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
This is a HORROR game. Each challenge you attempt drains your sanity:

"#,
        crossterm::style::Color::White,
    )?;

    // Show a sample sanity bar
    ui::print_progress_bar(75, 100, "Sanity")?;

    ui::print_colored(
        r#"
  • You start with 100 sanity
  • Each challenge costs 5-15 sanity (harder = more cost)
  • Failed challenges drain an additional 10 sanity
  • If sanity reaches 0, it's GAME OVER!

Strategy: Choose challenges wisely and use hints when stuck!
"#,
        crossterm::style::Color::White,
    )?;

    ui::print_colored(
        "\n⚠️  WARNING: The deeper you go, the more disturbing it gets...\n",
        crossterm::style::Color::Red,
    )?;

    ui::pause()?;
    Ok(())
}

fn explain_challenges() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored("\n🎯 CHALLENGE SYSTEM\n", crossterm::style::Color::Green)?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
Challenges are organized by DIFFICULTY LEVEL (0-4):

  Level 0 - The Awakening 🌅
    → Beginner-friendly encoding and basic concepts
    → Base64, ROT13, binary, URL encoding

  Level 1 - Whispers in the Code 🌙
    → Intermediate cryptography and web basics
    → Caesar cipher, SQL injection, hexadecimal

  Level 2 - The Forgotten Server 🌑
    → Web and mobile security
    → HTTP headers, deep links, API security

  Level 3+ - Into the Abyss 💀
    → Advanced exploitation and reverse engineering
    → Buffer overflows, XOR operations, binary analysis

You unlock new levels by gaining experience points (XP).
"#,
        crossterm::style::Color::White,
    )?;

    ui::pause()?;
    Ok(())
}

fn practice_challenge_intro() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n🎓 PRACTICE CHALLENGES\n",
        crossterm::style::Color::Magenta,
    )?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
Great! Now let's put your knowledge to the test with some practice challenges.

These are simplified versions of real cybersecurity challenges you'll encounter.
They won't cost any sanity, and you'll earn XP for each correct answer!

We'll cover four key areas:
  1. 🧠 Basic Problem Solving
  2. 🔤 Encoding & Decoding
  3. 🌐 Web Security Basics
  4. 📁 File Analysis

Ready to become a cyber warrior? Let's begin!
"#,
        crossterm::style::Color::White,
    )?;

    ui::pause()?;
    Ok(())
}

fn practice_challenge_basic(state: &mut GameState) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n🧠 CHALLENGE 1: Basic Problem Solving\n",
        crossterm::style::Color::Cyan,
    )?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
📝 TUTORIAL CHALLENGE: First Steps

The Ghost Protocol begins with a simple question to test your resolve.

What is the answer to life, the universe, and everything?
(Hint: It's a number from a famous sci-fi novel)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#,
        crossterm::style::Color::White,
    )?;

    let mut attempts = 0;
    loop {
        let answer = ui::read_input("\nYour answer: ")?;

        if answer == "42" {
            ui::print_success("✅ Correct! Welcome to the Ghost Protocol.")?;
            ui::print_colored(
                "\n+25 XP (Tutorial Bonus)\n",
                crossterm::style::Color::Green,
            )?;
            state.experience += 25;
            break;
        } else {
            attempts += 1;
            if attempts == 1 {
                ui::print_colored(
                    "\n🔍 Not quite. Think about Douglas Adams' Hitchhiker's Guide to the Galaxy.\n",
                    crossterm::style::Color::Yellow,
                )?;
            } else if attempts == 2 {
                ui::print_colored(
                    "\n💡 HINT: It's a two-digit number: 4_\n",
                    crossterm::style::Color::Cyan,
                )?;
            } else {
                ui::print_colored(
                    "\n✨ The answer is 42! Don't worry, real challenges will be explained better.\n",
                    crossterm::style::Color::White,
                )?;
                state.experience += 10; // Partial credit
                break;
            }
        }
    }

    ui::print_colored(
        "\n📚 LEARNING: In cybersecurity, attention to detail and research skills are crucial!\n",
        crossterm::style::Color::DarkGrey,
    )?;
    ui::pause()?;
    Ok(())
}

fn practice_challenge_encoding(state: &mut GameState) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n🔤 CHALLENGE 2: Encoding & Decoding\n",
        crossterm::style::Color::Yellow,
    )?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
📝 ENCODING CHALLENGE: ROT13 Cipher

ROT13 is a simple letter substitution cipher that replaces each letter
with the letter 13 positions ahead in the alphabet.

For example: A → N, B → O, N → A, O → B

Your challenge:
Decode this ROT13 encrypted message: "URYYB"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#,
        crossterm::style::Color::White,
    )?;

    let mut attempts = 0;
    loop {
        let answer = ui::read_input("\nDecoded message: ")?;

        if answer.to_uppercase() == "HELLO" {
            ui::print_success("✅ Excellent! You decoded it perfectly!")?;
            ui::print_colored(
                "\n+30 XP (Encoding Mastery)\n",
                crossterm::style::Color::Green,
            )?;
            state.experience += 30;
            break;
        } else {
            attempts += 1;
            if attempts == 1 {
                ui::print_colored(
                    "\n🔍 Remember: ROT13 shifts each letter 13 positions. Try counting!\n",
                    crossterm::style::Color::Yellow,
                )?;
            } else if attempts == 2 {
                ui::print_colored(
                    "\n💡 HINT: U→H, R→E, Y→L, Y→L, B→O\n",
                    crossterm::style::Color::Cyan,
                )?;
            } else {
                ui::print_colored(
                    "\n✨ The answer is 'HELLO'! ROT13 is used in many CTF challenges.\n",
                    crossterm::style::Color::White,
                )?;
                state.experience += 15; // Partial credit
                break;
            }
        }
    }

    ui::print_colored(
        "\n📚 LEARNING: Encoding is fundamental in cybersecurity. Master Base64, ROT13, and hexadecimal!\n",
        crossterm::style::Color::DarkGrey,
    )?;
    ui::pause()?;
    Ok(())
}

fn practice_challenge_web(state: &mut GameState) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n🌐 CHALLENGE 3: Web Security Basics\n",
        crossterm::style::Color::Green,
    )?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
📝 WEB SECURITY CHALLENGE: HTTP Status Codes

HTTP status codes tell us what happened with a web request.
Understanding these is crucial for web security testing.

Question:
What HTTP status code indicates "Unauthorized" access?
(This means you need to authenticate to access the resource)

Common codes to know:
• 200 = OK (success)
• 404 = Not Found
• 500 = Internal Server Error
• ??? = Unauthorized

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#,
        crossterm::style::Color::White,
    )?;

    let mut attempts = 0;
    loop {
        let answer = ui::read_input("\nHTTP status code: ")?;

        if answer == "401" {
            ui::print_success("✅ Perfect! 401 Unauthorized is correct!")?;
            ui::print_colored(
                "\n+35 XP (Web Security Knowledge)\n",
                crossterm::style::Color::Green,
            )?;
            state.experience += 35;
            break;
        } else {
            attempts += 1;
            if attempts == 1 {
                ui::print_colored(
                    "\n🔍 Think about authentication. What 4xx error means you need to log in?\n",
                    crossterm::style::Color::Yellow,
                )?;
            } else if attempts == 2 {
                ui::print_colored(
                    "\n💡 HINT: It's a 400-series error. Think 40_\n",
                    crossterm::style::Color::Cyan,
                )?;
            } else {
                ui::print_colored(
                    "\n✨ The answer is 401! It means 'Unauthorized' - you need valid credentials.\n",
                    crossterm::style::Color::White,
                )?;
                state.experience += 18; // Partial credit
                break;
            }
        }
    }

    ui::print_colored(
        "\n📚 LEARNING: HTTP status codes reveal server behavior. Essential for web penetration testing!\n",
        crossterm::style::Color::DarkGrey,
    )?;
    ui::pause()?;
    Ok(())
}

fn practice_challenge_file_analysis(state: &mut GameState) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n📁 CHALLENGE 4: File Analysis\n",
        crossterm::style::Color::Red,
    )?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
📝 FILE ANALYSIS CHALLENGE: File Signatures

Every file type has a unique "magic number" or signature at the beginning.
This helps identify file types even if the extension is wrong.

Question:
A file starts with the bytes "89 50 4E 47". What file type is this?

Common signatures to know:
• FF D8 FF = JPEG image
• 50 4B 03 04 = ZIP archive
• 25 50 44 46 = PDF document
• 89 50 4E 47 = ???

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#,
        crossterm::style::Color::White,
    )?;

    let mut attempts = 0;
    loop {
        let answer = ui::read_input("\nFile type: ")?;

        if answer.to_uppercase() == "PNG" {
            ui::print_success("✅ Outstanding! PNG is absolutely correct!")?;
            ui::print_colored(
                "\n+40 XP (File Analysis Expert)\n",
                crossterm::style::Color::Green,
            )?;
            state.experience += 40;
            break;
        } else {
            attempts += 1;
            if attempts == 1 {
                ui::print_colored(
                    "\n🔍 This signature belongs to a common image format. Think graphics!\n",
                    crossterm::style::Color::Yellow,
                )?;
            } else if attempts == 2 {
                ui::print_colored(
                    "\n💡 HINT: It's a 3-letter image format that supports transparency: P_G\n",
                    crossterm::style::Color::Cyan,
                )?;
            } else {
                ui::print_colored(
                    "\n✨ The answer is PNG! '89 50 4E 47' translates to '.PNG' in ASCII.\n",
                    crossterm::style::Color::White,
                )?;
                state.experience += 20; // Partial credit
                break;
            }
        }
    }

    ui::print_colored(
        "\n📚 LEARNING: File signatures help identify hidden or disguised files in forensics!\n",
        crossterm::style::Color::DarkGrey,
    )?;

    // Show summary of all practice challenges
    show_practice_summary(state)?;
    Ok(())
}

fn show_practice_summary(state: &GameState) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n🎉 PRACTICE CHALLENGES COMPLETE!\n",
        crossterm::style::Color::Magenta,
    )?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
Congratulations! You've completed all tutorial practice challenges!

📊 YOUR SKILLS PREVIEW:
  ✅ Problem Solving - Critical thinking and research
  ✅ Encoding/Decoding - ROT13, Base64, and cipher techniques
  ✅ Web Security - HTTP status codes and authentication
  ✅ File Analysis - Magic numbers and file identification

These are just the beginning! The real challenges will test these skills
and many more advanced cybersecurity concepts.
"#,
        crossterm::style::Color::White,
    )?;

    ui::print_colored(
        &format!(
            "\n💪 Total Tutorial XP Earned: {} points!\n",
            if state.experience >= 130 {
                130
            } else {
                state.experience
            }
        ),
        crossterm::style::Color::Green,
    )?;

    ui::print_colored(
        "\n🚀 You're now ready for the real Ghost Protocol challenges!\n",
        crossterm::style::Color::Cyan,
    )?;

    ui::pause()?;
    Ok(())
}

fn explain_hints_and_commands() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored("\n⚙️  COMMANDS & HINTS\n", crossterm::style::Color::Cyan)?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
AVAILABLE COMMANDS:

  Navigation:
    • [1-N]   - Select a challenge by number
    • stats   - View your detailed statistics
    • save    - Manually save your progress
    • quit    - Exit the game (auto-saves)

  During Challenges:
    • hint    - Get progressive hints (use multiple times!)
    • skip    - Skip current challenge and try later

  Input Features:
    • ↑/↓     - Navigate command history
    • ←/→     - Move cursor in current input
    • Home/End- Jump to start/end of line

HINT SYSTEM:

Hints become more specific each time you use them:
  1st hint → General guidance
  2nd hint → More specific direction
  3rd hint → Very specific (sometimes the answer!)

Don't be afraid to use hints - learning is the goal! 🎓
"#,
        crossterm::style::Color::White,
    )?;

    ui::print_colored(
        "\n💡 TIP: After wrong answers, you'll get intelligent feedback!\n",
        crossterm::style::Color::Green,
    )?;

    ui::pause()?;
    Ok(())
}

fn show_tutorial_complete() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        r#"
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                   🎉 TUTORIAL COMPLETE! 🎉                               ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
"#,
        crossterm::style::Color::Green,
    )?;

    ui::print_colored(
        r#"
You're now ready to face the Ghost Protocol!

REMEMBER:
  ✓ Use hints when stuck
  ✓ Watch your sanity meter
  ✓ Learn from each challenge
  ✓ Have fun!

The game will now begin. May you survive the protocol...
"#,
        crossterm::style::Color::White,
    )?;

    ui::print_colored(
        "\n                    Good luck, recruit.\n",
        crossterm::style::Color::DarkGrey,
    )?;

    ui::pause()?;
    Ok(())
}

/// Show a quick tooltip for a specific game mechanic
pub fn show_tooltip(topic: &str) -> io::Result<()> {
    match topic {
        "sanity" => {
            ui::print_colored(
                "\n💡 TOOLTIP: Sanity represents your mental state. Each challenge costs sanity.\n",
                crossterm::style::Color::Cyan,
            )?;
            ui::print_colored(
                "   If it reaches 0, the game ends. Choose challenges wisely!\n",
                crossterm::style::Color::White,
            )?;
        }
        "xp" => {
            ui::print_colored(
                "\n💡 TOOLTIP: Experience Points (XP) unlock new challenge levels.\n",
                crossterm::style::Color::Cyan,
            )?;
            ui::print_colored(
                "   Complete challenges to gain XP and progress through the game.\n",
                crossterm::style::Color::White,
            )?;
        }
        "hints" => {
            ui::print_colored(
                "\n💡 TOOLTIP: Type 'hint' during a challenge for help.\n",
                crossterm::style::Color::Cyan,
            )?;
            ui::print_colored(
                "   Hints become more specific each time you use them.\n",
                crossterm::style::Color::White,
            )?;
        }
        "levels" => {
            ui::print_colored(
                "\n💡 TOOLTIP: Challenges are organized by difficulty level (0-4).\n",
                crossterm::style::Color::Cyan,
            )?;
            ui::print_colored(
                "   You unlock new levels by earning experience points.\n",
                crossterm::style::Color::White,
            )?;
        }
        _ => {
            ui::print_colored(
                "\n💡 No tooltip available for that topic.\n",
                crossterm::style::Color::DarkGrey,
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_state_tracking() {
        let mut state = GameState::new("TestPlayer".to_string());

        assert!(state.needs_tutorial());
        assert!(!state.tutorial_completed);

        state.mark_tutorial_completed();

        assert!(!state.needs_tutorial());
        assert!(state.tutorial_completed);
    }

    #[test]
    fn test_tutorial_not_needed_if_challenges_completed() {
        let mut state = GameState::new("TestPlayer".to_string());
        state.complete_challenge("welcome", 50);

        // Even if tutorial not completed, don't show if they have progress
        assert!(!state.needs_tutorial());
    }

    #[test]
    fn test_tutorial_xp_rewards() {
        let mut state = GameState::new("TestPlayer".to_string());
        let initial_xp = state.experience;

        // Simulate completing all tutorial challenges with full credit
        state.experience += 25; // Basic challenge
        state.experience += 30; // Encoding challenge
        state.experience += 35; // Web security challenge
        state.experience += 40; // File analysis challenge

        let total_tutorial_xp = state.experience - initial_xp;
        assert_eq!(total_tutorial_xp, 130, "Tutorial should award 130 XP total");
    }
}
