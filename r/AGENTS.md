# R Implementation

## Build Commands

```bash
mise run r:ci        # Full CI: clean → check → build → test
mise run r:test      # Run tests only
mise run r:check     # R CMD check (no tests, no dependencies)
mise run r:check:fix # Format with styler
mise run r:demo      # Run demo
mise run r:build     # Build source package (.tar.gz)
mise run r:restore   # Install package locally
mise run r:doc       # Build documentation with roxygen2
```

**Note:** R is not managed by mise. Uses system R via `Rscript`.

## Architecture

```
r/pragmastat/
├── R/
│   ├── aaa_constants.R     # Internal constants (loaded first)
│   ├── center.R            # Center estimator
│   ├── spread.R            # Spread estimator
│   ├── rel_spread.R        # Relative spread
│   ├── shift.R             # Shift estimator
│   ├── ratio.R             # Ratio estimator
│   ├── ratio_bounds.R      # Ratio confidence bounds
│   ├── avg_spread.R        # Average spread
│   ├── disparity.R         # Disparity (effect size)
│   ├── shift_bounds.R      # Shift confidence bounds
│   ├── pairwise_margin.R   # Margin calculation
│   ├── fast_center.R       # O(n log n) Hodges-Lehmann algorithm
│   ├── fast_spread.R       # O(n log n) Shamos algorithm
│   ├── fast_shift.R        # O((m+n) log L) shift quantiles
│   ├── rng.R               # Deterministic xoshiro256++ PRNG (R6 class)
│   ├── xoshiro256.R        # PRNG core implementation (R6 class)
│   └── dist_*.R            # Distribution classes
├── tests/testthat/
│   ├── helper-reference-tests.R
│   ├── test-center.R
│   ├── test-spread.R
│   ├── test-shift.R
│   └── ...
├── inst/examples/
│   └── demo.R
├── DESCRIPTION
└── NAMESPACE
```

## Key Classes

| Class | Type | Purpose |
|-------|------|---------|
| `Rng` | R6 | Deterministic PRNG with `uniform()`, `sample()`, `shuffle()` |
| `Uniform` | R6 | Uniform distribution |
| `Additive` | R6 | Additive (Laplace) distribution |
| `Exp` | R6 | Exponential distribution |
| `Power` | R6 | Power distribution |
| `Multiplic` | R6 | Multiplicative (log-Laplace) distribution |

## Public Functions

```r
center(x)                    # Hodges-Lehmann estimator
spread(x)                    # Shamos estimator
rel_spread(x)                # Spread / |Center|
shift(x, y)                  # Median of pairwise differences
ratio(x, y)                  # Geometric median of pairwise ratios (via log-space)
avg_spread(x, y)             # Weighted average of spreads
disparity(x, y)              # Shift / AvgSpread
shift_bounds(x, y, misrate)  # Confidence bounds on shift
ratio_bounds(x, y, misrate)  # Confidence bounds on ratio
pairwise_margin(n, m, misrate) # Margin for bounds calculation
```

## Testing

- **Reference tests**: Load JSON fixtures from `tests/` directory (copied during test)
- **Invariance tests**: Verify mathematical properties
- **Test framework**: testthat v3
- **Tolerance**: `1e-10` for floating-point comparisons

```r
devtools::test()             # All tests
testthat::test_file("tests/testthat/test-center.R") # Single file
```

## Error Handling

Functions use `stop()` for errors:

```r
tryCatch({
    result <- center(x)
}, error = function(e) {
    # Handle: empty input, invalid parameters
})
```

Error conditions:
- Empty input vectors
- `misrate` outside `[0, 1]`
- Division by zero (e.g., `rel_spread` when center is zero)
- Non-positive values in `y` for `ratio`

## Dependencies

- **Imports**: R6 (for OOP classes)
- **Suggests**: testthat (>= 3.0.0), jsonlite (for test fixtures)

## Package Structure

- Uses roxygen2 for documentation
- Follows CRAN submission guidelines
- MIT license
