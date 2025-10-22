#!/usr/bin/env pwsh
# Test runner with watch mode for continuous testing
# Usage: ./test-watch.ps1 [filter]

param(
    [string]$Filter = ""
)

Write-Host "🔍 Starting test watch mode..." -ForegroundColor Cyan
Write-Host ""

if (Get-Command cargo-watch -ErrorAction SilentlyContinue) {
    if ($Filter) {
        Write-Host "Watching tests matching: $Filter" -ForegroundColor Yellow
        cargo watch -x "test $Filter"
    }
    else {
        Write-Host "Watching all tests..." -ForegroundColor Yellow
        cargo watch -x test
    }
}
else {
    Write-Host "❌ cargo-watch not installed!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Install with: cargo install cargo-watch" -ForegroundColor Yellow
    exit 1
}
