# Pragmastat for Kotlin

[![DOI](https://zenodo.org/badge/doi/10.5281/zenodo.17236778.svg)](https://doi.org/10.5281/zenodo.17236778)

Kotlin implementation of the Pragmastat statistical toolkit.

## Usage

```kotlin
import com.pragmastat.*

fun main() {
    // One-sample analysis
    val x = listOf(1.2, 3.4, 2.5, 4.1, 2.8)

    println("Center: ${center(x)}")
    println("Spread: ${spread(x)}")
    println("RelSpread: ${relSpread(x) * 100}%")

    // Two-sample comparison
    val y = listOf(2.1, 4.3, 3.2, 5.0, 3.7)

    println("Shift: ${shift(x, y)}")
    println("Ratio: ${ratio(x, y)}")
    println("AvgSpread: ${avgSpread(x, y)}")
    println("Disparity: ${disparity(x, y)}")
}
```

## Estimators

### One-Sample Estimators

- **Center**: Hodges-Lehmann location estimator (median of pairwise averages)
- **Spread**: Shamos scale estimator (median of pairwise absolute differences)
- **RelSpread**: Relative dispersion (Spread/|Center|)

### Two-Sample Estimators

- **Shift**: Median of pairwise differences
- **Ratio**: Median of pairwise ratios
- **AvgSpread**: Weighted average of individual spreads
- **Disparity**: Effect size (Shift/AvgSpread)

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