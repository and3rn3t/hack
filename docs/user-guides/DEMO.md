# Game Demo & Screenshots

## Welcome Screen

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║     ████████╗██╗  ██╗███████╗    ██╗  ██╗ █████╗  ██████╗██╗  ██╗      ║
║     ╚══██╔══╝██║  ██║██╔════╝    ██║  ██║██╔══██╗██╔════╝██║ ██╔╝      ║
║        ██║   ███████║█████╗      ███████║███████║██║     █████╔╝       ║
║        ██║   ██╔══██║██╔══╝      ██╔══██║██╔══██║██║     ██╔═██╗       ║
║        ██║   ██║  ██║███████╗    ██║  ██║██║  ██║╚██████╗██║  ██╗      ║
║        ╚═╝   ╚═╝  ╚═╝╚══════╝    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝      ║
║                                                                           ║
║                    ═══ GHOST PROTOCOL INITIATED ═══                      ║
║                                                                           ║
║            A Horror-Themed Hacking Simulator & CTF Challenge             ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

## Horror Introduction

The game opens with an atmospheric horror narrative:

```
Welcome, [Your Name]...

You wake up in a dimly lit room. The air is thick with the smell of dust and decay.
A flickering monitor casts eerie shadows on the walls. As your eyes adjust to the 
darkness, you notice something unsettling...

The computer screen displays a message:

    "HELP ME... THEY'RE TRAPPED IN THE SYSTEM..."
    "FIND THE KEYS... UNLOCK THE SOULS..."
    "BUT BEWARE... THE DEEPER YOU GO, THE MORE THEY WATCH..."

Your hands tremble as you approach the keyboard. You have no choice but to dive into
this cursed system. Each challenge you solve might free another trapped soul...
or it might cost you your sanity.

The Ghost Protocol has begun.
```

## Level Screen

```
╔═══════════════════════════════════════════╗
║   Level 0: The Awakening   ║
╚═══════════════════════════════════════════╝

Sanity: [██████████] 100%

You begin your descent into the cursed system...
```

## Challenge Selection Menu

```
╔════════════════════════════════════════════════════════╗
║  Level 0 - Available Challenges                      ║
╚════════════════════════════════════════════════════════╝

Sanity: [██████████] 100%

Experience: 0 XP | Level: 0

Challenges:
  1. The First Message - ○ Available (50)
  2. Hidden Files - ○ Available (50)
  3. The Open Door - ○ Available (50)

Options:
  [1-3] - Select a challenge
  [stats] - View detailed statistics
  [save] - Save game
  [quit] - Quit game

Your choice: _
```

## Challenge Example

```
─────────────────────────────────────────────────────────────────────

🔒 CHALLENGE: The First Message

─────────────────────────────────────────────────────────────────────

The screen flickers and displays a corrupted message:

    "V2VsY29tZSB0byB0aGUgR2hvc3QgUHJvdG9jb2w="

This looks like Base64 encoding. Decode it to proceed.
(Hint: You can use online tools or the command 'base64 -d' on Linux)


Enter your answer (attempt 1/5) or 'hint' for help or 'skip': _
```

## Success Message

```
✓ SUCCESS: Challenge completed! +50 XP

The darkness recedes, if only for a moment...
```

## Stats View

```
╔════════════════════════════════════════════════════════╗
║                  Player Statistics                    ║
╚════════════════════════════════════════════════════════╝

Player: Hacker123
Current Level: 2
Experience: 325 XP
Sanity: 75%
Challenges Completed: 5
Secrets Discovered: 2

Completed Challenges:
  ✓ welcome
  ✓ file_discovery
  ✓ port_scan
  ✓ caesar_cipher
  ✓ sql_injection_basics

Discovered Secrets:
  🔍 The Hidden Message
  🔍 Developer's Note
```

## Features Demonstrated

### 1. Progressive Difficulty
- **Level 0**: Beginner challenges (Base64, hidden files, port scanning)
- **Level 1**: Intermediate challenges (Caesar cipher, SQL injection, hex decoding)
- **Level 2**: Web/Mobile challenges (HTTP headers, deep links)
- **Level 3+**: Advanced challenges (buffer overflow, reverse engineering)

### 2. Horror Atmosphere
- Glitch effects on text
- Ominous messages after completing challenges
- Sanity meter that decreases with each challenge
- Dark narrative that unfolds as you progress
- Twist ending revealing the true nature of the Ghost Protocol

### 3. Educational Content
Each challenge teaches real cybersecurity concepts:
- **Encoding**: Base64, Hexadecimal, ASCII
- **Cryptography**: Caesar cipher, XOR operations
- **Web Security**: HTTP headers, SQL injection
- **Mobile Security**: Deep linking, app analysis
- **Network**: Port scanning, service identification
- **Binary**: Buffer overflows, memory corruption
- **Reverse Engineering**: XOR logic, decompilation

### 4. Beginner-Friendly Features
- Multiple hints per challenge
- Progressive hint system (easier hints first)
- Ability to skip challenges
- Save/load system
- Clear instructions and examples
- Educational descriptions

### 5. Unique Horror Twist
Unlike traditional CTF platforms, this combines:
- Horror storytelling with technical challenges
- Sanity mechanic (psychological cost of hacking)
- Atmospheric terminal interface
- Dark narrative with twist ending
- Game-over condition (sanity reaches 0)

## Example Playthrough Timeline

1. **Start** → Enter name → See introduction
2. **Level 0** → Complete 3 beginner challenges (Base64, Files, Ports)
3. **Level 1** → Learn cryptography (Caesar cipher, Hex)
4. **Level 2** → Explore web/mobile security
5. **Level 3+** → Face advanced challenges
6. **Ending** → Discover the twist: You've become the next ghost in the machine

## Terminal Aesthetics

The game uses:
- ✅ Colored text (Red for errors, Green for success, Yellow for warnings)
- ✅ ASCII art banners
- ✅ Unicode symbols (✓, ○, 🔒, 🔍, ⚠, etc.)
- ✅ Progress bars for sanity meter
- ✅ Box drawing characters for UI elements
- ✅ Glitch effects for horror atmosphere

## Running the Demo

```bash
# Clone and run
git clone https://github.com/and3rn3t/hack.git
cd hack
cargo run --release

# Follow on-screen prompts
# Try solving the challenges
# Experience the horror narrative
```

## Tips for First-Time Players

1. **Start with Level 0** - These are beginner-friendly
2. **Use hints liberally** - They're there to help you learn
3. **Read challenge descriptions carefully** - Clues are hidden in the text
4. **Manage your sanity** - Do easier challenges first
5. **Save your progress** - Use the 'save' command frequently
6. **Learn from failures** - Wrong answers teach you what doesn't work
7. **Enjoy the story** - Pay attention to the narrative

## Educational Value

This game is perfect for:
- 🎓 Computer science students learning security
- 🔐 Aspiring penetration testers
- 💻 Developers interested in cybersecurity
- 🎮 Gamers who want to learn hacking
- 📚 Self-learners exploring CTF challenges
- 👥 Study groups practicing together

---

*"The Ghost Protocol never ends... it only waits for the next hacker..."*
