#!/usr/bin/env pwsh
# Run mutation testing with cargo-mutants
# This tests the quality of our test suite by introducing mutations

param(
    [switch]$Install,
    [switch]$Quick,
    [string]$File = ""
)

if ($Install) {
    Write-Host "📦 Installing cargo-mutants..." -ForegroundColor Cyan
    cargo install cargo-mutants
    exit 0
}

Write-Host "🧬 Running mutation testing..." -ForegroundColor Cyan
Write-Host ""
Write-Host "⚠️  This may take several minutes to complete" -ForegroundColor Yellow
Write-Host ""

$startTime = Get-Date

try {
    $cargoArgs = @("mutants")

    if ($Quick) {
        Write-Host "Running in quick mode (first 10 mutants only)..." -ForegroundColor Yellow
        $cargoArgs += "--timeout-multiplier", "2"
        $cargoArgs += "--level", "1"
    }

    if ($File) {
        Write-Host "Testing mutations in: $File" -ForegroundColor Yellow
        $cargoArgs += "--file", $File
    }

    # Run cargo mutants
    & cargo @cargoArgs

    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalSeconds

    Write-Host ""
    Write-Host "✅ Mutation testing complete!" -ForegroundColor Green
    Write-Host "⏱️  Total time: $($duration.ToString('F2'))s" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "📊 Results saved to: mutants.out/" -ForegroundColor Yellow
    Write-Host "💡 Review mutants.out/outcomes.txt for detailed results" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "📈 Mutation Score = (Caught + Timeout) / (Total - Unviable)" -ForegroundColor Cyan

}
catch {
    Write-Host ""
    Write-Host "❌ Mutation testing failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "💡 Install cargo-mutants with: pwsh scripts/run-mutation-tests.ps1 -Install" -ForegroundColor Yellow
    exit 1
}
