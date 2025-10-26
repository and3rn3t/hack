// Allow unused methods - some are prepared for future features
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Achievement system for tracking player progress and milestones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AchievementId {
    // Progress-based achievements
    FirstBlood,     // Complete first challenge
    SpeedDemon,     // Complete challenge under time limit
    HintFree,       // Complete challenge without hints
    SanityReserves, // Maintain high sanity (75%+)
    GhostHunter,    // Complete all challenges
    Explorer,       // Discover all challenge categories

    // Skill-based achievements
    CryptographyMaster, // Complete all cryptography challenges
    NetworkNinja,       // Complete all network challenges
    WebWarrior,         // Complete all web challenges
    OSINTOperative,     // Complete all OSINT challenges
    ForensicsExpert,    // Complete advanced challenges

    // Behavioral achievements
    Persistent,         // Play for multiple sessions
    ThemeMaster,        // Try all color themes
    CompletePerfection, // 100% completion with max sanity
    RapidResponse,      // Complete multiple challenges quickly

    // Discovery achievements
    SecretSeeker,     // Find hidden easter eggs
    TerminalMaster,   // Use advanced commands
    TutorialGraduate, // Complete tutorial
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: AchievementId,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl Achievement {
    pub fn new(id: AchievementId, title: &str, description: &str, icon: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            icon: icon.to_string(),
            unlocked_timestamp: None,
        }
    }

    pub fn unlock(&mut self) {
        if self.unlocked_timestamp.is_none() {
            self.unlocked_timestamp = Some(chrono::Utc::now());
        }
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked_timestamp.is_some()
    }
}

/// User preferences for customization (v1.2.0 feature)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    #[serde(default = "default_difficulty_scaling")]
    pub difficulty_scaling: DifficultyScaling,
    #[serde(default = "default_hint_verbosity")]
    pub hint_verbosity: HintVerbosity,
    #[serde(default = "default_color_theme")]
    pub color_theme: String,
    #[serde(default = "default_font_size")]
    pub font_size: FontSize,
    #[serde(default = "default_audio_enabled")]
    pub audio_enabled: bool,
    #[serde(default = "default_animation_speed")]
    pub animation_speed: AnimationSpeed,
    #[serde(default)]
    pub user_aliases: HashMap<String, String>,
    #[serde(default = "default_save_slot")]
    pub current_save_slot: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyScaling {
    Adaptive, // Adjusts based on performance
    Static,   // Fixed difficulty
    Custom,   // User-defined scaling
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HintVerbosity {
    Minimal,  // Brief hints
    Moderate, // Standard hints
    Detailed, // Comprehensive hints
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationSpeed {
    None,   // No animations
    Slow,   // Reduced speed for accessibility
    Normal, // Standard speed
    Fast,   // Quick animations
}

// Default functions for user preferences
fn default_difficulty_scaling() -> DifficultyScaling {
    DifficultyScaling::Adaptive
}
fn default_hint_verbosity() -> HintVerbosity {
    HintVerbosity::Moderate
}
fn default_color_theme() -> String {
    "Horror".to_string()
}
fn default_font_size() -> FontSize {
    FontSize::Medium
}
fn default_audio_enabled() -> bool {
    true
}
fn default_animation_speed() -> AnimationSpeed {
    AnimationSpeed::Normal
}
fn default_save_slot() -> u8 {
    0
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            difficulty_scaling: default_difficulty_scaling(),
            hint_verbosity: default_hint_verbosity(),
            color_theme: default_color_theme(),
            font_size: default_font_size(),
            audio_enabled: default_audio_enabled(),
            animation_speed: default_animation_speed(),
            user_aliases: HashMap::new(),
            current_save_slot: default_save_slot(),
        }
    }
}

/// Enhanced progress analytics for v1.2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressAnalytics {
    #[serde(default)]
    pub total_playtime_minutes: u64,
    #[serde(default)]
    pub challenges_attempted: HashMap<String, u32>,
    #[serde(default)]
    pub hints_used_per_challenge: HashMap<String, u32>,
    #[serde(default)]
    pub completion_times: HashMap<String, u64>, // in seconds
    #[serde(default)]
    pub difficulty_preferences: HashMap<String, String>, // challenge_id -> difficulty_level
    #[serde(default)]
    pub learning_streaks: u32,
    #[serde(default = "chrono::Utc::now")]
    pub last_session: chrono::DateTime<chrono::Utc>,
}

impl Default for ProgressAnalytics {
    fn default() -> Self {
        Self {
            total_playtime_minutes: 0,
            challenges_attempted: HashMap::new(),
            hints_used_per_challenge: HashMap::new(),
            completion_times: HashMap::new(),
            difficulty_preferences: HashMap::new(),
            learning_streaks: 0,
            last_session: chrono::Utc::now(),
        }
    }
}

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
    #[serde(default)]
    pub achievements: std::collections::HashMap<AchievementId, Achievement>,
    #[serde(default)]
    pub session_count: usize,
    #[serde(default)]
    pub themes_tried: HashSet<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub challenge_start_times: std::collections::HashMap<String, chrono::DateTime<chrono::Utc>>,

    // v1.2.0 new features
    #[serde(default)]
    pub user_preferences: UserPreferences,
    #[serde(default)]
    pub progress_analytics: ProgressAnalytics,

    // v1.6.0 branching narrative system
    #[serde(default)]
    pub story_flags: HashSet<String>,
    #[serde(default)]
    pub choice_history: Vec<String>,
    #[serde(default)]
    pub active_narrative_branch: String,
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
            achievements: Self::initialize_achievements(),
            session_count: 1,
            themes_tried: HashSet::new(),
            created_at: chrono::Utc::now(),
            challenge_start_times: std::collections::HashMap::new(),
            user_preferences: UserPreferences::default(),
            progress_analytics: ProgressAnalytics::default(),
            story_flags: HashSet::new(),
            choice_history: Vec::new(),
            active_narrative_branch: "main".to_string(),
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

    // Achievement System Implementation

    fn initialize_achievements() -> std::collections::HashMap<AchievementId, Achievement> {
        let mut achievements = std::collections::HashMap::new();

        // Progress-based achievements
        achievements.insert(
            AchievementId::FirstBlood,
            Achievement::new(
                AchievementId::FirstBlood,
                "🩸 First Blood",
                "Complete your first challenge",
                "🩸",
            ),
        );

        achievements.insert(
            AchievementId::SpeedDemon,
            Achievement::new(
                AchievementId::SpeedDemon,
                "⚡ Speed Demon",
                "Complete a challenge in under 2 minutes",
                "⚡",
            ),
        );

        achievements.insert(
            AchievementId::HintFree,
            Achievement::new(
                AchievementId::HintFree,
                "🧠 Hint Free",
                "Complete a challenge without using any hints",
                "🧠",
            ),
        );

        achievements.insert(
            AchievementId::SanityReserves,
            Achievement::new(
                AchievementId::SanityReserves,
                "😌 Sanity Reserves",
                "Maintain sanity above 75% throughout the game",
                "😌",
            ),
        );

        achievements.insert(
            AchievementId::GhostHunter,
            Achievement::new(
                AchievementId::GhostHunter,
                "👻 Ghost Hunter",
                "Complete all challenges in the game",
                "👻",
            ),
        );

        achievements.insert(
            AchievementId::Explorer,
            Achievement::new(
                AchievementId::Explorer,
                "🗺️ Explorer",
                "Try challenges from all available categories",
                "🗺️",
            ),
        );

        // Skill-based achievements
        achievements.insert(
            AchievementId::CryptographyMaster,
            Achievement::new(
                AchievementId::CryptographyMaster,
                "🔐 Cryptography Master",
                "Complete all cryptography challenges",
                "🔐",
            ),
        );

        achievements.insert(
            AchievementId::NetworkNinja,
            Achievement::new(
                AchievementId::NetworkNinja,
                "🥷 Network Ninja",
                "Complete all network security challenges",
                "🥷",
            ),
        );

        achievements.insert(
            AchievementId::WebWarrior,
            Achievement::new(
                AchievementId::WebWarrior,
                "🌐 Web Warrior",
                "Complete all web application security challenges",
                "🌐",
            ),
        );

        achievements.insert(
            AchievementId::OSINTOperative,
            Achievement::new(
                AchievementId::OSINTOperative,
                "🔍 OSINT Operative",
                "Complete all Open Source Intelligence challenges",
                "🔍",
            ),
        );

        achievements.insert(
            AchievementId::ForensicsExpert,
            Achievement::new(
                AchievementId::ForensicsExpert,
                "🔬 Forensics Expert",
                "Complete all advanced forensics challenges",
                "🔬",
            ),
        );

        // Behavioral achievements
        achievements.insert(
            AchievementId::Persistent,
            Achievement::new(
                AchievementId::Persistent,
                "💪 Persistent",
                "Play the game across 5 different sessions",
                "💪",
            ),
        );

        achievements.insert(
            AchievementId::ThemeMaster,
            Achievement::new(
                AchievementId::ThemeMaster,
                "🎨 Theme Master",
                "Try all available color themes",
                "🎨",
            ),
        );

        achievements.insert(
            AchievementId::CompletePerfection,
            Achievement::new(
                AchievementId::CompletePerfection,
                "💎 Complete Perfection",
                "Complete all challenges while maintaining 100% sanity",
                "💎",
            ),
        );

        achievements.insert(
            AchievementId::RapidResponse,
            Achievement::new(
                AchievementId::RapidResponse,
                "🚀 Rapid Response",
                "Complete 3 challenges within 10 minutes",
                "🚀",
            ),
        );

        // Discovery achievements
        achievements.insert(
            AchievementId::SecretSeeker,
            Achievement::new(
                AchievementId::SecretSeeker,
                "🕵️ Secret Seeker",
                "Discover all hidden easter eggs",
                "🕵️",
            ),
        );

        achievements.insert(
            AchievementId::TerminalMaster,
            Achievement::new(
                AchievementId::TerminalMaster,
                "💻 Terminal Master",
                "Use advanced terminal commands and features",
                "💻",
            ),
        );

        achievements.insert(
            AchievementId::TutorialGraduate,
            Achievement::new(
                AchievementId::TutorialGraduate,
                "🎓 Tutorial Graduate",
                "Complete the tutorial successfully",
                "🎓",
            ),
        );

        achievements
    }

    pub fn unlock_achievement(&mut self, achievement_id: AchievementId) -> bool {
        if let Some(achievement) = self.achievements.get_mut(&achievement_id) {
            if !achievement.is_unlocked() {
                achievement.unlock();
                return true; // Newly unlocked
            }
        }
        false // Already unlocked or doesn't exist
    }

    pub fn is_achievement_unlocked(&self, achievement_id: &AchievementId) -> bool {
        self.achievements
            .get(achievement_id)
            .map_or(false, |a| a.is_unlocked())
    }

    pub fn get_unlocked_achievements(&self) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| a.is_unlocked())
            .collect()
    }

    pub fn get_achievement_progress(&self) -> (usize, usize) {
        let unlocked = self
            .achievements
            .values()
            .filter(|a| a.is_unlocked())
            .count();
        let total = self.achievements.len();
        (unlocked, total)
    }

    pub fn check_and_unlock_achievements(&mut self) -> Vec<AchievementId> {
        let mut newly_unlocked = Vec::new();

        // Check progress-based achievements
        if self.completed_challenges.len() >= 1 {
            if self.unlock_achievement(AchievementId::FirstBlood) {
                newly_unlocked.push(AchievementId::FirstBlood);
            }
        }

        if self.completed_challenges.len() >= 17 {
            // All challenges including new OSINT ones
            if self.unlock_achievement(AchievementId::GhostHunter) {
                newly_unlocked.push(AchievementId::GhostHunter);
            }
        }

        if self.sanity >= 75 && self.completed_challenges.len() >= 5 {
            if self.unlock_achievement(AchievementId::SanityReserves) {
                newly_unlocked.push(AchievementId::SanityReserves);
            }
        }

        if self.sanity == 100 && self.completed_challenges.len() >= 17 {
            if self.unlock_achievement(AchievementId::CompletePerfection) {
                newly_unlocked.push(AchievementId::CompletePerfection);
            }
        }

        if self.session_count >= 5 {
            if self.unlock_achievement(AchievementId::Persistent) {
                newly_unlocked.push(AchievementId::Persistent);
            }
        }

        if self.themes_tried.len() >= 5 {
            // Assuming 5+ themes available
            if self.unlock_achievement(AchievementId::ThemeMaster) {
                newly_unlocked.push(AchievementId::ThemeMaster);
            }
        }

        if self.tutorial_completed {
            if self.unlock_achievement(AchievementId::TutorialGraduate) {
                newly_unlocked.push(AchievementId::TutorialGraduate);
            }
        }

        if self.discovered_secrets.len() >= 3 {
            // Assuming 3+ secrets available
            if self.unlock_achievement(AchievementId::SecretSeeker) {
                newly_unlocked.push(AchievementId::SecretSeeker);
            }
        }

        // Check category-based achievements by analyzing completed challenges
        self.check_category_achievements(&mut newly_unlocked);

        newly_unlocked
    }

    fn check_category_achievements(&mut self, newly_unlocked: &mut Vec<AchievementId>) {
        // Count challenges by category (simplified - in real implementation,
        // you'd parse challenge IDs or maintain category tracking)

        let cryptography_challenges =
            ["caesar_cipher", "rot13_ghost", "md5_collision", "jwt_token"];
        let network_challenges = ["port_scan", "path_traversal", "command_injection"];
        let web_challenges = [
            "sql_injection_basics",
            "xss_attack",
            "cors_bypass",
            "session_hijack",
        ];
        let osint_challenges = [
            "osint_social_media",
            "osint_domain_recon",
            "osint_email_analysis",
            "osint_geolocation",
            "osint_breach_investigation",
        ];

        if cryptography_challenges
            .iter()
            .all(|c| self.completed_challenges.contains(*c))
        {
            if self.unlock_achievement(AchievementId::CryptographyMaster) {
                newly_unlocked.push(AchievementId::CryptographyMaster);
            }
        }

        if network_challenges
            .iter()
            .all(|c| self.completed_challenges.contains(*c))
        {
            if self.unlock_achievement(AchievementId::NetworkNinja) {
                newly_unlocked.push(AchievementId::NetworkNinja);
            }
        }

        if web_challenges
            .iter()
            .all(|c| self.completed_challenges.contains(*c))
        {
            if self.unlock_achievement(AchievementId::WebWarrior) {
                newly_unlocked.push(AchievementId::WebWarrior);
            }
        }

        if osint_challenges
            .iter()
            .all(|c| self.completed_challenges.contains(*c))
        {
            if self.unlock_achievement(AchievementId::OSINTOperative) {
                newly_unlocked.push(AchievementId::OSINTOperative);
            }
        }

        // Check if user has tried challenges from all categories
        let has_crypto = cryptography_challenges
            .iter()
            .any(|c| self.completed_challenges.contains(*c));
        let has_network = network_challenges
            .iter()
            .any(|c| self.completed_challenges.contains(*c));
        let has_web = web_challenges
            .iter()
            .any(|c| self.completed_challenges.contains(*c));
        let has_osint = osint_challenges
            .iter()
            .any(|c| self.completed_challenges.contains(*c));

        if has_crypto && has_network && has_web && has_osint {
            if self.unlock_achievement(AchievementId::Explorer) {
                newly_unlocked.push(AchievementId::Explorer);
            }
        }
    }

    pub fn start_challenge(&mut self, challenge_id: &str) {
        self.challenge_start_times
            .insert(challenge_id.to_string(), chrono::Utc::now());
    }

    pub fn complete_challenge_with_timing(
        &mut self,
        challenge_id: &str,
        reward_xp: i32,
        hints_used: usize,
    ) -> Vec<AchievementId> {
        // Record completion
        self.complete_challenge(challenge_id, reward_xp);

        let mut newly_unlocked = Vec::new();

        // Check speed achievement
        if let Some(start_time) = self.challenge_start_times.get(challenge_id) {
            let duration = chrono::Utc::now().signed_duration_since(*start_time);
            if duration.num_minutes() < 2 {
                if self.unlock_achievement(AchievementId::SpeedDemon) {
                    newly_unlocked.push(AchievementId::SpeedDemon);
                }
            }
        }

        // Check hint-free achievement
        if hints_used == 0 {
            if self.unlock_achievement(AchievementId::HintFree) {
                newly_unlocked.push(AchievementId::HintFree);
            }
        }

        // Check other achievements
        newly_unlocked.extend(self.check_and_unlock_achievements());

        newly_unlocked
    }

    pub fn increment_session(&mut self) {
        self.session_count += 1;
    }

    pub fn add_theme_tried(&mut self, theme_name: &str) {
        self.themes_tried.insert(theme_name.to_string());
    }

    // ===== v1.2.0 Advanced Command System Methods =====

    /// Add or update a user alias
    pub fn add_alias(&mut self, alias: &str, command: &str) {
        self.user_preferences
            .user_aliases
            .insert(alias.to_string(), command.to_string());
    }

    /// Remove a user alias
    pub fn remove_alias(&mut self, alias: &str) -> bool {
        self.user_preferences.user_aliases.remove(alias).is_some()
    }

    /// Get command for alias, or None if not found
    pub fn get_alias(&self, alias: &str) -> Option<&String> {
        self.user_preferences.user_aliases.get(alias)
    }

    /// List all user aliases
    pub fn list_aliases(&self) -> &HashMap<String, String> {
        &self.user_preferences.user_aliases
    }

    /// Resolve command through aliases (supports chaining)
    pub fn resolve_command(&self, input: &str) -> String {
        let mut resolved = input.to_string();
        let mut seen_aliases = HashSet::new();

        // Prevent infinite loops by tracking seen aliases
        while let Some(command) = self.get_alias(&resolved) {
            if seen_aliases.contains(&resolved) {
                break; // Circular alias detected
            }
            seen_aliases.insert(resolved.clone());
            resolved = command.clone();
        }

        resolved
    }

    // ===== v1.2.0 User Preferences Methods =====

    /// Update difficulty scaling preference
    pub fn set_difficulty_scaling(&mut self, scaling: DifficultyScaling) {
        self.user_preferences.difficulty_scaling = scaling;
    }

    /// Get current difficulty scaling
    pub fn get_difficulty_scaling(&self) -> &DifficultyScaling {
        &self.user_preferences.difficulty_scaling
    }

    /// Update hint verbosity preference
    pub fn set_hint_verbosity(&mut self, verbosity: HintVerbosity) {
        self.user_preferences.hint_verbosity = verbosity;
    }

    /// Get current hint verbosity
    pub fn get_hint_verbosity(&self) -> &HintVerbosity {
        &self.user_preferences.hint_verbosity
    }

    /// Set preferred color theme
    pub fn set_color_theme(&mut self, theme: &str) {
        self.user_preferences.color_theme = theme.to_string();
    }

    /// Get current color theme preference
    pub fn get_color_theme(&self) -> &String {
        &self.user_preferences.color_theme
    }

    /// Set font size preference
    pub fn set_font_size(&mut self, size: FontSize) {
        self.user_preferences.font_size = size;
    }

    /// Get current font size
    pub fn get_font_size(&self) -> &FontSize {
        &self.user_preferences.font_size
    }

    /// Toggle audio preference
    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.user_preferences.audio_enabled = enabled;
    }

    /// Get audio enabled status
    pub fn is_audio_enabled(&self) -> bool {
        self.user_preferences.audio_enabled
    }

    /// Set animation speed preference
    pub fn set_animation_speed(&mut self, speed: AnimationSpeed) {
        self.user_preferences.animation_speed = speed;
    }

    /// Get current animation speed
    pub fn get_animation_speed(&self) -> &AnimationSpeed {
        &self.user_preferences.animation_speed
    }

    // ===== v1.2.0 Progress Analytics Methods =====

    /// Record challenge attempt
    pub fn record_challenge_attempt(&mut self, challenge_id: &str) {
        *self
            .progress_analytics
            .challenges_attempted
            .entry(challenge_id.to_string())
            .or_insert(0) += 1;
    }

    /// Record hint usage
    pub fn record_hint_used(&mut self, challenge_id: &str) {
        *self
            .progress_analytics
            .hints_used_per_challenge
            .entry(challenge_id.to_string())
            .or_insert(0) += 1;
    }

    /// Record challenge completion time
    pub fn record_completion_time(&mut self, challenge_id: &str, seconds: u64) {
        self.progress_analytics
            .completion_times
            .insert(challenge_id.to_string(), seconds);
    }

    /// Set difficulty preference for a challenge
    pub fn set_challenge_difficulty_preference(&mut self, challenge_id: &str, difficulty: &str) {
        self.progress_analytics
            .difficulty_preferences
            .insert(challenge_id.to_string(), difficulty.to_string());
    }

    /// Get completion rate percentage
    pub fn get_completion_rate(&self) -> f32 {
        if self.progress_analytics.challenges_attempted.is_empty() {
            return 0.0;
        }

        let total_attempts: u32 = self.progress_analytics.challenges_attempted.values().sum();
        let unique_completions = self.completed_challenges.len() as u32;

        if total_attempts > 0 {
            (unique_completions as f32 / total_attempts as f32) * 100.0
        } else {
            0.0
        }
    }

    /// Get average completion time for completed challenges
    pub fn get_average_completion_time(&self) -> Option<u64> {
        if self.progress_analytics.completion_times.is_empty() {
            return None;
        }

        let total: u64 = self.progress_analytics.completion_times.values().sum();
        Some(total / self.progress_analytics.completion_times.len() as u64)
    }

    /// Increment learning streak
    pub fn increment_learning_streak(&mut self) {
        self.progress_analytics.learning_streaks += 1;
    }

    /// Reset learning streak (called on failed attempt)
    pub fn reset_learning_streak(&mut self) {
        self.progress_analytics.learning_streaks = 0;
    }

    /// Update session analytics
    pub fn update_session_analytics(&mut self, minutes_played: u64) {
        self.progress_analytics.total_playtime_minutes += minutes_played;
        self.progress_analytics.last_session = chrono::Utc::now();
    }

    // ===== v1.6.0 Branching Narrative Methods =====

    /// Add a story flag when a narrative choice is made
    pub fn add_story_flag(&mut self, flag: String) {
        self.story_flags.insert(flag);
    }

    /// Check if a story flag is set
    pub fn has_story_flag(&self, flag: &str) -> bool {
        self.story_flags.contains(flag)
    }

    /// Get all story flags
    pub fn get_story_flags(&self) -> &HashSet<String> {
        &self.story_flags
    }

    /// Record a narrative choice
    pub fn record_narrative_choice(&mut self, branch_id: String) {
        self.choice_history.push(branch_id);
    }

    /// Get narrative choice history
    pub fn get_choice_history(&self) -> &[String] {
        &self.choice_history
    }

    /// Set active narrative branch
    pub fn set_active_narrative_branch(&mut self, branch: String) {
        self.active_narrative_branch = branch;
    }

    /// Get active narrative branch
    pub fn get_active_narrative_branch(&self) -> &str {
        &self.active_narrative_branch
    }

    /// Check if a narrative branch has been visited
    pub fn has_visited_branch(&self, branch_prefix: &str) -> bool {
        self.choice_history
            .iter()
            .any(|choice| choice.starts_with(branch_prefix))
    }

    // ===== v1.2.0 Enhanced Save System Methods =====

    /// Save to specific slot (0-4)
    pub fn save_to_slot(&self, slot: u8) -> std::io::Result<()> {
        if slot > 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Save slot must be 0-4",
            ));
        }

        let filename = format!("save_slot_{}.json", slot);
        self.save_to(&filename)
    }

    /// Load from specific slot
    pub fn load_from_slot(slot: u8) -> std::io::Result<GameState> {
        if slot > 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Save slot must be 0-4",
            ));
        }

        let filename = format!("save_slot_{}.json", slot);
        Self::load_from(&filename)
    }

    /// Export game state as compressed JSON
    pub fn export_to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Import game state from JSON string
    pub fn import_from_string(json: &str) -> Result<GameState, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Check if save slot exists
    pub fn slot_exists(slot: u8) -> bool {
        if slot > 4 {
            return false;
        }

        let filename = format!("save_slot_{}.json", slot);
        std::path::Path::new(&filename).exists()
    }

    /// Get save slot metadata (size, last modified, etc.)
    pub fn get_slot_metadata(slot: u8) -> Option<SaveSlotMetadata> {
        if slot > 4 || !Self::slot_exists(slot) {
            return None;
        }

        let filename = format!("save_slot_{}.json", slot);
        if let Ok(metadata) = std::fs::metadata(&filename) {
            if let Ok(state) = Self::load_from_slot(slot) {
                return Some(SaveSlotMetadata {
                    slot,
                    player_name: state.player_name,
                    level: state.current_level,
                    completed_challenges: state.completed_challenges.len(),
                    file_size: metadata.len(),
                    last_modified: metadata.modified().ok(),
                });
            }
        }

        None
    }
}

/// Metadata for save slot information (v1.2.0)
#[derive(Debug, Clone)]
pub struct SaveSlotMetadata {
    pub slot: u8,
    pub player_name: String,
    pub level: usize,
    pub completed_challenges: usize,
    pub file_size: u64,
    pub last_modified: Option<std::time::SystemTime>,
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
