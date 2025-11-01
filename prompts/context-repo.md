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
├── Auxiliary tools (4)
│   ├── gen/         # Content generation (Markdown, configs)
│   ├── img/         # Image generation (plots, diagrams, logo)
│   ├── pdf/         # PDF manual (Pandoc/LaTeX)
│   └── web/         # Website (Hugo)
│
├── manual/          # Documentation source (Markdown)
│   ├── **/*.md      # Manual content
│   └── version.txt  # Single source of truth for version
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
    ├── build.sh     # Main dispatcher (entry point)
    ├── docker-compose.yml
    └── <subdir>/build.sh
```

## Directory Purpose

### Language Implementations (`cs/`, `go/`, `kt/`, `py/`, `r/`, `rs/`, `ts/`)

Each implements 7 core estimators with:
- Fast algorithms (O(n log n))
- Unit and reference tests
- Demo examples
- Build script and Dockerfile
- Package metadata

### Auxiliary Tools (`gen/`, `img/`, `pdf/`, `web/`)

- **gen**: Reads `manual/version.txt`, generates version-dependent files
- **img**: Generates plots and logo using Python/matplotlib
- **pdf**: Pandoc-based PDF generation from `manual/`
- **web**: Hugo-based website, downloads tools via `init` command

### Documentation (`manual/`)

Organized Markdown source covering:
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

- **Build system**: `./build.sh` (see @prompts/context-build.md)
- **Version**: `manual/version.txt`
- **Documentation**: `manual/*.md`
- **Tests**: `tests/<estimator>/*.json`
