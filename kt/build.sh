#!/bin/bash

# Build script for Pragmastat Kotlin implementation

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
    run_command "./gradlew run" "Running demo"
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/kt/build.sh ${HIGHLIGHT}<command>${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}    ${DIM}# Run all tests${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET}   ${DIM}# Build the Kotlin package${RESET}"
    echo -e "  ${HIGHLIGHT}jar${RESET}     ${DIM}# Package JAR file${RESET}"
    echo -e "  ${HIGHLIGHT}demo${RESET}    ${DIM}# Run demo examples${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}   ${DIM}# Clean build artifacts${RESET}"
    echo -e "  ${HIGHLIGHT}deps${RESET}    ${DIM}# Display project dependencies${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}     ${DIM}# Run all tasks (clean, test, build, jar)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}  ${DIM}# Run all tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET} ${DIM}# Build package${RESET}"
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
        run_command "./gradlew test" "Running tests"
        print_status "Test results available at: build/reports/tests/test/index.html"
        ;;
    build)
        run_command "./gradlew build" "Building Kotlin package"
        ;;
    jar)
        run_command "./gradlew jar" "Packaging JAR"
        ;;
    demo)
        run_demo
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
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac