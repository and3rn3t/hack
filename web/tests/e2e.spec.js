import { test, expect } from "@playwright/test";

test.describe("The Hack: Ghost Protocol - End-to-End Tests", () => {
    test.beforeEach(async ({ page }) => {
        // Navigate to the game
        await page.goto("https://hack.andernet.dev");

        // Wait for the game to load
        await page.waitForSelector("#gameContainer", { state: "visible" });
        await page.waitForTimeout(2000); // Allow WASM to fully initialize
    });

    test("should load the game successfully", async ({ page }) => {
        // Check that the game title is visible
        await expect(page).toHaveTitle(/The Hack: Ghost Protocol/);

        // Check that the terminal is visible
        const terminal = page.locator(".terminal-container");
        await expect(terminal).toBeVisible();

        // Check that the welcome message appears
        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain(
            "Welcome to The Hack: Ghost Protocol"
        );
    });

    test("should display help command", async ({ page }) => {
        // Type help command
        await page.keyboard.type("help");
        await page.keyboard.press("Enter");

        // Wait for response
        await page.waitForTimeout(1000);

        // Check that help content is displayed
        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Available Commands:");
        expect(terminalContent).toContain("challenges");
        expect(terminalContent).toContain("stats");
    });

    test("should show challenges list", async ({ page }) => {
        await page.keyboard.type("challenges");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Available Challenges:");
        expect(terminalContent).toContain("Level 0");
    });

    test("should display stats", async ({ page }) => {
        await page.keyboard.type("stats");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Player Statistics");
        expect(terminalContent).toContain("Level:");
        expect(terminalContent).toContain("Sanity:");
        expect(terminalContent).toContain("Experience:");
    });

    test("should show achievements", async ({ page }) => {
        await page.keyboard.type("achievements");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Achievements");
        expect(terminalContent).toContain("Progress:");
    });

    test("should show available themes", async ({ page }) => {
        await page.keyboard.type("theme");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Available themes:");
        expect(terminalContent).toContain("horror");
        expect(terminalContent).toContain("matrix");
        expect(terminalContent).toContain("cyberpunk");
    });

    test("should change theme successfully", async ({ page }) => {
        await page.keyboard.type("theme matrix");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Theme changed to: matrix");
    });

    test("should start a challenge", async ({ page }) => {
        // First get the list of challenges to find a valid ID
        await page.keyboard.type("challenges");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        // Try to start the first challenge (assuming basic_base64 exists)
        await page.keyboard.type("challenge basic_base64");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("CHALLENGE:");
    });

    test("should show tutorial", async ({ page }) => {
        await page.keyboard.type("tutorial");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Interactive Tutorial");
        expect(terminalContent).toContain(
            "Welcome to The Hack: Ghost Protocol"
        );
    });

    test("should show progress report", async ({ page }) => {
        await page.keyboard.type("progress");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Progress Report");
    });

    test("should handle invalid commands gracefully", async ({ page }) => {
        await page.keyboard.type("invalidcommand123");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Unknown command");
    });

    test("should clear terminal", async ({ page }) => {
        // Type some commands first
        await page.keyboard.type("help");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(500);

        // Clear the terminal
        await page.keyboard.type("clear");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(500);

        // Terminal should be mostly empty (just prompt)
        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).not.toContain("Available Commands:");
    });

    test("should save and load game state", async ({ page }) => {
        // Save current state
        await page.keyboard.type("save");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        let terminalContent = await page.locator(".xterm-screen").textContent();
        expect(terminalContent).toContain("Game saved");

        // Load state
        await page.keyboard.type("load");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        terminalContent = await page.locator(".xterm-screen").textContent();
        expect(terminalContent).toContain("Game loaded");
    });

    test("should show leaderboard placeholder", async ({ page }) => {
        await page.keyboard.type("leaderboard");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Community Leaderboard");
        expect(terminalContent).toContain("Your stats");
    });

    test("should handle keyboard navigation", async ({ page }) => {
        // Type a command
        await page.keyboard.type("help");

        // Use arrow keys to navigate (should work without crashing)
        await page.keyboard.press("ArrowLeft");
        await page.keyboard.press("ArrowRight");
        await page.keyboard.press("Home");
        await page.keyboard.press("End");

        // Execute the command
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Available Commands:");
    });

    test("should handle mobile viewport", async ({ page }) => {
        // Set mobile viewport
        await page.setViewportSize({ width: 375, height: 667 });

        // Game should still be responsive
        const terminal = page.locator(".terminal-container");
        await expect(terminal).toBeVisible();

        // Should be able to type commands
        await page.keyboard.type("help");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        const terminalContent = await page
            .locator(".xterm-screen")
            .textContent();
        expect(terminalContent).toContain("Available Commands:");
    });
});

test.describe("Visual Regression Tests", () => {
    test("should match visual snapshot of initial load", async ({ page }) => {
        await page.goto("https://hack.andernet.dev");
        await page.waitForSelector("#gameContainer", { state: "visible" });
        await page.waitForTimeout(3000); // Allow full initialization

        // Take a screenshot for visual regression testing
        await expect(page).toHaveScreenshot("initial-load.png", {
            fullPage: true,
            animations: "disabled",
        });
    });

    test("should match visual snapshot of help screen", async ({ page }) => {
        await page.goto("https://hack.andernet.dev");
        await page.waitForSelector("#gameContainer", { state: "visible" });
        await page.waitForTimeout(2000);

        await page.keyboard.type("help");
        await page.keyboard.press("Enter");
        await page.waitForTimeout(1000);

        await expect(page).toHaveScreenshot("help-screen.png", {
            fullPage: true,
            animations: "disabled",
        });
    });
});

test.describe("Performance Tests", () => {
    test("should load within acceptable time", async ({ page }) => {
        const startTime = Date.now();

        await page.goto("https://hack.andernet.dev");
        await page.waitForSelector("#gameContainer", { state: "visible" });

        const loadTime = Date.now() - startTime;

        // Should load within 5 seconds
        expect(loadTime).toBeLessThan(5000);
    });

    test("should respond to commands quickly", async ({ page }) => {
        await page.goto("https://hack.andernet.dev");
        await page.waitForSelector("#gameContainer", { state: "visible" });
        await page.waitForTimeout(2000);

        const startTime = Date.now();

        await page.keyboard.type("help");
        await page.keyboard.press("Enter");

        // Wait for the help text to appear
        await page.waitForFunction(() => {
            const content =
                document.querySelector(".xterm-screen")?.textContent || "";
            return content.includes("Available Commands:");
        });

        const responseTime = Date.now() - startTime;

        // Should respond within 2 seconds
        expect(responseTime).toBeLessThan(2000);
    });
});
