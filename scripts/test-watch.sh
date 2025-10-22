#!/bin/bash
# Test runner with watch mode for continuous testing (Linux/macOS)
# Usage: ./test-watch.sh [filter]

FILTER="${1:-}"

echo "🔍 Starting test watch mode..."
echo ""

if command -v cargo-watch &> /dev/null; then
    if [ -n "$FILTER" ]; then
        echo "Watching tests matching: $FILTER"
        cargo watch -x "test $FILTER"
    else
        echo "Watching all tests..."
        cargo watch -x test
    fi
else
    echo "❌ cargo-watch not installed!"
    echo ""
    echo "Install with: cargo install cargo-watch"
    exit 1
fi
