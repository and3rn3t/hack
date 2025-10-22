#!/usr/bin/env pwsh
# Run specific test with verbose output
# Usage: ./test-verbose.ps1 [test_name]

param(
    [string]$TestName = ""
)

if ($TestName) {
    Write-Host "🔍 Running test: $TestName" -ForegroundColor Cyan
    Write-Host ""
    cargo test $TestName -- --nocapture --test-threads=1
}
else {
    Write-Host "🔍 Running all tests with verbose output" -ForegroundColor Cyan
    Write-Host ""
    cargo test -- --nocapture --test-threads=1
}
