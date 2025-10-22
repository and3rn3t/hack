#![no_main]

use hack_simulator::challenges::get_all_challenges;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string
    if let Ok(input) = std::str::from_utf8(data) {
        let challenges = get_all_challenges();

        // Fuzz all challenge validators
        for challenge in challenges.iter() {
            // This should never panic, regardless of input
            let _ = (challenge.check_answer)(input);
        }
    }
});
