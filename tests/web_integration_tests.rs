//! Integration tests for WebAssembly bindings and web-specific functionality

#[cfg(feature = "web")]
mod web_tests {
    use hack_simulator::web::{WebChallenge, WebGameEngine, WebGameState};
    use serde_json;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_web_game_engine_creation() {
        let engine = WebGameEngine::new();
        assert!(!engine.get_introduction().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_web_game_state_creation() {
        let state = WebGameState::new();
        assert_eq!(state.get_level(), 0);
        assert_eq!(state.get_sanity(), 100);
        assert_eq!(state.get_xp(), 0);
    }

    #[wasm_bindgen_test]
    fn test_challenge_serialization() {
        let engine = WebGameEngine::new();
        let challenges_json = engine.get_challenges_json();

        // Should be valid JSON
        let challenges: Vec<serde_json::Value> =
            serde_json::from_str(&challenges_json).expect("Challenges JSON should be valid");

        // Should have expected number of challenges
        assert!(challenges.len() > 0);

        // Each challenge should have required fields
        for challenge in challenges {
            assert!(challenge.get("id").is_some());
            assert!(challenge.get("title").is_some());
            assert!(challenge.get("level").is_some());
            assert!(challenge.get("category").is_some());
            assert!(challenge.get("description").is_some());
            assert!(challenge.get("solution").is_some());
        }
    }

    #[wasm_bindgen_test]
    fn test_challenge_validation() {
        let engine = WebGameEngine::new();
        let mut state = WebGameState::new();

        // Test basic encoding challenge
        let result_json = engine.validate_challenge_answer(
            "basic_base64",
            "aGVsbG8=", // base64 for "hello"
            &mut state,
        );

        let result: serde_json::Value =
            serde_json::from_str(&result_json).expect("Result should be valid JSON");

        // Check if it's a valid result structure
        assert!(result.get("success").is_some());
        assert!(result.get("message").is_some());
    }

    #[wasm_bindgen_test]
    fn test_stats_serialization() {
        let state = WebGameState::new();
        let stats_json = state.get_stats_json();

        let stats: serde_json::Value =
            serde_json::from_str(&stats_json).expect("Stats JSON should be valid");

        // Should have expected stat fields
        assert!(stats.get("level").is_some());
        assert!(stats.get("xp").is_some());
        assert!(stats.get("sanity").is_some());
        assert!(stats.get("completed_challenges").is_some());
    }

    #[wasm_bindgen_test]
    fn test_save_load_functionality() {
        let mut state = WebGameState::new();

        // Modify state
        state.add_xp(50);
        state.decrease_sanity(10);

        // Save state
        let save_data = state.save_to_json();

        // Create new state and load
        let mut new_state = WebGameState::new();
        let load_result = new_state.load_from_json(&save_data);

        assert!(load_result, "Load should succeed");
        assert_eq!(new_state.get_xp(), 50);
        assert_eq!(new_state.get_sanity(), 90);
    }

    #[wasm_bindgen_test]
    fn test_challenge_attempts() {
        let mut state = WebGameState::new();
        let challenge_id = "basic_base64";

        // Should be able to attempt initially
        assert!(state.can_attempt_challenge(challenge_id));

        // Add as completed
        state.add_completed_challenge(challenge_id);

        // Should not be able to attempt again
        assert!(!state.can_attempt_challenge(challenge_id));

        // Should appear in completed list
        let completed = state.get_completed_challenges();
        assert!(completed.iter().any(|id| id == challenge_id));
    }

    #[wasm_bindgen_test]
    fn test_level_progression() {
        let mut state = WebGameState::new();

        // Start at level 0
        assert_eq!(state.get_level(), 0);

        // Add enough XP to level up
        state.add_xp(100);

        // Should level up (assuming 100 XP per level)
        assert!(state.get_level() >= 1);
    }

    #[wasm_bindgen_test]
    fn test_sanity_boundaries() {
        let mut state = WebGameState::new();

        // Start at full sanity
        assert_eq!(state.get_sanity(), 100);

        // Decrease sanity
        state.decrease_sanity(30);
        assert_eq!(state.get_sanity(), 70);

        // Should not go below 0
        state.decrease_sanity(150);
        assert_eq!(state.get_sanity(), 0);

        // Should not go above 100 when adding
        state.add_sanity(200);
        assert!(state.get_sanity() <= 100);
    }

    #[wasm_bindgen_test]
    fn test_error_handling() {
        let engine = WebGameEngine::new();
        let mut state = WebGameState::new();

        // Test invalid challenge ID
        let result_json =
            engine.validate_challenge_answer("nonexistent_challenge", "test", &mut state);

        let result: serde_json::Value =
            serde_json::from_str(&result_json).expect("Result should still be valid JSON");

        // Should indicate failure
        assert_eq!(result.get("success").unwrap().as_bool().unwrap(), false);
    }
}
