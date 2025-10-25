# Challenge Design Guide for Developers

**Target Audience**: Developers who want to create new challenges for The Hack: Ghost Protocol
**Last Updated**: October 24, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Challenge Architecture](#challenge-architecture)
3. [Design Patterns](#design-patterns)
4. [Category Guidelines](#category-guidelines)
5. [Implementation Steps](#implementation-steps)
6. [Testing Requirements](#testing-requirements)
7. [Horror Theme Integration](#horror-theme-integration)
8. [Difficulty Balancing](#difficulty-balancing)
9. [Educational Objectives](#educational-objectives)
10. [Code Examples](#code-examples)
11. [Best Practices](#best-practices)
12. [Common Pitfalls](#common-pitfalls)
13. [Review Checklist](#review-checklist)

---

## Overview

The Hack: Ghost Protocol uses a sophisticated challenge system that balances **educational value**, **horror atmosphere**, and **progressive difficulty**. This guide provides everything developers need to create high-quality challenges that fit seamlessly into the game's ecosystem.

### Core Principles

1. **Educational First**: Every challenge should teach a real cybersecurity concept
2. **Horror Integration**: Challenges should contribute to the dark narrative
3. **Progressive Difficulty**: Each challenge should build on previous knowledge
4. **Accessibility**: Beginners should be able to learn with proper hints
5. **Authenticity**: Use real-world techniques, not fictional hacks

---

## Challenge Architecture

### Challenge Struct

```rust
pub struct Challenge {
    pub id: &'static str,           // Unique identifier (snake_case)
    pub title: &'static str,        // Display name (Title Case)
    pub description: &'static str,  // Educational context and scenario
    pub prompt: &'static str,       // What the player sees
    pub level: u32,                // Difficulty level (0-4)
    pub xp_reward: u32,            // Experience points awarded
    pub sanity_cost: u32,          // Sanity points lost
    pub check_answer: fn(&str) -> bool, // Validation function
    pub hints: Vec<String>,        // Progressive hints
    pub category: ChallengeCategory, // Challenge type
}
```

### Key Components

#### **ID Convention**

-   **Format**: `snake_case` with descriptive names
-   **Pattern**: `{category}_{concept}` or `{level}_{name}`
-   **Examples**: `crypto_caesar`, `web_sql_injection`, `basic_base64`

#### **Validation Function**

The heart of every challenge - must be:

-   **Pure function**: No side effects
-   **Robust**: Handle any input without panicking
-   **Case-insensitive**: When appropriate
-   **Flexible**: Accept multiple valid formats

#### **Hint System**

Progressive learning support:

-   **Hint 1**: General concept explanation
-   **Hint 2**: Specific technique or tool
-   **Hint 3**: Nearly direct answer (last resort)

---

## Design Patterns

### Pattern 1: Encoding/Decoding Challenges

**Use Case**: Teaching data representation and transformation

```rust
Challenge {
    id: "encoding_base64",
    title: "The Encoded Message",
    description: "You discover a suspicious file containing what appears to be encoded text...",
    prompt: "Decode this Base64 string: aGVsbG8gd29ybGQ=",
    level: 0,
    xp_reward: 50,
    sanity_cost: 5,
    check_answer: |answer| {
        let decoded = answer.to_lowercase().trim();
        decoded == "hello world"
    },
    hints: vec![
        "This looks like Base64 encoding - notice the = padding".to_string(),
        "Base64 is commonly used to encode binary data as text".to_string(),
        "Try using a Base64 decoder tool or command".to_string(),
    ],
    category: ChallengeCategory::Encoding,
}
```

### Pattern 2: Cryptography Challenges

**Use Case**: Teaching cipher techniques and cryptanalysis

```rust
Challenge {
    id: "crypto_caesar",
    title: "The Shifted Letters",
    description: "An encrypted note was found in the ghost's belongings...",
    prompt: "Decrypt this Caesar cipher (ROT-13): URYYB JBEYQ",
    level: 1,
    xp_reward: 75,
    sanity_cost: 8,
    check_answer: |answer| {
        let normalized = answer.to_lowercase().replace(" ", "");
        normalized == "helloworld" || normalized == "hello world"
    },
    hints: vec![
        "Caesar cipher shifts each letter by a fixed amount".to_string(),
        "ROT-13 means each letter is shifted by 13 positions".to_string(),
        "A becomes N, B becomes O, etc.".to_string(),
    ],
    category: ChallengeCategory::Cryptography,
}
```

### Pattern 3: Web Security Challenges

**Use Case**: Teaching web vulnerabilities and attack vectors

```rust
Challenge {
    id: "web_sql_injection",
    title: "The Vulnerable Database",
    description: "You find a login form that seems poorly secured...",
    prompt: "What SQL injection payload bypasses authentication?\nLogin form: SELECT * FROM users WHERE username='?' AND password='?'",
    level: 1,
    xp_reward: 100,
    sanity_cost: 10,
    check_answer: |answer| {
        let normalized = answer.to_lowercase().replace(" ", "").replace("'", "");
        normalized.contains("or1=1") || normalized.contains("or1=1--")
            || normalized.contains("admin--") || normalized == "or1=1"
    },
    hints: vec![
        "SQL injection exploits poor input validation in database queries".to_string(),
        "Try to make the WHERE condition always true".to_string(),
        "The payload 'admin' OR '1'='1' -- would work here".to_string(),
    ],
    category: ChallengeCategory::WebSecurity,
}
```

### Pattern 4: Analysis Challenges

**Use Case**: Teaching investigation and forensic techniques

```rust
Challenge {
    id: "forensics_headers",
    title: "The Hidden Clues",
    description: "Network traffic reveals suspicious HTTP communications...",
    prompt: "Analyze these HTTP headers - what's the server technology?\nServer: nginx/1.18.0\nX-Powered-By: PHP/7.4.3",
    level: 2,
    xp_reward: 125,
    sanity_cost: 12,
    check_answer: |answer| {
        let lower = answer.to_lowercase();
        (lower.contains("nginx") && lower.contains("php"))
            || lower.contains("nginx/1.18.0")
            || lower.contains("php/7.4.3")
    },
    hints: vec![
        "HTTP headers reveal information about server configuration".to_string(),
        "Look at both the Server and X-Powered-By headers".to_string(),
        "The server is running nginx version 1.18.0 with PHP 7.4.3".to_string(),
    ],
    category: ChallengeCategory::Forensics,
}
```

---

## Category Guidelines

### Level 0: The Awakening (Beginner)

**Target Skills**: Basic encoding, file operations, simple patterns

| Category         | Concepts                               | Example Challenges     |
| ---------------- | -------------------------------------- | ---------------------- |
| **Encoding**     | Base64, Hex, ASCII, URL encoding       | Decode hidden messages |
| **Files**        | File discovery, hidden files, metadata | Find secret files      |
| **Network**      | Port scanning basics, service ID       | Identify open services |
| **Basic Crypto** | ROT13, Caesar with small shifts        | Simple ciphers         |

**Design Tips**:

-   Use standard tools/techniques
-   Provide clear examples
-   Keep complexity low
-   Focus on pattern recognition

### Level 1: Whispers in the Code (Intermediate)

**Target Skills**: Intermediate crypto, basic web security, system analysis

| Category         | Concepts                                        | Example Challenges     |
| ---------------- | ----------------------------------------------- | ---------------------- |
| **Cryptography** | Caesar cipher, substitution, frequency analysis | Classical ciphers      |
| **Web Security** | SQL injection basics, XSS, directory traversal  | Basic web attacks      |
| **System**       | Command injection, log analysis                 | System exploitation    |
| **Mobile**       | Deep links, app security basics                 | Mobile vulnerabilities |

**Design Tips**:

-   Introduce multi-step solutions
-   Combine multiple concepts
-   Add realistic scenarios
-   Increase hint specificity

### Level 2: The Forgotten Server (Web/Mobile Focus)

**Target Skills**: Advanced web/mobile security, network analysis

| Category            | Concepts                                   | Example Challenges      |
| ------------------- | ------------------------------------------ | ----------------------- |
| **Web Advanced**    | HTTP headers, JWT, CORS, session attacks   | Complex web scenarios   |
| **Mobile Security** | Deep links, API abuse, certificate pinning | Mobile-specific attacks |
| **Network**         | DNS analysis, traffic inspection           | Network forensics       |
| **API Security**    | REST attacks, authentication bypass        | API vulnerabilities     |

**Design Tips**:

-   Focus on real-world scenarios
-   Introduce tool usage
-   Multi-vector attacks
-   Professional-level concepts

### Level 3+: Advanced Territories

**Target Skills**: Binary exploitation, advanced crypto, reverse engineering

| Category                | Concepts                                     | Example Challenges    |
| ----------------------- | -------------------------------------------- | --------------------- |
| **Binary**              | Buffer overflow concepts, memory corruption  | Binary exploitation   |
| **Reverse Engineering** | XOR analysis, decompilation, obfuscation     | Code analysis         |
| **Advanced Crypto**     | Modern ciphers, hash attacks, key management | Cryptographic attacks |
| **Protocol**            | Custom protocols, packet analysis            | Protocol security     |

**Design Tips**:

-   Require deeper technical knowledge
-   Multi-layered problems
-   Advanced tooling
-   Research-level concepts

---

## Implementation Steps

### Step 1: Design Phase

1. **Choose Educational Objective**

    - What specific skill should players learn?
    - How does it fit into the progression?
    - What real-world scenario does it represent?

2. **Horror Integration Planning**

    - How does this challenge advance the narrative?
    - What atmospheric elements can be added?
    - How does it contribute to the ghost story?

3. **Difficulty Assessment**
    - What level should this be?
    - What prerequisites are needed?
    - How does it compare to existing challenges?

### Step 2: Implementation

1. **Create the Challenge Struct**

    ```rust
    // Add to get_challenges_for_level() in challenges.rs
    Challenge {
        id: "your_unique_id",
        title: "Your Challenge Title",
        description: "Educational context...",
        prompt: "Player-facing challenge...",
        level: appropriate_level,
        xp_reward: balanced_reward,
        sanity_cost: appropriate_cost,
        check_answer: |answer| {
            // Robust validation logic
            let normalized = answer.to_lowercase().trim();
            // Multiple valid formats accepted
            normalized == "expected" || normalized == "alternate"
        },
        hints: vec![
            "General concept hint".to_string(),
            "Specific technique hint".to_string(),
            "Nearly direct answer".to_string(),
        ],
        category: ChallengeCategory::YourCategory,
    }
    ```

2. **Implement Validation Function**

    ```rust
    // Best practices for validation
    |answer| {
        // Normalize input
        let cleaned = answer.to_lowercase()
                           .trim()
                           .replace([' ', '-', '_'], "");

        // Multiple valid answers
        let valid_answers = [
            "answer1",
            "answer2",
            "alternateformat"
        ];

        // Flexible matching
        valid_answers.iter().any(|&valid| cleaned.contains(valid))
            || cleaned == "special_case"
    }
    ```

### Step 3: Integration

1. **Add to Challenge Lists**

    ```rust
    // In get_challenges_for_level() function
    match level {
        0 => vec![/* existing */, your_new_challenge()],
        1 => vec![/* existing */, your_new_challenge()],
        // etc.
    }
    ```

2. **Add Category if New**
    ```rust
    // In ChallengeCategory enum if needed
    #[derive(Debug, Clone, PartialEq)]
    pub enum ChallengeCategory {
        // existing categories
        YourNewCategory,
    }
    ```

### Step 4: Testing Implementation

1. **Create Unit Tests**

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_your_challenge_validation() {
            let challenge = your_challenge();

            // Test correct answers
            assert!((challenge.check_answer)("correct_answer"));
            assert!((challenge.check_answer)("CORRECT_ANSWER")); // case insensitive
            assert!((challenge.check_answer)(" correct_answer ")); // whitespace

            // Test incorrect answers
            assert!(!(challenge.check_answer)("wrong_answer"));
            assert!(!(challenge.check_answer)(""));
            assert!(!(challenge.check_answer)("random_string"));
        }

        #[test]
        fn test_your_challenge_properties() {
            let challenge = your_challenge();

            assert!(!challenge.id.is_empty());
            assert!(!challenge.title.is_empty());
            assert!(!challenge.description.is_empty());
            assert!(!challenge.hints.is_empty());
            assert!(challenge.xp_reward > 0);
            assert!(challenge.sanity_cost > 0);
            assert!(challenge.level <= 4);
        }
    }
    ```

---

## Testing Requirements

### Mandatory Tests

1. **Validation Function Tests**

    ```rust
    #[test]
    fn test_challenge_accepts_correct_answers() {
        let challenge = your_challenge();

        // All valid formats should work
        let valid_inputs = [
            "primary_answer",
            "PRIMARY_ANSWER",
            " primary_answer ",
            "primary-answer",
            "primary_answer",
        ];

        for input in valid_inputs {
            assert!(
                (challenge.check_answer)(input),
                "Should accept: '{}'", input
            );
        }
    }

    #[test]
    fn test_challenge_rejects_incorrect_answers() {
        let challenge = your_challenge();

        let invalid_inputs = [
            "",
            "wrong",
            "completely_different",
            "123456",
            "random_garbage",
        ];

        for input in invalid_inputs {
            assert!(
                !(challenge.check_answer)(input),
                "Should reject: '{}'", input
            );
        }
    }

    #[test]
    fn test_challenge_handles_edge_cases() {
        let challenge = your_challenge();

        // Edge cases that shouldn't crash
        let edge_cases = [
            "\n\r\t",                    // Whitespace only
            "a".repeat(1000).as_str(),   // Very long input
            "🎃👻🔥",                    // Unicode/emojis
            "'; DROP TABLE users; --",   // Injection attempts
            "\0null\0byte\0",           // Null bytes
        ];

        for input in edge_cases {
            // Should not panic, regardless of result
            let _ = (challenge.check_answer)(input);
        }
    }
    ```

2. **Property Tests** (using existing infrastructure)

    ```rust
    // The framework automatically tests your challenge with:
    // - Random strings
    // - Long inputs
    // - Special characters
    // - Unicode input
    // - Empty/whitespace
    ```

3. **Integration Tests**

    ```rust
    #[test]
    fn test_challenge_integrates_properly() {
        let challenges = get_challenges_for_level(your_level);

        // Should be included in level
        assert!(challenges.iter().any(|c| c.id == "your_challenge_id"));

        // Should have unique ID
        let ids: Vec<_> = challenges.iter().map(|c| c.id).collect();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(ids.len(), unique_ids.len(), "Duplicate challenge ID found");
    }
    ```

### Testing Checklist

-   [ ] **Validation function never panics** (any input)
-   [ ] **Accepts all correct answer formats** (case, spacing, punctuation)
-   [ ] **Rejects clearly incorrect answers**
-   [ ] **Handles edge cases gracefully** (empty, long, unicode, special chars)
-   [ ] **Properties are reasonable** (XP, sanity, level appropriate)
-   [ ] **Hints are progressive and helpful**
-   [ ] **ID is unique across all challenges**
-   [ ] **Category is appropriate**
-   [ ] **Educational objective is clear**
-   [ ] **Horror theme integration works**

---

## Horror Theme Integration

### Narrative Elements

Every challenge should contribute to the overarching ghost story:

1. **Discovery Context**

    - Challenges should feel like uncovering clues
    - Use atmospheric descriptions
    - Reference the ghost's past or victims

2. **Atmospheric Language**

    ```rust
    // Good: atmospheric and educational
    description: "You discover a corrupted file on the ghost's ancient computer. The data appears to be encoded, but the encryption method is unclear. As you examine the strange symbols, you realize this might be the key to understanding what happened here..."

    // Bad: purely technical
    description: "Decode this Base64 string to find the flag."
    ```

3. **Progressive Revelation**
    - Early challenges: Basic discoveries
    - Mid challenges: Understanding the tragedy
    - Late challenges: Confronting the truth
    - Final challenges: Becoming the next ghost

### Horror Writing Guidelines

1. **Tone**: Dark, mysterious, but not over-the-top
2. **Language**: Sophisticated horror vocabulary
3. **Pacing**: Build tension gradually
4. **Imagery**: Focus on digital decay, corrupted data, ghostly presence
5. **Themes**: Technology as conduit for spirits, digital haunting

### Example Horror Descriptions

```rust
// Encoding Challenge
description: "The ghost's digital traces are corrupted, transformed into indecipherable symbols. You find fragments of what might once have been readable text, now twisted into an alien encoding. The terminal flickers as you examine the data - something doesn't want you to understand..."

// Cryptography Challenge
description: "A encrypted message pulses on the screen, the letters shifting and writhing as if alive. The cipher seems ancient despite the digital medium, as if the ghost learned its secrets from darker times. Your sanity wavers as you realize - some knowledge comes with a price..."

// Web Security Challenge
description: "The ghost's web presence lingers in forgotten databases, protected by authentication that was never meant to keep spirits at bay. As you probe the digital defenses, you sense you're not the first to try breaking in. The question is: what happened to those who succeeded?"
```

---

## Difficulty Balancing

### XP Reward Guidelines

| Level | Challenge Type      | XP Range | Rationale                 |
| ----- | ------------------- | -------- | ------------------------- |
| 0     | Simple encoding     | 25-50    | Basic pattern recognition |
| 0     | Basic concepts      | 50-75    | Fundamental learning      |
| 1     | Intermediate skills | 75-125   | Multi-step solutions      |
| 2     | Advanced techniques | 100-175  | Complex analysis          |
| 3+    | Expert level        | 150-250  | Research-level problems   |

### Sanity Cost Guidelines

| Difficulty | Sanity Cost | Player Experience    |
| ---------- | ----------- | -------------------- |
| Tutorial   | 2-5         | Gentle introduction  |
| Easy       | 5-8         | Comfortable learning |
| Medium     | 8-12        | Moderate challenge   |
| Hard       | 12-18       | Significant effort   |
| Expert     | 15-25       | Major undertaking    |

### Balancing Formula

```rust
// Recommended balancing approach
fn calculate_rewards(difficulty: u32, complexity: u32, novelty: u32) -> (u32, u32) {
    let base_xp = match difficulty {
        0 => 50,
        1 => 100,
        2 => 150,
        3 => 200,
        _ => 250,
    };

    let xp_modifier = (complexity + novelty) as f32 * 0.25;
    let xp_reward = (base_xp as f32 * (1.0 + xp_modifier)) as u32;

    let sanity_cost = (difficulty + 1) * 3 + complexity * 2;

    (xp_reward, sanity_cost)
}
```

### Progression Guidelines

1. **Level 0**: Should be completable by absolute beginners
2. **Level 1**: Requires basic understanding from Level 0
3. **Level 2**: Combines multiple concepts, tool usage
4. **Level 3+**: Advanced techniques, research required

---

## Educational Objectives

### Learning Outcome Framework

Every challenge should have clear learning objectives:

```rust
// Document in challenge comments
/*
LEARNING OBJECTIVES:
- Primary: Understanding Base64 encoding/decoding
- Secondary: Recognizing encoded data patterns
- Tertiary: Tool usage for data transformation

REAL-WORLD APPLICATIONS:
- Email attachment encoding
- URL parameter obfuscation
- API response data format
- Configuration file storage

SKILLS DEVELOPED:
- Pattern recognition
- Tool selection
- Data format analysis
- Systematic problem solving
*/
```

### Knowledge Progression Map

```
Level 0: Recognition & Basic Tools
├── Encoding (Base64, Hex, ASCII)
├── File Operations (find, grep, ls)
├── Network Basics (ports, services)
└── Simple Patterns (ROT13, substitution)

Level 1: Analysis & Combination
├── Cryptography (Caesar, frequency analysis)
├── Web Basics (HTTP, forms, parameters)
├── System Commands (injection, traversal)
└── Multi-step Solutions

Level 2: Exploitation & Investigation
├── Web Security (SQLi, XSS, headers)
├── Mobile Security (deep links, APIs)
├── Network Analysis (DNS, traffic)
└── Forensic Techniques

Level 3+: Advanced & Specialized
├── Binary Exploitation (overflow concepts)
├── Reverse Engineering (decompilation, XOR)
├── Advanced Crypto (modern algorithms)
└── Protocol Analysis (custom formats)
```

### Assessment Criteria

1. **Accessibility**: Can beginners learn this with hints?
2. **Authenticity**: Is this a real-world technique?
3. **Progression**: Does it build on previous knowledge?
4. **Scope**: Is the complexity appropriate for the level?
5. **Utility**: Will players use this knowledge elsewhere?

---

## Code Examples

### Complete Challenge Implementation

```rust
// File: src/challenges.rs
// Add this function and call it in get_challenges_for_level()

fn steganography_lsb() -> Challenge {
    Challenge {
        id: "stego_lsb_basic",
        title: "Hidden in Plain Sight",
        description: r#"You discover a peculiar pattern in what seems like random binary data. The ghost's digital footprint includes files that appear normal, but something feels off about the bit patterns. In steganography, information can be hidden in the least significant bits of data - invisible to casual observation, but detectable to those who know where to look."#,
        prompt: r#"Examine this binary sequence and extract the hidden message:
01001000 01100101 01101100 01101100 01101111
The message is hidden in the least significant bit (rightmost bit) of each byte.
What word is spelled out?"#,
        level: 1,
        xp_reward: 90,
        sanity_cost: 10,
        check_answer: |answer| {
            let normalized = answer.to_lowercase().trim();
            normalized == "hello" || normalized == "h-e-l-l-o"
        },
        hints: vec![
            "LSB steganography hides data in the least significant (rightmost) bit of each byte".to_string(),
            "Extract the rightmost bit from each byte: 0,1,0,0,1 then convert to ASCII".to_string(),
            "The bits spell out 'HELLO' when converted from binary to ASCII characters".to_string(),
        ],
        category: ChallengeCategory::Steganography,
    }
}

// Add to get_challenges_for_level() Level 1 section:
1 => vec![
    // ... existing challenges
    steganography_lsb(),
],
```

### Advanced Validation Pattern

```rust
// For challenges with multiple valid answer formats
fn advanced_validation_example() -> Challenge {
    Challenge {
        id: "web_jwt_decode",
        title: "The Broken Token",
        description: "You intercept a JWT token from the ghost's authenticated session...",
        prompt: "Decode this JWT header and find the algorithm: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
        level: 2,
        xp_reward: 125,
        sanity_cost: 12,
        check_answer: |answer| {
            // Normalize input thoroughly
            let cleaned = answer
                .to_lowercase()
                .trim()
                .replace([' ', '-', '_', '.', ',', ':', ';'], "");

            // Accept various formats
            let valid_patterns = [
                "hs256",           // Just the algorithm
                "hmacsha256",      // Full name
                "sha256",          // Shortened
                "hs256jwt",        // With context
                "algorithmhs256",  // With label
            ];

            // Flexible matching
            valid_patterns.iter().any(|&pattern| cleaned.contains(pattern))
        },
        hints: vec![
            "JWT tokens have three parts separated by dots: header.payload.signature".to_string(),
            "The header is Base64 encoded JSON containing algorithm information".to_string(),
            "Decode the first part to reveal: {\"alg\":\"HS256\",\"typ\":\"JWT\"}".to_string(),
        ],
        category: ChallengeCategory::WebSecurity,
    }
}
```

### Testing Template

```rust
#[cfg(test)]
mod your_challenge_tests {
    use super::*;

    #[test]
    fn test_challenge_basic_functionality() {
        let challenge = your_challenge_function();

        // Basic properties
        assert!(!challenge.id.is_empty());
        assert!(!challenge.title.is_empty());
        assert!(challenge.hints.len() >= 2);
        assert!(challenge.xp_reward > 0);
        assert!(challenge.sanity_cost > 0);
    }

    #[test]
    fn test_validation_accepts_correct_formats() {
        let challenge = your_challenge_function();

        let correct_answers = [
            "expected_answer",
            "Expected_Answer",
            " expected_answer ",
            "expected-answer",
            "expected.answer",
        ];

        for answer in correct_answers {
            assert!(
                (challenge.check_answer)(answer),
                "Should accept: '{}'", answer
            );
        }
    }

    #[test]
    fn test_validation_rejects_incorrect() {
        let challenge = your_challenge_function();

        let wrong_answers = [
            "",
            "wrong_answer",
            "123456",
            "random_text",
            "completely_different",
        ];

        for answer in wrong_answers {
            assert!(
                !(challenge.check_answer)(answer),
                "Should reject: '{}'", answer
            );
        }
    }

    #[test]
    fn test_robustness_against_edge_cases() {
        let challenge = your_challenge_function();

        // These should not panic
        let _ = (challenge.check_answer)("🎃👻");
        let _ = (challenge.check_answer)(&"x".repeat(10000));
        let _ = (challenge.check_answer)("\n\r\t");
        let _ = (challenge.check_answer)("'; DROP TABLE users; --");
    }
}
```

---

## Best Practices

### Code Quality

1. **Function Naming**

    ```rust
    // Good: Descriptive and consistent
    fn crypto_rsa_basic() -> Challenge { }
    fn web_xss_reflected() -> Challenge { }
    fn mobile_deeplink_hijack() -> Challenge { }

    // Bad: Unclear or inconsistent
    fn challenge1() -> Challenge { }
    fn the_crypto_one() -> Challenge { }
    fn WebChallenge() -> Challenge { }
    ```

2. **Input Validation Robustness**

    ```rust
    // Good: Comprehensive normalization
    |answer| {
        let normalized = answer
            .to_lowercase()
            .trim()
            .replace([' ', '-', '_', '.'], "");
        normalized == "expected" || normalized.contains("partial")
    }

    // Bad: Fragile validation
    |answer| answer == "exact_match_only"
    ```

3. **Hint Progression**

    ```rust
    // Good: Educational progression
    hints: vec![
        "This is a Caesar cipher - each letter is shifted by a fixed amount".to_string(),
        "The shift appears to be 13 positions (ROT-13)".to_string(),
        "Try decoding: A→N, B→O, C→P, etc.".to_string(),
    ],

    // Bad: Non-progressive or spoiler hints
    hints: vec![
        "It's encrypted".to_string(),
        "Use ROT-13".to_string(),
        "The answer is 'HELLO'".to_string(), // Too direct!
    ],
    ```

### Performance Considerations

1. **Efficient Validation**

    ```rust
    // Good: Early returns, minimal processing
    |answer| {
        if answer.is_empty() { return false; }
        let cleaned = answer.to_lowercase().trim();
        cleaned == "fast_check" || cleaned.starts_with("prefix")
    }

    // Bad: Expensive operations
    |answer| {
        // Avoid regex, complex parsing, or external calls
        let regex = Regex::new(r"complex_pattern").unwrap();
        regex.is_match(answer)
    }
    ```

2. **Memory Efficiency**

    ```rust
    // Good: Static strings, minimal allocation
    description: "Static string literal",

    // Bad: Dynamic allocation in challenge definition
    description: &format!("Dynamic string {}", variable), // Don't do this
    ```

### Documentation Standards

1. **Challenge Comments**

    ```rust
    /*
    CHALLENGE: crypto_substitution_basic
    CATEGORY: Cryptography
    LEVEL: 1
    CONCEPTS: Substitution cipher, frequency analysis
    TOOLS: Manual analysis, cipher tools
    REAL_WORLD: Historical cryptography, puzzle solving
    PREREQUISITES: Basic alphabet manipulation
    FOLLOW_UP: crypto_substitution_advanced, crypto_frequency
    */
    fn crypto_substitution_basic() -> Challenge {
        // Implementation
    }
    ```

2. **Educational Context**

    ```rust
    description: r#"
    [NARRATIVE CONTEXT]
    You discover an encrypted diary entry from the ghost's past...

    [EDUCATIONAL CONTEXT]
    Substitution ciphers replace each letter with another letter consistently.
    This was one of the earliest forms of cryptography, used by Julius Caesar
    and many others throughout history.

    [TECHNICAL CONTEXT]
    The cipher uses a simple alphabetic substitution where A→D, B→E, etc.
    "#,
    ```

---

## Common Pitfalls

### Design Pitfalls

1. **Too Narrow Validation**

    ```rust
    // Problem: Only accepts one exact format
    |answer| answer == "flag{exact_format}"

    // Solution: Accept reasonable variations
    |answer| {
        let clean = answer.to_lowercase().replace(['_', '-', ' '], "");
        clean.contains("expected") || clean == "alternate_format"
    }
    ```

2. **Inappropriate Difficulty**

    ```rust
    // Problem: Level 0 challenge requiring advanced skills
    Challenge {
        level: 0,  // Beginner level
        prompt: "Reverse engineer this x86 assembly code...", // Expert task!
    }

    // Solution: Match complexity to level
    Challenge {
        level: 0,
        prompt: "Decode this simple Base64 string: aGVsbG8=", // Appropriate
    }
    ```

3. **Poor Horror Integration**

    ```rust
    // Problem: Generic CTF description
    description: "Here's a crypto challenge. Solve it."

    // Solution: Atmospheric narrative
    description: "The ghost's encrypted diary pulses with malevolent energy..."
    ```

### Technical Pitfalls

1. **Validation Function Panics**

    ```rust
    // Problem: Can crash on bad input
    |answer| {
        let number: u32 = answer.parse().unwrap(); // Panics on non-numbers!
        number == 42
    }

    // Solution: Handle errors gracefully
    |answer| {
        if let Ok(number) = answer.trim().parse::<u32>() {
            number == 42
        } else {
            false
        }
    }
    ```

2. **Case Sensitivity Issues**

    ```rust
    // Problem: Rejects valid answers due to case
    |answer| answer == "Hello World" // Rejects "hello world"

    // Solution: Normalize case
    |answer| answer.to_lowercase() == "hello world"
    ```

3. **Whitespace Issues**

    ```rust
    // Problem: Sensitive to spacing
    |answer| answer == "hello world" // Rejects " hello world "

    // Solution: Trim whitespace
    |answer| answer.trim().to_lowercase() == "hello world"
    ```

### Educational Pitfalls

1. **Assuming Prior Knowledge**

    ```rust
    // Problem: Level 0 assuming advanced knowledge
    hints: vec!["Use Ghidra to decompile the binary".to_string()],

    // Solution: Progressive learning
    hints: vec![
        "Binary files contain machine code instructions".to_string(),
        "You can use tools like 'strings' to find readable text".to_string(),
        "Look for the hidden message in the binary data".to_string(),
    ],
    ```

2. **Non-Educational Challenges**

    ```rust
    // Problem: Puzzle without learning value
    prompt: "What's my favorite color?", // No educational value

    // Solution: Teach real concepts
    prompt: "What HTTP status code indicates 'Not Found'?", // Teaches web fundamentals
    ```

### Balancing Pitfalls

1. **Reward Mismatch**

    ```rust
    // Problem: Trivial task with huge reward
    Challenge {
        prompt: "What is 1+1?",
        xp_reward: 500, // Way too much!
        sanity_cost: 1,
    }

    // Solution: Match rewards to difficulty
    Challenge {
        prompt: "Decode this complex cipher...",
        xp_reward: 150, // Appropriate for difficulty
        sanity_cost: 15,
    }
    ```

---

## Review Checklist

### Pre-Implementation Review

**Design Phase:**

-   [ ] **Educational objective clearly defined**
-   [ ] **Difficulty appropriate for target level**
-   [ ] **Horror theme integration planned**
-   [ ] **Real-world relevance confirmed**
-   [ ] **Prerequisites identified**
-   [ ] **Follow-up challenges considered**

**Technical Phase:**

-   [ ] **Unique ID selected (snake_case)**
-   [ ] **Title is descriptive and engaging**
-   [ ] **Category assignment is correct**
-   [ ] **XP/Sanity costs are balanced**
-   [ ] **Validation logic designed**
-   [ ] **Multiple answer formats considered**

### Implementation Review

**Code Quality:**

-   [ ] **Function follows naming conventions**
-   [ ] **Challenge struct properly formatted**
-   [ ] **All required fields populated**
-   [ ] **No hardcoded magic numbers**
-   [ ] **Efficient validation function**
-   [ ] **Proper error handling**

**Validation Robustness:**

-   [ ] **Accepts all reasonable correct formats**
-   [ ] **Case-insensitive where appropriate**
-   [ ] **Whitespace tolerant**
-   [ ] **Handles edge cases without panicking**
-   [ ] **Rejects clearly incorrect answers**
-   [ ] **No false positives on common wrong answers**

**Educational Quality:**

-   [ ] **Hint progression is logical**
-   [ ] **Concepts are accurately explained**
-   [ ] **Real-world context provided**
-   [ ] **Learning objectives are clear**
-   [ ] **Difficulty matches level expectations**

### Testing Review

**Test Coverage:**

-   [ ] **Unit tests for validation function**
-   [ ] **Edge case testing (empty, long, unicode)**
-   [ ] **Integration test for challenge inclusion**
-   [ ] **Property tests pass (automatic)**
-   [ ] **Manual testing with real users**

**Quality Assurance:**

-   [ ] **Horror narrative fits theme**
-   [ ] **No spelling or grammar errors**
-   [ ] **Technical accuracy verified**
-   [ ] **Challenge is solvable as intended**
-   [ ] **Hints are helpful but not spoilers**

### Final Review

**Integration:**

-   [ ] **Added to appropriate level**
-   [ ] **ID uniqueness verified**
-   [ ] **Category enum updated if needed**
-   [ ] **Documentation updated**
-   [ ] **Tests pass in CI**

**Polish:**

-   [ ] **Final proofread of all text**
-   [ ] **Balancing verified against similar challenges**
-   [ ] **User experience tested**
-   [ ] **Ready for production**

---

## Getting Help

### Resources

**Documentation:**

-   [API Documentation](API_DOCUMENTATION.md) - Technical implementation details
-   [Contributing Guide](../CONTRIBUTING.md) - General contribution workflow
-   [Roadmap](ROADMAP.md) - Future challenge ideas and priorities

**Code References:**

-   `src/challenges.rs` - All existing challenges for reference
-   `src/state.rs` - Game state management
-   `src/ui.rs` - User interface functions

**Community:**

-   [GitHub Discussions](https://github.com/and3rn3t/hack/discussions) - Design questions
-   [GitHub Issues](https://github.com/and3rn3t/hack/issues) - Bug reports, feature requests
-   [Challenge Proposal Template](../.github/ISSUE_TEMPLATE/challenge_proposal.md) - Structured proposals

### Getting Feedback

1. **Open a Discussion** for design questions
2. **Create a Draft PR** for early code review
3. **Use the Challenge Proposal Template** for formal proposals
4. **Test with Real Users** before finalizing

### Contributing Your Challenge

1. Follow this design guide
2. Implement with tests
3. Open a pull request
4. Respond to review feedback
5. Celebrate when merged! 🎉

---

**Happy challenge creating! Remember: Every great hack starts with curiosity and ends with knowledge.** 👻

_"In the Ghost Protocol, knowledge is the only currency that survives..."_
