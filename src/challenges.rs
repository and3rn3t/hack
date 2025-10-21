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
            let input = ui::read_input(&format!("Enter your answer (attempt {}/{}) or 'hint' for help or 'skip': ", attempts + 1, max_attempts))?;

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
                if attempts >= max_attempts {
                    ui::print_error("Maximum attempts reached. Challenge failed.")?;
                    state.modify_sanity(-10);
                    ui::pause()?;
                    return Ok(false);
                }
                ui::print_error("Incorrect answer. Try again.")?;
            }
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
                normalized.contains("'or'1'='1'--") || normalized.contains("'or1=1--") || normalized == "admin'--"
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
            |answer| answer.chars().next() == Some('o') || answer == "111" || answer == "0x6F",
            vec![
                "XOR is reversible: A XOR B = C means A XOR C = B".to_string(),
                "Calculate: 0x2D XOR 0x42 = ?".to_string(),
                "The answer is 'o' (ASCII 111 or 0x6F)".to_string(),
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
