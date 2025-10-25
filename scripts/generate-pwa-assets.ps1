#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Generate branded icons and screenshots for The Hack: Ghost Protocol PWA

.DESCRIPTION
    This script generates horror-themed cybersecurity CTF game icons and captures
    screenshots for a complete PWA branding package.

.PARAMETER IconsOnly
    Generate only the icons, skip screenshot capture

.PARAMETER ScreenshotsOnly
    Capture only screenshots, skip icon generation

.PARAMETER OutputPath
    Custom output path for generated assets (default: web/static)

.EXAMPLE
    .\generate-pwa-assets.ps1
    Generate all icons and capture all screenshots

.EXAMPLE
    .\generate-pwa-assets.ps1 -IconsOnly
    Generate only the branded icons
#>

param(
    [switch]$IconsOnly,
    [switch]$ScreenshotsOnly,
    [string]$OutputPath = "web/static"
)

$ErrorActionPreference = "Stop"

# Colors for output
$Red = "`e[31m"
$Green = "`e[32m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Magenta = "`e[35m"
$Cyan = "`e[36m"
$Reset = "`e[0m"

function Write-Header {
    param([string]$Text)
    Write-Host "${Magenta}$('=' * 60)${Reset}"
    Write-Host "${Magenta} $Text${Reset}"
    Write-Host "${Magenta}$('=' * 60)${Reset}"
}

function Write-Success {
    param([string]$Text)
    Write-Host "${Green}✅ $Text${Reset}"
}

function Write-Info {
    param([string]$Text)
    Write-Host "${Blue}ℹ️  $Text${Reset}"
}

function Write-Warning {
    param([string]$Text)
    Write-Host "${Yellow}⚠️  $Text${Reset}"
}

function Write-Error {
    param([string]$Text)
    Write-Host "${Red}❌ $Text${Reset}"
}

function Test-Prerequisites {
    Write-Info "Checking prerequisites..."

    # Check if we're in the right directory
    if (-not (Test-Path "Cargo.toml")) {
        Write-Error "Not in project root. Run from the hack directory."
        exit 1
    }

    # Check Node.js for icon generation
    try {
        $nodeVersion = node --version 2>$null
        Write-Success "Node.js found: $nodeVersion"
    }
    catch {
        Write-Error "Node.js not found. Required for icon generation."
        exit 1
    }

    # Check if output directories exist
    $iconsDir = Join-Path $OutputPath "icons"
    $screenshotsDir = Join-Path $OutputPath "screenshots"

    if (-not (Test-Path $iconsDir)) {
        New-Item -ItemType Directory -Path $iconsDir -Force | Out-Null
        Write-Info "Created icons directory: $iconsDir"
    }

    if (-not (Test-Path $screenshotsDir)) {
        New-Item -ItemType Directory -Path $screenshotsDir -Force | Out-Null
        Write-Info "Created screenshots directory: $screenshotsDir"
    }
}

function Generate-Icons {
    Write-Header "🎨 GENERATING BRANDED ICONS"

    try {
        # Try Node.js version first
        Write-Info "Attempting Node.js icon generation..."

        # Check if canvas is installed
        $canvasInstalled = $false
        try {
            node -e "require('canvas')" 2>$null
            $canvasInstalled = $true
        }
        catch {
            Write-Warning "Canvas package not found. Installing..."
        }

        if (-not $canvasInstalled) {
            Write-Info "Installing canvas package..."
            npm install canvas --no-save 2>$null
        }

        # Run icon generation
        node "scripts/generate-icons.js"
        Write-Success "Icons generated successfully with Node.js"

    }
    catch {
        Write-Warning "Node.js generation failed. Using browser method..."
        Write-Info "Opening HTML icon generator..."

        $htmlPath = Resolve-Path "scripts/generate-icons.html"
        Start-Process $htmlPath

        Write-Info "Manual steps:"
        Write-Host "  1. Click '${Green}Generate All Icons${Reset}'"
        Write-Host "  2. Click '${Green}📦 Download All Icons${Reset}'"
        Write-Host "  3. Extract downloaded files to: ${Yellow}$iconsDir${Reset}"
        Write-Host "  4. Press any key when complete..."
        Read-Host
    }

    # Verify icons were created
    $iconFiles = Get-ChildItem "$iconsDir/*.png" -ErrorAction SilentlyContinue
    if ($iconFiles.Count -gt 0) {
        Write-Success "$($iconFiles.Count) icon files found in $iconsDir"

        # List generated icons
        Write-Info "Generated icons:"
        $iconFiles | ForEach-Object {
            $sizeKB = [math]::Round($_.Length / 1KB, 1)
            Write-Host "  📱 $($_.Name) - ${sizeKB} KB"
        }
    }
    else {
        Write-Error "No icon files found. Generation may have failed."
    }
}

function Capture-Screenshots {
    Write-Header "📸 CAPTURING GAME SCREENSHOTS"

    Write-Info "Starting game for screenshot capture..."

    # Build the game first
    Write-Info "Building game..."
    cargo build --release

    if ($LASTEXITCODE -ne 0) {
        Write-Error "Game build failed"
        return
    }

    Write-Success "Game built successfully"

    # Start game in background
    Write-Info "Starting game server..."
    $gameProcess = Start-Process "cargo" -ArgumentList "run", "--release" -NoNewWindow -PassThru

    # Wait for game to start
    Start-Sleep 5

    Write-Info "Game should now be running. Manual screenshot steps:"
    Write-Host ""
    Write-Host "📋 ${Yellow}DESKTOP SCREENSHOTS (1280x720):${Reset}"
    Write-Host "  1. Open game in browser"
    Write-Host "  2. Take screenshot of main menu → save as ${Green}desktop-main.png${Reset}"
    Write-Host "  3. Start a challenge → take screenshot → save as ${Green}desktop-challenge.png${Reset}"
    Write-Host ""
    Write-Host "📋 ${Yellow}MOBILE SCREENSHOTS:${Reset}"
    Write-Host "  1. Open browser dev tools (F12)"
    Write-Host "  2. Toggle device toolbar (Ctrl+Shift+M)"
    Write-Host "  3. Select iPhone/Android device"
    Write-Host "  4. Portrait mode (360x640) → screenshot → save as ${Green}mobile-portrait.png${Reset}"
    Write-Host "  5. Landscape mode (640x360) → screenshot → save as ${Green}mobile-landscape.png${Reset}"
    Write-Host ""
    Write-Host "💾 Save all screenshots to: ${Cyan}$(Resolve-Path $OutputPath)/screenshots/${Reset}"
    Write-Host ""
    Read-Host "Press Enter when screenshots are complete..."

    # Stop game process
    if ($gameProcess -and -not $gameProcess.HasExited) {
        Write-Info "Stopping game..."
        Stop-Process -Id $gameProcess.Id -Force -ErrorAction SilentlyContinue
    }

    # Verify screenshots
    $screenshotDir = Join-Path $OutputPath "screenshots"
    $screenshots = Get-ChildItem "$screenshotDir/*.png" -ErrorAction SilentlyContinue

    if ($screenshots.Count -ge 4) {
        Write-Success "Found $($screenshots.Count) screenshot files"
        $screenshots | ForEach-Object {
            $sizeKB = [math]::Round($_.Length / 1KB, 1)
            Write-Host "  📷 $($_.Name) - ${sizeKB} KB"
        }
    }
    else {
        Write-Warning "Expected 4 screenshots, found $($screenshots.Count)"
        Write-Info "Required files: desktop-main.png, desktop-challenge.png, mobile-portrait.png, mobile-landscape.png"
    }
}

function Optimize-Assets {
    Write-Header "⚡ OPTIMIZING ASSETS"

    $iconsDir = Join-Path $OutputPath "icons"
    $screenshotsDir = Join-Path $OutputPath "screenshots"

    # Check file sizes
    $allImages = Get-ChildItem "$iconsDir/*.png", "$screenshotsDir/*.png" -ErrorAction SilentlyContinue
    $totalSize = ($allImages | Measure-Object Length -Sum).Sum

    Write-Info "Asset Summary:"
    Write-Host "  📱 Icons: $(($allImages | Where-Object { $_.Directory.Name -eq 'icons' }).Count) files"
    Write-Host "  📷 Screenshots: $(($allImages | Where-Object { $_.Directory.Name -eq 'screenshots' }).Count) files"
    Write-Host "  💾 Total size: $([math]::Round($totalSize / 1KB, 1)) KB"

    # Check if any files are too large
    $largeFiles = $allImages | Where-Object { $_.Length -gt 100KB }
    if ($largeFiles) {
        Write-Warning "Large files detected (>100KB):"
        $largeFiles | ForEach-Object {
            $sizeKB = [math]::Round($_.Length / 1KB, 1)
            Write-Host "  ⚠️  $($_.Name) - ${sizeKB} KB"
        }
        Write-Info "Consider compressing large files for better PWA performance"
    }
    else {
        Write-Success "All files are optimally sized"
    }

    # Validate manifest references
    $manifestPath = Join-Path $OutputPath "manifest.json"
    if (Test-Path $manifestPath) {
        Write-Info "Validating manifest.json references..."
        $manifest = Get-Content $manifestPath | ConvertFrom-Json

        $missingIcons = @()
        foreach ($icon in $manifest.icons) {
            $iconPath = Join-Path $OutputPath $icon.src
            if (-not (Test-Path $iconPath)) {
                $missingIcons += $icon.src
            }
        }

        if ($missingIcons.Count -eq 0) {
            Write-Success "All manifest icons found"
        }
        else {
            Write-Warning "Missing manifest icons:"
            $missingIcons | ForEach-Object { Write-Host "  ❌ $_" }
        }
    }
}

# Main execution
try {
    Write-Header "🎯 THE HACK: GHOST PROTOCOL - PWA ASSET GENERATOR"

    Test-Prerequisites

    if (-not $ScreenshotsOnly) {
        Generate-Icons
    }

    if (-not $IconsOnly) {
        Capture-Screenshots
    }

    Optimize-Assets

    Write-Header "🎉 PWA ASSET GENERATION COMPLETE"
    Write-Success "Your horror-themed cybersecurity CTF game is ready for deployment!"
    Write-Info "Next steps:"
    Write-Host "  1. Review generated assets in ${Cyan}$OutputPath${Reset}"
    Write-Host "  2. Test PWA manifest validation"
    Write-Host "  3. Deploy to production"
    Write-Host "  4. Test app installation on mobile devices"

}
catch {
    Write-Error "Asset generation failed: $($_.Exception.Message)"
    exit 1
}
