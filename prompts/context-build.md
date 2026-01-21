# Pragmastat Build Infrastructure

## Overview

Pragmastat is a multi-language statistical library
  with **7 language implementations** (C#, Go, Kotlin, Python, R, Rust, TypeScript)
  plus **4 auxiliary tools** (gen, img, pdf, web).
The build system uses [mise](https://mise.jdx.dev/) as the task runner.

## Architecture

```
mise.toml (root)
├── Languages: cs, go, kt, py, r, rs, ts
├── Auxiliary: gen, img, pdf, web
└── Aggregate: build, test, check, clean, demo, ci
```

## Usage

```bash
# List all tasks
mise tasks

# Run a specific task
mise run <task>

# Examples
mise run build:rs          # Build Rust
mise run test:py           # Test Python
mise run check:go          # Check Go code
```

## Task Naming Convention

Tasks follow the pattern `<action>:<target>` or `<action>:<target>:<variant>`:

```
build:cs          # Build C# (debug)
build:cs:release  # Build C# (release)
test:go           # Run Go tests
check:rs          # Check Rust code
```

## Language-Specific Tasks

All languages support: `build`, `test`, `check`, `check:fix`, `clean`, `demo`, `ci`

| Task | Description |
|------|-------------|
| `build:<lang>` | Build the package |
| `build:<lang>:release` | Build in release mode (cs, rs only) |
| `test:<lang>` | Run tests |
| `check:<lang>` | Run linting and formatting checks |
| `check:fix:<lang>` | Auto-fix formatting issues |
| `clean:<lang>` | Clean build artifacts |
| `restore:<lang>` | Restore/install dependencies |
| `demo:<lang>` | Run demo examples |
| `ci:<lang>` | Run full CI pipeline |

**Additional tasks:**
- `pack:cs`, `pack:kt`, `pack:rs`, `pack:ts` - Create distribution packages
- `bench:go`, `bench:rs` - Run benchmarks
- `coverage:go`, `coverage:ts` - Run tests with coverage
- `doc:r`, `doc:rs` - Build documentation
- `gen:cs`, `sim:cs` - Generate tests / run simulations

## Auxiliary Tools

**gen**: Generates version-dependent files (Markdown, configs). Run after version changes.
- `mise run build:gen` - Draft mode
- `mise run build:gen:release` - Release mode

**img**: Generates plots/diagrams using Python. Auto-manages venv.
- `mise run build:img` - Generate images
- `mise run logo:img` - Generate logo

**pdf**: Generates PDF manual using Pandoc/LaTeX. Reads `manual/version.txt`.
- `mise run build:pdf` - Draft mode
- `mise run build:pdf:release` - Release mode
- Requires: gen, img

**web**: Hugo-based website.
- `mise run restore:web` - Download Hugo/Tailwind (one-time)
- `mise run build:web` - Build draft site
- `mise run build:web:release` - Build release site
- `mise run serve:web` - Start dev server (port 1729)
- Requires: gen, img, pdf

## Aggregate Tasks

```bash
mise run build   # Build all language implementations
mise run test    # Test all language implementations
mise run check   # Check all language implementations
mise run clean   # Clean all build artifacts
mise run demo    # Run demos for all implementations
mise run ci      # Run CI pipeline for all languages
```

## Release Process

```bash
mise run release 3.2.5           # Create release locally
mise run release 3.2.5 --push    # Create and push release
```

Steps:
1. Writes version to `manual/version.txt`
2. Runs `mise run build:gen`
3. Creates commit "set version <version>"
4. Moves `main` branch to HEAD
5. With `--push`: Creates tags `v<version>` and `go/v<version>`, pushes to upstream

## Docker Support

Docker can be used via `docker-compose.yml` for containerized builds:

```bash
mise run docker:build    # Build all Docker images
mise run docker:clean    # Remove all Docker containers/images
```

Or use `docker-run.sh` for individual container execution:

```bash
./docker-run.sh cs mise run build:cs
./docker-run.sh py mise run test:py
```

## Key Files

- `mise.toml` - Task definitions
- `docker-compose.yml` - Docker services
- `manual/version.txt` - Current version

## Tips for LLM Agents

1. **Use mise**: Always run `mise run <task>` instead of raw commands
2. **Check tasks**: Run `mise tasks` to list all available tasks
3. **Dependencies**: gen → img → pdf → web
4. **CI test**: `mise run ci` validates all languages
5. **Web init**: Run `mise run restore:web` once before building website
