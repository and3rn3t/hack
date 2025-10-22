#!/usr/bin/env pwsh
# Clean all build artifacts and start fresh
# Usage: ./clean-all.ps1

Write-Host "🧹 Cleaning all build artifacts..." -ForegroundColor Cyan
Write-Host ""

# Clean Cargo artifacts
Write-Host "Cleaning Cargo build files..." -ForegroundColor Yellow
cargo clean

# Clean coverage reports
if (Test-Path "coverage") {
    Write-Host "Removing coverage reports..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force "coverage"
}

# Clean save files (optional - ask user)
if (Test-Path "game_save.json") {
    $clean = Read-Host "Remove game save file? (y/n)"
    if ($clean -eq 'y') {
        Remove-Item "game_save.json"
        Write-Host "Removed game_save.json" -ForegroundColor Yellow
    }
}

# Clean test artifacts
$testFiles = Get-ChildItem -Filter "test_*.json"
if ($testFiles.Count -gt 0) {
    Write-Host "Removing test artifacts..." -ForegroundColor Yellow
    $testFiles | Remove-Item -Force
}

Write-Host ""
Write-Host "✅ Cleanup complete!" -ForegroundColor Green
