# v1.5.0 Content Expansion - Planning Document

**Version**: v1.5.0 (Next Release)
**Target Date**: Q1 2026
**Status**: 🔵 Planning Phase
**Previous Version**: v1.2.0

---

## 📋 **VERSION OVERVIEW**

v1.5.0 focuses on **Content Expansion** with three new challenge categories:

1. **Steganography** (5+ challenges) - Hidden data in images, audio, and files
2. **Malware Analysis** (3+ challenges) - Behavioral analysis and obfuscation
3. **IoT Security** (3+ challenges) - Smart device vulnerabilities

**Target Total**: 36 → 47+ challenges (11 new challenges minimum)

---

## 🎯 **PRIMARY GOALS**

### **Educational Objectives**

- Teach data hiding and discovery techniques
- Introduce malware analysis fundamentals
- Raise awareness of IoT security risks
- Progressive difficulty within each category

### **Technical Objectives**

- Maintain <300KB bundle size with new challenges
- Keep 90%+ test coverage
- Ensure cross-platform compatibility
- Support lazy loading for challenge categories

### **User Experience Objectives**

- Engaging, story-driven challenge narratives
- Clear learning progression
- Practical, real-world scenarios
- Balanced difficulty and sanity costs

---

## 🕵️ **CATEGORY 1: STEGANOGRAPHY CHALLENGES**

**Target**: 5+ challenges at Level 2
**Theme**: "Hidden in Plain Sight"
**Learning Path**: Basic → Intermediate → Advanced techniques

### **Challenge 1: LSB Steganography** (`steg_lsb_basics`)

**Difficulty**: Level 2 (Beginner Steganography)
**XP**: 110 | **Sanity**: 10

**Concept**: Least Significant Bit (LSB) manipulation in images

**Scenario**:

> You discover a seemingly innocent profile picture from a hacker's social media. The image metadata shows unusual file size for its resolution. Analysis reveals binary data hidden in the least significant bits of pixel values.

**Learning Points**:

- LSB steganography concept
- Binary data extraction from images
- File size anomalies as detection method
- Common steganography tools (steghide, stegsolve)

**Implementation Approach**:

```rust
// Provide base64-encoded "image data" with embedded message
// User must extract LSB pattern to reveal hidden text
Challenge::new_legacy(
    "steg_lsb_basics",
    2,
    "The Hidden Pixel",
    ChallengeCategory::Steganography,
    "A hacker's profile picture contains more than meets the eye...",
    "Extract the hidden message from the LSB data: [binary string provided]",
    "hidden message", // or decoded result
    vec![
        "LSB stands for Least Significant Bit",
        "Read every 8th bit to extract ASCII characters",
        "The pattern reveals: 01001000...",
    ],
    10,
    110,
)
```

---

### **Challenge 2: Image Metadata** (`steg_exif_data`)

**Difficulty**: Level 2
**XP**: 110 | **Sanity**: 8

**Concept**: Extracting hidden information from EXIF metadata

**Scenario**:

> A leaked document photo contains GPS coordinates and camera details in its EXIF data. The photographer's identity and location can be determined from the metadata.

**Learning Points**:

- EXIF data structure
- GPS coordinate extraction
- Camera fingerprinting
- Metadata scrubbing importance

**Solution Format**: GPS coordinates or hidden comment field

---

### **Challenge 3: Audio Steganography** (`steg_audio_spectrum`)

**Difficulty**: Level 2
**XP**: 120 | **Sanity**: 12

**Concept**: Hidden messages in audio spectrograms

**Scenario**:

> An intercepted audio file of static noise contains no audible message. When viewed as a spectrogram, the frequency spectrum reveals text encoded in the visual pattern.

**Learning Points**:

- Spectrogram analysis
- Frequency domain steganography
- Visual encoding in audio
- Tools: Audacity, Sonic Visualizer

**Implementation**: Provide description of spectrogram pattern, user decodes

---

### **Challenge 4: Whitespace Steganography** (`steg_whitespace`)

**Difficulty**: Level 2
**XP**: 115 | **Sanity**: 10

**Concept**: Hidden data in whitespace characters (spaces vs tabs)

**Scenario**:

> A seemingly normal text file's source code contains invisible messages. The pattern of spaces and tabs encodes binary data representing a hidden passphrase.

**Learning Points**:

- Whitespace encoding techniques
- Zero-width characters
- Source code steganography
- Detection methods

**Implementation**:

```rust
// Provide text with space/tab patterns representing binary
// "   \t  \t\t   \t" = binary message
let encoded_text = "Regular text\n\t  \t   \t\t  \nMore text";
// User must decode space=0, tab=1 pattern
```

---

### **Challenge 5: File Concatenation** (`steg_file_concat`)

**Difficulty**: Level 2
**XP**: 120 | **Sanity**: 12

**Concept**: Hidden files appended to legitimate files

**Scenario**:

> A PNG image file is larger than expected. Examining the raw bytes reveals a complete ZIP archive appended after the PNG end marker (IEND). Extract the hidden archive.

**Learning Points**:

- File format structures
- Magic number identification
- File carving techniques
- Polyglot files

**Solution**: Contents of hidden file or password from hidden readme

---

### **Bonus Challenge 6: DNA Steganography** (`steg_dna_encoding`)

**Difficulty**: Level 3 (Advanced)
**XP**: 150 | **Sanity**: 15

**Concept**: Encoding data in DNA sequences

**Scenario**:

> A genetic sequence contains non-biological patterns. Using DNA-to-text encoding (A=00, C=01, G=10, T=11), decode the hidden message in the genome data.

**Learning Points**:

- Biological data encoding
- Base-4 to binary conversion
- Cutting-edge steganography research
- DNA data storage concepts

---

## 🦠 **CATEGORY 2: MALWARE ANALYSIS CHALLENGES**

**Target**: 3+ challenges at Level 3
**Theme**: "Behavioral Analysis"
**Focus**: Conceptual understanding, not actual malware

### **Challenge 1: Simple Obfuscation** (`malware_obfuscation_basics`)

**Difficulty**: Level 3
**XP**: 140 | **Sanity**: 15

**Concept**: Deobfuscating simple encoded malware

**Scenario**:

> A suspicious script uses base64 encoding and string reversals to hide its true behavior. Decode the layered obfuscation to reveal the malicious command.

**Learning Points**:

- Obfuscation techniques (base64, hex, reverse)
- Deobfuscation strategies
- Static analysis approach
- Common obfuscation patterns

**Implementation**:

```rust
// Provide obfuscated code snippet
let obfuscated = "eval(atob('ZWNobyAibWFsd2FyZSI='.split('').reverse().join('')))";
// User must decode: reverse → base64 decode → reveal command
```

---

### **Challenge 2: Behavioral Indicators** (`malware_behavioral_analysis`)

**Difficulty**: Level 3
**XP**: 150 | **Sanity**: 18

**Concept**: Identifying malware from behavioral patterns

**Scenario**:

> A system shows suspicious behavior: registry modifications, outbound connections to unusual IPs, process injection attempts. Identify the malware family from the behavioral signature.

**Learning Points**:

- Behavioral analysis vs static analysis
- Indicators of Compromise (IoC)
- Process monitoring
- Network traffic analysis
- Malware family identification

**Solution**: Malware family name or primary behavior type

---

### **Challenge 3: Sandbox Detection** (`malware_sandbox_evasion`)

**Difficulty**: Level 3
**XP**: 150 | **Sanity**: 20

**Concept**: Understanding anti-analysis techniques

**Scenario**:

> Malware checks for virtualization, debuggers, and common sandbox usernames before executing. Identify which environment checks are being performed and how to bypass them.

**Learning Points**:

- VM detection techniques
- Debugger detection
- Sandbox evasion methods
- Time-based execution triggers
- Environment fingerprinting

---

## 🌐 **CATEGORY 3: IoT SECURITY CHALLENGES**

**Target**: 3+ challenges at Level 2-3
**Theme**: "The Internet of Vulnerable Things"

### **Challenge 1: Default Credentials** (`iot_default_creds`)

**Difficulty**: Level 2
**XP**: 110 | **Sanity**: 10

**Concept**: IoT devices with unchanged default passwords

**Scenario**:

> A smart home camera is accessible on the network. The manufacturer's default credentials are publicly documented but never changed by the owner.

**Learning Points**:

- Default credential risks
- IoT device discovery
- Manufacturer documentation
- Credential databases (DefaultCreds-cheat-sheet)

**Solution**: Default username/password combination

---

### **Challenge 2: Firmware Analysis** (`iot_firmware_strings`)

**Difficulty**: Level 3
**XP**: 140 | **Sanity**: 15

**Concept**: Extracting secrets from firmware

**Scenario**:

> A router's firmware update file contains hardcoded API keys and backdoor credentials. Use string analysis to discover the hidden secrets.

**Learning Points**:

- Firmware extraction basics
- String analysis tools
- Hardcoded credentials
- Firmware security best practices

**Implementation**: Provide hex dump or base64 firmware excerpt with hidden strings

---

### **Challenge 3: MQTT Protocol** (`iot_mqtt_interception`)

**Difficulty**: Level 3
**XP**: 150 | **Sanity**: 15

**Concept**: Insecure MQTT communication

**Scenario**:

> Smart home devices communicate via MQTT without encryption. Intercept and decode the plaintext messages to discover the door lock PIN code.

**Learning Points**:

- MQTT protocol basics
- Publish/subscribe model
- Lack of encryption in IoT
- Protocol analysis

---

## 📊 **CHALLENGE DISTRIBUTION AFTER v1.5.0**

| Level     | Current (v1.2.0) | After v1.5.0 | New     | Category Distribution           |
| --------- | ---------------- | ------------ | ------- | ------------------------------- |
| **0**     | 7                | 7            | 0       | Encoding basics                 |
| **1**     | 8                | 8            | 0       | Crypto, web basics              |
| **2**     | 15               | 21           | +6      | Stego (5), IoT (1)              |
| **3**     | 5                | 10           | +5      | Stego (1), Malware (3), IoT (2) |
| **4**     | 1                | 1            | 0       | Final challenge                 |
| **Total** | **36**           | **47**       | **+11** | Balanced expansion              |

---

## 🏗️ **IMPLEMENTATION PLAN**

### **Phase 1: Design & Documentation** (Weeks 1-2)

- ✅ Complete challenge specifications
- ✅ Write educational content for each challenge
- ✅ Design hint progression
- ✅ Create solution validation logic
- ✅ Review for balance and difficulty

### **Phase 2: Implementation** (Weeks 3-4)

- [ ] Add challenges to `challenges.rs`
- [ ] Implement validation functions
- [ ] Write comprehensive unit tests
- [ ] Test hint progression
- [ ] Verify XP/sanity balance

### **Phase 3: Testing & Polish** (Week 5)

- [ ] Integration testing
- [ ] Cross-platform verification
- [ ] Documentation updates
- [ ] Walkthrough creation
- [ ] Beta testing with users

### **Phase 4: Release** (Week 6)

- [ ] Update CHANGELOG.md
- [ ] Version bump to 1.5.0
- [ ] Deploy to production
- [ ] Announce new challenges
- [ ] Monitor user feedback

---

## 🎨 **NARRATIVE INTEGRATION**

### **Horror Theme Connections**

**Steganography Arc**: "What's Hidden in the Code"

> _"The ghost hides in plain sight, embedded in every pixel, every frequency, every space between words. Can you see what others cannot?"_

**Malware Analysis Arc**: "Infected Memories"

> _"The virus spreads through the system like a digital plague. To understand it, you must think like it. But be careful—the longer you analyze, the more it analyzes you."_

**IoT Security Arc**: "The Watching Devices"

> _"Every smart device is a window into someone's life. Every camera, every sensor, every connection—they all whisper secrets to those who know how to listen."_

---

## 🧪 **TESTING STRATEGY**

### **Challenge Validation Tests**

```rust
#[test]
fn test_steg_lsb_basics_solution() {
    let challenge = get_challenge_by_id("steg_lsb_basics");
    assert!(challenge.is_some());

    let challenge = challenge.unwrap();
    assert!(challenge.validate_answer("hidden message"));
    assert!(!challenge.validate_answer("wrong answer"));
}

#[test]
fn test_all_steg_challenges_have_hints() {
    let challenges = get_challenges_by_category(ChallengeCategory::Steganography);
    for challenge in challenges {
        assert!(challenge.hints.len() >= 3);
    }
}
```

### **Balance Testing**

- Verify XP rewards scale appropriately
- Ensure sanity costs are balanced
- Test hint progression is educational
- Validate difficulty curve

---

## 📚 **DOCUMENTATION REQUIREMENTS**

### **Files to Create/Update**

1. `docs/STEGANOGRAPHY_GUIDE.md` - Steganography challenge walkthrough
2. `docs/MALWARE_ANALYSIS_GUIDE.md` - Malware challenge guide
3. `docs/IOT_SECURITY_GUIDE.md` - IoT security concepts
4. `CHANGELOG.md` - Version 1.5.0 changes
5. `docs/V1_5_0_SUMMARY.md` - Implementation summary
6. `README.md` - Update challenge count

### **Learning Resources**

- Steganography tools and techniques
- Malware analysis basics
- IoT security fundamentals
- Recommended practice platforms

---

## 🎯 **SUCCESS METRICS**

### **Completion Criteria**

- [ ] 11+ new challenges implemented
- [ ] All tests passing (90%+ coverage maintained)
- [ ] Bundle size <300KB
- [ ] Educational content reviewed
- [ ] User testing completed
- [ ] Documentation complete

### **Quality Metrics**

- Challenge completion rate >60%
- Average hint usage <2 per challenge
- User satisfaction >4.5/5
- Educational effectiveness >85%

---

## 🚀 **RELEASE CHECKLIST**

### **Pre-Release**

- [ ] All challenges implemented and tested
- [ ] Documentation complete
- [ ] Version numbers updated
- [ ] CHANGELOG.md updated
- [ ] Beta testing completed
- [ ] Performance benchmarks pass

### **Release**

- [ ] Merge to main branch
- [ ] Tag release v1.5.0
- [ ] Deploy to production
- [ ] Update live documentation
- [ ] Social media announcement

### **Post-Release**

- [ ] Monitor user feedback
- [ ] Track completion rates
- [ ] Address bug reports
- [ ] Plan v2.0.0 features

---

## 💡 **FUTURE CONSIDERATIONS**

### **Potential Enhancements for v2.0**

- Dynamic steganography challenges with randomized images
- Interactive malware sandbox simulation
- Real IoT device challenges (virtualized)
- Challenge difficulty variants
- Achievements for category mastery

### **Community Contributions**

- Open challenge submission system
- Community voting on new challenges
- User-created steganography puzzles
- Collaborative CTF events

---

**Status**: 🔵 Planning Complete - Ready for Implementation
**Next Step**: Begin Phase 2 Implementation
**Timeline**: 6 weeks to v1.5.0 release

_"Every file has secrets. Every byte tells a story. Learn to read between the lines."_
