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

**Specification**: @manual/tests/*.md describe the complete reference test suite

**Generator**: @cs/Pragmastat.TestGenerator/ generates the actual JSON test files

**Output**: @tests/ contains generated JSON files

## Task

### Step 1: Learn the Specification

Examine the files in `@manual/tests/` (e.g., `_framework.md`, `_motivation.md`, `center-tests.md`, `spread-tests.md`, etc.) to understand:
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
- Update generator code to match the documentation in `@manual/tests/*.md`
- Regenerate JSON files using `./build.sh cs generate`
- Verify output correctness

### Step 4: Validate

Ensure the generator implementation perfectly matches the test specification.

## Commands

**Regenerate reference test JSON files:**

```bash
./build.sh cs generate
```

**Run all reference tests:**

```bash
./build.sh test
```

**Individual language tests:**

```bash
./build.sh cs test    # C#
./build.sh go test    # Go
./build.sh kt test    # Kotlin
./build.sh py test    # Python
./build.sh r test     # R
./build.sh rs test    # Rust
./build.sh ts test    # TypeScript
```
