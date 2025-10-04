#!/bin/bash

# Build script for Pragmastat R implementation

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

# Main script
if [ -z "$1" ]; then
    echo "Usage: $0 {test|build|check|check-full|install|docs|clean|format|lint|all}"
    echo ""
    echo "Commands:"
    echo "  test       - Run all tests (copies test data first)"
    echo "  build      - Build source package (copies test data first)"
    echo "  check      - Run R CMD check (fast, skips tests and dependencies)"
    echo "  check-full - Run R CMD check --as-cran (full CRAN checks)"
    echo "  install    - Install package locally (copies test data first)"
    echo "  docs       - Build documentation with roxygen2"
    echo "  clean      - Clean build artifacts and test data"
    echo "  format     - Format R code with styler"
    echo "  lint       - Lint R code with lintr"
    echo "  all        - Run all tasks (copy data, format, docs, test, build, check)"
    exit 1
fi

case "$1" in
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
        echo "Usage: $0 {test|build|check|check-full|install|docs|clean|format|lint|all}"
        echo ""
        echo "Commands:"
        echo "  test       - Run all tests (copies test data first)"
        echo "  build      - Build source package (copies test data first)"
        echo "  check      - Run R CMD check (fast, skips tests and dependencies)"
        echo "  check-full - Run R CMD check --as-cran (full CRAN checks)"
        echo "  install    - Install package locally (copies test data first)"
        echo "  docs       - Build documentation with roxygen2"
        echo "  clean      - Clean build artifacts and test data"
        echo "  format     - Format R code with styler"
        echo "  lint       - Lint R code with lintr"
        echo "  all        - Run all tasks (copy data, format, docs, test, build, check)"
        exit 1
        ;;
esac
