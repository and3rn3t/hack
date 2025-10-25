#!/bin/bash
# Web Testing Script for The Hack: Ghost Protocol

echo "🧪 Web Testing Suite for The Hack: Ghost Protocol"
echo "=========================================================="

# Navigate to web directory
cd web || { echo "❌ Failed to navigate to web directory"; exit 1; }

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is required for web testing"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Install dependencies if package.json exists
if [ -f "package.json" ]; then
    echo "📦 Installing test dependencies..."
    npm install
    if [ $? -ne 0 ]; then
        echo "❌ Failed to install dependencies"
        exit 1
    fi
else
    echo "❌ package.json not found in web directory"
    exit 1
fi

echo ""
echo "🧪 Running Web Tests..."

# Get test type parameter
TEST_TYPE=${1:-"all"}

case $TEST_TYPE in
    "unit")
        echo "🔬 Running Unit Tests..."
        npm test
        ;;
    "e2e")
        echo "🌐 Running End-to-End Tests..."
        npx playwright install --with-deps chromium
        npm run test:e2e
        ;;
    "wasm")
        echo "🦀 Running WebAssembly Tests..."
        cd ..
        wasm-pack test --chrome --headless --features web
        cd web
        ;;
    "coverage")
        echo "📊 Running Tests with Coverage..."
        npm run test:coverage
        ;;
    "all")
        echo "🔬 Running Unit Tests..."
        npm test
        
        if [ $? -eq 0 ]; then
            echo ""
            echo "🌐 Running End-to-End Tests..."
            npx playwright install --with-deps chromium
            npm run test:e2e
        fi
        
        if [ $? -eq 0 ]; then
            echo ""
            echo "🦀 Running WebAssembly Tests..."
            cd ..
            wasm-pack test --chrome --headless --features web
            cd web
        fi
        ;;
    *)
        echo "❌ Unknown test type: $TEST_TYPE"
        echo "Available types: unit, e2e, wasm, coverage, all"
        exit 1
        ;;
esac

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ All tests completed successfully!"
else
    echo ""
    echo "❌ Some tests failed!"
    exit 1
fi

# Return to original directory
cd ..