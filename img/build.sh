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

# Function to ensure virtual environment is set up (for local runs)
ensure_venv() {
    local venv_dir="venv"
    
    # Skip if in Docker or already in a venv
    if [ -f "/.dockerenv" ] || [ -n "$VIRTUAL_ENV" ]; then
        return 0
    fi
    
    if [ ! -d "$venv_dir" ]; then
        print_status "Creating virtual environment..."
        python3 -m venv "$venv_dir"
    fi
    
    # Check if dependencies are installed by trying to import them
    if ! "$venv_dir/bin/python3" -c "import numpy, matplotlib, scipy" &>/dev/null; then
        print_status "Installing Python dependencies from requirements.txt..."
        "$venv_dir/bin/pip" install -q --upgrade pip
        "$venv_dir/bin/pip" install -q -r requirements.txt
    fi
}

# Function to run Python command with venv (or directly in Docker)
run_python() {
    ensure_venv
    
    # Use python3 directly in Docker, otherwise use venv
    if [ -f "/.dockerenv" ] || [ -n "$VIRTUAL_ENV" ]; then
        python3 "$@"
    else
        ./venv/bin/python3 "$@"
    fi
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
        print_status "Running: Generating images"
        if run_python generate-images.py; then
            print_status "✓ Generating images completed successfully"
        else
            print_error "✗ Generating images failed"
            exit 1
        fi
        ;;
    logo)
        print_status "Running: Generating logo"
        if run_python generate-logo.py; then
            print_status "✓ Generating logo completed successfully"
        else
            print_error "✗ Generating logo failed"
            exit 1
        fi
        ;;
    clean)
        print_status "Cleaning generated images..."
        find . -maxdepth 1 -type f \( -name "*.png" ! -name "logo.png" -o -name "*.jpg" -o -name "*.svg" \) -delete 2>/dev/null || true
        # Also clean virtual environment
        if [ -d "venv" ]; then
            print_status "Removing virtual environment..."
            rm -rf venv
        fi
        print_status "✓ Clean complete"
        ;;
    all)
        print_status "Running all tasks..."
        print_status "Running: Generating images"
        if run_python generate-images.py; then
            print_status "✓ Generating images completed successfully"
        else
            print_error "✗ Generating images failed"
            exit 1
        fi
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac