// Service Worker for The Hack: Ghost Protocol PWA
// Provides offline functionality and caching for enhanced user experience

const CACHE_NAME = "hack-ghost-protocol-v1.1.0";
const STATIC_CACHE = "hack-ghost-static-v1.1.0";
const DYNAMIC_CACHE = "hack-ghost-dynamic-v1.1.0";

// Files to cache for offline functionality
const STATIC_ASSETS = [
    "/",
    "/index.html",
    "/game.js",
    "/manifest.json",
    "/pkg/hack_simulator.js",
    "/pkg/hack_simulator_bg.wasm",
    "https://cdn.jsdelivr.net/npm/xterm@5.3.0/lib/xterm.js",
    "https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.css",
    "https://cdn.jsdelivr.net/npm/xterm-addon-fit@0.8.0/lib/xterm-addon-fit.js",
];

// Install event - cache static assets
self.addEventListener("install", (event) => {
    console.log("[ServiceWorker] Installing...");

    event.waitUntil(
        caches
            .open(STATIC_CACHE)
            .then((cache) => {
                console.log("[ServiceWorker] Caching static assets");
                return cache.addAll(STATIC_ASSETS);
            })
            .catch((error) => {
                console.error(
                    "[ServiceWorker] Failed to cache static assets:",
                    error
                );
            })
    );

    // Force the service worker to take control immediately
    self.skipWaiting();
});

// Activate event - clean up old caches
self.addEventListener("activate", (event) => {
    console.log("[ServiceWorker] Activating...");

    event.waitUntil(
        caches
            .keys()
            .then((cacheNames) => {
                return Promise.all(
                    cacheNames.map((cacheName) => {
                        if (
                            cacheName !== STATIC_CACHE &&
                            cacheName !== DYNAMIC_CACHE
                        ) {
                            console.log(
                                "[ServiceWorker] Deleting old cache:",
                                cacheName
                            );
                            return caches.delete(cacheName);
                        }
                    })
                );
            })
            .then(() => {
                console.log("[ServiceWorker] Ready to handle fetches");
                return self.clients.claim();
            })
    );
});

// Fetch event - serve from cache with fallback strategies
self.addEventListener("fetch", (event) => {
    const request = event.request;
    const url = new URL(request.url);

    // Skip non-GET requests and chrome-extension URLs
    if (request.method !== "GET" || url.protocol === "chrome-extension:") {
        return;
    }

    event.respondWith(handleFetchRequest(request));
});

async function handleFetchRequest(request) {
    const url = new URL(request.url);

    try {
        // Static assets: Cache first, then network
        if (isStaticAsset(request.url)) {
            return await cacheFirst(request);
        }

        // API calls or dynamic content: Network first, then cache
        if (isApiCall(request.url) || isDynamicContent(request.url)) {
            return await networkFirst(request);
        }

        // Default: Try cache first, fallback to network
        return await cacheFirst(request);
    } catch (error) {
        console.error("[ServiceWorker] Fetch error:", error);

        // Return offline fallback if available
        if (request.destination === "document") {
            return await getOfflineFallback();
        }

        throw error;
    }
}

// Cache first strategy (for static assets)
async function cacheFirst(request) {
    const cachedResponse = await caches.match(request);

    if (cachedResponse) {
        return cachedResponse;
    }

    // Not in cache, fetch from network and cache
    const networkResponse = await fetch(request);

    if (networkResponse.ok && request.url.startsWith("http")) {
        const cache = await caches.open(STATIC_CACHE);
        cache.put(request, networkResponse.clone());
    }

    return networkResponse;
}

// Network first strategy (for dynamic content)
async function networkFirst(request) {
    try {
        const networkResponse = await fetch(request);

        if (networkResponse.ok && request.url.startsWith("http")) {
            const cache = await caches.open(DYNAMIC_CACHE);
            cache.put(request, networkResponse.clone());
        }

        return networkResponse;
    } catch (error) {
        // Network failed, try cache
        const cachedResponse = await caches.match(request);

        if (cachedResponse) {
            return cachedResponse;
        }

        throw error;
    }
}

// Helper functions
function isStaticAsset(url) {
    return (
        url.includes("/pkg/") ||
        url.includes(".wasm") ||
        url.includes(".js") ||
        url.includes(".css") ||
        url.includes("cdn.jsdelivr.net")
    );
}

function isApiCall(url) {
    return url.includes("/api/") || url.includes("api.");
}

function isDynamicContent(url) {
    return url.includes("?") && !isStaticAsset(url);
}

async function getOfflineFallback() {
    const cache = await caches.open(STATIC_CACHE);
    return (
        (await cache.match("/index.html")) ||
        (await cache.match("/")) ||
        new Response("Offline - The Hack: Ghost Protocol", {
            status: 200,
            headers: { "Content-Type": "text/html" },
        })
    );
}

// Background sync for game saves (when online again)
self.addEventListener("sync", (event) => {
    console.log("[ServiceWorker] Background sync:", event.tag);

    if (event.tag === "game-save-sync") {
        event.waitUntil(syncGameSaves());
    }
});

async function syncGameSaves() {
    try {
        // Get pending saves from IndexedDB
        const pendingSaves = await getPendingGameSaves();

        if (pendingSaves.length > 0) {
            console.log(
                "[ServiceWorker] Syncing",
                pendingSaves.length,
                "game saves"
            );

            for (const save of pendingSaves) {
                await syncSingleSave(save);
            }

            // Clear pending saves
            await clearPendingGameSaves();
        }
    } catch (error) {
        console.error("[ServiceWorker] Game save sync failed:", error);
    }
}

// Push notifications (for updates and achievements)
self.addEventListener("push", (event) => {
    if (!event.data) return;

    const data = event.data.json();

    const options = {
        body: data.body || "New content available in The Hack: Ghost Protocol",
        icon: "/icons/icon-192x192.png",
        badge: "/icons/icon-72x72.png",
        image: data.image,
        vibrate: [200, 100, 200],
        data: data.data || {},
        actions: [
            {
                action: "open",
                title: "Open Game",
                icon: "/icons/shortcut-continue.png",
            },
            {
                action: "dismiss",
                title: "Dismiss",
                icon: "/icons/shortcut-close.png",
            },
        ],
        tag: data.tag || "hack-ghost-notification",
        renotify: true,
        requireInteraction: false,
    };

    event.waitUntil(
        self.registration.showNotification(
            data.title || "The Hack: Ghost Protocol",
            options
        )
    );
});

// Handle notification clicks
self.addEventListener("notificationclick", (event) => {
    const notification = event.notification;
    const action = event.action;

    notification.close();

    if (action === "open" || !action) {
        event.waitUntil(
            clients.matchAll().then((clientList) => {
                // If app is already open, focus it
                for (const client of clientList) {
                    if (
                        client.url.includes("hack.andernet.dev") &&
                        "focus" in client
                    ) {
                        return client.focus();
                    }
                }

                // Otherwise open new window
                if (clients.openWindow) {
                    return clients.openWindow("/");
                }
            })
        );
    }
});

// Handle periodic background sync (for checking updates)
self.addEventListener("periodicsync", (event) => {
    if (event.tag === "content-sync") {
        event.waitUntil(checkForUpdates());
    }
});

async function checkForUpdates() {
    try {
        // Check for new challenges or game updates
        const response = await fetch("/api/version-check");
        const data = await response.json();

        if (data.hasUpdates) {
            await self.registration.showNotification("New Content Available!", {
                body: "New challenges and features are ready in The Hack: Ghost Protocol",
                icon: "/icons/icon-192x192.png",
                tag: "content-update",
            });
        }
    } catch (error) {
        console.log("[ServiceWorker] Update check failed (offline)");
    }
}

// Utility functions for IndexedDB (game save sync)
async function getPendingGameSaves() {
    // This would integrate with IndexedDB to store offline saves
    return [];
}

async function syncSingleSave(save) {
    // Sync individual save to cloud/server
    console.log("[ServiceWorker] Syncing save:", save.id);
}

async function clearPendingGameSaves() {
    // Clear synced saves from IndexedDB
    console.log("[ServiceWorker] Cleared pending saves");
}
