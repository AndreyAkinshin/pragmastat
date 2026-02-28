package main

import (
	"fmt"
	"log"

	pragmastat "github.com/AndreyAkinshin/pragmastat/go/v10"
)

func mustM(val pragmastat.Measurement, err error) pragmastat.Measurement {
	if err != nil {
		log.Fatal(err)
	}
	return val
}

func mustB(val pragmastat.Bounds, err error) pragmastat.Bounds {
	if err != nil {
		log.Fatal(err)
	}
	return val
}

func mustS(val *pragmastat.Sample, err error) *pragmastat.Sample {
	if err != nil {
		log.Fatal(err)
	}
	return val
}

func main() {
	// --- One-Sample ---

	x := mustS(pragmastat.NewSample([]float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
		11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
		21, 22}))

	fmt.Println(mustM(pragmastat.Center(x)).Value)                       // 11.5
	fmt.Println(mustB(pragmastat.CenterBounds(x, 1e-3)))                 // [6;17]
	fmt.Println(mustM(pragmastat.Spread(x)).Value)                       // 7
	fmt.Println(mustB(pragmastat.SpreadBoundsWithSeed(x, 1e-3, "demo"))) // [1;18]

	// --- Two-Sample ---

	sx := mustS(pragmastat.NewSample([]float64{
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
		16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30}))
	sy := mustS(pragmastat.NewSample([]float64{
		21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
		36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50}))

	fmt.Println(mustM(pragmastat.Shift(sx, sy)).Value)                           // -20
	fmt.Println(mustB(pragmastat.ShiftBounds(sx, sy, 1e-3)))                     // [-28;-12]
	fmt.Println(mustM(pragmastat.Ratio(sx, sy)).Value)                           // 0.4366979828269513
	fmt.Println(mustB(pragmastat.RatioBounds(sx, sy, 1e-3)))                     // [0.23...;0.64...]
	fmt.Println(mustM(pragmastat.Disparity(sx, sy)).Value)                       // -2.2222222222222223
	fmt.Println(mustB(pragmastat.DisparityBoundsWithSeed(sx, sy, 1e-3, "demo"))) // [-29;-0.47...]

	// --- Randomization ---

	rng := pragmastat.NewRngFromString("demo-uniform")
	fmt.Println(rng.UniformFloat64()) // 0.2640554428629759
	fmt.Println(rng.UniformFloat64()) // 0.9348534835582796

	rng = pragmastat.NewRngFromString("demo-uniform-int")
	fmt.Println(rng.UniformInt64(0, 100)) // 41

	rng = pragmastat.NewRngFromString("demo-sample")
	fmt.Println(pragmastat.RngSample(rng, []float64{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}, 3)) // [3 8 9]

	rng = pragmastat.NewRngFromString("demo-resample")
	fmt.Println(pragmastat.RngResample(rng, []float64{1, 2, 3, 4, 5}, 7)) // [3 1 3 2 4 1 2]

	rng = pragmastat.NewRngFromString("demo-shuffle")
	fmt.Println(pragmastat.RngShuffle(rng, []float64{1, 2, 3, 4, 5})) // [4 2 3 5 1]

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
