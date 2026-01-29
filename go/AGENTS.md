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
├── estimators.go       # Public API: Median, Center, Spread, Shift, etc.
├── pairwise_margin.go  # Margin calculation for shift bounds
├── rng.go              # Deterministic xoshiro256++ PRNG
├── xoshiro256.go       # PRNG core implementation
├── fast_center.go      # O(n log n) Hodges-Lehmann algorithm
├── fast_spread.go      # O(n log n) Shamos algorithm
├── fast_shift.go       # O((m+n) log L) shift quantiles
├── distribution.go     # Distribution interface
├── uniform.go          # Uniform distribution
├── additive.go         # Additive (Laplace) distribution
├── exp.go              # Exponential distribution
├── power.go            # Power distribution
├── multiplic.go        # Multiplicative (log-Laplace) distribution
├── demo/
│   └── main.go         # Demo application
├── reference_test.go   # JSON fixture validation
├── invariance_test.go  # Mathematical property tests
└── performance_test.go
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `Uniform()`, `Sample()`, `Shuffle()` |
| `Distribution` | Interface for sampling distributions |
| `Bounds` | Lower/upper bounds for `ShiftBounds` |

## Public Functions

```go
func Median(x []float64) (float64, error)
func Center(x []float64) (float64, error)
func Spread(x []float64) (float64, error)
func RelSpread(x []float64) (float64, error)
func Shift(x, y []float64) (float64, error)
func Ratio(x, y []float64) (float64, error)
func AvgSpread(x, y []float64) (float64, error)
func Disparity(x, y []float64) (float64, error)
func ShiftBounds(x, y []float64, misrate float64) (Bounds, error)
func PairwiseMargin(n, m int, misrate float64) (int, error)
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Invariance tests**: Verify mathematical properties
- **Tolerance**: `1e-10` for floating-point comparisons

```bash
go test ./...           # All tests
go test -v ./...        # Verbose output
go test -bench=. ./...  # Run benchmarks
```

## Error Handling

All estimator functions return `(T, error)`:

```go
val, err := pragmastat.Center(x)
if err != nil {
    // Handle: empty input, NaN values, invalid parameters
}
```

Error variables:
- `errEmptyInput` - empty slice provided
- `errNMustBePositive` - n must be > 0
- `errMMustBePositive` - m must be > 0
- `errMisrateOutOfRange` - misrate not in [0, 1]

## Determinism

The `fastCenter` and `fastSpread` algorithms use deterministic pivot selection via FNV-1a hash of input values. Uses generics with `Number` constraint for type safety.

## Linting

Uses `golangci-lint` with default configuration. Format check via `go fmt`.
