use crate::ui;
use crate::ui::theme_border;
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

    // Chance of jumpscare during level transition
    if sanity < 50 {
        ui::random_jumpscare(0.25)?;
    }

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
        theme_border(),
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

    // Small chance of jumpscare even on success
    ui::random_jumpscare(0.05)?;

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

    use rand::prelude::IndexedRandom;
    let mut rng = rand::rng();
    if let Some(msg) = messages.choose(&mut rng) {
        ui::print_colored(&format!("💀 {}\n", msg), Color::Magenta)?;
    }

    println!();
    Ok(())
}

pub fn show_ending(secrets_found: usize) -> io::Result<()> {
    ui::clear_screen()?;

    // Epic jumpscare sequence for the ending
    ui::random_jumpscare(0.8)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    ui::random_jumpscare(0.8)?;
    std::thread::sleep(std::time::Duration::from_millis(300));

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

// ============================================================================
// Branching Narrative System (v1.6.0)
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a choice in a branching narrative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeChoice {
    pub text: String,
    pub response: String,
    pub branch_id: String,
    pub effects: ChoiceEffects,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<ChoiceRequirements>,
}

/// Effects that a choice has on gameplay
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChoiceEffects {
    pub sanity_change: i32,
    pub xp_change: i32,
    #[serde(default)]
    pub unlock_challenges: Vec<String>,
    #[serde(default)]
    pub story_flags: Vec<String>,
}

/// Requirements for a choice to be available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_challenges: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_flags: Option<Vec<String>>,
}

/// Represents a branching point in the narrative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeBranch {
    pub id: String,
    pub title: String,
    pub description: String,
    pub trigger_level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_sanity: Option<u32>,
    pub choices: Vec<NarrativeChoice>,
    pub default_branch: String,
}

/// Manages the branching narrative system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingNarrative {
    branches: HashMap<String, NarrativeBranch>,
    active_branch: String,
    story_flags: HashSet<String>,
    choice_history: Vec<String>,
}

impl Default for BranchingNarrative {
    fn default() -> Self {
        Self::new()
    }
}

impl BranchingNarrative {
    pub fn new() -> Self {
        let mut narrative = Self {
            branches: HashMap::new(),
            active_branch: "main".to_string(),
            story_flags: HashSet::new(),
            choice_history: Vec::new(),
        };
        narrative.initialize_branches();
        narrative
    }

    fn initialize_branches(&mut self) {
        // Branch 1: First Ghost Encounter (Level 1)
        self.branches.insert(
            "ghost_encounter_1".to_string(),
            NarrativeBranch {
                id: "ghost_encounter_1".to_string(),
                title: "The Ghost Appears".to_string(),
                description: "A spectral figure materializes before you, its form flickering like corrupted data. It seems to want something from you...".to_string(),
                trigger_level: 1,
                trigger_sanity: None,
                choices: vec![
                    NarrativeChoice {
                        text: "Ask the ghost who they are".to_string(),
                        response: "The ghost's eyes glow as it whispers: 'I am what remains... what you will become... We were hackers once, just like you. Now we're bound to this system forever.'".to_string(),
                        branch_id: "ghost_reveal_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -5,
                            xp_change: 10,
                            unlock_challenges: vec![],
                            story_flags: vec!["ghost_questioned".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Ignore the ghost and continue hacking".to_string(),
                        response: "You focus on the terminal, refusing to acknowledge the apparition. The ghost grows silent... for now. But you can feel its eyes watching your every keystroke.".to_string(),
                        branch_id: "focused_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 0,
                            xp_change: 5,
                            unlock_challenges: vec![],
                            story_flags: vec!["ghost_ignored".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Challenge the ghost to prove it's real".to_string(),
                        response: "The ghost laughs, a sound like static. Suddenly, your screen flickers and displays your deepest fears. Real enough for you?".to_string(),
                        branch_id: "skeptic_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -10,
                            xp_change: 15,
                            unlock_challenges: vec![],
                            story_flags: vec!["ghost_challenged".to_string()],
                        },
                        requirements: None,
                    },
                ],
                default_branch: "main".to_string(),
            },
        );

        // Branch 2: Sanity Crisis (Level 2, when sanity < 50)
        self.branches.insert(
            "sanity_crisis".to_string(),
            NarrativeBranch {
                id: "sanity_crisis".to_string(),
                title: "The Breaking Point".to_string(),
                description: "Your mind fractures under the weight of the dark knowledge you've uncovered. The line between reality and the digital realm blurs...".to_string(),
                trigger_level: 2,
                trigger_sanity: Some(50),
                choices: vec![
                    NarrativeChoice {
                        text: "Take a break and restore sanity (+10 sanity)".to_string(),
                        response: "You step back from the terminal... breathe... The ghosts' laughter echoes in your mind, but you push through. Clarity returns, if only temporarily.".to_string(),
                        branch_id: "recovery_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 10,
                            xp_change: 0,
                            unlock_challenges: vec![],
                            story_flags: vec!["took_break".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Embrace the madness (-5 sanity, unlock hidden challenge)".to_string(),
                        response: "You dive deeper into the chaos. The voices grow louder, but so does your understanding. New paths become visible in the darkness... paths others fear to tread.".to_string(),
                        branch_id: "madness_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -5,
                            xp_change: 50,
                            unlock_challenges: vec!["hidden_madness".to_string()],
                            story_flags: vec!["embraced_madness".to_string()],
                        },
                        requirements: None,
                    },
                ],
                default_branch: "main".to_string(),
            },
        );

        // Branch 3: Ghost's Bargain (Level 3)
        self.branches.insert(
            "ghost_bargain".to_string(),
            NarrativeBranch {
                id: "ghost_bargain".to_string(),
                title: "A Dark Offer".to_string(),
                description: "The ghost offers you a deal: 'I can help you solve these challenges... but every shortcut has a price. What's a little corruption between friends?'".to_string(),
                trigger_level: 3,
                trigger_sanity: None,
                choices: vec![
                    NarrativeChoice {
                        text: "Accept the ghost's help (get hints, lose independence)".to_string(),
                        response: "The ghost's cold touch guides your hands. Solutions come easier now... but you notice your code taking on strange patterns. Are these your solutions, or its?".to_string(),
                        branch_id: "corrupted_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -15,
                            xp_change: 30,
                            unlock_challenges: vec![],
                            story_flags: vec!["accepted_bargain".to_string(), "corrupted".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Refuse the offer (maintain independence)".to_string(),
                        response: "You reject the ghost's offer. It hisses in anger but backs away. The challenges remain difficult, but your solutions are truly yours. The ghost respects strength.".to_string(),
                        branch_id: "independent_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 0,
                            xp_change: 10,
                            unlock_challenges: vec![],
                            story_flags: vec!["refused_bargain".to_string(), "independent".to_string()],
                        },
                        requirements: None,
                    },
                ],
                default_branch: "main".to_string(),
            },
        );

        // Branch 4: The Truth Glimpsed (Level 3, after 30+ challenges)
        self.branches.insert(
            "truth_glimpse".to_string(),
            NarrativeBranch {
                id: "truth_glimpse".to_string(),
                title: "Fragments of Truth".to_string(),
                description: "You find a corrupted log file. It contains usernames of previous hackers... and yours is already on the list. How is that possible?".to_string(),
                trigger_level: 3,
                trigger_sanity: None,
                choices: vec![
                    NarrativeChoice {
                        text: "Investigate the anomaly further".to_string(),
                        response: "You dig deeper into the logs. The timestamps are wrong. The entries for 'you' date back years. A horrifying realization dawns: you've been here before. Or have you?".to_string(),
                        branch_id: "truth_seeker_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -10,
                            xp_change: 40,
                            unlock_challenges: vec![],
                            story_flags: vec!["truth_seeker".to_string(), "investigated_logs".to_string()],
                        },
                        requirements: Some(ChoiceRequirements {
                            min_level: Some(3),
                            required_challenges: None,
                            required_flags: None,
                        }),
                    },
                    NarrativeChoice {
                        text: "Delete the file and move on".to_string(),
                        response: "You delete the corrupted file. Some truths are better left undiscovered. But as you work, you can't shake the feeling that you're forgetting something important...".to_string(),
                        branch_id: "denial_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 5,
                            xp_change: 0,
                            unlock_challenges: vec![],
                            story_flags: vec!["in_denial".to_string()],
                        },
                        requirements: None,
                    },
                ],
                default_branch: "main".to_string(),
            },
        );

        // Branch 5: The Final Choice (Level 4)
        self.branches.insert(
            "final_choice".to_string(),
            NarrativeBranch {
                id: "final_choice".to_string(),
                title: "The Point of No Return".to_string(),
                description: "The ghost reveals the truth: you died years ago. This is your prison. You can accept your fate and become a guide for the next victim, or you can fight for escape... knowing it might destroy everything.".to_string(),
                trigger_level: 4,
                trigger_sanity: None,
                choices: vec![
                    NarrativeChoice {
                        text: "Accept your fate and become a guide ghost".to_string(),
                        response: "You embrace your new existence. The pain fades. You'll help the next hacker... just as those before you helped. The cycle continues, but perhaps that's not so bad.".to_string(),
                        branch_id: "acceptance_ending".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 0,
                            xp_change: 100,
                            unlock_challenges: vec![],
                            story_flags: vec!["accepted_fate".to_string(), "ghost_ending".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Fight for escape, no matter the cost".to_string(),
                        response: "You refuse to accept this fate. You'll break the system itself if you must. The ghosts scream in terror as you begin tearing apart the very code that binds them. What happens next is uncertain...".to_string(),
                        branch_id: "rebellion_ending".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -20,
                            xp_change: 200,
                            unlock_challenges: vec![],
                            story_flags: vec!["fought_fate".to_string(), "rebellion_ending".to_string()],
                        },
                        requirements: Some(ChoiceRequirements {
                            min_level: Some(4),
                            required_challenges: None,
                            required_flags: Some(vec!["independent".to_string()]),
                        }),
                    },
                    NarrativeChoice {
                        text: "Attempt to free all the trapped souls together".to_string(),
                        response: "Perhaps there's another way. If you're trapped here, so are the others. Together, you might have the strength to break free. The ghosts look at you with hope for the first time...".to_string(),
                        branch_id: "liberation_ending".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -10,
                            xp_change: 300,
                            unlock_challenges: vec![],
                            story_flags: vec!["liberation_ending".to_string(), "hero".to_string()],
                        },
                        requirements: Some(ChoiceRequirements {
                            min_level: Some(4),
                            required_challenges: None,
                            required_flags: Some(vec!["ghost_questioned".to_string()]),
                        }),
                    },
                ],
                default_branch: "main".to_string(),
            },
        );
    }

    /// Check if a branch should be triggered based on current game state
    pub fn check_trigger(
        &self,
        level: u32,
        sanity: u32,
        completed_challenges: &HashSet<String>,
    ) -> Option<&NarrativeBranch> {
        self.branches.values().find(|branch| {
            // Check level requirement
            if branch.trigger_level != level {
                return false;
            }

            // Check sanity requirement if specified
            if let Some(required_sanity) = branch.trigger_sanity {
                if sanity > required_sanity {
                    return false;
                }
            }

            // Check if this branch hasn't been triggered yet
            let branch_already_seen = self
                .choice_history
                .iter()
                .any(|choice_branch| choice_branch.starts_with(&branch.id));

            !branch_already_seen
        })
    }

    /// Make a choice and apply its effects
    pub fn make_choice(
        &mut self,
        branch_id: &str,
        choice_index: usize,
    ) -> Result<ChoiceEffects, String> {
        let branch = self
            .branches
            .get(branch_id)
            .ok_or_else(|| format!("Branch '{}' not found", branch_id))?;

        let choice = branch
            .choices
            .get(choice_index)
            .ok_or_else(|| format!("Choice index {} out of bounds", choice_index))?;

        // Record the choice
        self.choice_history.push(choice.branch_id.clone());

        // Apply story flags
        for flag in &choice.effects.story_flags {
            self.story_flags.insert(flag.clone());
        }

        // Update active branch
        self.active_branch = choice.branch_id.clone();

        Ok(choice.effects.clone())
    }

    /// Check if a choice is available based on requirements
    pub fn is_choice_available(
        &self,
        choice: &NarrativeChoice,
        level: u32,
        completed_challenges: &HashSet<String>,
    ) -> bool {
        if let Some(reqs) = &choice.requirements {
            // Check level requirement
            if let Some(min_level) = reqs.min_level {
                if level < min_level {
                    return false;
                }
            }

            // Check required challenges
            if let Some(required_challenges) = &reqs.required_challenges {
                for challenge_id in required_challenges {
                    if !completed_challenges.contains(challenge_id) {
                        return false;
                    }
                }
            }

            // Check required flags
            if let Some(required_flags) = &reqs.required_flags {
                for flag in required_flags {
                    if !self.story_flags.contains(flag) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn get_story_flags(&self) -> &HashSet<String> {
        &self.story_flags
    }

    pub fn get_choice_history(&self) -> &[String] {
        &self.choice_history
    }

    pub fn get_branch(&self, id: &str) -> Option<&NarrativeBranch> {
        self.branches.get(id)
    }

    pub fn get_all_branches(&self) -> Vec<&NarrativeBranch> {
        self.branches.values().collect()
    }
}
