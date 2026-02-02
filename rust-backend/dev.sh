#!/bin/bash

# Rust Backend Development Script
# Runs in development mode with cargo run

set -e

echo "========================================"
echo "  Rust Backend Development Mode"
echo "========================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Change to script directory
cd "$(dirname "$0")"

# Check if .env exists
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        echo "Creating .env file from .env.example..."
        cp .env.example .env
    fi
fi

echo "Starting development server..."
echo ""

cargo run
