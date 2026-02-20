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
│   ├── Estimators.kt       # Public API: center, spread, shift, etc.
│   ├── PairwiseMargin.kt   # Margin calculation for shift bounds (internal)
│   ├── Rng.kt              # Deterministic xoshiro256++ PRNG
│   ├── Xoshiro256.kt       # PRNG core implementation
│   ├── FastCenter.kt       # O(n log n) Hodges-Lehmann algorithm
│   ├── FastSpread.kt       # O(n log n) Shamos algorithm
│   ├── FastShift.kt        # O((m+n) log L) shift quantiles
│   ├── Constants.kt        # Internal constants
│   └── distributions/
│       ├── Distribution.kt
│       ├── Uniform.kt
│       ├── Additive.kt
│       ├── Exp.kt
│       ├── Power.kt
│       └── Multiplic.kt
├── src/test/kotlin/dev/pragmastat/
│   ├── ReferenceTest.kt    # JSON fixture validation
│   ├── InvarianceTest.kt   # Mathematical property tests
│   └── PerformanceTest.kt
└── build.gradle.kts
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `uniformDouble()`, `sample()`, `shuffle()` |
| `Bounds` | Data class with `lower` and `upper` properties |
| `Distribution` | Interface for sampling distributions |

## Public Functions

```kotlin
fun center(x: List<Double>): Double
fun spread(x: List<Double>): Double
fun relSpread(x: List<Double>): Double  // deprecated: use spread(x) / abs(center(x))
fun shift(x: List<Double>, y: List<Double>): Double
fun ratio(x: List<Double>, y: List<Double>): Double
fun disparity(x: List<Double>, y: List<Double>): Double
fun shiftBounds(x: List<Double>, y: List<Double>, misrate: Double = 1e-3): Bounds
fun ratioBounds(x: List<Double>, y: List<Double>, misrate: Double = 1e-3): Bounds
fun disparityBounds(x: List<Double>, y: List<Double>, misrate: Double = 1e-3, seed: String? = null): Bounds
fun centerBounds(x: List<Double>, misrate: Double = 1e-3): Bounds
fun spreadBounds(x: List<Double>, misrate: Double = 1e-3, seed: String? = null): Bounds
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
