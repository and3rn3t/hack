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
            let completed_all = all_challenges
                .iter()
                .all(|c| state.has_completed(&c.id));

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
        ui::print_colored(
            &format!("\n╔════════════════════════════════════════════════════════╗\n"),
            crossterm::style::Color::Cyan,
        )?;
        ui::print_colored(
            &format!("║  Level {} - Available Challenges                      ║\n", state.current_level),
            crossterm::style::Color::Cyan,
        )?;
        ui::print_colored(
            &format!("╚════════════════════════════════════════════════════════╝\n"),
            crossterm::style::Color::Cyan,
        )?;

        narrative::show_sanity_meter(state.sanity)?;
        ui::print_colored(
            &format!("\nExperience: {} XP | Level: {}\n", state.experience, state.current_level),
            crossterm::style::Color::White,
        )?;

        println!("\nChallenges:");
        for (idx, challenge) in level_challenges.iter().enumerate() {
            let status = if state.has_completed(&challenge.id) {
                "✓ COMPLETED"
            } else {
                "○ Available"
            };
            println!("  {}. {} - {} ({})", idx + 1, challenge.title, status, challenge.xp_reward);
        }

        println!("\nOptions:");
        println!("  [1-{}] - Select a challenge", level_challenges.len());
        println!("  [stats] - View detailed statistics");
        println!("  [save] - Save game");
        println!("  [quit] - Quit game");

        let choice = ui::read_input("\nYour choice: ")?;

        match choice.to_lowercase().as_str() {
            "stats" => show_stats(&state)?,
            "save" => {
                state.save()?;
                ui::print_success("Game saved successfully!")?;
                ui::pause()?;
            }
            "quit" => {
                state.save()?;
                ui::print_colored("\n\nThe Ghost Protocol awaits your return...\n", crossterm::style::Color::Red)?;
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
        "\n╔════════════════════════════════════════════════════════╗\n",
        crossterm::style::Color::Cyan,
    )?;
    ui::print_colored(
        "║                  Player Statistics                    ║\n",
        crossterm::style::Color::Cyan,
    )?;
    ui::print_colored(
        "╚════════════════════════════════════════════════════════╝\n",
        crossterm::style::Color::Cyan,
    )?;

    println!("\nPlayer: {}", state.player_name);
    println!("Current Level: {}", state.current_level);
    println!("Experience: {} XP", state.experience);
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
