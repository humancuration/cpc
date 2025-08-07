#!/bin/bash

# Script to verify all Shtairir demo applications are working correctly
# This script runs a quick check of all examples to ensure they compile and execute

echo "🔍 Checking Shtairir Demo Applications"
echo "======================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: This script must be run from the shtairir_demos directory"
    exit 1
fi

# Array of example names
examples=("data_processing" "user_profiles" "ml_features")

# Track overall success
overall_success=true

# Check each example
for example in "${examples[@]}"; do
    echo
    echo "🧪 Testing $example..."
    echo "------------------------"
    
    # Check if the example compiles
    echo "📦 Checking compilation..."
    if cargo check -p "shtairir_demos_$example" --quiet; then
        echo "✅ Compilation successful"
    else
        echo "❌ Compilation failed"
        overall_success=false
        continue
    fi
    
    # Run the example in test mode (quick execution)
    echo "🏃 Running quick test..."
    if cargo run -p "shtairir_demos_$example" --quiet -- --test-mode; then
        echo "✅ Execution successful"
    else
        echo "❌ Execution failed"
        overall_success=false
    fi
done

# Run integration tests
echo
echo "🧪 Running integration tests..."
echo "-------------------------------"
if cargo test --quiet; then
    echo "✅ All integration tests passed"
else
    echo "❌ Some integration tests failed"
    overall_success=false
fi

# Final summary
echo
echo "📋 Final Summary"
echo "================"
if [ "$overall_success" = true ]; then
    echo "🎉 All Shtairir demo applications are working correctly!"
    exit 0
else
    echo "⚠️  Some issues were detected. Please check the output above."
    exit 1
fi