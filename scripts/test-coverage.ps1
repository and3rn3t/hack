#!/usr/bin/env pwsh
# Run tests with detailed output and coverage
# Usage: ./test-coverage.ps1

Write-Host "📊 Generating test coverage report..." -ForegroundColor Cyan
Write-Host ""

if (Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue) {
    # Create coverage directory
    New-Item -ItemType Directory -Force -Path "coverage" | Out-Null

    Write-Host "Running tests with coverage..." -ForegroundColor Yellow
    cargo tarpaulin --out Html --output-dir coverage --timeout 120

    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ Coverage report generated!" -ForegroundColor Green
        Write-Host "📂 Report location: coverage/index.html" -ForegroundColor Cyan

        # Try to open the report
        if (Test-Path "coverage/index.html") {
            Write-Host ""
            $open = Read-Host "Open coverage report in browser? (y/n)"
            if ($open -eq 'y') {
                Start-Process "coverage/index.html"
            }
        }
    }
    else {
        Write-Host ""
        Write-Host "❌ Coverage generation failed!" -ForegroundColor Red
        exit 1
    }
}
else {
    Write-Host "❌ cargo-tarpaulin not installed!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Install with: cargo install cargo-tarpaulin" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Note: Windows users may need to use WSL or Docker" -ForegroundColor Yellow
    exit 1
}
