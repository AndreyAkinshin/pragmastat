#!/bin/bash

# Build script for Pragmastat image generation

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
    echo -e "${BOLD}Usage:${RESET} pragmastat/img/build.sh ${HIGHLIGHT}[command]${RESET}"
    echo ""
    echo -e "If no command is specified, defaults to ${HIGHLIGHT}build${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}generate${RESET}   ${DIM}# Generate images using Python${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET}      ${DIM}# Alias for generate (default)${RESET}"
    echo -e "  ${HIGHLIGHT}logo${RESET}       ${DIM}# Generate logo.png using Python${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}      ${DIM}# Remove generated image files (preserves logo.png)${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}        ${DIM}# Run all tasks (generate)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh${RESET}       ${DIM}# Generate images (default)${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET} ${DIM}# Generate images${RESET}"
    echo -e "  ${SUCCESS}build.sh logo${RESET}  ${DIM}# Generate logo${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}   ${DIM}# Run all tasks${RESET}"
}

# Main script
# Default to 'build' if no arguments provided
if [ -z "$1" ]; then
    set -- "build"
fi

case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
    generate|build)
        run_command "python3 generate-images.py" "Generating images"
        ;;
    logo)
        run_command "python3 generate-logo.py" "Generating logo"
        ;;
    clean)
        print_status "Cleaning generated images..."
        find . -maxdepth 1 -type f \( -name "*.png" ! -name "logo.png" -o -name "*.jpg" -o -name "*.svg" \) -delete 2>/dev/null || true
        print_status "✓ Clean complete"
        ;;
    all)
        print_status "Running all tasks..."
        run_command "python3 generate-images.py" "Generating images"
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac