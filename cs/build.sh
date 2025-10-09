#!/bin/bash

# Build script for Pragmastat .NET implementation

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

# Function to run tests
run_tests() {
    run_command "(cd Pragmastat.UnitTests && dotnet run)" "Running unit tests"
    run_command "(cd Pragmastat.ReferenceTests && dotnet run)" "Running reference tests"
}

# Function to build package
build_package() {
    local release_flag="$1"

    if [ "$release_flag" == "--release" ]; then
        run_command "dotnet build -c Release" "Building package in release mode"
    else
        run_command "dotnet build -c Debug" "Building package in debug mode"
    fi
}

# Function to pack NuGet package
pack_package() {
    local release_flag="$1"
    local config="Debug"
    local output_dir="."

    if [ "$release_flag" == "--release" ]; then
        config="Release"
    fi

    # Use artifacts directory if it doesn't exist, create it
    if [ -n "$CI" ]; then
        output_dir="./artifacts"
        mkdir -p "$output_dir"
    fi

    run_command "dotnet pack ./Pragmastat/Pragmastat.csproj --configuration $config --include-symbols --include-source -p:SymbolPackageFormat=snupkg --output $output_dir" "Packing NuGet package ($config)"
}

# Function to restore dependencies
restore_deps() {
    run_command "dotnet restore" "Restoring dependencies"
}

# Function to clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    dotnet clean
    rm -rf bin/ obj/ *.nupkg
    print_status "✓ Clean complete"
}

# Function to format code
format_code() {
    run_command "dotnet format" "Formatting code"
}

# Function to lint code
lint_code() {
    run_command "dotnet format --verify-no-changes" "Verifying code formatting"
}

# Main script
if [ -z "$1" ]; then
    echo "Usage: $0 {test|build|pack|restore|clean|format|lint|all} [--release]"
    echo ""
    echo "Commands:"
    echo "  test              - Run all tests"
    echo "  build [--release] - Build package (debug by default, release with --release flag)"
    echo "  pack [--release]  - Pack NuGet package (debug by default, release with --release flag)"
    echo "  restore           - Restore dependencies"
    echo "  clean             - Clean build artifacts"
    echo "  format            - Format code with dotnet format"
    echo "  lint              - Verify code formatting"
    echo "  all               - Run all tasks (restore, format, test, build debug)"
    exit 1
fi

case "$1" in
    test)
        run_tests
        ;;
    build)
        build_package "$2"
        ;;
    pack)
        pack_package "$2"
        ;;
    restore)
        restore_deps
        ;;
    clean)
        clean
        ;;
    format)
        format_code
        ;;
    lint)
        lint_code
        ;;
    all)
        print_status "Running all tasks..."
        restore_deps
        format_code
        run_tests
        build_package ""
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        echo "Usage: $0 {test|build|pack|restore|clean|format|lint|all} [--release]"
        echo ""
        echo "Commands:"
        echo "  test              - Run all tests"
        echo "  build [--release] - Build package (debug by default, release with --release flag)"
        echo "  pack [--release]  - Pack NuGet package (debug by default, release with --release flag)"
        echo "  restore           - Restore dependencies"
        echo "  clean             - Clean build artifacts"
        echo "  format            - Format code with dotnet format"
        echo "  lint              - Verify code formatting"
        echo "  all               - Run all tasks (restore, format, test, build debug)"
        exit 1
        ;;
esac
