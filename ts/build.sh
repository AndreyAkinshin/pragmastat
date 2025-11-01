#!/bin/bash

# Build script for Pragmastat TypeScript implementation

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

# Function to run demo
run_demo() {
    run_command "npx ts-node examples/demo.ts" "Running demo"
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/ts/build.sh ${HIGHLIGHT}<command>${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}      ${DIM}# Run all tests${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET}     ${DIM}# Build TypeScript to JavaScript${RESET}"
    echo -e "  ${HIGHLIGHT}demo${RESET}      ${DIM}# Run demo examples${RESET}"
    echo -e "  ${HIGHLIGHT}lint${RESET}      ${DIM}# Run ESLint${RESET}"
    echo -e "  ${HIGHLIGHT}check${RESET}     ${DIM}# Run linting and format checking${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}     ${DIM}# Clean build artifacts${RESET}"
    echo -e "  ${HIGHLIGHT}format${RESET}    ${DIM}# Format code with Prettier${RESET}"
    echo -e "  ${HIGHLIGHT}install${RESET}   ${DIM}# Install npm dependencies${RESET}"
    echo -e "  ${HIGHLIGHT}coverage${RESET}  ${DIM}# Run tests with coverage report${RESET}"
    echo -e "  ${HIGHLIGHT}watch${RESET}     ${DIM}# Run tests in watch mode${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}       ${DIM}# Run all tasks (install, format, lint, test, build)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}  ${DIM}# Run all tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET} ${DIM}# Build TypeScript${RESET}"
    echo -e "  ${SUCCESS}build.sh demo${RESET}  ${DIM}# Run demo examples${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}   ${DIM}# Run all tasks${RESET}"
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
        run_command "npm test" "Running tests"
        ;;
    build)
        run_command "npm run build" "Building TypeScript"
        ;;
    demo)
        run_demo
        ;;
    lint)
        run_command "npm run lint" "Running ESLint"
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
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac