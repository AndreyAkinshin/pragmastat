#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to run tests
run_tests() {
    print_info "Running tests..."
    python -m pytest tests/ -v
}

# Function to build package
build_package() {
    print_info "Building package..."
    
    # Clean previous builds
    rm -rf dist/ build/ *.egg-info
    
    # Build the package
    python -m build
    
    print_info "Build complete. Files in dist/:"
    ls -la dist/
}

# Function to check package
check_package() {
    if [ ! -d "dist" ]; then
        print_error "No dist directory found. Run './build.sh build' first."
        exit 1
    fi
    
    print_info "Checking package with twine..."
    twine check dist/*
}

# Function to clean build artifacts
clean() {
    print_info "Cleaning build artifacts..."
    rm -rf build/ dist/ *.egg-info __pycache__/ .pytest_cache/
    find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
    find . -type f -name "*.pyc" -delete 2>/dev/null || true
    find . -type f -name "*.pyo" -delete 2>/dev/null || true
    print_info "Clean complete"
}

# Function to run development install
dev_install() {
    print_info "Installing package in development mode..."
    pip install -e .
    print_info "Development installation complete"
}

# Function to show help
show_help() {
    echo "Usage: ./build.sh [command]"
    echo ""
    echo "Commands:"
    echo "  help       Show this help message"
    echo "  dev        Install package in development mode"
    echo "  test       Run tests"
    echo "  build      Build distribution packages"
    echo "  check      Check package with twine"
    echo "  clean      Clean build artifacts"
    echo "  all        Run test, build, and check"
    echo ""
    echo "Examples:"
    echo "  ./build.sh test"
    echo "  ./build.sh build"
    echo "  ./build.sh all"
}

# Main script logic
case "${1:-help}" in
    help)
        show_help
        ;;
    dev)
        dev_install
        ;;
    test)
        run_tests
        ;;
    build)
        build_package
        ;;
    check)
        check_package
        ;;
    clean)
        clean
        ;;
    all)
        run_tests
        build_package
        check_package
        print_info "All tasks completed successfully!"
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac