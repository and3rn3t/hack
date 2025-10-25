# The Hack: Ghost Protocol 👻

[![CI/CD](https://github.com/and3rn3t/hack/workflows/CI%2FCD/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/ci.yml)
[![Quick Check](https://github.com/and3rn3t/hack/workflows/Quick%20Check/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/quick-check.yml)
[![Test Suite](https://github.com/and3rn3t/hack/workflows/Test%20Suite/badge.svg)](https://github.com/and3rn3t/hack/actions/workflows/test-suite.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A horror-themed hacking simulator and Capture The Flag (CTF) challenge game designed for both beginners and enthusiasts. Experience a progressive difficulty system while unraveling a dark narrative in a terminal-based horror adventure.

## 🎮 Features

### Core Experience

- **Horror Theme**: A chilling narrative that unfolds as you progress through the challenges
- **Progressive Difficulty**: Start with basic challenges and advance to complex hacking scenarios
- **Beginner-Friendly**: Designed for non-techies with hints and educational content
- **Sanity System**: A unique horror twist - lose sanity as you delve deeper

### Challenge System

- **26+ CTF Challenges**: Multiple categories including:
  - Basic encoding (Base64, Hex, Caesar cipher)
  - File system exploration
  - Network analysis
  - SQL injection
  - Cryptography
  - Web/Mobile security
  - Buffer overflow concepts
  - Reverse engineering

### v1.2.0 Enhanced Features ✨

- **Challenge Difficulty Variants**: Beginner, Standard, Advanced, and Expert modes for existing challenges
- **Adaptive Difficulty**: Smart difficulty selection based on your performance
- **Dynamic Practice Mode**: Randomly generated challenges for skill building
- **Advanced Settings**: Comprehensive customization (difficulty scaling, hint verbosity, themes, accessibility)
- **User Command Aliases**: Create shortcuts for frequently used commands
- **Multiple Save Slots**: 5 save slots with export/import functionality
- **Progress Analytics**: Detailed performance tracking and learning insights
- **Enhanced UI**: Improved menus, settings, and user experience

### Technical Features

- **Cross-Platform**: Works on Windows, Linux, macOS, and Web browsers
- **Save/Load System**: Multiple save slots with cloud export/import
- **Terminal UI**: Full terminal interface with colored output and ASCII art
- **Advanced Command System**: Tab completion, command history, and custom aliases

## 🎮 Play Online

**[🌐 Play Now at hack.andernet.dev →](https://hack.andernet.dev)**

Experience the full game directly in your web browser - no installation required! The web version includes all features plus:

- Cross-device save file export/import
- Enhanced atmospheric horror effects
- Interactive tutorial system
- Advanced progress tracking
- Mobile-responsive design

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)
- Terminal that supports ANSI color codes
  - **Windows**: Windows Terminal (recommended), PowerShell 7+
  - **Linux**: Any modern terminal (GNOME Terminal, Konsole, Alacritty)
  - **macOS**: iTerm2 or Terminal.app

**Terminal Setup:** For optimal experience, see [Terminal Setup Guide](docs/TERMINAL_SETUP.md)

## Quick Start

```bash
# Build the project
cargo build --release

# Run the game
cargo run --release

# Run tests
cargo test
```

## 🎯 How to Play

1. **Start the game**: Run `cargo run` from the project directory
2. **Enter your name**: You'll be prompted to enter a name for your character
3. **Read the story**: Immerse yourself in the horror narrative
4. **Select challenges**: Choose from available challenges at each level
5. **Solve puzzles**: Use your hacking knowledge (or learn as you go!)
6. **Progress through levels**: Complete challenges to earn XP and advance
7. **Manage your sanity**: Watch your sanity meter - reaching 0 means game over!

### Game Commands

- **[1-N]**: Select a challenge by number
- **stats**: View your detailed statistics
- **save**: Manually save your game (auto-saves after each challenge)
- **quit**: Exit the game (progress is saved)
- **hint**: Get a hint during a challenge (can be used multiple times)
- **skip**: Skip a challenge and try it later

### Navigation Features

- **Arrow Keys**: Use ↑/↓ to navigate through command history
- **Left/Right Arrows**: Move cursor within your current input
- **Home/End**: Jump to beginning/end of input
- **Backspace/Delete**: Edit your input
- **Tab**: Command completion (coming soon)

## 📚 Challenge Categories

### Level 0: The Awakening (Beginner)

- Base64 encoding/decoding
- Hidden file discovery
- Basic network port analysis
- ROT13 cipher decoding
- Binary to ASCII conversion
- URL encoding/decoding

### Level 1: Whispers in the Code (Intermediate)

- Caesar cipher cryptography
- SQL injection basics
- Hexadecimal decoding
- JWT token vulnerabilities
- Path traversal attacks
- MD5 hash cracking
- Command injection

### Level 2: The Forgotten Server (Web/Mobile)

- HTTP header analysis
- Mobile deep link exploitation
- DNS tunneling detection
- Cross-Site Scripting (XSS)
- API key leakage and secret exposure
- Session hijacking
- CORS misconfigurations

### Level 3+: Advanced Territories

- Buffer overflow concepts
- Reverse engineering and XOR operations
- Format string vulnerabilities
- Race conditions
- Integer overflow exploits
- Advanced cryptography
- Final protocol challenges

## 🎓 Educational Value

This simulator teaches:

- **Encoding schemes**: Base64, Hexadecimal, ASCII
- **Cryptography**: Caesar cipher, XOR operations
- **Web security**: HTTP headers, SQL injection
- **Mobile security**: Deep linking, app analysis
- **Network basics**: Port scanning, service identification
- **Binary exploitation**: Buffer overflows (conceptual)
- **Reverse engineering**: XOR logic, decompilation basics

## 🔧 Technical Details

### Built With

- **Rust** - A modern, safe systems programming language
- **crossterm** - Cross-platform terminal manipulation
- **serde** - Serialization/deserialization for save system
- **rand** - Random number generation for game elements

### Architecture

```
src/
├── main.rs          # Entry point
├── game.rs          # Main game loop
├── challenges.rs    # Challenge definitions and logic
├── narrative.rs     # Story and horror elements
├── state.rs         # Game state management
└── ui.rs           # Terminal UI and rendering
```

## 🎨 Horror Theme Elements

- **ASCII art banners**: Atmospheric visual design
- **Glitch effects**: Corrupted text for immersion
- **Sanity meter**: Psychological horror mechanic
- **Dark narrative**: Unfolding ghost story
- **Ominous messages**: Environmental storytelling
- **Twist ending**: Satisfying horror conclusion

## 🛠️ Development

### For New Developers

Get started quickly with our comprehensive setup guide:

- 📖 **[Setup Guide](docs/SETUP.md)** - Complete development environment setup
- ⚙️ **[Configuration Guide](docs/CONFIGURATION.md)** - All configuration options explained
- 📋 **[Configuration Summary](docs/CONFIG_SUMMARY.md)** - Quick reference for all config files
- ✅ **[Setup Checklist](.github/CHECKLIST.md)** - Verify your environment is ready

### For Contributors

- 🤝 **[Contributing Guide](CONTRIBUTING.md)** - How to contribute to the project
- 🗺️ **[Roadmap](docs/ROADMAP.md)** - Future plans and innovations
- 🔒 **[Security Policy](SECURITY.md)** - Reporting security issues

### Quick Start

```bash
# Clone the repository
git clone https://github.com/and3rn3t/hack.git
cd hack

# Build and run
cargo build
cargo run

# Development workflow
cargo fmt              # Format code
cargo clippy          # Lint code
cargo test            # Run tests
```

### Running in Development Mode

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
./target/release/hack_simulator
```

### Project Structure

```
src/
├── main.rs          # Entry point
├── game.rs          # Main game loop
├── challenges.rs    # Challenge definitions and logic
├── narrative.rs     # Story and horror elements
├── state.rs         # Game state management
└── ui.rs           # Terminal UI and rendering
```

### Configuration Files

The project includes comprehensive configuration for:

- **Rust/Cargo**: `Cargo.toml`, `rustfmt.toml`, `clippy.toml`, `.cargo/config.toml`
- **Editor**: `.editorconfig`, `.vscode/` settings
- **CI/CD**: `.github/workflows/ci.yml`
- **Git**: `.gitignore`

See [Configuration Guide](docs/CONFIGURATION.md) for details.

### Testing

The game includes educational hints and progressive difficulty to ensure accessibility while maintaining depth for experienced players.

## 📱 Mobile Challenges

The game includes mobile security challenges that teach:

- Deep link vulnerabilities
- Mobile app security concepts
- Cross-platform security considerations

These challenges are accessible through the terminal interface and don't require actual mobile devices.

## 🎯 Unique Twist

Unlike traditional CTF challenges, **The Hack: Ghost Protocol** combines:

1. **Horror narrative**: Every challenge is part of a larger, unsettling story
2. **Sanity mechanic**: Your mental state affects gameplay
3. **Progressive revelation**: The truth unfolds as you advance
4. **Atmospheric presentation**: Terminal-based horror aesthetic
5. **Educational horror**: Learn real hacking concepts through a horror lens

## 🤝 Contributing

This is an educational project designed to teach cybersecurity concepts through gamification. Contributions that add new challenges, improve the narrative, or enhance the horror atmosphere are welcome!

## 📝 License

This project is created for educational purposes to teach cybersecurity and programming concepts.

## 🎮 Tips for Players

1. **Read carefully**: Challenge descriptions contain important clues
2. **Use hints**: Don't struggle alone - hints are there to help
3. **Take notes**: Some challenges reference earlier concepts
4. **Think like a hacker**: Question everything, look for patterns
5. **Manage sanity**: Complete easier challenges first to build up XP
6. **Explore**: Try different approaches to problems
7. **Learn**: Use external resources to understand concepts

## ⚠️ Disclaimer

This is a learning tool. The hacking techniques discussed are for educational purposes only. Always practice ethical hacking and never attempt unauthorized access to systems.

---

**Remember**: In the Ghost Protocol, every hack has a cost. How far will you go to free the trapped souls?

_"THE PROTOCOL NEVER ENDS... IT ONLY WAITS FOR THE NEXT HACKER..."_
