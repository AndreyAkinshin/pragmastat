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
│   ├── Estimators.kt       # Public API: median, center, spread, shift, etc.
│   ├── PairwiseMargin.kt   # Margin calculation for shift bounds
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
| `Rng` | Deterministic PRNG with `uniform()`, `sample()`, `shuffle()` |
| `Bounds` | Data class with `lower` and `upper` properties |
| `Distribution` | Interface for sampling distributions |

## Public Functions

```kotlin
fun median(x: List<Double>): Double
fun center(x: List<Double>): Double
fun spread(x: List<Double>): Double
fun relSpread(x: List<Double>): Double
fun shift(x: List<Double>, y: List<Double>): Double
fun ratio(x: List<Double>, y: List<Double>): Double
fun avgSpread(x: List<Double>, y: List<Double>): Double
fun disparity(x: List<Double>, y: List<Double>): Double
fun shiftBounds(x: List<Double>, y: List<Double>, misrate: Double): Bounds
fun pairwiseMargin(n: Int, m: Int, misrate: Double): Int
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
- **Tolerance**: `1e-10` for floating-point comparisons

```bash
./gradlew test              # All tests
./gradlew test --info       # Verbose output
```

## Error Handling

Functions throw exceptions for invalid inputs:

```kotlin
try {
    val result = center(x)
} catch (e: IllegalArgumentException) {
    // Handle: empty input, invalid parameters
}
```

Error conditions:
- Empty input lists
- `misrate` outside `[0, 1]`
- Division by zero (e.g., `relSpread` when center is zero)
- Non-positive values in `y` for `ratio`

## Build Configuration

- Kotlin 2.0+
- JVM toolchain: 11
- Build tool: Gradle with Kotlin DSL
- Published to Maven Central under `dev.pragmastat:pragmastat`
