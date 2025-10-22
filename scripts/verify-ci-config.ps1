# Verify IDE Configuration Matches CI/CD
# This script checks that your local setup matches CI expectations

Write-Host "🔍 Verifying IDE Configuration Matches CI/CD..." -ForegroundColor Cyan
Write-Host ""

$errors = 0
$warnings = 0

# Check rust-toolchain.toml exists
Write-Host "Checking rust-toolchain.toml..." -NoNewline
if (Test-Path "rust-toolchain.toml") {
    Write-Host " ✓" -ForegroundColor Green
}
else {
    Write-Host " ✗ Missing" -ForegroundColor Red
    $errors++
}

# Check Rust version
Write-Host "Checking Rust version..." -NoNewline
try {
    $rustVersion = rustc --version
    if ($rustVersion -match "stable") {
        Write-Host " ✓ $rustVersion" -ForegroundColor Green
    }
    else {
        Write-Host " ⚠ Using $rustVersion (CI uses stable)" -ForegroundColor Yellow
        $warnings++
    }
}
catch {
    Write-Host " ✗ Rust not found" -ForegroundColor Red
    $errors++
}

# Check rustfmt is installed
Write-Host "Checking rustfmt..." -NoNewline
try {
    $null = rustfmt --version
    Write-Host " ✓" -ForegroundColor Green
}
catch {
    Write-Host " ✗ Not installed" -ForegroundColor Red
    $errors++
}

# Check clippy is installed
Write-Host "Checking clippy..." -NoNewline
try {
    $null = cargo clippy --version
    Write-Host " ✓" -ForegroundColor Green
}
catch {
    Write-Host " ✗ Not installed" -ForegroundColor Red
    $errors++
}

# Check cargo-audit
Write-Host "Checking cargo-audit..." -NoNewline
try {
    $null = cargo audit --version
    Write-Host " ✓" -ForegroundColor Green
}
catch {
    Write-Host " ⚠ Not installed (optional but recommended)" -ForegroundColor Yellow
    Write-Host "  Install: cargo install cargo-audit" -ForegroundColor Gray
    $warnings++
}

# Check cargo-deny
Write-Host "Checking cargo-deny..." -NoNewline
try {
    $null = cargo deny --version
    Write-Host " ✓" -ForegroundColor Green
}
catch {
    Write-Host " ⚠ Not installed (optional but recommended)" -ForegroundColor Yellow
    Write-Host "  Install: cargo install cargo-deny" -ForegroundColor Gray
    $warnings++
}

# Check configuration files
Write-Host ""
Write-Host "Checking configuration files..."
$configFiles = @(
    "rustfmt.toml",
    "clippy.toml",
    "deny.toml",
    ".cargo/config.toml",
    ".editorconfig",
    ".vscode/settings.json",
    ".vscode/tasks.json",
    ".vscode/extensions.json"
)

foreach ($file in $configFiles) {
    Write-Host "  $file..." -NoNewline
    if (Test-Path $file) {
        Write-Host " ✓" -ForegroundColor Green
    }
    else {
        Write-Host " ✗ Missing" -ForegroundColor Red
        $errors++
    }
}

# Run a quick format check
Write-Host ""
Write-Host "Running format check (as CI does)..." -NoNewline
$formatOutput = cargo fmt --all -- --check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host " ✓" -ForegroundColor Green
}
else {
    Write-Host " ✗ Code needs formatting" -ForegroundColor Red
    Write-Host $formatOutput -ForegroundColor Gray
    $errors++
}

# Run a quick clippy check
Write-Host "Running clippy check (as CI does)..." -NoNewline
$clippyOutput = cargo clippy --all-targets --all-features -- -D warnings 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host " ✓" -ForegroundColor Green
}
else {
    Write-Host " ✗ Clippy found issues" -ForegroundColor Red
    Write-Host $clippyOutput -ForegroundColor Gray
    $errors++
}

# Summary
Write-Host ""
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
if ($errors -eq 0 -and $warnings -eq 0) {
    Write-Host "✓ All checks passed! IDE configuration matches CI/CD." -ForegroundColor Green
}
elseif ($errors -eq 0) {
    Write-Host "⚠ Configuration OK with $warnings warning(s)" -ForegroundColor Yellow
}
else {
    Write-Host "✗ Found $errors error(s) and $warnings warning(s)" -ForegroundColor Red
    Write-Host ""
    Write-Host "To fix, run:" -ForegroundColor Yellow
    if ($errors -gt 0) {
        Write-Host "  cargo fmt" -ForegroundColor White
        Write-Host "  cargo clippy --fix --allow-dirty" -ForegroundColor White
    }
    if ($warnings -gt 0) {
        Write-Host "  just install-tools  # or check docs/TOOLS.md" -ForegroundColor White
    }
}
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

if ($errors -eq 0) {
    Write-Host "💡 Tip: Run 'just ci' to run all CI checks locally" -ForegroundColor Cyan
}

exit $errors
