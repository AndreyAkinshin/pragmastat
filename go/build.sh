#!/bin/bash

# Build script for Pragmastat Go implementation

set -e

cd "$(dirname "$0")" || exit 1

# Colors for output (purpose-oriented names)
ERROR='\033[0;31m'
SUCCESS='\033[0;32m'
HIGHLIGHT='\033[1;33m'
HEADER='\033[0;36m'
UNUSED='\033[0;34m'
ARG='\033[0;35m'
BOLD='\033[1m'
DIM='\033[2m'
RESET='\033[0m'

# Function to print colored output
print_error() {
    echo -e "${ERROR}ERROR:${RESET} $1" >&2
}

print_info() {
    echo -e "${SUCCESS}INFO:${RESET} $1"
}

print_warning() {
    echo -e "${HIGHLIGHT}WARNING:${RESET} $1"
}

print_status() {
    echo -e "${SUCCESS}[$(date +'%H:%M:%S')]${RESET} $1"
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

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/go/build.sh ${HIGHLIGHT}<command>${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}              ${DIM}# Run all tests${RESET}"
    echo -e "  ${HIGHLIGHT}test-verbose${RESET}      ${DIM}# Run all tests with verbose output${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET}             ${DIM}# Build the Go package${RESET}"
    echo -e "  ${HIGHLIGHT}lint${RESET}              ${DIM}# Run golangci-lint (if installed)${RESET}"
    echo -e "  ${HIGHLIGHT}format${RESET}            ${DIM}# Format code with go fmt${RESET}"
    echo -e "  ${HIGHLIGHT}coverage${RESET}          ${DIM}# Run tests with coverage summary${RESET}"
    echo -e "  ${HIGHLIGHT}coverage-detailed${RESET} ${DIM}# Generate detailed coverage report${RESET}"
    echo -e "  ${HIGHLIGHT}bench${RESET}             ${DIM}# Run benchmarks${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}             ${DIM}# Clean build cache and coverage files${RESET}"
    echo -e "  ${HIGHLIGHT}deps${RESET}              ${DIM}# Download and verify dependencies${RESET}"
    echo -e "  ${HIGHLIGHT}tidy${RESET}              ${DIM}# Tidy module dependencies${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}               ${DIM}# Run all tasks (download, tidy, format, lint, test, build)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}   ${DIM}# Run all tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET}  ${DIM}# Build the package${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}    ${DIM}# Run all tasks${RESET}"
}

# Main script
if [ -z "$1" ]; then
    show_help
    exit 1
fi

case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
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
        run_command "go mod tidy" "Tidying module dependencies"
        run_command "go fmt ./..." "Formatting code"
        if command -v golangci-lint &> /dev/null; then
            run_command "golangci-lint run" "Running linter"
        fi
        run_command "go test ./..." "Running tests"
        run_command "go build ./..." "Building Go package"
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac