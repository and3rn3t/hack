#!/usr/bin/env pwsh
# Script to help update challenge definitions to new format

$challengesFile = "src/challenges.rs"
$content = Get-Content $challengesFile -Raw

# Read the file
Write-Host "Analyzing challenges.rs..." -ForegroundColor Cyan

# Count Challenge::new occurrences
$newCount = ([regex]::Matches($content, "Challenge::new\(")).Count
Write-Host "Found $newCount Challenge::new() calls" -ForegroundColor Yellow

Write-Host "`nTo update challenges to new format:" -ForegroundColor Green
Write-Host "1. Replace 'Challenge::new(' with 'Challenge::with_basic_feedback('" -ForegroundColor White
Write-Host "2. After the hints vec, add:" -ForegroundColor White
Write-Host "   ChallengeCategory::Encoding,  // or appropriate category" -ForegroundColor Gray
Write-Host "   vec![""concept1"".to_string(), ""concept2"".to_string()]," -ForegroundColor Gray

Write-Host "`nCategories available:" -ForegroundColor Green
Write-Host "  - Encoding" -ForegroundColor Gray
Write-Host "  - Cryptography" -ForegroundColor Gray
Write-Host "  - WebSecurity" -ForegroundColor Gray
Write-Host "  - NetworkSecurity" -ForegroundColor Gray
Write-Host "  - FileSystem" -ForegroundColor Gray
Write-Host "  - BinaryExploitation" -ForegroundColor Gray
Write-Host "  - ReverseEngineering" -ForegroundColor Gray
Write-Host "  - MobileSecurity" -ForegroundColor Gray
