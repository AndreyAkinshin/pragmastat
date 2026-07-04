# Go Implementation

## Build Commands

```bash
mise run go:ci       # Full CI: clean → restore → check → build → test
mise run go:test     # Run tests only
mise run go:check    # Format verification + golangci-lint
mise run go:check:fix # Auto-format code
mise run go:demo     # Run demo
mise run go:bench    # Run benchmarks
mise run go:coverage # Run tests with coverage
```

## Architecture

```
go/
├── estimators.go              # Public API: Center, Spread, Shift, etc.
├── assumptions.go             # Input validation and error types
├── pairwise_margin.go         # Margin calculation for shift bounds
├── sign_margin.go             # Sign margin for binomial CDF inversion
├── signed_rank_margin.go      # Signed-rank margin computation
├── min_misrate.go             # Minimum achievable misrate calculation
├── gauss_cdf.go               # Standard normal CDF (ACM Algorithm 209)
├── rng.go                     # Deterministic xoshiro256++ PRNG
├── xoshiro256.go              # PRNG core implementation
├── center_impl.go             # O(n log n) Hodges-Lehmann algorithm
├── center_quantiles_impl.go   # Center quantile binary search
├── spread_impl.go             # O(n log n) Shamos algorithm
├── shift_impl.go              # O((m+n) log L) shift quantiles
├── distribution.go            # Distribution interface
├── uniform.go                 # Uniform distribution
├── additive.go                # Additive (Normal/Gaussian) distribution
├── exp.go                     # Exponential distribution
├── power.go                   # Power distribution
├── multiplic.go               # Multiplicative (Log-Normal) distribution
├── demo/
│   └── main.go                # Demo application
├── assume_sorted_test.go      # assume-sorted equivalence
├── properties_test.go         # Unit propagation, misrate domain, n==2 symmetry
├── center_convergence_test.go # Center convergence-guard regression
├── compare_test.go            # Compare framework
├── dualpath_test.go           # Dual-path reference (raw + Sample)
├── invariance_test.go         # Mathematical property tests
├── mutation_test.go           # Raw-API input-mutation safety
├── performance_test.go        # Performance smoke test
├── ratio_bounds_test.go       # ratioBounds error priority
├── reference_test.go          # JSON fixture validation
├── sample_race_test.go        # Concurrent Sample access (race detector)
├── sample_test.go             # Sample construction
├── spread_convergence_test.go # Spread convergence-guard regression
└── subject_test.go            # Positional subject assignment
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `UniformFloat64()`, `UniformBool()`, `SampleSlice()`, `ResampleSlice()`, `ShuffleSlice()` |
| `Distribution` | Interface for sampling distributions |
| `Bounds` | Lower/upper bounds for `ShiftBounds` |

## Public Functions

The library exposes two parallel entry points for every estimator: a **typed
Sample API** (methods on `*Sample`, returning `Measurement`/`Bounds` with unit
propagation) and a **raw native-slice API** (package-level functions on
`[]float64` with an explicit `assumeSorted bool` parameter).

### Typed Sample API (methods on `*Sample`)

```go
// Point estimators (return Measurement with propagated unit)
func (s *Sample) Center() (Measurement, error)
func (s *Sample) Spread() (Measurement, error)
func (s *Sample) Shift(other *Sample) (Measurement, error)
func (s *Sample) Ratio(other *Sample) (Measurement, error)
func (s *Sample) Disparity(other *Sample) (Measurement, error)

// Bounds estimators
func (s *Sample) CenterBounds(misrate float64) (Bounds, error)
func (s *Sample) SpreadBounds(misrate float64) (Bounds, error)
func (s *Sample) SpreadBoundsWithSeed(misrate float64, seed string) (Bounds, error)
func (s *Sample) ShiftBounds(other *Sample, misrate float64) (Bounds, error)
func (s *Sample) RatioBounds(other *Sample, misrate float64) (Bounds, error)
func (s *Sample) DisparityBounds(other *Sample, misrate float64) (Bounds, error)
func (s *Sample) DisparityBoundsWithSeed(other *Sample, misrate float64, seed string) (Bounds, error)
```

Construct via `NewSample`, `NewSampleWithUnit`, or `NewWeightedSample`. A
`Sample` caches its sorted values, so estimator calls reuse the sorted view.

### Raw native-slice API (package-level, `[]float64` + `assumeSorted`)

Each function takes `assumeSorted bool`: pass `true` ONLY when the input slice
is already sorted ascending (the estimators use sorted-only kernels). Passing
`true` on unsorted input is undefined behavior. For the shuffle-based
`SpreadBounds`/`DisparityBounds` the disjoint-pair shuffle always runs on the
passed order (the flag never affects the shuffle); it only reaches the
order-independent sub-computations, so `SpreadBounds` is effectively inert to it
while `DisparityBounds` (whose sub-computation embeds `ShiftBounds`) can silently
differ on unsorted input.

```go
// Point estimators
func Center(x []float64, assumeSorted bool) (float64, error)
func Spread(x []float64, assumeSorted bool) (float64, error)
func Shift(x, y []float64, assumeSorted bool) (float64, error)
func Ratio(x, y []float64, assumeSorted bool) (float64, error)
func Disparity(x, y []float64, assumeSorted bool) (float64, error)

// Bounds estimators
func CenterBounds(x []float64, misrate float64, assumeSorted bool) (Bounds, error)
func SpreadBounds(x []float64, misrate float64, assumeSorted bool) (Bounds, error)
func SpreadBoundsWithSeed(x []float64, misrate float64, seed string, assumeSorted bool) (Bounds, error)
func ShiftBounds(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error)
func RatioBounds(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error)
func DisparityBounds(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error)
func DisparityBoundsWithSeed(x, y []float64, misrate float64, seed string, assumeSorted bool) (Bounds, error)
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Invariance tests**: Verify mathematical properties
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run go:test        # All tests (preferred)
go test ./...           # All tests (raw)
go test -v ./...        # Verbose output
go test -bench=. ./...  # Run benchmarks
```

## Error Handling

All estimator functions return `(T, error)`. There are two error classes:

**Assumption violations** are `*AssumptionError` with a `Violation` struct:

```go
val, err := pragmastat.Center(x, false)
if err != nil {
    var ae *pragmastat.AssumptionError
    if errors.As(err, &ae) {
        // ae.Violation.ID: Validity, Domain, Positivity, Sparity
        // ae.Violation.Subject: SubjectX, SubjectY, SubjectMisrate
    }
}
```

Assumption error conditions:
- Empty or non-finite input (`Validity`)
- `misrate` outside valid range (`Domain`)
- Non-positive values for `Ratio` (`Positivity`)
- Tie-dominant sample (`Sparity`)

**Plain errors** (NOT `*AssumptionError`) cover everything else:
- `convergence failure (pathological input)` from the bounded `centerImpl`/
  `spreadImpl` selection loops when `assumeSorted=true` is misused on
  unsorted input (the convergence tests assert this is a plain error)
- `Sample` construction/usage failures (weights length mismatch, negative or
  zero total weight, weighted samples passed to unweighted-only estimators)

Always match with `errors.As` instead of assuming every error is an
`*AssumptionError`.

## Determinism

The `centerImpl` and `spreadImpl` algorithms use deterministic pivot selection via FNV-1a hash of input values. Uses generics with `Number` constraint for type safety.

## Linting

Uses `golangci-lint` with default configuration. Format check via `go fmt`.
