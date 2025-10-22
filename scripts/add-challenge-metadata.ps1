# Script to add category and related_concepts to all challenges missing them

$filePath = "src\challenges.rs"
$content = Get-Content $filePath -Raw

# Define category mappings based on challenge IDs/titles
$categories = @{
    "caesar"    = @("ChallengeCategory::Cryptography", @("Caesar cipher", "Substitution cipher"))
    "sql"       = @("ChallengeCategory::WebSecurity", @("SQL injection", "Database security"))
    "hex"       = @("ChallengeCategory::Encoding", @("Hexadecimal encoding", "Number systems"))
    "http"      = @("ChallengeCategory::WebSecurity", @("HTTP headers", "Web protocols"))
    "deep_link" = @("ChallengeCategory::MobileSecurity", @("Deep linking", "Mobile app security"))
    "buffer"    = @("ChallengeCategory::BinaryExploitation", @("Buffer overflow", "Memory corruption"))
    "xor"       = @("ChallengeCategory::ReverseEngineering", @("XOR cipher", "Bitwise operations"))
    "default"   = @("ChallengeCategory::Encoding", @("General concepts", "CTF fundamentals"))
}

# Find challenges missing parameters and add them
# Pattern: find "),\s*\)," which indicates end of hints vec followed by end of Challenge::with_basic_feedback
$pattern = '(\s+\],\s*)\),\s*(Challenge::with_basic_feedback\(|$)'

# Replacement with category and concepts
$replacement = '$1            ChallengeCategory::Encoding,
            vec!["Encoding".to_string(), "CTF basics".to_string()],
        ),
        $2'

# Note: This is a simplified approach - manual categorization is better for accuracy
Write-Host "This script provides a template. Manual categorization recommended for accuracy." -ForegroundColor Yellow
Write-Host "Would add category and concepts to challenges missing them." -ForegroundColor Cyan
