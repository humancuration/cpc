#!/bin/bash

# Build script for Finance-Sheets web application

set -e

echo "Building Finance-Sheets web application..."

# Create pkg directory if it doesn't exist
mkdir -p pkg

# Build the wasm package
wasm-pack build --target web --out-dir pkg

# Copy index.html to pkg directory
cp index.html pkg/

echo "Build complete! Files are in the pkg/ directory."
echo "To serve the application, run: python -m http.server 8000 --directory pkg"