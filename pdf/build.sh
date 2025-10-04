#!/bin/bash

# Build script for Pragmastat PDF generation

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

# Constants
BASENAME=pragmastat
VERSION=$(cat ../manual/version.txt)

# Function to build PDF
build_pdf() {
    local release_flag="$1"
    local filename

    if [ "$release_flag" == "--release" ]; then
        filename="$BASENAME-v$VERSION.pdf"
        print_status "Building release PDF: $filename"
    else
        filename="$BASENAME-v$VERSION-draft.pdf"
        print_status "Building draft PDF: $filename"
    fi

    # Run pandoc with citeproc for faster bibliography processing
    run_command "pandoc '$BASENAME.md' \
        --pdf-engine=latexmk \
        --pdf-engine-opt=-xelatex \
        --pdf-engine-opt=-shell-escape \
        --pdf-engine-opt=-interaction=batchmode \
        --number-sections \
        --citeproc \
        -o '$filename'" "Rendering Markdown to PDF with Pandoc"

    print_status "Result: $filename"
}

# Main script
if [ -z "$1" ]; then
    build_pdf ""
    exit 0
fi

case "$1" in
    build)
        build_pdf "$2"
        ;;
    clean)
        print_status "Cleaning generated PDF files..."
        rm -f "$BASENAME-v"*.pdf 2>/dev/null || true
        print_status "✓ Clean complete"
        ;;
    all)
        print_status "Running all tasks..."
        build_pdf ""
        print_status "✓ All tasks completed successfully!"
        ;;
    *)
        echo "Usage: $0 {build|clean|all} [--release]"
        echo ""
        echo "Commands:"
        echo "  build [--release]  - Build PDF (draft by default, release with --release flag)"
        echo "  clean              - Remove generated PDF files"
        echo "  all                - Run all tasks (build draft)"
        exit 1
        ;;
esac
