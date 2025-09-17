# Pragmastat Go

A Go implementation of the Pragmastat statistical toolkit for reliable analysis of real-world data.

## Features

- **Robust estimators** that handle outliers gracefully
- **High efficiency** comparable to traditional methods under normality
- **Simple implementation** without complex dependencies
- **Well-tested** with comprehensive test coverage

## Installation

```bash
go get github.com/AndreyAkinshin/pragmastat
```

## Usage

```go
package main

import (
    "fmt"
    "github.com/AndreyAkinshin/pragmastat"
)

func main() {
    // One-sample analysis
    x := []float64{1.2, 3.4, 2.5, 4.1, 2.8}
    
    center := pragmastat.Center(x)
    spread := pragmastat.Spread(x)
    relSpread := pragmastat.RelSpread(x)

    fmt.Printf("Center: %.2f\n", center)
    fmt.Printf("Spread: %.2f\n", spread)
    fmt.Printf("RelSpread: %.2f%%\n", relSpread*100)

    // Two-sample comparison
    y := []float64{2.1, 4.3, 3.2, 5.0, 3.7}

    shift := pragmastat.Shift(x, y)
    ratio := pragmastat.Ratio(x, y)
    disparity := pragmastat.Disparity(x, y)

    fmt.Printf("\nX vs Y comparison:\n")
    fmt.Printf("Shift: %.2f\n", shift)
    fmt.Printf("Ratio: %.2f\n", ratio)
    fmt.Printf("Disparity: %.2f\n", disparity)
}
```

## Estimators

### One-Sample Estimators

- **Center(x)**: Robust measure of average (Hodges-Lehmann estimator)
- **Spread(x)**: Robust measure of dispersion (Shamos estimator)
- **RelSpread(x)**: Relative dispersion (robust coefficient of variation)

### Two-Sample Estimators

- **Shift(x, y)**: Typical difference between samples (Hodges-Lehmann shift)
- **Ratio(x, y)**: Typical ratio between samples
- **AvgSpread(x, y)**: Combined spread of both samples
- **Disparity(x, y)**: Effect size (robust alternative to Cohen's d)

## Mathematical Properties

The estimators maintain important mathematical invariances:

- **Location invariance**: Spread is unaffected by shifting all values
- **Scale equivariance**: Estimators scale appropriately with the data
- **Robustness**: Resistant to outliers and extreme values

## Testing

Run the test suite:

```bash
go test ./...
```

Run tests with coverage:

```bash
go test -cover ./...
```

Run specific test suites:

```bash
# Unit tests only
go test -run "^Test[^R]" ./...

# Reference tests only
go test -run "TestReference" ./...

# Invariance tests only
go test -run "TestInvariance" ./...
```

## License

MIT