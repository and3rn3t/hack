// The Hack: Ghost Protocol - Web Interface
// Integrates with WebAssembly backend

let gameEngine = null;
let gameState = null;
let terminal = null;
let currentTheme = "horror";
let currentChallenge = null;
let commandHistory = [];
let historyIndex = -1;

// Initialize the game
async function init() {
    try {
        // Update loading progress
        updateLoadingProgress(10);

        // Load WebAssembly module
        const wasm = await import("./pkg/hack_simulator.js");
        await wasm.default();

        updateLoadingProgress(30);

        // Initialize game engine and state
        gameEngine = new wasm.WebGameEngine();
        gameState = new wasm.WebGameState();

        updateLoadingProgress(50);

        // Initialize terminal
        initializeTerminal();

        updateLoadingProgress(70);

        // Load saved game if available
        loadSavedGame();

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
    const commandHandlers = {
        help: () => showHelpCommand(),
        stats: () => showStatsCommand(),
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

    updateUI();
    showPrompt();
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
    terminal.writeln("  theme <name>       - Change color theme");
    terminal.writeln("  clear              - Clear the terminal");
}

function showStatsCommand() {
    const stats = JSON.parse(gameState.get_stats_json());

    writeInfo("Player Statistics:");
    terminal.writeln("");
    terminal.writeln(`Level: ${stats.level}`);
    terminal.writeln(`Experience: ${stats.xp}`);
    terminal.writeln(`Sanity: ${stats.sanity}/100`);
    terminal.writeln(
        `Challenges Completed: ${stats.completed_challenges.length}`
    );
    terminal.writeln(`Total Attempts: ${stats.total_attempts || 0}`);
    terminal.writeln(`Hints Used: ${stats.hints_used || 0}`);
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
        const completed = gameState
            .completed_challenges()
            .includes(challenge.id);
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
    const challenge = gameEngine.get_challenge(challengeId);

    if (!challenge) {
        writeError(`Challenge '${challengeId}' not found.`);
        return;
    }

    if (!gameState.can_attempt_challenge(challengeId)) {
        writeWarning("You have already completed this challenge.");
        return;
    }

    currentChallenge = challenge;

    terminal.writeln("");
    writeHorrorText("═".repeat(60));
    writeAccent(`CHALLENGE: ${challenge.title()}`);
    writeHorrorText("═".repeat(60));
    terminal.writeln("");

    writeMuted(
        `Level: ${challenge.level()} | Category: ${challenge.category()}`
    );
    writeMuted(
        `Reward: ${challenge.xp_reward()} XP | Sanity Cost: ${challenge.sanity_cost()}`
    );
    terminal.writeln("");

    terminal.writeln(challenge.description());
    terminal.writeln("");
    writeInfo("Challenge:");
    terminal.writeln(challenge.prompt());
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
        currentChallenge.id(),
        answer,
        gameState
    );

    const result = JSON.parse(resultJson);

    if (result.success) {
        writeSuccess(result.message);
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

    // Get next available hint (simple implementation)
    const hint = currentChallenge.get_hint(0); // For now, just show first hint
    if (hint) {
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
    const stillAlive = gameState.decrease_sanity(
        currentChallenge.sanity_cost()
    );

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
    gameState = new gameEngine.constructor.Module.WebGameState();
    currentChallenge = null;
    document.getElementById("gameOver").style.display = "none";
    updateUI();
    startGame();
}

function saveGame() {
    try {
        const saveData =
            gameEngine.constructor.Module.generate_save_file(gameState);
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
            gameState = gameEngine.constructor.Module.load_save_file(saveData);
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
            gameState = gameEngine.constructor.Module.load_save_file(saveData);
        } catch (error) {
            console.error("Could not load saved game, starting fresh:", error);
            // Clear corrupted save data
            localStorage.removeItem("hack_game_save");
            // Initialize new game state
            gameState = new gameEngine.constructor.Module.WebGameState();
        }
    }
}

function updateUI() {
    document.getElementById("playerLevel").textContent = gameState.level();
    document.getElementById("playerXP").textContent = gameState.xp();
    document.getElementById("playerSanity").textContent = gameState.sanity();
    document.getElementById("challengesCompleted").textContent =
        gameState.completed_challenges().length;

    // Update sanity color based on level
    const sanityElement = document.getElementById("playerSanity");
    const sanity = gameState.sanity();
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
    };

    return themes[currentTheme] || themes.horror;
}

function changeTheme(themeName) {
    if (["horror", "green", "blue"].includes(themeName)) {
        currentTheme = themeName;
        if (terminal) {
            terminal.options.theme = getTerminalTheme();
        }
        writeSuccess(`Theme changed to: ${themeName}`);
    } else {
        showThemes();
    }
}

function showThemes() {
    writeInfo("Available themes:");
    terminal.writeln("  horror - Red and white horror theme");
    terminal.writeln("  green  - Classic green terminal theme");
    terminal.writeln("  blue   - Cool blue terminal theme");
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

// Initialize when page loads
document.addEventListener("DOMContentLoaded", init);

// Handle page visibility for auto-save
document.addEventListener("visibilitychange", () => {
    if (document.hidden && gameState) {
        saveGame();
    }
});
