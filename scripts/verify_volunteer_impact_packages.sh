#!/bin/bash

# Script to verify that the new volunteer impact tracking packages compile correctly

echo "Verifying volunteer impact tracking packages..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: This script must be run from the project root directory"
    exit 1
fi

# Verify that the packages compile
echo "Checking volunteer_impact_tracker package..."
cargo check -p volunteer_impact_tracker

if [ $? -ne 0 ]; then
    echo "Error: volunteer_impact_tracker failed to compile"
    exit 1
fi

echo "Checking learning_impact_tracker package..."
cargo check -p learning_impact_tracker

if [ $? -ne 0 ]; then
    echo "Error: learning_impact_tracker failed to compile"
    exit 1
fi

echo "Checking impact_viz package..."
cargo check -p impact_viz

if [ $? -ne 0 ]; then
    echo "Error: impact_viz failed to compile"
    exit 1
fi

echo "Checking volunteer_coordination_admin package..."
cargo check -p volunteer-coordination-admin

if [ $? -ne 0 ]; then
    echo "Error: volunteer-coordination-admin failed to compile"
    exit 1
fi

echo "All volunteer impact tracking packages compiled successfully!"
echo "Running unit tests..."

# Run unit tests for the new packages
echo "Running tests for volunteer_impact_tracker..."
cargo test -p volunteer_impact_tracker

if [ $? -ne 0 ]; then
    echo "Error: volunteer_impact_tracker tests failed"
    exit 1
fi

echo "Running tests for learning_impact_tracker..."
cargo test -p learning_impact_tracker

if [ $? -ne 0 ]; then
    echo "Error: learning_impact_tracker tests failed"
    exit 1
fi

echo "Running tests for impact_viz..."
cargo test -p impact_viz

if [ $? -ne 0 ]; then
    echo "Error: impact_viz tests failed"
    exit 1
fi

echo "All tests passed successfully!"
echo "Verification complete."