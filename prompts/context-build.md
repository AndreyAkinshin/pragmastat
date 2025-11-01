# Pragmastat Build Infrastructure

## Overview

Pragmastat is a multi-language statistical library
  with **7 language implementations** (C#, Go, Kotlin, Python, R, Rust, TypeScript)
  plus **4 auxiliary tools** (gen, img, pdf, web).
The build system provides a unified interface with support for both native and Docker-based builds.

## Architecture

```
build.sh (root dispatcher)
├── Languages: cs, go, kt, py, r, rs, ts → <lang>/build.sh
├── Tools: gen, img, pdf, web → <tool>/build.sh
└── Meta: all, ci, test, demo, clean, release, docker-*
```

## Usage

```
Usage: ./build.sh <lang> <command> [args] [--docker]
       ./build.sh <aux>  [command] [args] [--docker]
       ./build.sh <meta> [args] [--docker]
       ./build.sh -h | --help | --man

Pragmastat Build Dispatcher

Language commands:
  cs   # C# (.NET)
  go   # Go
  kt   # Kotlin (JVM)
  py   # Python
  r    # R
  rs   # Rust
  ts   # TypeScript (npm)

Auxiliary commands:
  gen  # Content and auxiliary files generation
  img  # Image generation
  pdf  # PDF manual generation
  web  # Online manual/website (Hugo)

Meta commands:
  all [--release] [--docker] # Build all projects
  ci [--release] [--docker]  # Run full CI build (replicates GitHub Actions)
  test [--docker]            # Run tests for all projects
  demo [--docker]            # Run demos for all language projects
  clean [--docker]           # Clean all projects
  release <ver> [--push]     # Create release version

Docker support:
  --docker                   # Run builds in Docker containers
  PRAGMASTAT_DOCKER=1        # Environment variable to auto-enable Docker mode
  docker-build               # Build all Docker images
  docker-clean               # Remove all Docker containers and images

Help and documentation:
  -h, --help                 # Show this help message
  --man                      # Show detailed manual page
```

## Language-Specific Commands

All language scripts support: `build`, `test`, `demo`, `clean`, `all`, `-h|--help`

**Differences:**
- **cs, rs**: Support `--release` flag for optimized builds
- **cs**: Has `generate` (reference tests), `pack` (NuGet), `format`, `lint`
- **py**: Has `dev` (editable install), `check` (twine), auto-creates venv
- **r**: Copies test data from `../tests`, has `check`, `check-full`, `docs`
- **go**: Has `deps`, `tidy`, `coverage`, `bench`, `lint` (optional golangci-lint)
- **rs**: Works in `pragmastat/` subdirectory, has `check` (clippy+fmt), `bench`, `doc`
- **kt**: Uses Gradle wrapper, has `jar`
- **ts**: Uses npm scripts, has `lint`, `check`, `format`, `coverage`, `watch`

## Auxiliary Tools

**gen**: Generates version-dependent files (Markdown, configs). Run after version changes.
- Usage: `./build.sh gen [--release]`

**img**: Generates plots/diagrams using Python. Auto-manages venv.
- Usage: `./build.sh img build`

**pdf**: Generates PDF manual using Pandoc/LaTeX. Reads `manual/version.txt`.
- Usage: `./build.sh pdf [--release]`
- Requires: gen, img

**web**: Hugo-based website. Run `init` first (downloads Hugo/Tailwind to `.bin/`).
- Usage: `./build.sh web init` (one-time), `./build.sh web build [--release]`
- Serve: `./build.sh web serve` (port 1729)
- Requires: gen, img, pdf

## CI Build

`./build.sh ci [--release]` runs full build pipeline:
1. img → gen → pdf → web (documentation)
2. r, cs, py, rs, ts, go, kt (all languages)

Collects artifacts to `./artifacts/` with versioned outputs.

## Release Process

`./build.sh release <version> [--push]`

Steps:
1. Writes version to `manual/version.txt`
2. Runs `./build.sh gen`
3. Creates commit "set version <version>"
4. Moves `main` branch to HEAD
5. With `--push`: Creates tags `v<version>` and `go/v<version>`, pushes to upstream

## Docker Mode

Enable with `--docker` flag or `PRAGMASTAT_DOCKER=1`. Runs commands in containers with user mapping to prevent root ownership. No local toolchains required.

## Key Files & Variables

- `./build.sh` - Main dispatcher
- `./docker-compose.yml` - Docker services
- `./manual/version.txt` - Current version
- `PRAGMASTAT_DOCKER` - Auto-enable Docker mode

## Docker User Mapping

**Python/R special handling:**
- **py (Docker)**: `PYTHONUSERBASE=/workspace/py/.pip-packages`
- **r (Docker)**: `R_LIBS_USER=/workspace/r/.r-packages`
- Ensures writable package installs without root

## Tips for LLM Agents

1. **Start from root**: `./build.sh <project> <command>`
2. **Check help**: Every script has `-h`
3. **Dependencies**: gen → img → pdf → web
4. **CI test**: `./build.sh ci` validates everything
5. **Web init**: Run `./build.sh web init` once before building website
