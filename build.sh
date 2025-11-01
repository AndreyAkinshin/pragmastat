#!/bin/bash

# man: -*- nroff -*-

: <<'MANPAGE'
.TH PRAGMASTAT-BUILD 1 "November 2025" "3.1" "Pragmastat Build System"
.SH NAME
pragmastat build.sh \- build dispatcher for Pragmastat multi-language statistical library
.SH SYNOPSIS
.B ./build.sh
.I <lang>
.I <command>
[\fIargs\fR] [\fB--docker\fR]
.br
.B ./build.sh
.I <aux>
[\fIcommand\fR] [\fIargs\fR] [\fB--docker\fR]
.br
.B ./build.sh
.I <meta>
[\fIargs\fR] [\fB--docker\fR]
.br
.B ./build.sh
.B --man
.SH DESCRIPTION
.B build.sh
is the main build dispatcher for the Pragmastat project, a multi-language
statistical library. It provides a unified interface to build, test, and manage
implementations across 7 programming languages (C#, Go, Kotlin, Python, R, Rust,
TypeScript) plus auxiliary tools for documentation, images, and web content.
.PP
The script supports both native and Docker-based builds, making it easy to
build without installing language-specific toolchains.
.SH LANGUAGE COMMANDS
.TP
.B cs
C# (.NET) implementation
.TP
.B go
Go implementation
.TP
.B kt
Kotlin (JVM) implementation
.TP
.B py
Python implementation
.TP
.B r
R implementation
.TP
.B rs
Rust implementation
.TP
.B ts
TypeScript (npm) implementation
.SH AUXILIARY COMMANDS
.TP
.B gen
Content and auxiliary files generation
.TP
.B img
Image generation (plots, diagrams, logo)
.TP
.B pdf
PDF manual generation
.TP
.B web
Online manual/website (Hugo-based)
.SH META COMMANDS
.TP
.BI "all " "[--release] [--docker]"
Build all projects (language implementations and auxiliary tools)
.TP
.BI "ci " "[--release] [--docker]"
Run full CI build pipeline, replicating GitHub Actions workflow
.TP
.BI "test " "[--docker]"
Run tests for all language implementations
.TP
.BI "clean " "[--docker]"
Clean build artifacts for all projects
.TP
.BI "release " "<version> [--push]"
Create a new release version
.TP
.B docker-build
Build all Docker images used for containerized builds
.TP
.B docker-clean
Remove all Docker containers, images, and volumes
.SH OPTIONS
.TP
.BR -h ", " --help
Display brief help message with command overview.
.TP
.B --man
Display this detailed manual page.
.TP
.B --docker
Run builds in Docker containers instead of using local toolchains.
Can appear anywhere in the argument list.
.TP
.B --release
Build in release mode (optimized, production-ready). Supported by: pdf, web, cs, rs.
.TP
.B --push
For release command only: push the release to upstream repository with tags.
.SH ENVIRONMENT VARIABLES
.TP
.B PRAGMASTAT_DOCKER
If set to "1" or "true", automatically enables Docker mode for all builds.
Equivalent to passing --docker flag.
.SH EXAMPLES
.TP
Build C# implementation locally:
.B ./build.sh cs build
.TP
Build C# implementation in Docker:
.B ./build.sh cs build --docker
.TP
Build C# using environment variable:
.B PRAGMASTAT_DOCKER=1 ./build.sh cs build
.TP
Run tests for all implementations:
.B ./build.sh test
.TP
Build all projects in release mode using Docker:
.B ./build.sh all --release --docker
.TP
Run full CI pipeline:
.B ./build.sh ci --docker
.TP
Create a release locally:
.B ./build.sh release 3.1.31
.TP
Create and push a release:
.B ./build.sh release 3.1.31 --push
.TP
Build Docker images:
.B ./build.sh docker-build
.SH PROJECT-SPECIFIC COMMANDS
Each language and auxiliary project has its own build.sh with specific commands.
Common commands include:
.TP
.B build
Compile/build the project
.TP
.B test
Run unit tests
.TP
.B check
Run linters, formatters, or validation
.TP
.B clean
Remove build artifacts
.TP
.B init
Initialize project dependencies (if needed)
.PP
Run
.B ./build.sh <project> --help
for project-specific documentation (if available).
.SH DOCKER WORKFLOW
The Docker workflow uses docker-compose.yml to define services for each component.
When --docker flag is used or PRAGMASTAT_DOCKER is set:
.IP 1. 4
Docker images are automatically built if not present
.IP 2. 4
Commands execute inside containers with appropriate toolchains
.IP 3. 4
Build artifacts are persisted via volume mounts
.IP 4. 4
No local installation of language toolchains required
.SH RELEASE PROCESS
The release command automates version management:
.IP 1. 4
Writes version to manual/version.txt
.IP 2. 4
Runs ./build.sh gen to regenerate version-dependent files
.IP 3. 4
Creates git commit "set version <version>"
.IP 4. 4
Moves main branch to HEAD
.IP 5. 4
With --push: creates tags v<version> and go/v<version>, pushes to upstream
.SH CI BUILD
The ci command replicates the GitHub Actions workflow locally:
.IP 1. 4
Builds images (img)
.IP 2. 4
Builds PDF manual (gen + pdf)
.IP 3. 4
Builds website (gen + web)
.IP 4. 4
Builds and tests R implementation
.IP 5. 4
Builds, tests, and packages C# implementation
.IP 6. 4
Builds, tests, and packages Python implementation
.IP 7. 4
Builds, tests, and packages Rust implementation
.IP 8. 4
Builds, tests, and packages TypeScript implementation
.IP 9. 4
Builds and tests Go implementation
.IP 10. 4
Builds and tests Kotlin implementation
.SH EXIT STATUS
.TP
.B 0
Success
.TP
.B 1
Error occurred (build failure, missing dependencies, git error, etc.)
.SH FILES
.TP
.I ./build.sh
Main build dispatcher
.TP
.I ./<project>/build.sh
Project-specific build scripts
.TP
.I ./docker-compose.yml
Docker services configuration
.TP
.I ./manual/version.txt
Current version file
.SH SEE ALSO
.BR docker (1),
.BR docker-compose (1),
.BR dotnet (1),
.BR cargo (1),
.BR npm (1),
.BR go (1),
.BR Rscript (1)
.SH BUGS
Report bugs at: https://github.com/AndreyAkinshin/pragmastat/issues
.SH AUTHOR
Andrey Akinshin
.SH COPYRIGHT
Copyright (c) Andrey Akinshin. Licensed under MIT License.
MANPAGE

# Main build dispatcher script for Pragmastat

set -e

# Change to script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

# Docker mode flag (can be set via environment variable PRAGMASTAT_DOCKER)
USE_DOCKER=false
if [ "${PRAGMASTAT_DOCKER:-}" = "1" ] || [ "${PRAGMASTAT_DOCKER:-}" = "true" ]; then
    USE_DOCKER=true
fi

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

# Function to check if Docker is available
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

    if ! docker compose version &> /dev/null; then
        print_error "Docker Compose is not available"
        print_error "Please install Docker Compose (v2+): https://docs.docker.com/compose/install/"
        exit 1
    fi
}

# Function to ensure Docker image is built
ensure_docker_image() {
    local service="$1"
    
    if ! docker compose images "$service" 2>/dev/null | grep -q "$service"; then
        print_info "Building Docker image for: $service"
        if ! docker compose build "$service"; then
            print_error "Failed to build Docker image for: $service"
            exit 1
        fi
    fi
}

# Function to run command in Docker container
run_in_docker() {
    local service="$1"
    shift
    local cmd="$@"
    
    ensure_docker_image "$service"
    
    # Determine the shell to use based on the service
    local shell="bash"
    if [ "$service" = "pdf" ]; then
        shell="sh"
    fi
    
    # Run the command in the container with user mapping to prevent root ownership
    docker compose run --rm --user "$(id -u):$(id -g)" "$service" "$shell" -c "$cmd"
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

    # Language projects (for test command)
    local lang_projects=(
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

    # Select appropriate projects based on command
    local projects_to_process=("${all_projects[@]}")
    if [ "$command" == "test" ]; then
        projects_to_process=("${lang_projects[@]}")
    fi

    for project in "${projects_to_process[@]}"; do
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

        # Run the command (in Docker if enabled)
        local build_cmd
        if [ "$supports_release" == "true" ] && [ "$release_flag" == "--release" ]; then
            build_cmd="./build.sh $command --release"
        else
            build_cmd="./build.sh $command"
        fi

        if [ "$USE_DOCKER" == "true" ]; then
            if run_in_docker "$project" "$build_cmd"; then
                succeeded_projects+=("$project")
            else
                failed_projects+=("$project")
                print_error "✗ Failed to run '$command' for: $project"
            fi
        else
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

    # Helper function to run step (with Docker support)
    run_step_impl() {
        local step_name="$1"
        local step_command="$2"
        
        if [ "$USE_DOCKER" == "true" ]; then
            # Extract service name from step name
            local service=$(echo "$step_name" | sed 's/.*-//')
            run_step "$step_name" "run_in_docker $service \"$step_command\""
        else
            run_step "$step_name" "$step_command"
        fi
    }

    # Build img
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "build-img" "run_in_docker img './build.sh build'" || exit 1
    else
        run_step "build-img" "./img/build.sh build" || exit 1
    fi

    # Build pdf (depends on img)
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "gen-for-pdf" "run_in_docker gen './build.sh gen $release_flag'" || exit 1
        run_step "build-pdf" "run_in_docker pdf './build.sh build $release_flag'" || exit 1
    else
        run_step "gen-for-pdf" "./build.sh gen $release_flag" || exit 1
        run_step "build-pdf" "./pdf/build.sh build $release_flag" || exit 1
    fi

    # Build web (depends on img and pdf)
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "gen-for-web" "run_in_docker gen './build.sh gen $release_flag'" || exit 1
        run_step "init-web" "run_in_docker web './build.sh init'" || exit 1
        run_step "build-web" "run_in_docker web './build.sh build $release_flag'" || exit 1
    else
        run_step "gen-for-web" "./build.sh gen $release_flag" || exit 1
        run_step "init-web" "./web/build.sh init" || exit 1
        run_step "setup-tailwind-wrapper" "./web/setup-tailwind-wrapper.sh" || exit 1
        run_step "build-web" "./web/build.sh build $release_flag" || exit 1
    fi

    # Build r
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "check-r" "run_in_docker r './build.sh check'" || exit 1
        run_step "build-r" "run_in_docker r './build.sh build'" || exit 1
        run_step "test-r" "run_in_docker r './build.sh test'" || exit 1
    else
        run_step "check-r" "./r/build.sh check" || exit 1
        run_step "build-r" "./r/build.sh build" || exit 1
        run_step "test-r" "./r/build.sh test" || exit 1
    fi

    # Build cs
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "build-cs" "run_in_docker cs './build.sh build $release_flag'" || exit 1
        run_step "test-cs" "run_in_docker cs './build.sh test'" || exit 1
        run_step "pack-cs" "run_in_docker cs 'CI=true ./build.sh pack $release_flag'" || exit 1
    else
        run_step "build-cs" "./cs/build.sh build $release_flag" || exit 1
        run_step "test-cs" "./cs/build.sh test" || exit 1
        run_step "pack-cs" "CI=true ./cs/build.sh pack $release_flag" || exit 1
    fi

    # Build py
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "test-py" "run_in_docker py './build.sh test'" || exit 1
        run_step "build-py" "run_in_docker py './build.sh build'" || exit 1
        run_step "check-py" "run_in_docker py './build.sh check'" || exit 1
    else
        run_step "test-py" "./py/build.sh test" || exit 1
        run_step "build-py" "./py/build.sh build" || exit 1
        run_step "check-py" "./py/build.sh check" || exit 1
    fi

    # Build rs
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "check-rs" "run_in_docker rs './build.sh check'" || exit 1
        run_step "test-rs" "run_in_docker rs './build.sh test'" || exit 1
        run_step "build-rs" "run_in_docker rs './build.sh build $release_flag'" || exit 1
        run_step "package-rs" "run_in_docker rs 'cd pragmastat && cargo package --verbose'" || exit 1
    else
        run_step "check-rs" "./rs/build.sh check" || exit 1
        run_step "test-rs" "./rs/build.sh test" || exit 1
        run_step "build-rs" "./rs/build.sh build $release_flag" || exit 1
        run_step "package-rs" "cd rs/pragmastat && cargo package --verbose" || exit 1
    fi

    # Build ts
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "check-ts" "run_in_docker ts './build.sh check'" || exit 1
        run_step "test-ts" "run_in_docker ts './build.sh test'" || exit 1
        run_step "build-ts" "run_in_docker ts './build.sh build'" || exit 1
        run_step "pack-ts" "run_in_docker ts 'npm pack'" || exit 1
    else
        run_step "install-ts" "cd ts && npm ci" || exit 1
        run_step "check-ts" "./ts/build.sh check" || exit 1
        run_step "test-ts" "./ts/build.sh test" || exit 1
        run_step "build-ts" "./ts/build.sh build" || exit 1
        run_step "pack-ts" "cd ts && npm pack" || exit 1
    fi

    # Build go
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "deps-go" "run_in_docker go './build.sh deps'" || exit 1
        run_step "test-go" "run_in_docker go './build.sh test-verbose'" || exit 1
        run_step "build-go" "run_in_docker go './build.sh build'" || exit 1
    else
        run_step "deps-go" "./go/build.sh deps" || exit 1
        run_step "test-go" "./go/build.sh test-verbose" || exit 1
        run_step "build-go" "./go/build.sh build" || exit 1
    fi

    # Build kt
    if [ "$USE_DOCKER" == "true" ]; then
        run_step "build-kt" "run_in_docker kt './gradlew build --info --stacktrace'" || exit 1
    else
        run_step "build-kt" "cd kt && ./gradlew build --info --stacktrace" || exit 1
    fi

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
        
        # Collect artifacts
        echo ""
        print_status "════════════════════════════════════════"
        print_status "Collecting Artifacts"
        print_status "════════════════════════════════════════"
        
        # Read version
        if [ ! -f "manual/version.txt" ]; then
            print_error "Version file not found: manual/version.txt"
            exit 1
        fi
        VERSION=$(cat manual/version.txt | tr -d '\n')
        print_status "Version: $VERSION"
        
        # Clean and create artifacts directory
        print_status "Creating artifacts directory..."
        rm -rf artifacts
        mkdir -p artifacts/{web,pdf,cs,py,rs,ts,go,r,kt}
        
        # Copy web artifacts
        if [ -d "web/public" ]; then
            print_status "Copying web artifacts..."
            cp -r web/public/* artifacts/web/
        else
            print_warning "web/public directory not found"
        fi
        
        # Copy pdf artifacts
        if ls pdf/*.pdf 1> /dev/null 2>&1; then
            print_status "Copying pdf artifacts..."
            cp pdf/*.pdf artifacts/pdf/
        else
            print_warning "No PDF files found in pdf/"
        fi
        
        # Copy cs artifacts
        if [ -d "cs/artifacts" ]; then
            print_status "Copying cs artifacts..."
            cp -r cs/artifacts/* artifacts/cs/
        else
            print_warning "cs/artifacts directory not found"
        fi
        
        # Copy py artifacts
        if [ -d "py/dist" ]; then
            print_status "Copying py artifacts..."
            cp -r py/dist/* artifacts/py/
        else
            print_warning "py/dist directory not found"
        fi
        
        # Copy rs artifacts
        if ls rs/pragmastat/target/package/*.crate 1> /dev/null 2>&1; then
            print_status "Copying rs artifacts..."
            cp rs/pragmastat/target/package/*.crate artifacts/rs/
        else
            print_warning "No Rust crate files found in rs/pragmastat/target/package/"
        fi
        
        # Copy ts artifacts
        if ls ts/*.tgz 1> /dev/null 2>&1; then
            print_status "Copying ts artifacts..."
            cp ts/*.tgz artifacts/ts/
        else
            print_warning "No TypeScript tarball files found in ts/"
        fi
        
        # Copy go artifacts (entire go directory)
        if [ -d "go" ]; then
            print_status "Copying go artifacts..."
            cp -r go/* artifacts/go/
        else
            print_warning "go directory not found"
        fi
        
        # Copy r artifacts
        if ls r/*.tar.gz 1> /dev/null 2>&1; then
            print_status "Copying r artifacts..."
            cp r/*.tar.gz artifacts/r/
        else
            print_warning "No R package files found in r/"
        fi
        
        # Copy kt artifacts
        if [ -d "kt/build/libs" ]; then
            print_status "Copying kt artifacts (libs)..."
            mkdir -p artifacts/kt/libs
            cp -r kt/build/libs/* artifacts/kt/libs/
        else
            print_warning "kt/build/libs directory not found"
        fi
        if [ -d "kt/build/staging-deploy" ]; then
            print_status "Copying kt artifacts (staging-deploy)..."
            mkdir -p artifacts/kt/staging-deploy
            cp -r kt/build/staging-deploy/* artifacts/kt/staging-deploy/
        fi
        
        # Add version suffix to documentation files
        print_status "Creating versioned documentation files..."
        if [ -f "artifacts/web/pragmastat.md" ]; then
            cp artifacts/web/pragmastat.md "artifacts/pragmastat-v${VERSION}.md"
            print_status "  ✓ pragmastat-v${VERSION}.md"
        fi
        if [ -f "artifacts/web/pragmastat.pdf" ]; then
            cp artifacts/web/pragmastat.pdf "artifacts/pragmastat-v${VERSION}.pdf"
            print_status "  ✓ pragmastat-v${VERSION}.pdf"
        fi
        
        echo ""
        print_status "════════════════════════════════════════"
        print_status "Artifacts Summary"
        print_status "════════════════════════════════════════"
        print_status "All artifacts have been collected in: ./artifacts/"
        echo ""
        print_status "Available artifacts:"
        for dir in artifacts/*/; do
            if [ -d "$dir" ]; then
                dir_name=$(basename "$dir")
                dir_size=$(du -sh "$dir" 2>/dev/null | cut -f1)
                print_status "  - artifacts/${dir_name}/ (${dir_size})"
            fi
        done
        if [ -f "artifacts/pragmastat-v${VERSION}.md" ]; then
            file_size=$(du -sh "artifacts/pragmastat-v${VERSION}.md" 2>/dev/null | cut -f1)
            print_status "  - artifacts/pragmastat-v${VERSION}.md (${file_size})"
        fi
        if [ -f "artifacts/pragmastat-v${VERSION}.pdf" ]; then
            file_size=$(du -sh "artifacts/pragmastat-v${VERSION}.pdf" 2>/dev/null | cut -f1)
            print_status "  - artifacts/pragmastat-v${VERSION}.pdf (${file_size})"
        fi
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
    echo -e "${BOLD}Usage:${RESET} $0 ${HIGHLIGHT}<lang>${RESET} ${ARG}<command> [args] [--docker]${RESET}"
    echo -e "       $0 ${HIGHLIGHT}<aux>${RESET}  ${ARG}[command] [args] [--docker]${RESET}"
    echo -e "       $0 ${HIGHLIGHT}<meta>${RESET} ${ARG}[args] [--docker]${RESET}"
    echo -e "       $0 ${ARG}-h | --help | --man${RESET}"
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
    echo -e "  ${HIGHLIGHT}all${RESET} ${ARG}[--release] [--docker]${RESET} ${DIM}# Build all projects${RESET}"
    echo -e "  ${HIGHLIGHT}ci${RESET} ${ARG}[--release] [--docker]${RESET}  ${DIM}# Run full CI build (replicates GitHub Actions)${RESET}"
    echo -e "  ${HIGHLIGHT}test${RESET} ${ARG}[--docker]${RESET}            ${DIM}# Run tests for all projects${RESET}"
    echo -e "  ${HIGHLIGHT}clean${RESET} ${ARG}[--docker]${RESET}           ${DIM}# Clean all projects${RESET}"
    echo -e "  ${HIGHLIGHT}release${RESET} ${ARG}<ver> [--push]${RESET}     ${DIM}# Create release version${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Docker support:${RESET}"
    echo -e "  ${ARG}--docker${RESET}                   ${DIM}# Run builds in Docker containers${RESET}"
    echo -e "  ${ARG}PRAGMASTAT_DOCKER=1${RESET}        ${DIM}# Environment variable to auto-enable Docker mode${RESET}"
    echo -e "  ${HIGHLIGHT}docker-build${RESET}               ${DIM}# Build all Docker images${RESET}"
    echo -e "  ${HIGHLIGHT}docker-clean${RESET}               ${DIM}# Remove all Docker containers and images${RESET}"
    echo ""
    echo -e "${HEADER}${BOLD}Help and documentation:${RESET}"
    echo -e "  ${ARG}-h, --help${RESET}                 ${DIM}# Show this help message${RESET}"
    echo -e "  ${ARG}--man${RESET}                      ${DIM}# Show detailed manual page${RESET}"
}

# Handle help and man flags
if [[ "${1:-}" == "-h" ]] || [[ "${1:-}" == "--help" ]]; then
  show_help
  exit 0
fi

if [[ "${1:-}" == "--man" ]]; then
  # Extract and display the embedded man page
  # Try available formatters in order: mandoc, groff, nroff
  if command -v mandoc &> /dev/null; then
    sed -n '/^: <<'\''MANPAGE'\''/,/^MANPAGE$/p' "$0" \
      | sed '1d;$d' \
      | mandoc -T utf8 \
      | "${PAGER:-less}"
  elif command -v groff &> /dev/null; then
    sed -n '/^: <<'\''MANPAGE'\''/,/^MANPAGE$/p' "$0" \
      | sed '1d;$d' \
      | groff -mandoc -Tutf8 \
      | "${PAGER:-less}"
  elif command -v nroff &> /dev/null; then
    sed -n '/^: <<'\''MANPAGE'\''/,/^MANPAGE$/p' "$0" \
      | sed '1d;$d' \
      | nroff -mandoc \
      | "${PAGER:-less}"
  else
    echo "Error: No man page formatter found (mandoc, groff, or nroff required)" >&2
    exit 1
  fi
  exit 0
fi

# Check if no arguments provided
if [ $# -eq 0 ]; then
    show_help
    exit 1
fi

# Parse --docker flag from any position
ARGS=()
for arg in "$@"; do
    if [ "$arg" == "--docker" ]; then
        USE_DOCKER=true
        check_docker
    else
        ARGS+=("$arg")
    fi
done

# Reset positional parameters without --docker flag
set -- "${ARGS[@]}"

# Extract directory and remaining arguments
DIR="$1"
shift

# Check for meta-commands
case "$DIR" in
    docker-build)
        print_info "Building all Docker images..."
        docker compose build
        exit 0
        ;;
    docker-clean)
        print_info "Cleaning Docker containers and images..."
        docker compose down --rmi all --volumes --remove-orphans
        exit 0
        ;;
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
    # Dispatch to the ecosystem-specific build script (in Docker if enabled)
    if [ "$USE_DOCKER" == "true" ]; then
        run_in_docker "$DIR" "./build.sh $@"
    else
        exec "./$DIR/build.sh" "$@"
    fi
else
    print_error "Unknown command or directory: $DIR"
    echo ""
    show_help
    exit 1
fi
