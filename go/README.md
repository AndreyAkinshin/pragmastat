# Go

Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v4@v4.0.2
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v4.0.2/go

## Demo

```go
package main

import (
	"fmt"
	"log"

	pragmastat "github.com/AndreyAkinshin/pragmastat/go/v4"
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
