use crate::ui::CompletionContext;
use crate::ui::{
    theme_accent, theme_border, theme_error, theme_muted, theme_primary, theme_success,
    theme_warning,
};
use crate::{challenges, narrative, state::GameState, tutorial, ui};
use std::io;

pub fn run_game() -> io::Result<()> {
    // Try to load existing save or start new game
    let mut state = match GameState::load() {
        Ok(saved_state) => {
            ui::print_horror_banner()?;
            ui::print_colored(
                &format!("\n\nWelcome back, {}...\n", saved_state.player_name),
                theme_accent(),
            )?;
            ui::print_colored("Your saved progress has been loaded.\n", theme_primary())?;
            ui::pause()?;
            saved_state
        }
        Err(_) => {
            // New game
            ui::print_horror_banner()?;
            ui::print_colored(
                "\n\nBefore we begin, what should we call you?\n",
                theme_primary(),
            )?;
            let name = ui::read_input("Enter your name: ")?;
            let mut state = GameState::new(name);

            // Offer tutorial for new players
            if state.needs_tutorial() {
                ui::print_colored(
                    "\n\nWould you like to play the interactive tutorial?\n",
                    theme_accent(),
                )?;
                ui::print_colored("(Recommended for first-time players)\n", theme_muted())?;

                let choice = ui::read_input("[Y/n]: ")?;
                if choice.is_empty() || choice.to_lowercase().starts_with('y') {
                    tutorial::run_tutorial(&mut state)?;
                } else {
                    ui::print_colored(
                        "\nSkipping tutorial. You can review game mechanics in the README.\n",
                        theme_warning(),
                    )?;
                    state.mark_tutorial_completed();
                    state.save()?;
                    ui::pause()?;
                }
            }

            narrative::show_intro(&state.player_name)?;
            state
        }
    };

    // Main game loop
    loop {
        // Random jumpscare based on sanity level (lower sanity = more likely)
        let jumpscare_probability = (100.0 - state.sanity as f64) / 500.0; // 0-20% chance
        ui::random_jumpscare(jumpscare_probability)?;

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

        // Occasional subtle scare when viewing menu
        ui::subtle_jumpscare()?;

        // Main header
        ui::print_colored(
            "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
            theme_border(),
        )?;
        ui::print_colored(
            &format!(
                "║{} LEVEL {} - CHALLENGE SELECTION {}║\n",
                " ".repeat(20),
                state.current_level,
                " ".repeat(20)
            ),
            theme_accent(),
        )?;
        ui::print_colored(
            "╚═══════════════════════════════════════════════════════════════════════════╝\n",
            theme_border(),
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
            theme_primary(),
        )?;

        ui::print_separator()?;

        println!("\n📋 AVAILABLE CHALLENGES:\n");
        for (idx, challenge) in level_challenges.iter().enumerate() {
            let (status, status_color) = if state.has_completed(&challenge.id) {
                ("✓ COMPLETED", theme_success())
            } else {
                ("○ Available", theme_accent())
            };

            print!("  ");
            ui::print_colored(&format!("[{}]", idx + 1), theme_accent())?;
            print!(" {} ", challenge.title);
            ui::print_colored(status, status_color)?;
            ui::print_colored(
                &format!(
                    " (+{} XP, -{} sanity)",
                    challenge.xp_reward, challenge.sanity_cost
                ),
                theme_muted(),
            )?;
            println!();
        }

        ui::print_separator()?;
        println!("\n⚙️  OPTIONS:\n");
        ui::print_menu_option("1-N", "Select a challenge by number", None)?;
        ui::print_menu_option("stats", "View detailed statistics", None)?;
        ui::print_menu_option(
            "practice",
            "Practice with dynamic challenges (v1.2.0)",
            None,
        )?;
        ui::print_menu_option("help", "Show available tooltips", None)?;
        ui::print_menu_option("theme", "Change color theme", None)?;
        ui::print_menu_option("settings", "Configure preferences (v1.2.0)", None)?;
        ui::print_menu_option("alias", "Manage command aliases (v1.2.0)", None)?;
        ui::print_menu_option("save", "Save your progress", None)?;
        ui::print_menu_option("quit", "Exit the Ghost Protocol", None)?;

        // Create completion context with available challenges
        let completion_context = CompletionContext::MainMenu {
            challenge_count: level_challenges.len(),
        };
        let raw_choice =
            ui::read_input_with_completion("\n> Enter your choice: ", completion_context, true)?;

        // v1.2.0: Resolve command through user aliases
        let choice = state.resolve_command(&raw_choice);

        match choice.to_lowercase().as_str() {
            "stats" => show_stats(&state)?,
            "practice" | "dynamic" => {
                show_practice_mode(&mut state)?;
            }
            "help" | "tutorial" | "?" => show_help()?,
            "theme" | "themes" => ui::show_theme_selection()?,
            // v1.2.0: New settings menu
            "settings" | "preferences" | "config" => {
                ui::show_settings_menu(&mut state)?;
                state.save()?; // Auto-save preferences
            }
            // v1.2.0: Alias management
            "alias" | "aliases" => {
                ui::show_alias_menu(&mut state)?;
                state.save()?; // Auto-save aliases
            }
            // v1.2.0: Enhanced save menu
            "save" => {
                ui::show_save_slot_menu(&mut state)?;
            }
            "quit" => {
                state.save()?;
                ui::print_colored(
                    "\n\nThe Ghost Protocol awaits your return...\n",
                    theme_error(),
                )?;
                ui::pause()?;
                break;
            }
            _ => {
                // Try to parse as challenge number
                if let Ok(num) = choice.parse::<usize>() {
                    if num > 0 && num <= level_challenges.len() {
                        let base_challenge = &level_challenges[num - 1];

                        if state.has_completed(&base_challenge.id) {
                            ui::print_warning("You've already completed this challenge.")?;

                            // v1.2.0: Offer to replay with different difficulty
                            if base_challenge.has_variants() {
                                ui::print_info(
                                    "Would you like to try a different difficulty? [y/N]: ",
                                )?;
                                let replay = ui::read_input("")?;
                                if replay.to_lowercase().starts_with('y') {
                                    if let Some(difficulty) =
                                        ui::show_challenge_difficulty_menu(base_challenge)?
                                    {
                                        let challenge_variant =
                                            base_challenge.with_difficulty(difficulty);
                                        challenge_variant.attempt(&mut state)?;
                                    }
                                }
                            } else {
                                ui::pause()?;
                            }
                        } else {
                            // v1.2.0: Show difficulty selection for new challenges
                            let difficulty = if base_challenge.has_variants() {
                                // Check user preference for automatic difficulty
                                match state.get_difficulty_scaling() {
                                    crate::state::DifficultyScaling::Adaptive => {
                                        // Use adaptive difficulty based on performance
                                        select_adaptive_difficulty(&state, base_challenge)
                                    }
                                    crate::state::DifficultyScaling::Static => {
                                        // Always use standard difficulty
                                        challenges::ChallengeDifficulty::Standard
                                    }
                                    crate::state::DifficultyScaling::Custom => {
                                        // Let user choose each time
                                        if let Some(chosen) =
                                            ui::show_challenge_difficulty_menu(base_challenge)?
                                        {
                                            chosen
                                        } else {
                                            continue; // User cancelled
                                        }
                                    }
                                }
                            } else {
                                challenges::ChallengeDifficulty::Standard
                            };

                            // Create challenge with selected difficulty
                            let challenge = base_challenge.with_difficulty(difficulty);

                            // Record challenge attempt
                            state.record_challenge_attempt(&challenge.id);

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
            // Final jumpscare before game over
            ui::random_jumpscare(1.0)?;
            std::thread::sleep(std::time::Duration::from_millis(500));

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
                theme_error(),
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
        theme_border(),
    )?;
    ui::print_colored(
        &format!(
            "║{} 📊 PLAYER STATISTICS {}║\n",
            " ".repeat(25),
            " ".repeat(25)
        ),
        theme_accent(),
    )?;
    ui::print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n",
        theme_border(),
    )?;

    println!();
    ui::print_colored(
        &format!("👤 Player: {}\n", state.player_name),
        theme_accent(),
    )?;
    ui::print_colored(
        &format!("⚡ Current Level: {}\n", state.current_level),
        theme_warning(),
    )?;
    ui::print_colored(
        &format!("🌟 Experience: {} XP\n", state.experience),
        theme_success(),
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
            theme_success()
        } else if completed > 0 {
            theme_warning()
        } else {
            theme_muted()
        };

        let bars = "█".repeat((completed as f32 / total as f32 * 10.0) as usize);
        let empty = "░".repeat(10 - (completed as f32 / total as f32 * 10.0) as usize);

        ui::print_colored(&format!("  Level {}: ", level), theme_primary())?;
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
                ui::print_colored("  ✓ ", theme_success())?;
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

fn show_help() -> io::Result<()> {
    ui::clear_screen()?;

    ui::print_colored(
        "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
        theme_border(),
    )?;
    ui::print_colored(
        "║                          📚 HELP & TOOLTIPS 📚                           ║\n",
        theme_accent(),
    )?;
    ui::print_colored(
        "╚═══════════════════════════════════════════════════════════════════════════╝\n",
        theme_border(),
    )?;

    println!("\nSelect a topic to learn more:\n");
    ui::print_menu_option("1", "Sanity System", None)?;
    ui::print_menu_option("2", "Experience & Levels", None)?;
    ui::print_menu_option("3", "Hint System", None)?;
    ui::print_menu_option("4", "Challenge Levels", None)?;
    ui::print_menu_option("5", "All Commands", None)?;
    ui::print_menu_option("back", "Return to game", None)?;

    let choice = ui::read_input_with_completion("\n> Topic: ", CompletionContext::HelpMenu, true)?;

    match choice.as_str() {
        "1" => {
            tutorial::show_tooltip("sanity")?;
            ui::pause()?;
            show_help()?; // Recursive for multiple lookups
        }
        "2" => {
            tutorial::show_tooltip("xp")?;
            ui::pause()?;
            show_help()?;
        }
        "3" => {
            tutorial::show_tooltip("hints")?;
            ui::pause()?;
            show_help()?;
        }
        "4" => {
            tutorial::show_tooltip("levels")?;
            ui::pause()?;
            show_help()?;
        }
        "5" => {
            show_all_commands()?;
            ui::pause()?;
            show_help()?;
        }
        _ => {} // Return to game
    }

    Ok(())
}

fn show_all_commands() -> io::Result<()> {
    ui::print_colored("\n⚙️  ALL AVAILABLE COMMANDS\n", theme_accent())?;
    ui::print_separator()?;

    ui::print_colored(
        r#"
MAIN MENU:
  [1-N]  → Select challenge by number
  stats  → View detailed statistics
  help   → Show this help menu
  save   → Manually save progress
  quit   → Exit game (auto-saves)

DURING CHALLENGES:
  hint   → Get progressive hints
  skip   → Skip current challenge

NAVIGATION (Input):
  ↑/↓    → Navigate command history
  ←/→    → Move cursor within input
  Home   → Jump to start of line
  End    → Jump to end of line

TIPS:
  • Use hints liberally - learning is the goal!
  • Watch your sanity meter carefully
  • Failed challenges cost extra sanity
  • Save often (or it auto-saves after each challenge)
"#,
        theme_primary(),
    )?;

    Ok(())
}

/// Select adaptive difficulty based on player performance (v1.2.0)
fn select_adaptive_difficulty(
    state: &GameState,
    challenge: &challenges::Challenge,
) -> challenges::ChallengeDifficulty {
    let completion_rate = state.get_completion_rate();
    let current_level = state.current_level;
    let sanity_percentage = state.sanity as f32 / 100.0;

    // Calculate adaptive difficulty score
    let mut score = 0.0;

    // High completion rate suggests player is doing well
    if completion_rate > 80.0 {
        score += 0.3;
    } else if completion_rate > 60.0 {
        score += 0.1;
    } else if completion_rate < 40.0 {
        score -= 0.2;
    }

    // High sanity suggests challenges aren't too difficult
    if sanity_percentage > 0.75 {
        score += 0.2;
    } else if sanity_percentage < 0.5 {
        score -= 0.3;
    }

    // Higher levels can handle more difficulty
    score += (current_level as f32) * 0.1;

    // Check if player has attempted this challenge before
    if let Some(attempts) = state
        .progress_analytics
        .challenges_attempted
        .get(&challenge.id)
    {
        if *attempts > 3 {
            score -= 0.2; // Multiple attempts suggest difficulty
        }
    }

    // Determine difficulty based on score
    if score > 0.4
        && challenge
            .get_available_difficulties()
            .contains(&challenges::ChallengeDifficulty::Advanced)
    {
        challenges::ChallengeDifficulty::Advanced
    } else if score < -0.3
        && challenge
            .get_available_difficulties()
            .contains(&challenges::ChallengeDifficulty::Beginner)
    {
        challenges::ChallengeDifficulty::Beginner
    } else {
        challenges::ChallengeDifficulty::Standard
    }
}

/// Show practice mode with dynamic challenges (v1.2.0)
fn show_practice_mode(state: &mut GameState) -> io::Result<()> {
    loop {
        ui::clear_screen()?;

        ui::print_colored(
            "\n╔═══════════════════════════════════════════════════════════════════════════╗\n",
            theme_border(),
        )?;
        ui::print_colored(
            "║                            PRACTICE MODE                                 ║\n",
            theme_accent(),
        )?;
        ui::print_colored(
            "╚═══════════════════════════════════════════════════════════════════════════╝\n",
            theme_border(),
        )?;

        println!();
        ui::print_colored(
            "🎯 Practice with randomly generated challenges!\n",
            theme_primary(),
        )?;
        ui::print_colored(
            "These don't affect your main progress but help build skills.\n\n",
            theme_muted(),
        )?;

        let dynamic_challenges = challenges::get_dynamic_challenges();

        println!("📋 PRACTICE CHALLENGES:\n");
        for (idx, challenge) in dynamic_challenges.iter().enumerate() {
            print!("  ");
            ui::print_colored(&format!("[{}]", idx + 1), theme_accent())?;
            print!(" {} ", challenge.title);
            ui::print_colored(
                &format!(
                    "(+{} XP, -{} sanity)",
                    challenge.xp_reward, challenge.sanity_cost
                ),
                theme_muted(),
            )?;
            println!();
        }

        ui::print_separator()?;
        println!("\n⚙️  OPTIONS:\n");
        ui::print_menu_option("1-N", "Select a practice challenge", None)?;
        ui::print_menu_option("random", "Random challenge", None)?;
        ui::print_menu_option("back", "Return to main menu", None)?;

        let choice = ui::read_input_with_completion(
            "\n> Select practice challenge: ",
            ui::CompletionContext::MainMenu {
                challenge_count: dynamic_challenges.len(),
            },
            true,
        )?;

        match choice.to_lowercase().as_str() {
            "back" => break,
            "random" => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..dynamic_challenges.len());
                let challenge = &dynamic_challenges[idx];

                ui::print_info(&format!("Randomly selected: {}", challenge.title))?;
                ui::pause()?;

                // Practice challenges don't affect main progress but give small XP
                let practice_result = challenge.attempt(state)?;
                if practice_result {
                    // Give bonus XP for practice
                    state.add_xp(challenge.xp_reward / 2);
                    ui::print_success(&format!(
                        "Practice bonus: +{} XP!",
                        challenge.xp_reward / 2
                    ))?;
                }
                ui::pause()?;
            }
            _ => {
                if let Ok(num) = choice.parse::<usize>() {
                    if num > 0 && num <= dynamic_challenges.len() {
                        let challenge = &dynamic_challenges[num - 1];

                        ui::print_info(&format!("Starting practice: {}", challenge.title))?;
                        ui::pause()?;

                        // Practice challenges don't affect main progress but give small XP
                        let practice_result = challenge.attempt(state)?;
                        if practice_result {
                            // Give bonus XP for practice
                            state.add_xp(challenge.xp_reward / 2);
                            ui::print_success(&format!(
                                "Practice bonus: +{} XP!",
                                challenge.xp_reward / 2
                            ))?;
                        }
                        ui::pause()?;
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
    }

    Ok(())
}
