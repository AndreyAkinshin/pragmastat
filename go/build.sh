#!/bin/bash

# Build script for Pragmastat Go implementation

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
        run_command "go test ./..." "Running tests"
        ;;
    test-verbose)
        run_command "go test -v ./..." "Running tests (verbose)"
        ;;
    build)
        run_command "go build ./..." "Building Go package"
        ;;
    lint)
        if command -v golangci-lint &> /dev/null; then
            run_command "golangci-lint run" "Running linter"
        else
            print_warning "golangci-lint not installed, skipping lint"
            print_warning "Install with: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest"
        fi
        ;;
    format)
        run_command "go fmt ./..." "Formatting code"
        ;;
    coverage)
        run_command "go test -cover ./..." "Running tests with coverage"
        ;;
    coverage-detailed)
        run_command "go test -coverprofile=coverage.out ./..." "Generating coverage profile"
        run_command "go tool cover -html=coverage.out -o coverage.html" "Generating coverage HTML"
        print_status "Coverage report generated at coverage.html"
        ;;
    bench)
        run_command "go test -bench=. ./..." "Running benchmarks"
        ;;
    clean)
        run_command "go clean" "Cleaning build cache"
        run_command "rm -f coverage.out coverage.html" "Removing coverage files"
        ;;
    deps)
        run_command "go mod download" "Downloading dependencies"
        run_command "go mod verify" "Verifying dependencies"
        ;;
    tidy)
        run_command "go mod tidy" "Tidying module dependencies"
        ;;
    all)
        print_status "Running all tasks..."
        run_command "go mod download" "Downloading dependencies"
        run_command "go fmt ./..." "Formatting code"
        if command -v golangci-lint &> /dev/null; then
            run_command "golangci-lint run" "Running linter"
        fi
        run_command "go test ./..." "Running tests"
        run_command "go build ./..." "Building Go package"
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        echo "Usage: $0 {test|test-verbose|build|lint|format|coverage|coverage-detailed|bench|clean|deps|tidy|all}"
        echo ""
        echo "Commands:"
        echo "  test              - Run all tests"
        echo "  test-verbose      - Run all tests with verbose output"
        echo "  build             - Build the Go package"
        echo "  lint              - Run golangci-lint (if installed)"
        echo "  format            - Format code with go fmt"
        echo "  coverage          - Run tests with coverage summary"
        echo "  coverage-detailed - Generate detailed coverage report"
        echo "  bench             - Run benchmarks"
        echo "  clean             - Clean build cache and coverage files"
        echo "  deps              - Download and verify dependencies"
        echo "  tidy              - Tidy module dependencies"
        echo "  all               - Run all tasks (download, format, lint, test, build)"
        exit 1
        ;;
esac