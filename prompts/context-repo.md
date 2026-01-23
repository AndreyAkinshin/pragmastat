# Pragmastat Repository Structure

## Repository Layout

```
pragmastat/
├── Language implementations (7)
│   ├── cs/          # C# (.NET)
│   ├── go/          # Go
│   ├── kt/          # Kotlin (JVM)
│   ├── py/          # Python
│   ├── r/           # R
│   ├── rs/          # Rust
│   └── ts/          # TypeScript/JavaScript
│
├── Auxiliary tools (3)
│   ├── tools/       # Docs/version sync + conversion (Rust)
│   ├── img/         # Image generation (plots, diagrams, logo)
│   └── web/         # Website (Astro)
│
├── manual/          # Documentation source (Typst)
│   ├── **/*.typ     # Manual content in Typst format
│   └── definitions.typ  # Shared definitions and version
│
├── tests/           # Reference tests (JSON)
│
├── sim/             # Simulation data (JSON)
│
├── prompts/         # LLM context and tasks files
│   └── *.md
│
├── artifacts/       # Generated after CI build
│
└── Build system
    ├── mise.toml           # Task definitions (entry point)
    └── docker-compose.yml  # Docker services
```

## Directory Purpose

### Language Implementations (`cs/`, `go/`, `kt/`, `py/`, `r/`, `rs/`, `ts/`)

Each implements 7 core estimators with:
- Fast algorithms (O(n log n))
- Unit and reference tests
- Demo examples
- Build script and Dockerfile
- Package metadata

### Auxiliary Tools (`tools/`, `img/`, `web/`)

- **tools**: Reads version from `manual/definitions.typ`, syncs versioned files and generates Astro content
- **img**: Generates plots and logo using Python/matplotlib
- **web**: Astro-based website, uses pnpm

### Documentation (`manual/`)

Organized Typst source covering:
- Estimators, algorithms, properties
- Distributions and studies
- Implementation guides for each language
- Academic references

### Reference Tests (`tests/`)

JSON files with input samples and expected outputs.
Shared across all language implementations to ensure consistency.

### Simulation Data (`sim/`)

JSON files with drift simulation results for documentation plots.

## Key Entry Points

- **Build system**: `mise run <task>` (see @prompts/context-build.md)
- **Version**: `manual/version.typ`
- **Documentation**: `manual/*.typ`
- **Tests**: `tests/<estimator>/*.json`
