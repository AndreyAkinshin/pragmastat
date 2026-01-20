# Pragmastat Development Guide

This guide is for developers and AI agents working on the Pragmastat codebase.

## Project Overview

Pragmastat is a multi-language statistical library with implementations in 7 programming languages:
- C# (.NET)
- Go
- Kotlin (JVM)
- Python
- R
- Rust
- TypeScript

Plus auxiliary tools for content generation, images, PDF manual, and website.

## Build System

All builds are managed via [mise](https://mise.jdx.dev/). Never run raw commands directly.

### Task Naming Convention

Tasks follow the pattern `<action>:<language>` or `<action>:<qualifier>:<target>`:

```
build:cs          # Build C# (debug)
build:release:cs  # Build C# (release)
test:go           # Run Go tests
check:rs          # Check Rust code
```

### Common Tasks

| Task | Description |
|------|-------------|
| `mise run build` | Build all language implementations |
| `mise run test` | Test all language implementations |
| `mise run check` | Run all static analysis |
| `mise run clean` | Clean all build artifacts |
| `mise run demo` | Run demos for all implementations |
| `mise run ci` | Run full CI pipeline for all languages |

### Per-Language Tasks

Each language (cs, go, kt, py, r, rs, ts) supports:

| Task | Description |
|------|-------------|
| `build:<lang>` | Build the package |
| `build:release:<lang>` | Build in release mode (cs, rs only) |
| `test:<lang>` | Run tests |
| `check:<lang>` | Run linting and formatting checks |
| `check:fix:<lang>` | Auto-fix formatting issues |
| `clean:<lang>` | Clean build artifacts |
| `restore:<lang>` | Restore/install dependencies |
| `demo:<lang>` | Run demo examples |
| `ci:<lang>` | Run full CI pipeline |

Additional tasks:
- `pack:cs`, `pack:kt`, `pack:rs`, `pack:ts` - Create distribution packages
- `bench:go`, `bench:rs` - Run benchmarks
- `coverage:go`, `coverage:ts` - Run tests with coverage
- `doc:r`, `doc:rs` - Build documentation
- `gen:cs`, `sim:cs` - Generate tests / run simulations

### Auxiliary Tasks

| Task | Description |
|------|-------------|
| `build:gen` | Generate content (draft mode) |
| `build:release:gen` | Generate content (release mode) |
| `build:img` | Generate images |
| `logo:img` | Generate logo |
| `build:pdf` | Build PDF manual (draft) |
| `build:release:pdf` | Build PDF manual (release) |
| `restore:web` | Download Hugo and Tailwind |
| `build:web` | Build website (draft) |
| `build:release:web` | Build website (release) |
| `serve:web` | Start Hugo dev server |

### Release Tasks

```bash
mise run release 3.2.5           # Create release locally
mise run release 3.2.5 --push    # Create and push release
```

### Docker Tasks

```bash
mise run docker:build    # Build all Docker images
mise run docker:clean    # Remove all Docker containers/images
```

## Repository Structure

```
pragmastat/
├── cs/                 # C# implementation
├── go/                 # Go implementation
├── kt/                 # Kotlin implementation
├── py/                 # Python implementation
├── r/                  # R implementation
├── rs/                 # Rust implementation
├── ts/                 # TypeScript implementation
├── gen/                # Content generation scripts
├── img/                # Image generation (plots, logo)
├── pdf/                # PDF manual (Pandoc/LaTeX)
├── web/                # Hugo-based website
├── manual/             # Manual source content
│   └── version.txt     # Version file (single source of truth)
├── tests/              # Cross-language test data (JSON)
├── sim/                # Simulation data
├── schema/             # JSON schemas
├── mise.toml           # Build configuration
└── docker-compose.yml  # Docker services
```

## Architecture

### Cross-Language Test Data

All implementations share test data in `tests/` as JSON files. Each language loads these via a `SharedTestData` pattern.

### Version Management

- Single version in `manual/version.txt`
- Propagated to language manifests via `build:gen`
- Release process: `mise run release <version> [--push]`

### Manual Generation Pipeline

1. `build:img` - Generate images
2. `build:gen` - Generate auxiliary files
3. `build:pdf` - Generate PDF manual
4. `build:web` - Build website

## Development Workflows

### Local Development

```bash
# Install mise (if not installed)
# See: https://mise.jdx.dev/getting-started.html

# Build and test a specific language
mise run build:rs
mise run test:rs

# Run all checks before committing
mise run check:rs
```

### CI Pipeline

The GitHub Actions workflow (`.github/workflows/build.yml`) runs:
1. `build-img` - Generate images
2. `build-pdf` - Generate PDF (requires images)
3. `build-web` - Build website (requires images and PDF)
4. `build-r` - Build/test R package (Docker container)
5. `build-cs` - Build/test C# package
6. `build-py` - Build/test Python package
7. `build-rs` - Build/test Rust crate
8. `build-ts` - Build/test TypeScript package
9. `build-go` - Build/test Go module
10. `build-kt` - Build/test Kotlin library

### Adding a New Feature

1. Implement in the appropriate language directory
2. Add tests (preferably cross-language in `tests/`)
3. Run `mise run ci:<lang>` to verify
4. Update documentation if needed

## Key Interfaces

Each language implementation exposes the same core statistical functions. See individual language READMEs for API details.

## Error Handling

- All builds use `set -e` behavior (fail on error)
- Test failures exit with non-zero status
- CI fails fast on any error

## Notes for AI Agents

- Always use `mise run <task>` instead of raw commands
- Check `mise tasks` for full task list
- R tasks run outside mise management (uses system R)
- PDF builds require LaTeX (handled via Docker in CI)
- Website builds require Hugo and Tailwind (auto-downloaded via `restore:web`)
