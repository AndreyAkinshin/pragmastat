# C# Implementation

## Build Commands

```bash
mise run cs:ci       # Full CI: clean → restore → check → build → test
mise run cs:test     # Run tests only
mise run cs:check    # Format verification (dotnet format)
mise run cs:check:fix # Auto-format code
mise run cs:demo     # Run demo application
mise run cs:build    # Build (debug mode)
mise run cs:build:release # Build (release mode)
mise run cs:pack     # Create NuGet package
mise run cs:gen      # Generate reference test files
```

## Architecture

```
cs/
├── Pragmastat/
│   ├── Algorithms/
│   │   ├── CenterImpl.cs           # O(n log n) Hodges-Lehmann center
│   │   ├── CenterQuantilesImpl.cs  # Quantiles of pairwise averages via binary search
│   │   ├── RatioImpl.cs            # Ratio quantiles via log-transform + ShiftImpl
│   │   ├── ShiftImpl.cs            # O((m+n) log L) shift quantiles
│   │   └── SpreadImpl.cs           # O(n log n) Shamos spread
│   ├── Estimators/
│   │   ├── IOneSampleEstimator.cs
│   │   ├── ITwoSampleEstimator.cs
│   │   ├── CenterEstimator.cs
│   │   ├── SpreadEstimator.cs
│   │   ├── ShiftEstimator.cs
│   │   └── ...
│   ├── Functions/
│   │   ├── PairwiseMargin.cs
│   │   ├── ErrorFunction.cs
│   │   └── ...
│   ├── Bounds.cs               # Lower/upper bound pair
│   ├── Probability.cs          # Probability value type
│   ├── Sample.cs               # Core sample type
│   ├── Toolkit.cs              # Static API entry point
│   └── Randomization/          # Rng, Xoshiro256
├── Pragmastat.Demo/            # Demo application
├── Pragmastat.Tests/           # Unit tests
└── Pragmastat.TestGenerator/   # Reference test generator
```

## Key Types

| Type | Purpose |
|------|---------|
| `Sample` | Immutable array wrapper with arithmetic operators |
| `Rng` | Deterministic xoshiro256++ PRNG |
| `IDistribution` | Interface for sampling distributions |
| `Bounds` | Struct with `Lower` and `Upper` properties |

## Public API

Each estimator is exposed through two parallel entry points on `Toolkit`:
(a) the **typed `Sample` API** (unit-aware, returns `Measurement`/`Bounds`), and
(b) the **raw native-array API** (`double[]`, returns `double`/`Bounds`) with an
`assumeSorted` flag that skips the internal sort when the caller guarantees the
input is already sorted ascending. `assumeSorted` is inert on already-sorted
input; passing `assumeSorted: true` on unsorted input is undefined behavior. For
the shuffle-based `SpreadBounds`/`DisparityBounds` the disjoint-pair shuffle
always runs on the passed order (the flag never affects the shuffle); it only
reaches the order-independent sub-computations, so `SpreadBounds` is effectively
inert to it while `DisparityBounds` (whose sub-computation embeds `ShiftBounds`)
can silently differ on unsorted input.

```csharp
// (a) Typed Sample API
Measurement Center(Sample x)
Measurement Spread(Sample x)
Measurement Shift(Sample x, Sample y)
Measurement Ratio(Sample x, Sample y)
Measurement Disparity(Sample x, Sample y)
Bounds CenterBounds(Sample x, Probability misrate)
Bounds SpreadBounds(Sample x, Probability misrate, string seed)
Bounds ShiftBounds(Sample x, Sample y, Probability misrate)
Bounds RatioBounds(Sample x, Sample y, Probability misrate)
Bounds DisparityBounds(Sample x, Sample y, Probability misrate, string seed)

// (b) Raw native-array API (assumeSorted defaults to false)
double Center(double[] x, bool assumeSorted = false)
double Spread(double[] x, bool assumeSorted = false)
double Shift(double[] x, double[] y, bool assumeSorted = false)
double Ratio(double[] x, double[] y, bool assumeSorted = false)
double Disparity(double[] x, double[] y, bool assumeSorted = false)
Bounds CenterBounds(double[] x, double misrate, bool assumeSorted = false)
Bounds SpreadBounds(double[] x, double misrate, string seed, bool assumeSorted = false)
Bounds ShiftBounds(double[] x, double[] y, double misrate, bool assumeSorted = false)
Bounds RatioBounds(double[] x, double[] y, double misrate, bool assumeSorted = false)
Bounds DisparityBounds(double[] x, double[] y, double misrate, string seed, bool assumeSorted = false)
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory via TestGenerator
- **Test runner**: xunit.v3 (self-executing test project)
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run cs:test                 # All tests (preferred)
dotnet test                      # All tests (raw)
```

## Error Handling

Uses `AssumptionException` (extends `ArgumentException`) with `Violation` property:

```csharp
try {
    var result = Toolkit.Center(sample);
} catch (AssumptionException e) {
    // e.Violation.Id: Validity, Domain, Positivity, Sparity
    // e.Violation.Subject: X, Y, Misrate
}
```

## Unique Features

- **Sample type**: Supports arithmetic operators (`+`, `-`, `*`, `/`)
- **Metrology**: Unit-aware measurements with formatting
## Build Configuration

- Library targets: netstandard2.0, net6.0 (multi-targeting)
- Tooling targets: .NET 10.0 (Demo, Tests, TestGenerator)
- Nullable reference types enabled
- Implicit usings enabled
- Central package management via `Directory.Build.props`
