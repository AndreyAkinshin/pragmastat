# Project Instructions

## AGENTS.md Requirement

**MANDATORY**: Before starting any development work in this repository, read `/AGENTS.md` at the repository root and follow its instructions exactly.

AGENTS.md contains:
- Project architecture and structure
- Language-specific conventions
- Testing patterns and requirements
- Development workflows

All instructions in AGENTS.md take precedence over general conventions.

---

## Build System

**MANDATORY**: Always use `mise` for all build, test, and development actions.

- Never use raw language toolchain commands (`cargo`, `go`, `dotnet`, `pnpm`, etc.) directly
- Always use the corresponding mise task instead
- Run `mise tasks` to see available tasks

Common patterns:
```bash
# Building
mise run build          # Single-language project
mise run rs:build       # Multi-language: Rust
mise run cs:build       # Multi-language: C#

# Testing
mise run test           # Single-language project
mise run rs:test        # Multi-language: Rust

# Full CI pipeline
mise run ci             # Single-language project
mise run rs:ci          # Multi-language: Rust
```

If a task doesn't exist for what you need, ask before running raw commands.

---

## Generated Files

**MANDATORY**: Never edit auto-generated files directly.

Generated files in this repository:
- `web/src/content/manual/*.mdx` — generated from `manual/**/*.typ` (Typst sources)

Always edit the source files and run the generation task to update generated outputs.
