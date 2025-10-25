#!/usr/bin/env node

/**
 * Simple Icon Generator using SVG to PNG conversion
 * Creates horror-themed cybersecurity icons without canvas dependency
 */

const fs = require("fs");
const path = require("path");

const iconSizes = [72, 96, 128, 144, 152, 192, 384, 512];
const shortcutIcons = [
    {
        name: "shortcut-new",
        label: "New Game",
        symbol: "NEW",
        color: "#00ff41",
    },
    {
        name: "shortcut-continue",
        label: "Continue",
        symbol: "CONT",
        color: "#ffaa00",
    },
    {
        name: "shortcut-challenges",
        label: "Challenges",
        symbol: "CHAL",
        color: "#ff4444",
    },
    {
        name: "shortcut-stats",
        label: "Statistics",
        symbol: "STAT",
        color: "#4444ff",
    },
    { name: "file-import", label: "Import", symbol: "FILE", color: "#aa44ff" },
];

function createHorrorIconSVG(size, isShortcut = false, shortcutData = null) {
    const centerX = size / 2;
    const centerY = size / 2;

    if (isShortcut && shortcutData) {
        return `<svg width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <radialGradient id="bg-${
                    shortcutData.name
                }" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" style="stop-color:#001100;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#000000;stop-opacity:1" />
                </radialGradient>
            </defs>
            <rect width="${size}" height="${size}" fill="url(#bg-${
            shortcutData.name
        })" />
            <rect x="${size / 8}" y="${size / 8}" width="${
            (size * 3) / 4
        }" height="${(size * 3) / 4}"
                  fill="none" stroke="${shortcutData.color}" stroke-width="${
            size / 32
        }"
                  stroke-dasharray="${size / 16},${size / 32}" />
            <text x="${centerX}" y="${centerY}" text-anchor="middle" dominant-baseline="middle"
                  fill="${
                      shortcutData.color
                  }" font-family="monospace" font-size="${
            size / 4
        }" font-weight="bold">
                ${shortcutData.symbol}
            </text>
        </svg>`;
    } else {
        // Main app icon
        const skullSize = size * 0.6;
        const eyeSize = skullSize / 8;
        const eyeY = centerY - skullSize / 6;

        return `<svg width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <radialGradient id="bg-main" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" style="stop-color:#001100;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#000000;stop-opacity:1" />
                </radialGradient>
            </defs>
            <!-- Background -->
            <rect width="${size}" height="${size}" fill="url(#bg-main)" />

            <!-- Border -->
            <rect x="1" y="1" width="${size - 2}" height="${size - 2}"
                  fill="none" stroke="#00ff41" stroke-width="2"
                  stroke-dasharray="${size / 16},${size / 32}" />

            <!-- Skull outline -->
            <ellipse cx="${centerX}" cy="${centerY - skullSize / 6}"
                     rx="${skullSize / 2.5}" ry="${skullSize / 3}"
                     fill="#002200" stroke="#00ff41" stroke-width="${Math.max(
                         2,
                         size / 48
                     )}" />

            <!-- Left eye socket -->
            <rect x="${centerX - skullSize / 4 - eyeSize / 2}" y="${
            eyeY - eyeSize / 2
        }"
                  width="${eyeSize}" height="${eyeSize}"
                  fill="#000" stroke="#00ff41" stroke-width="1" />

            <!-- Right eye socket -->
            <rect x="${centerX + skullSize / 4 - eyeSize / 2}" y="${
            eyeY - eyeSize / 2
        }"
                  width="${eyeSize}" height="${eyeSize}"
                  fill="#000" stroke="#00ff41" stroke-width="1" />

            <!-- Eye terminals -->
            <text x="${centerX - skullSize / 4}" y="${
            eyeY + 2
        }" text-anchor="middle"
                  fill="#00ff41" font-family="monospace" font-size="${Math.max(
                      8,
                      size / 32
                  )}">></text>
            <text x="${centerX + skullSize / 4}" y="${
            eyeY + 2
        }" text-anchor="middle"
                  fill="#00ff41" font-family="monospace" font-size="${Math.max(
                      8,
                      size / 32
                  )}">_</text>

            <!-- Nose (USB port) -->
            <rect x="${centerX - skullSize / 24}" y="${centerY}"
                  width="${skullSize / 12}" height="${skullSize / 20}"
                  fill="#000" stroke="#00ff41" stroke-width="1" />

            <!-- Mouth (HACK keys) -->
            <g fill="#002200" stroke="#00ff41" stroke-width="1" font-family="monospace" font-size="${Math.max(
                6,
                size / 48
            )}" text-anchor="middle">
                ${["H", "A", "C", "K"]
                    .map((key, i) => {
                        const keyX =
                            centerX -
                            (4 * skullSize) / 24 / 2 +
                            (i * skullSize) / 24 +
                            skullSize / 48;
                        const mouthY = centerY + skullSize / 6;
                        return `
                        <rect x="${keyX - skullSize / 48}" y="${
                            mouthY - skullSize / 48
                        }"
                              width="${skullSize / 24}" height="${
                            skullSize / 24
                        }" />
                        <text x="${keyX}" y="${
                            mouthY + 2
                        }" fill="#00ff41">${key}</text>
                    `;
                    })
                    .join("")}
            </g>

            <!-- Matrix rain effect -->
            <g fill="rgba(0,255,65,0.3)" font-family="monospace" font-size="${Math.max(
                6,
                size / 64
            )}">
                ${Array.from({ length: 8 }, (_, i) => {
                    const x = Math.random() * size;
                    const y = Math.random() * size;
                    return `<text x="${x}" y="${y}">${Math.floor(
                        Math.random() * 2
                    )}</text>`;
                }).join("")}
            </g>
        </svg>`;
    }
}

function generateIconsWithSVG() {
    const outputDir = path.join(__dirname, "../web/static/icons");

    // Ensure output directory exists
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    let totalIcons = 0;
    const generatedFiles = [];

    console.log("🎨 Generating SVG-based Horror Icons...\n");

    // Generate main app icons
    console.log("📱 Creating app icons:");
    iconSizes.forEach((size) => {
        const svg = createHorrorIconSVG(size);
        const filename = `icon-${size}x${size}.svg`;
        const filepath = path.join(outputDir, filename);
        fs.writeFileSync(filepath, svg);
        generatedFiles.push(filepath);
        console.log(`  ✅ ${filename} (${size}×${size})`);
        totalIcons++;
    });

    console.log("\n🔗 Creating shortcut icons:");
    // Generate shortcut icons
    shortcutIcons.forEach((shortcut) => {
        const svg = createHorrorIconSVG(96, true, shortcut);
        const filename = `${shortcut.name}.svg`;
        const filepath = path.join(outputDir, filename);
        fs.writeFileSync(filepath, svg);
        generatedFiles.push(filepath);
        console.log(`  ✅ ${filename} (${shortcut.label})`);
        totalIcons++;
    });

    console.log(`\n🎯 SVG Icon Generation Complete!`);
    console.log(
        `📊 Generated ${totalIcons} SVG icons (${iconSizes.length} app + ${shortcutIcons.length} shortcuts)`
    );
    console.log(`📂 Saved to: ${outputDir}`);

    console.log("\n💡 To convert to PNG:");
    console.log("   1. Open HTML generator: scripts/generate-icons.html");
    console.log("   2. Or use online SVG to PNG converter");
    console.log("   3. Or install imagemagick: convert icon.svg icon.png");

    return { totalIcons, generatedFiles };
}

// Run if called directly
if (require.main === module) {
    try {
        generateIconsWithSVG();
    } catch (error) {
        console.error("❌ Error generating SVG icons:", error.message);
        process.exit(1);
    }
}

module.exports = { generateIconsWithSVG, createHorrorIconSVG };
