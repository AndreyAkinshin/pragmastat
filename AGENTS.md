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
└── mise.toml                           # Build configuration
```

## Architecture

**Version flow:** `manual/definitions.typ` → `mise run docs:sync` → all language manifests

**Content flow:** `manual/*.typ` → `typst` → `manual/pragmastat.pdf` and `tools/` → `web/` (Astro)

**Test data:** JSON files in `tests/` organized by estimator. Each language loads these via its own test loader.

## Key APIs

All implementations expose: `center`, `spread`, `relSpread`, `shift`, `ratio`, `avgSpread`, `disparity`, `shiftBounds`, `ratioBounds`, `pairwiseMargin`

See individual language READMEs for API details.

## Notes for AI Agents

- Always use `mise run <task>` instead of raw commands
- R uses system R (not mise-managed)
- PDF uses Typst (not LaTeX); run `pdf:restore` for fonts
- Test data in `tests/` is the cross-language validation source
