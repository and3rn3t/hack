#!/usr/bin/env pwsh
# Quick play script for The Hack: Ghost Protocol

Write-Host "🎮 Starting The Hack: Ghost Protocol..." -ForegroundColor Cyan
Write-Host ""

# Run the game
cargo run --release

# When game exits, show message
Write-Host ""
Write-Host "👻 Thanks for playing! The Ghost Protocol awaits your return..." -ForegroundColor Magenta
Write-Host ""
