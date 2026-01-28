#!/bin/bash

# Jakarta EE Backend Startup Script
# This script helps you set up and run the Jakarta EE backend server

set -e

echo "========================================"
echo "  Jakarta EE Backend Server Startup"
echo "========================================"
echo ""

# Check if Java is installed
if ! command -v java &> /dev/null; then
    echo "Error: Java is not installed."
    echo "Please install Java 17 or later."
    echo "  - macOS: brew install openjdk@17"
    echo "  - Ubuntu: sudo apt install openjdk-17-jdk"
    exit 1
fi

# Check Java version
JAVA_VERSION=$(java -version 2>&1 | head -n 1 | cut -d'"' -f2 | cut -d'.' -f1)
if [ "$JAVA_VERSION" -lt 17 ]; then
    echo "Error: Java 17 or later is required. Current version: $JAVA_VERSION"
    exit 1
fi

echo "Java version: $(java -version 2>&1 | head -n 1)"
echo ""

# Check if Maven is installed
if ! command -v mvn &> /dev/null; then
    echo "Error: Maven is not installed."
    echo "Please install Maven."
    echo "  - macOS: brew install maven"
    echo "  - Ubuntu: sudo apt install maven"
    exit 1
fi

echo "Maven version: $(mvn -version 2>&1 | head -n 1)"
echo ""

# Change to script directory
cd "$(dirname "$0")"

# Clean and package
echo "Building the application..."
mvn clean package -DskipTests
echo "Build completed successfully."
echo ""

# Run with Payara Micro
echo "Starting the Jakarta EE backend server..."
echo "Server will be available at http://localhost:8081"
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
echo "  GET  /api/health         - Health check"
echo ""
echo "Press Ctrl+C to stop the server."
echo "========================================"
echo ""

mvn payara-micro:start
