#!/usr/bin/env pwsh
# Quick development check - runs fmt, clippy, and tests
# Usage: ./quick-check.ps1

Write-Host "🚀 Running quick development check..." -ForegroundColor Cyan
Write-Host ""

$startTime = Get-Date

# Step 1: Format code
Write-Host "1️⃣  Formatting code..." -ForegroundColor Yellow
cargo fmt
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Formatting failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Code formatted" -ForegroundColor Green
Write-Host ""

# Step 2: Run clippy
Write-Host "2️⃣  Running clippy..." -ForegroundColor Yellow
cargo clippy --quiet -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy found issues!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Clippy passed" -ForegroundColor Green
Write-Host ""

# Step 3: Run tests
Write-Host "3️⃣  Running tests..." -ForegroundColor Yellow
cargo test --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ All tests passed" -ForegroundColor Green
Write-Host ""

$endTime = Get-Date
$duration = $endTime - $startTime

Write-Host "🎉 All checks passed!" -ForegroundColor Green
Write-Host "⏱️  Total time: $($duration.TotalSeconds.ToString('0.00'))s" -ForegroundColor Cyan
