# Script to batch update Challenge::new to Challenge::with_basic_feedback

$challengesFile = "src\challenges.rs"
$content = Get-Content $challengesFile -Raw

# Pattern to match Challenge::new( but not Challenge::with_basic_feedback
$pattern = "Challenge::new\("
$replacement = "Challenge::with_basic_feedback("

# Simple replacement
$updated = $content -replace [regex]::Escape($pattern), $replacement

# Count changes
$changeCount = ([regex]::Matches($content, [regex]::Escape($pattern))).Count

Write-Host "Would replace $changeCount occurrences of 'Challenge::new(' with 'Challenge::with_basic_feedback('" -ForegroundColor Cyan
Write-Host "This script is for reference only - manual updating is safer." -ForegroundColor Yellow

# Don't actually write - this is just for analysis
# Set-Content $challengesFile -Value $updated -NoNewline
