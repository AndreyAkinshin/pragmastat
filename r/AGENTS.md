# R Implementation

## Build Commands

```bash
mise run r:ci        # Full CI: clean → restore → check → build → test
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
│   ├── aaa_constants.R          # Internal constants (loaded first)
│   ├── aa_assumptions.R         # Input validation and error types
│   ├── center.R                 # Center estimator
│   ├── center_bounds.R          # Center confidence bounds
│   ├── spread.R                 # Spread estimator
│   ├── spread_bounds.R          # Spread confidence bounds
│   ├── shift.R                  # Shift estimator
│   ├── shift_bounds.R           # Shift confidence bounds
│   ├── ratio.R                  # Ratio estimator
│   ├── ratio_bounds.R           # Ratio confidence bounds
│   ├── avg_spread.R             # Average spread
│   ├── avg_spread_bounds.R      # Average spread confidence bounds
│   ├── disparity.R              # Disparity (effect size)
│   ├── disparity_bounds.R       # Disparity confidence bounds
│   ├── pairwise_margin.R        # Margin calculation
│   ├── sign_margin.R            # Sign margin for binomial CDF inversion
│   ├── signed_rank_margin.R     # Signed-rank margin computation
│   ├── min_misrate.R            # Minimum achievable misrate calculation
│   ├── fast_center.R            # O(n log n) Hodges-Lehmann algorithm
│   ├── fast_center_quantiles.R  # Center quantile binary search
│   ├── fast_spread.R            # O(n log n) Shamos algorithm
│   ├── fast_shift.R             # O((m+n) log L) shift quantiles
│   ├── rng.R                    # Deterministic xoshiro256++ PRNG (R6 class)
│   ├── xoshiro256.R             # PRNG core implementation (R6 class)
│   └── dist_*.R                 # Distribution classes
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
| `Rng` | R6 | Deterministic PRNG with `uniform_float()`, `uniform_float_range()`, `uniform_int()`, `uniform_bool()`, `sample()`, `resample()`, `shuffle()` |
| `Uniform` | R6 | Uniform distribution |
| `Additive` | R6 | Additive (Normal/Gaussian) distribution |
| `Exp` | R6 | Exponential distribution |
| `Power` | R6 | Power distribution |
| `Multiplic` | R6 | Multiplicative (Log-Normal) distribution |

## Public Functions

```r
center(x)                              # Hodges-Lehmann estimator
spread(x)                              # Shamos estimator
shift(x, y)                            # Median of pairwise differences
ratio(x, y)                            # Geometric median of pairwise ratios
disparity(x, y)                        # Shift / AvgSpread
shift_bounds(x, y, misrate = 1e-3)     # Confidence bounds on shift
ratio_bounds(x, y, misrate = 1e-3)     # Confidence bounds on ratio
disparity_bounds(x, y, misrate = 1e-3, seed = NULL) # Confidence bounds on disparity
center_bounds(x, misrate = 1e-3)       # Confidence bounds on center
spread_bounds(x, misrate = 1e-3, seed = NULL)       # Confidence bounds on spread
```

## Testing

- **Reference tests**: Load JSON fixtures from `tests/` directory (copied during test)
- **Invariance tests**: Verify mathematical properties
- **Test framework**: testthat v3
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run r:test              # All tests (preferred)
```

```r
devtools::test()             # All tests (from R console)
testthat::test_file("tests/testthat/test-center.R") # Single file
```

## Error Handling

Functions signal `assumption_error` conditions (with `violation` field containing `id` and `subject`):

```r
tryCatch({
    result <- center(x)
}, assumption_error = function(e) {
    # e$violation$id: "validity", "domain", "positivity", "sparity"
    # e$violation$subject: "x", "y", "misrate"
})
```

Error conditions:
- Empty or non-finite input vectors (`validity`)
- `misrate` outside valid range (`domain`)
- Non-positive values for `ratio` (`positivity`)
- Tie-dominant sample (`sparity`)

## Dependencies

- **Imports**: R6 (for OOP classes)
- **Suggests**: testthat (>= 3.0.0), jsonlite (for test fixtures)

## Package Structure

- Uses roxygen2 for documentation
- Follows CRAN submission guidelines
- MIT license
