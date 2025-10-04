# Pragmastat

This is a Go implementation of 'Pragmastat: Pragmatic Statistical Toolkit', which presents a toolkit of statistical procedures that provide reliable results across diverse real-world distributions, with ready-to-use implementations and detailed explanations.

- PDF manual for this version: https://pragmastat.dev/pragmastat-v3.1.18.pdf
- Online manual for the latest version: https://pragmastat.dev
- Manual DOI: [10.5281/zenodo.17236778](https://doi.org/10.5281/zenodo.17236778)
- Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v3.1.18/go

## Installation

To install the package from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v3
```

## Demo

```go
package main

import (
	"fmt"
	"log"

	pragmastat "github.com/AndreyAkinshin/pragmastat/go/v3"
)

func add(x []float64, val float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v + val
	}
	return result
}

func subtract(x []float64, val float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v - val
	}
	return result
}

func divide(x []float64, val float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v / val
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
	fmt.Println(mustCenter(x))              // 4
	fmt.Println(mustCenter(add(x, 10)))     // 14
	fmt.Println(mustCenter(multiply(x, 3))) // 12

	fmt.Println(mustSpread(x))              // 4
	fmt.Println(mustSpread(add(x, 10)))     // 4
	fmt.Println(mustSpread(multiply(x, 2))) // 8

	fmt.Println(mustRelSpread(x))              // 1
	fmt.Println(mustRelSpread(multiply(x, 5))) // 1

	y := []float64{10, 12, 14, 16, 18}
	fmt.Println(mustShift(x, y))                           // -10
	fmt.Println(mustShift(x, x))                           // 0
	fmt.Println(mustShift(add(x, 7), add(y, 3)))           // -6
	fmt.Println(mustShift(multiply(x, 2), multiply(y, 2))) // -20
	fmt.Println(mustShift(y, x))                           // 10

	x = []float64{1, 2, 4, 8, 16}
	y = []float64{2, 4, 8, 16, 32}
	fmt.Println(mustRatio(x, y))                           // 0.5
	fmt.Println(mustRatio(x, x))                           // 1
	fmt.Println(mustRatio(multiply(x, 2), multiply(y, 5))) // 0.2

	x = []float64{0, 3, 6, 9, 12}
	y = []float64{0, 2, 4, 6, 8}
	fmt.Println(mustSpread(x)) // 6
	fmt.Println(mustSpread(y)) // 4

	fmt.Println(mustAvgSpread(x, y))                           // 5
	fmt.Println(mustAvgSpread(x, x))                           // 6
	fmt.Println(mustAvgSpread(multiply(x, 2), multiply(x, 3))) // 15
	fmt.Println(mustAvgSpread(y, x))                           // 5
	fmt.Println(mustAvgSpread(multiply(x, 2), multiply(y, 2))) // 10

	fmt.Println(mustShift(x, y))     // 2
	fmt.Println(mustAvgSpread(x, y)) // 5

	fmt.Println(mustDisparity(x, y))                           // 0.4
	fmt.Println(mustDisparity(add(x, 5), add(y, 5)))           // 0.4
	fmt.Println(mustDisparity(multiply(x, 2), multiply(y, 2))) // 0.4
	fmt.Println(mustDisparity(y, x))                           // -0.4
}

func mustCenter(x []float64) float64 {
	result, err := pragmastat.Center(x)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func mustSpread(x []float64) float64 {
	result, err := pragmastat.Spread(x)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func mustRelSpread(x []float64) float64 {
	result, err := pragmastat.RelSpread(x)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func mustShift(x, y []float64) float64 {
	result, err := pragmastat.Shift(x, y)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func mustRatio(x, y []float64) float64 {
	result, err := pragmastat.Ratio(x, y)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func mustAvgSpread(x, y []float64) float64 {
	result, err := pragmastat.AvgSpread(x, y)
	if err != nil {
		log.Fatal(err)
	}
	return result
}

func mustDisparity(x, y []float64) float64 {
	result, err := pragmastat.Disparity(x, y)
	if err != nil {
		log.Fatal(err)
	}
	return result
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
