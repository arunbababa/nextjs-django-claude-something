#!/bin/bash

# Jakarta EE Backend Development Script
# Builds and runs with hot-reload using Payara Micro

set -e

echo "========================================"
echo "  Jakarta EE Backend Development Mode"
echo "========================================"
echo ""

# Check if Java is installed
if ! command -v java &> /dev/null; then
    echo "Error: Java is not installed."
    echo "Please install Java 17 or later."
    exit 1
fi

# Check if Maven is installed
if ! command -v mvn &> /dev/null; then
    echo "Error: Maven is not installed."
    exit 1
fi

# Change to script directory
cd "$(dirname "$0")"

echo "Starting development server..."
echo "Server will be available at http://localhost:8081"
echo ""

# Build and run
mvn clean package payara-micro:start -DskipTests
