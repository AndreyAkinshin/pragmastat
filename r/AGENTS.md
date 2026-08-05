# R Implementation

## Build Commands

```bash
mise run r:ci        # Full CI: clean → restore → check → build → test
mise run r:test      # Run tests only
mise run r:check     # R CMD check (no tests, no dependencies)
mise run r:demo      # Run demo
mise run r:build     # Build source package (.tar.gz)
mise run r:restore   # Install package locally
mise run r:doc       # Regenerate the three roxygen-driven .Rd files
```

**Note:** R is not managed by mise. Uses system R via `Rscript`.

**NAMESPACE and most .Rd files are written by hand.** Only three .Rd files are roxygen output,
all for internal `*_impl_compute` helpers, and they say so in their first line. NAMESPACE carries
no roxygen header, which is what stops `r:doc` from overwriting it.

Do not hand the exports to roxygen without doing the work first. Measured: 14 of the 39 exports
have no roxygen block to generate from, and they are `center`, `spread`, `shift`, `ratio`,
`disparity`, the five `*_bounds`, the three `*_unit` values and `DEFAULT_MISRATE`, which is
everything a caller of this package actually calls. Generating NAMESPACE today deletes them.

`#' @export` tags used to sit above 28 definitions and produced nothing, since NAMESPACE is not
generated. They are gone: a tag that looks like it wires an export, and does not, is worse than
no tag.

**No format gate.** Every other port has a checking half and a fixing half of one pair:
`dotnet format --verify-no-changes`, `gofmt` with a guard, `ruff format --check`, `cargo fmt
--check`, `prettier --check`, `ktlint`. R has neither. A format check is only a check when the
thing doing the checking is pinned, and each of those is pinned by a lockfile or a toolchain,
while R here has no lockfile, installs from CRAN HEAD, and mise carries no CRAN backend to pin
from. An unpinned formatter turns the next styler release into a red build on a commit that
touched no R.

## Architecture

```
r/pragmastat/
├── R/
│   ├── aaa_constants.R         # Internal constants (loaded first)
│   ├── aa_assumptions.R        # Input validation and error types
│   ├── normalize_zero.R        # Drops the sign of a negative zero on estimator output
│   ├── center.R                # Center estimator
│   ├── center_bounds.R         # Center confidence bounds
│   ├── spread.R                # Spread estimator
│   ├── spread_bounds.R         # Spread confidence bounds
│   ├── shift.R                 # Shift estimator
│   ├── shift_bounds.R          # Shift confidence bounds
│   ├── ratio.R                 # Ratio estimator
│   ├── ratio_bounds.R          # Ratio confidence bounds
│   ├── avg_spread.R            # Average spread
│   ├── avg_spread_bounds.R     # Average spread confidence bounds
│   ├── disparity.R             # Disparity (effect size)
│   ├── disparity_bounds.R      # Disparity confidence bounds
│   ├── pairwise_margin.R       # Margin calculation
│   ├── sign_margin.R           # Sign margin for binomial CDF inversion
│   ├── signed_rank_margin.R    # Signed-rank margin computation
│   ├── min_misrate.R           # Minimum achievable misrate calculation
│   ├── additive_cumulative.R   # Standard normal CDF (Chebyshev-fitted erf/erfc)
│   ├── exp_function.R          # Reproducible exp: range reduction plus fitted polynomial
│   ├── center_impl.R           # O(n log n) Hodges-Lehmann algorithm
│   ├── center_quantiles_impl.R # Center quantile binary search
│   ├── spread_impl.R           # O(n log n) Shamos algorithm
│   ├── shift_impl.R            # O((m+n) log L) shift quantiles
│   ├── rng.R                   # Deterministic xoshiro256++ PRNG (R6 class)
│   ├── xoshiro256.R            # PRNG core implementation (plain functions)
│   └── dist_*.R                # Distribution classes
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

Each function is dual-dispatch: it accepts either a native numeric **vector**
(returns a plain unitless `numeric`) or a **`Sample`** object (returns a
`Measurement` carrying the unit). Dispatch is by `inherits(x, "Sample")`; for the
two-sample functions, the `Sample` path is taken only when *both* `x` and `y` are
Samples, otherwise the vector path runs.

The `assume_sorted` flag applies to the **vector path only**: when `TRUE`, the
already-ascending input is used as-is and the internal sort is skipped (undefined
behavior if the data is not actually sorted). For the shuffle-based
`spread_bounds`/`disparity_bounds` the disjoint-pair shuffle always runs on the
passed order (the flag never affects the shuffle); it only reaches the
order-independent sub-computations, so `spread_bounds` is effectively inert to it
while `disparity_bounds` (whose sub-computation embeds `shift_bounds`) can
silently differ on unsorted input. It is ignored on the `Sample` path,
which always reuses the Sample's cached `sorted_values` view.

```r
# x, y below are either a numeric vector or a Sample.
center(x, assume_sorted = FALSE)                              # Hodges-Lehmann estimator
spread(x, assume_sorted = FALSE)                              # Shamos estimator
shift(x, y, assume_sorted = FALSE)                            # Median of pairwise differences
ratio(x, y, assume_sorted = FALSE)                            # Geometric median of pairwise ratios
disparity(x, y, assume_sorted = FALSE)                        # Shift / AvgSpread
center_bounds(x, misrate = DEFAULT_MISRATE, assume_sorted = FALSE)       # Confidence bounds on center
spread_bounds(x, misrate = DEFAULT_MISRATE, seed = NULL, assume_sorted = FALSE)  # Confidence bounds on spread
shift_bounds(x, y, misrate = DEFAULT_MISRATE, assume_sorted = FALSE)     # Confidence bounds on shift
ratio_bounds(x, y, misrate = DEFAULT_MISRATE, assume_sorted = FALSE)     # Confidence bounds on ratio
disparity_bounds(x, y, misrate = DEFAULT_MISRATE, seed = NULL, assume_sorted = FALSE) # Confidence bounds on disparity
```

Internal (not exported by `NAMESPACE`): `avg_spread(x, y)` and
`avg_spread_bounds(x, y, misrate, seed)` follow the same dual dispatch but have
no `assume_sorted` flag; the test suite reaches them via the package namespace.

## Testing

- **Reference tests**: Load JSON fixtures from `tests/` directory (copied during test)
- **Invariance tests**: Verify mathematical properties
- **Test framework**: testthat v3
- **Comparison**: bitwise. `helper-reference-tests.R` defaults to `tolerance = NULL`, which
  compares bit patterns, and only the ratio family sets a tolerance, because ratio is
  `exp(median(log x - log y))` and libm may legitimately move it. A selection-based estimator
  returns an element of the pairwise set or the midpoint of two, so a divergence is not a small
  error: either the same element was selected and the answer is bit-identical, or a different one
  was, and no epsilon bounds that gap

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

- Documentation is written by hand, except three .Rd files for internal helpers (see above)
- Follows CRAN submission guidelines
- MIT license
