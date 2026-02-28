# Go

Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v10@v10.0.6
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.6/go

## Demo

```go
package main

import (
	"fmt"
	"log"

	pragmastat "github.com/AndreyAkinshin/pragmastat/go/v10"
)

func must[T any](val T, err error) T {
	if err != nil {
		log.Fatal(err)
	}
	return val
}

func main() {
	// --- One-Sample ---

	x := []float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
		11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
		21, 22}

	fmt.Println(must(pragmastat.Center(x)))                             // 11.5
	fmt.Println(must(pragmastat.CenterBounds(x, 1e-3)))                 // {6 17}
	fmt.Println(must(pragmastat.Spread(x)))                             // 7
	fmt.Println(must(pragmastat.SpreadBoundsWithSeed(x, 1e-3, "demo"))) // {1 18}

	// --- Two-Sample ---

	x = []float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
		16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30}
	y := []float64{
		21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
		36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50}

	fmt.Println(must(pragmastat.Shift(x, y)))                                 // -20
	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-3)))                     // {-28 -12}
	fmt.Println(must(pragmastat.Ratio(x, y)))                                 // 0.4366979828269513
	fmt.Println(must(pragmastat.RatioBounds(x, y, 1e-3)))                     // {0.23255813953488377 0.6428571428571428}
	fmt.Println(must(pragmastat.Disparity(x, y)))                             // -2.2222222222222223
	fmt.Println(must(pragmastat.DisparityBoundsWithSeed(x, y, 1e-3, "demo"))) // {-29 -0.4782608695652174}

	// --- Randomization ---

	rng := pragmastat.NewRngFromString("demo-uniform")
	fmt.Println(rng.UniformFloat64()) // 0.2640554428629759
	fmt.Println(rng.UniformFloat64()) // 0.9348534835582796

	rng = pragmastat.NewRngFromString("demo-uniform-int")
	fmt.Println(rng.UniformInt64(0, 100)) // 41

	rng = pragmastat.NewRngFromString("demo-sample")
	fmt.Println(pragmastat.Sample(rng, []float64{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}, 3)) // [3 8 9]

	rng = pragmastat.NewRngFromString("demo-resample")
	fmt.Println(pragmastat.Resample(rng, []float64{1, 2, 3, 4, 5}, 7)) // [3 1 3 2 4 1 2]

	rng = pragmastat.NewRngFromString("demo-shuffle")
	fmt.Println(pragmastat.Shuffle(rng, []float64{1, 2, 3, 4, 5})) // [4 2 3 5 1]

	// --- Distributions ---

	rng = pragmastat.NewRngFromString("demo-dist-additive")
	fmt.Println(pragmastat.NewAdditive(0, 1).Sample(rng)) // 0.1741044867956819

	rng = pragmastat.NewRngFromString("demo-dist-multiplic")
	fmt.Println(pragmastat.NewMultiplic(0, 1).Sample(rng)) // 1.1273244602673853

	rng = pragmastat.NewRngFromString("demo-dist-exp")
	fmt.Println(pragmastat.NewExp(1).Sample(rng)) // 0.6589065267276553

	rng = pragmastat.NewRngFromString("demo-dist-power")
	fmt.Println(pragmastat.NewPower(1, 2).Sample(rng)) // 1.023677535537084

	rng = pragmastat.NewRngFromString("demo-dist-uniform")
	fmt.Println(pragmastat.NewUniform(0, 10).Sample(rng)) // 6.54043657816832
}
```
