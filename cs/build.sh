#!/bin/bash

# Build script for Pragmastat .NET implementation

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

# Function to generate reference tests
generate_tests() {
    run_command "dotnet run --project Pragmastat.ReferenceTests.Generator/Pragmastat.ReferenceTests.Generator.csproj" "Generating reference test files"
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

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/cs/build.sh ${HIGHLIGHT}<command>${RESET} ${ARG}[--release]${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}generate${RESET}                ${DIM}# Generate reference test files${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}                    ${DIM}# Run all tests${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET} ${ARG}[--release]${RESET}       ${DIM}# Build package (debug by default, release with --release flag)${RESET}"
    echo -e "  ${HIGHLIGHT}pack${RESET} ${ARG}[--release]${RESET}        ${DIM}# Pack NuGet package (debug by default, release with --release flag)${RESET}"
    echo -e "  ${HIGHLIGHT}restore${RESET}                 ${DIM}# Restore dependencies${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}                   ${DIM}# Clean build artifacts${RESET}"
    echo -e "  ${HIGHLIGHT}format${RESET}                  ${DIM}# Format code with dotnet format${RESET}"
    echo -e "  ${HIGHLIGHT}lint${RESET}                    ${DIM}# Verify code formatting${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}                     ${DIM}# Run all tasks (restore, format, test, build debug)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh generate${RESET}         ${DIM}# Generate reference test files${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}             ${DIM}# Run all tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET}            ${DIM}# Build debug package${RESET}"
    echo -e "  ${SUCCESS}build.sh build ${ARG}--release${RESET}  ${DIM}# Build release package${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}              ${DIM}# Run all tasks${RESET}"
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
    generate|gen)
        generate_tests
        ;;
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
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
