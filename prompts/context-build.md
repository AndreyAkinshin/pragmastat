# Pragmastat Build Infrastructure

## Overview

Pragmastat is a multi-language statistical library
  with **7 language implementations** (C#, Go, Kotlin, Python, R, Rust, TypeScript)
  plus **4 auxiliary tools** (docs, img, pdf, web).
The build system uses [mise](https://mise.jdx.dev/) as the task runner.

## Architecture

```
mise.toml (root)
├── Languages: cs, go, kt, py, r, rs, ts
├── Auxiliary: docs, img, pdf, web
└── Aggregate: build, test, check, clean, demo, ci
```

## Usage

```bash
# List all tasks
mise tasks

# Run a specific task
mise run <task>

# Examples
mise run rs:build          # Build Rust
mise run py:test           # Test Python
mise run go:check          # Check Go code
```

## Task Naming Convention

Tasks follow the pattern `<target>:<action>` or `<target>:<action>:<variant>`:

```
cs:build          # Build C# (debug)
cs:build:release  # Build C# (release)
go:test           # Run Go tests
rs:check          # Check Rust code
```

## Language-Specific Tasks

All languages support: `build`, `test`, `check`, `check:fix`, `clean`, `demo`, `ci`

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

**Additional tasks:**
- `cs:pack`, `kt:pack`, `rs:pack`, `ts:pack` - Create distribution packages
- `go:bench`, `rs:bench` - Run benchmarks
- `go:coverage`, `ts:coverage` - Run tests with coverage
- `r:doc`, `rs:doc` - Build documentation
- `cs:gen`, `cs:sim` - Generate tests / run simulations

## Auxiliary Tools

**docs**: Syncs versioned manifests and templated docs.
- `mise run docs:version` - Sync versions
- `mise run docs:templates` - Sync templated docs and READMEs
- `mise run docs:sync` - Sync versions + templates

**img**: Generates plots/diagrams using Python. Auto-manages venv.
- `mise run img:build` - Generate images
- `mise run img:logo` - Generate logo

**pdf**: Generates PDF manual using Typst. Reads `manual/version.typ`.
- `mise run pdf:build` - Draft mode
- `mise run pdf:build:release` - Release mode
- Requires: docs, img

**web**: Astro-based website.
- `mise run web:restore` - Install dependencies (one-time)
- `mise run web:build` - Build draft site
- `mise run web:build:release` - Build release site
- `mise run web:serve` - Start dev server (port 1729)
- Requires: docs, img, pdf

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
1. Writes version to `manual/version.typ`
2. Runs `mise run docs:sync`
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
./docker-run.sh cs mise run cs:build
./docker-run.sh py mise run py:test
```

## Key Files

- `mise.toml` - Task definitions
- `docker-compose.yml` - Docker services
- `manual/version.typ` - Current version

## Tips for LLM Agents

1. **Use mise**: Always run `mise run <task>` instead of raw commands
2. **Check tasks**: Run `mise tasks` to list all available tasks
3. **Dependencies**: docs → img → pdf → web
4. **CI test**: `mise run ci` validates all languages
5. **Web init**: Run `mise run web:restore` once before building website
