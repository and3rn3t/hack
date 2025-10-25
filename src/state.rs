// Allow unused methods - some are prepared for future features
#![allow(dead_code)]

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
    #[serde(default)]
    pub tutorial_completed: bool,
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
            tutorial_completed: false,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.save_to("game_save.json")
    }

    pub fn save_to(&self, path: &str) -> std::io::Result<()> {
        // Use compact JSON instead of pretty for smaller file size
        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Save with pretty formatting for debugging (dev use only)
    #[cfg(debug_assertions)]
    pub fn save_pretty(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load() -> std::io::Result<Self> {
        Self::load_from("game_save.json")
    }

    pub fn load_from(path: &str) -> std::io::Result<Self> {
        if Path::new(path).exists() {
            let json = fs::read_to_string(path)?;
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

    pub fn mark_tutorial_completed(&mut self) {
        self.tutorial_completed = true;
    }

    pub fn needs_tutorial(&self) -> bool {
        !self.tutorial_completed && self.completed_challenges.is_empty()
    }

    // Compatibility methods for web module
    pub fn level(&self) -> usize {
        self.current_level
    }

    pub fn xp(&self) -> i32 {
        self.experience
    }

    pub fn add_xp(&mut self, amount: i32) {
        self.experience += amount;
        // Auto-level up based on XP
        let new_level = ((self.experience / 100).min(10)) as usize;
        if new_level > self.current_level {
            self.current_level = new_level;
        }
    }

    pub fn check_level_up(&mut self) -> bool {
        let new_level = ((self.experience / 100).min(10)) as usize;
        if new_level > self.current_level {
            self.current_level = new_level;
            true
        } else {
            false
        }
    }

    pub fn decrease_sanity(&mut self, amount: i32) {
        self.sanity = (self.sanity - amount).max(0);
    }

    pub fn add_completed_challenge(&mut self, challenge_id: &str) {
        self.completed_challenges.insert(challenge_id.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper to clean up test save files
    fn cleanup_test_save() {
        let _ = fs::remove_file("test_game_save.json");
        let _ = fs::remove_file("game_save.json");
    }

    #[test]
    fn test_new_game_state() {
        let state = GameState::new("TestPlayer".to_string());

        assert_eq!(state.player_name, "TestPlayer");
        assert_eq!(state.current_level, 0);
        assert_eq!(state.sanity, 100);
        assert_eq!(state.experience, 0);
        assert!(state.completed_challenges.is_empty());
        assert!(state.discovered_secrets.is_empty());
    }

    #[test]
    fn test_complete_challenge_increases_xp() {
        let mut state = GameState::new("Test".to_string());

        state.complete_challenge("welcome", 50);

        assert_eq!(state.experience, 50);
        assert!(state.has_completed("welcome"));
    }

    #[test]
    fn test_complete_multiple_challenges() {
        let mut state = GameState::new("Test".to_string());

        state.complete_challenge("welcome", 50);
        state.complete_challenge("file_discovery", 50);
        state.complete_challenge("port_scan", 50);

        assert_eq!(state.experience, 150);
        assert_eq!(state.completed_challenges.len(), 3);
        assert!(state.has_completed("welcome"));
        assert!(state.has_completed("file_discovery"));
        assert!(state.has_completed("port_scan"));
    }

    #[test]
    fn test_complete_same_challenge_twice_is_idempotent() {
        let mut state = GameState::new("Test".to_string());

        state.complete_challenge("welcome", 50);
        state.complete_challenge("welcome", 50);

        // Should only have one entry in completed_challenges
        assert_eq!(state.completed_challenges.len(), 1);
        // But XP is added each time (this is current behavior)
        assert_eq!(state.experience, 100);
    }

    #[test]
    fn test_level_progression() {
        let mut state = GameState::new("Test".to_string());

        // Start at level 0
        assert_eq!(state.current_level, 0);

        // Get 100 XP -> level 1
        state.complete_challenge("challenge1", 100);
        assert_eq!(state.current_level, 1);

        // Get 200 XP total -> level 2
        state.complete_challenge("challenge2", 100);
        assert_eq!(state.current_level, 2);

        // Get 500 XP total -> level 5
        state.complete_challenge("challenge3", 300);
        assert_eq!(state.current_level, 5);
    }

    #[test]
    fn test_level_caps_at_10() {
        let mut state = GameState::new("Test".to_string());

        // Give massive XP
        state.complete_challenge("super_challenge", 10000);

        // Level should cap at 10
        assert!(state.current_level <= 10);
    }

    #[test]
    fn test_modify_sanity_increases() {
        let mut state = GameState::new("Test".to_string());
        state.sanity = 50;

        state.modify_sanity(25);

        assert_eq!(state.sanity, 75);
    }

    #[test]
    fn test_modify_sanity_decreases() {
        let mut state = GameState::new("Test".to_string());
        state.sanity = 100;

        state.modify_sanity(-30);

        assert_eq!(state.sanity, 70);
    }

    #[test]
    fn test_sanity_cannot_go_below_zero() {
        let mut state = GameState::new("Test".to_string());
        state.sanity = 20;

        state.modify_sanity(-50);

        assert_eq!(state.sanity, 0);
    }

    #[test]
    fn test_sanity_cannot_exceed_100() {
        let mut state = GameState::new("Test".to_string());
        state.sanity = 80;

        state.modify_sanity(50);

        assert_eq!(state.sanity, 100);
    }

    #[test]
    fn test_discover_secret() {
        let mut state = GameState::new("Test".to_string());

        state.discover_secret("hidden_file".to_string());
        state.discover_secret("easter_egg".to_string());

        assert_eq!(state.discovered_secrets.len(), 2);
        assert!(state.discovered_secrets.contains("hidden_file"));
        assert!(state.discovered_secrets.contains("easter_egg"));
    }

    #[test]
    fn test_discover_same_secret_twice() {
        let mut state = GameState::new("Test".to_string());

        state.discover_secret("hidden_file".to_string());
        state.discover_secret("hidden_file".to_string());

        // Should only have one entry (HashSet behavior)
        assert_eq!(state.discovered_secrets.len(), 1);
    }

    #[test]
    fn test_has_completed_returns_false_for_new_challenge() {
        let state = GameState::new("Test".to_string());

        assert!(!state.has_completed("welcome"));
    }

    #[test]
    fn test_save_creates_file() {
        cleanup_test_save();

        let state = GameState::new("SaveTest".to_string());
        let result = state.save();

        assert!(result.is_ok());
        assert!(Path::new("game_save.json").exists());

        cleanup_test_save();
    }

    #[test]
    fn test_save_and_load_preserves_state() {
        let test_file = "test_save_preserves_state.json";
        let _ = fs::remove_file(test_file); // Clean before test

        let mut state = GameState::new("TestPlayer".to_string());
        state.complete_challenge("welcome", 50);
        state.complete_challenge("file_discovery", 50);
        state.modify_sanity(-20);
        state.discover_secret("secret1".to_string());

        state.save_to(test_file).expect("Save failed");

        let loaded = GameState::load_from(test_file).expect("Load failed");

        assert_eq!(loaded.player_name, "TestPlayer");
        assert_eq!(loaded.experience, 100);
        assert_eq!(loaded.sanity, 80);
        assert_eq!(loaded.completed_challenges.len(), 2);
        assert!(loaded.has_completed("welcome"));
        assert!(loaded.has_completed("file_discovery"));
        assert_eq!(loaded.discovered_secrets.len(), 1);
        assert!(loaded.discovered_secrets.contains("secret1"));

        let _ = fs::remove_file(test_file); // Clean after test
    }

    #[test]
    fn test_load_fails_when_no_save_exists() {
        cleanup_test_save();

        let result = GameState::load();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_save_overwrites_existing_file() {
        let test_file = "test_save_overwrites.json";
        let _ = fs::remove_file(test_file); // Clean before test

        let mut state1 = GameState::new("Player1".to_string());
        state1.complete_challenge("welcome", 50);
        state1.save_to(test_file).expect("First save failed");

        let mut state2 = GameState::new("Player2".to_string());
        state2.complete_challenge("file_discovery", 100);
        state2.save_to(test_file).expect("Second save failed");

        let loaded = GameState::load_from(test_file).expect("Load failed");

        // Should have Player2's state
        assert_eq!(loaded.player_name, "Player2");
        assert_eq!(loaded.experience, 100);
        assert!(loaded.has_completed("file_discovery"));
        assert!(!loaded.has_completed("welcome"));

        let _ = fs::remove_file(test_file); // Clean after test
    }

    #[test]
    fn test_serialization_format() {
        let test_file = "test_serialization_format.json";
        let _ = fs::remove_file(test_file); // Clean before test

        let mut state = GameState::new("FormatTest".to_string());
        state.complete_challenge("test_challenge", 75);
        state.save_to(test_file).expect("Save failed");

        let json = fs::read_to_string(test_file).expect("Read failed");

        // Verify it's valid JSON
        assert!(json.contains("\"player_name\":"));
        assert!(json.contains("\"experience\":"));
        assert!(json.contains("\"sanity\":"));
        assert!(json.contains("\"completed_challenges\":"));
        assert!(json.contains("FormatTest"));

        let _ = fs::remove_file(test_file); // Clean after test
    }

    #[test]
    fn test_state_cloning() {
        let mut state = GameState::new("Clone".to_string());
        state.complete_challenge("welcome", 50);

        let cloned = state.clone();

        assert_eq!(cloned.player_name, state.player_name);
        assert_eq!(cloned.experience, state.experience);
        assert_eq!(
            cloned.completed_challenges.len(),
            state.completed_challenges.len()
        );
    }

    #[test]
    fn test_extreme_values() {
        let mut state = GameState::new("Extreme".to_string());

        // Very large XP
        state.complete_challenge("mega", i32::MAX / 2);
        assert!(state.experience > 0);

        // Sanity edge cases
        state.modify_sanity(-1000);
        assert_eq!(state.sanity, 0);

        state.modify_sanity(1000);
        assert_eq!(state.sanity, 100);
    }

    #[test]
    fn test_empty_player_name() {
        let state = GameState::new("".to_string());
        assert_eq!(state.player_name, "");
    }

    #[test]
    fn test_unicode_player_name() {
        let state = GameState::new("测试玩家👻".to_string());
        assert_eq!(state.player_name, "测试玩家👻");
    }

    #[test]
    fn test_long_player_name() {
        let long_name = "A".repeat(1000);
        let state = GameState::new(long_name.clone());
        assert_eq!(state.player_name, long_name);
    }

    #[test]
    fn test_challenge_id_with_special_chars() {
        let mut state = GameState::new("Test".to_string());

        state.complete_challenge("challenge_with_underscores", 50);
        state.complete_challenge("challenge-with-dashes", 50);
        state.complete_challenge("challenge.with.dots", 50);

        assert!(state.has_completed("challenge_with_underscores"));
        assert!(state.has_completed("challenge-with-dashes"));
        assert!(state.has_completed("challenge.with.dots"));
    }

    // Property-based tests
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// Property: Sanity should always stay within 0-100 range
            #[test]
            fn test_sanity_always_bounded(initial in 0..=100i32, changes in prop::collection::vec(-50..=50i32, 0..20)) {
                let mut state = GameState::new("PropTest".to_string());
                state.sanity = initial;

                for change in changes {
                    state.modify_sanity(change);
                    assert!(state.sanity >= 0 && state.sanity <= 100,
                        "Sanity {} out of bounds after change {}", state.sanity, change);
                }
            }

            /// Property: Experience should never decrease
            #[test]
            fn test_experience_never_decreases(rewards in prop::collection::vec(1..=200i32, 1..20)) {
                let mut state = GameState::new("PropTest".to_string());
                let mut prev_xp = 0;

                for (i, reward) in rewards.iter().enumerate() {
                    state.complete_challenge(&format!("challenge_{}", i), *reward);
                    assert!(state.experience >= prev_xp,
                        "Experience decreased from {} to {}", prev_xp, state.experience);
                    prev_xp = state.experience;
                }
            }

            /// Property: Completing same challenge multiple times adds to set once
            #[test]
            fn test_challenge_completion_idempotent(challenge_id in "[a-z_]{5,20}", times in 1..=10usize) {
                let mut state = GameState::new("PropTest".to_string());

                for _ in 0..times {
                    state.complete_challenge(&challenge_id, 50);
                }

                // Should only appear once in the set
                assert_eq!(
                    state.completed_challenges.iter().filter(|&c| c == &challenge_id).count(),
                    1,
                    "Challenge appeared multiple times in set"
                );
            }

            /// Property: has_completed is consistent with completed_challenges
            #[test]
            fn test_has_completed_consistency(challenge_ids in prop::collection::vec("[a-z_]{5,15}", 1..20)) {
                let mut state = GameState::new("PropTest".to_string());

                for id in &challenge_ids {
                    state.complete_challenge(id, 50);
                }

                // Every challenge in the set should return true for has_completed
                for id in &state.completed_challenges {
                    assert!(state.has_completed(id),
                        "has_completed returned false for challenge in set: {}", id);
                }
            }

            /// Property: Secrets can be discovered multiple times without issue
            #[test]
            fn test_secret_discovery_safe(secrets in prop::collection::vec("[a-z0-9_]{5,20}", 1..30)) {
                let mut state = GameState::new("PropTest".to_string());

                for secret in &secrets {
                    state.discover_secret(secret.clone());
                    assert!(state.discovered_secrets.contains(secret));
                }

                // Count unique secrets
                let unique_count = secrets.iter().collect::<std::collections::HashSet<_>>().len();
                assert_eq!(state.discovered_secrets.len(), unique_count);
            }

            /// Property: Player name can be any string
            #[test]
            fn test_player_name_accepts_any_string(name in "\\PC{0,100}") {
                let state = GameState::new(name.clone());
                assert_eq!(state.player_name, name);
            }

            /// Property: Level progression is monotonic
            #[test]
            fn test_level_monotonic(xp_increments in prop::collection::vec(10..=100i32, 1..20)) {
                let mut state = GameState::new("PropTest".to_string());
                let mut prev_level = state.current_level;

                for (i, xp) in xp_increments.iter().enumerate() {
                    state.complete_challenge(&format!("c{}", i), *xp);
                    assert!(state.current_level >= prev_level,
                        "Level decreased from {} to {}", prev_level, state.current_level);
                    prev_level = state.current_level;
                }
            }

            /// Property: Serialization is lossless
            #[test]
            fn test_serialization_lossless(
                name in "[a-zA-Z0-9 ]{1,50}",
                xp in 0..1000i32,
                sanity in 0..=100i32,
                level in 0..=10usize
            ) {
                let mut state = GameState::new(name.clone());
                state.experience = xp;
                state.sanity = sanity;
                state.current_level = level;

                let json = serde_json::to_string(&state).unwrap();
                let deserialized: GameState = serde_json::from_str(&json).unwrap();

                assert_eq!(state.player_name, deserialized.player_name);
                assert_eq!(state.experience, deserialized.experience);
                assert_eq!(state.sanity, deserialized.sanity);
                assert_eq!(state.current_level, deserialized.current_level);
            }

            /// Property: Challenge IDs with any valid characters work
            #[test]
            fn test_arbitrary_challenge_ids(id in "[a-zA-Z0-9_\\-\\.]{1,50}") {
                let mut state = GameState::new("Test".to_string());
                state.complete_challenge(&id, 50);
                assert!(state.has_completed(&id));
            }
        }
    }
}
