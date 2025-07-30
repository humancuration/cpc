#!/bin/bash

# Test script for Finance-Sheets web application

set -e

echo "Running tests for Finance-Sheets web application..."

# Run unit tests
wasm-pack test --headless --firefox

echo "Tests completed successfully!"