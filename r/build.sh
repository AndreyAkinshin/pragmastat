#!/bin/bash

# Build script for Pragmastat R implementation

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

# Function to copy test data
copy_test_data() {
    print_status "Copying test data..."

    # Remove old test data if exists
    if [ -d "pragmastat/tests/tests" ]; then
        rm -rf pragmastat/tests/tests
    fi

    # Create directory structure
    mkdir -p pragmastat/tests/tests

    # Copy test files from project root
    if [ -d "../tests" ]; then
        cp -r ../tests/* pragmastat/tests/tests/
        print_status "✓ Test data copied to pragmastat/tests/tests/"
    else
        print_error "Test data directory ../tests not found"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    cd pragmastat
    run_command "Rscript -e \"devtools::test()\"" "Running tests"
    cd ..
}

# Function to check package
check_package() {
    run_command "R CMD check --no-tests --no-check-dependencies pragmastat" "Checking package with R CMD check"
}

# Function to check package with full CRAN checks
check_package_full() {
    run_command "R CMD check --as-cran pragmastat" "Checking package with R CMD check --as-cran"
}

# Function to build package
build_package() {
    # Clean compiled objects before building
    rm -rf pragmastat/src/*.o pragmastat/src/*.so pragmastat/src/*.dll 2>/dev/null || true

    run_command "R CMD build pragmastat" "Building source package"
    print_status "Package built. Files in current directory:"
    ls -la pragmastat_*.tar.gz 2>/dev/null || print_warning "No .tar.gz files found"
}

# Function to install package
install_package() {
    cd pragmastat
    run_command "Rscript -e \"if (!requireNamespace('devtools', quietly = TRUE)) install.packages('devtools', repos = 'https://cloud.r-project.org'); devtools::install()\"" "Installing package"
    cd ..
}

# Function to build documentation
build_docs() {
    cd pragmastat
    run_command "Rscript -e \"if (!requireNamespace('devtools', quietly = TRUE)) install.packages('devtools', repos = 'https://cloud.r-project.org'); devtools::document()\"" "Building documentation"
    cd ..
}

# Function to clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    rm -rf pragmastat.Rcheck pragmastat_*.tar.gz pragmastat/..Rcheck ..Rcheck
    rm -rf pragmastat/src/*.o pragmastat/src/*.so pragmastat/src/*.dll
    rm -rf pragmastat/inst/testdata
    print_status "✓ Clean complete"
}

# Function to format code
format_code() {
    cd pragmastat
    if command -v Rscript &> /dev/null; then
        run_command "Rscript -e \"if (!requireNamespace('styler', quietly = TRUE)) install.packages('styler', repos = 'https://cloud.r-project.org'); styler::style_pkg()\"" "Formatting R code"
    else
        print_error "Rscript not found"
        exit 1
    fi
    cd ..
}

# Function to run linting
lint() {
    cd pragmastat
    run_command "Rscript -e \"if (!requireNamespace('lintr', quietly = TRUE)) install.packages('lintr', repos = 'https://cloud.r-project.org'); lintr::lint_package()\"" "Linting R code"
    cd ..
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/r/build.sh ${HIGHLIGHT}<command>${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}       ${DIM}# Run all tests (copies test data first)${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET}      ${DIM}# Build source package (copies test data first)${RESET}"
    echo -e "  ${HIGHLIGHT}check${RESET}      ${DIM}# Run R CMD check (fast, skips tests and dependencies)${RESET}"
    echo -e "  ${HIGHLIGHT}check-full${RESET} ${DIM}# Run R CMD check --as-cran (full CRAN checks)${RESET}"
    echo -e "  ${HIGHLIGHT}install${RESET}    ${DIM}# Install package locally (copies test data first)${RESET}"
    echo -e "  ${HIGHLIGHT}docs${RESET}       ${DIM}# Build documentation with roxygen2${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}      ${DIM}# Clean build artifacts and test data${RESET}"
    echo -e "  ${HIGHLIGHT}format${RESET}     ${DIM}# Format R code with styler${RESET}"
    echo -e "  ${HIGHLIGHT}lint${RESET}       ${DIM}# Lint R code with lintr${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}        ${DIM}# Run all tasks (copy data, format, docs, test, build, check)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}  ${DIM}# Run all tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET} ${DIM}# Build source package${RESET}"
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
        copy_test_data
        run_tests
        ;;
    build)
        copy_test_data
        build_package
        ;;
    check)
        copy_test_data
        check_package
        ;;
    check-full)
        copy_test_data
        check_package_full
        ;;
    install)
        copy_test_data
        install_package
        ;;
    docs)
        build_docs
        ;;
    clean)
        clean
        ;;
    format)
        format_code
        ;;
    lint)
        lint
        ;;
    all)
        print_status "Running all tasks..."
        copy_test_data
        format_code
        build_docs
        run_tests
        build_package
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
