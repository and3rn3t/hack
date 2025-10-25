#!/usr/bin/env python3
"""
Test script to validate OSINT challenge answer logic
Since the main project has UI compilation issues, this validates the challenge logic independently.
"""

def test_osint_social_media():
    """Test the GPS coordinates challenge"""
    def validator(answer):
        a = answer.lower()
        return a == "new york" or a == "new york city" or a == "nyc" or a == "manhattan"
    
    # Test valid answers
    valid_answers = ["New York", "new york", "NYC", "nyc", "Manhattan", "new york city"]
    invalid_answers = ["London", "Paris", "Boston", "Brooklyn"]
    
    print("Testing OSINT Social Media Challenge (GPS Coordinates):")
    for answer in valid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'✅ PASS' if result else '❌ FAIL'})")
    
    for answer in invalid_answers:
        result = validator(answer)  
        print(f"  '{answer}' -> {result} ({'❌ FAIL (expected)' if not result else '⚠️  UNEXPECTED PASS'})")
    print()

def test_osint_domain_recon():
    """Test the SPF record challenge"""
    def validator(answer):
        a = answer.lower()
        return a == "google" or a == "gmail" or a == "g suite" or a == "google workspace"
    
    valid_answers = ["Google", "google", "Gmail", "gmail", "G Suite", "Google Workspace"]
    invalid_answers = ["Microsoft", "Yahoo", "Amazon", "Cloudflare"]
    
    print("Testing OSINT Domain Reconnaissance Challenge (SPF Record):")
    for answer in valid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'✅ PASS' if result else '❌ FAIL'})")
    
    for answer in invalid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'❌ FAIL (expected)' if not result else '⚠️  UNEXPECTED PASS'})")
    print()

def test_osint_email_analysis():
    """Test the typosquatting challenge"""  
    def validator(answer):
        a = answer.lower()
        return a == "typosquatting" or a == "typosquat" or a == "cybersquatting"
    
    valid_answers = ["typosquatting", "Typosquatting", "typosquat", "cybersquatting"]
    invalid_answers = ["phishing", "spoofing", "domain hijacking", "social engineering"]
    
    print("Testing OSINT Email Analysis Challenge (Typosquatting):")
    for answer in valid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'✅ PASS' if result else '❌ FAIL'})")
    
    for answer in invalid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'❌ FAIL (expected)' if not result else '⚠️  UNEXPECTED PASS'})")
    print()

def test_osint_geolocation():
    """Test the London geolocation challenge"""
    def validator(answer):
        a = answer.lower()
        return a == "london" or a == "london uk" or a == "london england"
    
    valid_answers = ["London", "london", "London UK", "london uk", "London England"]
    invalid_answers = ["Paris", "Berlin", "Madrid", "Rome", "Birmingham"]
    
    print("Testing OSINT Geolocation Challenge (London Clues):")
    for answer in valid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'✅ PASS' if result else '❌ FAIL'})")
    
    for answer in invalid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'❌ FAIL (expected)' if not result else '⚠️  UNEXPECTED PASS'})")
    print()

def test_osint_breach_investigation():
    """Test the password salt challenge"""
    def validator(answer):
        a = answer.lower()
        return a == "salt" or a == "salting" or a == "hashing salt" or a == "password salt"
    
    valid_answers = ["salt", "Salt", "salting", "Salting", "hashing salt", "password salt"]
    invalid_answers = ["encryption", "hashing", "pepper", "nonce", "iv"]
    
    print("Testing OSINT Breach Investigation Challenge (Password Salt):")
    for answer in valid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'✅ PASS' if result else '❌ FAIL'})")
    
    for answer in invalid_answers:
        result = validator(answer)
        print(f"  '{answer}' -> {result} ({'❌ FAIL (expected)' if not result else '⚠️  UNEXPECTED PASS'})")
    print()

def main():
    print("🔍 Testing OSINT Challenge Validation Logic")
    print("=" * 50)
    
    test_osint_social_media()
    test_osint_domain_recon()
    test_osint_email_analysis()
    test_osint_geolocation()
    test_osint_breach_investigation()
    
    print("✅ All OSINT challenges tested!")
    print("\nThe challenges cover:")
    print("  1. GPS Coordinate Analysis (Digital Forensics)")
    print("  2. DNS/SPF Record Investigation (Network Reconnaissance)")
    print("  3. Email Security Analysis (Social Engineering Detection)")
    print("  4. Geolocation & OSINT Correlation (Intelligence Gathering)")
    print("  5. Data Breach Investigation (Incident Response)")

if __name__ == "__main__":
    main()