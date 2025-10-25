#!/usr/bin/env pwsh
# Performance analysis script for WASM bundle

param(
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Stop"

Write-Host "🔍 WASM Bundle Performance Analysis" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan

# Check if WASM file exists
$wasmPath = "target/wasm32-unknown-unknown/release/hack_simulator.wasm"
if (-not (Test-Path $wasmPath)) {
    Write-Error "WASM file not found. Please build first with: cargo build --target wasm32-unknown-unknown --release --no-default-features --features web"
    exit 1
}

# Get file size
$wasmFile = Get-Item $wasmPath
$sizeKB = [math]::Round($wasmFile.Length / 1KB, 2)
$sizeMB = [math]::Round($wasmFile.Length / 1MB, 2)

Write-Host "📊 Current Bundle Analysis:" -ForegroundColor Green
Write-Host "  File: $($wasmFile.Name)"
Write-Host "  Size: $($wasmFile.Length) bytes ($sizeKB KB / $sizeMB MB)"
Write-Host "  Path: $wasmPath"

# Performance recommendations
Write-Host "`n🚀 Performance Optimizations:" -ForegroundColor Yellow

if ($sizeKB -gt 300) {
    Write-Host "  ⚠️  Bundle size >300KB - consider optimizations" -ForegroundColor Yellow
}
elseif ($sizeKB -gt 200) {
    Write-Host "  ✅ Bundle size reasonable (200-300KB)" -ForegroundColor Green
}
else {
    Write-Host "  ✨ Excellent bundle size (<200KB)" -ForegroundColor Green
}

Write-Host "`n📈 Optimization Strategies:" -ForegroundColor Blue
Write-Host "  1. Use lazy loading for challenge data"
Write-Host "  2. Minimize included features in Cargo.toml"
Write-Host "  3. Remove unused dependencies"
Write-Host "  4. Use smaller data structures"
Write-Host "  5. Enable link-time optimization (LTO)"

# Check for wasm-opt
$wasmOptPath = Get-Command "wasm-opt" -ErrorAction SilentlyContinue
if ($wasmOptPath) {
    Write-Host "`n🔧 Running wasm-opt optimization..." -ForegroundColor Magenta
    $optimizedPath = "$wasmPath.optimized"

    try {
        & wasm-opt -Oz --enable-bulk-memory $wasmPath -o $optimizedPath

        if (Test-Path $optimizedPath) {
            $optimizedFile = Get-Item $optimizedPath
            $optimizedKB = [math]::Round($optimizedFile.Length / 1KB, 2)
            $savings = $sizeKB - $optimizedKB
            $percentage = [math]::Round(($savings / $sizeKB) * 100, 2)

            Write-Host "  Original: $sizeKB KB" -ForegroundColor White
            Write-Host "  Optimized: $optimizedKB KB" -ForegroundColor Green
            Write-Host "  Savings: $savings KB ($percentage%)" -ForegroundColor Green

            # Replace original with optimized
            Move-Item $optimizedPath $wasmPath -Force
            Write-Host "  ✅ Optimized WASM file saved" -ForegroundColor Green
        }
    }
    catch {
        Write-Warning "wasm-opt optimization failed: $($_.Exception.Message)"
    }
}
else {
    Write-Host "`n💡 Install wasm-opt for further optimization:" -ForegroundColor Yellow
    Write-Host "  npm install -g wasm-opt"
    Write-Host "  # or via binaryen package"
}

# Show build profile recommendations
Write-Host "`n⚙️  Build Profile Optimizations:" -ForegroundColor Magenta
Write-Host "  Current: --release (good)"
Write-Host "  Consider: Add LTO to Cargo.toml:"
Write-Host "    [profile.release]"
Write-Host "    lto = true"
Write-Host "    panic = 'abort'"
Write-Host "    opt-level = 'z'  # Optimize for size"

# Memory usage analysis
Write-Host "`n🧠 Memory Optimization Tips:" -ForegroundColor Blue
Write-Host "  1. Use Box<T> for large structs"
Write-Host "  2. Implement lazy loading for challenge data"
Write-Host "  3. Use Vec<T> capacity reservations"
Write-Host "  4. Consider using smaller integer types (u16 vs u32)"

if ($Verbose) {
    Write-Host "`n🔍 Verbose Analysis:" -ForegroundColor DarkGray
    Write-Host "  Build target: wasm32-unknown-unknown"
    Write-Host "  Features: web (no native deps)"
    Write-Host "  Profile: release"
    Write-Host "  Generated: $(Get-Date)"
}

Write-Host "`n✅ Analysis Complete!" -ForegroundColor Green
