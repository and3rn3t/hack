// Cloudflare Worker for The Hack: Ghost Protocol
// Serves the WebAssembly-compiled game

export default {
    async fetch(request, env) {
        const url = new URL(request.url);

        // Serve static files from web/static/
        if (url.pathname.startsWith("/static/") || url.pathname === "/") {
            return await serveStaticFile(request, env);
        }

        // API endpoints for game data
        if (url.pathname.startsWith("/api/")) {
            return await handleAPI(request, env);
        }

        // Default to index.html
        return await serveFile("index.html", "text/html");
    },
};

async function serveStaticFile(request, env) {
    const url = new URL(request.url);
    let pathname = url.pathname;

    // Default to index.html
    if (pathname === "/") {
        pathname = "/index.html";
    }

    // Remove /static/ prefix if present
    if (pathname.startsWith("/static/")) {
        pathname = pathname.substring(8);
    }

    // Determine content type
    const contentType = getContentType(pathname);

    try {
        return await serveFile(pathname, contentType);
    } catch (error) {
        return new Response("File not found", { status: 404 });
    }
}

async function serveFile(filename, contentType) {
    // In a real deployment, you would store your static files
    // in Cloudflare's KV storage or use a static asset handler

    // For now, return a placeholder response
    if (filename === "index.html") {
        const html = `<!DOCTYPE html>
<html>
<head>
    <title>The Hack: Ghost Protocol</title>
    <meta charset="utf-8">
    <style>
        body {
            background: #000;
            color: #0f0;
            font-family: 'Courier New', monospace;
            text-align: center;
            padding: 50px;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            border: 1px solid #0f0;
            padding: 30px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎮 The Hack: Ghost Protocol</h1>
        <p>WebAssembly deployment coming soon...</p>
        <p>This Cloudflare Worker is ready to serve the compiled WASM game.</p>
        <div style="margin-top: 30px;">
            <h2>🏗️ Implementation Status</h2>
            <ul style="text-align: left; margin-left: 200px;">
                <li>✅ Rust game engine (11 challenges, horror narrative)</li>
                <li>✅ Web interface with xterm.js terminal emulator</li>
                <li>✅ WebAssembly bindings created</li>
                <li>✅ Cloudflare Workers deployment setup</li>
                <li>🔄 Final WASM compilation (in progress)</li>
            </ul>
        </div>
    </div>
</body>
</html>`;
        return new Response(html, {
            headers: { "Content-Type": "text/html; charset=utf-8" },
        });
    }

    return new Response("File not found", { status: 404 });
}

async function handleAPI(request, env) {
    const url = new URL(request.url);

    // Save game state
    if (url.pathname === "/api/save" && request.method === "POST") {
        const gameData = await request.json();
        // In production: await env.GAME_DATA.put(gameData.playerId, JSON.stringify(gameData));
        return new Response(JSON.stringify({ success: true }), {
            headers: { "Content-Type": "application/json" },
        });
    }

    // Load game state
    if (url.pathname === "/api/load" && request.method === "GET") {
        const playerId = url.searchParams.get("playerId");
        // In production: const savedData = await env.GAME_DATA.get(playerId);
        return new Response(JSON.stringify({ gameState: null }), {
            headers: { "Content-Type": "application/json" },
        });
    }

    return new Response("API endpoint not found", { status: 404 });
}

function getContentType(filename) {
    const ext = filename.split(".").pop();
    const types = {
        html: "text/html",
        js: "application/javascript",
        wasm: "application/wasm",
        css: "text/css",
        json: "application/json",
        png: "image/png",
        jpg: "image/jpeg",
        gif: "image/gif",
        svg: "image/svg+xml",
    };
    return types[ext] || "text/plain";
}
