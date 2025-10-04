#!/bin/bash

# Build script for Pragmastat Rust implementation

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

# Change to the pragmastat directory
cd "$(dirname "$0")/pragmastat"

# Function to run tests
run_tests() {
    run_command "cargo test --verbose" "Running tests"
}

# Function to build package
build_package() {
    local release_flag="$1"

    if [ "$release_flag" == "--release" ]; then
        run_command "cargo build --release" "Building package in release mode"
        print_status "Built artifacts:"
        ls -la target/release/
    else
        run_command "cargo build" "Building package in debug mode"
        print_status "Built artifacts:"
        ls -la target/debug/
    fi
}

# Function to check package
check_package() {
    print_status "Checking package..."

    # Run clippy for linting
    if command -v cargo-clippy &> /dev/null; then
        run_command "cargo clippy -- -D warnings" "Running clippy"
    else
        print_warning "cargo-clippy not installed. Install with: rustup component add clippy"
    fi

    # Check formatting
    if command -v rustfmt &> /dev/null; then
        run_command "cargo fmt -- --check" "Checking code formatting"
    else
        print_warning "rustfmt not installed. Install with: rustup component add rustfmt"
    fi

    # Run cargo check
    run_command "cargo check" "Running cargo check"

    print_status "✓ Package check complete"
}

# Function to clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    cargo clean
    rm -rf Cargo.lock
    print_status "✓ Clean complete"
}

# Function to format code
format() {
    if command -v rustfmt &> /dev/null; then
        run_command "cargo fmt" "Formatting code"
    else
        print_error "rustfmt not installed. Install with: rustup component add rustfmt"
        exit 1
    fi
}

# Function to build documentation
doc() {
    run_command "cargo doc --no-deps --open" "Building and opening documentation"
}

# Function to run benchmarks
bench() {
    run_command "cargo bench" "Running benchmarks"
}

# Function to publish package (dry run by default)
publish() {
    run_command "cargo publish --dry-run" "Publishing package (dry run)"

    echo ""
    print_warning "This was a dry run. To actually publish, run:"
    print_warning "cd pragmastat && cargo publish"
}

# Function to show help
show_help() {
    echo "Usage: ./build.sh [command] [--release]"
    echo ""
    echo "Commands:"
    echo "  test              Run tests"
    echo "  build [--release] Build package (debug by default, release with --release flag)"
    echo "  check             Check package (clippy, fmt check, cargo check)"
    echo "  clean             Clean build artifacts"
    echo "  format            Format code with rustfmt"
    echo "  doc               Build and open documentation"
    echo "  bench             Run benchmarks"
    echo "  publish           Dry run of publishing to crates.io"
    echo "  all               Run test, build (debug), and check"
    echo ""
    echo "Examples:"
    echo "  ./build.sh test"
    echo "  ./build.sh build"
    echo "  ./build.sh build --release"
    echo "  ./build.sh all"
}

# Main script logic
if [ -z "$1" ]; then
    show_help
    exit 1
fi

case "$1" in
    test)
        run_tests
        ;;
    build)
        build_package "$2"
        ;;
    check)
        check_package
        ;;
    clean)
        clean
        ;;
    format)
        format
        ;;
    doc)
        doc
        ;;
    bench)
        bench
        ;;
    publish)
        publish
        ;;
    all)
        print_status "Running all tasks..."
        run_tests
        build_package ""
        check_package
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac