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

	rng := pragmastat.NewRngFromString("demo-uniform")
	fmt.Println(rng.Uniform()) // 0.2640554428629759
	fmt.Println(rng.Uniform()) // 0.9348534835582796

	rng = pragmastat.NewRngFromString("demo-sample")
	fmt.Println(pragmastat.Sample(rng, []float64{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}, 3)) // [3 8 9]

	rng = pragmastat.NewRngFromString("demo-shuffle")
	fmt.Println(pragmastat.Shuffle(rng, []float64{1, 2, 3, 4, 5})) // [4 2 3 5 1]

	rng = pragmastat.NewRngFromString("demo-resample")
	fmt.Println(pragmastat.Resample(rng, []float64{1, 2, 3, 4, 5}, 7)) // [5 1 1 3 3 4 5]

	// --- Distribution Sampling ---

	rng = pragmastat.NewRngFromString("demo-dist-uniform")
	dist := pragmastat.NewUniform(0, 10)
	fmt.Println(dist.Sample(rng)) // 6.54043657816832

	rng = pragmastat.NewRngFromString("demo-dist-additive")
	addDist := pragmastat.NewAdditive(0, 1)
	fmt.Println(addDist.Sample(rng)) // 0.17410448679568188

	rng = pragmastat.NewRngFromString("demo-dist-exp")
	expDist := pragmastat.NewExp(1)
	fmt.Println(expDist.Sample(rng)) // 0.6589065267276553

	rng = pragmastat.NewRngFromString("demo-dist-power")
	powDist := pragmastat.NewPower(1, 2)
	fmt.Println(powDist.Sample(rng)) // 1.023677535537084

	rng = pragmastat.NewRngFromString("demo-dist-multiplic")
	mulDist := pragmastat.NewMultiplic(0, 1)
	fmt.Println(mulDist.Sample(rng)) // 1.1273244602673853

	// --- Single-Sample Statistics ---

	x := []float64{1, 3, 5, 7, 9}

	print(pragmastat.Center(x))              // 5
	print(pragmastat.Spread(x))              // 4
	print(pragmastat.Spread(add(x, 10)))     // 4
	print(pragmastat.Spread(multiply(x, 2))) // 8
	print(pragmastat.RelSpread(x))           // 0.8

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
	print(pragmastat.Ratio(y, x)) // 2

	// --- One-Sample Bounds ---

	x = []float64{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

	print(pragmastat.Center(x))                         // 5.5
	fmt.Println(must(pragmastat.CenterBounds(x, 0.05))) // {Lower: 3.5, Upper: 7.5}

	// --- Two-Sample Bounds ---

	x = []float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
		16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30}
	y = []float64{
		21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
		36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50}

	print(pragmastat.Shift(x, y))                         // -20
	fmt.Println(must(pragmastat.ShiftBounds(x, y, 1e-4))) // {Lower: -30, Upper: -10}

	x = []float64{1, 2, 3, 4, 5}
	y = []float64{2, 3, 4, 5, 6}
	fmt.Println(must(pragmastat.RatioBounds(x, y, 0.05))) // {Lower: 0.333..., Upper: 1.5}
}
