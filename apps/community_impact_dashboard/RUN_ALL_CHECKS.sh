#!/bin/bash

# Make this script executable
chmod +x "$0"

# Unified Community Impact Dashboard - Complete Verification Script
# This script runs all verification and testing procedures

echo "========================================="
echo "Unified Community Impact Dashboard"
echo "Complete Verification and Testing Script"
echo "========================================="
echo

# Make sure we're in the right directory
cd "$(dirname "$0")"

# Run deployment verification
echo "1. Running deployment verification..."
if [ -f "VERIFY_DEPLOYMENT.sh" ]; then
    bash VERIFY_DEPLOYMENT.sh
    if [ $? -ne 0 ]; then
        echo "   ❌ Deployment verification failed"
        exit 1
    else
        echo "   ✅ Deployment verification passed"
    fi
else
    echo "   ❌ Verification script not found"
    exit 1
fi

echo

# Run unit tests
echo "2. Running unit tests..."
if wasm-pack test --firefox --headless; then
    echo "   ✅ Unit tests passed"
else
    echo "   ❌ Unit tests failed"
    exit 1
fi

echo

# Run integration tests
echo "3. Running integration tests..."

INTEGRATION_TESTS=(
    "connectivity_tests.rs"
    "data_flow_tests.rs"
    "visualization_tests.rs"
    "complexity_tests.rs"
    "story_contribution_tests.rs"
    "community_validation_tests.rs"
)

for test in "${INTEGRATION_TESTS[@]}"; do
    test_name=$(basename "$test" .rs)
    echo "   Running $test_name..."
    if wasm-pack test --firefox --headless -- "tests/integration/$test"; then
        echo "     ✅ $test_name passed"
    else
        echo "     ❌ $test_name failed"
        exit 1
    fi
done

echo

# Summary
echo "========================================="
echo "ALL CHECKS PASSED SUCCESSFULLY"
echo "========================================="
echo
echo "✅ Deployment verification: PASSED"
echo "✅ Unit tests: PASSED"
echo "✅ Integration tests: PASSED"
echo
echo "The Unified Community Impact Dashboard is fully functional"
echo "and ready for development or production use."
echo
echo "Next steps:"
echo "- Run 'trunk serve' to start development server"
echo "- Run 'bash BUILD_AND_DEPLOY.sh' to build for production"
echo

exit 0