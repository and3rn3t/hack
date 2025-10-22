#!/usr/bin/env bash
# Run fuzzing tests with cargo-fuzz
# Note: Requires nightly Rust toolchain

set -e

INSTALL=false
TARGET="fuzz_challenge_validators"
SECONDS=60

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --install)
            INSTALL=true
            shift
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        --seconds)
            SECONDS="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [ "$INSTALL" = true ]; then
    echo "📦 Installing cargo-fuzz..."
    cargo install cargo-fuzz
    exit 0
fi

echo "🎲 Running fuzzing tests..."
echo ""

# Check for nightly toolchain
if ! cargo +nightly --version &> /dev/null; then
    echo "❌ Nightly Rust toolchain required!"
    echo "💡 Install with: rustup install nightly"
    exit 1
fi

echo "Target: $TARGET"
echo "Duration: $SECONDS seconds"
echo ""

# Run fuzzer
cargo +nightly fuzz run "$TARGET" -- -max_total_time="$SECONDS"

echo ""
echo "✅ Fuzzing complete!"
echo "📊 Results saved to: fuzz/corpus/$TARGET/"
echo ""
echo "Available targets:"
echo "  - fuzz_challenge_validators"
echo "  - fuzz_state_deserialization"
echo "  - fuzz_state_operations"
