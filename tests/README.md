# Cross-Language Test Data

This directory contains reference test data shared across all language implementations.
Each language loads these JSON files to verify correctness and cross-language consistency.

## Directory Structure

```
tests/
├── assumptions/         # Assumption validation tests
├── avg-spread/          # AvgSpread estimator tests
├── center/              # Center estimator tests
├── center-bounds/       # CenterBounds estimator tests
├── disparity/           # Disparity estimator tests
├── distributions/       # Distribution sampling tests
├── median-bounds/       # MedianBounds estimator tests
├── pairwise-margin/     # PairwiseMargin function tests
├── ratio/               # Ratio estimator tests
├── ratio-bounds/        # RatioBounds estimator tests
├── rel-spread/          # RelSpread estimator tests
├── resample/            # Resample with replacement (bootstrap) tests
├── rng/                 # Random number generator tests
├── sample/              # Sample without replacement tests
├── shift/               # Shift estimator tests
├── shift-bounds/        # ShiftBounds estimator tests
├── shuffle/             # Shuffle tests
├── signed-rank-margin/  # SignedRankMargin function tests
└── spread/              # Spread estimator tests
```

## Test File Format

Each test file is a JSON object with `input` and `output` fields:

```json
{
  "input": { ... },
  "output": ...
}
```

### One-sample estimators (center, spread, rel-spread)

```json
{
  "input": { "x": [1, 2, 3, 4, 5] },
  "output": 3.0
}
```

### Two-sample estimators (shift, ratio, avg-spread, disparity)

```json
{
  "input": { "x": [1, 2, 3], "y": [4, 5, 6] },
  "output": -3.0
}
```

### PairwiseMargin (two-sample)

```json
{
  "input": { "n": 10, "m": 10, "misrate": 0.05 },
  "output": 42
}
```

### SignedRankMargin (one-sample)

```json
{
  "input": { "n": 10, "misrate": 0.05 },
  "output": 6
}
```

### Bounds estimators (shift-bounds, ratio-bounds, median-bounds, center-bounds)

```json
{
  "input": { "x": [1, 2, 3, 4, 5], "misrate": 0.1 },
  "output": { "lower": 1.5, "upper": 4.5 }
}
```

### sample / resample

Both use the same format. `sample` draws without replacement; `resample` draws with replacement (bootstrap).

```json
{
  "input": { "seed": 1729, "x": [0, 1, 2, 3, 4], "k": 3 },
  "output": [3.0, 1.0, 4.0]
}
```

### Error test cases

Error test cases verify domain validation. They use `expected_error` instead of `output`:

```json
{
  "input": { "n": 1, "misrate": 0.5 },
  "expected_error": {
    "id": "domain"
  }
}
```

The `id` field identifies the error type (e.g., "domain", "validity", "sparity").

## Test Case Naming

Test cases follow a consistent naming taxonomy:

| Prefix | Purpose |
|--------|---------|
| `demo-*` | Basic demonstration cases from documentation |
| `edge-*` | Edge cases and boundary conditions |
| `boundary-*` | Boundary value tests |
| `exact-*` | Exact algorithm tests (vs approximation) |
| `property-*` | Algebraic property verification |
| `unsorted-*` | Input ordering invariance tests |
| `natural-*` | Natural number sequences (1, 2, 3, ...) |
| `additive-*` | Additive distribution samples |
| `uniform-*` | Uniform distribution samples |
| `asymmetric-*` | Asymmetric distribution tests |
| `symmetric-*` | Symmetric distribution tests |
| `medium-*` | Medium-size sample tests |
| `misrate-*` | Misrate parameter variation tests |

### Misrate notation

Misrate values use compact notation: `mr<mantissa>e<exponent>` represents `mantissa × 10^-exponent`:
- `mr1e1` = 0.1 (10%)
- `mr5e2` = 0.05 (5%)
- `mr1e2` = 0.01 (1%)

## Tolerance Values

Tests use these standard tolerances for floating-point comparison:

| Tolerance | Value | Usage |
|-----------|-------|-------|
| Exact | 1e-15 | RNG reproducibility tests |
| Strict | 1e-10 | Exact algorithms (margin functions) |
| Standard | 1e-9 | Most estimators |
| Relaxed | 1e-6 | Bootstrap/approximate methods (reserved for future use) |

## Adding New Tests

1. Generate test data using the C# test generator:
   ```bash
   mise run cs:generate-tests
   ```

2. Test files are created in the appropriate subdirectory

3. Run all language CIs to verify:
   ```bash
   mise run ci
   ```

## Test Generation

Tests are generated from `cs/Pragmastat.TestGenerator/`. Each `*TestCases.cs` file
defines inputs, and the framework computes expected outputs using the C# implementation
as the reference.

See `manual/tests/` for documentation of each test suite's purpose and coverage.
