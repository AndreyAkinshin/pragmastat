#!/bin/bash

# Build script for Pragmastat image generation

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
# Default to 'build' if no arguments provided
if [ -z "$1" ]; then
    set -- "build"
fi

case "$1" in
    -h|--help)
        echo "Usage: $0 {generate|build|logo|clean|all}"
        echo ""
        echo "Commands:"
        echo "  generate   - Generate images using Python"
        echo "  build      - Alias for generate (default if no command specified)"
        echo "  logo       - Generate logo.png using Python"
        echo "  clean      - Remove generated image files (preserves logo.png)"
        echo "  all        - Run all tasks (generate)"
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
        echo "Usage: $0 {generate|build|logo|clean|all}"
        echo ""
        echo "Commands:"
        echo "  generate   - Generate images using Python"
        echo "  build      - Alias for generate (default if no command specified)"
        echo "  logo       - Generate logo.png using Python"
        echo "  clean      - Remove generated image files (preserves logo.png)"
        echo "  all        - Run all tasks (generate)"
        exit 1
        ;;
esac