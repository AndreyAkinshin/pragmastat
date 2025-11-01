#!/bin/bash

# Build script for Pragmastat Rust implementation

set -e

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

# Function to run demo
run_demo() {
    run_command "cargo run --example demo" "Running demo"
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/rs/build.sh ${HIGHLIGHT}<command>${RESET} ${ARG}[--release]${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}                    ${DIM}# Run tests${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET} ${ARG}[--release]${RESET}       ${DIM}# Build package (debug by default, release with --release flag)${RESET}"
    echo -e "  ${HIGHLIGHT}check${RESET}                   ${DIM}# Check package (clippy, fmt check, cargo check)${RESET}"
    echo -e "  ${HIGHLIGHT}demo${RESET}                    ${DIM}# Run demo examples${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}                   ${DIM}# Clean build artifacts${RESET}"
    echo -e "  ${HIGHLIGHT}format${RESET}                  ${DIM}# Format code with rustfmt${RESET}"
    echo -e "  ${HIGHLIGHT}doc${RESET}                     ${DIM}# Build and open documentation${RESET}"
    echo -e "  ${HIGHLIGHT}bench${RESET}                   ${DIM}# Run benchmarks${RESET}"
    echo -e "  ${HIGHLIGHT}publish${RESET}                 ${DIM}# Dry run of publishing to crates.io${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}                     ${DIM}# Run test, build (debug), and check${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}             ${DIM}# Run tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET}            ${DIM}# Build debug package${RESET}"
    echo -e "  ${SUCCESS}build.sh build ${ARG}--release${RESET}  ${DIM}# Build release package${RESET}"
    echo -e "  ${SUCCESS}build.sh demo${RESET}             ${DIM}# Run demo examples${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}              ${DIM}# Run all tasks${RESET}"
}

# Main script logic
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
        run_tests
        ;;
    build)
        build_package "$2"
        ;;
    check)
        check_package
        ;;
    demo)
        run_demo
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