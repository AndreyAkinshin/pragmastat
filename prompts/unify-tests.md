# Reference Tests Unification

## Context

Pragmastat maintains seven implementations across different programming languages.
All implementations must produce identical numerical results for all estimators.

The **C# implementation** serves as the reference generator:
- Defines test cases programmatically
- Executes them to produce expected outputs
- Serializes to JSON format in @tests/ directory

Other implementations load these JSON files and verify their estimators match within numerical tolerance.

## Source of Truth

**Specification**: @manual/tests/*.typ describe the complete reference test suite

**Generator**: @cs/Pragmastat.TestGenerator/ generates the actual JSON test files

**Output**: @tests/ contains generated JSON files

## Task

### Step 1: Learn the Specification

Examine the files in `@manual/tests/` (e.g., `_framework.typ`, `_motivation.typ`, `center-tests.typ`, `spread-tests.typ`, etc.) to understand:
- Complete test inventory for each estimator (Center, Spread, RelSpread, Shift, Ratio, AvgSpread, Disparity)
- Test categories (demo, natural, edge cases, fuzzy, stress tests, unsorted)
- Test naming conventions
- Random generation mechanisms (seeds, distributions)

### Step 2: Audit the Generator

Examine @cs/Pragmastat.TestGenerator/ and verify:
- Test case counts match the specification
- Test names follow documented conventions
- Input values match specification exactly
- Random generation uses correct seeds and distributions
- All test categories are implemented

### Step 3: Fix Discrepancies

For any inconsistencies found:
- Update generator code to match the documentation in `@manual/tests/*.typ`
- Regenerate JSON files using `mise run cs:gen`
- Verify output correctness

### Step 4: Validate

Ensure the generator implementation perfectly matches the test specification.

## Commands

**Regenerate reference test JSON files:**

```bash
mise run cs:gen
```

**Run all reference tests:**

```bash
mise run test
```

**Individual language tests:**

```bash
mise run cs:test    # C#
mise run go:test    # Go
mise run kt:test    # Kotlin
mise run py:test    # Python
mise run r:test     # R
mise run rs:test    # Rust
mise run ts:test    # TypeScript
```
