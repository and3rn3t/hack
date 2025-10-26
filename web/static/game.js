// The Hack: Ghost Protocol - Web Interface
// Integrates with WebAssembly backend

// Register Service Worker for PWA functionality
if ("serviceWorker" in navigator) {
    window.addEventListener("load", () => {
        registerServiceWorker();
    });
}

async function registerServiceWorker() {
    try {
        const registration = await navigator.serviceWorker.register("/sw.js", {
            scope: "/",
        });

        console.log(
            "✅ Service Worker registered successfully:",
            registration.scope
        );

        // Listen for updates
        registration.addEventListener("updatefound", () => {
            const newWorker = registration.installing;
            newWorker.addEventListener("statechange", () => {
                if (
                    newWorker.state === "installed" &&
                    navigator.serviceWorker.controller
                ) {
                    showUpdateNotification();
                }
            });
        });

        // Handle message from service worker
        navigator.serviceWorker.addEventListener("message", (event) => {
            handleServiceWorkerMessage(event.data);
        });
    } catch (error) {
        console.warn("❌ Service Worker registration failed:", error);
    }
}

function showUpdateNotification() {
    if (terminal && terminal.element) {
        terminal.writeln(
            "\x1b[33m📱 App update available! Refresh to get the latest features.\x1b[0m"
        );
    }
}

// Enhanced service worker message handling with offline support
function handleServiceWorkerMessage(data) {
    switch (data.type) {
        case "CACHE_UPDATED":
            console.log("🔄 Cache updated:", data.payload);
            if (terminal) {
                writeMuted("App cache updated for offline play");
            }
            break;

        case "SW_ACTIVATED":
            console.log("🎯 Service Worker activated:", data.version);
            if (terminal) {
                writeSuccess(`App updated to v${data.version}!`);
            }
            break;

        case "OFFLINE_MODE":
            handleOfflineMode(data.isOffline);
            break;

        case "BACKGROUND_SYNC_COMPLETE":
            if (terminal) {
                writeSuccess("Game data synchronized!");
            }
            break;

        case "NOTIFICATION_ACTION":
            handleNotificationAction(data);
            break;

        default:
            console.log("� Service Worker message:", data);
    }
}

// Handle offline/online mode changes
function handleOfflineMode(isOffline) {
    const offlineIndicator =
        document.querySelector(".offline-indicator") ||
        createOfflineIndicator();

    if (isOffline) {
        offlineIndicator.style.display = "block";
        if (terminal) {
            writeWarning(
                "⚠️ Offline mode - progress will sync when reconnected"
            );
        }

        // Switch to offline-first game mode
        enableOfflineMode();
    } else {
        offlineIndicator.style.display = "none";
        if (terminal) {
            writeSuccess("🌐 Back online - syncing game data...");
        }

        // Trigger background sync
        if (
            "serviceWorker" in navigator &&
            "sync" in window.ServiceWorkerRegistration.prototype
        ) {
            navigator.serviceWorker.ready
                .then((registration) => {
                    return registration.sync.register("game-save-sync");
                })
                .catch(() => {});
        }
    }
}

function createOfflineIndicator() {
    const indicator = document.createElement("div");
    indicator.className = "offline-indicator";
    indicator.innerHTML = "📡 Offline Mode";
    indicator.style.cssText = `
        position: fixed;
        top: 10px;
        right: 10px;
        background: var(--text-warning);
        color: var(--bg-primary);
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 0.8em;
        z-index: 1500;
        display: none;
        animation: pulse 2s infinite;
    `;
    document.body.appendChild(indicator);
    return indicator;
}

// Enable full offline functionality
function enableOfflineMode() {
    // Store game state locally more frequently
    if (gameState) {
        storeOfflineGameState();
    }

    // Cache current challenges for offline play
    if (gameEngine) {
        cacheOfflineChallenges();
    }

    // Enable offline-only features
    showOfflineFeatures();
}

async function storeOfflineGameState() {
    try {
        const saveData = wasmModule.generate_save_file(gameState);

        // Store in multiple locations for redundancy
        localStorage.setItem("hack_game_save_offline", saveData);

        // Also store in IndexedDB via service worker
        if (
            "serviceWorker" in navigator &&
            navigator.serviceWorker.controller
        ) {
            navigator.serviceWorker.controller.postMessage({
                type: "STORE_OFFLINE_SAVE",
                data: {
                    saveData: saveData,
                    timestamp: Date.now(),
                    gameVersion: "1.3.0",
                },
            });
        }

        console.log("📱 Offline game state stored");
    } catch (error) {
        console.error("Failed to store offline game state:", error);
    }
}

async function cacheOfflineChallenges() {
    try {
        const challengesJson = gameEngine.get_challenges_json();
        const challenges = JSON.parse(challengesJson);

        // Store challenges for offline access
        localStorage.setItem("hack_challenges_offline", challengesJson);

        // Cache challenge data in service worker
        if (
            "serviceWorker" in navigator &&
            navigator.serviceWorker.controller
        ) {
            navigator.serviceWorker.controller.postMessage({
                type: "CACHE_CHALLENGES",
                data: challenges,
            });
        }

        console.log("🎮 Challenges cached for offline play");
    } catch (error) {
        console.error("Failed to cache challenges:", error);
    }
}

function showOfflineFeatures() {
    if (terminal) {
        terminal.writeln("");
        writeInfo("🔒 Offline Mode Features Available:");
        writeMuted("• All 26 challenges playable offline");
        writeMuted("• Progress saves locally and syncs when online");
        writeMuted("• Settings and achievements preserved");
        writeMuted("• Practice mode fully functional");
        terminal.writeln("");
    }
}

// Handle notification actions from service worker
function handleNotificationAction(data) {
    switch (data.action) {
        case "continue_game":
            // Already loaded, just show stats
            processCommand("stats");
            break;

        case "show_achievements":
            processCommand("achievements");
            break;

        case "start_challenge":
            if (data.challengeId) {
                processCommand(`challenge ${data.challengeId}`);
            } else {
                processCommand("challenges");
            }
            break;

        case "show_challenges":
            processCommand("challenges");
            break;

        case "reload_app":
            location.reload();
            break;

        default:
            console.log("Unknown notification action:", data);
    }
}

// Install PWA prompt
let deferredPrompt = null;

window.addEventListener("beforeinstallprompt", (event) => {
    event.preventDefault();
    deferredPrompt = event;
    showInstallPrompt();
});

function showInstallPrompt() {
    // Add install button to UI
    const installBtn = document.createElement("button");
    installBtn.className = "btn btn-primary install-btn";
    installBtn.textContent = "📱 Install App";
    installBtn.onclick = installPWA;

    const controls = document.querySelector(".controls");
    if (controls && !controls.querySelector(".install-btn")) {
        controls.appendChild(installBtn);
    }
}

// Mobile interface enhancements
let isMobile = false;
let virtualKeyboardVisible = false;

// Detect mobile device
function detectMobile() {
    isMobile =
        /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
            navigator.userAgent
        ) || window.innerWidth <= 768;

    if (isMobile) {
        document.body.classList.add("mobile-device");
        initializeMobileFeatures();
    }

    return isMobile;
}

// Send quick command from mobile quick bar
function sendQuickCommand(command) {
    if (!terminal) return;

    // Provide haptic feedback if available
    if ("vibrate" in navigator) {
        navigator.vibrate(10);
    }

    // Write command to terminal
    terminal.write(command);

    // Execute the command
    setTimeout(() => {
        terminal.write("\r");
        handleCommand(command);
    }, 100);
}

// Initialize mobile-specific features
function initializeMobileFeatures() {
    // Add virtual keyboard
    addVirtualKeyboard();

    // Add touch gesture support
    addTouchGestures();

    // Handle virtual keyboard visibility
    handleVirtualKeyboard();

    // Add pull-to-refresh functionality
    addPullToRefresh();

    // Optimize terminal for mobile
    optimizeTerminalForMobile();

    console.log("📱 Mobile features initialized");
}

// Enhanced virtual keyboard with multiple layouts and better functionality
function addVirtualKeyboard() {
    const gameContainer = document.querySelector(".game-container");

    const keyboard = document.createElement("div");
    keyboard.className = "mobile-keyboard";
    keyboard.innerHTML = `
        <div class="keyboard-header">
            <button class="keyboard-mode-btn active" data-mode="commands">Commands</button>
            <button class="keyboard-mode-btn" data-mode="typing">Typing</button>
            <button class="keyboard-mode-btn" data-mode="numbers">Numbers</button>
            <button class="keyboard-close-btn" data-key="toggle">✕</button>
        </div>

        <div class="keyboard-layout commands-layout active">
            <div class="keyboard-row">
                <button class="key-btn" data-key="help">help</button>
                <button class="key-btn" data-key="list">challenges</button>
                <button class="key-btn" data-key="hint">hint</button>
                <button class="key-btn" data-key="stats">stats</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn" data-key="start">start</button>
                <button class="key-btn" data-key="answer">answer</button>
                <button class="key-btn" data-key="skip">skip</button>
                <button class="key-btn" data-key="save">save</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn" data-key="settings">settings</button>
                <button class="key-btn" data-key="practice">practice</button>
                <button class="key-btn" data-key="theme">theme</button>
                <button class="key-btn" data-key="clear">clear</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn wide" data-key="backspace">⌫ Backspace</button>
                <button class="key-btn wide" data-key="enter">↵ Enter</button>
                <button class="key-btn" data-key="tab">⇥ Tab</button>
            </div>
        </div>

        <div class="keyboard-layout typing-layout">
            <div class="keyboard-row">
                <button class="key-btn" data-key="q">q</button>
                <button class="key-btn" data-key="w">w</button>
                <button class="key-btn" data-key="e">e</button>
                <button class="key-btn" data-key="r">r</button>
                <button class="key-btn" data-key="t">t</button>
                <button class="key-btn" data-key="y">y</button>
                <button class="key-btn" data-key="u">u</button>
                <button class="key-btn" data-key="i">i</button>
                <button class="key-btn" data-key="o">o</button>
                <button class="key-btn" data-key="p">p</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn" data-key="a">a</button>
                <button class="key-btn" data-key="s">s</button>
                <button class="key-btn" data-key="d">d</button>
                <button class="key-btn" data-key="f">f</button>
                <button class="key-btn" data-key="g">g</button>
                <button class="key-btn" data-key="h">h</button>
                <button class="key-btn" data-key="j">j</button>
                <button class="key-btn" data-key="k">k</button>
                <button class="key-btn" data-key="l">l</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn" data-key="z">z</button>
                <button class="key-btn" data-key="x">x</button>
                <button class="key-btn" data-key="c">c</button>
                <button class="key-btn" data-key="v">v</button>
                <button class="key-btn" data-key="b">b</button>
                <button class="key-btn" data-key="n">n</button>
                <button class="key-btn" data-key="m">m</button>
                <button class="key-btn" data-key="backspace">⌫</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn wide" data-key="space">Space</button>
                <button class="key-btn wide" data-key="enter">↵ Enter</button>
            </div>
        </div>

        <div class="keyboard-layout numbers-layout">
            <div class="keyboard-row">
                <button class="key-btn" data-key="1">1</button>
                <button class="key-btn" data-key="2">2</button>
                <button class="key-btn" data-key="3">3</button>
                <button class="key-btn" data-key="4">4</button>
                <button class="key-btn" data-key="5">5</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn" data-key="6">6</button>
                <button class="key-btn" data-key="7">7</button>
                <button class="key-btn" data-key="8">8</button>
                <button class="key-btn" data-key="9">9</button>
                <button class="key-btn" data-key="0">0</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn" data-key="{">{ }</button>
                <button class="key-btn" data-key="_">_</button>
                <button class="key-btn" data-key="-">-</button>
                <button class="key-btn" data-key="=">=</button>
                <button class="key-btn" data-key="+">+</button>
            </div>
            <div class="keyboard-row">
                <button class="key-btn wide" data-key="backspace">⌫ Backspace</button>
                <button class="key-btn wide" data-key="enter">↵ Enter</button>
            </div>
        </div>
    `;

    gameContainer.appendChild(keyboard);

    // Add event listeners for all interactions
    keyboard.addEventListener("click", (event) => {
        event.preventDefault();

        if (
            event.target.classList.contains("key-btn") ||
            event.target.classList.contains("keyboard-close-btn")
        ) {
            const key = event.target.dataset.key;
            handleVirtualKeyPress(key);
        } else if (event.target.classList.contains("keyboard-mode-btn")) {
            switchKeyboardMode(event.target.dataset.mode);
        }
    });

    // Add touch event handling for better responsiveness
    keyboard.addEventListener(
        "touchstart",
        (event) => {
            if (event.target.classList.contains("key-btn")) {
                event.target.classList.add("active");
            }
        },
        { passive: true }
    );

    keyboard.addEventListener(
        "touchend",
        (event) => {
            if (event.target.classList.contains("key-btn")) {
                event.target.classList.remove("active");
            }
        },
        { passive: true }
    );
}

// Switch between keyboard modes
function switchKeyboardMode(mode) {
    const keyboard = document.querySelector(".mobile-keyboard");
    if (!keyboard) return;

    // Update mode buttons
    keyboard.querySelectorAll(".keyboard-mode-btn").forEach((btn) => {
        btn.classList.toggle("active", btn.dataset.mode === mode);
    });

    // Update layouts
    keyboard.querySelectorAll(".keyboard-layout").forEach((layout) => {
        layout.classList.toggle(
            "active",
            layout.classList.contains(`${mode}-layout`)
        );
    });

    // Haptic feedback for mode switch
    if (navigator.vibrate) {
        navigator.vibrate([50, 25, 50]);
    }
}

// Enhanced virtual key press handling with better input management
function handleVirtualKeyPress(key) {
    if (!terminal) return;

    // Enhanced haptic feedback patterns
    const hapticPatterns = {
        key: [30],
        backspace: [50, 25, 50],
        enter: [100],
        toggle: [75, 50, 75],
    };

    switch (key) {
        case "backspace":
            if (terminal._currentInput && terminal._cursorPosition > 0) {
                terminal._currentInput =
                    terminal._currentInput.slice(
                        0,
                        terminal._cursorPosition - 1
                    ) + terminal._currentInput.slice(terminal._cursorPosition);
                terminal._cursorPosition--;
                updateInputLine();

                // Haptic feedback
                if (navigator.vibrate) {
                    navigator.vibrate(hapticPatterns.backspace);
                }
            }
            break;

        case "enter":
            if (terminal._currentInput !== undefined) {
                processCommand(terminal._currentInput.trim());
                terminal._currentInput = "";
                terminal._cursorPosition = 0;
            }

            if (navigator.vibrate) {
                navigator.vibrate(hapticPatterns.enter);
            }
            break;

        case "space":
            terminal._currentInput =
                terminal._currentInput.slice(0, terminal._cursorPosition) +
                " " +
                terminal._currentInput.slice(terminal._cursorPosition);
            terminal._cursorPosition++;
            updateInputLine();

            if (navigator.vibrate) {
                navigator.vibrate(hapticPatterns.key);
            }
            break;

        case "clear":
            terminal._currentInput = "";
            terminal._cursorPosition = 0;
            updateInputLine();

            if (navigator.vibrate) {
                navigator.vibrate([100, 50, 100]);
            }
            break;

        case "toggle":
            toggleVirtualKeyboard();

            if (navigator.vibrate) {
                navigator.vibrate(hapticPatterns.toggle);
            }
            break;

        case "tab":
            // Implement tab completion for mobile
            handleMobileTabCompletion();
            break;

        default:
            // Type the key/command
            if (key.length === 1) {
                // Single character
                terminal._currentInput =
                    terminal._currentInput.slice(0, terminal._cursorPosition) +
                    key +
                    terminal._currentInput.slice(terminal._cursorPosition);
                terminal._cursorPosition++;
            } else {
                // Command shortcut
                terminal._currentInput = key;
                terminal._cursorPosition = key.length;
            }
            updateInputLine();

            if (navigator.vibrate) {
                navigator.vibrate(hapticPatterns.key);
            }
            break;
    }
}

// Mobile tab completion implementation
function handleMobileTabCompletion() {
    if (!terminal._currentInput) return;

    const input = terminal._currentInput.toLowerCase();
    const availableCommands = [
        "help",
        "tutorial",
        "stats",
        "progress",
        "challenges",
        "list",
        "start",
        "challenge",
        "hint",
        "skip",
        "answer",
        "submit",
        "save",
        "load",
        "export",
        "import",
        "achievements",
        "theme",
        "clear",
        "settings",
        "practice",
    ];

    const matches = availableCommands.filter((cmd) => cmd.startsWith(input));

    if (matches.length === 1) {
        // Single match - complete it
        terminal._currentInput = matches[0];
        terminal._cursorPosition = matches[0].length;
        updateInputLine();

        // Haptic feedback for completion
        if (navigator.vibrate) {
            navigator.vibrate([50, 25, 50]);
        }
    } else if (matches.length > 1) {
        // Multiple matches - show them briefly
        showMobileCompletionHints(matches);
    }
}

// Show completion hints on mobile
function showMobileCompletionHints(matches) {
    const hintsContainer =
        document.querySelector(".mobile-completion-hints") ||
        createMobileCompletionHints();

    hintsContainer.innerHTML = "";
    matches.slice(0, 5).forEach((match) => {
        const hint = document.createElement("button");
        hint.className = "completion-hint-btn";
        hint.textContent = match;
        hint.onclick = () => {
            terminal._currentInput = match;
            terminal._cursorPosition = match.length;
            updateInputLine();
            hintsContainer.style.display = "none";
        };
        hintsContainer.appendChild(hint);
    });

    hintsContainer.style.display = "flex";

    // Auto-hide after 3 seconds
    setTimeout(() => {
        hintsContainer.style.display = "none";
    }, 3000);
}

// Create mobile completion hints container
function createMobileCompletionHints() {
    const container = document.createElement("div");
    container.className = "mobile-completion-hints";
    container.style.cssText = `
        position: fixed;
        bottom: 200px;
        left: 10px;
        right: 10px;
        display: none;
        flex-wrap: wrap;
        gap: 5px;
        z-index: 1001;
        padding: 10px;
        background: var(--bg-secondary);
        border: 1px solid var(--border-color);
        border-radius: 5px;
    `;

    document.body.appendChild(container);
    return container;
}

// Toggle virtual keyboard
function toggleVirtualKeyboard() {
    const keyboard = document.querySelector(".mobile-keyboard");
    keyboard.classList.toggle("active");
    virtualKeyboardVisible = keyboard.classList.contains("active");

    // Adjust terminal container padding
    const terminalContainer = document.querySelector(".terminal-container");
    if (virtualKeyboardVisible) {
        terminalContainer.style.paddingBottom = "200px";
    } else {
        terminalContainer.style.paddingBottom = isMobile ? "120px" : "20px";
    }
}

// Enhanced touch gesture support with multi-touch and gesture recognition
function addTouchGestures() {
    const gameContainer = document.querySelector(".game-container");
    let gestureState = {
        startX: 0,
        startY: 0,
        startTime: 0,
        touchCount: 0,
        isScrolling: false,
        lastTap: 0,
        gestureStarted: false,
    };

    gameContainer.addEventListener(
        "touchstart",
        (event) => {
            handleTouchStart(event, gestureState);
        },
        { passive: false }
    );

    gameContainer.addEventListener(
        "touchmove",
        (event) => {
            handleTouchMove(event, gestureState);
        },
        { passive: false }
    );

    gameContainer.addEventListener(
        "touchend",
        (event) => {
            handleTouchEnd(event, gestureState);
        },
        { passive: false }
    );

    // Add gesture hints
    showGestureHints();

    // Add pinch-to-zoom detection for terminal font scaling
    addPinchGestures();
}

// Enhanced touch start handling
function handleTouchStart(event, gestureState) {
    const touch = event.touches[0];
    gestureState.startX = touch.clientX;
    gestureState.startY = touch.clientY;
    gestureState.startTime = Date.now();
    gestureState.touchCount = event.touches.length;
    gestureState.isScrolling = false;
    gestureState.gestureStarted = true;

    // Handle double-tap detection
    const now = Date.now();
    const timeBetweenTaps = now - gestureState.lastTap;

    if (timeBetweenTaps < 300 && timeBetweenTaps > 0) {
        // Double tap detected
        handleDoubleTap(touch.clientX, touch.clientY);
        event.preventDefault();
        return;
    }

    gestureState.lastTap = now;

    // Multi-touch gestures
    if (gestureState.touchCount === 2) {
        // Two finger gesture - prepare for pinch or two-finger scroll
        handleTwoFingerStart(event);
    }
}

// Enhanced touch move handling with gesture recognition
function handleTouchMove(event, gestureState) {
    if (!gestureState.gestureStarted) return;

    const touch = event.touches[0];
    const deltaX = touch.clientX - gestureState.startX;
    const deltaY = touch.clientY - gestureState.startY;

    // Determine if this is a scroll or swipe gesture
    if (Math.abs(deltaY) > Math.abs(deltaX) && Math.abs(deltaY) > 10) {
        gestureState.isScrolling = true;
    }

    // Allow scrolling in terminal area
    const target = event.target;
    if (target.closest(".terminal-container") && gestureState.isScrolling) {
        return; // Allow default scrolling
    }

    // Handle two-finger gestures
    if (gestureState.touchCount === 2 && event.touches.length === 2) {
        handleTwoFingerMove(event);
        event.preventDefault();
        return;
    }

    // Prevent other gestures during scrolling
    if (gestureState.isScrolling && !target.closest(".terminal-container")) {
        event.preventDefault();
    }
}

// Enhanced touch end handling with gesture completion
function handleTouchEnd(event, gestureState) {
    if (!gestureState.gestureStarted) return;

    const touch = event.changedTouches[0];
    const deltaX = touch.clientX - gestureState.startX;
    const deltaY = touch.clientY - gestureState.startY;
    const duration = Date.now() - gestureState.startTime;
    const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

    // Reset gesture state
    gestureState.gestureStarted = false;

    // Ignore if this was scrolling
    if (gestureState.isScrolling) return;

    // Detect swipe gestures
    if (duration < 500 && distance > 50) {
        const angle = (Math.atan2(deltaY, deltaX) * 180) / Math.PI;
        handleSwipeGesture(angle, distance, duration);
        event.preventDefault();
        return;
    }

    // Detect tap gestures
    if (duration < 300 && distance < 20) {
        handleTapGesture(touch.clientX, touch.clientY);
    }
}

// Handle swipe gestures with improved recognition
function handleSwipeGesture(angle, distance, duration) {
    const velocity = distance / duration;

    // Determine swipe direction
    let direction;
    if (angle > -45 && angle <= 45) {
        direction = "right";
    } else if (angle > 45 && angle <= 135) {
        direction = "down";
    } else if (angle > 135 || angle <= -135) {
        direction = "left";
    } else {
        direction = "up";
    }

    // Execute swipe actions
    switch (direction) {
        case "up":
            if (velocity > 0.5) {
                // Fast swipe up - show virtual keyboard
                if (!virtualKeyboardVisible) {
                    toggleVirtualKeyboard();
                    showGestureConfirmation("Virtual keyboard opened");
                }
            } else {
                // Slow swipe up - scroll to top
                scrollTerminalToTop();
            }
            break;

        case "down":
            if (velocity > 0.5) {
                // Fast swipe down - hide virtual keyboard or show stats
                if (virtualKeyboardVisible) {
                    toggleVirtualKeyboard();
                    showGestureConfirmation("Virtual keyboard closed");
                } else {
                    processCommand("stats");
                    showGestureConfirmation("Showing statistics");
                }
            }
            break;

        case "left":
            // Swipe left - navigate command history backward
            if (commandHistory.length > 0) {
                navigateHistory(1);
                showGestureConfirmation("Previous command");
            }
            break;

        case "right":
            // Swipe right - navigate command history forward or show help
            if (historyIndex > 0) {
                navigateHistory(-1);
                showGestureConfirmation("Next command");
            } else {
                processCommand("help");
                showGestureConfirmation("Showing help");
            }
            break;
    }

    // Haptic feedback for swipe
    if (navigator.vibrate) {
        navigator.vibrate([75, 25, 75]);
    }
}

// Handle double-tap gestures
function handleDoubleTap(x, y) {
    const target = document.elementFromPoint(x, y);

    if (target.closest(".terminal-container")) {
        // Double-tap in terminal - toggle virtual keyboard
        toggleVirtualKeyboard();
        showGestureConfirmation("Virtual keyboard toggled");
    } else if (target.closest(".controls")) {
        // Double-tap in controls - quick save
        processCommand("save");
        showGestureConfirmation("Game saved");
    } else {
        // Double-tap elsewhere - show quick actions
        showQuickActions(x, y);
    }

    // Haptic feedback for double-tap
    if (navigator.vibrate) {
        navigator.vibrate([50, 25, 50, 25, 50]);
    }
}

// Handle single tap gestures
function handleTapGesture(x, y) {
    const target = document.elementFromPoint(x, y);

    if (target.closest(".terminal-container") && terminal) {
        // Tap in terminal - focus for input
        terminal.focus();
    }
}

// Two-finger gesture handling for advanced interactions
function handleTwoFingerStart(event) {
    // Store initial two-finger position for pinch/zoom detection
    const touch1 = event.touches[0];
    const touch2 = event.touches[1];

    twoFingerState = {
        initialDistance: Math.sqrt(
            Math.pow(touch2.clientX - touch1.clientX, 2) +
                Math.pow(touch2.clientY - touch1.clientY, 2)
        ),
        initialCenterX: (touch1.clientX + touch2.clientX) / 2,
        initialCenterY: (touch1.clientY + touch2.clientY) / 2,
        startTime: Date.now(),
    };
}

function handleTwoFingerMove(event) {
    if (!twoFingerState) return;

    const touch1 = event.touches[0];
    const touch2 = event.touches[1];

    const currentDistance = Math.sqrt(
        Math.pow(touch2.clientX - touch1.clientX, 2) +
            Math.pow(touch2.clientY - touch1.clientY, 2)
    );

    const scale = currentDistance / twoFingerState.initialDistance;

    // Handle pinch-to-zoom for font size
    if (Math.abs(scale - 1) > 0.1) {
        handlePinchZoom(scale);
    }
}

// Pinch gesture handling for font scaling
let twoFingerState = null;

function addPinchGestures() {
    // This is handled in the two-finger gesture system above
}

function handlePinchZoom(scale) {
    if (!terminal) return;

    const currentFontSize = terminal.options.fontSize || 14;
    let newFontSize = currentFontSize;

    if (scale > 1.1) {
        // Pinch out - increase font size
        newFontSize = Math.min(currentFontSize + 1, 20);
    } else if (scale < 0.9) {
        // Pinch in - decrease font size
        newFontSize = Math.max(currentFontSize - 1, 10);
    }

    if (newFontSize !== currentFontSize) {
        terminal.options.fontSize = newFontSize;

        // Save font preference
        localStorage.setItem("terminal-font-size", newFontSize);

        showGestureConfirmation(`Font size: ${newFontSize}px`);

        // Haptic feedback
        if (navigator.vibrate) {
            navigator.vibrate([25, 25, 25]);
        }
    }
}

// Show gesture confirmation feedback
function showGestureConfirmation(message) {
    const existing = document.querySelector(".gesture-feedback");
    if (existing) existing.remove();

    const feedback = document.createElement("div");
    feedback.className = "gesture-feedback";
    feedback.textContent = message;
    feedback.style.cssText = `
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: var(--bg-secondary);
        color: var(--text-accent);
        padding: 10px 20px;
        border: 1px solid var(--border-color);
        border-radius: 5px;
        font-size: 0.9em;
        z-index: 2000;
        pointer-events: none;
        opacity: 0;
        transition: opacity 0.3s;
    `;

    document.body.appendChild(feedback);

    // Animate in
    setTimeout(() => (feedback.style.opacity = "1"), 10);

    // Remove after 2 seconds
    setTimeout(() => {
        feedback.style.opacity = "0";
        setTimeout(() => feedback.remove(), 300);
    }, 2000);
}

// Show quick action menu
function showQuickActions(x, y) {
    const existing = document.querySelector(".quick-actions");
    if (existing) existing.remove();

    const actions = document.createElement("div");
    actions.className = "quick-actions";
    actions.style.cssText = `
        position: fixed;
        top: ${y - 100}px;
        left: ${Math.max(10, Math.min(x - 75, window.innerWidth - 160))}px;
        background: var(--bg-secondary);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 5px;
        z-index: 2000;
        display: flex;
        flex-direction: column;
        gap: 2px;
        min-width: 150px;
        max-width: 200px;
    `;

    const quickCommands = [
        { text: "📊 Stats", command: "stats" },
        { text: "💡 Help", command: "help" },
        { text: "🎮 Challenges", command: "challenges" },
        { text: "💾 Save", command: "save" },
        { text: "🎨 Theme", command: "theme" },
    ];

    quickCommands.forEach(({ text, command }) => {
        const btn = document.createElement("button");
        btn.className = "quick-action-btn";
        btn.textContent = text;
        btn.style.cssText = `
            background: var(--bg-primary);
            color: var(--text-primary);
            border: none;
            padding: 8px 12px;
            font-size: 0.9em;
            cursor: pointer;
            border-radius: 4px;
            transition: background-color 0.2s;
        `;

        btn.addEventListener("touchstart", () => {
            btn.style.backgroundColor = "var(--text-accent)";
            btn.style.color = "var(--bg-primary)";
        });

        btn.addEventListener("touchend", () => {
            processCommand(command);
            actions.remove();
        });

        actions.appendChild(btn);
    });

    document.body.appendChild(actions);

    // Remove after 5 seconds or on outside touch
    setTimeout(() => {
        if (actions.parentNode) actions.remove();
    }, 5000);

    document.addEventListener("touchstart", function removeQuickActions(e) {
        if (!actions.contains(e.target)) {
            actions.remove();
            document.removeEventListener("touchstart", removeQuickActions);
        }
    });
}

// Scroll terminal to top
function scrollTerminalToTop() {
    if (terminal && terminal.element) {
        const viewport = terminal.element.querySelector(".xterm-viewport");
        if (viewport) {
            viewport.scrollTop = 0;
            showGestureConfirmation("Scrolled to top");
        }
    }
}

function showGestureHints() {
    if (!isMobile) return;

    const hints = [
        "👆 Swipe up: Virtual keyboard",
        "👇 Swipe down: Stats / Hide keyboard",
        "👈 Swipe left: Previous command",
        "👉 Swipe right: Next command / Help",
        "👆👆 Double-tap: Quick keyboard toggle",
        "👆👆 Two fingers: Font size control",
    ];

    let currentHintIndex = 0;

    function showNextHint() {
        if (currentHintIndex >= hints.length) return;

        const hint = document.createElement("div");
        hint.className = "gesture-hint";
        hint.textContent = hints[currentHintIndex];
        hint.style.cssText = `
            position: fixed;
            top: 20px;
            left: 50%;
            transform: translateX(-50%);
            background: var(--bg-secondary);
            color: var(--text-accent);
            padding: 8px 16px;
            border: 1px solid var(--border-color);
            border-radius: 20px;
            font-size: 0.85em;
            z-index: 1500;
            opacity: 0;
            transition: opacity 0.3s;
            pointer-events: none;
            max-width: 90%;
            text-align: center;
        `;

        document.body.appendChild(hint);

        // Animate in
        setTimeout(() => (hint.style.opacity = "1"), 100);

        // Animate out and remove
        setTimeout(() => {
            hint.style.opacity = "0";
            setTimeout(() => {
                hint.remove();
                currentHintIndex++;
                // Show next hint after a delay
                if (currentHintIndex < hints.length) {
                    setTimeout(showNextHint, 1000);
                }
            }, 300);
        }, 2500);
    }

    // Start showing hints after a delay
    setTimeout(showNextHint, 2000);
}

// Handle device keyboard visibility changes
function handleVirtualKeyboard() {
    if (!isMobile) return;

    let initialViewportHeight = window.innerHeight;

    window.addEventListener("resize", () => {
        const currentHeight = window.innerHeight;
        const heightDifference = initialViewportHeight - currentHeight;

        const terminalContainer = document.querySelector(".terminal-container");
        const controls = document.querySelector(".controls");

        if (heightDifference > 150) {
            // Virtual keyboard is likely open
            document.body.classList.add("keyboard-open");
            terminalContainer.style.paddingBottom = "10px";
            controls.style.display = "none";
        } else {
            // Virtual keyboard is likely closed
            document.body.classList.remove("keyboard-open");
            terminalContainer.style.paddingBottom = "120px";
            controls.style.display = "flex";
        }
    });
}

// Pull-to-refresh functionality
function addPullToRefresh() {
    if (!isMobile) return;

    let startY = 0;
    let currentY = 0;
    let pullDistance = 0;
    let isPulling = false;
    let refreshThreshold = 80;

    const refreshIndicator = createRefreshIndicator();

    const terminalContainer = document.querySelector(".terminal-container");

    terminalContainer.addEventListener(
        "touchstart",
        (e) => {
            // Only trigger pull-to-refresh at the top of the terminal
            const terminal = document.querySelector("#terminal");
            const terminalViewport = terminal?.querySelector(".xterm-viewport");

            if (terminalViewport && terminalViewport.scrollTop <= 0) {
                startY = e.touches[0].clientY;
                isPulling = false;
            }
        },
        { passive: true }
    );

    terminalContainer.addEventListener(
        "touchmove",
        (e) => {
            if (startY === 0) return;

            currentY = e.touches[0].clientY;
            pullDistance = currentY - startY;

            // Only activate pull-to-refresh when pulling down
            if (pullDistance > 0) {
                isPulling = true;

                // Show refresh indicator
                const progress = Math.min(pullDistance / refreshThreshold, 1);
                updateRefreshIndicator(
                    refreshIndicator,
                    progress,
                    pullDistance >= refreshThreshold
                );

                // Add some resistance
                if (pullDistance > refreshThreshold) {
                    const resistance =
                        1 -
                        Math.min((pullDistance - refreshThreshold) / 100, 0.8);
                    terminalContainer.style.transform = `translateY(${
                        refreshThreshold +
                        (pullDistance - refreshThreshold) * resistance
                    }px)`;
                } else {
                    terminalContainer.style.transform = `translateY(${pullDistance}px)`;
                }

                // Prevent default scrolling
                e.preventDefault();
            }
        },
        { passive: false }
    );

    terminalContainer.addEventListener(
        "touchend",
        () => {
            if (isPulling) {
                terminalContainer.style.transform = "";
                terminalContainer.style.transition = "transform 0.3s ease";

                if (pullDistance >= refreshThreshold) {
                    // Trigger refresh
                    triggerRefresh(refreshIndicator);
                } else {
                    // Hide indicator
                    hideRefreshIndicator(refreshIndicator);
                }

                // Reset transition after animation
                setTimeout(() => {
                    terminalContainer.style.transition = "";
                }, 300);
            }

            startY = 0;
            currentY = 0;
            pullDistance = 0;
            isPulling = false;
        },
        { passive: true }
    );
}

function createRefreshIndicator() {
    const indicator = document.createElement("div");
    indicator.className = "pull-refresh-indicator";
    indicator.style.cssText = `
        position: fixed;
        top: -60px;
        left: 50%;
        transform: translateX(-50%);
        width: 40px;
        height: 40px;
        background: var(--bg-secondary);
        border: 2px solid var(--border-color);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        transition: top 0.2s, transform 0.2s;
        font-size: 1.2em;
    `;
    indicator.innerHTML = "↓";
    document.body.appendChild(indicator);
    return indicator;
}

function updateRefreshIndicator(indicator, progress, canRefresh) {
    const rotation = progress * 180;
    const topPosition = Math.min(-60 + progress * 80, 20);

    indicator.style.top = `${topPosition}px`;
    indicator.style.transform = `translateX(-50%) rotate(${rotation}deg)`;

    if (canRefresh) {
        indicator.style.borderColor = "var(--text-accent)";
        indicator.style.color = "var(--text-accent)";
        indicator.innerHTML = "↻";
    } else {
        indicator.style.borderColor = "var(--border-color)";
        indicator.style.color = "var(--text-primary)";
        indicator.innerHTML = "↓";
    }
}

function triggerRefresh(indicator) {
    // Show loading state
    indicator.style.top = "20px";
    indicator.style.animation = "spin 1s linear infinite";
    indicator.innerHTML = "⟲";

    // Simulate refresh actions
    Promise.all([
        // Refresh game state
        new Promise((resolve) => {
            updateUI();
            resolve();
        }),

        // Check for updates
        new Promise((resolve) => {
            if (
                "serviceWorker" in navigator &&
                navigator.serviceWorker.controller
            ) {
                navigator.serviceWorker.controller.postMessage({
                    type: "CHECK_FOR_UPDATES",
                });
            }
            setTimeout(resolve, 1000);
        }),

        // Sync offline data
        new Promise((resolve) => {
            if (navigator.onLine) {
                // Trigger background sync
                if (
                    "serviceWorker" in navigator &&
                    "sync" in window.ServiceWorkerRegistration.prototype
                ) {
                    navigator.serviceWorker.ready
                        .then((registration) => {
                            return registration.sync.register("game-save-sync");
                        })
                        .catch(() => {});
                }
            }
            setTimeout(resolve, 500);
        }),
    ])
        .then(() => {
            // Show success feedback
            showRefreshSuccess(indicator);
        })
        .catch(() => {
            // Show error feedback
            showRefreshError(indicator);
        });
}

function showRefreshSuccess(indicator) {
    indicator.style.animation = "";
    indicator.style.borderColor = "var(--text-accent)";
    indicator.style.color = "var(--text-accent)";
    indicator.innerHTML = "✓";

    // Show success message in terminal
    if (terminal) {
        writeSuccess("Content refreshed successfully!");
    }

    // Hide after delay
    setTimeout(() => hideRefreshIndicator(indicator), 1500);
}

function showRefreshError(indicator) {
    indicator.style.animation = "";
    indicator.style.borderColor = "var(--text-danger)";
    indicator.style.color = "var(--text-danger)";
    indicator.innerHTML = "✗";

    // Show error message
    if (terminal) {
        writeWarning("Refresh failed - check your connection");
    }

    // Hide after delay
    setTimeout(() => hideRefreshIndicator(indicator), 2000);
}

function hideRefreshIndicator(indicator) {
    indicator.style.top = "-60px";
    indicator.style.borderColor = "var(--border-color)";
    indicator.style.color = "var(--text-primary)";
    indicator.innerHTML = "↓";
}

// CSS for refresh indicator animation
const refreshStyles = document.createElement("style");
refreshStyles.textContent = `
    @keyframes spin {
        from { transform: translateX(-50%) rotate(0deg); }
        to { transform: translateX(-50%) rotate(360deg); }
    }
`;
document.head.appendChild(refreshStyles);

// Optimize terminal for mobile
function optimizeTerminalForMobile() {
    if (!terminal || !isMobile) return;

    // Adjust font size for mobile
    terminal.options.fontSize = window.innerWidth < 480 ? 11 : 12;

    // Enable touch scrolling
    terminal.options.scrollback = 1000;

    // Optimize cursor for touch
    terminal.options.cursorBlink = true;
    terminal.options.cursorStyle = "block";

    // Add touch-friendly input handling
    const terminalElement = document.querySelector("#terminal");
    if (terminalElement) {
        terminalElement.addEventListener("touchstart", (event) => {
            // Focus on touch for better keyboard handling
            terminal.focus();

            // Double-tap to show virtual keyboard
            if (event.touches.length === 1) {
                const now = Date.now();
                const lastTap = terminalElement.dataset.lastTap || 0;

                if (now - lastTap < 300) {
                    toggleVirtualKeyboard();
                }

                terminalElement.dataset.lastTap = now;
            }
        });
    }
}

async function installPWA() {
    if (!deferredPrompt) return;

    deferredPrompt.prompt();
    const result = await deferredPrompt.userChoice;

    if (result.outcome === "accepted") {
        console.log("🎉 PWA installed successfully");
        achievements.push("pwa_installer");
        saveAchievements();

        // Remove install button
        const installBtn = document.querySelector(".install-btn");
        if (installBtn) installBtn.remove();
    }

    deferredPrompt = null;
}

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

// Enhanced initialization with lazy loading and performance optimization
async function init() {
    try {
        const startTime = performance.now();

        // Update loading progress
        updateLoadingProgress(5, "Initializing...");

        // Detect mobile early for optimization decisions
        const isMobileDevice = detectMobile();

        // Lazy load WebAssembly with performance optimization
        updateLoadingProgress(10, "Loading core engine...");

        // Load WASM with streaming compilation if supported
        const wasmPromise = loadWebAssemblyOptimized();

        // Initialize UI components while WASM loads
        updateLoadingProgress(20, "Setting up interface...");
        initializeTerminal();

        // Wait for WASM to complete
        wasmModule = await wasmPromise;

        updateLoadingProgress(40, "Starting game engine...");

        // Initialize game engine and state
        gameEngine = new wasmModule.WebGameEngine();
        gameState = new wasmModule.WebGameState();

        updateLoadingProgress(60, "Loading saved progress...");

        // Load saved game and preferences
        await Promise.all([
            loadSavedGame(),
            loadAchievements(),
            loadUserPreferences(),
        ]);

        updateLoadingProgress(75, "Initializing features...");

        // Initialize audio (non-blocking)
        initializeAudio();

        // Cache critical data for offline play
        if (navigator.onLine) {
            cacheOfflineChallenges();
        }

        updateLoadingProgress(85, "Starting game...");

        // Start the game
        startGame();

        updateLoadingProgress(95, "Finalizing...");

        // Initialize mobile features if needed
        if (isMobileDevice) {
            console.log("📱 Mobile device detected, optimizing interface");
            // Delay mobile initialization to prevent blocking
            setTimeout(initializeMobileFeatures, 100);
        }

        updateLoadingProgress(100, "Ready!");

        // Performance metrics
        const loadTime = performance.now() - startTime;
        console.log(`🚀 Game initialized in ${loadTime.toFixed(2)}ms`);

        // Hide loading screen with smooth transition
        await hideLoadingScreen();

        // Post-initialization optimizations
        setTimeout(postInitOptimizations, 1000);
    } catch (error) {
        console.error("Failed to initialize game:", error);
        showError("Failed to load game. Please refresh the page.");
    }
}

// Optimized WebAssembly loading with streaming compilation
async function loadWebAssemblyOptimized() {
    try {
        // Try streaming compilation first (faster on supported browsers)
        if (
            "compileStreaming" in WebAssembly &&
            "instantiateStreaming" in WebAssembly
        ) {
            console.log("🚀 Using streaming WebAssembly compilation");

            // Import the JS module first
            const wasmModule = await import("./pkg/hack_simulator.js");

            // Stream compile the WASM in parallel
            const wasmUrl = "./pkg/hack_simulator_bg.wasm";
            await wasmModule.default(wasmUrl);

            return wasmModule;
        } else {
            // Fallback to standard loading
            console.log("📦 Using standard WebAssembly loading");
            const wasmModule = await import("./pkg/hack_simulator.js");
            await wasmModule.default();
            return wasmModule;
        }
    } catch (error) {
        console.warn("WebAssembly streaming failed, falling back:", error);
        // Final fallback
        const wasmModule = await import("./pkg/hack_simulator.js");
        await wasmModule.default();
        return wasmModule;
    }
}

// Load user preferences asynchronously
async function loadUserPreferences() {
    try {
        const fontSize = localStorage.getItem("terminal-font-size");
        if (fontSize && terminal) {
            terminal.options.fontSize = parseInt(fontSize);
        }

        const theme = localStorage.getItem("preferred-theme");
        if (theme) {
            currentTheme = theme;
        }

        // Load other preferences from service worker storage
        if (
            "serviceWorker" in navigator &&
            navigator.serviceWorker.controller
        ) {
            navigator.serviceWorker.controller.postMessage({
                type: "LOAD_PREFERENCES",
            });
        }
    } catch (error) {
        console.warn("Failed to load user preferences:", error);
    }
}

// Smooth loading screen hide with performance monitoring
async function hideLoadingScreen() {
    return new Promise((resolve) => {
        const loading = document.getElementById("loading");
        const gameContainer = document.getElementById("gameContainer");

        // Fade out loading screen
        loading.style.transition = "opacity 0.5s ease";
        loading.style.opacity = "0";

        setTimeout(() => {
            loading.style.display = "none";
            gameContainer.style.display = "flex";

            // Fade in game container
            gameContainer.style.opacity = "0";
            gameContainer.style.transition = "opacity 0.3s ease";

            requestAnimationFrame(() => {
                gameContainer.style.opacity = "1";
                resolve();
            });
        }, 500);
    });
}

// Post-initialization performance optimizations
function postInitOptimizations() {
    // Preload commonly used commands for faster execution
    preloadCommands(["help", "stats", "challenges", "save"]);

    // Initialize background caching
    if (navigator.onLine) {
        cacheOptionalAssets();
    }

    // Set up performance monitoring
    setupPerformanceMonitoring();

    // Clean up any temporary loading assets
    cleanupLoadingAssets();

    console.log("✨ Post-initialization optimizations complete");
}

// Preload command data for faster response
function preloadCommands(commands) {
    try {
        for (const command of commands) {
            switch (command) {
                case "challenges":
                    if (gameEngine) {
                        gameEngine.get_challenges_json(); // Warm up the cache
                    }
                    break;
                case "stats":
                    if (gameState) {
                        gameState.get_stats_json(); // Warm up the cache
                    }
                    break;
            }
        }
        console.log("⚡ Preloaded common commands for faster response");
    } catch (error) {
        console.warn("Command preloading failed:", error);
    }
}

// Cache optional assets in the background
async function cacheOptionalAssets() {
    try {
        // Cache external dependencies that might not be cached yet
        const optionalAssets = [
            "https://cdn.jsdelivr.net/npm/xterm@5.3.0/lib/xterm.js",
            "https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.css",
        ];

        for (const asset of optionalAssets) {
            fetch(asset, { mode: "no-cors" }).catch(() => {
                // Silently fail - these are optional
            });
        }

        console.log("🔄 Background asset caching initiated");
    } catch (error) {
        console.warn("Background caching failed:", error);
    }
}

function setupPerformanceMonitoring() {
    // Monitor long tasks (if supported)
    if ("PerformanceObserver" in window) {
        try {
            const observer = new PerformanceObserver((list) => {
                for (const entry of list.getEntries()) {
                    if (entry.duration > 50) {
                        // Tasks longer than 50ms
                        console.warn(
                            `⚠️ Long task detected: ${entry.duration.toFixed(
                                2
                            )}ms`
                        );

                        // Adjust performance mode if needed
                        if (entry.duration > 100 && isMobile) {
                            enablePerformanceMode();
                        }
                    }
                }
            });

            observer.observe({ entryTypes: ["longtask"] });
        } catch (error) {
            // Performance Observer not fully supported
        }
    }
}

function cleanupLoadingAssets() {
    // Remove any temporary loading elements
    const tempElements = document.querySelectorAll(
        ".temp-loading, .loading-temp"
    );
    tempElements.forEach((el) => el.remove());

    // Clear loading-related variables
    if (window.loadingInterval) {
        clearInterval(window.loadingInterval);
    }
}

function updateLoadingProgress(percent, message) {
    const progress = document.getElementById("loadingProgress");
    const percentDisplay = document.getElementById("loadingPercent");
    const statusDisplay = document.getElementById("loadingStatus");
    const tipDisplay = document.getElementById("loadingTip");

    if (progress) {
        progress.style.width = `${percent}%`;
    }

    if (percentDisplay) {
        percentDisplay.textContent = `${percent}%`;
    }

    // Update loading message if provided
    if (message && statusDisplay) {
        statusDisplay.textContent = message;
    }

    // Rotate tips during loading
    if (tipDisplay && Math.random() < 0.3) {
        // 30% chance to change tip
        const tips = [
            "💡 Tip: Use 'hint' command when stuck on challenges",
            "💡 Tip: Your sanity decreases with each challenge",
            "💡 Tip: Tab completion works for all commands",
            "💡 Tip: Try different color themes with 'theme' command",
            "💡 Tip: Don't trust everything the ghost tells you...",
            "💡 Tip: Some challenges have hidden clues in descriptions",
            "💡 Tip: Save your progress frequently!",
            "💡 Tip: Commands are case-insensitive",
            "💡 Tip: Check 'status' to see your progress",
            "💡 Tip: The ghost knows more than it reveals...",
        ];
        tipDisplay.textContent = tips[Math.floor(Math.random() * tips.length)];
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

    // Start atmospheric effects and ambient audio
    startAtmosphericEffects();

    // Start ambient audio now that everything is initialized
    if (audioContext) {
        setTimeout(() => startAmbientAudio(), 2000); // Delay slightly for better UX
    }

    updateUI();
    showPrompt();
}

function startAtmosphericEffects() {
    // Occasional subtle glitch effects
    setInterval(() => {
        if (
            Math.random() < 0.02 &&
            gameState &&
            typeof gameState.get_sanity === "function"
        ) {
            try {
                if (gameState.get_sanity() < 50) {
                    triggerSubtleGlitch();
                }
            } catch (error) {
                // Silently ignore if sanity can't be retrieved
            }
        }
    }, 5000);

    // Sanity-based effects
    setInterval(() => {
        if (gameState && typeof gameState.get_sanity === "function") {
            try {
                if (gameState.get_sanity() < 25) {
                    triggerSanityEffect();
                }
            } catch (error) {
                // Silently ignore if sanity can't be retrieved
            }
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
    const xpBar =
        "█".repeat(Math.floor(xpProgress)) +
        "░".repeat(20 - Math.floor(xpProgress));
    terminal.writeln(`   XP: ${stats.xp} [${xpBar}]`);
    // Sanity with colored bar
    const sanityPercent = stats.sanity;
    const sanityProgress = Math.floor(sanityPercent / 5); // 20 segments
    const sanityBar =
        "█".repeat(sanityProgress) + "░".repeat(20 - sanityProgress);
    const sanityColor =
        sanityPercent > 60 ? "🟢" : sanityPercent > 30 ? "🟡" : "🔴";
    writeAccent(`🧠 Sanity: ${stats.sanity}/100 ${sanityColor}`);
    terminal.writeln(`   [${sanityBar}]`);
    terminal.writeln("");

    // Challenge completion
    writeAccent(
        `🎮 Challenges Completed: ${stats.completed_challenges.length}/11`
    );
    const challengeProgress = Math.floor(
        (stats.completed_challenges.length / 11) * 20
    );
    const challengeBar =
        "█".repeat(challengeProgress) + "░".repeat(20 - challengeProgress);
    terminal.writeln(`   [${challengeBar}]`);
    terminal.writeln("");

    // Performance metrics
    writeAccent("📈 Performance Metrics:");
    terminal.writeln(`   Total Attempts: ${stats.total_attempts || 0}`);
    terminal.writeln(`   Hints Used: ${stats.hints_used || 0}`);
    const efficiency =
        stats.completed_challenges.length > 0
            ? (
                  (stats.total_attempts || 0) /
                  stats.completed_challenges.length
              ).toFixed(1)
            : "N/A";
    terminal.writeln(`   Avg Attempts per Challenge: ${efficiency}`);
    terminal.writeln("");

    // Achievement summary
    const unlockedCount = achievements.length;
    const totalAchievements = Object.keys(ACHIEVEMENTS).length;
    writeAccent(
        `🏆 Achievements: ${unlockedCount}/${totalAchievements} unlocked`
    );

    if (unlockedCount > 0) {
        terminal.writeln("   Recent achievements:");
        achievements.slice(-3).forEach((id) => {
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
        // Don't start ambient audio immediately - wait for game to fully initialize
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
    if (
        !audioContext ||
        !gameState ||
        typeof gameState.get_sanity !== "function"
    )
        return;

    try {
        if (gameState.get_sanity() > 70) return;
    } catch (error) {
        console.log("Could not get sanity level for ambient audio:", error);
        return;
    }

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

    const sanityLevel =
        gameState && typeof gameState.get_sanity === "function"
            ? gameState.get_sanity() / 100
            : 0.5;
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
    terminal.writeln(
        `• Challenges completed: ${stats.completed_challenges.length}/11`
    );
    terminal.writeln(`• Current level: ${stats.level}`);
    terminal.writeln(
        `• Achievements unlocked: ${achievements.length}/${
            Object.keys(ACHIEVEMENTS).length
        }`
    );
    terminal.writeln("");

    writeMuted(
        "🔒 Your privacy is protected - we never collect personal data!"
    );
    writeMuted("🎮 Keep playing to improve your ranking!");
}

function shareProgress() {
    const stats = JSON.parse(gameState.get_stats_json());
    const completionPercent = Math.floor(
        (stats.completed_challenges.length / 11) * 100
    );
    const achievementCount = achievements.length;

    const shareText = `🎮 I'm playing The Hack: Ghost Protocol!
📊 Progress: ${completionPercent}% complete (${
        stats.completed_challenges.length
    }/11 challenges)
🏆 Achievements: ${achievementCount}/${
        Object.keys(ACHIEVEMENTS).length
    } unlocked
💀 Sanity: ${stats.sanity}/100

Try it yourself at hack.andernet.dev - A horror-themed cybersecurity CTF game! 👻`;

    // Copy to clipboard
    try {
        navigator.clipboard
            .writeText(shareText)
            .then(() => {
                writeSuccess("📋 Progress shared to clipboard!");
                writeMuted("Paste this anywhere to share your achievements!");
            })
            .catch(() => {
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

// ============================================================================
// Branching Narrative System (v1.6.0)
// ============================================================================

/**
 * Show a narrative choice modal to the player
 * @param {Object} branch - The narrative branch object
 */
function showNarrativeChoice(branch) {
    const modal = document.getElementById('narrativeModal');
    const title = document.getElementById('narrativeTitle');
    const description = document.getElementById('narrativeDescription');
    const choicesContainer = document.getElementById('narrativeChoices');

    if (!modal || !title || !description || !choicesContainer) {
        console.error('Narrative modal elements not found');
        return;
    }

    // Set title and description
    title.textContent = branch.title;
    description.textContent = branch.description;

    // Clear previous choices
    choicesContainer.innerHTML = '';

    // Create choice buttons
    branch.choices.forEach((choice, index) => {
        const choiceBtn = document.createElement('button');
        choiceBtn.className = 'narrative-choice-btn';

        // Check if choice is available
        const isAvailable = checkChoiceRequirements(choice);
        if (!isAvailable) {
            choiceBtn.classList.add('locked');
            choiceBtn.disabled = true;
        }

        // Choice text
        const choiceText = document.createElement('div');
        choiceText.className = 'choice-text';
        choiceText.textContent = choice.text;
        choiceBtn.appendChild(choiceText);

        // Effects display
        const effectsDiv = document.createElement('div');
        effectsDiv.className = 'choice-effects';
        
        if (choice.effects) {
            if (choice.effects.sanity_change !== 0) {
                const sanityEffect = document.createElement('span');
                sanityEffect.className = 'effect-item effect-sanity';
                if (choice.effects.sanity_change > 0) {
                    sanityEffect.classList.add('positive');
                }
                const sign = choice.effects.sanity_change > 0 ? '+' : '';
                sanityEffect.textContent = `${sign}${choice.effects.sanity_change} Sanity`;
                effectsDiv.appendChild(sanityEffect);
            }

            if (choice.effects.xp_change > 0) {
                const xpEffect = document.createElement('span');
                xpEffect.className = 'effect-item effect-xp';
                xpEffect.textContent = `+${choice.effects.xp_change} XP`;
                effectsDiv.appendChild(xpEffect);
            }

            if (choice.effects.unlock_challenges && choice.effects.unlock_challenges.length > 0) {
                const unlockEffect = document.createElement('span');
                unlockEffect.className = 'effect-item effect-unlock';
                unlockEffect.textContent = '🔓 Unlocks hidden challenge';
                effectsDiv.appendChild(unlockEffect);
            }
        }

        choiceBtn.appendChild(effectsDiv);

        // Requirements display
        if (!isAvailable && choice.requirements) {
            const reqDiv = document.createElement('div');
            reqDiv.className = 'choice-requirement';
            reqDiv.textContent = '🔒 Requirements not met';
            choiceBtn.appendChild(reqDiv);
        }

        // Click handler
        if (isAvailable) {
            choiceBtn.onclick = () => makeNarrativeChoice(branch.id, index, choice);
        }

        choicesContainer.appendChild(choiceBtn);
    });

    // Show modal
    modal.classList.add('active');

    // Haptic feedback on mobile
    if ('vibrate' in navigator) {
        navigator.vibrate(20);
    }
}

/**
 * Check if choice requirements are met
 * @param {Object} choice - The choice object
 * @returns {boolean} - Whether requirements are met
 */
function checkChoiceRequirements(choice) {
    if (!choice.requirements || !gameState) {
        return true;
    }

    const reqs = choice.requirements;

    // Check level requirement
    if (reqs.min_level !== undefined && reqs.min_level !== null) {
        if (gameState.current_level < reqs.min_level) {
            return false;
        }
    }

    // Check required challenges
    if (reqs.required_challenges && reqs.required_challenges.length > 0) {
        for (const challengeId of reqs.required_challenges) {
            if (!gameState.completed_challenges.includes(challengeId)) {
                return false;
            }
        }
    }

    // Check required flags
    if (reqs.required_flags && reqs.required_flags.length > 0) {
        for (const flag of reqs.required_flags) {
            if (!gameState.story_flags || !gameState.story_flags.includes(flag)) {
                return false;
            }
        }
    }

    return true;
}

/**
 * Handle player making a narrative choice
 * @param {string} branchId - The branch ID
 * @param {number} choiceIndex - Index of the chosen option
 * @param {Object} choice - The choice object
 */
async function makeNarrativeChoice(branchId, choiceIndex, choice) {
    try {
        // Apply choice effects through WASM
        if (wasmModule && gameState) {
            // Apply sanity change
            if (choice.effects.sanity_change) {
                gameState.sanity = Math.max(0, Math.min(100, 
                    gameState.sanity + choice.effects.sanity_change));
                updateUI();
            }

            // Apply XP change
            if (choice.effects.xp_change) {
                gameState.experience += choice.effects.xp_change;
                updateUI();
            }

            // Add story flags
            if (choice.effects.story_flags) {
                if (!gameState.story_flags) {
                    gameState.story_flags = [];
                }
                choice.effects.story_flags.forEach(flag => {
                    if (!gameState.story_flags.includes(flag)) {
                        gameState.story_flags.push(flag);
                    }
                });
            }

            // Record choice in history
            if (!gameState.choice_history) {
                gameState.choice_history = [];
            }
            gameState.choice_history.push(choice.branch_id);
            gameState.active_narrative_branch = choice.branch_id;

            // Show choice response
            terminal.writeln('');
            terminal.writeln('\x1b[35m═══════════════════════════════════════════════════\x1b[0m');
            terminal.writeln('\x1b[36m' + choice.response + '\x1b[0m');
            terminal.writeln('\x1b[35m═══════════════════════════════════════════════════\x1b[0m');
            terminal.writeln('');

            // Show effects applied
            if (choice.effects.sanity_change !== 0) {
                const sign = choice.effects.sanity_change > 0 ? '+' : '';
                const color = choice.effects.sanity_change > 0 ? '\x1b[32m' : '\x1b[33m';
                terminal.writeln(`${color}Sanity ${sign}${choice.effects.sanity_change}\x1b[0m`);
            }
            if (choice.effects.xp_change > 0) {
                terminal.writeln(`\x1b[32m+${choice.effects.xp_change} XP\x1b[0m`);
            }
            terminal.writeln('');

            // Save game state
            saveGame();
        }

        // Hide modal
        hideNarrativeModal();

        // Haptic feedback
        if ('vibrate' in navigator) {
            navigator.vibrate([10, 50, 10]);
        }

    } catch (error) {
        console.error('Error applying narrative choice:', error);
        terminal.writeln('\x1b[31mError processing your choice. Please try again.\x1b[0m');
    }
}

/**
 * Hide the narrative modal
 */
function hideNarrativeModal() {
    const modal = document.getElementById('narrativeModal');
    if (modal) {
        modal.classList.remove('active');
    }
}

/**
 * Check if a narrative branch should trigger
 * @param {number} level - Current player level
 * @param {number} sanity - Current sanity
 * @returns {Object|null} - Branch to trigger or null
 */
function checkNarrativeTriggers(level, sanity) {
    // This will be called by WASM after level changes
    // For now, return null - will be implemented with WASM integration
    return null;
}

// Initialize when page loads
document.addEventListener("DOMContentLoaded", init);

// Enhanced page visibility and network handling
document.addEventListener("visibilitychange", () => {
    if (document.hidden && gameState) {
        saveGame();
        storeOfflineGameState(); // Also store offline backup
    }
});

// Network status monitoring for offline/online modes
window.addEventListener("online", () => {
    console.log("🌐 Network connection restored");
    handleOfflineMode(false);
});

window.addEventListener("offline", () => {
    console.log("📡 Network connection lost - switching to offline mode");
    handleOfflineMode(true);
});

// Enhanced auto-save with offline support
setInterval(() => {
    if (gameState) {
        // Regular save
        try {
            const saveData = localStorage.getItem("hack_game_save");
            const currentSave = wasmModule.generate_save_file(gameState);

            // Only save if there are changes
            if (saveData !== currentSave) {
                localStorage.setItem("hack_game_save", currentSave);
                storeOfflineGameState(); // Backup offline
                console.log("💾 Auto-saved game progress");
            }
        } catch (error) {
            console.warn("Auto-save failed:", error);
        }
    }
}, 30000); // Auto-save every 30 seconds

// Performance monitoring for mobile optimization
if (isMobile) {
    let performanceMetrics = {
        loadTime: 0,
        renderTime: 0,
        memoryUsage: 0,
    };

    // Monitor load performance
    window.addEventListener("load", () => {
        performanceMetrics.loadTime = performance.now();
        console.log(
            `📊 Load time: ${performanceMetrics.loadTime.toFixed(2)}ms`
        );

        // Optimize based on performance
        if (performanceMetrics.loadTime > 5000) {
            // Slow loading - enable performance mode
            enablePerformanceMode();
        }
    });

    // Monitor memory usage (if available)
    if ("memory" in performance) {
        setInterval(() => {
            const memory = performance.memory;
            performanceMetrics.memoryUsage =
                memory.usedJSHeapSize / 1024 / 1024; // MB

            // If memory usage is high, trigger cleanup
            if (performanceMetrics.memoryUsage > 50) {
                performMemoryCleanup();
            }
        }, 60000); // Check every minute
    }
}

function enablePerformanceMode() {
    console.log("⚡ Enabling performance mode for slower devices");

    // Reduce animation frequency
    document.documentElement.style.setProperty("--animation-speed", "slow");

    // Disable some visual effects
    document.querySelectorAll(".scanlines").forEach((el) => {
        el.style.display = "none";
    });

    // Reduce terminal font size slightly
    if (terminal && isMobile) {
        terminal.options.fontSize = Math.max(terminal.options.fontSize - 1, 10);
    }

    if (terminal) {
        writeMuted("⚡ Performance mode enabled for better responsiveness");
    }
}

function performMemoryCleanup() {
    console.log("🧹 Performing memory cleanup");

    // Clear old command history
    if (commandHistory.length > 25) {
        commandHistory = commandHistory.slice(-25);
        console.log("Cleared old command history");
    }

    // Clear old achievement notifications
    document
        .querySelectorAll(".gesture-feedback, .gesture-hint")
        .forEach((el) => {
            if (el.style.opacity === "0") {
                el.remove();
            }
        });

    // Force garbage collection if available
    if (window.gc) {
        window.gc();
    }
}
