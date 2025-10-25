// Integration tests for save/load functionality

mod common;

use common::*;
use hack_simulator::state::GameState;
use std::fs;

#[test]
fn test_save_and_load_round_trip() {
    let temp = TempSaveFile::new("test_round_trip.json");

    // Create a state with some progress
    let mut original = create_test_state();
    original.complete_challenge("welcome", 50);
    original.complete_challenge("file_discovery", 50);
    original.modify_sanity(-25);

    // Save to temp file
    fs::write(
        temp.path(),
        serde_json::to_string_pretty(&original).unwrap(),
    )
    .expect("Failed to save");

    // Load from temp file
    let json = fs::read_to_string(temp.path()).expect("Failed to read");
    let loaded: GameState = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_states_equal(&original, &loaded, "Round trip");
}

#[test]
fn test_save_preserves_all_fields() {
    let temp = TempSaveFile::new("test_all_fields.json");

    let state = create_advanced_state();

    // Save
    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    // Load
    let loaded_json = fs::read_to_string(temp.path()).expect("Read failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    // Verify all fields
    assert_eq!(state.player_name, loaded.player_name);
    assert_eq!(state.current_level, loaded.current_level);
    assert_eq!(state.experience, loaded.experience);
    assert_eq!(state.sanity, loaded.sanity);

    // Verify collections
    for challenge in &state.completed_challenges {
        assert!(
            loaded.completed_challenges.contains(challenge),
            "Missing challenge: {}",
            challenge
        );
    }

    for secret in &state.discovered_secrets {
        assert!(
            loaded.discovered_secrets.contains(secret),
            "Missing secret: {}",
            secret
        );
    }
}

#[test]
fn test_multiple_save_load_cycles() {
    let temp = TempSaveFile::new("test_multiple_cycles.json");

    let mut state = create_test_state();

    // Cycle 1
    state.complete_challenge("welcome", 50);
    let json1 = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json1).expect("Save 1 failed");

    // Cycle 2
    state.complete_challenge("file_discovery", 50);
    let json2 = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json2).expect("Save 2 failed");

    // Cycle 3
    state.complete_challenge("port_scan", 50);
    let json3 = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json3).expect("Save 3 failed");

    // Final load
    let loaded_json = fs::read_to_string(temp.path()).expect("Final load failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    assert_eq!(loaded.experience, 150);
    assert_eq!(loaded.completed_challenges.len(), 3);
}

#[test]
fn test_save_with_unicode_characters() {
    let temp = TempSaveFile::new("test_unicode.json");

    let mut state = GameState::new("玩家👻".to_string());
    state.complete_challenge("test_挑战", 50);
    state.discover_secret("秘密🔥".to_string());

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    let loaded_json = fs::read_to_string(temp.path()).expect("Read failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    assert_eq!(loaded.player_name, "玩家👻");
    assert!(loaded.has_completed("test_挑战"));
    assert!(loaded.discovered_secrets.contains("秘密🔥"));
}

#[test]
fn test_save_file_is_human_readable() {
    let temp = TempSaveFile::new("test_readable.json");

    let mut state = GameState::new("ReadableTest".to_string());
    state.complete_challenge("welcome", 50);

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    let contents = fs::read_to_string(temp.path()).expect("Read failed");

    // Should be pretty-printed and contain expected fields
    assert!(contents.contains("\"player_name\""));
    assert!(contents.contains("ReadableTest"));
    assert!(contents.contains("\"experience\""));
    assert!(contents.contains("\"sanity\""));
    assert!(contents.contains("\"completed_challenges\""));
    assert!(contents.contains("welcome"));

    // Should have multiple lines (pretty-printed)
    assert!(contents.lines().count() > 5);
}

#[test]
fn test_empty_collections_serialize_correctly() {
    let temp = TempSaveFile::new("test_empty_collections.json");

    let state = create_test_state(); // No completed challenges or secrets

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    let loaded_json = fs::read_to_string(temp.path()).expect("Read failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    assert!(loaded.completed_challenges.is_empty());
    assert!(loaded.discovered_secrets.is_empty());
}

#[test]
fn test_large_number_of_challenges() {
    let temp = TempSaveFile::new("test_many_challenges.json");

    let mut state = create_test_state();

    // Complete many challenges
    for i in 0..100 {
        state.complete_challenge(&format!("challenge_{}", i), 10);
    }

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    let loaded_json = fs::read_to_string(temp.path()).expect("Read failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    assert_eq!(loaded.completed_challenges.len(), 100);
    assert!(loaded.has_completed("challenge_0"));
    assert!(loaded.has_completed("challenge_99"));
}

#[test]
fn test_state_after_game_over() {
    let temp = TempSaveFile::new("test_game_over.json");

    let state = create_game_over_state();
    assert_eq!(state.sanity, 0);

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    let loaded_json = fs::read_to_string(temp.path()).expect("Read failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    assert_eq!(loaded.sanity, 0);
}

#[test]
fn test_backward_compatibility_basic_fields() {
    let temp = TempSaveFile::new("test_backward_compat.json");

    // Manually create a minimal save file (simulating old version)
    let minimal_json = r#"{
        "current_level": 0,
        "completed_challenges": [],
        "discovered_secrets": [],
        "player_name": "OldSave",
        "sanity": 100,
        "experience": 0
    }"#;

    fs::write(temp.path(), minimal_json).expect("Write failed");

    let loaded_json = fs::read_to_string(temp.path()).expect("Read failed");
    let loaded: GameState = serde_json::from_str(&loaded_json).expect("Deserialize failed");

    assert_eq!(loaded.player_name, "OldSave");
    assert_eq!(loaded.sanity, 100);
}

#[test]
fn test_save_file_size_is_reasonable() {
    let temp = TempSaveFile::new("test_file_size.json");

    let state = create_advanced_state();

    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(temp.path(), &json).expect("Save failed");

    let metadata = fs::metadata(temp.path()).expect("Metadata failed");
    let file_size = metadata.len();

    // Save file should be under 10KB for reasonable progress
    assert!(
        file_size < 10_000,
        "Save file too large: {} bytes",
        file_size
    );

    // Should be at least 100 bytes (not empty)
    assert!(
        file_size > 100,
        "Save file suspiciously small: {} bytes",
        file_size
    );
}
