#!/usr/bin/env node

/**
 * The Hack: Ghost Protocol - Icon Generator
 * Generates horror-themed cybersecurity CTF game icons
 */

const fs = require("fs");
const path = require("path");

// Check if canvas package is available, if not provide instructions
let Canvas;
try {
    Canvas = require("canvas");
} catch (e) {
    console.log("🚨 Canvas package not found. Installing...");
    console.log("Run: npm install canvas");
    console.log("Or use the HTML version: open scripts/generate-icons.html");
    process.exit(1);
}

const { createCanvas } = Canvas;

const iconSizes = [72, 96, 128, 144, 152, 192, 384, 512];
const shortcutIcons = [
    { name: "shortcut-new", label: "New Game", symbol: "🎮", color: "#00ff41" },
    {
        name: "shortcut-continue",
        label: "Continue",
        symbol: "▶️",
        color: "#ffaa00",
    },
    {
        name: "shortcut-challenges",
        label: "Challenges",
        symbol: "🎯",
        color: "#ff4444",
    },
    {
        name: "shortcut-stats",
        label: "Statistics",
        symbol: "📊",
        color: "#4444ff",
    },
    { name: "file-import", label: "Import", symbol: "📁", color: "#aa44ff" },
];

function createHorrorIcon(size, isShortcut = false, shortcutData = null) {
    const canvas = createCanvas(size, size);
    const ctx = canvas.getContext("2d");

    // Background - dark with subtle gradient
    const gradient = ctx.createRadialGradient(
        size / 2,
        size / 2,
        0,
        size / 2,
        size / 2,
        size / 2
    );
    gradient.addColorStop(0, "#001100");
    gradient.addColorStop(1, "#000000");
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, size, size);

    // Border with glitch effect
    ctx.strokeStyle = "#00ff41";
    ctx.lineWidth = Math.max(1, size / 64);
    ctx.setLineDash([size / 32, size / 64]);
    ctx.strokeRect(
        ctx.lineWidth / 2,
        ctx.lineWidth / 2,
        size - ctx.lineWidth,
        size - ctx.lineWidth
    );
    ctx.setLineDash([]);

    if (isShortcut && shortcutData) {
        // Shortcut-specific design
        ctx.fillStyle = shortcutData.color;
        ctx.font = `${size * 0.4}px monospace`;
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";

        // For Node.js canvas, we'll use text instead of emoji
        let symbol = shortcutData.symbol;
        if (shortcutData.name === "shortcut-new") symbol = "NEW";
        if (shortcutData.name === "shortcut-continue") symbol = "CONT";
        if (shortcutData.name === "shortcut-challenges") symbol = "CHAL";
        if (shortcutData.name === "shortcut-stats") symbol = "STAT";
        if (shortcutData.name === "file-import") symbol = "FILE";

        ctx.font = `bold ${size * 0.25}px monospace`;
        ctx.fillText(symbol, size / 2, size / 2);

        // Add border glow
        ctx.shadowColor = shortcutData.color;
        ctx.shadowBlur = size / 16;
        ctx.strokeStyle = shortcutData.color;
        ctx.lineWidth = size / 32;
        ctx.strokeRect(size / 8, size / 8, (size * 3) / 4, (size * 3) / 4);
    } else {
        // Main app icon - skull terminal hybrid
        const centerX = size / 2;
        const centerY = size / 2;
        const skullSize = size * 0.6;

        // Skull outline
        ctx.strokeStyle = "#00ff41";
        ctx.lineWidth = Math.max(2, size / 48);
        ctx.fillStyle = "#002200";

        // Skull shape (simplified)
        ctx.beginPath();
        ctx.ellipse(
            centerX,
            centerY - skullSize / 6,
            skullSize / 2.5,
            skullSize / 3,
            0,
            0,
            Math.PI * 2
        );
        ctx.fill();
        ctx.stroke();

        // Eye sockets (terminal screens)
        const eyeSize = skullSize / 8;
        const eyeY = centerY - skullSize / 6;

        // Left eye
        ctx.fillStyle = "#000";
        ctx.fillRect(
            centerX - skullSize / 4 - eyeSize / 2,
            eyeY - eyeSize / 2,
            eyeSize,
            eyeSize
        );
        ctx.strokeRect(
            centerX - skullSize / 4 - eyeSize / 2,
            eyeY - eyeSize / 2,
            eyeSize,
            eyeSize
        );

        // Right eye
        ctx.fillRect(
            centerX + skullSize / 4 - eyeSize / 2,
            eyeY - eyeSize / 2,
            eyeSize,
            eyeSize
        );
        ctx.strokeRect(
            centerX + skullSize / 4 - eyeSize / 2,
            eyeY - eyeSize / 2,
            eyeSize,
            eyeSize
        );

        // Terminal text in eyes
        ctx.fillStyle = "#00ff41";
        ctx.font = `${Math.max(8, size / 32)}px monospace`;
        ctx.textAlign = "center";
        ctx.fillText(">", centerX - skullSize / 4, eyeY);
        ctx.fillText("_", centerX + skullSize / 4, eyeY);

        // Nose (USB port shape)
        const noseWidth = skullSize / 12;
        const noseHeight = skullSize / 20;
        ctx.fillStyle = "#000";
        ctx.fillRect(centerX - noseWidth / 2, centerY, noseWidth, noseHeight);
        ctx.strokeRect(centerX - noseWidth / 2, centerY, noseWidth, noseHeight);

        // Mouth (keyboard keys)
        const mouthY = centerY + skullSize / 6;
        const keySize = skullSize / 24;
        const keys = ["H", "A", "C", "K"];

        ctx.fillStyle = "#002200";
        ctx.font = `${Math.max(6, size / 48)}px monospace`;
        ctx.textAlign = "center";

        for (let i = 0; i < keys.length; i++) {
            const keyX =
                centerX -
                (keys.length * keySize) / 2 +
                i * keySize +
                keySize / 2;
            ctx.fillRect(
                keyX - keySize / 3,
                mouthY - keySize / 3,
                (keySize * 2) / 3,
                (keySize * 2) / 3
            );
            ctx.strokeRect(
                keyX - keySize / 3,
                mouthY - keySize / 3,
                (keySize * 2) / 3,
                (keySize * 2) / 3
            );
            ctx.fillStyle = "#00ff41";
            ctx.fillText(keys[i], keyX, mouthY);
            ctx.fillStyle = "#002200";
        }

        // Add matrix-style digital rain effect
        ctx.fillStyle = "rgba(0, 255, 65, 0.3)";
        ctx.font = `${Math.max(6, size / 64)}px monospace`;
        for (let i = 0; i < 8; i++) {
            const x = Math.random() * size;
            const y = Math.random() * size;
            ctx.fillText(String(Math.floor(Math.random() * 2)), x, y);
        }
    }

    return canvas;
}

function saveIcon(canvas, filename, outputDir) {
    const buffer = canvas.toBuffer("image/png");
    const filepath = path.join(outputDir, `${filename}.png`);
    fs.writeFileSync(filepath, buffer);
    return filepath;
}

function generateAllIcons() {
    const outputDir = path.join(__dirname, "../web/static/icons");

    // Ensure output directory exists
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    let totalIcons = 0;
    const generatedFiles = [];

    console.log("🎨 Generating The Hack: Ghost Protocol Icons...\n");

    // Generate main app icons
    console.log("📱 Creating app icons:");
    iconSizes.forEach((size) => {
        const canvas = createHorrorIcon(size);
        const filename = `icon-${size}x${size}`;
        const filepath = saveIcon(canvas, filename, outputDir);
        generatedFiles.push(filepath);
        console.log(`  ✅ ${filename}.png (${size}×${size})`);
        totalIcons++;
    });

    console.log("\n🔗 Creating shortcut icons:");
    // Generate shortcut icons
    shortcutIcons.forEach((shortcut) => {
        const canvas = createHorrorIcon(96, true, shortcut);
        const filepath = saveIcon(canvas, shortcut.name, outputDir);
        generatedFiles.push(filepath);
        console.log(`  ✅ ${shortcut.name}.png (${shortcut.label})`);
        totalIcons++;
    });

    console.log(`\n🎯 Icon Generation Complete!`);
    console.log(
        `📊 Generated ${totalIcons} icons (${iconSizes.length} app + ${shortcutIcons.length} shortcuts)`
    );
    console.log(`📂 Saved to: ${outputDir}`);

    // List file sizes
    console.log("\n📋 File Details:");
    generatedFiles.forEach((file) => {
        const stats = fs.statSync(file);
        const sizeKB = (stats.size / 1024).toFixed(1);
        console.log(`  ${path.basename(file)} - ${sizeKB} KB`);
    });

    return { totalIcons, generatedFiles };
}

// Run if called directly
if (require.main === module) {
    try {
        generateAllIcons();
    } catch (error) {
        console.error("❌ Error generating icons:", error.message);
        console.log(
            "\n💡 Alternative: Open scripts/generate-icons.html in browser"
        );
        process.exit(1);
    }
}

module.exports = { generateAllIcons, createHorrorIcon };
