#!/bin/bash

# Main build dispatcher script for Pragmastat

set -e

# Change to script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

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

# Function to run a command on all projects
run_all_projects() {
    local command="$1"
    local release_flag="$2"
    local start_time=$(date +%s)

    print_status "Starting '$command' for all projects..."
    echo ""

    # Define all projects
    local all_projects=(
        "img"
        "pdf"
        "web"
        "r"
        "cs"
        "py"
        "rs"
        "ts"
        "go"
        "kt"
    )

    # Projects that support --release flag
    local projects_with_release=("pdf" "web" "cs" "rs")

    local failed_projects=()
    local succeeded_projects=()
    local skipped_projects=()

    for project in "${all_projects[@]}"; do
        echo ""
        print_status "════════════════════════════════════════"
        print_status "Running '$command' for: $project"
        print_status "════════════════════════════════════════"

        # Check if project has build.sh
        if [ ! -f "./$project/build.sh" ]; then
            print_warning "No build.sh found for $project, skipping"
            skipped_projects+=("$project")
            continue
        fi

        # Check if this project supports --release flag
        local supports_release=false
        for release_proj in "${projects_with_release[@]}"; do
            if [ "$project" == "$release_proj" ]; then
                supports_release=true
                break
            fi
        done

        # Run the command
        if [ "$supports_release" == "true" ] && [ "$release_flag" == "--release" ]; then
            if "./$project/build.sh" "$command" --release; then
                succeeded_projects+=("$project")
            else
                failed_projects+=("$project")
                print_error "✗ Failed to run '$command' for: $project"
            fi
        else
            if "./$project/build.sh" "$command"; then
                succeeded_projects+=("$project")
            else
                failed_projects+=("$project")
                print_error "✗ Failed to run '$command' for: $project"
            fi
        fi
    done

    local end_time=$(date +%s)
    local elapsed=$((end_time - start_time))

    echo ""
    print_status "════════════════════════════════════════"
    print_status "Summary: $command"
    print_status "════════════════════════════════════════"
    print_status "Total time: ${elapsed}s"
    print_status "Succeeded: ${#succeeded_projects[@]}"
    print_status "Failed: ${#failed_projects[@]}"
    print_status "Skipped: ${#skipped_projects[@]}"

    if [ ${#failed_projects[@]} -gt 0 ]; then
        echo ""
        print_error "Failed projects:"
        for project in "${failed_projects[@]}"; do
            print_error "  - $project"
        done
        exit 1
    else
        echo ""
        print_status "✓ All projects completed '$command' successfully!"
    fi
}

# Function to run CI build (replicates GitHub Actions workflow)
run_ci() {
    local release_flag="$1"
    local start_time=$(date +%s)

    print_status "Starting CI build process..."
    echo ""

    local failed_steps=()
    local succeeded_steps=()

    # Helper function to run a step
    run_step() {
        local step_name="$1"
        shift
        local step_command="$@"

        echo ""
        print_status "════════════════════════════════════════"
        print_status "CI Step: $step_name"
        print_status "════════════════════════════════════════"

        if (cd "$SCRIPT_DIR" && eval "$step_command"); then
            succeeded_steps+=("$step_name")
            print_status "✓ Step completed: $step_name"
        else
            failed_steps+=("$step_name")
            print_error "✗ Step failed: $step_name"
            return 1
        fi
    }

    # Build img
    run_step "build-img" "./img/build.sh build" || exit 1

    # Build pdf (depends on img)
    run_step "gen-for-pdf" "./build.sh gen $release_flag" || exit 1
    run_step "build-pdf" "./pdf/build.sh build $release_flag" || exit 1

    # Build web (depends on img and pdf)
    run_step "gen-for-web" "./build.sh gen $release_flag" || exit 1
    run_step "init-web" "./web/build.sh init" || exit 1
    run_step "setup-tailwind-wrapper" "./web/setup-tailwind-wrapper.sh" || exit 1
    run_step "build-web" "./web/build.sh build $release_flag" || exit 1

    # Build r
    run_step "check-r" "./r/build.sh check" || exit 1
    run_step "build-r" "./r/build.sh build" || exit 1
    run_step "test-r" "./r/build.sh test" || exit 1

    # Build cs
    run_step "build-cs" "./cs/build.sh build $release_flag" || exit 1
    run_step "test-cs" "./cs/build.sh test" || exit 1
    run_step "pack-cs" "./cs/build.sh pack $release_flag" || exit 1

    # Build py
    run_step "test-py" "./py/build.sh test" || exit 1
    run_step "build-py" "./py/build.sh build" || exit 1
    run_step "check-py" "./py/build.sh check" || exit 1

    # Build rs
    run_step "check-rs" "./rs/build.sh check" || exit 1
    run_step "test-rs" "./rs/build.sh test" || exit 1
    run_step "build-rs" "./rs/build.sh build $release_flag" || exit 1
    run_step "package-rs" "cd rs/pragmastat && cargo package --verbose" || exit 1

    # Build ts
    run_step "install-ts" "cd ts && npm ci" || exit 1
    run_step "check-ts" "./ts/build.sh check" || exit 1
    run_step "test-ts" "./ts/build.sh test" || exit 1
    run_step "build-ts" "./ts/build.sh build" || exit 1
    run_step "pack-ts" "cd ts && npm pack" || exit 1

    # Build go
    run_step "deps-go" "./go/build.sh deps" || exit 1
    run_step "test-go" "./go/build.sh test-verbose" || exit 1
    run_step "build-go" "./go/build.sh build" || exit 1

    # Build kt
    run_step "build-kt" "cd kt && ./gradlew build --info --stacktrace" || exit 1

    local end_time=$(date +%s)
    local elapsed=$((end_time - start_time))

    echo ""
    print_status "════════════════════════════════════════"
    print_status "CI Build Summary"
    print_status "════════════════════════════════════════"
    print_status "Total time: ${elapsed}s"
    print_status "Succeeded: ${#succeeded_steps[@]}"
    print_status "Failed: ${#failed_steps[@]}"

    if [ ${#failed_steps[@]} -gt 0 ]; then
        echo ""
        print_error "Failed steps:"
        for step in "${failed_steps[@]}"; do
            print_error "  - $step"
        done
        exit 1
    else
        echo ""
        print_status "✓ All CI steps completed successfully!"
    fi
}

# Function to perform release
do_release() {
    local version="$1"
    local push_flag="$2"

    if [ -z "$version" ] || [[ "$version" == -* ]]; then
        echo "Usage: $0 release <version> [--push]"
        echo ""
        echo "Creates a new release by:"
        echo "  1. Writing version to manual/version.txt"
        echo "  2. Running ./build.sh gen"
        echo "  3. Creating commit 'set version <version>'"
        echo "  4. Moving main branch to HEAD"
        echo ""
        echo "With --push flag, also:"
        echo "  5. Adding upstream remote"
        echo "  6. Creating tags v<version> and go/v<version>"
        echo "  7. Pushing main branch and tags to upstream"
        echo "  8. Removing upstream remote"
        echo ""
        echo "Examples:"
        echo "  $0 release 0.1.0         # Create release 0.1.0 locally"
        echo "  $0 release v0.2.0 --push # Create and push release 0.2.0"
        exit 1
    fi

    # Remove 'v' prefix if present
    version="${version#v}"

    # Check git status for uncommitted changes
    if ! git diff-index --quiet HEAD -- 2>/dev/null; then
        print_error "Git working directory is not clean"
        print_error "Please commit or stash your changes before creating a release"
        git status --short
        exit 1
    fi

    print_status "Starting release process for version: $version"

    # 1. Dump version to manual/version.txt
    print_status "Writing version to manual/version.txt"
    echo "$version" > ./manual/version.txt

    # 2. Run ./build.sh gen
    print_status "Running ./build.sh gen"
    if ! ./build.sh gen; then
        print_error "Failed to run ./build.sh gen"
        exit 1
    fi

    # 3. Make commit
    print_status "Creating commit"
    git add -A
    if ! git commit -m "set version $version"; then
        print_error "Failed to create commit"
        exit 1
    fi

    # 4. Move branch main to HEAD
    print_status "Moving main branch to HEAD"
    if ! git branch -f main HEAD; then
        print_error "Failed to move main branch"
        exit 1
    fi

    # 5. If --push flag is given
    if [ "$push_flag" == "--push" ]; then
        print_status "Pushing to upstream..."

        # Add upstream remote
        print_status "Adding upstream remote"
        if ! git remote add upstream git@github.com:AndreyAkinshin/pragmastat.git 2>/dev/null; then
            print_warning "upstream remote already exists, updating URL"
            git remote set-url upstream git@github.com:AndreyAkinshin/pragmastat.git
        fi

        # Create tags
        print_status "Creating tags: v$version and go/v$version"
        if ! git tag "v$version"; then
            print_error "Failed to create tag v$version"
            exit 1
        fi
        if ! git tag "go/v$version"; then
            print_error "Failed to create tag go/v$version"
            exit 1
        fi

        # Push main branch and tags
        print_status "Pushing main branch and tags to upstream"
        if ! git push upstream main; then
            print_error "Failed to push main branch"
            exit 1
        fi
        if ! git push upstream "v$version"; then
            print_error "Failed to push tag v$version"
            exit 1
        fi
        if ! git push upstream "go/v$version"; then
            print_error "Failed to push tag go/v$version"
            exit 1
        fi

        # Remove upstream remote
        print_status "Removing upstream remote"
        git remote remove upstream

        print_status "✓ Release $version pushed successfully!"
    else
        print_status "✓ Release $version completed locally (not pushed)"
    fi
}

# Function to show help
show_help() {
    echo -e "${BOLD}Usage:${RESET} $0 ${HIGHLIGHT}<lang>${RESET} ${ARG}<command> [args]${RESET}"
    echo -e "       $0 ${HIGHLIGHT}<aux>${RESET}  ${ARG}[command] [args]${RESET}"
    echo -e "       $0 ${HIGHLIGHT}<meta>${RESET} ${ARG}[args]${RESET}"
    echo ""
    echo -e "Pragmastat Build Dispatcher"
    echo ""
    echo -e "${HEADER}${BOLD}Language commands:${RESET}"
    echo -e "  ${HIGHLIGHT}cs${RESET}   ${DIM}# C# (.NET)${RESET}"
    echo -e "  ${HIGHLIGHT}go${RESET}   ${DIM}# Go${RESET}"
    echo -e "  ${HIGHLIGHT}kt${RESET}   ${DIM}# Kotlin (JVM)${RESET}"
    echo -e "  ${HIGHLIGHT}py${RESET}   ${DIM}# Python${RESET}"
    echo -e "  ${HIGHLIGHT}r${RESET}    ${DIM}# R${RESET}"
    echo -e "  ${HIGHLIGHT}rs${RESET}   ${DIM}# Rust${RESET}"
    echo -e "  ${HIGHLIGHT}ts${RESET}   ${DIM}# TypeScript (npm)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Auxiliary commands:${RESET}"
    echo -e "  ${HIGHLIGHT}gen${RESET}  ${DIM}# Content and auxiliary files generation${RESET}"
    echo -e "  ${HIGHLIGHT}img${RESET}  ${DIM}# Image generation${RESET}"
    echo -e "  ${HIGHLIGHT}pdf${RESET}  ${DIM}# PDF manual generation${RESET}"
    echo -e "  ${HIGHLIGHT}web${RESET}  ${DIM}# Online manual/website (Hugo)${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Meta commands:${RESET}"
    echo -e "  ${HIGHLIGHT}all${RESET} ${ARG}[--release]${RESET}        ${DIM}# Build all projects${RESET}"
    echo -e "  ${HIGHLIGHT}ci${RESET} ${ARG}[--release]${RESET}         ${DIM}# Run full CI build (replicates GitHub Actions)${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET}                   ${DIM}# Run tests for all projects${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET}                  ${DIM}# Clean all projects${RESET}"
    echo -e "  ${HIGHLIGHT}release${RESET} ${ARG}<ver> [--push]${RESET} ${DIM}# Create release version${RESET}"
}

# Check if no arguments provided
if [ $# -eq 0 ]; then
    show_help
    exit 1
fi

# Extract directory and remaining arguments
DIR="$1"
shift

# Check for meta-commands
case "$DIR" in
    all)
        # Handle optional --release flag
        release_flag=""
        if [ "$1" == "--release" ]; then
            release_flag="--release"
        fi
        run_all_projects "build" "$release_flag"
        exit 0
        ;;
    ci)
        # Handle optional --release flag
        release_flag=""
        if [ "$1" == "--release" ]; then
            release_flag="--release"
        fi
        run_ci "$release_flag"
        exit 0
        ;;
    test)
        run_all_projects "test" ""
        exit 0
        ;;
    clean)
        run_all_projects "clean" ""
        exit 0
        ;;
    release)
        # Handle version and optional --push flag
        version="$1"
        push_flag=""
        if [ -n "$1" ]; then
            shift
            if [ "$1" == "--push" ]; then
                push_flag="--push"
            fi
        fi
        do_release "$version" "$push_flag"
        exit 0
        ;;
esac

# Check if directory/build.sh exists
if [ -f "$DIR/build.sh" ]; then
    # Dispatch to the ecosystem-specific build script
    exec "./$DIR/build.sh" "$@"
else
    print_error "Unknown command or directory: $DIR"
    echo ""
    show_help
    exit 1
fi
