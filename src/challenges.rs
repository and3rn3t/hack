use crate::ui::CompletionContext;
use crate::{narrative, state::GameState, ui};
use std::io;

#[derive(Clone)]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub description: String,
    pub level: usize,
    pub xp_reward: i32,
    pub sanity_cost: i32,
    pub check_answer: fn(&str) -> bool,
    pub hints: Vec<String>,
}

impl Challenge {
    pub fn new(
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
            level,
            xp_reward,
            sanity_cost,
            check_answer,
            hints,
        }
    }

    pub fn attempt(&self, state: &mut GameState) -> io::Result<bool> {
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

    fn provide_feedback(&self, answer: &str, attempt_num: usize) -> io::Result<()> {
        // Basic feedback - can be enhanced per challenge
        if attempt_num == 1 {
            ui::print_colored("\n❌ Incorrect. ", crossterm::style::Color::Red)?;
            ui::print_colored(
                "Review the challenge description carefully.\n",
                crossterm::style::Color::White,
            )?;
        } else if attempt_num == 2 {
            ui::print_colored("\n❌ Still incorrect. ", crossterm::style::Color::Red)?;
            ui::print_colored(
                "Consider using 'hint' for guidance.\n",
                crossterm::style::Color::Yellow,
            )?;
        } else if attempt_num >= 3 {
            ui::print_colored("\n❌ Incorrect. ", crossterm::style::Color::Red)?;

            // Provide specific feedback based on answer analysis
            if answer.is_empty() {
                ui::print_colored(
                    "Your answer is empty. Please provide a value.\n",
                    crossterm::style::Color::White,
                )?;
            } else if answer.len() < 3 {
                ui::print_colored(
                    "Your answer seems very short. Check if you're missing something.\n",
                    crossterm::style::Color::White,
                )?;
            } else if answer.len() > 100 {
                ui::print_colored(
                    "Your answer is quite long. The solution might be simpler.\n",
                    crossterm::style::Color::White,
                )?;
            } else {
                ui::print_colored(
                    &format!(
                        "You've tried {} times. Use 'hint' if you need help.\n",
                        attempt_num
                    ),
                    crossterm::style::Color::White,
                )?;
            }
        }

        Ok(())
    }

    fn show_learning_resources(&self) -> io::Result<()> {
        ui::print_separator()?;
        ui::print_colored("\n💡 LEARNING RESOURCES:\n", crossterm::style::Color::Cyan)?;

        // Provide category-specific learning tips
        let category_tip = self.get_category_tip();
        ui::print_colored(
            &format!("   • {}\n", category_tip),
            crossterm::style::Color::White,
        )?;

        ui::print_colored(
            "   • Review the challenge description and hints carefully\n",
            crossterm::style::Color::White,
        )?;

        ui::print_colored(
            "   • Try searching online for the concepts mentioned\n",
            crossterm::style::Color::White,
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

pub fn get_all_challenges() -> Vec<Challenge> {
    vec![
        // Level 0: Beginner challenges
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        // Level 3+: Advanced challenges
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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
        Challenge::new(
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

pub fn get_challenges_for_level(level: usize) -> Vec<Challenge> {
    get_all_challenges()
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
        for challenge in &challenges {
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
        for challenge in &challenges {
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
        for challenge in &challenges {
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
        for challenge in &challenges {
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
        for challenge in &challenges {
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
        for challenge in &challenges {
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
        for challenge in &challenges {
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
        assert_eq!(challenges.len(), 26, "Expected 26 total challenges");
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
        assert_eq!(level_2.len(), 7, "Expected 7 challenges in Level 2");
        assert_eq!(level_3.len(), 5, "Expected 5 challenges in Level 3");
        assert_eq!(level_4.len(), 1, "Expected 1 challenge in Level 4");
    }

    #[test]
    fn test_no_empty_hints() {
        let challenges = get_all_challenges();
        for challenge in &challenges {
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
        for challenge in &challenges {
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
                for challenge in &challenges {
                    // Should not panic, regardless of input
                    let _ = (challenge.check_answer)(&input);
                }
            }

            /// Property: Empty input should never be accepted as correct
            #[test]
            fn test_empty_input_never_valid(whitespace in "[ \t\n\r]*") {
                let challenges = get_all_challenges();
                for challenge in &challenges {
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
                for challenge in &challenges {
                    let _ = (challenge.check_answer)(&long_input);
                }
            }

            /// Property: Special characters don't cause panics
            #[test]
            fn test_special_chars_safe(special in "[!@#$%^&*()_+\\-=\\[\\]{}|;':\",./<>?`~]*") {
                let challenges = get_all_challenges();
                for challenge in &challenges {
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

                for challenge in &challenges {
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
                for challenge in &challenges {
                    let _ = (challenge.check_answer)(&num_str);
                }
            }

            /// Property: Unicode characters don't break validators
            #[test]
            fn test_unicode_safe(unicode in "[\\u{0000}-\\u{FFFF}]{0,20}") {
                let challenges = get_all_challenges();
                for challenge in &challenges {
                    let _ = (challenge.check_answer)(&unicode);
                }
            }
        }
    }
}
