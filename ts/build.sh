#!/bin/bash

# Build script for Pragmastat TypeScript implementation

set -e

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
case "$1" in
    test)
        run_command "npm test" "Running tests"
        ;;
    build)
        run_command "npm run build" "Building TypeScript"
        ;;
    check)
        run_command "npm run lint" "Running ESLint"
        run_command "npm run format:check" "Checking formatting"
        ;;
    clean)
        run_command "npm run clean" "Cleaning build artifacts"
        ;;
    format)
        run_command "npm run format" "Formatting code"
        ;;
    install)
        run_command "npm install" "Installing dependencies"
        ;;
    coverage)
        run_command "npm run test:coverage" "Running tests with coverage"
        ;;
    watch)
        run_command "npm run test:watch" "Running tests in watch mode"
        ;;
    all)
        print_status "Running all tasks..."
        run_command "npm install" "Installing dependencies"
        run_command "npm run format" "Formatting code"
        run_command "npm run lint" "Running ESLint"
        run_command "npm test" "Running tests"
        run_command "npm run build" "Building TypeScript"
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        echo "Usage: $0 {test|build|check|clean|format|install|coverage|watch|all}"
        echo ""
        echo "Commands:"
        echo "  test      - Run all tests"
        echo "  build     - Build TypeScript to JavaScript"
        echo "  check     - Run linting and format checking"
        echo "  clean     - Clean build artifacts"
        echo "  format    - Format code with Prettier"
        echo "  install   - Install npm dependencies"
        echo "  coverage  - Run tests with coverage report"
        echo "  watch     - Run tests in watch mode"
        echo "  all       - Run all tasks (install, format, lint, test, build)"
        exit 1
        ;;
esac