#!/bin/bash
# Run tests with detailed output and coverage (Linux/macOS)
# Usage: ./test-coverage.sh

echo "📊 Generating test coverage report..."
echo ""

if command -v cargo-tarpaulin &> /dev/null; then
    # Create coverage directory
    mkdir -p coverage

    echo "Running tests with coverage..."
    cargo tarpaulin --out Html --output-dir coverage --timeout 120

    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ Coverage report generated!"
        echo "📂 Report location: coverage/index.html"

        # Try to open the report
        if [ -f "coverage/index.html" ]; then
            echo ""
            read -p "Open coverage report in browser? (y/n) " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                if command -v xdg-open &> /dev/null; then
                    xdg-open coverage/index.html
                elif command -v open &> /dev/null; then
                    open coverage/index.html
                fi
            fi
        fi
    else
        echo ""
        echo "❌ Coverage generation failed!"
        exit 1
    fi
else
    echo "❌ cargo-tarpaulin not installed!"
    echo ""
    echo "Install with: cargo install cargo-tarpaulin"
    exit 1
fi
