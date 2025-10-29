#!/bin/bash

# Build script for Pragmastat PDF generation

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

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/pdf/build.sh ${HIGHLIGHT}[command]${RESET} ${ARG}[--release]${RESET}"
    echo ""
    echo -e "If no command is specified, defaults to ${HIGHLIGHT}build${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET} ${ARG}[--release]${RESET}  ${DIM}# Build PDF (draft by default, release with --release flag, default)${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}              ${DIM}# Remove generated PDF files${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET}                ${DIM}# Run all tasks (build draft)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh${RESET}                  ${DIM}# Build draft PDF (default)${RESET}"
    echo -e "  ${SUCCESS}build.sh ${ARG}--release${RESET}          ${DIM}# Build release PDF${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET}            ${DIM}# Build draft PDF${RESET}"
    echo -e "  ${SUCCESS}build.sh build ${ARG}--release${RESET}  ${DIM}# Build release PDF${RESET}"
    echo -e "  ${SUCCESS}build.sh all${RESET}              ${DIM}# Run all tasks${RESET}"
}

# Main script
if [ -z "$1" ]; then
    build_pdf ""
    exit 0
fi

case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
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
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
