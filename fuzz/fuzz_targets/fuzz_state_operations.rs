#![no_main]

use hack_simulator::state::GameState;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        // Test state operations with random inputs
        let mut state = GameState::new(input.to_string());

        // Try various operations
        let _ = state.complete_challenge(input, 50);
        let _ = state.has_completed(input);
        let _ = state.discover_secret(input.to_string());

        // Sanity modifications with fuzzed values
        if let Some(&byte) = data.first() {
            state.modify_sanity(byte as i32);
        }
    }
});
