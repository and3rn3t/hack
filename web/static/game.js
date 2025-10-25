// The Hack: Ghost Protocol - Web Interface
// Integrates with WebAssembly backend

let gameEngine = null;
let gameState = null;
let wasmModule = null;
let terminal = null;
let currentTheme = "horror";
let currentChallenge = null;
let commandHistory = [];
let historyIndex = -1;
let achievements = [];
let audioContext = null;
let ambientAudio = null;

// Initialize the game
async function init() {
    try {
        // Update loading progress
        updateLoadingProgress(10);

        // Load WebAssembly module
        wasmModule = await import("./pkg/hack_simulator.js");
        await wasmModule.default();

        updateLoadingProgress(30);

        // Initialize game engine and state
        gameEngine = new wasmModule.WebGameEngine();
        gameState = new wasmModule.WebGameState();

        updateLoadingProgress(50);

        // Initialize terminal
        initializeTerminal();

        updateLoadingProgress(70);

        // Load saved game if available
        loadSavedGame();

        // Load achievements and initialize audio
        loadAchievements();
        initializeAudio();

        updateLoadingProgress(90);

        // Start the game
        startGame();

        updateLoadingProgress(100);

        // Hide loading screen
        setTimeout(() => {
            document.getElementById("loading").style.display = "none";
            document.getElementById("gameContainer").style.display = "flex";
        }, 500);
    } catch (error) {
        console.error("Failed to initialize game:", error);
        showError("Failed to load game. Please refresh the page.");
    }
}

function updateLoadingProgress(percent) {
    const progress = document.getElementById("loadingProgress");
    if (progress) {
        progress.style.width = `${percent}%`;
    }
}

function initializeTerminal() {
    // Create terminal instance
    terminal = new Terminal({
        theme: getTerminalTheme(),
        fontFamily: '"Courier New", "Monaco", "Menlo", monospace',
        fontSize: 14,
        lineHeight: 1.2,
        cursorBlink: true,
        cursorStyle: "block",
        allowProposedApi: true,
        convertEol: true,
        scrollback: 1000,
    });

    // Add fit addon
    const fitAddon = new FitAddon.FitAddon();
    terminal.loadAddon(fitAddon);

    // Mount terminal to DOM
    const terminalElement = document.getElementById("terminal");
    terminal.open(terminalElement);

    // Fit terminal to container
    fitAddon.fit();

    // Handle terminal input
    let currentInput = "";
    let cursorPosition = 0;

    terminal.onData((data) => {
        handleTerminalInput(data, currentInput, cursorPosition);
    });

    // Handle window resize
    globalThis.addEventListener("resize", () => {
        fitAddon.fit();
    });

    // Store references for input handling
    terminal._currentInput = "";
    terminal._cursorPosition = 0;
}

function handleTerminalInput(data, currentInput, cursorPosition) {
    const char = data;

    switch (char) {
        case "\r": // Enter
            processCommand(terminal._currentInput.trim());
            terminal._currentInput = "";
            terminal._cursorPosition = 0;
            break;

        case "\u007F": // Backspace
            if (terminal._cursorPosition > 0) {
                terminal._currentInput =
                    terminal._currentInput.slice(
                        0,
                        terminal._cursorPosition - 1
                    ) + terminal._currentInput.slice(terminal._cursorPosition);
                terminal._cursorPosition--;
                updateInputLine();
            }
            break;

        case "\u001b[A": // Up arrow
            navigateHistory(-1);
            break;

        case "\u001b[B": // Down arrow
            navigateHistory(1);
            break;

        case "\u001b[C": // Right arrow
            if (terminal._cursorPosition < terminal._currentInput.length) {
                terminal._cursorPosition++;
                updateInputLine();
            }
            break;

        case "\u001b[D": // Left arrow
            if (terminal._cursorPosition > 0) {
                terminal._cursorPosition--;
                updateInputLine();
            }
            break;

        default:
            // Regular character input
            if (char >= " " && char <= "~") {
                terminal._currentInput =
                    terminal._currentInput.slice(0, terminal._cursorPosition) +
                    char +
                    terminal._currentInput.slice(terminal._cursorPosition);
                terminal._cursorPosition++;
                updateInputLine();
            }
            break;
    }
}

function updateInputLine() {
    // Clear current line and rewrite with updated input
    terminal.write("\r\x1b[K> " + terminal._currentInput);

    // Position cursor correctly
    const targetPosition = terminal._cursorPosition + 2; // Account for "> "
    terminal.write(`\x1b[${targetPosition}G`);
}

function navigateHistory(direction) {
    if (commandHistory.length === 0) return;

    historyIndex += direction;

    if (historyIndex < 0) {
        historyIndex = 0;
        return;
    }

    if (historyIndex >= commandHistory.length) {
        historyIndex = commandHistory.length - 1;
        return;
    }

    terminal._currentInput = commandHistory[historyIndex];
    terminal._cursorPosition = terminal._currentInput.length;
    updateInputLine();
}

function processCommand(input) {
    if (!input) {
        showPrompt();
        return;
    }

    addToCommandHistory(input);
    terminal.writeln("");

    const parts = input.toLowerCase().split(" ");
    const command = parts[0];
    const args = parts.slice(1);

    executeCommand(command, args, input);
    showPrompt();
}

function addToCommandHistory(input) {
    if (commandHistory.length === 0 || commandHistory.at(-1) !== input) {
        commandHistory.push(input);
        if (commandHistory.length > 50) {
            commandHistory.shift();
        }
    }
    historyIndex = commandHistory.length;
}

function executeCommand(command, args, originalInput) {
    // Track command usage for achievements
    trackCommand(command);

    const commandHandlers = {
        help: () => showHelpCommand(),
        tutorial: () => showTutorial(),
        stats: () => showStatsCommand(),
        progress: () => showProgressCommand(),
        challenges: () => showChallengesCommand(args[0]),
        list: () => showChallengesCommand(args[0]),
        start: () => handleChallengeCommand(args),
        challenge: () => handleChallengeCommand(args),
        hint: () => showHint(),
        skip: () => skipChallenge(),
        answer: () => handleAnswerCommand(args),
        submit: () => handleAnswerCommand(args),
        save: () => saveGame(),
        load: () => loadGame(),
        export: () => exportSaveFile(),
        import: () => showImportHelp(),
        achievements: () => showAchievements(),
        leaderboard: () => showLeaderboard(),
        share: () => shareProgress(),
        theme: () => handleThemeCommand(args),
        clear: () => terminal.clear(),
        exit: () =>
            writeWarning("The ghost will not let you leave so easily..."),
        quit: () =>
            writeWarning("The ghost will not let you leave so easily..."),
    };

    const handler = commandHandlers[command];
    if (handler) {
        handler();
    } else {
        handleUnknownCommand(command, originalInput);
    }
}

function handleChallengeCommand(args) {
    if (args[0]) {
        startChallenge(args[0]);
    } else {
        writeError("Usage: challenge <id>");
    }
}

function handleAnswerCommand(args) {
    if (currentChallenge && args.length > 0) {
        submitAnswer(args.join(" "));
    } else {
        writeError("Usage: answer <your_answer>");
    }
}

function handleThemeCommand(args) {
    if (args[0]) {
        changeTheme(args[0]);
    } else {
        showThemes();
    }
}

function handleUnknownCommand(command, originalInput) {
    if (currentChallenge) {
        submitAnswer(originalInput);
    } else {
        writeError(
            `Unknown command: ${command}. Type 'help' for available commands.`
        );
    }
}

function startGame() {
    terminal.clear();

    // Show horror introduction
    const intro = gameEngine.get_introduction();
    writeHorrorText(intro);

    terminal.writeln("");
    writeSuccess("Welcome to The Hack: Ghost Protocol");
    terminal.writeln(
        'Type "help" for available commands or "challenges" to see all challenges.'
    );

    // Start atmospheric effects
    startAtmosphericEffects();

    updateUI();
    showPrompt();
}

function startAtmosphericEffects() {
    // Occasional subtle glitch effects
    setInterval(() => {
        if (Math.random() < 0.02 && gameState.sanity < 50) {
            // 2% chance, only when sanity is low
            triggerSubtleGlitch();
        }
    }, 5000);

    // Sanity-based effects
    setInterval(() => {
        if (gameState.sanity < 25) {
            triggerSanityEffect();
        }
    }, 10000);
}

function triggerSubtleGlitch() {
    document.querySelector(".terminal-container").style.filter =
        "hue-rotate(180deg)";
    setTimeout(() => {
        document.querySelector(".terminal-container").style.filter = "";
    }, 100);
}

function triggerSanityEffect() {
    const messages = [
        "...something watches from the shadows...",
        "...the code whispers secrets...",
        "...reality.exe has stopped responding...",
        "...the ghost grows stronger...",
    ];

    const msg = messages[Math.floor(Math.random() * messages.length)];
    writeMuted(`\n${msg}\n`);
}

function showHelpCommand() {
    writeInfo("Available Commands:");
    terminal.writeln("");
    writeAccent("Navigation:");
    terminal.writeln("  help                 - Show this help message");
    terminal.writeln(
        "  challenges [level]   - List challenges (optionally by level)"
    );
    terminal.writeln("  stats               - Show player statistics");
    terminal.writeln("");
    writeAccent("Challenge Commands:");
    terminal.writeln("  challenge <id>      - Start a specific challenge");
    terminal.writeln(
        "  answer <text>       - Submit an answer to current challenge"
    );
    terminal.writeln("  hint               - Get a hint for current challenge");
    terminal.writeln(
        "  skip               - Skip current challenge (costs sanity)"
    );
    terminal.writeln("");
    writeAccent("Game Management:");
    terminal.writeln("  save               - Save your progress");
    terminal.writeln("  load               - Load saved progress");
    terminal.writeln("  export             - Export save file for backup");
    terminal.writeln("  import             - Import save file from backup");
    terminal.writeln("  theme <name>       - Change color theme");
    terminal.writeln("  clear              - Clear the terminal");
    terminal.writeln("  tutorial           - Show interactive tutorial");
    terminal.writeln("  progress           - Show detailed progress report");
    terminal.writeln("  achievements       - View unlocked achievements");
    terminal.writeln("");
    writeAccent("Community:");
    terminal.writeln("  leaderboard        - View community leaderboard");
    terminal.writeln("  share              - Share your progress");
}

function showTutorial() {
    writeInfo("🎓 Interactive Tutorial");
    terminal.writeln("");
    writeAccent("Welcome to The Hack: Ghost Protocol!");
    terminal.writeln("");

    writeMuted(
        "This is a horror-themed cybersecurity challenge game where you'll learn:"
    );
    terminal.writeln("• Base64 and other encoding techniques");
    terminal.writeln("• Cryptography and hash functions");
    terminal.writeln("• Web security vulnerabilities");
    terminal.writeln("• Digital forensics methods");
    terminal.writeln("• Binary exploitation basics");
    terminal.writeln("");

    writeWarning("⚠️  SANITY SYSTEM:");
    writeMuted(
        "You start with 100 sanity points. Each challenge costs sanity."
    );
    writeMuted("If your sanity reaches 0, the ghost will consume your mind!");
    terminal.writeln("");

    writeSuccess("💡 GETTING STARTED:");
    terminal.writeln('• Type "challenges" to see available challenges');
    terminal.writeln('• Use "challenge <number>" to start a challenge');
    terminal.writeln('• Submit answers with "answer <your_solution>"');
    terminal.writeln('• Get hints with "hint" if you\'re stuck');
    terminal.writeln('• Use "skip" to skip a challenge (costs sanity)');
    terminal.writeln("");

    writeAccent("The ghost is waiting... Begin when ready.");
}

function showProgressCommand() {
    const challengesJson = gameEngine.get_challenges_json();
    const allChallenges = JSON.parse(challengesJson);

    writeInfo("📊 Your Progress:");
    terminal.writeln("");

    const completedCount = gameState.completed_challenges.length;
    const totalCount = allChallenges.length;
    const completionPercent = Math.round((completedCount / totalCount) * 100);

    writeSuccess(
        `Challenges Completed: ${completedCount}/${totalCount} (${completionPercent}%)`
    );

    // Progress by level
    for (let level = 0; level <= 4; level++) {
        const levelChallenges = allChallenges.filter((c) => c.level === level);
        const levelCompleted = levelChallenges.filter((c) =>
            gameState.completed_challenges.includes(c.id)
        ).length;
        const levelName = [
            "Beginner",
            "Intermediate",
            "Advanced",
            "Expert",
            "Master",
        ][level];

        const bar =
            "█".repeat(
                Math.floor((levelCompleted / levelChallenges.length) * 10)
            ) +
            "░".repeat(
                10 - Math.floor((levelCompleted / levelChallenges.length) * 10)
            );

        terminal.writeln(
            `Level ${level} (${levelName}): [${bar}] ${levelCompleted}/${levelChallenges.length}`
        );
    }

    terminal.writeln("");
    writeWarning(`Sanity: ${gameState.sanity}/100`);
    writeAccent(`Experience: ${gameState.xp} XP (Level ${gameState.level})`);
}

function showStatsCommand() {
    const stats = JSON.parse(gameState.get_stats_json());

    writeInfo("📊 Player Statistics:");
    terminal.writeln("");

    // Level and XP with visual bar
    writeAccent(`🎯 Level: ${stats.level}`);
    const xpProgress = Math.min(stats.xp / 100, 1) * 20; // Scale to 20 chars
    const xpBar = "█".repeat(Math.floor(xpProgress)) + "░".repeat(20 - Math.floor(xpProgress));
    terminal.writeln(`   XP: ${stats.xp} [${xpBar}]`);
    // Sanity with colored bar
    const sanityPercent = stats.sanity;
    const sanityProgress = Math.floor(sanityPercent / 5); // 20 segments
    const sanityBar = "█".repeat(sanityProgress) + "░".repeat(20 - sanityProgress);
    const sanityColor = sanityPercent > 60 ? "🟢" : sanityPercent > 30 ? "🟡" : "🔴";
    writeAccent(`🧠 Sanity: ${stats.sanity}/100 ${sanityColor}`);
    terminal.writeln(`   [${sanityBar}]`);
    terminal.writeln("");

    // Challenge completion
    writeAccent(`🎮 Challenges Completed: ${stats.completed_challenges.length}/11`);
    const challengeProgress = Math.floor((stats.completed_challenges.length / 11) * 20);
    const challengeBar = "█".repeat(challengeProgress) + "░".repeat(20 - challengeProgress);
    terminal.writeln(`   [${challengeBar}]`);
    terminal.writeln("");

    // Performance metrics
    writeAccent("📈 Performance Metrics:");
    terminal.writeln(`   Total Attempts: ${stats.total_attempts || 0}`);
    terminal.writeln(`   Hints Used: ${stats.hints_used || 0}`);
    const efficiency = stats.completed_challenges.length > 0 ?
        ((stats.total_attempts || 0) / stats.completed_challenges.length).toFixed(1) : "N/A";
    terminal.writeln(`   Avg Attempts per Challenge: ${efficiency}`);
    terminal.writeln("");

    // Achievement summary
    const unlockedCount = achievements.length;
    const totalAchievements = Object.keys(ACHIEVEMENTS).length;
    writeAccent(`🏆 Achievements: ${unlockedCount}/${totalAchievements} unlocked`);

    if (unlockedCount > 0) {
        terminal.writeln("   Recent achievements:");
        achievements.slice(-3).forEach(id => {
            const achievement = ACHIEVEMENTS[id];
            if (achievement) {
                writeMuted(`   • ${achievement.title}`);
            }
        });
    }
}

function showChallengesCommand(level) {
    writeInfo("Available Challenges:");
    terminal.writeln("");

    const challengesJson = gameEngine.get_challenges_json();
    const challenges = JSON.parse(challengesJson);

    const filteredChallenges = level
        ? challenges.filter((c) => c.level === Number.parseInt(level))
        : challenges;

    for (const challenge of filteredChallenges) {
        const completed = gameState.completed_challenges.includes(challenge.id);
        const status = completed ? "[✓]" : "[ ]";
        const statusColor = completed ? "success" : "muted";

        write(`${status} `, statusColor);
        writeAccent(`${challenge.id}: `);
        terminal.write(`${challenge.title} `);
        writeMuted(`(Level ${challenge.level}, ${challenge.xp_reward} XP)`);
        terminal.writeln("");
    }
}

function startChallenge(challengeId) {
    // Get all challenges as JSON and find the one we need
    const challengesJson = gameEngine.get_challenges_json();
    const challenges = JSON.parse(challengesJson);
    const challenge = challenges.find((c) => c.id === challengeId);

    if (!challenge) {
        writeError(`Challenge '${challengeId}' not found.`);
        return;
    }

    if (!gameState.can_attempt_challenge(challengeId)) {
        writeWarning("You have already completed this challenge.");
        return;
    }

    currentChallenge = challenge;

    // Track challenge start for achievements
    trackChallengeStart();

    terminal.writeln("");
    writeHorrorText("═".repeat(60));
    writeAccent(`CHALLENGE: ${challenge.title}`);
    writeHorrorText("═".repeat(60));
    terminal.writeln("");

    writeMuted(`Level: ${challenge.level} | Category: ${challenge.category}`);
    writeMuted(
        `Reward: ${challenge.xp_reward} XP | Sanity Cost: ${challenge.sanity_cost}`
    );
    terminal.writeln("");

    writeInfo("Challenge:");
    terminal.writeln(challenge.description);
    terminal.writeln("");
    writeMuted("Submit your answer with: answer <your_answer>");
    writeMuted("Get a hint with: hint");
    writeMuted("Skip challenge with: skip");
}

function submitAnswer(answer) {
    if (!currentChallenge) {
        writeError('No active challenge. Use "challenge <id>" to start one.');
        return;
    }

    const resultJson = gameEngine.validate_challenge_answer(
        currentChallenge.id,
        answer,
        gameState
    );

    const result = JSON.parse(resultJson);

    if (result.success) {
        writeSuccess(result.message);

        // Track achievement for successful challenge completion
        trackChallengeComplete(true);

        if (result.leveled_up) {
            writeAccent(`🎉 LEVEL UP! You are now level ${result.new_level}!`);
            triggerHorrorEffect();
        }
        currentChallenge = null;
    } else {
        writeError(result.message);
        if (result.game_over) {
            showGameOver();
            return;
        }
    }

    updateUI();
}

function showHint() {
    if (!currentChallenge) {
        writeError("No active challenge.");
        return;
    }

    // Track hint usage for achievements
    challengeHintCount++;

    // Get hints from challenge JSON data
    const hints = currentChallenge.hints || [];
    if (hints.length > 0) {
        // For now, show the first hint. Could be enhanced to track hint progression
        const hint = hints[0];
        writeWarning(`💡 Hint: ${hint}`);
    } else {
        writeError("No hints available for this challenge.");
    }
}

function skipChallenge() {
    if (!currentChallenge) {
        writeError("No active challenge.");
        return;
    }

    writeWarning("Skipping challenge...");
    const stillAlive = gameState.decrease_sanity(currentChallenge.sanity_cost);

    if (!stillAlive) {
        showGameOver();
        return;
    }

    currentChallenge = null;
    updateUI();
}

function showGameOver() {
    document.getElementById("gameOver").style.display = "flex";
    writeError("GAME OVER - Your sanity has been completely drained...");
    triggerHorrorEffect();
}

function resetGame() {
    gameState = new wasmModule.WebGameState();
    currentChallenge = null;
    document.getElementById("gameOver").style.display = "none";
    updateUI();
    startGame();
}

function saveGame() {
    try {
        const saveData = wasmModule.generate_save_file(gameState);
        localStorage.setItem("hack_game_save", saveData);
        writeSuccess("Game saved successfully!");
    } catch (error) {
        writeError("Failed to save game.");
        console.error("Save error:", error);
    }
}

function loadGame() {
    try {
        const saveData = localStorage.getItem("hack_game_save");
        if (saveData) {
            gameState = wasmModule.load_save_file(saveData);
            writeSuccess("Game loaded successfully!");
            updateUI();
        } else {
            writeWarning("No saved game found.");
        }
    } catch (error) {
        writeError("Failed to load game.");
        console.error("Load error:", error);
    }
}

function loadSavedGame() {
    const saveData = localStorage.getItem("hack_game_save");
    if (saveData) {
        try {
            gameState = wasmModule.load_save_file(saveData);
        } catch (error) {
            console.error("Could not load saved game, starting fresh:", error);
            // Clear corrupted save data
            localStorage.removeItem("hack_game_save");
            // Initialize new game state
            gameState = new wasmModule.WebGameState();
        }
    }
}

function updateUI() {
    document.getElementById("playerLevel").textContent = gameState.level;
    document.getElementById("playerXP").textContent = gameState.xp;
    document.getElementById("playerSanity").textContent = gameState.sanity;
    document.getElementById("challengesCompleted").textContent =
        gameState.completed_challenges.length;

    // Update sanity color based on level
    const sanityElement = document.getElementById("playerSanity");
    const sanity = gameState.sanity;
    if (sanity < 25) {
        sanityElement.style.color = "var(--text-danger)";
    } else if (sanity < 50) {
        sanityElement.style.color = "var(--text-warning)";
    } else {
        sanityElement.style.color = "var(--text-accent)";
    }
}

function showPrompt() {
    terminal.write("\r\n> ");
}

// Terminal output helpers
function writeSuccess(text) {
    terminal.writeln(`\x1b[32m✓ ${text}\x1b[0m`);
}

function writeError(text) {
    terminal.writeln(`\x1b[31m❌ ${text}\x1b[0m`);
}

function writeWarning(text) {
    terminal.writeln(`\x1b[33m⚠ ${text}\x1b[0m`);
}

function writeInfo(text) {
    terminal.writeln(`\x1b[36mℹ ${text}\x1b[0m`);
}

function writeAccent(text) {
    terminal.writeln(`\x1b[92m${text}\x1b[0m`);
}

function writeMuted(text) {
    terminal.writeln(`\x1b[90m${text}\x1b[0m`);
}

function write(text, type = "normal") {
    const colors = {
        normal: "",
        success: "\x1b[32m",
        error: "\x1b[31m",
        warning: "\x1b[33m",
        info: "\x1b[36m",
        accent: "\x1b[92m",
        muted: "\x1b[90m",
    };

    terminal.write(`${colors[type] || ""}${text}\x1b[0m`);
}

function writeHorrorText(text) {
    terminal.writeln(`\x1b[31m${text}\x1b[0m`);
}

function showError(message) {
    const loading = document.getElementById("loading");
    if (loading) {
        loading.innerHTML = `
            <div style="color: var(--text-danger); text-align: center;">
                <div style="font-size: 1.5em; margin-bottom: 20px;">💀 ERROR</div>
                <div>${message}</div>
                <button class="btn btn-primary" style="margin-top: 20px;" onclick="location.reload()">
                    Retry
                </button>
            </div>
        `;
    }
}

// Theme system
function getTerminalTheme() {
    const themes = {
        horror: {
            background: "#000000",
            foreground: "#ffffff",
            cursor: "#ff0000",
            selection: "#333333",
        },
        green: {
            background: "#000000",
            foreground: "#00ff41",
            cursor: "#00ff41",
            selection: "#003300",
        },
        blue: {
            background: "#0a0a0a",
            foreground: "#00aaff",
            cursor: "#00aaff",
            selection: "#001a33",
        },
        matrix: {
            background: "#000000",
            foreground: "#00ff00",
            cursor: "#00ff00",
            selection: "#002200",
            brightGreen: "#00ff00",
        },
        cyberpunk: {
            background: "#0d1117",
            foreground: "#ff00ff",
            cursor: "#00ffff",
            selection: "#330033",
            cyan: "#00ffff",
            magenta: "#ff00ff",
        },
        retro: {
            background: "#000000",
            foreground: "#ffb000",
            cursor: "#ffb000",
            selection: "#332200",
        },
        contrast: {
            background: "#000000",
            foreground: "#ffffff",
            cursor: "#ffffff",
            selection: "#ffffff",
        },
        midnight: {
            background: "#0f1419",
            foreground: "#c7d2fe",
            cursor: "#818cf8",
            selection: "#312e81",
        },
    };

    return themes[currentTheme] || themes.horror;
}

function changeTheme(themeName) {
    const availableThemes = [
        "horror",
        "green",
        "blue",
        "matrix",
        "cyberpunk",
        "retro",
        "contrast",
        "midnight",
    ];

    if (availableThemes.includes(themeName)) {
        currentTheme = themeName;
        if (terminal) {
            terminal.options.theme = getTerminalTheme();
        }
        writeSuccess(`Theme changed to: ${themeName}`);

        // Track theme achievement
        const usedThemes = JSON.parse(
            localStorage.getItem("usedThemes") || "[]"
        );
        if (!usedThemes.includes(themeName)) {
            usedThemes.push(themeName);
            localStorage.setItem("usedThemes", JSON.stringify(usedThemes));

            if (usedThemes.length >= availableThemes.length) {
                unlockAchievement("theme_master");
            }
        }
    } else {
        showThemes();
    }
}

function showThemes() {
    writeInfo("Available themes:");
    terminal.writeln("  horror     - 🩸 Red and white horror theme");
    terminal.writeln("  green      - 🟢 Classic green terminal theme");
    terminal.writeln("  blue       - 🔵 Cool blue terminal theme");
    terminal.writeln("  matrix     - 💚 Classic Matrix green theme");
    terminal.writeln("  cyberpunk  - 💜 Neon pink and cyan theme");
    terminal.writeln("  retro      - 🟨 Amber on black retro theme");
    terminal.writeln("  contrast   - ⚫ High contrast accessibility theme");
    terminal.writeln("  midnight   - 🌙 Dark blue midnight theme");
    terminal.writeln("Usage: theme <name>");
}

function toggleTheme() {
    const themes = ["horror", "green", "blue"];
    const currentIndex = themes.indexOf(currentTheme);
    const nextTheme = themes[(currentIndex + 1) % themes.length];
    changeTheme(nextTheme);
}

// Horror effects
function triggerHorrorEffect() {
    // Random glitch effect
    document.body.classList.add("glitch");
    setTimeout(() => {
        document.body.classList.remove("glitch");
    }, 500);

    // Terminal flicker
    if (terminal) {
        terminal.write("\x1b[31m█\x1b[0m");
        setTimeout(() => {
            terminal.write("\b \b");
        }, 100);
    }
}

// UI button handlers
globalThis.showHelp = () => {
    processCommand("help");
};

globalThis.showStats = () => {
    processCommand("stats");
};

globalThis.saveGame = () => {
    processCommand("save");
};

globalThis.loadGame = () => {
    processCommand("load");
};

globalThis.toggleTheme = () => {
    toggleTheme();
};

globalThis.resetGame = () => {
    resetGame();
};

function exportSaveFile() {
    try {
        const saveData = localStorage.getItem("hackSimulatorSave");
        if (!saveData) {
            writeWarning("No save file found to export.");
            return;
        }

        // Create downloadable file
        const blob = new Blob([saveData], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `hack_simulator_save_${
            new Date().toISOString().split("T")[0]
        }.json`;
        document.body.appendChild(a);
        a.click();
        a.remove();
        URL.revokeObjectURL(url);

        writeSuccess("Save file exported successfully!");
        writeMuted("File saved to your Downloads folder.");
    } catch (error) {
        writeError("Failed to export save file: " + error.message);
    }
}

function showImportHelp() {
    writeInfo("Import Save File:");
    writeMuted("1. Use 'export' command first to backup your current progress");
    writeMuted("2. Visit this page on another device");
    writeMuted("3. Drag and drop your .json save file onto the terminal");
    writeMuted("4. Or use the file input that will appear...");

    // Create hidden file input
    const fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.accept = ".json";
    fileInput.style.display = "none";

    fileInput.addEventListener("change", (event) => {
        const file = event.target.files[0];
        if (file) {
            importSaveFile(file);
        }
    });

    document.body.appendChild(fileInput);
    fileInput.click();
    setTimeout(() => fileInput.remove(), 1000);
}

async function importSaveFile(file) {
    try {
        const saveData = await file.text();
        JSON.parse(saveData); // Validate JSON

        // Backup current save
        const currentSave = localStorage.getItem("hackSimulatorSave");
        if (currentSave) {
            localStorage.setItem("hackSimulatorSave_backup", currentSave);
        }

        localStorage.setItem("hackSimulatorSave", saveData);
        location.reload(); // Reload to apply new save
    } catch (error) {
        writeError("Invalid save file format: " + error.message);
    }
}

// Achievement System
const ACHIEVEMENTS = {
    first_blood: {
        id: "first_blood",
        title: "🩸 First Blood",
        description: "Complete your first challenge",
        condition: (stats) => stats.challengesCompleted >= 1,
    },
    speed_demon: {
        id: "speed_demon",
        title: "⚡ Speed Demon",
        description: "Complete a challenge in under 30 seconds",
        hidden: true,
    },
    hint_free: {
        id: "hint_free",
        title: "🧠 Hint-Free Hero",
        description: "Complete a challenge without using hints",
        hidden: true,
    },
    sanity_saver: {
        id: "sanity_saver",
        title: "🧘 Sanity Saver",
        description: "Complete a level with over 80% sanity remaining",
        hidden: true,
    },
    ghost_hunter: {
        id: "ghost_hunter",
        title: "👻 Ghost Hunter",
        description: "Complete all challenges in the game",
        condition: (stats) => stats.challengesCompleted >= 11,
    },
    explorer: {
        id: "explorer",
        title: "🗺️ Digital Explorer",
        description: "Try at least 5 different commands",
        hidden: true,
    },
    persistent: {
        id: "persistent",
        title: "🔄 Persistent Hacker",
        description: "Fail a challenge 3 times but still complete it",
        hidden: true,
    },
    theme_master: {
        id: "theme_master",
        title: "🎨 Theme Master",
        description: "Try all available color themes",
        hidden: true,
    },
};

function checkAchievements() {
    const stats = {
        challengesCompleted: gameState.get_completed_challenges().length,
        sanity: gameState.get_sanity(),
        level: gameState.get_level(),
    };

    for (const achievement of Object.values(ACHIEVEMENTS)) {
        if (achievement.condition && !achievements.includes(achievement.id)) {
            if (achievement.condition(stats)) {
                unlockAchievement(achievement.id);
            }
        }
    }
}

function unlockAchievement(achievementId) {
    if (achievements.includes(achievementId)) return;

    achievements.push(achievementId);
    const achievement = ACHIEVEMENTS[achievementId];

    // Show achievement notification
    showAchievementNotification(achievement);

    // Save achievements
    localStorage.setItem(
        "hackSimulatorAchievements",
        JSON.stringify(achievements)
    );

    // Play achievement sound
    playAchievementSound();
}

function showAchievementNotification(achievement) {
    writeAccent(`\n🏆 ACHIEVEMENT UNLOCKED! 🏆`);
    writeSuccess(`${achievement.title}`);
    writeMuted(`${achievement.description}\n`);
}

function showAchievements() {
    writeInfo("🏆 Achievements");
    terminal.writeln("");

    let unlockedCount = 0;
    for (const achievement of Object.values(ACHIEVEMENTS)) {
        const unlocked = achievements.includes(achievement.id);
        if (unlocked) unlockedCount++;

        if (unlocked || !achievement.hidden) {
            const status = unlocked ? "✅" : "🔒";
            const title = unlocked ? achievement.title : "???";
            const desc = unlocked
                ? achievement.description
                : "Keep playing to unlock!";

            terminal.writeln(`  ${status} ${title}`);
            writeMuted(`     ${desc}`);
            terminal.writeln("");
        }
    }

    writeAccent(
        `Progress: ${unlockedCount}/${
            Object.keys(ACHIEVEMENTS).length
        } achievements unlocked`
    );
}

function loadAchievements() {
    const saved = localStorage.getItem("hackSimulatorAchievements");
    if (saved) {
        achievements = JSON.parse(saved);
    }
}

// Achievement tracking helpers
let challengeStartTime = null;
let challengeHintCount = 0;
let commandsUsed = new Set();

function trackChallengeStart() {
    challengeStartTime = Date.now();
    challengeHintCount = 0;
}

function trackCommand(command) {
    commandsUsed.add(command);
    if (commandsUsed.size >= 5) {
        unlockAchievement("explorer");
    }
}

function trackChallengeComplete(success) {
    if (!success || !challengeStartTime) return;

    const timeElapsed = Date.now() - challengeStartTime;

    // Speed achievement
    if (timeElapsed < 30000) {
        unlockAchievement("speed_demon");
    }

    // Hint-free achievement
    if (challengeHintCount === 0) {
        unlockAchievement("hint_free");
    }

    // Sanity saver (check after level completion)
    if (gameState.get_sanity() > 80 && gameState.get_level() > 0) {
        unlockAchievement("sanity_saver");
    }

    checkAchievements();
}

// Audio System
function initializeAudio() {
    try {
        audioContext = new (globalThis.AudioContext ||
            globalThis.webkitAudioContext)();
        startAmbientAudio();
    } catch (error) {
        console.log("Audio not available:", error);
    }
}

function playAchievementSound() {
    if (!audioContext) return;

    // Create a simple success tone
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);

    oscillator.frequency.setValueAtTime(440, audioContext.currentTime);
    oscillator.frequency.setValueAtTime(880, audioContext.currentTime + 0.1);

    gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
    gainNode.gain.exponentialRampToValueAtTime(
        0.01,
        audioContext.currentTime + 0.5
    );

    oscillator.start(audioContext.currentTime);
    oscillator.stop(audioContext.currentTime + 0.5);
}

function startAmbientAudio() {
    if (!audioContext || gameState.get_sanity() > 70) return;

    // Create subtle ambient horror sounds based on sanity level
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();
    const filter = audioContext.createBiquadFilter();

    oscillator.connect(filter);
    filter.connect(gainNode);
    gainNode.connect(audioContext.destination);

    oscillator.type = "sawtooth";
    oscillator.frequency.setValueAtTime(60, audioContext.currentTime);

    filter.type = "lowpass";
    filter.frequency.setValueAtTime(200, audioContext.currentTime);

    const sanityLevel = gameState.get_sanity() / 100;
    gainNode.gain.setValueAtTime(
        0.02 * (1 - sanityLevel),
        audioContext.currentTime
    );

    oscillator.start();

    // Stop after 10 seconds, restart randomly
    setTimeout(() => {
        oscillator.stop();
        if (Math.random() < 0.3) {
            setTimeout(startAmbientAudio, Math.random() * 20000 + 10000);
        }
    }, 10000);
}

// Add drag and drop support for save files
document.addEventListener("dragover", (e) => {
    e.preventDefault();
});

document.addEventListener("drop", (e) => {
    e.preventDefault();
    const files = e.dataTransfer.files;
    if (files.length > 0 && files[0].name.endsWith(".json")) {
        writeInfo("Importing save file...");
        importSaveFile(files[0]);
    }
});

// Social Features
function showLeaderboard() {
    writeInfo("🏆 Community Leaderboard");
    terminal.writeln("");
    
    writeMuted("📊 Anonymous leaderboard coming soon!");
    terminal.writeln("");
    writeMuted("Your stats (local only):");
    const stats = JSON.parse(gameState.get_stats_json());
    terminal.writeln(`• Challenges completed: ${stats.completed_challenges.length}/11`);
    terminal.writeln(`• Current level: ${stats.level}`);
    terminal.writeln(`• Achievements unlocked: ${achievements.length}/${Object.keys(ACHIEVEMENTS).length}`);
    terminal.writeln("");
    
    writeMuted("🔒 Your privacy is protected - we never collect personal data!");
    writeMuted("🎮 Keep playing to improve your ranking!");
}

function shareProgress() {
    const stats = JSON.parse(gameState.get_stats_json());
    const completionPercent = Math.floor((stats.completed_challenges.length / 11) * 100);
    const achievementCount = achievements.length;
    
    const shareText = `🎮 I'm playing The Hack: Ghost Protocol! 
📊 Progress: ${completionPercent}% complete (${stats.completed_challenges.length}/11 challenges)
🏆 Achievements: ${achievementCount}/${Object.keys(ACHIEVEMENTS).length} unlocked
💀 Sanity: ${stats.sanity}/100

Try it yourself at hack.andernet.dev - A horror-themed cybersecurity CTF game! 👻`;

    // Copy to clipboard
    try {
        navigator.clipboard.writeText(shareText).then(() => {
            writeSuccess("📋 Progress shared to clipboard!");
            writeMuted("Paste this anywhere to share your achievements!");
        }).catch(() => {
            // Fallback: show the text for manual copying
            showShareText(shareText);
        });
    } catch (error) {
        showShareText(shareText);
    }
}

function showShareText(text) {
    writeInfo("📢 Share Your Progress:");
    terminal.writeln("");
    writeMuted("Copy this text to share:");
    terminal.writeln("─".repeat(50));
    terminal.writeln(text);
    terminal.writeln("─".repeat(50));
}

// Initialize when page loads
document.addEventListener("DOMContentLoaded", init);

// Handle page visibility for auto-save
document.addEventListener("visibilitychange", () => {
    if (document.hidden && gameState) {
        saveGame();
    }
});
