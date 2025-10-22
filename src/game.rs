use crate::{challenges, narrative, state::GameState, ui};
use std::io;

pub fn run_game() -> io::Result<()> {
    // Try to load existing save or start new game
    let mut state = match GameState::load() {
        Ok(saved_state) => {
            ui::print_horror_banner()?;
            ui::print_colored(
                &format!("\n\nWelcome back, {}...\n", saved_state.player_name),
                crossterm::style::Color::Yellow,
            )?;
            ui::print_colored(
                "Your saved progress has been loaded.\n",
                crossterm::style::Color::White,
            )?;
            ui::pause()?;
            saved_state
        }
        Err(_) => {
            // New game
            ui::print_horror_banner()?;
            ui::print_colored(
                "\n\nBefore we begin, what should we call you?\n",
                crossterm::style::Color::White,
            )?;
            let name = ui::read_input("Enter your name: ")?;
            let state = GameState::new(name);
            narrative::show_intro(&state.player_name)?;
            state
        }
    };

    // Main game loop
    loop {
        // Show current level
        narrative::show_level_transition(state.current_level, state.sanity)?;

        // Get challenges for current level
        let level_challenges = challenges::get_challenges_for_level(state.current_level);

        if level_challenges.is_empty() {
            // Check if player completed all challenges
            let all_challenges = challenges::get_all_challenges();
            let completed_all = all_challenges.iter().all(|c| state.has_completed(&c.id));

            if completed_all {
                // Game complete!
                narrative::show_ending(state.discovered_secrets.len())?;
                break;
            } else {
                // Move to next level
                state.current_level += 1;
                state.save()?;
                continue;
            }
        }

        // Show available challenges
        ui::clear_screen()?;

        // Main header
        ui::print_colored(
            "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
            crossterm::style::Color::Cyan,
        )?;
        ui::print_colored(
            &format!(
                "║{} LEVEL {} - CHALLENGE SELECTION {}║\n",
                " ".repeat(20),
                state.current_level,
                " ".repeat(20)
            ),
            crossterm::style::Color::Yellow,
        )?;
        ui::print_colored(
            "╚═══════════════════════════════════════════════════════════════════════════╝\n",
            crossterm::style::Color::Cyan,
        )?;

        // Stats section
        println!();
        ui::print_progress_bar(state.sanity, 100, "Sanity")?;
        ui::print_colored(
            &format!(
                "Experience: {} XP  │  Level: {}  │  Challenges: {}/{}\n",
                state.experience,
                state.current_level,
                state.completed_challenges.len(),
                challenges::get_all_challenges().len()
            ),
            crossterm::style::Color::White,
        )?;

        ui::print_separator()?;

        println!("\n📋 AVAILABLE CHALLENGES:\n");
        for (idx, challenge) in level_challenges.iter().enumerate() {
            let (status, status_color) = if state.has_completed(&challenge.id) {
                ("✓ COMPLETED", crossterm::style::Color::Green)
            } else {
                ("○ Available", crossterm::style::Color::Yellow)
            };

            print!("  ");
            ui::print_colored(&format!("[{}]", idx + 1), crossterm::style::Color::Cyan)?;
            print!(" {} ", challenge.title);
            ui::print_colored(status, status_color)?;
            ui::print_colored(
                &format!(
                    " (+{} XP, -{} sanity)",
                    challenge.xp_reward, challenge.sanity_cost
                ),
                crossterm::style::Color::DarkGrey,
            )?;
            println!();
        }

        ui::print_separator()?;
        println!("\n⚙️  OPTIONS:\n");
        ui::print_menu_option("1-N", "Select a challenge by number", None)?;
        ui::print_menu_option("stats", "View detailed statistics", None)?;
        ui::print_menu_option("save", "Save your progress", None)?;
        ui::print_menu_option("quit", "Exit the Ghost Protocol", None)?;

        let choice = ui::read_input("\n> Enter your choice: ")?;

        match choice.to_lowercase().as_str() {
            "stats" => show_stats(&state)?,
            "save" => {
                state.save()?;
                ui::print_success("Game saved successfully!")?;
                ui::pause()?;
            }
            "quit" => {
                state.save()?;
                ui::print_colored(
                    "\n\nThe Ghost Protocol awaits your return...\n",
                    crossterm::style::Color::Red,
                )?;
                ui::pause()?;
                break;
            }
            _ => {
                // Try to parse as challenge number
                if let Ok(num) = choice.parse::<usize>() {
                    if num > 0 && num <= level_challenges.len() {
                        let challenge = &level_challenges[num - 1];

                        if state.has_completed(&challenge.id) {
                            ui::print_warning("You've already completed this challenge.")?;
                            ui::pause()?;
                        } else {
                            // Attempt the challenge
                            challenge.attempt(&mut state)?;
                        }
                    } else {
                        ui::print_error("Invalid challenge number.")?;
                        ui::pause()?;
                    }
                } else {
                    ui::print_error("Invalid input.")?;
                    ui::pause()?;
                }
            }
        }

        // Check if player's sanity is too low
        if state.sanity <= 0 {
            ui::clear_screen()?;
            ui::print_colored(
                r#"
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                            SANITY DEPLETED                               ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

Your mind fractures. The ghosts have claimed another victim.

The last thing you see is the screen flickering:

    "YOU CANNOT ESCAPE..."
    "YOU ARE ONE OF US NOW..."

GAME OVER
"#,
                crossterm::style::Color::Red,
            )?;
            ui::pause()?;
            break;
        }
    }

    Ok(())
}

fn show_stats(state: &GameState) -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
        crossterm::style::Color::Magenta,
    )?;
    ui::print_colored(
        &format!(
            "║{} 📊 PLAYER STATISTICS {}║\n",
            " ".repeat(25),
            " ".repeat(25)
        ),
        crossterm::style::Color::Yellow,
    )?;
    ui::print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n",
        crossterm::style::Color::Magenta,
    )?;

    println!();
    ui::print_colored(
        &format!("👤 Player: {}\n", state.player_name),
        crossterm::style::Color::Cyan,
    )?;
    ui::print_colored(
        &format!("⚡ Current Level: {}\n", state.current_level),
        crossterm::style::Color::Yellow,
    )?;
    ui::print_colored(
        &format!("🌟 Experience: {} XP\n", state.experience),
        crossterm::style::Color::Green,
    )?;

    println!();
    ui::print_progress_bar(state.sanity, 100, "🧠 Sanity")?;

    let all_challenges = challenges::get_all_challenges();
    ui::print_progress_bar(
        state.completed_challenges.len() as i32,
        all_challenges.len() as i32,
        "✓  Challenges Completed",
    )?;

    ui::print_separator()?;

    println!("\n📈 PROGRESS BREAKDOWN:\n");
    for level in 0..=4 {
        let level_challenges = challenges::get_challenges_for_level(level);
        if level_challenges.is_empty() {
            continue;
        }

        let completed = level_challenges
            .iter()
            .filter(|c| state.has_completed(&c.id))
            .count();
        let total = level_challenges.len();
        let percentage = (completed as f32 / total as f32 * 100.0) as i32;

        let color = if completed == total {
            crossterm::style::Color::Green
        } else if completed > 0 {
            crossterm::style::Color::Yellow
        } else {
            crossterm::style::Color::DarkGrey
        };

        let bars = "█".repeat((completed as f32 / total as f32 * 10.0) as usize);
        let empty = "░".repeat(10 - (completed as f32 / total as f32 * 10.0) as usize);

        ui::print_colored(
            &format!("  Level {}: ", level),
            crossterm::style::Color::White,
        )?;
        ui::print_colored(&format!("[{}{}]", bars, empty), color)?;
        ui::print_colored(
            &format!(" {}/{} ({}%)\n", completed, total, percentage),
            color,
        )?;
    }

    ui::print_separator()?;

    if !state.completed_challenges.is_empty() {
        println!("\n🏆 COMPLETED CHALLENGES:\n");
        for challenge_id in &state.completed_challenges {
            if let Some(challenge) = all_challenges.iter().find(|c| &c.id == challenge_id) {
                ui::print_colored("  ✓ ", crossterm::style::Color::Green)?;
                println!("{} (+{} XP)", challenge.title, challenge.xp_reward);
            }
        }
    } else {
        println!("\n💀 No challenges completed yet...\n");
    }

    ui::print_separator()?;
    println!("Sanity: {}%", state.sanity);
    println!("Challenges Completed: {}", state.completed_challenges.len());
    println!("Secrets Discovered: {}", state.discovered_secrets.len());

    if !state.completed_challenges.is_empty() {
        println!("\nCompleted Challenges:");
        for challenge_id in &state.completed_challenges {
            println!("  ✓ {}", challenge_id);
        }
    }

    if !state.discovered_secrets.is_empty() {
        println!("\nDiscovered Secrets:");
        for secret in &state.discovered_secrets {
            println!("  🔍 {}", secret);
        }
    }

    ui::pause()?;
    Ok(())
}
