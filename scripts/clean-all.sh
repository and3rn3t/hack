#!/bin/bash
# Clean all build artifacts and start fresh (Linux/macOS)
# Usage: ./clean-all.sh

echo "🧹 Cleaning all build artifacts..."
echo ""

# Clean Cargo artifacts
echo "Cleaning Cargo build files..."
cargo clean

# Clean coverage reports
if [ -d "coverage" ]; then
    echo "Removing coverage reports..."
    rm -rf coverage
fi

# Clean save files (optional - ask user)
if [ -f "game_save.json" ]; then
    read -p "Remove game save file? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm game_save.json
        echo "Removed game_save.json"
    fi
fi

# Clean test artifacts
TEST_FILES=$(ls test_*.json 2> /dev/null)
if [ -n "$TEST_FILES" ]; then
    echo "Removing test artifacts..."
    rm -f test_*.json
fi

echo ""
echo "✅ Cleanup complete!"
