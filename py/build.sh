#!/bin/bash

# Build script for Pragmastat Python implementation

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
    
    # Activate virtual environment
    # shellcheck disable=SC1091
    source "$venv_dir/bin/activate"
    
    # Ensure basic build tools are installed in the venv
    if ! python -m pytest --version &>/dev/null || ! python -m build --version &>/dev/null; then
        print_status "Installing build tools in virtual environment..."
        pip install -q --upgrade pip
        pip install -q pytest build twine
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

# Function to ensure package is installed
ensure_package_installed() {
    # Set up venv for local runs
    ensure_venv
    
    # Check if package is already installed
    if ! python -c "import pragmastat" &>/dev/null; then
        print_status "Package not installed, installing in editable mode..."
        # In Docker with user mapping, install to a writable location
        if [ -f "/.dockerenv" ]; then
            # Create a writable pip install directory
            export PYTHONUSERBASE="/workspace/py/.pip-packages"
            export PATH="$PYTHONUSERBASE/bin:$PATH"
            export PYTHONPATH="$PYTHONUSERBASE/lib/python$(python -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}")')/site-packages:$PYTHONPATH"
            pip install -q --user -e .
        else
            pip install -q -e .
        fi
    fi
}

# Function to run tests
run_tests() {
    ensure_package_installed
    # Set PYTHONPATH for Docker if needed
    if [ -f "/.dockerenv" ] && [ -d "/workspace/py/.pip-packages" ]; then
        export PYTHONUSERBASE="/workspace/py/.pip-packages"
        export PATH="$PYTHONUSERBASE/bin:$PATH"
        export PYTHONPATH="$PYTHONUSERBASE/lib/python$(python -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}")')/site-packages:$PYTHONPATH"
    fi
    run_command "python -m pytest tests/ -v" "Running tests"
}

# Function to build package
build_package() {
    # Set up venv for local runs
    ensure_venv
    
    # Check if build is available (either as module or pipx command)
    if command -v pyproject-build &> /dev/null; then
        BUILD_CMD="pyproject-build"
    elif python -m build --version &> /dev/null; then
        BUILD_CMD="python -m build"
    else
        print_error "Python 'build' module not installed. Install with: pip install build or pipx install build"
        exit 1
    fi

    # Ensure numpy is installed (required by setup.py for building C extensions)
    if ! python -c "import numpy" &>/dev/null; then
        print_status "Installing numpy (required for building C extensions)..."
        # In Docker with user mapping, install to a writable location
        if [ -f "/.dockerenv" ]; then
            export PYTHONUSERBASE="/workspace/py/.pip-packages"
            export PATH="$PYTHONUSERBASE/bin:$PATH"
            export PYTHONPATH="$PYTHONUSERBASE/lib/python$(python -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}")')/site-packages:$PYTHONPATH"
            pip install -q --user "numpy>=1.20"
        else
            pip install -q "numpy>=1.20"
        fi
    fi

    print_status "Building package..."

    # Clean previous builds
    rm -rf dist/ build/ *.egg-info

    # Build the package
    run_command "$BUILD_CMD" "Building Python package"

    print_status "Build complete. Files in dist/:"
    ls -la dist/
}

# Function to check package
check_package() {
    if [ ! -d "dist" ]; then
        print_error "No dist directory found. Run './build.sh build' first."
        exit 1
    fi

    # Check if twine is available
    if ! command -v twine &> /dev/null; then
        print_error "twine not installed. Install with: pip install twine"
        exit 1
    fi

    run_command "twine check dist/*" "Checking package with twine"
}

# Function to clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    rm -rf build/ dist/ *.egg-info __pycache__/ .pytest_cache/
    find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
    find . -type f -name "*.pyc" -delete 2>/dev/null || true
    find . -type f -name "*.pyo" -delete 2>/dev/null || true
    # Also clean virtual environment and Docker pip packages
    if [ -d "venv" ]; then
        print_status "Removing virtual environment..."
        rm -rf venv
    fi
    if [ -d ".pip-packages" ]; then
        print_status "Removing Docker pip packages..."
        rm -rf .pip-packages
    fi
    print_status "✓ Clean complete"
}

# Function to run development install
dev_install() {
    # Set up venv for local runs
    ensure_venv
    run_command "pip install -e ." "Installing package in development mode"
}

# Function to format code
format_code() {
    ensure_venv
    if command -v black &> /dev/null; then
        run_command "black ." "Formatting code with black"
    else
        print_error "black not installed. Install with: pip install black"
        exit 1
    fi
}

# Function to lint code
lint_code() {
    ensure_venv
    if command -v flake8 &> /dev/null; then
        run_command "flake8 ." "Linting code with flake8"
    else
        print_error "flake8 not installed. Install with: pip install flake8"
        exit 1
    fi
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/py/build.sh ${HIGHLIGHT}<command>${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}dev${RESET}        ${DIM}# Install package in development mode${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}       ${DIM}# Run tests${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET}      ${DIM}# Build distribution packages${RESET}"
    echo -e "  ${HIGHLIGHT}check${RESET}      ${DIM}# Check package with twine${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}      ${DIM}# Clean build artifacts${RESET}"
    echo -e "  ${HIGHLIGHT}format${RESET}     ${DIM}# Format code with black${RESET}"
    echo -e "  ${HIGHLIGHT}lint${RESET}       ${DIM}# Lint code with flake8${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}        ${DIM}# Run test, build, and check${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh test${RESET}  ${DIM}# Run tests${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET} ${DIM}# Build package${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}   ${DIM}# Run all tasks${RESET}"
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
    format)
        format_code
        ;;
    lint)
        lint_code
        ;;
    all)
        print_status "Running all tasks..."

        # Run format if black is available
        if command -v black &> /dev/null; then
            format_code
        else
            print_warning "black not installed, skipping formatting (install with: pip install black)"
        fi

        # Run lint if flake8 is available
        if command -v flake8 &> /dev/null; then
            lint_code
        else
            print_warning "flake8 not installed, skipping linting (install with: pip install flake8)"
        fi

        run_tests

        # Build if build is available (either as module or pipx command)
        if command -v pyproject-build &> /dev/null || python -m build --version &> /dev/null; then
            build_package
        else
            print_warning "Python 'build' module not installed, skipping build (install with: pip install build or pipx install build)"
        fi

        # Check if twine is available and dist exists
        if [ -d "dist" ] && command -v twine &> /dev/null; then
            check_package
        elif [ -d "dist" ]; then
            print_warning "twine not installed, skipping package check (install with: pip install twine)"
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