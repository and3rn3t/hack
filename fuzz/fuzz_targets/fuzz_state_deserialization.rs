#![no_main]

use hack_simulator::state::GameState;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Try to deserialize random data as GameState
    if let Ok(json_str) = std::str::from_utf8(data) {
        // Should handle invalid JSON gracefully
        let _ = serde_json::from_str::<GameState>(json_str);
    }
});
