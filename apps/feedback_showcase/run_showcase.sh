#!/bin/bash

# Feedback System Showcase Runner

echo "=== Feedback System Showcase ==="
echo "This script will run the showcase and clean up generated files"
echo

# Run the showcase
echo "Running the showcase..."
cargo run

# Check if the command succeeded
if [ $? -eq 0 ]; then
    echo
    echo "Showcase completed successfully!"
    echo
    
    # List generated files
    echo "Generated files:"
    ls -la *.png *.svg *.html 2>/dev/null || echo "No visualization files found"
    echo
    
    # Offer to clean up
    read -p "Do you want to clean up generated files? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Cleaning up generated files..."
        rm -f rating_distribution.png trend_comparison.svg correlation_matrix.html
        echo "Cleanup complete!"
    else
        echo "Generated files kept."
    fi
else
    echo "Showcase failed to run!"
    exit 1
fi