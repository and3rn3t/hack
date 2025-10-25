// Test setup for Jest
import { jest } from '@jest/globals';

// Mock WebAssembly module for testing
global.mockWasmModule = {
    WebGameEngine: jest.fn(() => ({
        get_introduction: jest.fn(() => "Mock introduction"),
        get_challenges_json: jest.fn(() => JSON.stringify([
            {
                id: "test_challenge",
                title: "Test Challenge",
                level: 0,
                category: "Encoding", 
                description: "A test challenge",
                solution: "test_answer",
                hints: ["Test hint"]
            }
        ])),
        validate_challenge_answer: jest.fn((id, answer, state) => JSON.stringify({
            success: answer === "test_answer",
            message: answer === "test_answer" ? "Correct!" : "Wrong answer",
            leveled_up: false,
            game_over: false
        }))
    })),
    WebGameState: jest.fn(() => ({
        get_level: jest.fn(() => 0),
        get_sanity: jest.fn(() => 100),
        get_xp: jest.fn(() => 0),
        get_stats_json: jest.fn(() => JSON.stringify({
            level: 0,
            xp: 0,
            sanity: 100,
            completed_challenges: []
        })),
        get_completed_challenges: jest.fn(() => []),
        can_attempt_challenge: jest.fn(() => true),
        add_completed_challenge: jest.fn(),
        add_xp: jest.fn(),
        decrease_sanity: jest.fn(),
        save_to_json: jest.fn(() => "{}"),
        load_from_json: jest.fn(() => true)
    })),
    default: jest.fn(() => Promise.resolve())
};

// Mock xterm.js
global.Terminal = jest.fn(() => ({
    open: jest.fn(),
    write: jest.fn(),
    writeln: jest.fn(),
    clear: jest.fn(),
    onData: jest.fn(),
    onKey: jest.fn(),
    loadAddon: jest.fn(),
    focus: jest.fn(),
    options: {}
}));

// Mock FitAddon
global.FitAddon = jest.fn(() => ({
    fit: jest.fn()
}));

// Mock Web APIs
global.localStorage = {
    getItem: jest.fn(),
    setItem: jest.fn(),
    removeItem: jest.fn(),
    clear: jest.fn()
};

global.navigator = {
    clipboard: {
        writeText: jest.fn(() => Promise.resolve())
    }
};

global.AudioContext = jest.fn(() => ({
    createOscillator: jest.fn(() => ({
        connect: jest.fn(),
        start: jest.fn(),
        stop: jest.fn(),
        frequency: { setValueAtTime: jest.fn() },
        type: 'sine'
    })),
    createGain: jest.fn(() => ({
        connect: jest.fn(),
        gain: { 
            setValueAtTime: jest.fn(),
            exponentialRampToValueAtTime: jest.fn()
        }
    })),
    createBiquadFilter: jest.fn(() => ({
        connect: jest.fn(),
        type: 'lowpass',
        frequency: { setValueAtTime: jest.fn() }
    })),
    destination: {},
    currentTime: 0
}));

// Mock document methods
document.querySelector = jest.fn(() => ({
    style: {},
    classList: {
        add: jest.fn(),
        remove: jest.fn()
    },
    appendChild: jest.fn(),
    remove: jest.fn()
}));

document.createElement = jest.fn((tag) => ({
    type: '',
    accept: '',
    style: { display: '' },
    href: '',
    download: '',
    click: jest.fn(),
    remove: jest.fn(),
    addEventListener: jest.fn(),
    files: []
}));

document.body = {
    appendChild: jest.fn(),
    classList: {
        add: jest.fn(),
        remove: jest.fn()
    }
};

// Suppress console.log during tests unless needed
global.console = {
    ...console,
    log: jest.fn(),
    warn: jest.fn(),
    error: jest.fn()
};