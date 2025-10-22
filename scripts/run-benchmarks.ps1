#!/usr/bin/env pwsh
# Run performance benchmarks
# Usage: ./run-benchmarks.ps1 [benchmark_name]

param(
    [string]$BenchmarkName = ""
)

Write-Host "🏃 Running performance benchmarks..." -ForegroundColor Cyan
Write-Host ""

$startTime = Get-Date

try {
    if ($BenchmarkName) {
        Write-Host "Running specific benchmark: $BenchmarkName" -ForegroundColor Yellow
        cargo bench --bench $BenchmarkName
    }
    else {
        Write-Host "Running all benchmarks..." -ForegroundColor Yellow
        cargo bench
    }

    if ($LASTEXITCODE -eq 0) {
        $endTime = Get-Date
        $duration = ($endTime - $startTime).TotalSeconds

        Write-Host ""
        Write-Host "✅ Benchmarks complete!" -ForegroundColor Green
        Write-Host "⏱️  Total time: $($duration.ToString('F2'))s" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "📊 Results saved to: target/criterion/" -ForegroundColor Yellow
        Write-Host "💡 Open target/criterion/report/index.html to view detailed results" -ForegroundColor Yellow
    }
    else {
        Write-Host ""
        Write-Host "❌ Benchmarks failed!" -ForegroundColor Red
        exit 1
    }
}
catch {
    Write-Host ""
    Write-Host "❌ Error running benchmarks: $_" -ForegroundColor Red
    exit 1
}
