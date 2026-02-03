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
│   │   ├── FastCenter.cs       # O(n log n) Hodges-Lehmann algorithm
│   │   └── FastShift.cs        # O((m+n) log L) shift quantiles
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
│   ├── Sample.cs               # Core sample type
│   └── Randomization/          # Rng, Xoshiro256
├── Pragmastat.Demo/            # Demo application
├── Pragmastat.Tests/           # Unit tests
├── Pragmastat.TestGenerator/   # Reference test generator
└── Pragmastat.Extended/        # Extended estimators (Mean, MAD, StdDev)
```

## Key Types

| Type | Purpose |
|------|---------|
| `Sample` | Immutable array wrapper with arithmetic operators |
| `Rng` | Deterministic xoshiro256++ PRNG |
| `IDistribution` | Interface for sampling distributions |
| `Bounds` | Struct with `Lower` and `Upper` properties |

## Public API

```csharp
// Extension methods on Sample
sample.Center()
sample.Spread()
sample.RelSpread()

// Static methods in Toolkit
Toolkit.Median(sample)
Toolkit.Shift(x, y)
Toolkit.Ratio(x, y)
Toolkit.AvgSpread(x, y)
Toolkit.Disparity(x, y)
Toolkit.ShiftBounds(x, y, misrate)
Toolkit.RatioBounds(x, y, misrate)

// PairwiseMargin
PairwiseMargin.Instance.Calc(n, m, misrate)
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory via TestGenerator
- **Test runner**: Uses custom runner in `Pragmastat.Tests`
- **Tolerance**: `1e-10` for floating-point comparisons

```bash
dotnet test                      # All tests
dotnet run --project Pragmastat.Tests # Run test project
```

## Error Handling

Uses exceptions for error conditions:

```csharp
try {
    var result = sample.Center();
} catch (ArgumentException e) {
    // Handle: empty input, invalid parameters
}
```

## Unique Features

- **Sample type**: Supports arithmetic operators (`+`, `-`, `*`, `/`)
- **Metrology**: Unit-aware measurements with formatting
- **Extended estimators**: Traditional estimators (Mean, MAD, StdDev) in separate assembly

## Build Configuration

- Library targets: netstandard2.0, net6.0 (multi-targeting)
- Tooling targets: .NET 10.0 (Demo, Tests, TestGenerator)
- Nullable reference types enabled
- Implicit usings enabled
- Central package management via `Directory.Build.props`
