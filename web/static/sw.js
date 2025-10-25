// Service Worker for The Hack: Ghost Protocol PWA
// Provides offline functionality and caching for enhanced user experience

const VERSION = "1.3.0";
const CACHE_NAME = `hack-ghost-protocol-v${VERSION}`;
const STATIC_CACHE = `hack-ghost-static-v${VERSION}`;
const DYNAMIC_CACHE = `hack-ghost-dynamic-v${VERSION}`;
const RUNTIME_CACHE = `hack-ghost-runtime-v${VERSION}`;

// Enhanced cache configuration
const CACHE_CONFIG = {
    // Static assets that rarely change
    STATIC_TTL: 7 * 24 * 60 * 60 * 1000, // 7 days
    // Dynamic content
    DYNAMIC_TTL: 24 * 60 * 60 * 1000, // 1 day
    // Runtime cache
    RUNTIME_TTL: 60 * 60 * 1000, // 1 hour
    // Maximum entries per cache
    MAX_ENTRIES: {
        static: 100,
        dynamic: 50,
        runtime: 30,
    },
};

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

// Precache critical resources for instant loading
const CRITICAL_RESOURCES = ["/", "/index.html", "/game.js", "/manifest.json"];

// Lazy load resources
const LAZY_LOAD_RESOURCES = [
    "/pkg/hack_simulator.js",
    "/pkg/hack_simulator_bg.wasm",
];

// Install event - cache critical assets first, then lazy load others
self.addEventListener("install", (event) => {
    console.log("[ServiceWorker] Installing v" + VERSION + "...");

    event.waitUntil(
        Promise.all([
            // Cache critical resources first for fast startup
            caches.open(STATIC_CACHE).then((cache) => {
                console.log("[ServiceWorker] Caching critical resources");
                return cache.addAll(CRITICAL_RESOURCES);
            }),
            // Pre-cache external dependencies
            caches.open(STATIC_CACHE).then((cache) => {
                console.log("[ServiceWorker] Caching external dependencies");
                const externalAssets = STATIC_ASSETS.filter((url) =>
                    url.startsWith("http")
                );
                return cache.addAll(externalAssets).catch((error) => {
                    console.warn(
                        "[ServiceWorker] Some external assets failed to cache:",
                        error
                    );
                });
            }),
            // Initialize IndexedDB for offline game saves
            initializeOfflineStorage(),
        ])
            .then(() => {
                console.log(
                    "[ServiceWorker] Critical resources cached successfully"
                );
                // Lazy load remaining assets in background
                setTimeout(() => lazyLoadAssets(), 1000);
            })
            .catch((error) => {
                console.error("[ServiceWorker] Installation failed:", error);
            })
    );

    // Force the service worker to take control immediately
    self.skipWaiting();
});

// Lazy load non-critical assets in background
async function lazyLoadAssets() {
    try {
        const cache = await caches.open(STATIC_CACHE);
        console.log("[ServiceWorker] Lazy loading remaining assets");

        for (const asset of LAZY_LOAD_RESOURCES) {
            try {
                await cache.add(asset);
                console.log(`[ServiceWorker] Lazy loaded: ${asset}`);
            } catch (error) {
                console.warn(
                    `[ServiceWorker] Failed to lazy load ${asset}:`,
                    error
                );
            }
        }
    } catch (error) {
        console.error("[ServiceWorker] Lazy loading failed:", error);
    }
}

// Initialize offline storage
async function initializeOfflineStorage() {
    if ("indexedDB" in self) {
        try {
            // Initialize IndexedDB for game saves, settings, and offline data
            const request = indexedDB.open("HackGhostProtocolDB", 1);

            request.onupgradeneeded = (event) => {
                const db = event.target.result;

                // Game saves store
                if (!db.objectStoreNames.contains("gameSaves")) {
                    const saveStore = db.createObjectStore("gameSaves", {
                        keyPath: "id",
                    });
                    saveStore.createIndex("timestamp", "timestamp", {
                        unique: false,
                    });
                }

                // Settings store
                if (!db.objectStoreNames.contains("settings")) {
                    db.createObjectStore("settings", { keyPath: "key" });
                }

                // Offline queue for background sync
                if (!db.objectStoreNames.contains("offlineQueue")) {
                    const queueStore = db.createObjectStore("offlineQueue", {
                        keyPath: "id",
                        autoIncrement: true,
                    });
                    queueStore.createIndex("timestamp", "timestamp", {
                        unique: false,
                    });
                }

                console.log("[ServiceWorker] IndexedDB initialized");
            };

            return new Promise((resolve, reject) => {
                request.onsuccess = () => resolve(request.result);
                request.onerror = () => reject(request.error);
            });
        } catch (error) {
            console.warn("[ServiceWorker] IndexedDB not available:", error);
        }
    }
}

// Activate event - clean up old caches and initialize new features
self.addEventListener("activate", (event) => {
    console.log("[ServiceWorker] Activating v" + VERSION + "...");

    event.waitUntil(
        Promise.all([
            // Clean up old caches
            caches.keys().then((cacheNames) => {
                return Promise.all(
                    cacheNames.map((cacheName) => {
                        if (
                            cacheName !== STATIC_CACHE &&
                            cacheName !== DYNAMIC_CACHE &&
                            cacheName !== RUNTIME_CACHE &&
                            (cacheName.includes("hack-ghost") ||
                                cacheName.includes("hack_ghost"))
                        ) {
                            console.log(
                                "[ServiceWorker] Deleting old cache:",
                                cacheName
                            );
                            return caches.delete(cacheName);
                        }
                    })
                );
            }),

            // Clean up expired cache entries
            cleanupExpiredCacheEntries(),

            // Initialize background sync registration
            registerBackgroundSync(),

            // Set up periodic sync for content updates
            setupPeriodicSync(),
        ])
            .then(() => {
                console.log(
                    "[ServiceWorker] Activation complete - Ready to handle requests"
                );

                // Notify all clients about the update
                return self.clients.matchAll().then((clients) => {
                    clients.forEach((client) => {
                        client.postMessage({
                            type: "SW_ACTIVATED",
                            version: VERSION,
                            message: "Service worker updated successfully!",
                        });
                    });
                });
            })
            .then(() => {
                return self.clients.claim();
            })
    );
});

// Clean up expired cache entries to prevent unlimited growth
async function cleanupExpiredCacheEntries() {
    try {
        const cacheNames = [STATIC_CACHE, DYNAMIC_CACHE, RUNTIME_CACHE];
        const now = Date.now();

        for (const cacheName of cacheNames) {
            const cache = await caches.open(cacheName);
            const keys = await cache.keys();

            // Get cache type for TTL
            const ttl = cacheName.includes("static")
                ? CACHE_CONFIG.STATIC_TTL
                : cacheName.includes("dynamic")
                ? CACHE_CONFIG.DYNAMIC_TTL
                : CACHE_CONFIG.RUNTIME_TTL;

            const maxEntries = cacheName.includes("static")
                ? CACHE_CONFIG.MAX_ENTRIES.static
                : cacheName.includes("dynamic")
                ? CACHE_CONFIG.MAX_ENTRIES.dynamic
                : CACHE_CONFIG.MAX_ENTRIES.runtime;

            // Remove expired entries
            for (const request of keys) {
                const response = await cache.match(request);
                if (response) {
                    const cachedTime = response.headers.get("sw-cache-time");
                    if (cachedTime && now - parseInt(cachedTime) > ttl) {
                        await cache.delete(request);
                        console.log(
                            `[ServiceWorker] Removed expired cache entry: ${request.url}`
                        );
                    }
                }
            }

            // Enforce max entries limit
            const remainingKeys = await cache.keys();
            if (remainingKeys.length > maxEntries) {
                const keysToDelete = remainingKeys.slice(
                    0,
                    remainingKeys.length - maxEntries
                );
                for (const key of keysToDelete) {
                    await cache.delete(key);
                }
                console.log(
                    `[ServiceWorker] Cleaned up ${keysToDelete.length} excess entries from ${cacheName}`
                );
            }
        }
    } catch (error) {
        console.error("[ServiceWorker] Cache cleanup failed:", error);
    }
}

// Register background sync for offline operations
async function registerBackgroundSync() {
    if ("sync" in self.registration) {
        try {
            console.log("[ServiceWorker] Background sync registered");
        } catch (error) {
            console.warn(
                "[ServiceWorker] Background sync registration failed:",
                error
            );
        }
    }
}

// Setup periodic sync for content updates (if supported)
async function setupPeriodicSync() {
    if ("periodicSync" in self.registration) {
        try {
            await self.registration.periodicSync.register("content-sync", {
                minInterval: 24 * 60 * 60 * 1000, // 24 hours
            });
            console.log(
                "[ServiceWorker] Periodic sync registered for content updates"
            );
        } catch (error) {
            console.warn("[ServiceWorker] Periodic sync not supported:", error);
        }
    }
}

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

// Cache first strategy (for static assets) with enhanced metadata
async function cacheFirst(request) {
    const cachedResponse = await caches.match(request);

    if (cachedResponse) {
        // Check if cache entry is still fresh
        const cachedTime = cachedResponse.headers.get("sw-cache-time");
        if (cachedTime) {
            const age = Date.now() - parseInt(cachedTime);
            const isStale =
                age >
                (isStaticAsset(request.url)
                    ? CACHE_CONFIG.STATIC_TTL
                    : CACHE_CONFIG.DYNAMIC_TTL);

            if (!isStale) {
                return cachedResponse;
            }

            // Serve stale content while updating in background
            updateCacheInBackground(request);
        }

        return cachedResponse;
    }

    // Not in cache, fetch from network and cache
    const networkResponse = await fetch(request);

    if (networkResponse.ok && request.url.startsWith("http")) {
        const responseToCache = networkResponse.clone();

        // Add cache metadata
        const modifiedResponse = new Response(responseToCache.body, {
            status: responseToCache.status,
            statusText: responseToCache.statusText,
            headers: {
                ...Object.fromEntries(responseToCache.headers.entries()),
                "sw-cache-time": Date.now().toString(),
                "sw-cache-version": VERSION,
            },
        });

        const cacheName = isStaticAsset(request.url)
            ? STATIC_CACHE
            : RUNTIME_CACHE;
        const cache = await caches.open(cacheName);
        cache.put(request, modifiedResponse);
    }

    return networkResponse;
}

// Update cache in background (stale-while-revalidate pattern)
async function updateCacheInBackground(request) {
    try {
        const networkResponse = await fetch(request);
        if (networkResponse.ok) {
            const responseToCache = networkResponse.clone();
            const modifiedResponse = new Response(responseToCache.body, {
                status: responseToCache.status,
                statusText: responseToCache.statusText,
                headers: {
                    ...Object.fromEntries(responseToCache.headers.entries()),
                    "sw-cache-time": Date.now().toString(),
                    "sw-cache-version": VERSION,
                },
            });

            const cacheName = isStaticAsset(request.url)
                ? STATIC_CACHE
                : RUNTIME_CACHE;
            const cache = await caches.open(cacheName);
            await cache.put(request, modifiedResponse);

            console.log(`[ServiceWorker] Background updated: ${request.url}`);
        }
    } catch (error) {
        console.log(
            `[ServiceWorker] Background update failed for ${request.url}:`,
            error
        );
    }
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

// Enhanced push notifications with better targeting and personalization
self.addEventListener("push", (event) => {
    if (!event.data) {
        // Handle server-sent notifications without data
        const options = {
            body: "New content is available in The Hack: Ghost Protocol",
            icon: "/icons/icon-192x192.png",
            badge: "/icons/icon-72x72.png",
            tag: "general-update",
            data: { action: "open" },
        };

        event.waitUntil(
            self.registration.showNotification(
                "The Hack: Ghost Protocol",
                options
            )
        );
        return;
    }

    const data = event.data.json();

    // Enhanced notification options based on type
    const baseOptions = {
        icon: "/icons/icon-192x192.png",
        badge: "/icons/icon-72x72.png",
        vibrate: [200, 100, 200],
        data: data.data || {},
        tag: data.tag || "hack-ghost-notification",
        timestamp: Date.now(),
    };

    // Customize notification based on type
    let options = { ...baseOptions };

    switch (data.type) {
        case "achievement":
            options = {
                ...options,
                body: data.body || "Achievement unlocked!",
                icon: "/icons/achievement-icon.png",
                vibrate: [100, 50, 100, 50, 100],
                actions: [
                    {
                        action: "view-achievements",
                        title: "View Achievements",
                        icon: "/icons/trophy.png",
                    },
                    {
                        action: "continue-playing",
                        title: "Continue Playing",
                        icon: "/icons/play.png",
                    },
                ],
                requireInteraction: true,
                silent: false,
            };
            break;

        case "challenge":
            options = {
                ...options,
                body: data.body || "New challenges available!",
                image: data.image,
                actions: [
                    {
                        action: "start-challenge",
                        title: "Start Challenge",
                        icon: "/icons/challenge.png",
                    },
                    {
                        action: "view-all",
                        title: "View All",
                        icon: "/icons/list.png",
                    },
                ],
                requireInteraction: false,
            };
            break;

        case "update":
            options = {
                ...options,
                body:
                    data.body || "The app has been updated with new features!",
                actions: [
                    {
                        action: "reload",
                        title: "Reload App",
                        icon: "/icons/reload.png",
                    },
                    {
                        action: "changelog",
                        title: "What's New",
                        icon: "/icons/info.png",
                    },
                ],
                requireInteraction: true,
            };
            break;

        case "reminder":
            options = {
                ...options,
                body:
                    data.body ||
                    "Ready to continue your cybersecurity journey?",
                actions: [
                    {
                        action: "continue",
                        title: "Continue Game",
                        icon: "/icons/continue.png",
                    },
                    {
                        action: "snooze",
                        title: "Remind Later",
                        icon: "/icons/snooze.png",
                    },
                ],
                requireInteraction: false,
            };
            break;

        default:
            options = {
                ...options,
                body:
                    data.body ||
                    "New content available in The Hack: Ghost Protocol",
                image: data.image,
                actions: [
                    {
                        action: "open",
                        title: "Open Game",
                        icon: "/icons/shortcut-continue.png",
                    },
                    {
                        action: "dismiss",
                        title: "Dismiss",
                        icon: "/icons/close.png",
                    },
                ],
                requireInteraction: false,
            };
    }

    event.waitUntil(
        Promise.all([
            self.registration.showNotification(
                data.title || "The Hack: Ghost Protocol",
                options
            ),

            // Log notification for analytics (privacy-friendly)
            logNotificationEvent(data.type, data.tag),
        ])
    );
});

// Privacy-friendly notification analytics
async function logNotificationEvent(type, tag) {
    try {
        // Store minimal analytics in IndexedDB (no personal data)
        const db = await openOfflineDB();
        const transaction = db.transaction(["settings"], "readwrite");
        const store = transaction.objectStore("settings");

        // Get or initialize notification stats
        const statsKey = "notification_stats";
        let stats = (await getFromStore(store, statsKey)) || {
            total_sent: 0,
            by_type: {},
            last_sent: null,
        };

        stats.total_sent++;
        stats.by_type[type] = (stats.by_type[type] || 0) + 1;
        stats.last_sent = Date.now();

        await store.put({ key: statsKey, value: stats });
    } catch (error) {
        console.log("[ServiceWorker] Notification analytics failed:", error);
    }
}

// Enhanced notification click handling with multiple action support
self.addEventListener("notificationclick", (event) => {
    const notification = event.notification;
    const action = event.action;
    const data = notification.data || {};

    notification.close();

    event.waitUntil(handleNotificationAction(action, data));
});

async function handleNotificationAction(action, data) {
    const clients = await self.clients.matchAll();
    let gameClient = null;

    // Find existing game client
    for (const client of clients) {
        if (
            client.url.includes("hack") ||
            client.url === self.location.origin + "/"
        ) {
            gameClient = client;
            break;
        }
    }

    switch (action) {
        case "open":
        case "continue-playing":
        case "continue":
            if (gameClient) {
                await gameClient.focus();
                gameClient.postMessage({
                    type: "NOTIFICATION_ACTION",
                    action: "continue_game",
                    data: data,
                });
            } else {
                await clients.openWindow("/");
            }
            break;

        case "view-achievements":
            if (gameClient) {
                await gameClient.focus();
                gameClient.postMessage({
                    type: "NOTIFICATION_ACTION",
                    action: "show_achievements",
                    data: data,
                });
            } else {
                await clients.openWindow("/?action=achievements");
            }
            break;

        case "start-challenge":
            if (gameClient) {
                await gameClient.focus();
                gameClient.postMessage({
                    type: "NOTIFICATION_ACTION",
                    action: "start_challenge",
                    challengeId: data.challengeId,
                    data: data,
                });
            } else {
                await clients.openWindow(
                    `/?action=challenge&id=${data.challengeId || ""}`
                );
            }
            break;

        case "view-all":
            if (gameClient) {
                await gameClient.focus();
                gameClient.postMessage({
                    type: "NOTIFICATION_ACTION",
                    action: "show_challenges",
                    data: data,
                });
            } else {
                await clients.openWindow("/?action=challenges");
            }
            break;

        case "reload":
            if (gameClient) {
                gameClient.postMessage({
                    type: "NOTIFICATION_ACTION",
                    action: "reload_app",
                    data: data,
                });
            } else {
                await clients.openWindow("/");
            }
            break;

        case "changelog":
            await clients.openWindow("/?action=changelog");
            break;

        case "snooze":
            // Schedule reminder for later (store in IndexedDB)
            await scheduleReminder(data.snoozeMinutes || 60);
            break;

        case "dismiss":
            // Just close - no action needed
            break;

        default:
            // Default action - open game
            if (gameClient) {
                await gameClient.focus();
            } else {
                await clients.openWindow("/");
            }
    }
}

// Schedule reminder notification
async function scheduleReminder(minutesFromNow) {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["offlineQueue"], "readwrite");
        const store = transaction.objectStore("offlineQueue");

        const reminder = {
            type: "reminder",
            scheduledFor: Date.now() + minutesFromNow * 60 * 1000,
            data: {
                title: "Ready to hack?",
                body: "Your cybersecurity adventure awaits!",
                type: "reminder",
            },
        };

        await store.add(reminder);
        console.log(
            `[ServiceWorker] Reminder scheduled for ${minutesFromNow} minutes`
        );
    } catch (error) {
        console.error("[ServiceWorker] Failed to schedule reminder:", error);
    }
}

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

// Enhanced IndexedDB utilities for offline functionality
async function openOfflineDB() {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open("HackGhostProtocolDB", 1);
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
    });
}

async function getFromStore(store, key) {
    return new Promise((resolve, reject) => {
        const request = store.get(key);
        request.onsuccess = () => resolve(request.result?.value);
        request.onerror = () => reject(request.error);
    });
}

// Game save synchronization utilities
async function getPendingGameSaves() {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["offlineQueue"], "readonly");
        const store = transaction.objectStore("offlineQueue");

        return new Promise((resolve, reject) => {
            const saves = [];
            const request = store.openCursor();

            request.onsuccess = (event) => {
                const cursor = event.target.result;
                if (cursor) {
                    if (cursor.value.type === "game_save") {
                        saves.push(cursor.value);
                    }
                    cursor.continue();
                } else {
                    resolve(saves);
                }
            };

            request.onerror = () => reject(request.error);
        });
    } catch (error) {
        console.error("[ServiceWorker] Failed to get pending saves:", error);
        return [];
    }
}

async function syncSingleSave(save) {
    try {
        console.log("[ServiceWorker] Syncing save:", save.id);

        // In a real implementation, this would sync to a backend
        // For now, we'll just mark it as synced locally
        const db = await openOfflineDB();
        const transaction = db.transaction(["gameSaves"], "readwrite");
        const store = transaction.objectStore("gameSaves");

        const syncedSave = {
            ...save.data,
            id: save.id,
            synced: true,
            lastSync: Date.now(),
        };

        await store.put(syncedSave);
        console.log("[ServiceWorker] Save synced successfully:", save.id);

        return true;
    } catch (error) {
        console.error("[ServiceWorker] Save sync failed:", error);
        return false;
    }
}

async function clearPendingGameSaves() {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["offlineQueue"], "readwrite");
        const store = transaction.objectStore("offlineQueue");

        // Remove all synced game saves from queue
        const request = store.openCursor();

        return new Promise((resolve, reject) => {
            request.onsuccess = (event) => {
                const cursor = event.target.result;
                if (cursor) {
                    if (
                        cursor.value.type === "game_save" &&
                        cursor.value.synced
                    ) {
                        cursor.delete();
                    }
                    cursor.continue();
                } else {
                    console.log("[ServiceWorker] Cleared synced pending saves");
                    resolve();
                }
            };

            request.onerror = () => reject(request.error);
        });
    } catch (error) {
        console.error("[ServiceWorker] Failed to clear pending saves:", error);
    }
}

// Enhanced offline storage functions
async function storeGameSave(saveData) {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["gameSaves"], "readwrite");
        const store = transaction.objectStore("gameSaves");

        const save = {
            id: `save_${Date.now()}`,
            data: saveData,
            timestamp: Date.now(),
            synced: navigator.onLine, // Mark as synced if online
        };

        await store.put(save);

        // If offline, add to sync queue
        if (!navigator.onLine) {
            const queueTransaction = db.transaction(
                ["offlineQueue"],
                "readwrite"
            );
            const queueStore = queueTransaction.objectStore("offlineQueue");

            await queueStore.add({
                type: "game_save",
                id: save.id,
                data: saveData,
                timestamp: Date.now(),
            });
        }

        return save.id;
    } catch (error) {
        console.error("[ServiceWorker] Failed to store game save:", error);
        throw error;
    }
}

async function getStoredGameSaves() {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["gameSaves"], "readonly");
        const store = transaction.objectStore("gameSaves");

        return new Promise((resolve, reject) => {
            const saves = [];
            const request = store.openCursor();

            request.onsuccess = (event) => {
                const cursor = event.target.result;
                if (cursor) {
                    saves.push(cursor.value);
                    cursor.continue();
                } else {
                    // Sort by timestamp, most recent first
                    saves.sort((a, b) => b.timestamp - a.timestamp);
                    resolve(saves);
                }
            };

            request.onerror = () => reject(request.error);
        });
    } catch (error) {
        console.error("[ServiceWorker] Failed to get stored saves:", error);
        return [];
    }
}

// Store user settings offline
async function storeSettings(key, value) {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["settings"], "readwrite");
        const store = transaction.objectStore("settings");

        await store.put({ key, value, timestamp: Date.now() });
    } catch (error) {
        console.error("[ServiceWorker] Failed to store settings:", error);
    }
}

async function getSettings(key) {
    try {
        const db = await openOfflineDB();
        const transaction = db.transaction(["settings"], "readonly");
        const store = transaction.objectStore("settings");

        const result = await getFromStore(store, key);
        return result;
    } catch (error) {
        console.error("[ServiceWorker] Failed to get settings:", error);
        return null;
    }
}
