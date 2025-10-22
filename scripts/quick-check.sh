#!/bin/bash
# Quick development check - runs fmt, clippy, and tests (Linux/macOS)
# Usage: ./quick-check.sh

echo "🚀 Running quick development check..."
echo ""

START_TIME=$(date +%s)

# Step 1: Format code
echo "1️⃣  Formatting code..."
cargo fmt
if [ $? -ne 0 ]; then
    echo "❌ Formatting failed!"
    exit 1
fi
echo "✅ Code formatted"
echo ""

# Step 2: Run clippy
echo "2️⃣  Running clippy..."
cargo clippy --quiet -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy found issues!"
    exit 1
fi
echo "✅ Clippy passed"
echo ""

# Step 3: Run tests
echo "3️⃣  Running tests..."
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "❌ Tests failed!"
    exit 1
fi
echo "✅ All tests passed"
echo ""

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "🎉 All checks passed!"
echo "⏱️  Total time: ${DURATION}s"
