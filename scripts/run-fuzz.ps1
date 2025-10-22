#!/usr/bin/env pwsh
# Run fuzzing tests with cargo-fuzz
# Note: Requires nightly Rust toolchain

param(
    [switch]$Install,
    [string]$Target = "fuzz_challenge_validators",
    [int]$Seconds = 60
)

if ($Install) {
    Write-Host "📦 Installing cargo-fuzz..." -ForegroundColor Cyan
    cargo install cargo-fuzz
    exit 0
}

Write-Host "🎲 Running fuzzing tests..." -ForegroundColor Cyan
Write-Host ""

# Check for nightly toolchain
$null = cargo +nightly --version 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Nightly Rust toolchain required!" -ForegroundColor Red
    Write-Host "💡 Install with: rustup install nightly" -ForegroundColor Yellow
    exit 1
}

Write-Host "Target: $Target" -ForegroundColor Yellow
Write-Host "Duration: $Seconds seconds" -ForegroundColor Yellow
Write-Host ""

try {
    # Run fuzzer
    cargo +nightly fuzz run $Target -- -max_total_time=$Seconds

    Write-Host ""
    Write-Host "✅ Fuzzing complete!" -ForegroundColor Green
    Write-Host "📊 Results saved to: fuzz/corpus/$Target/" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Available targets:" -ForegroundColor Cyan
    Write-Host "  - fuzz_challenge_validators" -ForegroundColor White
    Write-Host "  - fuzz_state_deserialization" -ForegroundColor White
    Write-Host "  - fuzz_state_operations" -ForegroundColor White

}
catch {
    Write-Host ""
    Write-Host "❌ Fuzzing failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "💡 Install cargo-fuzz with: pwsh scripts/run-fuzz.ps1 -Install" -ForegroundColor Yellow
    Write-Host "💡 Install nightly Rust with: rustup install nightly" -ForegroundColor Yellow
    exit 1
}
