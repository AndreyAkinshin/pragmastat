# Rust Implementation

## Build Commands

```bash
mise run rs:ci       # Full CI: clean → check → build → test
mise run rs:test     # Run tests only
mise run rs:check    # Lint (clippy) + format check
mise run rs:check:fix # Auto-format code
mise run rs:demo     # Run demo example
mise run rs:bench    # Run benchmarks
```

## Architecture

```
rs/pragmastat/
├── src/
│   ├── lib.rs                     # Public exports
│   ├── estimators.rs              # Public API: center, spread, shift, etc.
│   ├── assumptions.rs             # Input validation and error types
│   ├── pairwise_margin.rs         # Margin calculation for shift bounds (internal)
│   ├── sign_margin.rs             # Sign margin for binomial CDF inversion (internal)
│   ├── signed_rank_margin.rs      # Signed-rank margin computation (internal)
│   ├── min_misrate.rs             # Minimum achievable misrate calculation (internal)
│   ├── gauss_cdf.rs               # Standard normal CDF (ACM Algorithm 209) (internal)
│   ├── rng.rs                     # Deterministic xoshiro256++ PRNG
│   ├── distributions/             # Sampling distributions (Uniform, Additive, Exp, Power, Multiplic)
│   ├── center_impl.rs             # O(n log n) Hodges-Lehmann algorithm (internal)
│   ├── center_quantiles_impl.rs   # Center quantile binary search (internal)
│   ├── spread_impl.rs             # O(n log n) Shamos algorithm (internal)
│   ├── shift_impl.rs              # O((m+n) log L) shift quantiles (internal)
│   ├── xoshiro256.rs              # PRNG core implementation (internal)
│   ├── splitmix64.rs              # Seed mixing (internal)
│   ├── fnv1a.rs                   # Hash for deterministic seeding (internal)
│   ├── avg_spread_tests.rs        # Average spread unit tests
│   ├── avg_spread_bounds_tests.rs # Average spread bounds unit tests
│   ├── disparity_bounds_tests.rs  # Disparity bounds unit tests
│   ├── pairwise_margin_tests.rs   # Pairwise margin unit tests
│   ├── ratio_bounds_tests.rs      # Ratio bounds error-priority tests
│   └── signed_rank_margin_tests.rs # Signed-rank margin unit tests
├── tests/
│   ├── assume_sorted_tests.rs             # assume-sorted equivalence
│   ├── compare_tests.rs                   # Compare framework
│   ├── error_tests.rs                     # Error path coverage
│   ├── invariance_tests.rs                # Mathematical property tests
│   ├── metrology_tests.rs                 # Bounds unit re-attachment
│   ├── performance_tests.rs               # Performance smoke test
│   ├── reference_tests.rs                 # JSON fixture validation
│   └── sample_bounds_consistency_tests.rs # Sample vs raw bounds on unsorted input
└── examples/
    └── demo.rs
```

## Key Modules

| Module | Visibility | Purpose |
|--------|------------|---------|
| `estimators` | Public | All statistical estimators |
| `pairwise_margin` | Internal | Misclassification margin calculation |
| `rng` | Public | Deterministic PRNG with `Rng` struct |
| `distributions` | Public | `Distribution` trait + implementations |
| `*_impl` | Internal | O(n log n) algorithms, not part of public API |

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Invariance tests**: Verify mathematical properties (shift symmetry, spread scaling)
- **Error tests**: Validate error handling for invalid inputs
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run rs:test              # All tests (preferred)
cargo test                    # All tests (raw)
cargo test reference          # Reference tests only
cargo test invariance         # Invariance tests only
cargo test --test error_tests # Error handling tests
```

## Error Handling

The crate exposes two parallel entry points for every estimator. All public
functions return `Result<T, EstimatorError>`. Errors use
`EstimatorError::Assumption(AssumptionError)` with `violation()`.

### (a) Typed Sample API (`pragmastat::estimators::*`)

Takes `&Sample` and returns unit-carrying `Measurement`/`Bounds`. This is the
primary, recommended surface.

```rust
pub fn center(x: &Sample) -> Result<Measurement, EstimatorError>
pub fn spread(x: &Sample) -> Result<Measurement, EstimatorError>
pub fn shift(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError>
pub fn ratio(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError>
pub fn disparity(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError>
pub fn shift_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError>
pub fn ratio_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError>
pub fn disparity_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError>
pub fn center_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError>
pub fn spread_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError>
pub fn spread_bounds_with_seed(x: &Sample, misrate: f64, seed: &str) -> Result<Bounds, EstimatorError>
pub fn disparity_bounds_with_seed(x: &Sample, y: &Sample, misrate: f64, seed: &str) -> Result<Bounds, EstimatorError>
```

### (b) Raw native-slice API (`pragmastat::estimators::raw::*`)

Takes `&[f64]` directly, returns plain `f64` / `RawBounds` (no units). Every
function takes a trailing `assume_sorted: bool` — when `true`, the caller
guarantees the slice(s) are already ascending and the internal sort is skipped.

```rust
pub fn center(x: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError>
pub fn spread(x: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError>
pub fn shift(x: &[f64], y: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError>
pub fn ratio(x: &[f64], y: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError>
pub fn disparity(x: &[f64], y: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError>
pub fn shift_bounds(x: &[f64], y: &[f64], misrate: f64, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
pub fn ratio_bounds(x: &[f64], y: &[f64], misrate: f64, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
pub fn center_bounds(x: &[f64], misrate: f64, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
pub fn spread_bounds(x: &[f64], misrate: f64, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
pub fn spread_bounds_with_seed(x: &[f64], misrate: f64, seed: &str, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
pub fn disparity_bounds(x: &[f64], y: &[f64], misrate: f64, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
pub fn disparity_bounds_with_seed(x: &[f64], y: &[f64], misrate: f64, seed: &str, assume_sorted: bool) -> Result<RawBounds, EstimatorError>
```

The typed Sample API delegates to `raw`, passing `assume_sorted = true` from the
`Sample`'s cached sorted values. For the order-independent functions (`center`,
`spread`, `shift`, `ratio`, `disparity`, `center_bounds`, `shift_bounds`,
`ratio_bounds`) the flag skips the internal sort and changes the computation
path. For the shuffle-based `spread_bounds`/`disparity_bounds` (and `_with_seed`)
the disjoint-pair shuffle always runs on the caller's slice; the flag feeds the
slice as a pre-sorted view into the order-independent sub-computations only. For
`spread_bounds` that is just the sparity check, so on a genuinely sorted slice
the flag never changes the result. For `disparity_bounds` the view also feeds
the embedded `shift_bounds` sub-call, so passing `assume_sorted = true` with
UNSORTED input silently changes the result (on a genuinely sorted slice it is
again identical).

Error conditions:
- Empty or non-finite input slices (`Validity`)
- Invalid `misrate` (`Domain`)
- Non-positive values for `ratio` (`Positivity`)
- Tie-dominant sample (`Sparity`)

## Determinism

The `center_impl` and `spread_impl` algorithms use deterministic pivot selection via FNV-1a hash of input values. Same input always produces same output across runs and platforms.

## Linting

The warnings gate lives in CI, not in `Cargo.toml`: a published crate must not
break docs.rs or downstream source builds when a future rustc adds new warnings.
`mise run rs:check` runs:
- `cargo clippy -- -D warnings` (clippy and rustc warnings are errors in the library)
- `cargo fmt -- --check`
- `RUSTFLAGS="-D warnings" cargo check --all-targets` (rustc warnings are errors
  everywhere, including tests and examples; cargo caps lints for external
  dependencies, so the flag only gates this crate)

Note: `#[must_use]` is not needed on functions returning `Result` (already has `#[must_use]`)
