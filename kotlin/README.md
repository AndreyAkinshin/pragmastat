# Pragmastat for Kotlin

Kotlin implementation of the Pragmastat statistical toolkit.

## Installation

Add the dependency to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation("com.pragmastat:pragmastat:1.0.0")
}
```

## Usage

```kotlin
import com.pragmastat.*

fun main() {
    // One-sample analysis
    val x = listOf(1.2, 3.4, 2.5, 4.1, 2.8)
    
    println("Center: ${center(x)}")
    println("Spread: ${spread(x)}")
    println("Volatility: ${volatility(x) * 100}%")
    println("Precision: ${precision(x)}")
    
    // Two-sample comparison
    val y = listOf(2.1, 4.3, 3.2, 5.0, 3.7)
    
    println("MedShift: ${medShift(x, y)}")
    println("MedRatio: ${medRatio(x, y)}")
    println("MedSpread: ${medSpread(x, y)}")
    println("MedDisparity: ${medDisparity(x, y)}")
}
```

## Estimators

### One-Sample Estimators

- **Center**: Hodges-Lehmann location estimator (median of pairwise averages)
- **Spread**: Shamos scale estimator (median of pairwise absolute differences)
- **Volatility**: Relative dispersion (Spread/|Center|)
- **Precision**: Confidence interval half-width (2·Spread/√n)

### Two-Sample Estimators

- **MedShift**: Median of pairwise differences
- **MedRatio**: Median of pairwise ratios
- **MedSpread**: Weighted average of individual spreads
- **MedDisparity**: Effect size (MedShift/MedSpread)

## Development

Build the project:
```bash
./gradlew build
```

Run tests:
```bash
./gradlew test
```

## License

MIT