#!/bin/bash

# Go Backend Development Script
# Runs the server directly without building (faster for development)

set -e

echo "========================================"
echo "  Go Backend Development Mode"
echo "========================================"
echo ""

# Check if Go is installed
if ! command -v go &> /dev/null; then
    echo "Error: Go is not installed."
    echo "Please install Go from https://golang.org/dl/"
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

# Download dependencies if needed
if [ ! -d "vendor" ] && [ ! -f "go.sum" ]; then
    echo "Downloading dependencies..."
    go mod download
fi

echo "Starting development server..."
echo "Server will be available at http://localhost:${PORT:-8080}"
echo ""

go run ./cmd/server
