#!/bin/bash

# Rust API Startup Script
# This script helps ensure the application runs from the correct directory

echo "🦀 Rust API - Order Management System"
echo "======================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found in current directory"
    echo "Please run this script from the rustapi project root directory"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo/Rust not found"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust installation found"
echo "✅ Project directory verified"
echo ""

# Build the project
echo "🔨 Building project..."
if ! cargo build; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"
echo ""

# Run tests
echo "🧪 Running tests..."
if ! cargo test --quiet; then
    echo "❌ Tests failed"
    exit 1
fi

echo "✅ All tests passed"
echo ""

# Start the server
echo "🚀 Starting server..."
echo "Server will be available at http://localhost:3000"
echo "Press Ctrl+C to stop the server"
echo ""

cargo run
