#!/usr/bin/env pwsh
# Apply memory optimizations to WASM build

param(
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Stop"

Write-Host "🔧 Applying Memory Optimizations for WASM" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Backup current Cargo.toml
$backupPath = "Cargo.toml.backup"
if (-not (Test-Path $backupPath)) {
    Copy-Item "Cargo.toml" $backupPath
    Write-Host "✅ Backed up Cargo.toml to $backupPath" -ForegroundColor Green
}

# Add wee_alloc for smaller allocator (web only)
Write-Host "📝 Adding wee_alloc dependency for smaller memory footprint..." -ForegroundColor Yellow

$cargoContent = Get-Content "Cargo.toml" -Raw
$webDeps = @"

# Small allocator for WASM builds
wee_alloc = { version = "0.4", optional = true }
"@

if ($cargoContent -notmatch "wee_alloc") {
    $cargoContent = $cargoContent -replace "(console_error_panic_hook = \{ version = ""0\.1"", optional = true \})", "`$1$webDeps"
    
    # Update web features to include wee_alloc
    $cargoContent = $cargoContent -replace '(web = \[)(.*?)(\])', "`$1`$2, ""wee_alloc""`$3"
    
    Set-Content "Cargo.toml" $cargoContent
    Write-Host "✅ Added wee_alloc dependency" -ForegroundColor Green
} else {
    Write-Host "ℹ️  wee_alloc already present" -ForegroundColor Blue
}

Write-Host "🏗️  Building optimized WASM bundle..." -ForegroundColor Magenta
try {
    $env:RUSTFLAGS = "-C link-arg=-s"  # Strip debug symbols
    & cargo build --target wasm32-unknown-unknown --release --no-default-features --features web
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Build completed successfully" -ForegroundColor Green
        
        # Check new size
        $wasmPath = "target/wasm32-unknown-unknown/release/hack_simulator.wasm"
        if (Test-Path $wasmPath) {
            $wasmFile = Get-Item $wasmPath
            $sizeKB = [math]::Round($wasmFile.Length / 1KB, 2)
            Write-Host "📊 New bundle size: $sizeKB KB" -ForegroundColor Cyan
        }
    } else {
        Write-Error "Build failed with exit code $LASTEXITCODE"
    }
} catch {
    Write-Error "Build failed: $($_.Exception.Message)"
} finally {
    Remove-Item env:RUSTFLAGS -ErrorAction SilentlyContinue
}

# Additional optimization suggestions
Write-Host "`n💡 Additional Optimizations Applied:" -ForegroundColor Yellow
Write-Host "  ✅ LTO enabled (link-time optimization)"
Write-Host "  ✅ opt-level = 'z' (optimize for size)"
Write-Host "  ✅ panic = 'abort' (smaller panic handling)"
Write-Host "  ✅ codegen-units = 1 (better optimization)"
Write-Host "  ✅ Debug symbols stripped"

if ($Verbose) {
    Write-Host "`n🔍 Next Steps for Further Optimization:" -ForegroundColor DarkGray
    Write-Host "  1. Install wasm-opt: npm install -g wasm-opt"
    Write-Host "  2. Run: wasm-opt -Oz hack_simulator.wasm -o hack_simulator.wasm"
    Write-Host "  3. Consider using wasm-pack for production builds"
    Write-Host "  4. Implement lazy loading in JavaScript layer"
}

Write-Host "`n✅ Memory optimization complete!" -ForegroundColor Green