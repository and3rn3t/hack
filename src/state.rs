use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_level: usize,
    pub completed_challenges: HashSet<String>,
    pub discovered_secrets: HashSet<String>,
    pub player_name: String,
    pub sanity: i32,
    pub experience: i32,
}

impl GameState {
    pub fn new(player_name: String) -> Self {
        GameState {
            current_level: 0,
            completed_challenges: HashSet::new(),
            discovered_secrets: HashSet::new(),
            player_name,
            sanity: 100,
            experience: 0,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("game_save.json", json)?;
        Ok(())
    }

    pub fn load() -> std::io::Result<Self> {
        if Path::new("game_save.json").exists() {
            let json = fs::read_to_string("game_save.json")?;
            let state: GameState = serde_json::from_str(&json)?;
            Ok(state)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No save file found",
            ))
        }
    }

    pub fn complete_challenge(&mut self, challenge_id: &str, reward_xp: i32) {
        self.completed_challenges.insert(challenge_id.to_string());
        self.experience += reward_xp;

        // Level progression based on experience
        let new_level = ((self.experience / 100).min(10)) as usize;
        if new_level > self.current_level {
            self.current_level = new_level;
        }
    }

    pub fn modify_sanity(&mut self, amount: i32) {
        self.sanity = (self.sanity + amount).clamp(0, 100);
    }

    pub fn discover_secret(&mut self, secret: String) {
        self.discovered_secrets.insert(secret);
    }

    pub fn has_completed(&self, challenge_id: &str) -> bool {
        self.completed_challenges.contains(challenge_id)
    }
}
