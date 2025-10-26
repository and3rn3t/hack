# 🚀 Version 1.6.0 Planning Document

**Target Release**: Q4 2025 (November-December)
**Focus Areas**: UI/UX Polish + Content Enhancement
**Status**: Planning Phase
**Created**: October 26, 2025

---

## 📋 Executive Summary

Version 1.6.0 builds on the successful v1.5.0 release by enhancing user experience through visual polish and expanding educational content with branching narratives and enhanced achievements.

**Key Goals**:

1. **UI/UX Polish** - Improve visual feedback, loading experience, mobile touch
2. **Branching Narratives** - Add choice-driven story paths that affect gameplay
3. **Achievement Expansion** - More gamification with diverse achievement types
4. **Mobile Optimization** - Better touch controls and responsive design

---

## 🎨 Part 1: UI/UX Enhancements

### 1.1 Enhanced Loading Experience

**Current State**:

- Basic loading screen with progress bar
- Simple "Initializing Ghost Protocol..." text
- Minimal animation (pulse + glow)

**Proposed Improvements**:

```html
<!-- Enhanced loading with status messages -->
<div class="loading-screen">
    <div class="ghost-logo">
        <!-- Animated ASCII ghost -->
        <pre class="ascii-art animated">
    ▓▓▓▓▓▓▓▓▓▓▓
  ▓▓▓▓▓▓▓▓▓▓▓▓▓
 ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▒▒▓▓▓▓▓▒▒▓▓▓▓
▓▓▓▒▒▓▓▓▓▓▒▒▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓███▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
 ▓▓ ▓▓ ▓▓ ▓▓ ▓▓
▓  ▓  ▓  ▓  ▓  ▓
        </pre>
    </div>

    <div class="loading-status">
        <div class="status-text" id="loadingStatus">
            Initializing Ghost Protocol...
        </div>
        <div class="loading-bar">
            <div class="loading-progress" id="loadingProgress"></div>
            <div class="loading-percentage" id="loadingPercent">0%</div>
        </div>
    </div>

    <div class="loading-tips">
        <div class="tip-text" id="loadingTip">
            💡 Tip: Use 'hint' command when stuck on challenges
        </div>
    </div>
</div>
```

**Loading Status Messages**:

```javascript
const LOADING_STAGES = [
    { percent: 0, message: "⚡ Initializing Ghost Protocol..." },
    { percent: 20, message: "🔐 Loading encryption modules..." },
    { percent: 40, message: "🎯 Preparing 48 challenges..." },
    { percent: 60, message: "👻 Awakening the ghost..." },
    { percent: 80, message: "🌐 Establishing connection..." },
    { percent: 100, message: "✅ Ready to hack..." }
];

const LOADING_TIPS = [
    "💡 Tip: Use 'hint' command when stuck on challenges",
    "💡 Tip: Your sanity decreases with each challenge",
    "💡 Tip: Tab completion works for all commands",
    "💡 Tip: Try different color themes with 'theme' command",
    "💡 Tip: Don't trust everything the ghost tells you...",
    "💡 Tip: Some challenges have hidden clues in the descriptions",
    "💡 Tip: Save your progress frequently!",
    "💡 Tip: Commands are case-insensitive for your convenience"
];
```

**CSS Animations**:

```css
/* Smooth loading transitions */
@keyframes ghost-float {
    0%, 100% { transform: translateY(0px); }
    50% { transform: translateY(-10px); }
}

@keyframes tip-fade {
    0%, 100% { opacity: 0.5; }
    50% { opacity: 1; }
}

@keyframes progress-shimmer {
    0% { background-position: -100% 0; }
    100% { background-position: 200% 0; }
}

.ascii-art.animated {
    animation: ghost-float 3s ease-in-out infinite;
}

.loading-progress {
    background: linear-gradient(
        90deg,
        var(--text-accent) 0%,
        rgba(0, 255, 65, 0.5) 50%,
        var(--text-accent) 100%
    );
    background-size: 200% 100%;
    animation: progress-shimmer 2s linear infinite;
}

.tip-text {
    animation: tip-fade 3s infinite;
}
```

**Implementation Tasks**:

- [ ] Add ASCII ghost logo to loading screen
- [ ] Implement progressive loading messages
- [ ] Add rotating tips display
- [ ] Create smooth percentage counter
- [ ] Add fade-out transition when loading completes
- [ ] Test on slow connections (<100KB/s)

**Estimated Time**: 2-3 hours

---

### 1.2 Mobile Touch Enhancements

**Current Issues**:

- Keyboard input requires text selection on mobile
- No haptic feedback for actions
- Difficult to scroll terminal on mobile
- Small touch targets for buttons

**Proposed Solutions**:

```javascript
// Touch gesture handler
class MobileTouchHandler {
    constructor() {
        this.touchStartY = 0;
        this.touchStartX = 0;
        this.isScrolling = false;
    }

    handleTouchStart(e) {
        this.touchStartY = e.touches[0].clientY;
        this.touchStartX = e.touches[0].clientX;
    }

    handleTouchMove(e) {
        const deltaY = e.touches[0].clientY - this.touchStartY;
        const deltaX = e.touches[0].clientX - this.touchStartX;

        // Detect swipe gestures
        if (Math.abs(deltaX) > 50 && Math.abs(deltaY) < 30) {
            if (deltaX > 0) {
                this.onSwipeRight(); // Previous command
            } else {
                this.onSwipeLeft(); // Next command
            }
        }

        // Vertical scroll
        if (Math.abs(deltaY) > 10) {
            this.isScrolling = true;
        }
    }

    // Haptic feedback for supported devices
    vibrate(pattern = [10]) {
        if ('vibrate' in navigator) {
            navigator.vibrate(pattern);
        }
    }
}

// Virtual keyboard with common commands
const QUICK_COMMANDS = [
    'help', 'hint', 'list', 'status', 'save', 'theme'
];
```

**Mobile-Optimized Controls**:

```html
<!-- Quick command bar for mobile -->
<div class="mobile-quick-bar" id="mobileQuickBar">
    <button class="quick-cmd" data-cmd="help">❓ Help</button>
    <button class="quick-cmd" data-cmd="hint">💡 Hint</button>
    <button class="quick-cmd" data-cmd="list">📋 List</button>
    <button class="quick-cmd" data-cmd="status">📊 Status</button>
    <button class="quick-cmd" data-cmd="save">💾 Save</button>
</div>

<!-- Touch-friendly input area -->
<div class="mobile-input-area">
    <input
        type="text"
        id="mobileInput"
        placeholder="Enter command..."
        autocomplete="off"
        autocorrect="off"
        autocapitalize="off"
        spellcheck="false"
    />
    <button class="mobile-submit">▶</button>
</div>
```

**Responsive Design Improvements**:

```css
/* Enhanced mobile breakpoints */
@media (max-width: 768px) {
    .header {
        flex-direction: column;
        padding: 15px;
    }

    .stats {
        width: 100%;
        justify-content: space-around;
        margin-top: 10px;
    }

    .controls {
        overflow-x: auto;
        -webkit-overflow-scrolling: touch;
    }

    .btn {
        min-width: 80px;
        padding: 12px 20px; /* Larger touch targets */
    }

    .mobile-quick-bar {
        display: flex;
        gap: 8px;
        padding: 10px;
        background: var(--bg-secondary);
        border-top: 1px solid var(--border-color);
        overflow-x: auto;
    }

    .quick-cmd {
        flex-shrink: 0;
        padding: 10px 15px;
        border: 1px solid var(--border-color);
        background: var(--bg-primary);
        color: var(--text-primary);
        border-radius: 4px;
        min-width: 70px;
    }
}

@media (max-width: 480px) {
    .title {
        font-size: 1em;
    }

    .stat {
        font-size: 0.8em;
    }

    .loading-text {
        font-size: 1em;
        padding: 0 20px;
        text-align: center;
    }
}
```

**Implementation Tasks**:

- [ ] Add swipe gesture detection for command history
- [ ] Implement haptic feedback for button clicks
- [ ] Create mobile quick command bar
- [ ] Add dedicated mobile input field
- [ ] Improve touch target sizes (min 44x44px)
- [ ] Test on iOS Safari and Android Chrome
- [ ] Add pull-to-refresh gesture

**Estimated Time**: 3-4 hours

---

### 1.3 Sound Effects System (Optional)

**Proposed Features**:

```javascript
class SoundEffects {
    constructor() {
        this.enabled = true;
        this.volume = 0.3;
        this.sounds = {
            type: null,      // Typing sound
            success: null,   // Challenge completed
            error: null,     // Wrong answer
            hint: null,      // Hint used
            sanity: null,    // Sanity warning
            ghost: null,     // Horror effect
            levelup: null    // Level up
        };
    }

    async loadSounds() {
        // Load Web Audio API sounds
        const audioContext = new (window.AudioContext || window.webkitAudioContext)();

        // Generate simple procedural sounds
        this.sounds.type = this.generateClick(audioContext);
        this.sounds.success = this.generateSuccess(audioContext);
        this.sounds.error = this.generateError(audioContext);
    }

    play(soundName) {
        if (!this.enabled || !this.sounds[soundName]) return;

        const sound = this.sounds[soundName].cloneNode();
        sound.volume = this.volume;
        sound.play().catch(() => {
            // Autoplay blocked, ignore
        });
    }

    // Generate procedural keyboard click
    generateClick(ctx) {
        const buffer = ctx.createBuffer(1, ctx.sampleRate * 0.05, ctx.sampleRate);
        const data = buffer.getChannelData(0);

        for (let i = 0; i < buffer.length; i++) {
            data[i] = Math.random() * 0.1 * Math.exp(-i / 500);
        }

        return buffer;
    }
}

// Settings UI
<div class="settings-panel">
    <h3>Sound Settings</h3>
    <label>
        <input type="checkbox" id="soundEnabled" checked />
        Enable Sound Effects
    </label>
    <label>
        <input type="range" id="soundVolume" min="0" max="100" value="30" />
        Volume: <span id="volumePercent">30%</span>
    </label>
</div>
```

**Implementation Tasks**:

- [ ] Create procedural sound generator
- [ ] Add sound toggle in settings
- [ ] Implement volume control
- [ ] Add typing sounds (optional)
- [ ] Add success/error feedback sounds
- [ ] Respect user's autoplay preferences

**Estimated Time**: 2-3 hours

---

## 📖 Part 2: Branching Narrative System

### 2.1 Narrative Architecture

**Current State**:

- Linear story progression
- Fixed narrative events
- No player choices affecting story

**Proposed System**:

```rust
// src/narrative.rs - Enhanced narrative system

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeBranch {
    pub id: String,
    pub trigger_level: u32,
    pub trigger_sanity: Option<u32>,
    pub choices: Vec<NarrativeChoice>,
    pub default_branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeChoice {
    pub text: String,
    pub response: String,
    pub branch_id: String,
    pub effects: ChoiceEffects,
    pub requirements: Option<ChoiceRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceEffects {
    pub sanity_change: i32,
    pub xp_change: i32,
    pub unlock_challenges: Vec<String>,
    pub story_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRequirements {
    pub min_level: Option<u32>,
    pub required_challenges: Option<Vec<String>>,
    pub required_flags: Option<Vec<String>>,
}

pub struct BranchingNarrative {
    branches: HashMap<String, NarrativeBranch>,
    active_branch: String,
    story_flags: HashSet<String>,
    choice_history: Vec<String>,
}

impl BranchingNarrative {
    pub fn new() -> Self {
        let mut narrative = Self {
            branches: HashMap::new(),
            active_branch: "main".to_string(),
            story_flags: HashSet::new(),
            choice_history: Vec::new(),
        };
        narrative.initialize_branches();
        narrative
    }

    fn initialize_branches(&mut self) {
        // Level 1 Branch: First Ghost Encounter
        self.branches.insert(
            "ghost_encounter_1".to_string(),
            NarrativeBranch {
                id: "ghost_encounter_1".to_string(),
                trigger_level: 1,
                trigger_sanity: None,
                choices: vec![
                    NarrativeChoice {
                        text: "Ask the ghost who they are".to_string(),
                        response: "The ghost whispers: 'I am what remains... what you will become...'".to_string(),
                        branch_id: "ghost_reveal_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -5,
                            xp_change: 10,
                            unlock_challenges: vec![],
                            story_flags: vec!["ghost_questioned".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Ignore the ghost and continue hacking".to_string(),
                        response: "You focus on the challenges. The ghost grows silent... for now.".to_string(),
                        branch_id: "focused_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 0,
                            xp_change: 5,
                            unlock_challenges: vec![],
                            story_flags: vec!["ghost_ignored".to_string()],
                        },
                        requirements: None,
                    },
                ],
                default_branch: "main".to_string(),
            },
        );

        // Level 2 Branch: Sanity Crisis
        self.branches.insert(
            "sanity_crisis".to_string(),
            NarrativeBranch {
                id: "sanity_crisis".to_string(),
                trigger_level: 2,
                trigger_sanity: Some(50),
                choices: vec![
                    NarrativeChoice {
                        text: "Take a break and restore sanity (+10 sanity)".to_string(),
                        response: "You step back... breathe... but the ghost's laughter echoes.".to_string(),
                        branch_id: "recovery_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: 10,
                            xp_change: 0,
                            unlock_challenges: vec![],
                            story_flags: vec!["took_break".to_string()],
                        },
                        requirements: None,
                    },
                    NarrativeChoice {
                        text: "Push through the madness (-5 sanity, unlock bonus challenge)".to_string(),
                        response: "You embrace the chaos. New paths become visible in the darkness...".to_string(),
                        branch_id: "madness_path".to_string(),
                        effects: ChoiceEffects {
                            sanity_change: -5,
                            xp_change: 50,
                            unlock_challenges: vec!["hidden_challenge_1".to_string()],
                            story_flags: vec!["embraced_madness".to_string()],
                        },
                        requirements: None,
                    },
                ],
                default_branch: "main".to_string(),
            },
        );
    }

    pub fn check_branch_trigger(
        &self,
        level: u32,
        sanity: u32,
    ) -> Option<&NarrativeBranch> {
        self.branches.values().find(|branch| {
            branch.trigger_level == level
                && branch.trigger_sanity.map_or(true, |req| sanity <= req)
        })
    }

    pub fn make_choice(
        &mut self,
        choice_index: usize,
    ) -> Result<&ChoiceEffects, String> {
        let branch = self.branches.get(&self.active_branch)
            .ok_or("Invalid branch")?;

        let choice = branch.choices.get(choice_index)
            .ok_or("Invalid choice")?;

        // Record choice
        self.choice_history.push(choice.branch_id.clone());

        // Apply story flags
        for flag in &choice.effects.story_flags {
            self.story_flags.insert(flag.clone());
        }

        // Update active branch
        self.active_branch = choice.branch_id.clone();

        Ok(&choice.effects)
    }
}
```

**Example Narrative Branches**:

1. **Level 1: First Ghost Encounter**
   - Choice A: Ask who the ghost is → Reveal lore, lose sanity
   - Choice B: Ignore and continue → Stay focused, less XP

2. **Level 2: Sanity Crisis (when sanity < 50)**
   - Choice A: Take a break → Restore +10 sanity
   - Choice B: Embrace madness → Unlock secret challenge, lose sanity

3. **Level 3: Ghost's Bargain**
   - Choice A: Accept ghost's help → Get hints, risk corruption
   - Choice B: Refuse → Harder path, maintain independence

4. **Level 4: The Truth Emerges**
   - Choice A: Investigate the ghost → Learn twist earlier
   - Choice B: Focus on escaping → Different ending path

**Implementation Tasks**:

- [ ] Create `BranchingNarrative` struct in `narrative.rs`
- [ ] Design 5-7 major choice points
- [ ] Implement choice UI in web interface
- [ ] Add story flag tracking to save system
- [ ] Write branching dialogue for each path
- [ ] Test all narrative combinations
- [ ] Add "Story Path" visualization in stats

**Estimated Time**: 6-8 hours

---

### 2.2 Choice UI Implementation

**Web Interface**:

```javascript
// Display narrative choice dialog
function showNarrativeChoice(branch) {
    const modal = document.createElement('div');
    modal.className = 'narrative-modal';
    modal.innerHTML = `
        <div class="narrative-content">
            <h2 class="narrative-title">A Choice Emerges...</h2>
            <div class="narrative-description">
                ${branch.description}
            </div>
            <div class="narrative-choices">
                ${branch.choices.map((choice, index) => `
                    <button
                        class="narrative-choice-btn"
                        onclick="makeNarrativeChoice(${index})"
                        ${choice.requirements ? 'data-requirements="true"' : ''}
                    >
                        <div class="choice-text">${choice.text}</div>
                        <div class="choice-effects">
                            ${formatEffects(choice.effects)}
                        </div>
                    </button>
                `).join('')}
            </div>
        </div>
    `;

    document.body.appendChild(modal);
}

function formatEffects(effects) {
    const parts = [];
    if (effects.sanity_change !== 0) {
        const sign = effects.sanity_change > 0 ? '+' : '';
        parts.push(`<span class="effect-sanity">${sign}${effects.sanity_change} Sanity</span>`);
    }
    if (effects.xp_change > 0) {
        parts.push(`<span class="effect-xp">+${effects.xp_change} XP</span>`);
    }
    if (effects.unlock_challenges.length > 0) {
        parts.push(`<span class="effect-unlock">🔓 Unlocks hidden challenge</span>`);
    }
    return parts.join(' • ');
}
```

**CSS Styling**:

```css
.narrative-modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.95);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 2000;
    animation: fadeIn 0.5s ease;
}

.narrative-content {
    max-width: 600px;
    padding: 30px;
    background: var(--bg-primary);
    border: 2px solid var(--text-danger);
    box-shadow: 0 0 30px var(--text-danger);
}

.narrative-title {
    color: var(--text-danger);
    text-align: center;
    margin-bottom: 20px;
    font-size: 1.5em;
    text-shadow: 0 0 10px var(--text-danger);
}

.narrative-choices {
    display: flex;
    flex-direction: column;
    gap: 15px;
    margin-top: 20px;
}

.narrative-choice-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    padding: 15px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
}

.narrative-choice-btn:hover {
    border-color: var(--text-accent);
    box-shadow: 0 0 15px rgba(0, 255, 65, 0.3);
    transform: translateX(5px);
}

.choice-effects {
    margin-top: 10px;
    font-size: 0.9em;
    color: var(--text-muted);
}

.effect-sanity { color: var(--text-warning); }
.effect-xp { color: var(--text-accent); }
.effect-unlock { color: #00ffff; }
```

---

## 🏆 Part 3: Achievement System Expansion

### 3.1 Enhanced Achievement Types

**Current State**:

- Basic achievement tracking exists
- Limited achievement variety

**New Achievement Categories**:

```rust
// src/state.rs - Achievement system

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AchievementCategory {
    Skill,        // Master specific challenge types
    Behavioral,   // Play style achievements
    Discovery,    // Find secrets
    Speed,        // Time-based challenges
    Completionist,// Complete collections
    Social,       // Share/compete achievements
    Horror,       // Horror-themed special achievements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub rarity: AchievementRarity,
    pub icon: String,
    pub requirements: AchievementRequirements,
    pub reward_xp: u32,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRequirements {
    pub challenges_completed: Option<Vec<String>>,
    pub category_mastery: Option<(ChallengeCategory, u32)>,
    pub speedrun_time: Option<Duration>,
    pub no_hints_used: Option<bool>,
    pub story_flags: Option<Vec<String>>,
    pub sanity_threshold: Option<u32>,
}

// Achievement definitions
pub fn get_all_achievements() -> Vec<Achievement> {
    vec![
        // Skill-Based
        Achievement {
            id: "crypto_master".to_string(),
            name: "Cryptography Master".to_string(),
            description: "Complete all cryptography challenges".to_string(),
            category: AchievementCategory::Skill,
            rarity: AchievementRarity::Rare,
            icon: "🔐".to_string(),
            requirements: AchievementRequirements {
                category_mastery: Some((ChallengeCategory::Cryptography, 100)),
                ..Default::default()
            },
            reward_xp: 200,
            hidden: false,
        },

        Achievement {
            id: "web_warrior".to_string(),
            name: "Web Security Warrior".to_string(),
            description: "Complete 10 web security challenges".to_string(),
            category: AchievementCategory::Skill,
            rarity: AchievementRarity::Uncommon,
            icon: "🌐".to_string(),
            requirements: AchievementRequirements {
                category_mastery: Some((ChallengeCategory::WebSecurity, 10)),
                ..Default::default()
            },
            reward_xp: 150,
            hidden: false,
        },

        // Behavioral
        Achievement {
            id: "persistent_learner".to_string(),
            name: "Persistent Learner".to_string(),
            description: "Complete a challenge after using all 3 hints".to_string(),
            category: AchievementCategory::Behavioral,
            rarity: AchievementRarity::Common,
            icon: "📚".to_string(),
            requirements: AchievementRequirements {
                ..Default::default()
            },
            reward_xp: 50,
            hidden: false,
        },

        Achievement {
            id: "hintless_hero".to_string(),
            name: "Hintless Hero".to_string(),
            description: "Complete 5 challenges without using any hints".to_string(),
            category: AchievementCategory::Behavioral,
            rarity: AchievementRarity::Epic,
            icon: "🎯".to_string(),
            requirements: AchievementRequirements {
                no_hints_used: Some(true),
                ..Default::default()
            },
            reward_xp: 300,
            hidden: false,
        },

        // Speed
        Achievement {
            id: "speed_hacker".to_string(),
            name: "Speed Hacker".to_string(),
            description: "Complete a challenge in under 30 seconds".to_string(),
            category: AchievementCategory::Speed,
            rarity: AchievementRarity::Rare,
            icon: "⚡".to_string(),
            requirements: AchievementRequirements {
                speedrun_time: Some(Duration::from_secs(30)),
                ..Default::default()
            },
            reward_xp: 150,
            hidden: false,
        },

        // Discovery
        Achievement {
            id: "easter_egg_hunter".to_string(),
            name: "Easter Egg Hunter".to_string(),
            description: "Find a hidden message in the game".to_string(),
            category: AchievementCategory::Discovery,
            rarity: AchievementRarity::Rare,
            icon: "🥚".to_string(),
            requirements: AchievementRequirements {
                story_flags: Some(vec!["found_easter_egg".to_string()]),
                ..Default::default()
            },
            reward_xp: 100,
            hidden: true,
        },

        Achievement {
            id: "ghost_whisperer".to_string(),
            name: "Ghost Whisperer".to_string(),
            description: "Engage with all narrative branches".to_string(),
            category: AchievementCategory::Discovery,
            rarity: AchievementRarity::Epic,
            icon: "👻".to_string(),
            requirements: AchievementRequirements {
                story_flags: Some(vec![
                    "ghost_questioned".to_string(),
                    "embraced_madness".to_string(),
                    "ghost_bargain".to_string(),
                ]),
                ..Default::default()
            },
            reward_xp: 250,
            hidden: true,
        },

        // Horror-Themed
        Achievement {
            id: "sanity_survivor".to_string(),
            name: "Sanity Survivor".to_string(),
            description: "Complete 10 challenges with sanity below 20".to_string(),
            category: AchievementCategory::Horror,
            rarity: AchievementRarity::Legendary,
            icon: "🧠".to_string(),
            requirements: AchievementRequirements {
                sanity_threshold: Some(20),
                ..Default::default()
            },
            reward_xp: 500,
            hidden: false,
        },

        Achievement {
            id: "the_truth".to_string(),
            name: "The Truth".to_string(),
            description: "Discover the final twist... You are the ghost.".to_string(),
            category: AchievementCategory::Horror,
            rarity: AchievementRarity::Legendary,
            icon: "💀".to_string(),
            requirements: AchievementRequirements {
                story_flags: Some(vec!["discovered_truth".to_string()]),
                ..Default::default()
            },
            reward_xp: 1000,
            hidden: true,
        },

        // Completionist
        Achievement {
            id: "level_master".to_string(),
            name: "Level Master".to_string(),
            description: "Reach maximum level".to_string(),
            category: AchievementCategory::Completionist,
            rarity: AchievementRarity::Epic,
            icon: "⭐".to_string(),
            requirements: AchievementRequirements {
                ..Default::default()
            },
            reward_xp: 300,
            hidden: false,
        },

        Achievement {
            id: "completionist".to_string(),
            name: "Completionist".to_string(),
            description: "Complete all 48 challenges".to_string(),
            category: AchievementCategory::Completionist,
            rarity: AchievementRarity::Legendary,
            icon: "🏆".to_string(),
            requirements: AchievementRequirements {
                ..Default::default()
            },
            reward_xp: 1000,
            hidden: false,
        },
    ]
}
```

**Achievement UI**:

```javascript
// Web achievement display
function showAchievementUnlocked(achievement) {
    const popup = document.createElement('div');
    popup.className = 'achievement-popup';
    popup.innerHTML = `
        <div class="achievement-content ${achievement.rarity.toLowerCase()}">
            <div class="achievement-icon">${achievement.icon}</div>
            <div class="achievement-details">
                <div class="achievement-title">Achievement Unlocked!</div>
                <div class="achievement-name">${achievement.name}</div>
                <div class="achievement-description">${achievement.description}</div>
                <div class="achievement-reward">+${achievement.reward_xp} XP</div>
            </div>
        </div>
    `;

    document.body.appendChild(popup);

    // Animate in
    setTimeout(() => popup.classList.add('show'), 100);

    // Auto-remove after 5 seconds
    setTimeout(() => {
        popup.classList.remove('show');
        setTimeout(() => popup.remove(), 500);
    }, 5000);
}

// Achievement list view
function showAchievements() {
    const achievements = getAllAchievements();
    const unlocked = getUnlockedAchievements();

    const html = `
        <div class="achievements-grid">
            ${achievements.map(ach => `
                <div class="achievement-card ${unlocked.includes(ach.id) ? 'unlocked' : 'locked'}">
                    <div class="achievement-icon">${ach.hidden && !unlocked.includes(ach.id) ? '❓' : ach.icon}</div>
                    <div class="achievement-name">
                        ${ach.hidden && !unlocked.includes(ach.id) ? 'Hidden Achievement' : ach.name}
                    </div>
                    <div class="achievement-description">
                        ${ach.hidden && !unlocked.includes(ach.id) ? '???' : ach.description}
                    </div>
                    <div class="achievement-rarity ${ach.rarity.toLowerCase()}">
                        ${ach.rarity}
                    </div>
                </div>
            `).join('')}
        </div>
    `;

    showModal('Achievements', html);
}
```

**CSS**:

```css
.achievement-popup {
    position: fixed;
    top: -200px;
    right: 20px;
    width: 350px;
    background: var(--bg-primary);
    border: 2px solid var(--text-accent);
    padding: 15px;
    border-radius: 8px;
    box-shadow: 0 0 30px var(--text-accent);
    transition: top 0.5s ease;
    z-index: 3000;
}

.achievement-popup.show {
    top: 20px;
}

.achievement-content {
    display: flex;
    gap: 15px;
    align-items: center;
}

.achievement-icon {
    font-size: 3em;
}

.achievement-title {
    color: var(--text-accent);
    font-weight: bold;
    margin-bottom: 5px;
}

.achievement-name {
    font-size: 1.1em;
    margin-bottom: 5px;
}

.achievement-reward {
    color: var(--text-warning);
    font-weight: bold;
    margin-top: 5px;
}

/* Rarity colors */
.legendary { border-color: #ff6b00; box-shadow: 0 0 20px #ff6b00; }
.epic { border-color: #9d00ff; box-shadow: 0 0 15px #9d00ff; }
.rare { border-color: #0099ff; box-shadow: 0 0 10px #0099ff; }
.uncommon { border-color: #00ff41; }
.common { border-color: #666666; }
```

**Implementation Tasks**:

- [ ] Define 20+ achievements across all categories
- [ ] Implement achievement tracking in `state.rs`
- [ ] Create achievement popup UI
- [ ] Add achievement list/grid view
- [ ] Track achievement progress (e.g., 5/10 web challenges)
- [ ] Add achievement sounds
- [ ] Test all achievement conditions

**Estimated Time**: 5-6 hours

---

## 📅 Implementation Timeline

### Week 1: UI/UX Polish (Nov 4-8)

- Day 1-2: Enhanced loading screen with status messages
- Day 3-4: Mobile touch enhancements and gestures
- Day 5: Sound effects system (optional)

### Week 2: Branching Narratives (Nov 11-15)

- Day 1-2: Implement narrative system architecture
- Day 3-4: Write branching dialogue and choices
- Day 5: Add choice UI and testing

### Week 3: Achievement System (Nov 18-22)

- Day 1-2: Define all achievements
- Day 3-4: Implement tracking and UI
- Day 5: Testing and polish

### Week 4: Testing & Release (Nov 25-29)

- Day 1-2: Comprehensive testing all features
- Day 3: Bug fixes and refinements
- Day 4: Documentation updates
- Day 5: v1.6.0 release and deployment

**Total Estimated Time**: 80-100 hours (3-4 weeks part-time)

---

## 🎯 Success Metrics

### User Experience

- [ ] Loading time perception improved (user feedback)
- [ ] Mobile usability score >80 (Lighthouse)
- [ ] Touch target compliance 100% (44x44px minimum)
- [ ] Sound effects enhance experience (user survey)

### Content Engagement

- [ ] Average playtime increases by 20%
- [ ] 60%+ players explore narrative branches
- [ ] Achievement unlock rate >40% for common achievements
- [ ] Hidden achievement discovery rate >10%

### Technical Performance

- [ ] No regression in load times (<5 seconds)
- [ ] Mobile performance score >90
- [ ] Bundle size remains <450KB
- [ ] All 81+ tests passing

---

## 🔄 Future Considerations (v1.7.0+)

### Progressive Web App Features

- Offline mode with service worker
- Install prompts
- Push notifications for achievements
- Background sync for saves

### Social Features

- Anonymous leaderboards
- Challenge ratings
- Community statistics
- Shareable progress cards

### Advanced Challenges

- Time-limited challenges
- Multi-stage challenges
- Community-created challenges
- Seasonal events

---

## 📝 Notes

- Keep horror theme consistent throughout new features
- Ensure all features work in both native and web versions
- Maintain backward compatibility with v1.5.0 saves
- Document all new features in user guide
- Update CHANGELOG.md with all changes

---

**Status**: Planning Complete - Ready for Implementation
**Next Step**: Begin UI/UX polish implementation
**Review Date**: November 1, 2025
