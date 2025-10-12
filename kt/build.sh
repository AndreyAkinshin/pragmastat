#!/bin/bash

# Build script for Pragmastat Kotlin implementation

set -e

cd "$(dirname "$0")" || exit 1

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[$(date +'%H:%M:%S')]${NC} $1"
}

print_error() {
    echo -e "${RED}[$(date +'%H:%M:%S')] ERROR:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[$(date +'%H:%M:%S')] WARNING:${NC} $1"
}

# Function to run a command and check its status
run_command() {
    local cmd="$1"
    local description="$2"

    print_status "Running: $description"
    if eval "$cmd"; then
        print_status "✓ $description completed successfully"
    else
        print_error "✗ $description failed"
        exit 1
    fi
}

# Main script
if [ -z "$1" ]; then
    echo "Usage: $0 {test|build|jar|clean|deps|all}"
    echo ""
    echo "Commands:"
    echo "  test    - Run all tests"
    echo "  build   - Build the Kotlin package"
    echo "  jar     - Package JAR file"
    echo "  clean   - Clean build artifacts"
    echo "  deps    - Display project dependencies"
    echo "  all     - Run all tasks (clean, test, build, jar)"
    exit 1
fi

case "$1" in
    test)
        run_command "./gradlew test" "Running tests"
        print_status "Test results available at: build/reports/tests/test/index.html"
        ;;
    build)
        run_command "./gradlew build" "Building Kotlin package"
        ;;
    jar)
        run_command "./gradlew jar" "Packaging JAR"
        ;;
    clean)
        run_command "./gradlew clean" "Cleaning build artifacts"
        ;;
    deps)
        run_command "./gradlew dependencies" "Displaying dependencies"
        ;;
    all)
        print_status "Running all tasks..."
        run_command "./gradlew clean" "Cleaning build artifacts"
        run_command "./gradlew test" "Running tests"
        run_command "./gradlew build" "Building Kotlin package"
        run_command "./gradlew jar" "Packaging JAR"
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        echo "Usage: $0 {test|build|jar|clean|deps|all}"
        echo ""
        echo "Commands:"
        echo "  test    - Run all tests"
        echo "  build   - Build the Kotlin package"
        echo "  jar     - Package JAR file"
        echo "  clean   - Clean build artifacts"
        echo "  deps    - Display project dependencies"
        echo "  all     - Run all tasks (clean, test, build, jar)"
        exit 1
        ;;
esac