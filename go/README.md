# Pragmastat

This is a Go implementation of 'Pragmastat: Pragmatic Statistical Toolkit', which presents a toolkit of statistical procedures that provide reliable results across diverse real-world distributions, with ready-to-use implementations and detailed explanations.

- PDF manual for this version: [pragmastat-v4.0.0.pdf](https://github.com/AndreyAkinshin/pragmastat/releases/download/v4.0.0/pragmastat-v4.0.0.pdf)
- Markdown manual for this version: [pragmastat-v4.0.0.md](https://github.com/AndreyAkinshin/pragmastat/releases/download/v4.0.0/pragmastat-v4.0.0.md)
- Source code for this version: [pragmastat/go/v4.0.0](https://github.com/AndreyAkinshin/pragmastat/tree/v4.0.0/go)
- Latest online manual: https://pragmastat.dev
- Manual DOI: [10.5281/zenodo.17236778](https://doi.org/10.5281/zenodo.17236778)

## Installation

Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v3@v4.0.0
```

## Demo

```go
package main

import (
	"fmt"
	"log"

	pragmastat "github.com/AndreyAkinshin/pragmastat/go/v3"
)

func must[T any](val T, err error) T {
	if err != nil {
		log.Fatal(err)
	}
	return val
}

func print(val float64, err error) {
	fmt.Println(must(val, err))
}

func add(x []float64, val float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v + val
	}
	return result
}

func multiply(x []float64, val float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v * val
	}
	return result
}

func main() {
	x := []float64{0, 2, 4, 6, 8}
	print(pragmastat.Center(x))              // 4
	print(pragmastat.Center(add(x, 10)))     // 14
	print(pragmastat.Center(multiply(x, 3))) // 12

	print(pragmastat.Spread(x))              // 4
	print(pragmastat.Spread(add(x, 10)))     // 4
	print(pragmastat.Spread(multiply(x, 2))) // 8

	print(pragmastat.RelSpread(x))              // 1
	print(pragmastat.RelSpread(multiply(x, 5))) // 1

	y := []float64{10, 12, 14, 16, 18}
	print(pragmastat.Shift(x, y))                           // -10
	print(pragmastat.Shift(x, x))                           // 0
	print(pragmastat.Shift(add(x, 7), add(y, 3)))           // -6
	print(pragmastat.Shift(multiply(x, 2), multiply(y, 2))) // -20
	print(pragmastat.Shift(y, x))                           // 10

	x = []float64{1, 2, 4, 8, 16}
	y = []float64{2, 4, 8, 16, 32}
	print(pragmastat.Ratio(x, y))                           // 0.5
	print(pragmastat.Ratio(x, x))                           // 1
	print(pragmastat.Ratio(multiply(x, 2), multiply(y, 5))) // 0.2

	x = []float64{0, 3, 6, 9, 12}
	y = []float64{0, 2, 4, 6, 8}
	print(pragmastat.Spread(x)) // 6
	print(pragmastat.Spread(y)) // 4

	print(pragmastat.AvgSpread(x, y))                           // 5
	print(pragmastat.AvgSpread(x, x))                           // 6
	print(pragmastat.AvgSpread(multiply(x, 2), multiply(x, 3))) // 15
	print(pragmastat.AvgSpread(y, x))                           // 5
	print(pragmastat.AvgSpread(multiply(x, 2), multiply(y, 2))) // 10

	print(pragmastat.Shift(x, y))     // 2
	print(pragmastat.AvgSpread(x, y)) // 5

	print(pragmastat.Disparity(x, y))                           // 0.4
	print(pragmastat.Disparity(add(x, 5), add(y, 5)))           // 0.4
	print(pragmastat.Disparity(multiply(x, 2), multiply(y, 2))) // 0.4
	print(pragmastat.Disparity(y, x))                           // -0.4

	x = []float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
		16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30}
	y = []float64{
		21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
		36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50}

	fmt.Println(pragmastat.PairwiseMargin(30, 30, 1e-6)) // 276
	fmt.Println(pragmastat.PairwiseMargin(30, 30, 1e-5)) // 328
	fmt.Println(pragmastat.PairwiseMargin(30, 30, 1e-4)) // 390
	fmt.Println(pragmastat.PairwiseMargin(30, 30, 1e-3)) // 464

	print(pragmastat.Shift(x, y)) // -20

	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-6))) // [-33, -7]
	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-5))) // [-32, -8]
	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-4))) // [-30, -10]
	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-3))) // [-28, -12]
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
