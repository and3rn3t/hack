#!/usr/bin/env node

/**
 * SVG to PNG Converter using base64 data URLs
 * Converts SVG icons to PNG format without external dependencies
 */

const fs = require("fs");
const path = require("path");

function convertSvgToPngDataUrl(svgContent, size) {
    // Create a minimal HTML page that renders the SVG and converts to PNG
    const html = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>body { margin: 0; padding: 20px; background: #000; }</style>
</head>
<body>
    <div id="svg-container">${svgContent}</div>
    <canvas id="canvas" width="${size}" height="${size}" style="border: 1px solid #00ff41;"></canvas>
    <div id="output" style="color: #00ff41; font-family: monospace; margin-top: 10px;"></div>

    <script>
        function convertSvgToPng() {
            const svg = document.querySelector('svg');
            const canvas = document.getElementById('canvas');
            const ctx = canvas.getContext('2d');

            // Create an image from the SVG
            const svgData = new XMLSerializer().serializeToString(svg);
            const svgBlob = new Blob([svgData], {type: 'image/svg+xml;charset=utf-8'});
            const url = URL.createObjectURL(svgBlob);

            const img = new Image();
            img.onload = function() {
                // Clear canvas and draw the image
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                ctx.drawImage(img, 0, 0, canvas.width, canvas.height);

                // Convert to PNG data URL
                const pngDataUrl = canvas.toDataURL('image/png');

                // Display result
                document.getElementById('output').innerHTML =
                    '<strong>PNG Data URL Generated!</strong><br>' +
                    '<textarea style="width:100%;height:100px;background:#111;color:#0f0;border:1px solid #0f0;">' +
                    pngDataUrl + '</textarea><br>' +
                    '<button onclick="downloadPng()" style="background:#0f0;color:#000;padding:10px;border:none;margin:10px;">Download PNG</button>';

                // Store for download
                window.pngDataUrl = pngDataUrl;
                URL.revokeObjectURL(url);
            };
            img.src = url;
        }

        function downloadPng() {
            if (window.pngDataUrl) {
                const link = document.createElement('a');
                link.download = 'converted-icon.png';
                link.href = window.pngDataUrl;
                link.click();
            }
        }

        // Auto-convert when page loads
        window.addEventListener('load', convertSvgToPng);
    </script>
</body>
</html>`;

    return html;
}

function generateConverterPages() {
    const iconsDir = path.join(__dirname, "../web/static/icons");
    const outputDir = path.join(__dirname, "../temp-converters");

    // Create temp directory for converter pages
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    // Find all SVG files
    const svgFiles = fs
        .readdirSync(iconsDir)
        .filter((file) => file.endsWith(".svg"));

    console.log("🎨 Generating SVG to PNG converter pages...\n");

    svgFiles.forEach((svgFile) => {
        const svgPath = path.join(iconsDir, svgFile);
        const svgContent = fs.readFileSync(svgPath, "utf8");

        // Extract size from filename or default to 96
        const sizeMatch = svgFile.match(/(\d+)x\d+/);
        const size = sizeMatch ? parseInt(sizeMatch[1]) : 96;

        const html = convertSvgToPngDataUrl(svgContent, size);
        const htmlFile = svgFile.replace(".svg", "-converter.html");
        const htmlPath = path.join(outputDir, htmlFile);

        fs.writeFileSync(htmlPath, html);
        console.log(`✅ ${htmlFile} - Converter for ${svgFile}`);
    });

    // Create index page
    const indexHtml = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>The Hack: Ghost Protocol - Icon Converters</title>
    <style>
        body { background: #000; color: #00ff41; font-family: monospace; padding: 20px; }
        h1 { color: #ff4444; text-shadow: 0 0 10px #ff4444; }
        .converter-link {
            display: block;
            color: #00ff41;
            text-decoration: none;
            padding: 10px;
            border: 1px solid #333;
            margin: 5px 0;
        }
        .converter-link:hover { background: #001100; border-color: #00ff41; }
        .instructions { background: #111; padding: 15px; margin: 20px 0; border: 1px solid #333; }
    </style>
</head>
<body>
    <h1>🎯 SVG to PNG Icon Converters</h1>

    <div class="instructions">
        <h2>📋 Instructions:</h2>
        <ol>
            <li>Click on any converter link below</li>
            <li>Wait for the PNG to generate (should be instant)</li>
            <li>Click "Download PNG" to save the converted icon</li>
            <li>Repeat for all icons you need</li>
            <li>Copy downloaded PNGs to <code>web/static/icons/</code> (replace existing placeholder PNGs)</li>
        </ol>
    </div>

    <h2>🎨 Available Converters:</h2>
    ${svgFiles
        .map((svgFile) => {
            const htmlFile = svgFile.replace(".svg", "-converter.html");
            return `<a href="${htmlFile}" class="converter-link">📱 ${svgFile} → PNG</a>`;
        })
        .join("")}

    <div class="instructions" style="margin-top: 30px;">
        <h2>🔧 Alternative Methods:</h2>
        <p>If this doesn't work, you can:</p>
        <ul>
            <li>Use online SVG to PNG converters (search "SVG to PNG converter")</li>
            <li>Open SVG files in browser and take screenshots</li>
            <li>Use the main HTML icon generator: <code>scripts/generate-icons.html</code></li>
        </ul>
    </div>
</body>
</html>`;

    fs.writeFileSync(path.join(outputDir, "index.html"), indexHtml);
    console.log(`\n🎯 Converter Complete!`);
    console.log(`📂 Generated ${svgFiles.length} converter pages`);
    console.log(`🌐 Open: ${path.join(outputDir, "index.html")}`);

    return outputDir;
}

// Run if called directly
if (require.main === module) {
    try {
        const outputDir = generateConverterPages();

        // Try to open the index page
        const { spawn } = require("child_process");
        const indexPath = path.join(outputDir, "index.html");

        try {
            spawn("start", [indexPath], { shell: true, detached: true });
            console.log(`✅ Opened converter index in browser`);
        } catch (e) {
            console.log(`💡 Manually open: ${indexPath}`);
        }
    } catch (error) {
        console.error("❌ Error generating converters:", error.message);
        process.exit(1);
    }
}

module.exports = { generateConverterPages };
