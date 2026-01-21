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

Tasks follow the pattern `<target>:<action>` or `<target>:<action>:<variant>`:

```
cs:build          # Build C# (debug)
cs:build:release  # Build C# (release)
go:test           # Run Go tests
rs:check          # Check Rust code
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
| `<lang>:build` | Build the package |
| `<lang>:build:release` | Build in release mode (cs, rs only) |
| `<lang>:test` | Run tests |
| `<lang>:check` | Run linting and formatting checks |
| `<lang>:check:fix` | Auto-fix formatting issues |
| `<lang>:clean` | Clean build artifacts |
| `<lang>:restore` | Restore/install dependencies |
| `<lang>:demo` | Run demo examples |
| `<lang>:ci` | Run full CI pipeline |

Additional tasks:
- `cs:pack`, `kt:pack`, `rs:pack`, `ts:pack` - Create distribution packages
- `go:bench`, `rs:bench` - Run benchmarks
- `go:coverage`, `ts:coverage` - Run tests with coverage
- `r:doc`, `rs:doc` - Build documentation
- `cs:gen`, `cs:sim` - Generate tests / run simulations

### Auxiliary Tasks

| Task | Description |
|------|-------------|
| `gen:build` | Generate content (draft mode) |
| `gen:build:release` | Generate content (release mode) |
| `img:build` | Generate images |
| `img:logo` | Generate logo |
| `pdf:build` | Build PDF manual (draft) |
| `pdf:build:release` | Build PDF manual (release) |
| `web:restore` | Download Hugo and Tailwind |
| `web:build` | Build website (draft) |
| `web:build:release` | Build website (release) |
| `web:serve` | Start Hugo dev server |

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
- Propagated to language manifests via `gen:build`
- Release process: `mise run release <version> [--push]`

### Manual Generation Pipeline

1. `img:build` - Generate images
2. `gen:build` - Generate auxiliary files
3. `pdf:build` - Generate PDF manual
4. `web:build` - Build website

## Development Workflows

### Local Development

```bash
# Install mise (if not installed)
# See: https://mise.jdx.dev/getting-started.html

# Build and test a specific language
mise run rs:build
mise run rs:test

# Run all checks before committing
mise run rs:check
```

### CI Pipeline

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs:
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
3. Run `mise run <lang>:ci` to verify
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
- Website builds require Hugo and Tailwind (auto-downloaded via `web:restore`)
