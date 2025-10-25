#!/usr/bin/env node

/**
 * Simple HTTP Server for Screenshot Capture
 * Serves the web version of The Hack: Ghost Protocol
 */

const http = require("http");
const fs = require("fs");
const path = require("path");
const url = require("url");

const PORT = 8000;
const WEB_ROOT = path.join(__dirname, "../web/static");

// MIME types
const mimeTypes = {
    ".html": "text/html",
    ".js": "application/javascript",
    ".css": "text/css",
    ".json": "application/json",
    ".png": "image/png",
    ".jpg": "image/jpeg",
    ".gif": "image/gif",
    ".svg": "image/svg+xml",
    ".ico": "image/x-icon",
    ".wasm": "application/wasm",
};

function getContentType(filePath) {
    const ext = path.extname(filePath).toLowerCase();
    return mimeTypes[ext] || "application/octet-stream";
}

function serveFile(res, filePath) {
    fs.readFile(filePath, (err, data) => {
        if (err) {
            res.writeHead(404, { "Content-Type": "text/html" });
            res.end("<h1>404 Not Found</h1>");
            return;
        }

        const contentType = getContentType(filePath);
        res.writeHead(200, {
            "Content-Type": contentType,
            "Cache-Control": "no-cache",
        });
        res.end(data);
    });
}

const server = http.createServer((req, res) => {
    // Enable CORS for development
    res.setHeader("Access-Control-Allow-Origin", "*");
    res.setHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
    res.setHeader("Access-Control-Allow-Headers", "Content-Type");

    const parsedUrl = url.parse(req.url);
    let pathname = parsedUrl.pathname;

    // Default to index.html
    if (pathname === "/") {
        pathname = "/index.html";
    }

    const filePath = path.join(WEB_ROOT, pathname);

    // Security check - prevent directory traversal
    if (!filePath.startsWith(WEB_ROOT)) {
        res.writeHead(403, { "Content-Type": "text/html" });
        res.end("<h1>403 Forbidden</h1>");
        return;
    }

    // Check if file exists
    fs.stat(filePath, (err, stats) => {
        if (err || !stats.isFile()) {
            res.writeHead(404, { "Content-Type": "text/html" });
            res.end(`
                <html>
                <head><title>404 Not Found</title></head>
                <body style="background:#000;color:#00ff41;font-family:monospace;padding:20px;">
                    <h1 style="color:#ff4444;">404 - File Not Found</h1>
                    <p>Requested: ${pathname}</p>
                    <p>Path: ${filePath}</p>
                    <a href="/" style="color:#00ff41;">← Back to Game</a>
                </body>
                </html>
            `);
            return;
        }

        serveFile(res, filePath);
    });
});

server.listen(PORT, () => {
    console.log("🎯 The Hack: Ghost Protocol - Screenshot Server");
    console.log("=====================================");
    console.log(`🌐 Server running at: http://localhost:${PORT}`);
    console.log(`📂 Serving files from: ${WEB_ROOT}`);
    console.log("");
    console.log("📸 For Screenshots:");
    console.log("1. Open: http://localhost:8000 in browser");
    console.log("2. Resize window to screenshot dimensions:");
    console.log("   - Desktop: 1280x720");
    console.log("   - Mobile: 360x640 (portrait) or 640x360 (landscape)");
    console.log("3. Navigate through game for different screens");
    console.log("4. Take screenshots (F12 → Screenshot tool)");
    console.log("");
    console.log("💡 Use Ctrl+C to stop server");
    console.log("=====================================");

    // Try to open browser automatically
    const { spawn } = require("child_process");
    try {
        spawn("start", [`http://localhost:${PORT}`], {
            shell: true,
            detached: true,
        });
        console.log("✅ Opened browser automatically");
    } catch (e) {
        console.log("💡 Manually open: http://localhost:8000");
    }
});

server.on("error", (err) => {
    if (err.code === "EADDRINUSE") {
        console.log(`❌ Port ${PORT} is already in use`);
        console.log(
            `💡 Try: killall node  OR  netstat -ano | findstr :${PORT}`
        );
    } else {
        console.log("❌ Server error:", err.message);
    }
});

// Graceful shutdown
process.on("SIGINT", () => {
    console.log("\\n🛑 Shutting down screenshot server...");
    server.close(() => {
        console.log("✅ Server stopped");
        process.exit(0);
    });
});

module.exports = { server, PORT, WEB_ROOT };
