package pragmastat

import (
	"math"
	"sort"
	"testing"
)

// floatEquals checks if two float64 values are approximately equal
func floatEquals(a, b, epsilon float64) bool {
	if math.IsInf(a, 1) && math.IsInf(b, 1) {
		return true
	}
	if math.IsInf(a, -1) && math.IsInf(b, -1) {
		return true
	}
	if math.IsNaN(a) && math.IsNaN(b) {
		return true
	}
	return math.Abs(a-b) < epsilon
}

const invarianceSeed int64 = 1729
const invarianceTolerance float64 = 1e-9

func uniformVec(rng *Rng, n int) []float64 {
	v := make([]float64, n)
	for i := range v {
		v[i] = rng.UniformFloat64()
	}
	return v
}

func addScalar(x []float64, c float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v + c
	}
	return result
}

func mulScalar(x []float64, c float64) []float64 {
	result := make([]float64, len(x))
	for i, v := range x {
		result[i] = v * c
	}
	return result
}

func mustVal(m Measurement, err error) float64 {
	if err != nil {
		panic(err)
	}
	return m.Value
}

func mustSampleOf(x []float64) *Sample {
	s, err := NewSample(x)
	if err != nil {
		panic(err)
	}
	return s
}

func mustSampleOfY(x []float64) *Sample {
	s, err := newSample(x, nil, nil, SubjectY)
	if err != nil {
		panic(err)
	}
	return s
}

// performTestOne tests a one-sample invariance property across sizes 2-10
func performTestOne(t *testing.T, expr1 func([]float64) float64, expr2 func([]float64) float64) {
	t.Helper()
	rng := NewRngFromSeed(invarianceSeed)
	for n := 2; n <= 10; n++ {
		x := uniformVec(rng, n)
		result1 := expr1(x)
		result2 := expr2(x)
		if !floatEquals(result1, result2, invarianceTolerance) {
			t.Errorf("Failed for n=%d: %v != %v", n, result1, result2)
		}
	}
}

// performTestTwo tests a two-sample invariance property across sizes 2-10
func performTestTwo(t *testing.T, expr1 func([]float64, []float64) float64, expr2 func([]float64, []float64) float64) {
	t.Helper()
	rng := NewRngFromSeed(invarianceSeed)
	for n := 2; n <= 10; n++ {
		x := uniformVec(rng, n)
		y := uniformVec(rng, n)
		result1 := expr1(x, y)
		result2 := expr2(x, y)
		if !floatEquals(result1, result2, invarianceTolerance) {
			t.Errorf("Failed for n=%d: %v != %v", n, result1, result2)
		}
	}
}

// Center invariance tests

func TestCenterShift(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(Center(mustSampleOf(addScalar(x, 2)))) },
		func(x []float64) float64 { return mustVal(Center(mustSampleOf(x))) + 2 },
	)
}

func TestCenterScale(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(Center(mustSampleOf(mulScalar(x, 2)))) },
		func(x []float64) float64 { return 2 * mustVal(Center(mustSampleOf(x))) },
	)
}

func TestCenterNegate(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(Center(mustSampleOf(mulScalar(x, -1)))) },
		func(x []float64) float64 { return -1 * mustVal(Center(mustSampleOf(x))) },
	)
}

// Spread invariance tests

func TestSpreadShift(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(Spread(mustSampleOf(addScalar(x, 2)))) },
		func(x []float64) float64 { return mustVal(Spread(mustSampleOf(x))) },
	)
}

func TestSpreadScale(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(Spread(mustSampleOf(mulScalar(x, 2)))) },
		func(x []float64) float64 { return 2 * mustVal(Spread(mustSampleOf(x))) },
	)
}

func TestSpreadNegate(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(Spread(mustSampleOf(mulScalar(x, -1)))) },
		func(x []float64) float64 { return mustVal(Spread(mustSampleOf(x))) },
	)
}

// RelSpread invariance tests

func TestRelSpreadScale(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(RelSpread(mustSampleOf(mulScalar(x, 2)))) },
		func(x []float64) float64 { return mustVal(RelSpread(mustSampleOf(x))) },
	)
}

// Shift invariance tests

func TestShiftShift(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(Shift(mustSampleOf(addScalar(x, 3)), mustSampleOfY(addScalar(y, 2))))
		},
		func(x, y []float64) float64 { return mustVal(Shift(mustSampleOf(x), mustSampleOfY(y))) + 1 },
	)
}

func TestShiftScale(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(Shift(mustSampleOf(mulScalar(x, 2)), mustSampleOfY(mulScalar(y, 2))))
		},
		func(x, y []float64) float64 { return 2 * mustVal(Shift(mustSampleOf(x), mustSampleOfY(y))) },
	)
}

func TestShiftAntisymmetry(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 { return mustVal(Shift(mustSampleOf(x), mustSampleOfY(y))) },
		func(x, y []float64) float64 { return -1 * mustVal(Shift(mustSampleOf(y), mustSampleOfY(x))) },
	)
}

// Ratio invariance tests

func TestRatioScale(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(Ratio(mustSampleOf(mulScalar(x, 2)), mustSampleOfY(mulScalar(y, 3))))
		},
		func(x, y []float64) float64 { return (2.0 / 3) * mustVal(Ratio(mustSampleOf(x), mustSampleOfY(y))) },
	)
}

// AvgSpread invariance tests

func TestAvgSpreadEqual(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 { return mustVal(avgSpread(mustSampleOf(x), mustSampleOfY(x))) },
		func(x []float64) float64 { return mustVal(Spread(mustSampleOf(x))) },
	)
}

func TestAvgSpreadSymmetry(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 { return mustVal(avgSpread(mustSampleOf(x), mustSampleOfY(y))) },
		func(x, y []float64) float64 { return mustVal(avgSpread(mustSampleOf(y), mustSampleOfY(x))) },
	)
}

func TestAvgSpreadAverage(t *testing.T) {
	performTestOne(t,
		func(x []float64) float64 {
			return mustVal(avgSpread(mustSampleOf(x), mustSampleOfY(mulScalar(x, 5))))
		},
		func(x []float64) float64 { return 3 * mustVal(Spread(mustSampleOf(x))) },
	)
}

func TestAvgSpreadScale(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(avgSpread(mustSampleOf(mulScalar(x, -2)), mustSampleOfY(mulScalar(y, -2))))
		},
		func(x, y []float64) float64 { return 2 * mustVal(avgSpread(mustSampleOf(x), mustSampleOfY(y))) },
	)
}

// Disparity invariance tests

func TestDisparityShift(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(Disparity(mustSampleOf(addScalar(x, 2)), mustSampleOfY(addScalar(y, 2))))
		},
		func(x, y []float64) float64 { return mustVal(Disparity(mustSampleOf(x), mustSampleOfY(y))) },
	)
}

func TestDisparityScale(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(Disparity(mustSampleOf(mulScalar(x, 2)), mustSampleOfY(mulScalar(y, 2))))
		},
		func(x, y []float64) float64 { return mustVal(Disparity(mustSampleOf(x), mustSampleOfY(y))) },
	)
}

func TestDisparityScaleNeg(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 {
			return mustVal(Disparity(mustSampleOf(mulScalar(x, -2)), mustSampleOfY(mulScalar(y, -2))))
		},
		func(x, y []float64) float64 {
			return -1 * mustVal(Disparity(mustSampleOf(x), mustSampleOfY(y)))
		},
	)
}

func TestDisparityAntisymmetry(t *testing.T) {
	performTestTwo(t,
		func(x, y []float64) float64 { return mustVal(Disparity(mustSampleOf(x), mustSampleOfY(y))) },
		func(x, y []float64) float64 {
			return -1 * mustVal(Disparity(mustSampleOf(y), mustSampleOfY(x)))
		},
	)
}

// Randomization invariance tests

func TestShuffleInvariance(t *testing.T) {
	t.Run("preserves multiset", func(t *testing.T) {
		for _, n := range []int{1, 2, 5, 10, 100} {
			x := make([]float64, n)
			for i := range x {
				x[i] = float64(i)
			}
			rng := NewRngFromSeed(42)
			shuffled := RngShuffle(rng, x)
			sortedShuffled := make([]float64, len(shuffled))
			copy(sortedShuffled, shuffled)
			sort.Float64s(sortedShuffled)
			for i, v := range x {
				if sortedShuffled[i] != v {
					t.Errorf("n=%d: sorted shuffle mismatch at %d", n, i)
				}
			}
		}
	})
}

func TestSampleInvariance(t *testing.T) {
	x := []float64{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}

	t.Run("correct size", func(t *testing.T) {
		for _, k := range []int{1, 5, 10} {
			rng := NewRngFromSeed(42)
			sampled := RngSample(rng, x, k)
			expectedLen := k
			if k > len(x) {
				expectedLen = len(x)
			}
			if len(sampled) != expectedLen {
				t.Errorf("k=%d: expected len %d, got %d", k, expectedLen, len(sampled))
			}
		}
	})

	t.Run("elements from source", func(t *testing.T) {
		rng := NewRngFromSeed(42)
		sampled := RngSample(rng, x, 5)
		xSet := make(map[float64]bool)
		for _, v := range x {
			xSet[v] = true
		}
		for i, v := range sampled {
			if !xSet[v] {
				t.Errorf("sampled[%d]=%v not in source", i, v)
			}
		}
	})

	t.Run("preserves order", func(t *testing.T) {
		rng := NewRngFromSeed(42)
		sampled := RngSample(rng, x, 5)
		for i := 1; i < len(sampled); i++ {
			if sampled[i] <= sampled[i-1] {
				t.Errorf("order violated: sampled[%d]=%v <= sampled[%d]=%v",
					i, sampled[i], i-1, sampled[i-1])
			}
		}
	})

	t.Run("no duplicates", func(t *testing.T) {
		for _, n := range []int{2, 3, 5, 10, 20} {
			source := make([]float64, n)
			for i := range source {
				source[i] = float64(i)
			}
			for _, k := range []int{1, n / 2, n} {
				rng := NewRngFromSeed(42)
				sampled := RngSample(rng, source, k)
				seen := make(map[float64]bool)
				for _, v := range sampled {
					if seen[v] {
						t.Errorf("n=%d, k=%d: duplicate element %v", n, k, v)
					}
					seen[v] = true
				}
			}
		}
	})
}

func TestResampleInvariance(t *testing.T) {
	x := []float64{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}

	t.Run("elements from source", func(t *testing.T) {
		rng := NewRngFromSeed(42)
		resampled := RngResample(rng, x, 20)
		xSet := make(map[float64]bool)
		for _, v := range x {
			xSet[v] = true
		}
		for i, v := range resampled {
			if !xSet[v] {
				t.Errorf("resampled[%d]=%v not in source", i, v)
			}
		}
	})

	t.Run("k0 panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Errorf("RngResample with k=0 should panic")
			}
		}()
		rng := NewRngFromSeed(42)
		RngResample(rng, x, 0)
	})
}

func TestResampleNegativeKPanics(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("RngResample with negative k should panic")
		}
	}()
	rng := NewRngFromString("test-resample-validation")
	RngResample(rng, []float64{1, 2, 3}, -1)
}

func TestShuffleEmptyPanics(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("RngShuffle with empty slice should panic")
		}
	}()
	rng := NewRngFromSeed(42)
	RngShuffle(rng, []float64{})
}

func TestSampleK0Panics(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("RngSample with k=0 should panic")
		}
	}()
	rng := NewRngFromSeed(42)
	RngSample(rng, []float64{1, 2, 3}, 0)
}

func TestSampleEmptyPanics(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("RngSample with empty slice should panic")
		}
	}()
	rng := NewRngFromSeed(42)
	RngSample(rng, []float64{}, 1)
}
