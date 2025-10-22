#!/bin/bash
# Build optimized release binaries (Linux/macOS)
# Usage: ./build-release.sh

echo "🏗️  Building release version..."
echo ""

START_TIME=$(date +%s)

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean --release

# Build release
echo "Building optimized binary..."
cargo build --release

if [ $? -eq 0 ]; then
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))

    echo ""
    echo "✅ Build successful!"
    echo "⏱️  Build time: ${DURATION}s"
    echo ""

    # Show binary info
    BINARY="target/release/hack_simulator"
    if [ -f "$BINARY" ]; then
        SIZE=$(du -h "$BINARY" | cut -f1)
        echo "📦 Binary location: $BINARY"
        echo "📊 Binary size: $SIZE"
        echo ""
        echo "Run with: ./$BINARY"
    fi
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
