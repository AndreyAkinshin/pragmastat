# Pragmastat Go

A Go implementation of 'Pragmastat: Pragmatic Statistical Toolkit' - robust summary estimators designed for real-world data analysis.
Online manual: https://pragmastat.dev

## Installation

```bash
go get github.com/AndreyAkinshin/pragmastat/go
```

## Demo

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

## The MIT License

Copyright (c) 2025 Andrey Akinshin

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
