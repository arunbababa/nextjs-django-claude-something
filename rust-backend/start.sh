#!/bin/bash

# Rust Backend Startup Script
# This script helps you set up and run the Rust backend server

set -e

echo "========================================"
echo "  Rust Backend Server Startup Script"
echo "========================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed."
    echo "Please install Rust from https://rustup.rs/"
    echo ""
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""

# Change to script directory
cd "$(dirname "$0")"

# Check if .env exists, if not copy from example
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        echo "Creating .env file from .env.example..."
        cp .env.example .env
        echo "Please edit .env file with your configuration."
        echo ""
    fi
fi

# Build in release mode
echo "Building the application (release mode)..."
cargo build --release
echo "Build completed successfully."
echo ""

# Run the server
echo "Starting the Rust backend server..."
./target/release/rust-backend
