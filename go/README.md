# Go

Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v4@v5.0.0
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v5.0.0/go

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
	// --- Randomization ---

	rng := pragmastat.NewRngFromSeed(1729)
	fmt.Println(rng.Uniform()) // 0.3943034703296536
	fmt.Println(rng.Uniform()) // 0.5730893757071377

	rng = pragmastat.NewRngFromString("experiment-1")
	fmt.Println(rng.Uniform()) // 0.9535207726895857

	rng = pragmastat.NewRngFromSeed(1729)
	fmt.Println(pragmastat.Sample(rng, []float64{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}, 3)) // [6 8 9]

	rng = pragmastat.NewRngFromSeed(1729)
	fmt.Println(pragmastat.Shuffle(rng, []float64{1, 2, 3, 4, 5})) // [4 2 3 5 1]

	// --- Distribution Sampling ---

	rng = pragmastat.NewRngFromSeed(1729)
	dist := pragmastat.NewUniform(0, 10)
	fmt.Println(dist.Sample(rng)) // 3.9430347032965365

	rng = pragmastat.NewRngFromSeed(1729)
	addDist := pragmastat.NewAdditive(0, 1)
	fmt.Println(addDist.Sample(rng)) // -1.222932972163442

	rng = pragmastat.NewRngFromSeed(1729)
	expDist := pragmastat.NewExp(1)
	fmt.Println(expDist.Sample(rng)) // 0.5013761944646019

	rng = pragmastat.NewRngFromSeed(1729)
	powDist := pragmastat.NewPower(1, 2)
	fmt.Println(powDist.Sample(rng)) // 1.284909255071668

	rng = pragmastat.NewRngFromSeed(1729)
	mulDist := pragmastat.NewMultiplic(0, 1)
	fmt.Println(mulDist.Sample(rng)) // 0.2943655336550937

	// --- Single-Sample Statistics ---

	x := []float64{0, 2, 4, 6, 8}

	print(pragmastat.Median(x))              // 4
	print(pragmastat.Center(x))              // 4
	print(pragmastat.Spread(x))              // 4
	print(pragmastat.Spread(add(x, 10)))     // 4
	print(pragmastat.Spread(multiply(x, 2))) // 8
	print(pragmastat.RelSpread(x))           // 1

	// --- Two-Sample Comparison ---

	x = []float64{0, 3, 6, 9, 12}
	y := []float64{0, 2, 4, 6, 8}

	print(pragmastat.Shift(x, y))     // 2
	print(pragmastat.Shift(y, x))     // -2
	print(pragmastat.AvgSpread(x, y)) // 5
	print(pragmastat.Disparity(x, y)) // 0.4
	print(pragmastat.Disparity(y, x)) // -0.4

	x = []float64{1, 2, 4, 8, 16}
	y = []float64{2, 4, 8, 16, 32}
	print(pragmastat.Ratio(x, y)) // 0.5

	// --- Confidence Bounds ---

	x = []float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
		16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30}
	y = []float64{
		21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
		36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50}

	margin, _ := pragmastat.PairwiseMargin(30, 30, 1e-4)
	fmt.Println(margin)                                   // 390
	print(pragmastat.Shift(x, y))                         // -20
	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-4))) // [-30, -10]
}
```
