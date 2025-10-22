#!/usr/bin/env pwsh
# Build optimized release binaries
# Usage: ./build-release.ps1

Write-Host "🏗️  Building release version..." -ForegroundColor Cyan
Write-Host ""

$startTime = Get-Date

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
cargo clean --release

# Build release
Write-Host "Building optimized binary..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -eq 0) {
    $endTime = Get-Date
    $duration = $endTime - $startTime

    Write-Host ""
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host "⏱️  Build time: $($duration.TotalSeconds.ToString('0.00'))s" -ForegroundColor Cyan
    Write-Host ""

    # Show binary info
    $binary = "target/release/hack_simulator.exe"
    if (Test-Path $binary) {
        $size = (Get-Item $binary).Length / 1MB
        Write-Host "📦 Binary location: $binary" -ForegroundColor Cyan
        Write-Host "📊 Binary size: $($size.ToString('0.00')) MB" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Run with: .\$binary" -ForegroundColor Yellow
    }
}
else {
    Write-Host ""
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
