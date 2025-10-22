#!/bin/bash
# Run specific test with verbose output (Linux/macOS)
# Usage: ./test-verbose.sh [test_name]

TEST_NAME="${1:-}"

if [ -n "$TEST_NAME" ]; then
    echo "🔍 Running test: $TEST_NAME"
    echo ""
    cargo test "$TEST_NAME" -- --nocapture --test-threads=1
else
    echo "🔍 Running all tests with verbose output"
    echo ""
    cargo test -- --nocapture --test-threads=1
fi
