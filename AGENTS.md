# Pragmastat Development Guide

Multi-language statistical library with implementations in C#, Go, Kotlin, Python, R, Rust, and TypeScript.

## Build System

All builds via [mise](https://mise.jdx.dev/). Never run raw commands directly.

```bash
mise tasks              # List all available tasks
mise run <lang>:ci      # Full CI for one language (cs, go, kt, py, r, rs, ts)
mise run ci             # Full CI for all languages
```

Task naming: `<target>:<action>` (e.g., `rs:build`, `cs:test`, `go:check:fix`)

See `mise.toml` for complete task definitions and toolchain versions.

## Repository Structure

```
pragmastat/
├── cs/, go/, kt/, py/, r/, rs/, ts/   # Language implementations
├── manual/                             # Source content (Typst)
│   └── definitions.typ                 # Shared definitions and version
├── tests/                              # Cross-language test data (JSON)
├── tools/                              # Rust CLI for content generation
├── img/                                # Source images (_light/_dark variants)
├── web/                                # Generated web output (Astro)
├── sim/                                # Simulation output data
├── VERSION                             # Single version source
├── CITATION.cff                        # Citation metadata
└── mise.toml                           # Build configuration
```

## Architecture

**Version flow:** `manual/definitions.typ` → `mise run docs:sync` → all language manifests

**Content flow:** `manual/*.typ` → `typst` → `manual/pragmastat.pdf` and `tools/` → `web/` (Astro)

**Test data:** JSON files in `tests/` organized by estimator. Each language loads these via its own test loader.

## Key APIs

All implementations expose:

**One-sample estimators:** `center`, `spread`, `relSpread` (deprecated)

**Two-sample estimators:** `shift`, `ratio`, `disparity`

**Bounds estimators:** `shiftBounds`, `ratioBounds`, `centerBounds`, `spreadBounds`, `disparityBounds`

**Randomization:** `Rng`, `sample`, `shuffle`, `resample`

**Note:** Function names follow language conventions: camelCase for TypeScript/Kotlin/Go, snake_case for Python/Rust/R, PascalCase for C#.

**Error handling:** All languages use structured assumption errors with `Violation(id, subject)`:
- IDs: `validity`, `domain`, `positivity`, `sparity` (checked in this priority order)
- Subjects: `x`, `y`, `misrate`
- Language-specific types: `AssumptionError` (Python/Go/Rust/TypeScript), `AssumptionException` (C#/Kotlin), `assumption_error` condition (R)

See individual language READMEs and AGENTS.md for API details.

## Notes for AI Agents

- Always use `mise run <task>` instead of raw commands
- R uses system R (not mise-managed)
- PDF uses Typst (not LaTeX); run `pdf:restore` for fonts
- Test data in `tests/` is the cross-language validation source
