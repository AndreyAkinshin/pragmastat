# Go

Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v12@v12.0.1
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v12.0.1/go

## Demo

```go
package main

import (
	"fmt"
	"log"

	pragmastat "github.com/AndreyAkinshin/pragmastat/go/v12"
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

	xVals := make([]float64, 200)
	for i := range xVals {
		xVals[i] = float64(i + 1)
	}
	x := mustS(pragmastat.NewSample(xVals))

	fmt.Println(mustM(pragmastat.Center(x)).Value)                       // 100.5
	fmt.Println(mustB(pragmastat.CenterBounds(x, 1e-3)))                 // [86;115]
	fmt.Println(mustM(pragmastat.Spread(x)).Value)                       // 59
	fmt.Println(mustB(pragmastat.SpreadBoundsWithSeed(x, 1e-3, "demo"))) // [44;87]

	// --- Two-Sample ---

	sxVals := make([]float64, 200)
	for i := range sxVals {
		sxVals[i] = float64(i + 1)
	}
	syVals := make([]float64, 200)
	for i := range syVals {
		syVals[i] = float64(i + 101)
	}
	sx := mustS(pragmastat.NewSample(sxVals))
	sy := mustS(pragmastat.NewSample(syVals))

	fmt.Println(mustM(pragmastat.Shift(sx, sy)).Value)                           // -100
	fmt.Println(mustB(pragmastat.ShiftBounds(sx, sy, 1e-3)))                     // [-120;-80]
	fmt.Println(mustM(pragmastat.Ratio(sx, sy)).Value)                           // 0.5008354224706336
	fmt.Println(mustB(pragmastat.RatioBounds(sx, sy, 1e-3)))                     // [0.4066666666666668;0.5958333333333332]
	fmt.Println(mustM(pragmastat.Disparity(sx, sy)).Value)                       // -1.694915254237288
	fmt.Println(mustB(pragmastat.DisparityBoundsWithSeed(sx, sy, 1e-3, "demo"))) // [-3.1025641025641026;-0.8494623655913979]

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
```
