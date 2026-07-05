# Kotlin Implementation

## Build Commands

```bash
mise run kt:ci       # Full CI: clean → restore → check → build → test
mise run kt:test     # Run tests only
mise run kt:check    # Run all checks (lint, format)
mise run kt:demo     # Run demo application
mise run kt:build    # Build with Gradle
mise run kt:pack     # Create JAR package
```

## Architecture

```
kt/
├── src/main/kotlin/dev/pragmastat/
│   ├── Estimators.kt            # Raw List-based API: center, spread, shift, etc.
│   ├── Sample.kt                # Typed Sample API: unit-aware overloads
│   ├── Measurement.kt           # Value + MeasurementUnit pair
│   ├── MeasurementUnit.kt       # Unit identity, family, conversion
│   ├── UnitRegistry.kt          # MeasurementUnit lookup by id
│   ├── Probability.kt           # Typed [0, 1] wrapper for misrate parameters
│   ├── Compare.kt               # compare1/compare2 threshold-verdict API
│   ├── Assumptions.kt           # Input validation and error types
│   ├── PairwiseMargin.kt        # Margin calculation for shift bounds (internal)
│   ├── SignMargin.kt            # Sign margin for binomial CDF inversion
│   ├── SignedRankMargin.kt      # Signed-rank margin computation
│   ├── MinMisrate.kt            # Minimum achievable misrate calculation
│   ├── GaussCdf.kt              # Standard normal CDF (ACM Algorithm 209)
│   ├── Rng.kt                   # Deterministic xoshiro256++ PRNG
│   ├── Xoshiro256.kt            # PRNG core implementation
│   ├── CenterImpl.kt            # O(n log n) Hodges-Lehmann algorithm
│   ├── CenterQuantilesImpl.kt   # Center quantile binary search
│   ├── SpreadImpl.kt            # O(n log n) Shamos algorithm
│   ├── ShiftImpl.kt             # O((m+n) log L) shift quantiles
│   ├── Constants.kt             # Internal constants
│   └── distributions/
│       ├── Distribution.kt
│       ├── Uniform.kt
│       ├── Additive.kt
│       ├── Exp.kt
│       ├── Power.kt
│       └── Multiplic.kt
├── src/test/kotlin/dev/pragmastat/
│   ├── ReferenceTest.kt                   # JSON fixture validation
│   ├── MetrologyTest.kt                   # JSON fixtures for the unit/metrology system
│   ├── InvarianceTest.kt                  # Mathematical property tests
│   ├── AssumeSortedTest.kt                # Raw-API assumeSorted=true branch coverage
│   ├── MutationTest.kt                    # Estimators must not mutate caller lists
│   ├── CenterMidpointSymmetryTest.kt      # n==2 midpoint exact order symmetry
│   ├── BoundsUnitTest.kt                  # Bounds unit propagation (Sample vs raw)
│   ├── ProbabilityTest.kt                 # Probability [0, 1] range validation
│   ├── RawMisrateDomainTest.kt            # Raw-API misrate domain errors
│   ├── RatioBoundsErrorPriorityTest.kt    # domain-before-positivity error priority
│   └── PerformanceTest.kt                 # Wall-clock sanity checks on large inputs
└── build.gradle.kts
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `uniformDouble()`, `sample()`, `shuffle()` |
| `Bounds` | Data class with `lower` and `upper` properties |
| `Distribution` | Interface for sampling distributions |

## Public Functions

There are two parallel public APIs:

- **Raw `List<Double>` API** (shown below): plain lists in, bare `Double` /
  unitless `Bounds` out. Each estimator takes an optional `assumeSorted: Boolean
  = false`; when `true` the caller guarantees the input is already sorted
  ascending and the internal sort is skipped (undefined behavior on unsorted
  input). For the shuffle-based `spreadBounds`/`disparityBounds` the disjoint-pair
  shuffle always runs on the passed order (the flag never affects the shuffle); it
  only reaches the order-independent sub-computations, so `spreadBounds` is
  effectively inert to it while `disparityBounds` (whose sub-computation embeds
  `shiftBounds`) can silently differ on unsorted input. This is the single DRY
  implementation.
- **Typed `Sample` API** (`Sample`-based overloads in `Sample.kt`): `Sample` /
  `Probability` in, `Measurement` / unit-carrying `Bounds` out. Thin adapters
  that delegate to the raw API, passing the cached sorted view with
  `assumeSorted = true` and re-attaching the appropriate `MeasurementUnit`.

```kotlin
fun center(x: List<Double>, assumeSorted: Boolean = false): Double
fun spread(x: List<Double>, assumeSorted: Boolean = false): Double
fun shift(x: List<Double>, y: List<Double>, assumeSorted: Boolean = false): Double
fun ratio(x: List<Double>, y: List<Double>, assumeSorted: Boolean = false): Double
fun disparity(x: List<Double>, y: List<Double>, assumeSorted: Boolean = false): Double
fun shiftBounds(x: List<Double>, y: List<Double>, misrate: Double = 1e-3, assumeSorted: Boolean = false): Bounds
fun ratioBounds(x: List<Double>, y: List<Double>, misrate: Double = 1e-3, assumeSorted: Boolean = false): Bounds
fun disparityBounds(x: List<Double>, y: List<Double>, misrate: Double = 1e-3, seed: String? = null, assumeSorted: Boolean = false): Bounds
fun centerBounds(x: List<Double>, misrate: Double = 1e-3, assumeSorted: Boolean = false): Bounds
fun spreadBounds(x: List<Double>, misrate: Double = 1e-3, seed: String? = null, assumeSorted: Boolean = false): Bounds
```

## Distributions

```kotlin
class Uniform(min: Double, max: Double) : Distribution
class Additive(location: Double, scale: Double) : Distribution
class Exp(rate: Double) : Distribution
class Power(scale: Double, exponent: Double) : Distribution
class Multiplic(location: Double, scale: Double) : Distribution
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Invariance tests**: Verify mathematical properties
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run kt:test            # All tests (preferred)
./gradlew test              # All tests (raw)
./gradlew test --info       # Verbose output
```

## Error Handling

Functions throw `AssumptionException` (extends `IllegalArgumentException`) with `violation` property:

```kotlin
try {
    val result = center(x)
} catch (e: AssumptionException) {
    // e.violation.id: VALIDITY, DOMAIN, POSITIVITY, SPARITY
    // e.violation.subject: X, Y, MISRATE
}
```

Error conditions:
- Empty or non-finite input lists (`VALIDITY`)
- `misrate` outside valid range (`DOMAIN`)
- Non-positive values for `ratio` (`POSITIVITY`)
- Tie-dominant sample (`SPARITY`)

## Build Configuration

- Kotlin 2.0+
- JVM toolchain: 11
- Build tool: Gradle with Kotlin DSL
- Published to Maven Central under `dev.pragmastat:pragmastat`
