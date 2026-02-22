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
│   ├── fast_center.rs             # O(n log n) Hodges-Lehmann algorithm (internal)
│   ├── fast_center_quantiles.rs   # Center quantile binary search (internal)
│   ├── fast_spread.rs             # O(n log n) Shamos algorithm (internal)
│   ├── fast_shift.rs              # O((m+n) log L) shift quantiles (internal)
│   ├── xoshiro256.rs              # PRNG core implementation (internal)
│   ├── splitmix64.rs              # Seed mixing (internal)
│   ├── fnv1a.rs                   # Hash for deterministic seeding (internal)
│   ├── avg_spread_tests.rs        # Average spread unit tests
│   ├── avg_spread_bounds_tests.rs # Average spread bounds unit tests
│   └── disparity_bounds_tests.rs  # Disparity bounds unit tests
├── tests/
│   ├── reference_tests.rs         # JSON fixture validation
│   ├── invariance_tests.rs        # Mathematical property tests
│   ├── error_tests.rs             # Error path coverage
│   └── performance_tests.rs
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
| `fast_*` | Internal | O(n log n) algorithms, not part of public API |

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

All public functions return `Result<T, EstimatorError>`. Errors use `EstimatorError::Assumption(AssumptionError)` with `violation()`:

```rust
pub fn center(x: &[f64]) -> Result<f64, EstimatorError>
pub fn spread(x: &[f64]) -> Result<f64, EstimatorError>
pub fn shift(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError>
pub fn ratio(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError>
pub fn disparity(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError>
pub fn shift_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, EstimatorError>
pub fn ratio_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, EstimatorError>
pub fn disparity_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, EstimatorError>
pub fn center_bounds(x: &[f64], misrate: f64) -> Result<Bounds, EstimatorError>
pub fn spread_bounds(x: &[f64], misrate: f64) -> Result<Bounds, EstimatorError>
pub fn spread_bounds_with_seed(x: &[f64], misrate: f64, seed: &str) -> Result<Bounds, EstimatorError>
pub fn disparity_bounds_with_seed(x: &[f64], y: &[f64], misrate: f64, seed: &str) -> Result<Bounds, EstimatorError>
```

Error conditions:
- Empty or non-finite input slices (`Validity`)
- Invalid `misrate` (`Domain`)
- Non-positive values for `ratio` (`Positivity`)
- Tie-dominant sample (`Sparity`)
- `rel_spread` is deprecated; use `spread(x) / center(x).abs()` instead

## Determinism

The `fast_center` and `fast_spread` algorithms use deterministic pivot selection via FNV-1a hash of input values. Same input always produces same output across runs and platforms.

## Linting

Strict clippy configuration in `Cargo.toml`:
- `clippy::all` and `clippy::pedantic` at deny level
- Warnings treated as errors (`-D warnings`)
- Note: `#[must_use]` is not needed on functions returning `Result` (already has `#[must_use]`)
