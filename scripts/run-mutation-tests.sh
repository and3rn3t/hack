#!/usr/bin/env bash
# Run mutation testing with cargo-mutants
# This tests the quality of our test suite by introducing mutations

set -e

INSTALL=false
QUICK=false
FILE=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --install)
            INSTALL=true
            shift
            ;;
        --quick)
            QUICK=true
            shift
            ;;
        --file)
            FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [ "$INSTALL" = true ]; then
    echo "📦 Installing cargo-mutants..."
    cargo install cargo-mutants
    exit 0
fi

echo "🧬 Running mutation testing..."
echo ""
echo "⚠️  This may take several minutes to complete"
echo ""

START_TIME=$(date +%s)

ARGS=("mutants")

if [ "$QUICK" = true ]; then
    echo "Running in quick mode (limited mutants)..."
    ARGS+=("--timeout-multiplier" "2")
    ARGS+=("--level" "1")
fi

if [ -n "$FILE" ]; then
    echo "Testing mutations in: $FILE"
    ARGS+=("--file" "$FILE")
fi

# Check if cargo-mutants is installed
if ! command -v cargo-mutants &> /dev/null; then
    echo "❌ cargo-mutants not found!"
    echo "💡 Install it with: ./scripts/run-mutation-tests.sh --install"
    exit 1
fi

# Run cargo mutants
cargo "${ARGS[@]}"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo "✅ Mutation testing complete!"
echo "⏱️  Total time: ${DURATION}s"
echo ""
echo "📊 Results saved to: mutants.out/"
echo "💡 Review mutants.out/outcomes.txt for detailed results"
echo ""
echo "📈 Mutation Score = (Caught + Timeout) / (Total - Unviable)"
