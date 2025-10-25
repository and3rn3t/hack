// Web-compatible version of The Hack: Ghost Protocol
// Uses wasm-bindgen for browser integration

// Use smaller allocator for WASM builds
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use js_sys::Date;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::{
    challenges::{get_all_challenges, Challenge, ChallengeCategory},
    state::GameState,
};

// Utility to log to browser console
macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()));
}

// Web-compatible game state that can be serialized to/from JavaScript
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGameState {
    #[wasm_bindgen(skip)]
    pub inner: GameState,
}

#[wasm_bindgen]
impl WebGameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGameState {
        WebGameState {
            inner: GameState::new("Player".to_string()),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> usize {
        self.inner.level()
    }

    #[wasm_bindgen(getter)]
    pub fn xp(&self) -> i32 {
        self.inner.xp()
    }

    #[wasm_bindgen(getter)]
    pub fn sanity(&self) -> i32 {
        self.inner.sanity
    }

    #[wasm_bindgen(getter)]
    pub fn completed_challenges(&self) -> Vec<JsValue> {
        self.inner
            .completed_challenges
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect()
    }

    // Get player statistics as JSON string
    pub fn get_stats_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap_or_else(|_| "{}".to_string())
    }

    // Save state to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap_or_else(|_| "{}".to_string())
    }

    // Load state from JSON string
    pub fn from_json(json: &str) -> Result<WebGameState, JsValue> {
        let inner: GameState = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse game state: {}", e)))?;

        Ok(WebGameState { inner })
    }

    // Check if player can take on a challenge
    pub fn can_attempt_challenge(&self, challenge_id: &str) -> bool {
        !self
            .inner
            .completed_challenges
            .contains(&challenge_id.to_string())
    }

    // Add XP and check for level up
    pub fn add_xp(&mut self, amount: i32) -> bool {
        self.inner.add_xp(amount);
        self.inner.check_level_up()
    }

    // Decrease sanity
    pub fn decrease_sanity(&mut self, amount: i32) -> bool {
        self.inner.decrease_sanity(amount);
        self.inner.sanity > 0
    }

    // Mark challenge as completed
    pub fn complete_challenge(&mut self, challenge_id: &str) {
        self.inner.add_completed_challenge(challenge_id);
    }
}

impl Default for WebGameState {
    fn default() -> Self {
        Self::new()
    }
}

// Web-compatible challenge representation
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebChallenge {
    #[wasm_bindgen(skip)]
    pub inner: Challenge,
}

#[wasm_bindgen]
impl WebChallenge {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> usize {
        self.inner.level
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.inner.title.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn category(&self) -> String {
        format!("{:?}", self.inner.category)
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.inner.description.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn prompt(&self) -> String {
        self.inner.prompt.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn sanity_cost(&self) -> i32 {
        self.inner.sanity_cost
    }

    #[wasm_bindgen(getter)]
    pub fn xp_reward(&self) -> i32 {
        self.inner.xp_reward
    }

    // Validate an answer
    pub fn validate_answer(&self, answer: &str) -> bool {
        self.inner.validate(answer)
    }

    // Get a hint by index
    pub fn get_hint(&self, index: usize) -> Option<String> {
        self.inner.hints.get(index).map(|s| s.to_string())
    }

    // Get total number of hints
    pub fn hint_count(&self) -> usize {
        self.inner.hints.len()
    }

    // Get all challenge data as JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap_or_else(|_| "{}".to_string())
    }
}

// Web game engine
#[wasm_bindgen]
pub struct WebGameEngine {
    challenges: Vec<WebChallenge>,
    current_challenge_index: Option<usize>,
}

#[wasm_bindgen]
impl WebGameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGameEngine {
        console_log!("Initializing The Hack: Ghost Protocol Web Engine");

        let challenges = get_all_challenges()
            .into_iter()
            .map(|c| WebChallenge { inner: c.clone() })
            .collect();

        WebGameEngine {
            challenges,
            current_challenge_index: None,
        }
    }

    // Get all available challenges as JSON
    pub fn get_challenges_json(&self) -> String {
        let challenges_data: Vec<serde_json::Value> = self
            .challenges
            .iter()
            .map(|c| serde_json::to_value(&c.inner).unwrap_or(serde_json::Value::Null))
            .collect();

        serde_json::to_string(&challenges_data).unwrap_or_else(|_| "[]".to_string())
    }

    // Get challenges for a specific level
    pub fn get_challenges_by_level(&self, level: usize) -> Vec<JsValue> {
        self.challenges
            .iter()
            .filter(|c| c.inner.level == level)
            .map(|c| {
                let json = c.to_json();
                JsValue::from_str(&json)
            })
            .collect()
    }

    // Get a challenge by ID
    pub fn get_challenge(&self, id: &str) -> Option<WebChallenge> {
        self.challenges.iter().find(|c| c.inner.id == id).cloned()
    }

    // Get the introduction narrative
    pub fn get_introduction(&self) -> String {
        "Welcome to The Hack: Ghost Protocol - A Horror-Themed Cybersecurity Challenge Game"
            .to_string()
    }

    // Get total number of challenges
    pub fn challenge_count(&self) -> usize {
        self.challenges.len()
    }

    // Get challenges by category
    pub fn get_challenges_by_category(&self, category: &str) -> Vec<JsValue> {
        let target_category = match category.to_lowercase().as_str() {
            "encoding" => ChallengeCategory::Encoding,
            "cryptography" => ChallengeCategory::Cryptography,
            "web" => ChallengeCategory::Web,
            "forensics" => ChallengeCategory::Forensics,
            "reverse" => ChallengeCategory::Reverse,
            "binary" => ChallengeCategory::Binary,
            _ => return Vec::new(),
        };

        self.challenges
            .iter()
            .filter(|c| c.inner.category == target_category)
            .map(|c| {
                let json = c.to_json();
                JsValue::from_str(&json)
            })
            .collect()
    }

    // Validate challenge answer and return detailed result
    pub fn validate_challenge_answer(
        &self,
        challenge_id: &str,
        answer: &str,
        state: &mut WebGameState,
    ) -> String {
        if let Some(challenge) = self.get_challenge(challenge_id) {
            if !state.can_attempt_challenge(challenge_id) {
                return serde_json::json!({
                    "success": false,
                    "message": "Challenge already completed",
                    "already_completed": true
                })
                .to_string();
            }

            let is_correct = challenge.validate_answer(answer);

            if is_correct {
                // Award XP and mark as completed
                let leveled_up = state.add_xp(challenge.inner.xp_reward);
                state.complete_challenge(challenge_id);

                serde_json::json!({
                    "success": true,
                    "message": "Correct! Challenge completed!",
                    "xp_awarded": challenge.inner.xp_reward,
                    "leveled_up": leveled_up,
                    "new_level": state.level(),
                    "new_xp": state.xp()
                })
                .to_string()
            } else {
                // Decrease sanity for wrong answer
                let still_alive = state.decrease_sanity(challenge.inner.sanity_cost / 2);

                serde_json::json!({
                    "success": false,
                    "message": "Incorrect answer. Try again...",
                    "sanity_lost": challenge.inner.sanity_cost / 2,
                    "remaining_sanity": state.sanity(),
                    "game_over": !still_alive
                })
                .to_string()
            }
        } else {
            serde_json::json!({
                "success": false,
                "message": "Challenge not found",
                "error": "invalid_challenge_id"
            })
            .to_string()
        }
    }
}

impl Default for WebGameEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Additional web utilities
#[wasm_bindgen]
pub fn get_current_timestamp() -> f64 {
    Date::now()
}

#[wasm_bindgen]
pub fn log_to_console(message: &str) {
    console_log!("{}", message);
}

// Initialize the web module when loaded
#[wasm_bindgen(start)]
pub fn init() {
    console_log!("The Hack: Ghost Protocol - Web Module Loaded");

    // Set up panic hook for better error reporting
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Generate a new game save as a JSON string for download
#[wasm_bindgen]
pub fn generate_save_file(state: &WebGameState) -> String {
    let save_data = serde_json::json!({
        "version": "1.0",
        "timestamp": Date::now(),
        "game_state": state.inner,
        "meta": {
            "platform": "web",
            "user_agent": "WebAssembly Browser"
        }
    });

    serde_json::to_string_pretty(&save_data).unwrap_or_else(|_| "{}".to_string())
}

// Load game save from JSON string
#[wasm_bindgen]
pub fn load_save_file(json: &str) -> Result<WebGameState, JsValue> {
    let save_data: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&format!("Invalid save file format: {}", e)))?;

    let game_state = save_data
        .get("game_state")
        .ok_or_else(|| JsValue::from_str("Save file missing game state"))?;

    let inner: GameState = serde_json::from_value(game_state.clone())
        .map_err(|e| JsValue::from_str(&format!("Failed to load game state: {}", e)))?;

    Ok(WebGameState { inner })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_game_state_creation() {
        let state = WebGameState::new();
        assert_eq!(state.level(), 0);
        assert_eq!(state.xp(), 0);
        assert_eq!(state.sanity(), 100);
    }

    #[test]
    fn test_web_game_engine_creation() {
        let engine = WebGameEngine::new();
        assert!(engine.challenge_count() > 0);
    }

    #[test]
    fn test_challenge_validation() {
        let engine = WebGameEngine::new();
        if let Some(challenge) = engine.challenges.first() {
            // This will depend on the actual challenge, but we can test the structure
            assert!(!challenge.id().is_empty());
            assert!(!challenge.title().is_empty());
        }
    }

    #[test]
    fn test_json_serialization() {
        let state = WebGameState::new();
        let json = state.to_json();
        assert!(!json.is_empty());

        let loaded_state = WebGameState::from_json(&json).unwrap();
        assert_eq!(loaded_state.level(), state.level());
    }
}
