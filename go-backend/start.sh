#!/bin/bash

# Go Backend Startup Script
# This script helps you set up and run the Go backend server

set -e

echo "========================================"
echo "  Go Backend Server Startup Script"
echo "========================================"
echo ""

# Check if Go is installed
if ! command -v go &> /dev/null; then
    echo "Error: Go is not installed."
    echo "Please install Go from https://golang.org/dl/"
    exit 1
fi

echo "Go version: $(go version)"
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

# Download dependencies
echo "Downloading dependencies..."
go mod download
go mod tidy
echo "Dependencies downloaded successfully."
echo ""

# Build the application
echo "Building the application..."
go build -o bin/server ./cmd/server
echo "Build completed successfully."
echo ""

# Run the server
echo "Starting the Go backend server..."
echo "Server will be available at http://localhost:${PORT:-8080}"
echo ""
echo "API Endpoints:"
echo "  POST /api/auth/register  - Register new user"
echo "  POST /api/auth/login     - Login and get token"
echo "  POST /api/auth/logout    - Logout (requires auth)"
echo "  GET  /api/users/me       - Get current user (requires auth)"
echo "  GET  /api/tasks/         - List tasks (requires auth)"
echo "  POST /api/tasks/         - Create task (requires auth)"
echo "  GET  /api/tasks/:id/     - Get task (requires auth)"
echo "  PATCH /api/tasks/:id/    - Update task (requires auth)"
echo "  DELETE /api/tasks/:id/   - Delete task (requires auth)"
echo "  GET  /health             - Health check"
echo ""
echo "Press Ctrl+C to stop the server."
echo "========================================"
echo ""

./bin/server
