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

# Change to the pragmastat directory
cd "$(dirname "$0")/pragmastat"

# Function to run tests
run_tests() {
    print_info "Running tests..."
    cargo test --verbose
}

# Function to build package
build_package() {
    print_info "Building package..."
    
    # Build in release mode
    cargo build --release
    
    print_info "Build complete."
    
    # Show the built artifacts
    print_info "Built artifacts:"
    ls -la target/release/
}

# Function to check package
check_package() {
    print_info "Checking package..."
    
    # Run clippy for linting
    if command -v cargo-clippy &> /dev/null; then
        cargo clippy -- -D warnings
    else
        print_warning "cargo-clippy not installed. Install with: rustup component add clippy"
    fi
    
    # Check formatting
    if command -v rustfmt &> /dev/null; then
        cargo fmt -- --check
    else
        print_warning "rustfmt not installed. Install with: rustup component add rustfmt"
    fi
    
    # Run cargo check
    cargo check
    
    print_info "Package check complete"
}

# Function to clean build artifacts
clean() {
    print_info "Cleaning build artifacts..."
    cargo clean
    rm -rf Cargo.lock
    print_info "Clean complete"
}

# Function to format code
format() {
    print_info "Formatting code..."
    if command -v rustfmt &> /dev/null; then
        cargo fmt
        print_info "Code formatted"
    else
        print_error "rustfmt not installed. Install with: rustup component add rustfmt"
        exit 1
    fi
}

# Function to build documentation
doc() {
    print_info "Building documentation..."
    cargo doc --no-deps --open
    print_info "Documentation built and opened in browser"
}

# Function to run benchmarks
bench() {
    print_info "Running benchmarks..."
    cargo bench
}

# Function to publish package (dry run by default)
publish() {
    print_info "Publishing package (dry run)..."
    cargo publish --dry-run
    
    echo ""
    print_warning "This was a dry run. To actually publish, run:"
    print_warning "cd pragmastat && cargo publish"
}

# Function to show help
show_help() {
    echo "Usage: ./build.sh [command]"
    echo ""
    echo "Commands:"
    echo "  help       Show this help message"
    echo "  test       Run tests"
    echo "  build      Build release version"
    echo "  check      Check package (clippy, fmt check, cargo check)"
    echo "  clean      Clean build artifacts"
    echo "  format     Format code with rustfmt"
    echo "  doc        Build and open documentation"
    echo "  bench      Run benchmarks"
    echo "  publish    Dry run of publishing to crates.io"
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