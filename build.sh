#!/bin/bash

# Main build dispatcher script for Pragmastat

set -e

# Change to script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_error() {
    echo -e "${RED}ERROR:${NC} $1" >&2
}

print_info() {
    echo -e "${GREEN}INFO:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

print_status() {
    echo -e "${GREEN}[$(date +'%H:%M:%S')]${NC} $1"
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
        "dotnet"
        "python"
        "rust"
        "ts"
        "go"
        "kotlin"
    )

    # Projects that support --release flag
    local projects_with_release=("pdf" "web" "dotnet" "rust")

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

    # Build dotnet
    run_step "build-dotnet" "./dotnet/build.sh build $release_flag" || exit 1
    run_step "test-dotnet" "./dotnet/build.sh test" || exit 1
    run_step "pack-dotnet" "./dotnet/build.sh pack $release_flag" || exit 1

    # Build python
    run_step "test-python" "./python/build.sh test" || exit 1
    run_step "build-python" "./python/build.sh build" || exit 1
    run_step "check-python" "./python/build.sh check" || exit 1

    # Build rust
    run_step "check-rust" "./rust/build.sh check" || exit 1
    run_step "test-rust" "./rust/build.sh test" || exit 1
    run_step "build-rust" "./rust/build.sh build $release_flag" || exit 1
    run_step "package-rust" "cd rust/pragmastat && cargo package --verbose" || exit 1

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

    # Build kotlin
    run_step "build-kotlin" "cd kotlin && ./gradlew build --info --stacktrace" || exit 1

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
    echo "Usage: $0 <dir> <verb> [flags]"
    echo "       $0 <meta-command> [--release]"
    echo ""
    echo "This script dispatches build commands to ecosystem-specific build scripts."
    echo ""
    echo "Meta-commands:"
    echo "  all [--release]       - Build all projects"
    echo "  ci [--release]        - Run full CI build (replicates GitHub Actions)"
    echo "  test                  - Run tests for all projects"
    echo "  clean                 - Clean all projects"
    echo "  release <ver> [--push] - Create release version"
    echo ""
    echo "Available directories with build scripts:"

    # Find all directories with build.sh
    for dir in */build.sh; do
        if [ -f "$dir" ]; then
            dirname=$(dirname "$dir")
            echo "  - $dirname"
        fi
    done | sort

    echo ""
    echo "Examples:"
    echo "  $0 all                      # Build all projects"
    echo "  $0 all --release            # Build all projects in release mode"
    echo "  $0 ci                       # Run full CI build locally"
    echo "  $0 ci --release             # Run full CI build in release mode"
    echo "  $0 test                     # Run tests for all projects"
    echo "  $0 clean                    # Clean all projects"
    echo "  $0 release 0.1.0            # Create release 0.1.0 locally"
    echo "  $0 release v0.2.0 --push    # Create and push release 0.2.0"
    echo "  $0 go test                  # Run tests in Go implementation"
    echo "  $0 rust build --release     # Build Rust implementation in release mode"
    echo "  $0 web build --release      # Build website in release mode"
    echo "  $0 python all               # Run all tasks for Python implementation"
    echo ""
    echo "For ecosystem-specific commands, run:"
    echo "  <dir>/build.sh          # Show help for specific ecosystem"
    echo ""
    echo "If the specified directory doesn't have a build.sh, the command will be"
    echo "passed to the .NET build system."
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
    print_info "Dispatching to $DIR/build.sh"
    exec "./$DIR/build.sh" "$@"
else
    # Fall back to .NET build system
    print_info "No build.sh found in '$DIR', using .NET build system"
    TEMP_SCRIPT="$(mktemp).sh"
    exec ./build/scripts/dotnet-bootstrap.cmd \
        --sln-dir ./build/src \
        ./build/src/Entry/Entry.csproj \
        -- "$DIR" "$@" \
        --output-script-path="$TEMP_SCRIPT"
fi
