#!/usr/bin/env bash
# Run performance benchmarks
# Usage: ./run-benchmarks.sh [benchmark_name]

set -e

BENCHMARK_NAME="${1:-}"

echo "🏃 Running performance benchmarks..."
echo ""

START_TIME=$(date +%s)

if [ -n "$BENCHMARK_NAME" ]; then
    echo "Running specific benchmark: $BENCHMARK_NAME"
    cargo bench --bench "$BENCHMARK_NAME"
else
    echo "Running all benchmarks..."
    cargo bench
fi

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo "✅ Benchmarks complete!"
echo "⏱️  Total time: ${DURATION}s"
echo ""
echo "📊 Results saved to: target/criterion/"
echo "💡 Open target/criterion/report/index.html to view detailed results"
