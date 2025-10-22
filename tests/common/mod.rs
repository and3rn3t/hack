// Common test utilities and fixtures for integration tests

use hack_simulator::state::GameState;
use std::fs;
use std::path::Path;

/// Creates a fresh GameState for testing
pub fn create_test_state() -> GameState {
    GameState::new("TestPlayer".to_string())
}

/// Creates a GameState with some progress
pub fn create_partially_completed_state() -> GameState {
    let mut state = create_test_state();
    state.complete_challenge("welcome", 50);
    state.complete_challenge("file_discovery", 50);
    state.modify_sanity(-10);
    state
}

/// Creates a GameState with significant progress
pub fn create_advanced_state() -> GameState {
    let mut state = create_test_state();

    // Complete several challenges
    state.complete_challenge("welcome", 50);
    state.complete_challenge("file_discovery", 50);
    state.complete_challenge("port_scan", 50);
    state.complete_challenge("caesar_cipher", 75);
    state.complete_challenge("sql_injection_basics", 75);

    // Reduce sanity
    state.modify_sanity(-40);

    // Discover some secrets
    state.discover_secret("hidden_path".to_string());
    state.discover_secret("easter_egg".to_string());

    state
}

/// Creates a GameState with low sanity
pub fn create_low_sanity_state() -> GameState {
    let mut state = create_test_state();
    state.modify_sanity(-90); // Sanity at 10
    state
}

/// Creates a GameState at game over condition
pub fn create_game_over_state() -> GameState {
    let mut state = create_test_state();
    state.modify_sanity(-100); // Sanity at 0
    state
}

/// Temporary save file manager for testing
pub struct TempSaveFile {
    path: String,
}

impl TempSaveFile {
    pub fn new(path: &str) -> Self {
        TempSaveFile {
            path: path.to_string(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn read(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

impl Drop for TempSaveFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

/// Common test inputs for challenge validation
pub mod test_inputs {
    pub const EMPTY: &str = "";
    pub const WHITESPACE: &str = "   ";
    pub const VERY_LONG: &str = "A very long string that repeats many times. A very long string that repeats many times. A very long string that repeats many times. A very long string that repeats many times. A very long string that repeats many times.";
    pub const SPECIAL_CHARS: &str = "!@#$%^&*()_+-={}[]|\\:\";<>?,./~`";
    pub const UNICODE: &str = "测试 👻 🎃 🔥";
    pub const SQL_INJECTION: &str = "' OR '1'='1' --";
    pub const HTML_TAGS: &str = "<script>alert('xss')</script>";
    pub const NULL_BYTE: &str = "test\0null";
}

/// Helper to compare game states
pub fn assert_states_equal(state1: &GameState, state2: &GameState, msg: &str) {
    assert_eq!(
        state1.player_name, state2.player_name,
        "{}: player_name mismatch",
        msg
    );
    assert_eq!(
        state1.current_level, state2.current_level,
        "{}: current_level mismatch",
        msg
    );
    assert_eq!(
        state1.experience, state2.experience,
        "{}: experience mismatch",
        msg
    );
    assert_eq!(state1.sanity, state2.sanity, "{}: sanity mismatch", msg);
    assert_eq!(
        state1.completed_challenges.len(),
        state2.completed_challenges.len(),
        "{}: completed_challenges count mismatch",
        msg
    );
    assert_eq!(
        state1.discovered_secrets.len(),
        state2.discovered_secrets.len(),
        "{}: discovered_secrets count mismatch",
        msg
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_state() {
        let state = create_test_state();
        assert_eq!(state.player_name, "TestPlayer");
        assert_eq!(state.experience, 0);
        assert_eq!(state.sanity, 100);
    }

    #[test]
    fn test_create_partially_completed_state() {
        let state = create_partially_completed_state();
        assert_eq!(state.experience, 100);
        assert_eq!(state.sanity, 90);
        assert_eq!(state.completed_challenges.len(), 2);
    }

    #[test]
    fn test_create_advanced_state() {
        let state = create_advanced_state();
        assert!(state.experience > 200);
        assert!(state.sanity < 100);
        assert!(state.completed_challenges.len() >= 5);
        assert!(state.discovered_secrets.len() >= 2);
    }

    #[test]
    fn test_temp_save_file_cleanup() {
        {
            let temp = TempSaveFile::new("test_cleanup.json");
            fs::write(temp.path(), "test").expect("Write failed");
            assert!(temp.exists());
        } // temp is dropped here

        // File should be cleaned up
        assert!(!Path::new("test_cleanup.json").exists());
    }
}
