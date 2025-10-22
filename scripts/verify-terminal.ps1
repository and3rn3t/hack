# Terminal Verification Script for The Hack: Ghost Protocol (PowerShell)
# Usage: pwsh scripts/verify-terminal.ps1

function Write-Header {
    Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║     Terminal Verification for The Hack: Ghost Protocol       ║" -ForegroundColor Cyan
    Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Write-SectionHeader($title) {
    Write-Host "`n$title" -ForegroundColor Cyan
}

function Write-Pass($message) {
    Write-Host "   ✓ $message" -ForegroundColor Green
}

function Write-Warn($message) {
    Write-Host "   ⚠ $message" -ForegroundColor Yellow
}

function Write-Fail($message) {
    Write-Host "   ✗ $message" -ForegroundColor Red
}

# Initialize counters
$script:warnings = 0
$script:errors = 0

Write-Header

# 1. Check PowerShell version
Write-SectionHeader "1. PowerShell Version"
$psVersion = $PSVersionTable.PSVersion
Write-Host "   Version: $psVersion"
if ($psVersion.Major -ge 7) {
    Write-Pass "PowerShell 7+ detected (optimal)"
} elseif ($psVersion.Major -ge 5) {
    Write-Warn "PowerShell 5.x (consider upgrading to PowerShell 7)"
    $script:warnings++
} else {
    Write-Fail "PowerShell version too old"
    $script:errors++
}

# 2. Check terminal size
Write-SectionHeader "2. Terminal Size"
try {
    $size = $Host.UI.RawUI.WindowSize
    $cols = $size.Width
    $rows = $size.Height
    Write-Host "   Current size: $($cols)x$($rows)"
    Write-Host "   Minimum required: 80x24"
    Write-Host "   Recommended: 100x30"
    
    if ($cols -ge 100 -and $rows -ge 30) {
        Write-Pass "Optimal size"
    } elseif ($cols -ge 80 -and $rows -ge 24) {
        Write-Warn "Adequate size (consider expanding)"
        $script:warnings++
    } else {
        Write-Fail "Terminal too small"
        $script:errors++
    }
} catch {
    Write-Fail "Could not determine terminal size"
    $script:errors++
}

# 3. Check encoding
Write-SectionHeader "3. Character Encoding"
$outputEncoding = [Console]::OutputEncoding
$inputEncoding = [Console]::InputEncoding
Write-Host "   Output Encoding: $($outputEncoding.EncodingName)"
Write-Host "   Input Encoding: $($inputEncoding.EncodingName)"

if ($outputEncoding.EncodingName -match "UTF-8" -and $inputEncoding.EncodingName -match "UTF-8") {
    Write-Pass "UTF-8 encoding configured"
} else {
    Write-Fail "UTF-8 encoding not set"
    Write-Host "   Add to PowerShell profile (`$PROFILE):" -ForegroundColor Yellow
    Write-Host "   [Console]::OutputEncoding = [System.Text.Encoding]::UTF8" -ForegroundColor Yellow
    Write-Host "   [Console]::InputEncoding = [System.Text.Encoding]::UTF8" -ForegroundColor Yellow
    $script:errors++
}

# 4. Check Windows Terminal
Write-SectionHeader "4. Terminal Emulator"
$isWindowsTerminal = $env:WT_SESSION
if ($isWindowsTerminal) {
    Write-Pass "Running in Windows Terminal (optimal)"
} else {
    Write-Warn "Not running in Windows Terminal"
    Write-Host "   Windows Terminal provides better color and Unicode support" -ForegroundColor Yellow
    Write-Host "   Install: winget install Microsoft.WindowsTerminal" -ForegroundColor Yellow
    $script:warnings++
}

# 5. Test box-drawing characters
Write-SectionHeader "5. Unicode Box-Drawing Test"
Write-Host "   ╔═══════════════════╗"
Write-Host "   ║  Unicode Test     ║"
Write-Host "   ╠═══════════════════╣"
Write-Host "   ║  ► Special chars  ║"
Write-Host "   ╚═══════════════════╝"
Write-Host ""
Write-Host "   If the above box looks correct:"
Write-Pass "Unicode rendering OK"

# 6. Test ANSI colors
Write-SectionHeader "6. ANSI Color Test"
Write-Host "   " -NoNewline
Write-Host "Red " -ForegroundColor Red -NoNewline
Write-Host "Green " -ForegroundColor Green -NoNewline
Write-Host "Yellow " -ForegroundColor Yellow -NoNewline
Write-Host "Blue " -ForegroundColor Blue -NoNewline
Write-Host "Magenta " -ForegroundColor Magenta -NoNewline
Write-Host "Cyan" -ForegroundColor Cyan
Write-Host "   " -NoNewline
Write-Host "DarkRed " -ForegroundColor DarkRed -NoNewline
Write-Host "DarkGreen " -ForegroundColor DarkGreen -NoNewline
Write-Host "DarkYellow " -ForegroundColor DarkYellow -NoNewline
Write-Host "DarkCyan" -ForegroundColor DarkCyan
Write-Host ""
Write-Host "   If colors display correctly:"
Write-Pass "ANSI color codes working"

# 7. Check for Rust tools
Write-SectionHeader "7. Required Tools"

try {
    $cargoVersion = cargo --version 2>&1
    Write-Pass "cargo found ($cargoVersion)"
} catch {
    Write-Fail "cargo not found"
    Write-Host "   Install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    $script:errors++
}

try {
    $rustcVersion = rustc --version 2>&1
    Write-Pass "rustc found ($rustcVersion)"
} catch {
    Write-Fail "rustc not found"
    $script:errors++
}

# 8. Check environment variables
Write-SectionHeader "8. Environment Variables"
if ($env:TERM) {
    Write-Host "   TERM: $($env:TERM)"
    Write-Pass "TERM variable set"
} else {
    Write-Host "   TERM: not set"
    Write-Warn "TERM variable not set (usually not needed on Windows)"
}

if ($env:COLORTERM) {
    Write-Host "   COLORTERM: $($env:COLORTERM)"
    Write-Pass "COLORTERM set (truecolor support)"
} else {
    Write-Host "   COLORTERM: not set"
}

# 9. Check for common issues
Write-SectionHeader "9. Common Issues Check"

# Check if running in VSCode terminal
if ($env:TERM_PROGRAM -eq "vscode") {
    Write-Warn "Running in VS Code integrated terminal"
    Write-Host "   For full experience, use standalone terminal" -ForegroundColor Yellow
    $script:warnings++
}

# Check if ConPTY is available (Windows 10 1809+)
$winVersion = [System.Environment]::OSVersion.Version
if ($winVersion.Major -ge 10 -and $winVersion.Build -ge 17763) {
    Write-Pass "Modern Windows version with ConPTY support"
} else {
    Write-Warn "Old Windows version, terminal features may be limited"
    $script:warnings++
}

# Summary
Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║                    Verification Summary                       ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

if ($script:errors -eq 0) {
    if ($script:warnings -eq 0) {
        Write-Host "✓ All checks passed! Terminal is optimally configured." -ForegroundColor Green
        Write-Host ""
        Write-Host "You're ready to play The Hack: Ghost Protocol!" -ForegroundColor Green
        Write-Host "Run: " -NoNewline
        Write-Host "cargo run" -ForegroundColor Cyan
    } else {
        Write-Host "⚠ Terminal is adequate but could be improved." -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Consider the recommendations above for the best experience." -ForegroundColor Yellow
        Write-Host "You can still play: " -NoNewline
        Write-Host "cargo run" -ForegroundColor Cyan
    }
} else {
    Write-Host "✗ Some critical checks failed." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please address the issues above before playing." -ForegroundColor Red
    Write-Host "See docs/TERMINAL_SETUP.md for configuration help." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "For detailed configuration, see: " -NoNewline
Write-Host "docs/TERMINAL_SETUP.md" -ForegroundColor Cyan

# Quick fix suggestions
if ($script:errors -gt 0 -or $script:warnings -gt 0) {
    Write-Host ""
    Write-Host "Quick Fix Suggestions:" -ForegroundColor Yellow
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor DarkGray
    
    if ($outputEncoding.EncodingName -notmatch "UTF-8") {
        Write-Host "→ Set UTF-8 encoding:" -ForegroundColor Yellow
        Write-Host "  [Console]::OutputEncoding = [System.Text.Encoding]::UTF8" -ForegroundColor White
        Write-Host "  [Console]::InputEncoding = [System.Text.Encoding]::UTF8" -ForegroundColor White
    }
    
    if (-not $isWindowsTerminal) {
        Write-Host "→ Install Windows Terminal:" -ForegroundColor Yellow
        Write-Host "  winget install Microsoft.WindowsTerminal" -ForegroundColor White
    }
    
    if ($cols -lt 80 -or $rows -lt 24) {
        Write-Host "→ Resize terminal window to at least 80x24" -ForegroundColor Yellow
    }
}
