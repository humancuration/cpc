#!/bin/bash

# Development script for CPC Desktop App
# Supports hot reload for both frontend and backend

set -e

echo "Starting CPC Desktop Development Environment..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is not installed. Please install Node.js to continue."
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust to continue."
    exit 1
fi

# Install frontend dependencies if needed
if [ ! -d "frontend/node_modules" ]; then
    echo "Installing frontend dependencies..."
    cd frontend && npm install && cd ..
fi

# Start development server
echo "Starting Tauri development server..."
cd src-tauri && cargo tauri dev