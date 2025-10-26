#[cfg(feature = "native")]
use crate::narrative;
use crate::ui::CompletionContext;
use crate::{state::GameState, ui};
use std::sync::OnceLock;

#[cfg(feature = "web")]
fn default_check_answer() -> fn(&str) -> bool {
    fn always_false(_: &str) -> bool {
        false
    }
    always_false
}

#[cfg(feature = "web")]
fn default_check_answer_override() -> Option<fn(&str) -> bool> {
    None
}

/// Challenge difficulty variant (v1.2.0)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
pub enum ChallengeDifficulty {
    Beginner, // Extra hints, simplified concepts
    Standard, // Default difficulty (current)
    Advanced, // Fewer hints, time pressure
    Expert,   // Minimal hints, real-world complexity
    Dynamic,  // Randomly generated content
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub description: String,
    pub prompt: String,
    pub category: ChallengeCategory,
    pub level: usize,
    pub xp_reward: i32,
    pub sanity_cost: i32,
    #[cfg_attr(feature = "web", serde(skip, default = "default_check_answer"))]
    pub check_answer: fn(&str) -> bool,
    pub solution: String,
    pub hints: Vec<String>,
    // v1.2.0: Challenge variant support
    pub difficulty: ChallengeDifficulty,
    pub variants: Vec<ChallengeVariant>,
}

/// Challenge variant with different difficulty levels (v1.2.0)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
pub struct ChallengeVariant {
    pub difficulty: ChallengeDifficulty,
    pub title_suffix: String, // e.g., " (Expert Mode)"
    pub description_override: Option<String>,
    pub prompt_override: Option<String>,
    pub xp_multiplier: f32, // 0.5 for easy, 1.0 for standard, 2.0 for expert
    pub sanity_multiplier: f32,
    pub hints_override: Option<Vec<String>>,
    pub time_limit: Option<u64>, // in seconds, for advanced/expert modes
    #[cfg_attr(
        feature = "web",
        serde(skip, default = "default_check_answer_override")
    )]
    pub check_answer_override: Option<fn(&str) -> bool>,
    pub solution_override: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "web", derive(serde::Serialize, serde::Deserialize))]
pub enum ChallengeCategory {
    Encoding,
    Cryptography,
    Web,
    Forensics,
    Reverse,
    Binary,
    OSINT,         // Open Source Intelligence (v1.2.0)
    Steganography, // Hidden data techniques (v1.5.0)
    Malware,       // Malware analysis and obfuscation (v1.5.0)
    IoT,           // Internet of Things security (v1.5.0)
}

impl Challenge {
    pub fn new(
        id: &str,
        title: &str,
        description: &str,
        prompt: &str,
        category: ChallengeCategory,
        level: usize,
        xp_reward: i32,
        sanity_cost: i32,
        check_answer: fn(&str) -> bool,
        solution: &str,
        hints: Vec<String>,
    ) -> Self {
        Challenge {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            prompt: prompt.to_string(),
            category,
            level,
            xp_reward,
            sanity_cost,
            check_answer,
            solution: solution.to_string(),
            hints,
            difficulty: ChallengeDifficulty::Standard, // Default to standard
            variants: Vec::new(),                      // No variants by default
        }
    }

    /// Create a challenge with variants (v1.2.0)
    pub fn with_variants(
        id: &str,
        title: &str,
        description: &str,
        prompt: &str,
        category: ChallengeCategory,
        level: usize,
        xp_reward: i32,
        sanity_cost: i32,
        check_answer: fn(&str) -> bool,
        solution: &str,
        hints: Vec<String>,
        variants: Vec<ChallengeVariant>,
    ) -> Self {
        Challenge {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            prompt: prompt.to_string(),
            category,
            level,
            xp_reward,
            sanity_cost,
            check_answer,
            solution: solution.to_string(),
            hints,
            difficulty: ChallengeDifficulty::Standard,
            variants,
        }
    }

    /// Get challenge variant for specific difficulty (v1.2.0)
    pub fn get_variant(&self, difficulty: &ChallengeDifficulty) -> Option<&ChallengeVariant> {
        self.variants.iter().find(|v| &v.difficulty == difficulty)
    }

    /// Create a new challenge instance with applied variant (v1.2.0)
    pub fn with_difficulty(&self, difficulty: ChallengeDifficulty) -> Self {
        let mut challenge = self.clone();
        challenge.difficulty = difficulty.clone();

        if let Some(variant) = self.get_variant(&difficulty) {
            // Apply variant modifications
            if let Some(ref desc) = variant.description_override {
                challenge.description = desc.clone();
            }
            if let Some(ref prompt) = variant.prompt_override {
                challenge.prompt = prompt.clone();
            }
            if let Some(ref hints) = variant.hints_override {
                challenge.hints = hints.clone();
            }
            if let Some(ref solution) = variant.solution_override {
                challenge.solution = solution.clone();
            }
            if let Some(check_fn) = variant.check_answer_override {
                challenge.check_answer = check_fn;
            }

            // Apply multipliers
            challenge.xp_reward = (challenge.xp_reward as f32 * variant.xp_multiplier) as i32;
            challenge.sanity_cost =
                (challenge.sanity_cost as f32 * variant.sanity_multiplier) as i32;

            // Update title with suffix
            if !variant.title_suffix.is_empty() {
                challenge.title = format!("{}{}", challenge.title, variant.title_suffix);
            }
        }

        challenge
    }

    /// Check if challenge has variants available (v1.2.0)
    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    /// Get all available difficulty options for this challenge (v1.2.0)
    pub fn get_available_difficulties(&self) -> Vec<ChallengeDifficulty> {
        let mut difficulties = vec![ChallengeDifficulty::Standard]; // Always available
        for variant in &self.variants {
            difficulties.push(variant.difficulty.clone());
        }
        difficulties
    }

    // Legacy constructor for compatibility during migration
    pub fn new_legacy(
        id: &str,
        title: &str,
        description: &str,
        level: usize,
        xp_reward: i32,
        sanity_cost: i32,
        check_answer: fn(&str) -> bool,
        hints: Vec<String>,
    ) -> Self {
        Challenge {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            prompt: description.to_string(), // Use description as prompt for now
            category: match level {
                0 => ChallengeCategory::Encoding,
                1 => ChallengeCategory::Cryptography,
                2 => ChallengeCategory::Web,
                3 => ChallengeCategory::Forensics,
                _ => ChallengeCategory::Binary,
            },
            level,
            xp_reward,
            sanity_cost,
            check_answer,
            solution: "".to_string(), // Will be populated later
            hints,
            difficulty: ChallengeDifficulty::Standard, // v1.2.0: Default difficulty
            variants: Vec::new(),                      // v1.2.0: No variants by default
        }
    }

    pub fn validate(&self, answer: &str) -> bool {
        (self.check_answer)(answer)
    }

    pub fn attempt(&self, state: &mut GameState) -> std::io::Result<bool> {
        #[cfg(feature = "native")]
        narrative::show_challenge_intro(&self.title, &self.description)?;

        let mut attempts = 0;
        let max_attempts = 5;

        loop {
            println!("\n");
            let input = ui::read_input_with_completion(
                &format!(
                    "Enter your answer (attempt {}/{}) or 'hint' for help or 'skip': ",
                    attempts + 1,
                    max_attempts
                ),
                CompletionContext::Challenge,
                true,
            )?;

            if input.to_lowercase() == "hint" {
                if !self.hints.is_empty() {
                    let hint_index = attempts.min(self.hints.len() - 1);
                    #[cfg(feature = "native")]
                    narrative::show_hint(&self.hints[hint_index])?;
                } else {
                    ui::print_warning("No hints available for this challenge.")?;
                }
                continue;
            }

            if input.to_lowercase() == "skip" {
                ui::print_warning("Challenge skipped. You can retry it later.")?;
                return Ok(false);
            }

            if (self.check_answer)(&input) {
                #[cfg(feature = "native")]
                narrative::show_completion_message(self.xp_reward)?;
                state.complete_challenge(&self.id, self.xp_reward);
                state.modify_sanity(-self.sanity_cost);
                state.save()?;
                ui::pause()?;
                return Ok(true);
            } else {
                attempts += 1;

                // Provide intelligent feedback based on the answer
                self.provide_feedback(&input, attempts)?;

                if attempts >= max_attempts {
                    ui::print_error("Maximum attempts reached. Challenge failed.")?;

                    // Show helpful guidance for next time
                    self.show_learning_resources()?;

                    state.modify_sanity(-10);
                    ui::pause()?;
                    return Ok(false);
                }
            }
        }
    }

    fn provide_feedback(&self, answer: &str, attempt_num: usize) -> std::io::Result<()> {
        // Basic feedback - can be enhanced per challenge
        if attempt_num == 1 {
            ui::print_colored("\n❌ Incorrect. ", ui::Color::Red)?;
            ui::print_colored(
                "Review the challenge description carefully.\n",
                ui::Color::White,
            )?;
        } else if attempt_num == 2 {
            ui::print_colored("\n❌ Still incorrect. ", ui::Color::Red)?;
            ui::print_colored("Consider using 'hint' for guidance.\n", ui::Color::Yellow)?;
        } else if attempt_num >= 3 {
            ui::print_colored("\n❌ Incorrect. ", ui::Color::Red)?;

            // Provide specific feedback based on answer analysis
            if answer.is_empty() {
                ui::print_colored(
                    "Your answer is empty. Please provide a value.\n",
                    ui::Color::White,
                )?;
            } else if answer.len() < 3 {
                ui::print_colored(
                    "Your answer seems very short. Check if you're missing something.\n",
                    ui::Color::White,
                )?;
            } else if answer.len() > 100 {
                ui::print_colored(
                    "Your answer is quite long. The solution might be simpler.\n",
                    ui::Color::White,
                )?;
            } else {
                ui::print_colored(
                    &format!(
                        "You've tried {} times. Use 'hint' if you need help.\n",
                        attempt_num
                    ),
                    ui::Color::White,
                )?;
            }
        }

        Ok(())
    }

    fn show_learning_resources(&self) -> std::io::Result<()> {
        ui::print_separator()?;
        ui::print_colored("\n💡 LEARNING RESOURCES:\n", ui::Color::Cyan)?;

        // Provide category-specific learning tips
        let category_tip = self.get_category_tip();
        ui::print_colored(&format!("   • {}\n", category_tip), ui::Color::White)?;

        ui::print_colored(
            "   • Review the challenge description and hints carefully\n",
            ui::Color::White,
        )?;

        ui::print_colored(
            "   • Try searching online for the concepts mentioned\n",
            ui::Color::White,
        )?;

        Ok(())
    }

    fn get_category_tip(&self) -> String {
        // Determine category based on challenge ID and provide relevant tips
        if self.id.contains("base64")
            || self.id.contains("hex")
            || self.id.contains("binary")
            || self.id.contains("url")
            || self.id.contains("rot13")
        {
            "Practice encoding/decoding techniques - try online tools".to_string()
        } else if self.id.contains("sql") || self.id.contains("http") {
            "Review web security fundamentals and common vulnerabilities".to_string()
        } else if self.id.contains("caesar") || self.id.contains("xor") {
            "Study cryptography basics and cipher techniques".to_string()
        } else if self.id.contains("buffer") || self.id.contains("format") {
            "Learn about binary exploitation and memory safety".to_string()
        } else if self.id.contains("file") || self.id.contains("port") {
            "Understand system fundamentals and reconnaissance".to_string()
        } else {
            "Research the specific technique mentioned in the challenge".to_string()
        }
    }
}

// Cached challenge list for performance
static CHALLENGE_CACHE: OnceLock<Vec<Challenge>> = OnceLock::new();

pub fn get_all_challenges() -> &'static Vec<Challenge> {
    CHALLENGE_CACHE.get_or_init(|| {
        let base_challenges = create_all_challenges();
        add_challenge_variants(base_challenges)
    })
}

/// Create all challenges - called only once and cached
fn create_all_challenges() -> Vec<Challenge> {
    vec![
        // Level 0: Beginner challenges
        Challenge::new_legacy(
            "welcome",
            "The First Message",
            r#"The screen flickers and displays a corrupted message:

    "V2VsY29tZSB0byB0aGUgR2hvc3QgUHJvdG9jb2w="

This looks like Base64 encoding. Decode it to proceed.
(Hint: You can use online tools or the command 'base64 -d' on Linux)"#,
            0,
            50,
            5,
            |answer| answer.to_lowercase() == "welcome to the ghost protocol",
            vec![
                "Base64 is a common encoding scheme. Try an online Base64 decoder.".to_string(),
                "The answer is the decoded text exactly as it appears.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "file_discovery",
            "Hidden Files",
            r#"You find a note with cryptic instructions:

    "The password is hidden in a file that doesn't want to be seen.
     On Unix systems, these files start with a special character.
     The file is named '.secret_key' and contains: ghost_admin_2024"

What is the password?"#,
            0,
            50,
            5,
            |answer| answer == "ghost_admin_2024",
            vec![
                "Files starting with '.' are hidden on Unix systems.".to_string(),
                "The password is: ghost_admin_2024".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "port_scan",
            "The Open Door",
            r#"A network scan reveals open ports on a mysterious server:

    PORT     STATE    SERVICE
    22/tcp   open     ssh
    80/tcp   open     http
    443/tcp  open     https
    3306/tcp open     mysql
    6666/tcp open     unknown

One port stands out as unusual. A message appears:
"The devil's port holds the key: EVIL"

What is the unusual port number?"#,
            0,
            50,
            5,
            |answer| answer == "6666",
            vec![
                "Look for the port that doesn't match common services.".to_string(),
                "Port 6666 is often called the 'devil's port' - 666 times 10.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "rot13_ghost",
            "Rotational Spirits",
            r#"A spectral message rotates before your eyes:

    "Gur nafjre vf: EBGNGVBA"

This appears to be ROT13 encoding - a simple letter substitution where
A↔N, B↔O, C↔P, etc. Each letter is rotated 13 positions in the alphabet.

Decode the message and enter the revealed answer."#,
            0,
            50,
            5,
            |answer| answer.to_uppercase() == "ROTATION",
            vec![
                "ROT13 shifts each letter by 13 positions (A→N, B→O, etc.).".to_string(),
                "ROT13 is its own inverse - applying it twice gives the original.".to_string(),
                "The answer is: ROTATION".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "binary_basics",
            "The Binary Whisper",
            r#"A ghost communicates in the language of machines:

    01000111 01001000 01001111 01010011 01010100

Each group of 8 bits represents one ASCII character.
Decode this binary message to reveal the answer."#,
            0,
            50,
            5,
            |answer| answer.to_uppercase() == "GHOST",
            vec![
                "Binary to text: each 8-bit byte is one ASCII character.".to_string(),
                "You can use online binary-to-text converters.".to_string(),
                "The answer is: GHOST".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "url_decode",
            "Encoded Pathway",
            r#"You intercept a URL being transmitted:

    https://ghost.corp/login?redirect=%2Fadmin%2Fsecrets%3Fkey%3DUNLOCK

The URL parameters are encoded. The 'key' parameter contains the answer.
URL encoding uses %XX where XX is the hexadecimal ASCII value.

What is the decoded value of the 'key' parameter?"#,
            0,
            50,
            5,
            |answer| answer.to_uppercase() == "UNLOCK",
            vec![
                "URL encoding: %2F = /, %3F = ?, %3D = =".to_string(),
                "The key parameter is after 'key%3D' which means 'key='".to_string(),
                "The answer is: UNLOCK".to_string(),
            ],
        ),
        // Level 1: Intermediate challenges
        Challenge::new_legacy(
            "caesar_cipher",
            "Whispers in Code",
            r#"An ancient terminal displays a shifted message:

    "JSHZW SURWRFRO FRPSOHWH. WKH DQVZHU LV: FUBSWRJUDSKB"

This appears to be a Caesar cipher with a shift of 3.
Decode the message and enter the answer it reveals."#,
            1,
            75,
            8,
            |answer| answer.to_lowercase() == "cryptography",
            vec![
                "Caesar cipher shifts each letter by a fixed number.".to_string(),
                "Try shifting each letter back by 3 positions (C→Z, D→A, etc.).".to_string(),
                "The answer is: CRYPTOGRAPHY".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "sql_injection_basics",
            "Database Breach",
            r#"You discover a vulnerable login form. The SQL query used is:

    SELECT * FROM users WHERE username='[INPUT]' AND password='[PASS]'

To bypass authentication, you need to make the query always return true.
What SQL injection payload would you use for the username field?
(Answer with the classic SQL injection that comments out the password check)"#,
            1,
            75,
            10,
            |answer| {
                let normalized = answer.to_lowercase().replace(" ", "");
                normalized.contains("'or'1'='1'--")
                    || normalized.contains("'or1=1--")
                    || normalized == "admin'--"
            },
            vec![
                "Think about how to make the WHERE clause always true.".to_string(),
                "Use OR logic and comment out the rest with --".to_string(),
                "Try: ' OR '1'='1' --".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "hex_decode",
            "Hexadecimal Horror",
            r#"A ghostly message appears in hexadecimal:

    48 45 58 41 44 45 43 49 4D 41 4C

Convert this hex to ASCII to reveal the answer."#,
            1,
            75,
            8,
            |answer| answer.to_uppercase() == "HEXADECIMAL",
            vec![
                "Each pair of hex digits represents one ASCII character.".to_string(),
                "You can use online hex-to-ASCII converters.".to_string(),
                "The answer is: HEXADECIMAL".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "jwt_token",
            "Token of Trust",
            r#"You intercept a JWT (JSON Web Token) used for authentication:

    eyJhbGciOiJub25lIn0.eyJ1c2VyIjoiZ3Vlc3QiLCJhZG1pbiI6ZmFsc2V9.

The header shows "alg":"none" - the algorithm is set to 'none'!
This is a critical vulnerability. JWTs have three parts separated by dots:
    HEADER.PAYLOAD.SIGNATURE

The payload (middle part) is Base64-encoded JSON. Decode it to see:
    {"user":"guest","admin":false}

What security vulnerability allows you to modify the token without a signature?
(Answer: the name of this attack, two words)"#,
            1,
            75,
            10,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "algorithm confusion" || a == "none algorithm" || a == "alg none"
            },
            vec![
                "The 'alg' field is set to 'none', bypassing signature verification.".to_string(),
                "This vulnerability is called 'Algorithm Confusion' or 'None Algorithm'."
                    .to_string(),
                "Answer: algorithm confusion (or 'none algorithm')".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "path_traversal",
            "Directory Descent",
            r#"A web application shows files from a directory:

    https://ghost.corp/files?file=report.pdf
    https://ghost.corp/files?file=data.txt

You suspect the 'file' parameter is vulnerable. By using special characters,
you could access files outside the intended directory.

What is the common character sequence used in path traversal attacks?
(Answer: the characters used to go up one directory level)"#,
            1,
            75,
            8,
            |answer| {
                let a = answer.replace(" ", "");
                a == "../" || a == ".." || a == "..\\"
            },
            vec![
                "Path traversal attacks navigate up directory levels.".to_string(),
                "Use '../' to move up one directory (or '..\\ on Windows').".to_string(),
                "The answer is: ../".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "md5_collision",
            "Hash Breakdown",
            r#"You discover an old authentication system using MD5 hashing:

    Password hash: 5f4dcc3b5aa765d61d8327deb882cf99

The system compares MD5 hashes of passwords. You know MD5 is cryptographically
broken and can be cracked easily with rainbow tables or online databases.

This specific hash is extremely common. What password does it represent?"#,
            1,
            75,
            10,
            |answer| answer.to_lowercase() == "password",
            vec![
                "This is one of the most common password hashes.".to_string(),
                "Try searching for this MD5 hash in an online database.".to_string(),
                "The answer is: password".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "command_injection",
            "Shell Escape",
            r#"A web app accepts user input to ping hosts:

    Command: ping -c 1 [USER_INPUT]

The app doesn't sanitize input properly. You could inject additional commands
using shell metacharacters like ; & | to execute arbitrary commands.

What single character allows you to chain multiple commands in a shell?
(Answer: the semicolon or one of the pipe/ampersand operators)"#,
            1,
            75,
            10,
            |answer| {
                let a = answer.trim();
                a == ";" || a == "semicolon" || a == "&" || a == "|" || a == "&&" || a == "||"
            },
            vec![
                "Shell metacharacters allow command chaining.".to_string(),
                "The semicolon (;) allows you to run multiple commands.".to_string(),
                "Common answers: ; or & or |".to_string(),
            ],
        ),
        // Level 2: Mobile/Web challenges
        Challenge::new_legacy(
            "http_header",
            "Hidden Headers",
            r#"A web request returns suspicious headers:

    HTTP/1.1 200 OK
    Content-Type: text/html
    X-Ghost-Token: R0hPU1RfVE9LRU4=
    Server: nginx

The X-Ghost-Token looks encoded. Decode it and enter the result."#,
            2,
            100,
            10,
            |answer| answer.to_uppercase() == "GHOST_TOKEN",
            vec![
                "The token looks like Base64 encoding.".to_string(),
                "Decode the Base64 string to get the answer.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "mobile_deeplink",
            "Deep Link Discovery",
            r#"A mobile app uses deep links for navigation:

    myapp://profile/user/admin
    myapp://settings/debug/false
    myapp://secret/unlock/MOBILEHACK

What parameter value unlocks the secret feature?"#,
            2,
            100,
            10,
            |answer| answer.to_uppercase() == "MOBILEHACK",
            vec![
                "Look at the URL structure carefully.".to_string(),
                "The answer is in the last deep link path.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "dns_tunneling",
            "Subdomain Secrets",
            r#"Network monitoring reveals suspicious DNS queries:

    6461746131.ghost-protocol.com
    6461746132.ghost-protocol.com
    6461746133.ghost-protocol.com

The subdomains look like encoded data. Each subdomain is hexadecimal ASCII.
Decode the first subdomain (6461746131) to discover what's being tunneled.

What word do these hex bytes spell out?"#,
            2,
            100,
            12,
            |answer| answer.to_lowercase() == "data1" || answer.to_lowercase() == "data",
            vec![
                "DNS tunneling hides data in subdomain names.".to_string(),
                "Convert the hex subdomain to ASCII characters.".to_string(),
                "6461746131 in hex = 'data1' in ASCII".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "xss_attack",
            "Script Injection",
            r#"A vulnerable web form doesn't sanitize user input:

    <div class="comment">
        [USER_INPUT_HERE]
    </div>

An attacker could inject JavaScript that executes in other users' browsers.

What HTML tag is commonly used for Cross-Site Scripting (XSS) attacks?
(Answer: just the tag name without < or >, e.g., 'div')"#,
            2,
            100,
            10,
            |answer| {
                let a = answer.to_lowercase();
                a == "script" || a == "<script>" || a == "script>"
            },
            vec![
                "XSS attacks inject code that runs in the browser.".to_string(),
                "The most common tag for XSS is used to include JavaScript.".to_string(),
                "The answer is: script".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "api_key_leak",
            "Exposed Secrets",
            r#"You find a public GitHub repository with a commit history:

    commit 1: "Add API key for testing"
    commit 2: "Remove API key" (deleted from current code)

The developer removed the API key in commit 2, but it's still in commit 1's history!
Anyone can view the Git history to find it. The leaked key format is:

    GHOST_API_KEY_2024_DEMO

What type of security issue is this called?
(Answer: two words, the practice of accidentally committing secrets)"#,
            2,
            100,
            12,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a.contains("secret") && a.contains("leak")
                    || a == "credential leak"
                    || a == "secrets leak"
                    || a == "key leak"
                    || a == "secret exposure"
            },
            vec![
                "Secrets accidentally committed to Git repos are a major security risk."
                    .to_string(),
                "Even if deleted, they remain in Git history forever.".to_string(),
                "This is called a 'secret leak' or 'credential leak'.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "session_hijack",
            "Cookie Monster",
            r#"You intercept HTTP traffic and find a session cookie:

    Set-Cookie: sessionid=abc123def456; Path=/

The cookie has no HttpOnly or Secure flags set. This means:
- JavaScript can access it (vulnerable to XSS)
- It can be sent over HTTP (vulnerable to interception)

What is the attack called when someone steals and uses your session cookie?
(Answer: two words, _____ hijacking)"#,
            2,
            100,
            12,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "session hijacking" || a == "session hijack" || a == "cookie hijacking"
            },
            vec![
                "This attack steals active user sessions.".to_string(),
                "An attacker uses your cookie to impersonate you.".to_string(),
                "The answer is: session hijacking".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "cors_bypass",
            "Origin Stories",
            r#"A web API has incorrect CORS (Cross-Origin Resource Sharing) headers:

    Access-Control-Allow-Origin: *
    Access-Control-Allow-Credentials: true

The wildcard (*) allows ANY website to make requests with credentials.
This is a critical misconfiguration!

What does CORS stand for?
(Answer: four words, or just 'CORS')"#,
            2,
            100,
            10,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "cors"
                    || a == "cross origin resource sharing"
                    || a.contains("cross") && a.contains("origin") && a.contains("resource")
            },
            vec![
                "CORS is a browser security feature.".to_string(),
                "It controls cross-domain requests.".to_string(),
                "CORS = Cross-Origin Resource Sharing".to_string(),
            ],
        ),
        // Level 2: OSINT (Open Source Intelligence) challenges
        Challenge::new_legacy(
            "osint_social_media",
            "Digital Footprints",
            r#"You're investigating a target who uses the handle "GhostHacker2024" online.
They posted a photo on social media with the caption: "At my favorite coffee shop again!"

The image metadata reveals these EXIF details:
- Camera: iPhone 12 Pro
- GPS Coordinates: 40.7128° N, 74.0060° W
- Timestamp: 2024-01-15 14:30:22

Based on the GPS coordinates, what major city is this person located in?
(Answer: City name only)"#,
            2,
            100,
            12,
            |answer| {
                let a = answer.to_lowercase();
                a == "new york" || a == "new york city" || a == "nyc" || a == "manhattan"
            },
            vec![
                "GPS coordinates can reveal exact locations from photos.".to_string(),
                "Those coordinates are for a very famous US city.".to_string(),
                "40.7128° N, 74.0060° W = New York City coordinates".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_domain_recon",
            "Domain Investigation",
            r#"During reconnaissance of "ghost-corp.example", you gather OSINT data:

WHOIS Record:
- Registrar: GoDaddy
- Created: 2020-05-15
- Expires: 2025-05-15
- Email: admin@ghost-corp.example
- Name Server: ns1.cloudflare.com

DNS Records:
- A Record: 192.168.1.100
- MX Record: mail.ghost-corp.example (Priority 10)
- TXT Record: "v=spf1 include:_spf.google.com ~all"

What email service provider do they use based on the SPF record?
(Answer: Company name)"#,
            2,
            100,
            10,
            |answer| {
                let a = answer.to_lowercase();
                a == "google" || a == "gmail" || a == "g suite" || a == "google workspace"
            },
            vec![
                "SPF records reveal authorized email servers.".to_string(),
                "Look at the 'include:' domain in the TXT record.".to_string(),
                "The SPF record includes Google's servers.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_email_analysis",
            "Electronic Trail",
            r#"You receive a suspicious email during investigation:

From: security@gh0st-bank.com
To: victim@company.com
Subject: Urgent: Verify Your Account
Date: Mon, 15 Jan 2024 10:30:00 +0000
Message-ID: <ABC123@gh0st-bank.com>
X-Originating-IP: 203.0.113.45
Return-Path: bounce@gh0st-bank.com

Several red flags suggest this is a phishing attempt:
1. Suspicious domain (gh0st-bank vs ghost-bank)
2. Urgent language creating pressure
3. Originating IP from a different country

What is the technique called when attackers use domains that look similar
to legitimate ones? (Answer: One word, starts with 'typo')"#,
            2,
            100,
            12,
            |answer| {
                let a = answer.to_lowercase();
                a == "typosquatting" || a == "typosquat" || a == "cybersquatting"
            },
            vec![
                "This attack uses domains with small spelling differences.".to_string(),
                "Attackers register domains that look like legitimate ones.".to_string(),
                "The answer is: typosquatting".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_geolocation",
            "Location Triangulation",
            r#"You're tracking a suspect who posted this message:

"Just grabbed lunch at that pizza place across from the big clock tower.
 Can see the river from here, and there's construction on the main bridge.
 The weather app says it's 23°C - perfect for a walk in the financial district!"

Additional clues from their profile:
- Posts often mention "the tube" (subway system)
- Uses British spelling: "colour", "realise"
- Recent check-in at "Borough Market"
- Time zone: GMT+0

Which major European city fits these location clues?
(Answer: City name)"#,
            2,
            100,
            15,
            |answer| {
                let a = answer.to_lowercase();
                a == "london" || a == "london uk" || a == "london england"
            },
            vec![
                "Look for clues about the country and famous landmarks.".to_string(),
                "'The tube', British spelling, and GMT+0 suggest which country?".to_string(),
                "Big Ben clock tower, Thames river, Borough Market = London".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_breach_investigation",
            "Data Breach Analysis",
            r#"You're investigating a data breach. Analysis reveals:

Breach Details:
- 50,000 user accounts compromised
- Password hashes: MD5 (unsalted)
- Stolen data: emails, usernames, phone numbers
- Attack vector: SQL injection on login form
- Timeline: Data exfiltrated over 3 months

The breach notification states: "We use industry-standard encryption"
But investigation shows they used MD5 for passwords with no salt.

What critical security practice was missing from their password storage?
(Answer: One word that makes rainbow table attacks much harder)"#,
            2,
            100,
            12,
            |answer| {
                let a = answer.to_lowercase();
                a == "salt" || a == "salting" || a == "hashing salt" || a == "password salt"
            },
            vec![
                "This makes each password hash unique even for identical passwords.".to_string(),
                "Prevents rainbow table attacks by adding random data.".to_string(),
                "The answer is: salt (or salting)".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_username_recon",
            "Digital Identity Hunt",
            r#"You're tracking a person of interest using username enumeration.
Their primary username "Gh0stRunner42" appears on multiple platforms:

Platform Analysis:
- GitHub: gh0strunner42 (joined 2019, 847 contributions)
- Twitter: @GhostRunner_42 (since 2018, 2.3K followers)
- Reddit: u/Gh0stRunner42 (4 year account, 15K karma)
- LinkedIn: Different name but email visible: gr42@protonmail.com

Cross-referencing the email prefix "gr42" with their other usernames
reveals a pattern. What technique are you using to link these accounts?
(Answer: Two words, first word is 'username')"#,
            2,
            110,
            12,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "username enumeration"
                    || a == "username recon"
                    || a == "username reconnaissance"
            },
            vec![
                "This OSINT technique involves searching for the same username across platforms."
                    .to_string(),
                "Also called account correlation or identity aggregation.".to_string(),
                "The answer is: username enumeration".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_wayback_machine",
            "Echoes from the Past",
            r#"A suspicious company suddenly changed their website after a scandal.
The current site at "secure-crypto-vault.com" shows:

Current Version (2024):
"We have always prioritized security and never had any breaches."

But using the Internet Archive's Wayback Machine, you find a cached
version from 2023:

Archived Version (2023):
"UPDATE: We experienced a security incident on May 15, 2023.
 Approximately 10,000 user accounts were affected."

This directly contradicts their current claims. What is this web archive
service commonly called? (Answer: Two words, commonly abbreviated as WBM)"#,
            2,
            110,
            10,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ");
                a == "wayback machine"
                    || a == "internet archive"
                    || a == "web archive"
                    || a == "archive.org"
            },
            vec![
                "This service archives historical versions of websites.".to_string(),
                "archive.org maintains this massive collection of web pages.".to_string(),
                "The answer is: Wayback Machine (or Internet Archive)".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_metadata_extraction",
            "Hidden Data Trails",
            r#"A document leaked online contains sensitive information in its metadata.

File: confidential_report.pdf
Visible Content: [REDACTED]
File Size: 2.4 MB

But examining the PDF metadata reveals:
- Author: John.Smith@ghost-corp.internal
- Creation Date: 2024-01-20 03:47:15
- Application: Microsoft Office Word 2019
- Company: Ghost Corporation Internal Affairs
- Last Modified By: j.smith
- Comments: "DRAFT - DO NOT DISTRIBUTE OUTSIDE DEPT"

What is the embedded information in files called that can reveal
details about when, where, and by whom it was created?
(Answer: One word, often abbreviated as EXIF for images)"#,
            2,
            110,
            12,
            |answer| {
                let a = answer.to_lowercase();
                a == "metadata"
                    || a == "exif"
                    || a == "exif data"
                    || a == "meta data"
                    || a == "file metadata"
            },
            vec![
                "This hidden information is embedded in files alongside the visible content."
                    .to_string(),
                "For images, it's called EXIF data. For documents, it's similar.".to_string(),
                "The answer is: metadata".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_shodan_recon",
            "The Internet Scanner",
            r#"You're using a specialized search engine for internet-connected devices.
Unlike Google, this tool indexes:

- Open ports and services
- Webcams and security cameras
- Industrial control systems (ICS/SCADA)
- IoT devices with default credentials
- Vulnerable servers and databases
- Banner information from services

Search Query Example: "port:3389 country:US"
Results: 2.4 million exposed RDP (Remote Desktop) servers in USA

This powerful OSINT tool has been called "the scariest search engine"
because it reveals insecure devices worldwide.

What is this internet-connected device search engine called?
(Answer: One word, rhymes with "rodan")"#,
            2,
            110,
            15,
            |answer| {
                let a = answer.to_lowercase();
                a == "shodan" || a == "shodan.io"
            },
            vec![
                "This search engine was created by John Matherly in 2009.".to_string(),
                "It's named after a character from the System Shock video game.".to_string(),
                "The answer is: Shodan".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "osint_pastebin_leak",
            "Data Dump Discovery",
            r#"During a security investigation, you find a suspicious paste on a public
text-sharing website. The paste titled "DB_BACKUP_2024" contains:

-- Ghost Corp Database Dump --
admin@ghost.com:Password123!hash=5f4dcc3b5aa765d61d8327deb882cf99
user@ghost.com:Welcome2024!hash=e10adc3949ba59abbe56e057f20f883e
root@ghost.com:GhostAdmin99!hash=25d55ad283aa400af464c76d713c07ad

[... 5,000 more lines ...]

Posted by: Anonymous
Date: 2024-01-10
Views: 47,239

Attackers often dump stolen credentials on text-sharing sites like:
- Pastebin
- Ghostbin
- Hastebin
- GitHub Gists (if not removed quickly)

What are these public credential dumps commonly called in cybersecurity?
(Answer: One or two words, rhymes with "taste")"#,
            2,
            110,
            12,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "paste"
                    || a == "pastes"
                    || a == "paste dump"
                    || a == "paste site"
                    || a == "pastebin dump"
                    || a == "data dump"
                    || a == "credential dump"
            },
            vec![
                "These are publicly shared text files containing stolen data.".to_string(),
                "The most famous site for this is Pastebin.com".to_string(),
                "The answer is: paste (or paste dump)".to_string(),
            ],
        ),
        // Level 2: Steganography challenges (v1.5.0)
        Challenge::new_legacy(
            "steg_lsb_basics",
            "The Hidden Pixel",
            r#"You discover a seemingly innocent profile picture from a suspected hacker's
social media account. The image metadata shows an unusual file size for its
resolution - 847KB for a 400x400 pixel PNG that should be around 200KB.

Running a steganography analysis tool reveals binary data hidden in the least
significant bits (LSB) of the pixel RGB values. The LSB extraction shows:

01001000 01101001 01100100 01100100 01100101 01101110
01001101 01100101 01110011 01110011 01100001 01100111 01100101

LSB steganography works by replacing the least significant bit of each pixel
color channel with message bits. Since changing the LSB only shifts color
values by ±1, the changes are invisible to the human eye.

Tools for LSB steganography:
- steghide (command-line tool)
- stegsolve (GUI analyzer)
- zsteg (Ruby tool for PNG/BMP)
- Online LSB decoders

What does the hidden binary message decode to when converted from binary to ASCII?"#,
            2,
            110,
            10,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "hiddenmessage"
                    || a == "hidden message"
                    || a.contains("hidden") && a.contains("message")
            },
            vec![
                "Convert each 8-bit binary sequence to its ASCII character.".to_string(),
                "01001000 = H, 01101001 = i, 01100100 = d, etc.".to_string(),
                "The hidden message is: HiddenMessage".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "steg_exif_data",
            "Metadata Secrets",
            r#"A leaked photo from a whistleblower contains more than meets the eye.
While the image shows an empty conference room, the EXIF metadata reveals:

Camera: Canon EOS 5D Mark IV
Date: 2024-09-15 14:32:07
GPS Latitude: 37.7749° N
GPS Longitude: 122.4194° W
Artist: J.Anderson
Copyright: Ghost Corporation Internal
Software: Adobe Photoshop CC 2024
Comment: Meeting-Room-B-Project-Blackout-Q4

EXIF (Exchangeable Image File Format) data stores metadata in image files:
- Camera settings and model
- Date and time photo was taken
- GPS coordinates (if location services enabled)
- Photographer information
- Software used for editing
- User comments and copyright info

This metadata can reveal:
- Who took the photo (Artist field)
- Where it was taken (GPS coordinates)
- When it was taken (timestamp)
- What software edited it
- Custom comments or notes

Privacy risk: Many people don't realize photos shared online contain this data.

Tools to view EXIF data:
- exiftool (command-line)
- Online EXIF viewers
- Windows/Mac file properties
- Metadata removal tools

Based on the Comment field, what is the project code name?"#,
            2,
            110,
            8,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "blackout" || a == "project blackout" || a.contains("blackout")
            },
            vec![
                "Look carefully at the Comment field in the EXIF data.".to_string(),
                "The comment mentions a project name after 'Project-'.".to_string(),
                "The project code name is: Blackout".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "steg_audio_spectrum",
            "The Sound of Silence",
            r#"You intercept an audio file that sounds like pure static noise. No audible
message can be heard even when amplified. However, a fellow analyst suggests
viewing it as a spectrogram.

A spectrogram displays audio as a visual image where:
- X-axis = time
- Y-axis = frequency
- Color/brightness = amplitude

Loading the file in Audacity and viewing the spectrogram reveals text encoded
in the frequency domain. The pattern shows clear letter shapes formed by
concentrated frequency bands at specific times:

Between 2-8 kHz, the spectrogram displays: "GHOSTPROTOCOL"

This technique is called spectrogram steganography or visual audio encoding.
It's used because:
- Invisible to casual listeners
- Survives audio compression (sometimes)
- Requires visual analysis to detect
- Can hide images or text in sound

Famous examples:
- Aphex Twin's "Windowlicker" has a face in the spectrogram
- Nine Inch Nails hid images in album tracks
- Various ARGs (Alternate Reality Games) use this technique

Tools for spectrogram analysis:
- Audacity (free, cross-platform)
- Sonic Visualizer
- Spek (spectrogram analyzer)
- Adobe Audition

What message is hidden in the audio spectrogram?"#,
            2,
            120,
            12,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "ghostprotocol"
                    || a == "ghost protocol"
                    || a.contains("ghost") && a.contains("protocol")
            },
            vec![
                "The message is formed by frequency patterns visible in the spectrogram."
                    .to_string(),
                "Look at the text displayed between 2-8 kHz frequency range.".to_string(),
                "The hidden message is: GHOSTPROTOCOL".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "steg_whitespace",
            "Invisible Ink",
            r#"You receive a text file containing what appears to be normal Python code:

```python
def process_data(input):
	    result = []
    	for item in input:
	        if item > 0:
    	        result.append(item * 2)
	    return result
```

The code runs fine, but something feels off. Examining the whitespace reveals
a pattern of spaces and tabs that doesn't match normal indentation:

Line 2: TAB SPACE SPACE SPACE SPACE
Line 3: SPACE SPACE SPACE SPACE TAB
Line 4: TAB SPACE SPACE SPACE SPACE SPACE SPACE SPACE
Line 5: SPACE SPACE SPACE SPACE TAB SPACE SPACE SPACE

Using whitespace steganography encoding where TAB = 1 and SPACE = 0:
10000 = P (ASCII 80 in binary: 01010000, using 5-bit encoding)
00001 = A (ASCII 65 in binary: 01000001, using last 5 bits)
10000 = P
00010 = S
10010 = S

When decoded using TAB=1, SPACE=0, the hidden message spells a common word.

Whitespace steganography:
- Hides data in spaces, tabs, newlines
- Zero-width characters in Unicode
- Invisible to casual observation
- Survives copy-paste operations
- Hard to detect without special tools

Detection methods:
- Convert whitespace to visible characters
- Look for irregular spacing patterns
- Check for mixing of tabs and spaces
- Use steganography detection tools

Tools:
- snow (Steganographic Nature Of Whitespace)
- stegsnow
- Custom scripts to analyze whitespace

What five-letter word is hidden in the whitespace pattern?"#,
            2,
            115,
            10,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "pass" || a == "password" || a.contains("pass")
            },
            vec![
                "Decode each line using TAB=1 and SPACE=0 to get 5-bit sequences.".to_string(),
                "The pattern forms letters: P-A-S-S using 5-bit encoding.".to_string(),
                "The hidden word is: PASS (or PASSWORD)".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "steg_file_concat",
            "The Polyglot File",
            r#"You download what appears to be a normal PNG image file (logo.png, 45KB).
However, the file size seems suspicious for a simple logo. Opening it in an
image viewer works fine, but examining the raw bytes reveals something unusual.

Using a hex editor, you find the PNG end marker at byte offset 28,672:
00006F70: 49 45 4E 44 AE 42 60 82  IEND®B`‚

But the file continues for another 18KB! After the PNG IEND chunk, you find:
00006F78: 50 4B 03 04 14 00 00 00  PK......

50 4B (ASCII "PK") is the magic number for ZIP files!

This is a polyglot file - a file that is valid in multiple formats simultaneously.
The PNG is complete and displays correctly, but a ZIP archive is appended after
the PNG end marker.

Extracting the hidden ZIP archive:
```bash
# Method 1: Extract using byte offset
dd if=logo.png of=hidden.zip bs=1 skip=28672

# Method 2: Use binwalk to find and extract
binwalk -e logo.png

# Method 3: Try opening with ZIP tools directly
unzip logo.png
```

Inside the hidden ZIP archive is a file named "credentials.txt" containing:
Username: ghost_admin
Password: SecretPass2024

File format magic numbers:
- PNG: 89 50 4E 47 (‰PNG)
- JPEG: FF D8 FF
- ZIP: 50 4B 03 04 (PK)
- PDF: 25 50 44 46 (%PDF)
- RAR: 52 61 72 21 (Rar!)

This technique is useful for:
- Hiding data in plain sight
- Bypassing simple file type filters
- Steganographic dead drops
- CTF challenges

What is the password found in the hidden credentials file?"#,
            2,
            120,
            12,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "secretpass2024"
                    || a == "secret pass 2024"
                    || a.contains("secret") && a.contains("pass")
                    || a.contains("secretpass")
            },
            vec![
                "The hidden ZIP archive contains a credentials.txt file.".to_string(),
                "Look for the password field in the credentials file.".to_string(),
                "The password is: SecretPass2024".to_string(),
            ],
        ),
        // Level 3+: Advanced challenges
        Challenge::new_legacy(
            "binary_exploit",
            "Buffer Overflow Ghost",
            r#"You find a vulnerable C program:

    char buffer[8];
    strcpy(buffer, user_input);

The buffer can only hold 8 characters, but there's no bounds checking.
If the password stored after the buffer in memory is "OVERFLOW", and you
need to overwrite it, what's a classic buffer overflow attack called?

(Answer: the type of vulnerability, one word)"#,
            3,
            125,
            15,
            |answer| {
                let a = answer.to_uppercase();
                a == "BUFFER OVERFLOW" || a == "BUFFEROVERFLOW" || a == "OVERFLOW"
            },
            vec![
                "This is a classic memory corruption vulnerability.".to_string(),
                "The vulnerability is called a 'buffer overflow'.".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "reverse_engineering",
            "Decompiled Secrets",
            r#"You decompile a binary and find this pseudocode:

    if (input XOR 0x42 == 0x2D) {
        grant_access();
    }

What single character input would grant access?
(Hint: XOR is reversible, and 0x42 = 'B' in ASCII)"#,
            3,
            125,
            15,
            |answer| answer.starts_with('o') || answer == "111" || answer == "0x6F",
            vec![
                "XOR is reversible: A XOR B = C means A XOR C = B".to_string(),
                "Calculate: 0x2D XOR 0x42 = ?".to_string(),
                "The answer is 'o' (ASCII 111 or 0x6F)".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "format_string",
            "String Exploitation",
            r#"You analyze a vulnerable C program:

    printf(user_input);  // DANGEROUS!

Instead of: printf("%s", user_input);

The program directly passes user input to printf without a format specifier.
An attacker can use format specifiers like %x, %s, or %n to read/write memory.

What is this classic vulnerability called?
(Answer: two or three words)"#,
            3,
            125,
            15,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a.contains("format string") || a == "format string vulnerability"
            },
            vec![
                "This vulnerability involves printf and format specifiers.".to_string(),
                "User-controlled format strings can read/write arbitrary memory.".to_string(),
                "The answer is: format string vulnerability".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "race_condition",
            "Time Paradox",
            r#"A program checks permissions then opens a file:

    1. if (can_access(file)) {
    2.     // Small time window here!
    3.     open_file(file);
    4. }

Between steps 1 and 3, an attacker could swap the file with a symbolic link
to a privileged file. The check passes, but a different file is opened!

What is this time-based vulnerability called?
(Answer: two words, _____ condition)"#,
            3,
            125,
            15,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "race condition" || a == "toctou" || a == "time of check time of use"
            },
            vec![
                "This exploits the time gap between operations.".to_string(),
                "Also known as TOCTOU (Time Of Check, Time Of Use).".to_string(),
                "The answer is: race condition".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "integer_overflow",
            "Number Nightmare",
            r#"A C program allocates memory based on user input:

    unsigned char size = user_input;  // Max value: 255
    char* buffer = malloc(size + 10);

If user_input = 250, then size + 10 = 260... but wait!
An unsigned char can only hold 0-255, so 260 wraps around to 4!
This allocates only 4 bytes instead of 260.

What is this vulnerability called?
(Answer: two words, _____ overflow)"#,
            3,
            125,
            15,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "integer overflow" || a == "integer wraparound" || a == "arithmetic overflow"
            },
            vec![
                "This happens when numbers exceed their maximum value.".to_string(),
                "Values 'wrap around' back to zero.".to_string(),
                "The answer is: integer overflow".to_string(),
            ],
        ),
        // Level 3: Advanced Steganography (v1.5.0)
        Challenge::new_legacy(
            "steg_dna_encoding",
            "Genetic Message",
            r#"A genetics research paper contains an unusual DNA sequence that doesn't
match any known biological patterns. The sequence appears to be synthetic:

ATCGATCGTAGCTAGCATCGATCGTAGCTAGCATCGATCGTAGC
TAGCATCGATCGTAGCTAGCATCGATCGTAGCTAGCATCGATCG

DNA can be used as a data storage medium using 4 nucleotides (A, C, G, T).
One encoding scheme uses: A=00, C=01, G=10, T=11

Converting the sequence to binary:
AT = 00 11 = 0011 = 3
CG = 01 10 = 0110 = 6
AT = 00 11 = 0011 = 3
CG = 01 10 = 0110 = 6

But there's a simpler pattern! Looking at pairs:
AT, CG, AT, CG, TA, GC, TA, GC, AT, CG, AT, CG...

The researcher notes: "Pairs alternate in a pattern that maps to ASCII."
When decoded properly, the first 8 bases spell "DATA" in 4-bit encoding:

D = 44 hex = 01000100 = ATCG (using A=01, T=00, C=11, G=10)
A = 41 hex = 01000001 = ATCG
T = 54 hex = 01010100 = ATTA
A = 41 hex = 01000001 = ATCG

But the actual message using proper 2-bit encoding (A=00, T=01, C=10, G=11):
The sequence spells out a familiar tech term when decoded.

DNA data storage facts:
- 1 gram of DNA can store 215 petabytes
- Harvard scientists encoded a book into DNA in 2012
- Microsoft and University of Washington have DNA storage projects
- DNA is stable for thousands of years

What four-letter word is encoded in the DNA sequence?"#,
            3,
            150,
            15,
            |answer| {
                let a = answer.to_lowercase();
                a == "data" || a == "code" || a == "gene" || a.contains("data")
            },
            vec![
                "Use the encoding: A=00, T=01, C=10, G=11 to convert pairs to binary.".to_string(),
                "Group the binary into 8-bit bytes and convert to ASCII characters.".to_string(),
                "The hidden message is: DATA".to_string(),
            ],
        ),
        // Level 3: Malware Analysis (v1.5.0)
        Challenge::new_legacy(
            "malware_obfuscation",
            "Layered Deception",
            r#"You discover a suspicious JavaScript file with heavily obfuscated code:

```javascript
eval(atob('ZXZhbChhdG9iKCdZWFJsWW5Rb1lYUnZZaWduYldGc2QyRnlaU2NwS1E9PScpKQ=='));
```

This is multi-layer obfuscation using Base64 encoding and eval().

Layer 1: atob('ZXZhbChhdG9iKCdZWFJsWW5Rb1lYUnZZaWduYldGc2QyRnlaU2NwS1E9PScpKQ==')
Decodes to: eval(atob('YXRlWm5RaGdYUnZZaWduYldGc2QyRnlaU2NwS1E9PQ=='))

Layer 2: atob('YXRlWm5RaGdYUnZZaWduYldGc2QyRnlaU2NwS1E9PQ==')
Decodes to: atob('bWFsd2FyZQ==')

Layer 3: atob('bWFsd2FyZQ==')
Decodes to: malware

Common obfuscation techniques:
- Base64 encoding (atob/btoa in JS)
- Hex encoding (\x41\x42\x43)
- String reversal
- Character code manipulation
- eval() for dynamic execution
- Multiple layers combined

Malware uses obfuscation to:
- Evade antivirus signatures
- Hide malicious intent
- Bypass code analysis
- Complicate reverse engineering

Tools for deobfuscation:
- CyberChef (for multi-layer encoding)
- js-beautify (for minified code)
- de4js (JavaScript deobfuscator)
- Manual analysis with debugging

What is the final decoded string after removing all obfuscation layers?"#,
            3,
            140,
            15,
            |answer| {
                let a = answer.to_lowercase();
                a == "malware" || a.contains("malware")
            },
            vec![
                "Decode each atob() layer step by step from inner to outer.".to_string(),
                "Each layer uses Base64 encoding (atob = ASCII to Binary).".to_string(),
                "The final decoded string is: malware".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "malware_behavior",
            "Behavioral Signature",
            r#"A security analyst observes suspicious system behavior on a compromised machine:

OBSERVED ACTIVITIES:
1. Process "svchost.exe" spawns from unusual parent: cmd.exe
2. Registry modification: HKLM\Software\Microsoft\Windows\CurrentVersion\Run
3. Outbound connection to IP: 203.0.113.42 on port 443 (non-standard cert)
4. File creation: C:\Users\Public\update.exe
5. Process injection into explorer.exe
6. Scheduled task created: "WindowsUpdate" (runs at startup)
7. Attempts to disable Windows Defender
8. Encrypts files in Documents folder with .ghost extension

These behavioral indicators suggest:
- Persistence mechanism (Registry Run key + Scheduled Task)
- Command and Control communication (C2 server)
- Process hollowing/injection
- Defense evasion (disabling AV)
- Potential ransomware (file encryption)

This is NOT static analysis (examining code) but BEHAVIORAL analysis
(observing what the malware does when executed).

Common behavioral indicators:
- Registry modifications for persistence
- Network connections to known bad IPs
- Process injection/hollowing
- File encryption
- Privilege escalation attempts
- Disabling security tools

Based on the file encryption with ".ghost" extension and ransom behavior,
what category of malware is this most likely?

(Answer: type of malware that encrypts files for ransom, one word)"#,
            3,
            150,
            18,
            |answer| {
                let a = answer.to_lowercase();
                a == "ransomware" || a == "ransom" || a.contains("ransom")
            },
            vec![
                "The malware encrypts files and likely demands payment.".to_string(),
                "The .ghost extension and encryption behavior are classic signs.".to_string(),
                "This is: ransomware".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "malware_sandbox",
            "Sandbox Escape Artist",
            r#"You analyze malware that refuses to execute in your analysis environment.
The malware checks for various sandbox/VM indicators before running:

ANTI-ANALYSIS CHECKS:
```python
# Check 1: VM Detection
vm_indicators = ['VMware', 'VirtualBox', 'QEMU', 'Xen']
for indicator in vm_indicators:
    if indicator in system_info:
        exit()  # Don't run in VM

# Check 2: Common Sandbox Usernames
sandbox_users = ['malware', 'sandbox', 'virus', 'sample']
if username.lower() in sandbox_users:
    exit()

# Check 3: Mouse Movement
if total_mouse_movement < 100:  # Sandboxes often don't simulate user activity
    exit()

# Check 4: Execution Time
if system_uptime < 600:  # Less than 10 minutes
    exit()  # Fresh sandbox

# Check 5: Number of Running Processes
if len(processes) < 30:  # Sandboxes have fewer processes
    exit()
```

This is called sandbox evasion or anti-analysis techniques.

Other evasion methods:
- Sleep/delay before execution (avoid time-limited analysis)
- Check for debugging tools (IDA, OllyDbg, x64dbg)
- Detect VM hardware (specific MAC addresses, registry keys)
- Look for analysis tools (Wireshark, Process Monitor)
- Check for common sandbox file paths
- Verify internet connectivity
- Geographic location checks

Defensive sandboxes try to:
- Simulate realistic user activity
- Hide VM indicators
- Use realistic usernames and file structures
- Provide proper timing simulation

What is this category of techniques called when malware detects analysis environments?

(Answer: two words, _____ evasion)"#,
            3,
            150,
            20,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "sandbox evasion"
                    || a == "sandbox detection"
                    || a == "vm evasion"
                    || a == "anti analysis"
                    || a.contains("sandbox") && a.contains("evasion")
            },
            vec![
                "The malware is trying to detect if it's being analyzed.".to_string(),
                "It checks for virtualization, sandboxes, and analysis tools.".to_string(),
                "These are called: sandbox evasion techniques".to_string(),
            ],
        ),
        // Level 3: IoT Security (v1.5.0)
        Challenge::new_legacy(
            "iot_default_creds",
            "The Unchanged Password",
            r#"During a security audit, you scan a home network and discover a smart
camera with an open web interface on port 80. The login page shows:

    "IP Camera Admin Panel"
    Username: [____________]
    Password: [____________]

Searching online for the device model reveals the manufacturer's documentation:

Model: GhostCam 3000
Default Credentials:
- Username: admin
- Password: admin

WARNING: Please change default credentials after first login!

The homeowner never changed these credentials. You can now:
- Access live camera feed
- Download recorded footage
- Change camera settings
- Add the camera to a botnet

IoT security problems:
- Many devices ship with default credentials
- Users rarely change them
- No forced password change on first use
- Credentials are publicly documented
- Same defaults across all units

Real-world impacts:
- Mirai botnet used default credentials to infect 600,000+ IoT devices
- Used in massive DDoS attacks (Dyn attack, 2016)
- Privacy violations (camera/baby monitor hacks)
- Smart home device compromise

Resources for default credentials:
- DefaultCreds-cheat-sheet (GitHub)
- Manufacturer documentation
- cirt.net default password database
- Router default password lists

What is the default password for the GhostCam 3000?"#,
            3,
            140,
            15,
            |answer| {
                let a = answer.to_lowercase();
                a == "admin" || a == "default" || a.contains("admin")
            },
            vec![
                "Check the manufacturer's documentation shown above.".to_string(),
                "IoT devices often use 'admin' as both username and password.".to_string(),
                "The default password is: admin".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "iot_firmware",
            "Firmware Secrets",
            r#"You extract firmware from a smart home hub and analyze the binary.
Using 'strings' command to find readable text, you discover:

```
# strings firmware.bin | grep -i "api\|key\|password"
API_KEY=sk_live_51HxK8900334GhostAPIKey
ADMIN_PASSWORD=SuperSecret2024
BACKDOOR_USER=ghostadmin
MQTT_PASSWORD=mqtt_ghost_pass_99
WIFI_PSK=hardcoded_wifi_password
AWS_SECRET=AKIAI44QH8DHBEXAMPLE
```

Firmware analysis reveals hardcoded secrets that should NEVER be in firmware:
- API keys (that can't be revoked per-device)
- Admin credentials (same across all devices!)
- Cloud service credentials
- Backdoor accounts
- WiFi passwords (in some cases)

Why this is dangerous:
- Anyone can extract firmware (often available for download)
- Secrets are same across all devices of that model
- Changing them requires firmware update
- Can't revoke compromised keys easily
- Attackers can access backend services

Tools for firmware analysis:
- binwalk (extract filesystems from firmware)
- strings (find readable text)
- Ghidra/IDA Pro (reverse engineering)
- firmware-mod-kit
- firmwalker (scan for secrets)

Real examples:
- Hard-coded root passwords in IP cameras
- AWS keys in smart lock firmware
- API credentials in smart speakers
- Debug interfaces left enabled

What is the hardcoded admin password found in the firmware?"#,
            3,
            140,
            15,
            |answer| {
                let a = answer.to_lowercase().replace("-", " ").replace("_", " ");
                a == "supersecret2024"
                    || a == "super secret 2024"
                    || a.contains("supersecret")
                    || a.contains("super") && a.contains("secret")
            },
            vec![
                "Look for the ADMIN_PASSWORD value in the strings output.".to_string(),
                "It's listed in the firmware secrets above.".to_string(),
                "The admin password is: SuperSecret2024".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "iot_mqtt",
            "Unencrypted Whispers",
            r#"You're monitoring network traffic from smart home devices and intercept
MQTT messages (Message Queuing Telemetry Transport - common IoT protocol):

MQTT Packet #1:
Topic: home/frontdoor/lock
Payload: {"command": "unlock", "user": "ghost", "pin": "1234"}

MQTT Packet #2:
Topic: home/security/alarm
Payload: {"status": "disarmed", "code": "5678"}

MQTT Packet #3:
Topic: home/lights/bedroom
Payload: {"power": "off"}

The protocol is running UNENCRYPTED (no TLS/SSL)!

Problems with unencrypted MQTT:
- Anyone on the network can read messages
- Commands can be intercepted
- Authentication credentials exposed
- Messages can be modified (MITM attacks)
- Device commands can be replayed

MQTT is a publish/subscribe protocol:
- Devices publish messages to topics
- Other devices subscribe to topics
- Broker routes messages between publishers/subscribers
- Very lightweight (designed for IoT)
- Often used without encryption by default

Secure MQTT should use:
- TLS/SSL encryption
- Username/password authentication
- Client certificates
- Access control lists (ACLs)
- Message encryption at application layer

Tools for MQTT:
- mosquitto_sub (subscribe to topics)
- mosquitto_pub (publish messages)
- MQTT Explorer (GUI client)
- Wireshark (analyze MQTT traffic)

What is the door lock PIN code revealed in the unencrypted MQTT traffic?"#,
            3,
            150,
            15,
            |answer| {
                let a = answer.replace("-", "").replace(" ", "").replace("_", "");
                a == "1234" || a.contains("1234")
            },
            vec![
                "Look at the first MQTT packet about the front door lock.".to_string(),
                "The PIN is in the 'pin' field of the JSON payload.".to_string(),
                "The door lock PIN is: 1234".to_string(),
            ],
        ),
        Challenge::new_legacy(
            "final_protocol",
            "The Ghost's True Name",
            r#"You've reached the core of the system. A final riddle appears:

    "I am the protocol that haunts this machine.
     My name is hidden in the challenges you've seen.
     Take the first letter of each challenge's ID,
     And you'll spell out what I'm meant to be."

What is the Ghost Protocol's true name?

(Combine the first letters of all challenge IDs in order)"#,
            4,
            200,
            20,
            |answer| {
                let a = answer.to_uppercase();
                // First letters: W, F, P, C, S, H, H, M, B, R, F
                a.contains("PHANTOM") || a.contains("FREEDOM") || a.contains("PROTOCOL")
            },
            vec![
                "Look back at all the challenge IDs you've completed.".to_string(),
                "Take the FIRST letter from each challenge ID.".to_string(),
                "The pattern spells 'PROTOCOL' or similar...".to_string(),
            ],
        ),
    ]
}

/// Create challenge variants for v1.2.0 (add variants to existing challenges)
fn add_challenge_variants(mut challenges: Vec<Challenge>) -> Vec<Challenge> {
    for challenge in &mut challenges {
        // Add variants based on challenge type
        match challenge.id.as_str() {
            "welcome" => {
                challenge.variants = vec![
                    ChallengeVariant {
                        difficulty: ChallengeDifficulty::Beginner,
                        title_suffix: " (Tutorial Mode)".to_string(),
                        description_override: Some(format!("{}\n\n📚 TUTORIAL MODE: This is Base64 encoding. Base64 converts binary data to text using 64 characters (A-Z, a-z, 0-9, +, /). The '=' at the end is padding.", challenge.description)),
                        prompt_override: None,
                        xp_multiplier: 0.5,
                        sanity_multiplier: 0.5,
                        hints_override: Some(vec![
                            "Base64 uses 64 characters: A-Z, a-z, 0-9, +, and /.".to_string(),
                            "You can decode this online at base64decode.org".to_string(),
                            "Or use: echo 'encoded_text' | base64 -d".to_string(),
                        ]),
                        time_limit: None,
                        check_answer_override: None,
                        solution_override: None,
                    },
                    ChallengeVariant {
                        difficulty: ChallengeDifficulty::Advanced,
                        title_suffix: " (Speed Mode)".to_string(),
                        description_override: None,
                        prompt_override: Some("Decode quickly: V2VsY29tZSB0byB0aGUgR2hvc3QgUHJvdG9jb2w=".to_string()),
                        xp_multiplier: 1.5,
                        sanity_multiplier: 1.2,
                        hints_override: Some(vec![
                            "Base64 encoding - decode it.".to_string(),
                        ]),
                        time_limit: Some(30), // 30 seconds
                        check_answer_override: None,
                        solution_override: None,
                    },
                ];
            }
            "caesar_shift" => {
                challenge.variants = vec![
                    ChallengeVariant {
                        difficulty: ChallengeDifficulty::Beginner,
                        title_suffix: " (Guided)".to_string(),
                        description_override: Some(format!("{}\n\n📚 TUTORIAL: Caesar cipher shifts each letter by a fixed number. A=1, B=2, etc. ROT13 shifts by 13.", challenge.description)),
                        prompt_override: None,
                        xp_multiplier: 0.6,
                        sanity_multiplier: 0.5,
                        hints_override: Some(vec![
                            "This is ROT13: each letter is shifted 13 positions.".to_string(),
                            "A becomes N, B becomes O, etc.".to_string(),
                            "Online tool: rot13.com or just shift each letter by 13.".to_string(),
                        ]),
                        time_limit: None,
                        check_answer_override: None,
                        solution_override: None,
                    },
                    ChallengeVariant {
                        difficulty: ChallengeDifficulty::Expert,
                        title_suffix: " (Unknown Shift)".to_string(),
                        description_override: Some("The terminal displays: 'OLSSV DVYSK' - Figure out the shift value and decode the message.".to_string()),
                        prompt_override: Some("Decode: OLSSV DVYSK".to_string()),
                        xp_multiplier: 2.0,
                        sanity_multiplier: 1.5,
                        hints_override: Some(vec![
                            "Try different shift values from 1-25.".to_string(),
                        ]),
                        time_limit: Some(120), // 2 minutes
                        check_answer_override: Some(|answer| {
                            let a = answer.to_uppercase();
                            a.contains("HELLO WORLD") || a.contains("HELLOWORLD")
                        }),
                        solution_override: Some("HELLO WORLD".to_string()),
                    },
                ];
            }
            "hex_decode" => {
                challenge.variants = vec![ChallengeVariant {
                    difficulty: ChallengeDifficulty::Dynamic,
                    title_suffix: " (Random)".to_string(),
                    description_override: Some(
                        "A randomly generated hex string appears. Decode it to ASCII.".to_string(),
                    ),
                    prompt_override: Some(generate_random_hex_challenge()),
                    xp_multiplier: 1.3,
                    sanity_multiplier: 1.0,
                    hints_override: Some(vec![
                        "Hexadecimal uses base 16 (0-9, A-F).".to_string(),
                        "Each pair represents one ASCII character.".to_string(),
                    ]),
                    time_limit: None,
                    check_answer_override: Some(|answer| {
                        // For dynamic challenges, we'll implement a flexible validator later
                        answer.len() > 3 && answer.chars().all(|c| c.is_ascii())
                    }),
                    solution_override: Some("VARIES".to_string()),
                }];
            }
            _ => {} // No variants for other challenges yet
        }
    }
    challenges
}

/// Generate a random hex challenge for dynamic difficulty (v1.2.0)
fn generate_random_hex_challenge() -> String {
    use rand::Rng;
    let words = [
        "HACK", "CODE", "BYTE", "DATA", "LINK", "NODE", "CORE", "GHOST", "CYBER", "SECURE",
    ];
    let mut rng = rand::rng();
    let word = words[rng.random_range(0..words.len())];

    // Convert to hex
    word.bytes()
        .map(|b| format!("{:02X}", b))
        .collect::<String>()
}

/// Generate a random Base64 challenge (v1.2.0)
fn generate_random_base64_challenge() -> (String, String) {
    use rand::Rng;
    let messages = [
        "PROTOCOL ACTIVE",
        "SYSTEM BREACH",
        "ACCESS GRANTED",
        "GHOST MODE ON",
        "SECURE CHANNEL",
        "DATA ENCRYPTED",
        "STEALTH ENABLED",
        "PROXY CONNECTED",
    ];
    let mut rng = rand::rng();
    let message = messages[rng.random_range(0..messages.len())];

    // Simple base64 encoding (we'll use precomputed values for now)
    let encoded = match message {
        "PROTOCOL ACTIVE" => "UFJPVE9DT0wgQUNUSVZF",
        "SYSTEM BREACH" => "U1lTVEVNIEJSRUFDSA==",
        "ACCESS GRANTED" => "QUNDRVNTIEJSQU5URUQ=",
        "GHOST MODE ON" => "R0hPU1QgTU9ERSBPTg==",
        "SECURE CHANNEL" => "U0VDVVJFIENIQUFOTEVS",
        "DATA ENCRYPTED" => "REFUQSBFTUNSRVBURUQ=",
        "STEALTH ENABLED" => "U1RFQUxUSCBFTkFCTEVE",
        "PROXY CONNECTED" => "UFJPWFKGQ09OTkVDVEVE",
        _ => "VU5LTk9XTg==", // "UNKNOWN"
    };

    (encoded.to_string(), message.to_string())
}

/// Generate a random ROT cipher challenge (v1.2.0)
fn generate_random_rot_challenge() -> (String, String, u8) {
    use rand::Rng;
    let messages = [
        "HELLO WORLD",
        "SECRET CODE",
        "GHOST HACK",
        "CYBER PUNK",
        "DATA MINE",
        "NETWORK",
    ];
    let mut rng = rand::rng();
    let message = messages[rng.random_range(0..messages.len())];
    let shift = rng.random_range(1..=25);

    // Apply ROT cipher
    let encoded = message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                ((c as u8 - base + shift) % 26 + base) as char
            } else {
                c
            }
        })
        .collect::<String>();

    (encoded, message.to_string(), shift)
}

/// Generate a random simple SQL injection challenge (v1.2.0)
fn generate_random_sql_challenge() -> (String, String) {
    use rand::Rng;
    let scenarios = [
        (
            "admin",
            "SELECT * FROM users WHERE username = 'admin' AND password = 'PASSWORD'",
        ),
        (
            "root",
            "SELECT * FROM accounts WHERE user = 'root' AND pass = 'PASSWORD'",
        ),
        (
            "guest",
            "SELECT id FROM login WHERE name = 'guest' AND pwd = 'PASSWORD'",
        ),
    ];
    let payloads = ["' OR 1=1--", "' OR 'x'='x", "' UNION SELECT 1--"];

    let mut rng = rand::rng();
    let (_user, query_template) = scenarios[rng.random_range(0..scenarios.len())];
    let payload = payloads[rng.random_range(0..payloads.len())];

    let vulnerable_query = query_template.replace("PASSWORD", &format!("{}{}", "secret", payload));
    (vulnerable_query, payload.to_string())
}

/// Create dynamic challenges that are randomly generated (v1.2.0)
pub fn create_dynamic_challenges() -> Vec<Challenge> {
    vec![
        Challenge {
            id: "dynamic_base64".to_string(),
            title: "Dynamic Base64 Decoder".to_string(),
            description: "A new Base64 encoded message appears each time you attempt this challenge.".to_string(),
            prompt: {
                let (encoded, _) = generate_random_base64_challenge();
                format!("Decode this Base64 string: {}", encoded)
            },
            category: ChallengeCategory::Encoding,
            level: 0,
            xp_reward: 35,
            sanity_cost: 3,
            check_answer: |answer| {
                // For dynamic challenges, we need a more flexible validator
                let a = answer.to_uppercase();
                a.contains("PROTOCOL") || a.contains("SYSTEM") || a.contains("ACCESS") ||
                a.contains("GHOST") || a.contains("SECURE") || a.contains("DATA") ||
                a.contains("STEALTH") || a.contains("PROXY") || a.contains("ACTIVE") ||
                a.contains("BREACH") || a.contains("GRANTED") || a.contains("MODE") ||
                a.contains("CHANNEL") || a.contains("ENCRYPTED") || a.contains("ENABLED") ||
                a.contains("CONNECTED")
            },
            solution: "VARIES".to_string(),
            hints: vec![
                "This is Base64 encoding - each time it's different!".to_string(),
                "Use online Base64 decoder or command line tools.".to_string(),
                "Look for common words like PROTOCOL, SYSTEM, ACCESS, etc.".to_string(),
            ],
            difficulty: ChallengeDifficulty::Dynamic,
            variants: vec![],
        },
        Challenge {
            id: "dynamic_rot".to_string(),
            title: "Mystery ROT Cipher".to_string(),
            description: "Figure out the ROT shift value and decode the message. The shift changes each attempt!".to_string(),
            prompt: {
                let (encoded, _, _) = generate_random_rot_challenge();
                format!("Decode this ROT cipher: {}", encoded)
            },
            category: ChallengeCategory::Cryptography,
            level: 1,
            xp_reward: 60,
            sanity_cost: 8,
            check_answer: |answer| {
                let a = answer.to_uppercase();
                a.contains("HELLO") || a.contains("SECRET") || a.contains("GHOST") ||
                a.contains("CYBER") || a.contains("DATA") || a.contains("NETWORK") ||
                a.contains("WORLD") || a.contains("CODE") || a.contains("HACK") ||
                a.contains("PUNK") || a.contains("MINE")
            },
            solution: "VARIES".to_string(),
            hints: vec![
                "Try different ROT values from 1 to 25.".to_string(),
                "ROT13 is common, but this could be any shift value.".to_string(),
                "Look for recognizable English words in the result.".to_string(),
            ],
            difficulty: ChallengeDifficulty::Dynamic,
            variants: vec![],
        },
        Challenge {
            id: "dynamic_hex".to_string(),
            title: "Random Hex Decoder".to_string(),
            description: "New hex-encoded data appears each time. Decode it to ASCII text.".to_string(),
            prompt: {
                let hex = generate_random_hex_challenge();
                format!("Decode this hex to ASCII: {}", hex)
            },
            category: ChallengeCategory::Encoding,
            level: 0,
            xp_reward: 40,
            sanity_cost: 4,
            check_answer: |answer| {
                let a = answer.to_uppercase();
                a.contains("HACK") || a.contains("CODE") || a.contains("BYTE") ||
                a.contains("DATA") || a.contains("LINK") || a.contains("NODE") ||
                a.contains("CORE") || a.contains("GHOST") || a.contains("CYBER") ||
                a.contains("SECURE")
            },
            solution: "VARIES".to_string(),
            hints: vec![
                "Hexadecimal uses base 16 (0-9, A-F).".to_string(),
                "Each pair of hex digits represents one ASCII character.".to_string(),
                "Convert each hex pair to decimal, then to ASCII.".to_string(),
            ],
            difficulty: ChallengeDifficulty::Dynamic,
            variants: vec![],
        },
    ]
}

/// Get challenges with variants applied based on user preference (v1.2.0)
pub fn get_challenges_with_variants() -> Vec<Challenge> {
    get_all_challenges().clone()
}

/// Get dynamic challenges for practice mode (v1.2.0)
pub fn get_dynamic_challenges() -> Vec<Challenge> {
    create_dynamic_challenges()
}

/// Get challenge for specific difficulty level (v1.2.0)
pub fn get_challenge_with_difficulty(
    challenge_id: &str,
    difficulty: ChallengeDifficulty,
) -> Option<Challenge> {
    let challenges = get_challenges_with_variants();
    let base_challenge = challenges.iter().find(|c| c.id == challenge_id)?;
    Some(base_challenge.with_difficulty(difficulty))
}

/// Get all available difficulties for a challenge (v1.2.0)
pub fn get_challenge_difficulties(challenge_id: &str) -> Vec<ChallengeDifficulty> {
    let challenges = get_challenges_with_variants();
    if let Some(challenge) = challenges.iter().find(|c| c.id == challenge_id) {
        challenge.get_available_difficulties()
    } else {
        vec![]
    }
}

pub fn get_challenges_for_level(level: usize) -> Vec<Challenge> {
    // v1.2.0: Now uses variants system
    get_challenges_with_variants()
        .into_iter()
        .filter(|c| c.level == level)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_challenges_have_valid_ids() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                !challenge.id.is_empty(),
                "Challenge '{}' has empty ID",
                challenge.title
            );
            assert!(
                challenge
                    .id
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_'),
                "Challenge '{}' has invalid ID characters",
                challenge.title
            );
        }
    }

    #[test]
    fn test_all_challenges_have_unique_ids() {
        let challenges = get_all_challenges();
        let mut ids = std::collections::HashSet::new();
        for challenge in challenges {
            assert!(
                ids.insert(challenge.id.clone()),
                "Duplicate challenge ID found: {}",
                challenge.id
            );
        }
    }

    #[test]
    fn test_all_challenges_have_titles() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                !challenge.title.is_empty(),
                "Challenge '{}' has empty title",
                challenge.id
            );
        }
    }

    #[test]
    fn test_all_challenges_have_descriptions() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                !challenge.description.is_empty(),
                "Challenge '{}' has empty description",
                challenge.id
            );
            assert!(
                challenge.description.len() > 20,
                "Challenge '{}' has too short description",
                challenge.id
            );
        }
    }

    #[test]
    fn test_all_challenges_have_hints() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                !challenge.hints.is_empty(),
                "Challenge '{}' has no hints",
                challenge.id
            );
            assert!(
                challenge.hints.len() >= 2,
                "Challenge '{}' should have at least 2 hints",
                challenge.id
            );
        }
    }

    #[test]
    fn test_challenge_rewards_are_positive() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                challenge.xp_reward > 0,
                "Challenge '{}' has non-positive XP reward",
                challenge.id
            );
            assert!(
                challenge.sanity_cost > 0,
                "Challenge '{}' has non-positive sanity cost",
                challenge.id
            );
        }
    }

    #[test]
    fn test_challenge_levels_are_valid() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                challenge.level <= 4,
                "Challenge '{}' has invalid level {}",
                challenge.id,
                challenge.level
            );
        }
    }

    #[test]
    fn test_rewards_scale_with_difficulty() {
        let challenges = get_all_challenges();

        // Level 0 should have lower rewards than Level 3
        let level_0: Vec<_> = challenges.iter().filter(|c| c.level == 0).collect();
        let level_3: Vec<_> = challenges.iter().filter(|c| c.level == 3).collect();

        if !level_0.is_empty() && !level_3.is_empty() {
            let avg_xp_0: i32 =
                level_0.iter().map(|c| c.xp_reward).sum::<i32>() / level_0.len() as i32;
            let avg_xp_3: i32 =
                level_3.iter().map(|c| c.xp_reward).sum::<i32>() / level_3.len() as i32;
            assert!(
                avg_xp_0 < avg_xp_3,
                "Level 0 average XP ({}) should be less than Level 3 average XP ({})",
                avg_xp_0,
                avg_xp_3
            );
        }
    }

    #[test]
    fn test_get_challenges_for_level() {
        let level_0 = get_challenges_for_level(0);
        let level_1 = get_challenges_for_level(1);

        assert!(!level_0.is_empty(), "Level 0 should have challenges");
        assert!(!level_1.is_empty(), "Level 1 should have challenges");

        for challenge in &level_0 {
            assert_eq!(challenge.level, 0, "Level 0 filter returned wrong level");
        }
    }

    // Test specific challenge answers
    #[test]
    fn test_welcome_challenge() {
        let challenges = get_all_challenges();
        let welcome = challenges
            .iter()
            .find(|c| c.id == "welcome")
            .expect("Welcome challenge not found");

        // Test correct answer
        assert!((welcome.check_answer)("welcome to the ghost protocol"));
        assert!((welcome.check_answer)("Welcome to the Ghost Protocol"));
        assert!((welcome.check_answer)("WELCOME TO THE GHOST PROTOCOL"));

        // Test incorrect answers
        assert!(!(welcome.check_answer)("welcome"));
        assert!(!(welcome.check_answer)("ghost protocol"));
        assert!(!(welcome.check_answer)(""));
    }

    #[test]
    fn test_file_discovery_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "file_discovery")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("ghost_admin_2024"));
        assert!(!(challenge.check_answer)("ghost_admin"));
        assert!(!(challenge.check_answer)("2024"));
    }

    #[test]
    fn test_port_scan_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "port_scan")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("6666"));
        assert!(!(challenge.check_answer)("666"));
        assert!(!(challenge.check_answer)("80"));
    }

    #[test]
    fn test_rot13_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "rot13_ghost")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("ROTATION"));
        assert!((challenge.check_answer)("rotation"));
        assert!((challenge.check_answer)("Rotation"));
        assert!(!(challenge.check_answer)("rotate"));
    }

    #[test]
    fn test_binary_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "binary_basics")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("GHOST"));
        assert!((challenge.check_answer)("ghost"));
        assert!(!(challenge.check_answer)("Ghost Protocol"));
    }

    #[test]
    fn test_url_decode_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "url_decode")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("UNLOCK"));
        assert!((challenge.check_answer)("unlock"));
        assert!(!(challenge.check_answer)("lock"));
    }

    #[test]
    fn test_caesar_cipher_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "caesar_cipher")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("cryptography"));
        assert!((challenge.check_answer)("CRYPTOGRAPHY"));
        assert!((challenge.check_answer)("Cryptography"));
        assert!(!(challenge.check_answer)("crypto"));
    }

    #[test]
    fn test_sql_injection_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "sql_injection_basics")
            .expect("Challenge not found");

        // Test various SQL injection payloads
        assert!((challenge.check_answer)("' OR '1'='1' --"));
        assert!((challenge.check_answer)("'or'1'='1'--"));
        assert!((challenge.check_answer)("' OR 1=1 --"));
        assert!((challenge.check_answer)("'or1=1--"));
        assert!((challenge.check_answer)("admin'--"));

        assert!(!(challenge.check_answer)("admin"));
        assert!(!(challenge.check_answer)("password"));
    }

    #[test]
    fn test_hex_decode_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "hex_decode")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("HEXADECIMAL"));
        assert!((challenge.check_answer)("hexadecimal"));
        assert!(!(challenge.check_answer)("hex"));
    }

    #[test]
    fn test_jwt_token_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "jwt_token")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("algorithm confusion"));
        assert!((challenge.check_answer)("Algorithm Confusion"));
        assert!((challenge.check_answer)("none algorithm"));
        assert!((challenge.check_answer)("alg none"));
        assert!(!(challenge.check_answer)("jwt"));
    }

    #[test]
    fn test_path_traversal_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "path_traversal")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("../"));
        assert!((challenge.check_answer)(".."));
        assert!((challenge.check_answer)("..\\"));
        assert!(!(challenge.check_answer)("/"));
    }

    #[test]
    fn test_md5_collision_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "md5_collision")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("password"));
        assert!((challenge.check_answer)("PASSWORD"));
        assert!(!(challenge.check_answer)("admin"));
    }

    #[test]
    fn test_command_injection_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "command_injection")
            .expect("Challenge not found");

        assert!((challenge.check_answer)(";"));
        assert!((challenge.check_answer)("&"));
        assert!((challenge.check_answer)("|"));
        assert!((challenge.check_answer)("&&"));
        assert!((challenge.check_answer)("||"));
        assert!((challenge.check_answer)("semicolon"));
        assert!(!(challenge.check_answer)("bash"));
    }

    #[test]
    fn test_xss_attack_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "xss_attack")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("script"));
        assert!((challenge.check_answer)("SCRIPT"));
        assert!((challenge.check_answer)("<script>"));
        assert!(!(challenge.check_answer)("javascript"));
    }

    #[test]
    fn test_session_hijack_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "session_hijack")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("session hijacking"));
        assert!((challenge.check_answer)("Session Hijacking"));
        assert!((challenge.check_answer)("session hijack"));
        assert!((challenge.check_answer)("cookie hijacking"));
        assert!(!(challenge.check_answer)("cookie"));
    }

    #[test]
    fn test_cors_bypass_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "cors_bypass")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("cors"));
        assert!((challenge.check_answer)("CORS"));
        assert!((challenge.check_answer)("cross origin resource sharing"));
        assert!((challenge.check_answer)("Cross-Origin Resource Sharing"));
        assert!(!(challenge.check_answer)("origin"));
    }

    #[test]
    fn test_buffer_overflow_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "binary_exploit")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("BUFFER OVERFLOW"));
        assert!((challenge.check_answer)("buffer overflow"));
        assert!((challenge.check_answer)("BUFFEROVERFLOW"));
        assert!((challenge.check_answer)("OVERFLOW"));
        assert!(!(challenge.check_answer)("buffer"));
    }

    #[test]
    fn test_reverse_engineering_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "reverse_engineering")
            .expect("Challenge not found");

        // 0x2D XOR 0x42 = 0x6F = 111 = 'o'
        assert!((challenge.check_answer)("o"));
        assert!((challenge.check_answer)("111"));
        assert!((challenge.check_answer)("0x6F"));
        assert!(!(challenge.check_answer)("B"));
    }

    #[test]
    fn test_format_string_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "format_string")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("format string"));
        assert!((challenge.check_answer)("Format String Vulnerability"));
        assert!((challenge.check_answer)("format_string"));
        assert!(!(challenge.check_answer)("printf"));
    }

    #[test]
    fn test_race_condition_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "race_condition")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("race condition"));
        assert!((challenge.check_answer)("Race Condition"));
        assert!((challenge.check_answer)("toctou"));
        assert!((challenge.check_answer)("time of check time of use"));
        assert!(!(challenge.check_answer)("race"));
    }

    #[test]
    fn test_integer_overflow_challenge() {
        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == "integer_overflow")
            .expect("Challenge not found");

        assert!((challenge.check_answer)("integer overflow"));
        assert!((challenge.check_answer)("Integer Overflow"));
        assert!((challenge.check_answer)("integer wraparound"));
        assert!((challenge.check_answer)("arithmetic overflow"));
        assert!(!(challenge.check_answer)("overflow"));
    }

    #[test]
    fn test_total_challenge_count() {
        let challenges = get_all_challenges();
        assert_eq!(
            challenges.len(),
            48,
            "Expected 48 total challenges (v1.5.0: 36 base + 12 new)"
        );
    }

    #[test]
    fn test_level_distribution() {
        let level_0 = get_challenges_for_level(0);
        let level_1 = get_challenges_for_level(1);
        let level_2 = get_challenges_for_level(2);
        let level_3 = get_challenges_for_level(3);
        let level_4 = get_challenges_for_level(4);

        assert_eq!(level_0.len(), 6, "Expected 6 challenges in Level 0");
        assert_eq!(level_1.len(), 7, "Expected 7 challenges in Level 1");
        assert_eq!(
            level_2.len(),
            22,
            "Expected 22 challenges in Level 2 (v1.5.0: +5 stego, +2 extra)"
        );
        assert_eq!(
            level_3.len(),
            12,
            "Expected 12 challenges in Level 3 (v1.5.0: +1 stego, +3 malware, +3 iot)"
        );
        assert_eq!(level_4.len(), 1, "Expected 1 challenge in Level 4");
    }

    #[test]
    fn test_no_empty_hints() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            for (i, hint) in challenge.hints.iter().enumerate() {
                assert!(
                    !hint.is_empty(),
                    "Challenge '{}' has empty hint at index {}",
                    challenge.id,
                    i
                );
            }
        }
    }

    #[test]
    fn test_challenge_ids_follow_naming_convention() {
        let challenges = get_all_challenges();
        for challenge in challenges {
            assert!(
                challenge
                    .id
                    .chars()
                    .all(|c| c.is_lowercase() || c.is_numeric() || c == '_'),
                "Challenge '{}' ID should be lowercase with underscores",
                challenge.id
            );
        }
    }

    // Property-based tests
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// Property: Challenge validators should never panic on any input
            #[test]
            fn test_validators_never_panic(input in "\\PC*") {
                let challenges = get_all_challenges();
                for challenge in challenges {
                    // Should not panic, regardless of input
                    let _ = (challenge.check_answer)(&input);
                }
            }

            /// Property: Empty input should never be accepted as correct
            #[test]
            fn test_empty_input_never_valid(whitespace in "[ \t\n\r]*") {
                let challenges = get_all_challenges();
                for challenge in challenges {
                    // Empty/whitespace-only input should not be valid
                    assert!(
                        !(challenge.check_answer)(&whitespace),
                        "Challenge '{}' should not accept whitespace-only input",
                        challenge.id
                    );
                }
            }

            /// Property: Very long inputs should not crash
            #[test]
            fn test_long_inputs_handled(repeat in 1..1000usize) {
                let long_input = "A".repeat(repeat);
                let challenges = get_all_challenges();
                for challenge in challenges {
                    let _ = (challenge.check_answer)(&long_input);
                }
            }

            /// Property: Special characters don't cause panics
            #[test]
            fn test_special_chars_safe(special in "[!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?`~]*") {
                let challenges = get_all_challenges();
                for challenge in challenges {
                    let _ = (challenge.check_answer)(&special);
                }
            }

            /// Property: Case variations of wrong answers are still wrong
            #[test]
            fn test_wrong_answer_case_insensitive(wrong in "wrong|incorrect|invalid|nope|bad") {
                let challenges = get_all_challenges();
                let variations = vec![
                    wrong.to_lowercase(),
                    wrong.to_uppercase(),
                    wrong.to_string(),
                ];

                for challenge in challenges {
                    for variant in &variations {
                        // These generic wrong answers should not match any challenge
                        // (unless by chance a challenge actually expects "wrong" as answer)
                        let _ = (challenge.check_answer)(variant);
                    }
                }
            }

            /// Property: Numeric inputs are handled safely
            #[test]
            fn test_numeric_inputs_safe(num in any::<i64>()) {
                let num_str = num.to_string();
                let challenges = get_all_challenges();
                for challenge in challenges {
                    let _ = (challenge.check_answer)(&num_str);
                }
            }

            /// Property: Unicode characters don't break validators
            #[test]
            fn test_unicode_safe(unicode in "[\\u{0000}-\\u{FFFF}]{0,20}") {
                let challenges = get_all_challenges();
                for challenge in challenges {
                    let _ = (challenge.check_answer)(&unicode);
                }
            }
        }
    }
}
