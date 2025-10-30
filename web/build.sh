#!/bin/bash

# Build script for Pragmastat website

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
BIN_DIR=".bin"
HUGO_BIN="$BIN_DIR/hugo"
TAILWIND_BIN="$BIN_DIR/tailwind"
VERSION_FILE="../manual/version.txt"

# Versions
HUGO_VERSION="0.152.2"
TAILWIND_VERSION="4.1.16"

# Detect OS and architecture
detect_os() {
    case "$(uname -s)" in
        Darwin*)
            echo "darwin"
            ;;
        Linux*)
            echo "linux"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows"
            ;;
        *)
            print_error "Unsupported OS: $(uname -s)"
            exit 1
            ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        arm64|aarch64)
            echo "arm64"
            ;;
        x86_64|amd64)
            echo "amd64"
            ;;
        *)
            print_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
}

# Initialize tools (download Hugo and Tailwind)
init_tools() {
    print_status "Initializing tools (Hugo and Tailwind)..."
    local start_time=$(date +%s)

    # Create .bin directory
    mkdir -p "$BIN_DIR"

    # Detect platform
    local os=$(detect_os)
    local arch=$(detect_arch)

    print_status "Detected platform: $os-$arch"
    echo ""

    # Download Hugo
    print_status "════════════════════════════════════════"
    print_status "Downloading Hugo v${HUGO_VERSION}"
    print_status "════════════════════════════════════════"

    local hugo_os="$os"
    local hugo_arch="$arch"
    local hugo_ext=".tar.gz"
    local hugo_filename="hugo.tar.gz"
    local hugo_binary="hugo"

    # Special case for macOS: use universal binary
    if [ "$os" == "darwin" ]; then
        hugo_arch="universal"
    fi

    # Special case for Windows
    if [ "$os" == "windows" ]; then
        hugo_ext=".zip"
        hugo_filename="hugo.zip"
        hugo_binary="hugo.exe"
    fi

    local hugo_url="https://github.com/gohugoio/hugo/releases/download/v${HUGO_VERSION}/hugo_${HUGO_VERSION}_${hugo_os}-${hugo_arch}${hugo_ext}"

    print_status "URL: $hugo_url"

    if curl -L -o "$BIN_DIR/$hugo_filename" "$hugo_url" --progress-bar; then
        print_status "✓ Hugo downloaded"
    else
        print_error "Failed to download Hugo"
        exit 1
    fi

    # Extract Hugo
    print_status "Extracting Hugo..."
    cd "$BIN_DIR"
    if [ "$os" == "windows" ]; then
        unzip -o "$hugo_filename" "$hugo_binary"
    else
        tar -xzf "$hugo_filename" "$hugo_binary"
    fi
    rm "$hugo_filename"
    chmod +x "$hugo_binary"
    cd - > /dev/null
    print_status "✓ Hugo extracted and ready"

    echo ""

    # Download Tailwind
    print_status "════════════════════════════════════════"
    print_status "Downloading Tailwind v${TAILWIND_VERSION}"
    print_status "════════════════════════════════════════"

    local tailwind_os="$os"
    local tailwind_arch="$arch"
    local tailwind_ext=""
    local tailwind_binary="tailwind"

    # Map OS names for Tailwind
    if [ "$os" == "darwin" ]; then
        tailwind_os="macos"
    fi

    # Map architecture names for Tailwind
    if [ "$arch" == "amd64" ]; then
        tailwind_arch="x64"
    fi

    # Special case for Windows
    if [ "$os" == "windows" ]; then
        tailwind_ext=".exe"
        tailwind_binary="tailwind.exe"
    fi

    local tailwind_url="https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-${tailwind_os}-${tailwind_arch}${tailwind_ext}"

    print_status "URL: $tailwind_url"

    if curl -L -o "$BIN_DIR/$tailwind_binary" "$tailwind_url" --progress-bar; then
        print_status "✓ Tailwind downloaded"
    else
        print_error "Failed to download Tailwind"
        exit 1
    fi

    # Set executable permission
    chmod +x "$BIN_DIR/$tailwind_binary"
    print_status "✓ Tailwind ready"

    local end_time=$(date +%s)
    local elapsed=$((end_time - start_time))

    echo ""
    print_status "════════════════════════════════════════"
    print_status "✓ Tools initialized successfully!"
    print_status "Total time: ${elapsed}s"
    print_status "════════════════════════════════════════"
    print_status "Downloaded to: $BIN_DIR/"
    print_status "  - hugo (v${HUGO_VERSION})"
    print_status "  - tailwind (v${TAILWIND_VERSION})"
}

# Check if required tools exist
check_tools() {
    if [ ! -f "$HUGO_BIN" ] || [ ! -f "$TAILWIND_BIN" ]; then
        print_error "Hugo or Tailwind not found in web/.bin directory"
        print_error "Please run './build.sh web init' or './web/build.sh init' to download tools"
        exit 1
    fi
}

# Run Tailwind to generate CSS
run_tailwind() {
    print_status "Running Tailwind CSS..."
    local start_time=$(date +%s)

    run_command "$TAILWIND_BIN -i ./assets/css/styles-tailwindcss.css -o ./assets/css/styles.css --minify" "Generating CSS with Tailwind"

    local end_time=$(date +%s)
    local elapsed=$((end_time - start_time))
    print_status "Tailwind completed in ${elapsed}s"
}

# Prepare website assets
prepare_assets() {
    local release_flag="$1"
    local is_release="false"

    if [ "$release_flag" == "--release" ]; then
        is_release="true"
    fi

    # Read version
    if [ ! -f "$VERSION_FILE" ]; then
        print_error "Version file not found: $VERSION_FILE"
        exit 1
    fi
    local version=$(cat "$VERSION_FILE")
    local date=$(date +%Y-%m-%d)

    print_status "Preparing assets (version: $version, release: $is_release)..."

    # Copy PDF file
    print_status "Copying PDF file to static/..."
    local suffix=""
    if [ "$is_release" == "false" ]; then
        suffix="-draft"
    fi
    local pdf_file="../pdf/pragmastat-v${version}${suffix}.pdf"

    if [ -f "$pdf_file" ]; then
        cp "$pdf_file" static/pragmastat.pdf
        print_status "  Copied: $(basename "$pdf_file") -> pragmastat.pdf"
        print_status "✓ PDF file copied successfully"
    else
        print_error "PDF file not found: $pdf_file"
        exit 1
    fi

    # Write config.toml
    print_status "Writing configuration to data/config.toml..."
    cat > data/config.toml << EOF
version = "$version"
date = "$date"
isRelease = $is_release
EOF
    print_status "✓ Configuration written"

    # Copy favicon
    if [ -f "../img/logo.ico" ]; then
        mkdir -p content/img
        cp ../img/logo.ico content/img/favicon.ico
        print_status "✓ Favicon copied"
    else
        print_warning "Logo file not found: ../img/logo.ico"
    fi
}

# Build website with Hugo
build_hugo() {
    print_status "Building website with Hugo..."
    local start_time=$(date +%s)

    run_command "$HUGO_BIN --minify" "Building Hugo site"

    local end_time=$(date +%s)
    local elapsed=$((end_time - start_time))
    print_status "Hugo build completed in ${elapsed}s"
    print_status "Website built to: public/"
}

# Serve website with Hugo
serve_hugo() {
    local port="1729"

    print_status "Starting Hugo development server..."
    print_status "Server will be available at: http://localhost:$port"
    print_status "Press Ctrl+C to stop the server"
    echo ""

    $HUGO_BIN server \
        --renderToMemory \
        --port "$port" \
        --liveReloadPort "$port" \
        --forceSyncStatic \
        --gc \
        --watch \
        --buildDrafts
}

# Clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    rm -rf public/ resources/
    rm -f .hugo_build.lock
    rm -f assets/css/styles.css
    print_status "✓ Clean complete"
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} pragmastat/web/build.sh ${HIGHLIGHT}[command]${RESET} ${ARG}[--release]${RESET}"
    echo ""
    echo -e "If no command is specified, defaults to ${HIGHLIGHT}build${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Commands:${RESET}"
    echo -e "  ${HIGHLIGHT}init${RESET}                     ${DIM}# Download Hugo and Tailwind for current platform${RESET}"
    echo -e "  ${HIGHLIGHT}build${RESET} ${ARG}[--release]${RESET}        ${DIM}# Build website (draft by default, release with --release flag, default)${RESET}"
    echo -e "  ${HIGHLIGHT}serve${RESET}                    ${DIM}# Start Hugo development server${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}                    ${DIM}# Clean build artifacts${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Examples:${RESET}"
    echo -e "  ${SUCCESS}build.sh${RESET}                  ${DIM}# Build draft website (default)${RESET}"
    echo -e "  ${SUCCESS}build.sh ${ARG}--release${RESET}          ${DIM}# Build release website${RESET}"
    echo -e "  ${SUCCESS}build.sh init${RESET}             ${DIM}# Download tools${RESET}"
    echo -e "  ${SUCCESS}build.sh build${RESET}            ${DIM}# Build draft website${RESET}"
    echo -e "  ${SUCCESS}build.sh build ${ARG}--release${RESET}  ${DIM}# Build release website${RESET}"
    echo -e "  ${SUCCESS}build.sh serve${RESET}            ${DIM}# Start dev server${RESET}"
}

# Main script
# Default to 'build' if no arguments provided
if [ -z "$1" ]; then
    set -- "build"
fi

case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
    --release)
        # If --release is the first arg, default to build --release
        check_tools
        run_tailwind
        prepare_assets "--release"
        build_hugo
        print_status "✓ Website build completed successfully!"
        ;;
    init)
        init_tools
        ;;
    build)
        check_tools
        run_tailwind
        prepare_assets "$2"
        build_hugo
        print_status "✓ Website build completed successfully!"
        ;;
    serve)
        check_tools
        prepare_assets ""
        serve_hugo
        ;;
    clean)
        clean
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
