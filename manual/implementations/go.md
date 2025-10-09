<span id="go"></span> <!-- [pdf] DELETE -->

## Go

Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v3@v3.1.28
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v3.1.28/go



Demo:

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
}
```