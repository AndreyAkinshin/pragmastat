#!/bin/bash

# Docker runner script for Pragmastat
# Runs build commands inside Docker containers

set -e

# Change to script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

# Colors for output
ERROR='\033[0;31m'
SUCCESS='\033[0;32m'
HIGHLIGHT='\033[1;33m'
RESET='\033[0m'

print_error() {
    echo -e "${ERROR}ERROR:${RESET} $1" >&2
}

print_info() {
    echo -e "${SUCCESS}INFO:${RESET} $1"
}

print_warning() {
    echo -e "${HIGHLIGHT}WARNING:${RESET} $1"
}

# Function to ensure Docker is available
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed or not in PATH"
        print_error "Please install Docker: https://docs.docker.com/get-docker/"
        exit 1
    fi

    if ! docker info &> /dev/null; then
        print_error "Docker is not running or you don't have permission to access it"
        print_error "Please start Docker daemon or add your user to the docker group"
        exit 1
    fi
}

# Function to ensure Docker Compose is available
check_docker_compose() {
    if ! docker compose version &> /dev/null; then
        print_error "Docker Compose is not available"
        print_error "Please install Docker Compose (v2+): https://docs.docker.com/compose/install/"
        exit 1
    fi
}

# Function to build Docker image for a service
build_docker_image() {
    local service="$1"
    
    print_info "Building Docker image for: $service"
    
    if docker compose build "$service"; then
        print_info "✓ Docker image built successfully for: $service"
    else
        print_error "Failed to build Docker image for: $service"
        exit 1
    fi
}

# Function to run command in Docker container
run_in_docker() {
    local service="$1"
    shift
    local cmd="$@"
    
    # Ensure the image is built
    if ! docker compose images "$service" | grep -q "$service"; then
        print_info "Image not found for $service, building..."
        build_docker_image "$service"
    fi
    
    print_info "Running in Docker container ($service): $cmd"
    
    # Determine the shell to use based on the service
    local shell="bash"
    if [ "$service" = "pdf" ]; then
        shell="sh"
    fi
    
    # Run the command in the container with user mapping to prevent root ownership
    docker compose run --rm --user "$(id -u):$(id -g)" "$service" "$shell" -c "$cmd"
}

# Function to show help
show_help() {
    echo "Usage: $0 <service> <command> [args...]"
    echo ""
    echo "Runs a command inside a Docker container for the specified service."
    echo ""
    echo "Services:"
    echo "  cs, py, rs, go, ts, kt, r    # Language implementations"
    echo "  img, pdf, web, gen           # Auxiliary builds"
    echo ""
    echo "Examples:"
    echo "  $0 cs ./build.sh build"
    echo "  $0 py ./build.sh test"
    echo "  $0 rs ./build.sh check"
    echo ""
    echo "Special commands:"
    echo "  $0 build-all                 # Build all Docker images"
    echo "  $0 clean                     # Remove all Docker containers and images"
}

# Check Docker availability
check_docker
check_docker_compose

# Handle special commands
case "$1" in
    -h|--help|help)
        show_help
        exit 0
        ;;
    build-all)
        print_info "Building all Docker images..."
        docker compose build
        exit 0
        ;;
    clean)
        print_info "Cleaning Docker containers and images..."
        docker compose down --rmi all --volumes --remove-orphans
        exit 0
        ;;
esac

# Check arguments
if [ $# -lt 2 ]; then
    print_error "Insufficient arguments"
    echo ""
    show_help
    exit 1
fi

SERVICE="$1"
shift

# Validate service name
VALID_SERVICES=("cs" "py" "rs" "go" "ts" "kt" "r" "img" "pdf" "web" "gen")
if [[ ! " ${VALID_SERVICES[@]} " =~ " ${SERVICE} " ]]; then
    print_error "Invalid service: $SERVICE"
    echo ""
    echo "Valid services: ${VALID_SERVICES[*]}"
    exit 1
fi

# Run the command
run_in_docker "$SERVICE" "$@"

