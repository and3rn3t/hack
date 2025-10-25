#!/usr/bin/env pwsh
# Web Testing Script for The Hack: Ghost Protocol

Write-Host "🧪 Web Testing Suite for The Hack: Ghost Protocol" -ForegroundColor Cyan
Write-Host "=" * 60

$ErrorActionPreference = "Continue"

# Navigate to web directory
Set-Location "web"

# Check if Node.js is installed
if (-not (Get-Command "node" -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Node.js is required for web testing" -ForegroundColor Red
    Write-Host "Please install Node.js from https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# Install dependencies if package.json exists
if (Test-Path "package.json") {
    Write-Host "📦 Installing test dependencies..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install dependencies" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "❌ package.json not found in web directory" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🧪 Running Web Tests..." -ForegroundColor Green

# Run different test suites based on parameter
param(
    [string]$TestType = "all"
)

switch ($TestType.ToLower()) {
    "unit" {
        Write-Host "🔬 Running Unit Tests..." -ForegroundColor Cyan
        npm test
    }
    "e2e" {
        Write-Host "🌐 Running End-to-End Tests..." -ForegroundColor Cyan
        npx playwright install --with-deps chromium
        npm run test:e2e
    }
    "wasm" {
        Write-Host "🦀 Running WebAssembly Tests..." -ForegroundColor Cyan
        Set-Location ".."
        wasm-pack test --chrome --headless --features web
        Set-Location "web"
    }
    "coverage" {
        Write-Host "📊 Running Tests with Coverage..." -ForegroundColor Cyan
        npm run test:coverage
    }
    "all" {
        Write-Host "🔬 Running Unit Tests..." -ForegroundColor Cyan
        npm test
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host ""
            Write-Host "🌐 Running End-to-End Tests..." -ForegroundColor Cyan
            npx playwright install --with-deps chromium
            npm run test:e2e
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host ""
            Write-Host "🦀 Running WebAssembly Tests..." -ForegroundColor Cyan
            Set-Location ".."
            wasm-pack test --chrome --headless --features web
            Set-Location "web"
        }
    }
    default {
        Write-Host "❌ Unknown test type: $TestType" -ForegroundColor Red
        Write-Host "Available types: unit, e2e, wasm, coverage, all" -ForegroundColor Yellow
        exit 1
    }
}

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ All tests completed successfully!" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "❌ Some tests failed!" -ForegroundColor Red
    exit 1
}

# Return to original directory
Set-Location ".."