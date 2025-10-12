package pragmastat

import (
	"math"
	"math/rand"
	"sort"
	"testing"
)

func naiveShift[T Number](x, y []T) float64 {
	var diffs []float64
	for _, xi := range x {
		for _, yj := range y {
			diffs = append(diffs, float64(xi-yj))
		}
	}
	sort.Float64s(diffs)

	n := len(diffs)
	if n%2 == 0 {
		return (diffs[n/2-1] + diffs[n/2]) / 2.0
	}
	return diffs[n/2]
}

func TestFastShiftSmallArrays(t *testing.T) {
	const tolerance = 1e-9
	rng := rand.New(rand.NewSource(1729))

	for m := 1; m <= 20; m++ {
		for n := 1; n <= 20; n++ {
			for iteration := 0; iteration < 3; iteration++ {
				x := make([]float64, m)
				y := make([]float64, n)
				for i := range x {
					x[i] = rng.NormFloat64()
				}
				for i := range y {
					y[i] = rng.NormFloat64()
				}

				actual, err := fastShift(x, y)
				if err != nil {
					t.Errorf("fastShift(%d, %d) error: %v", m, n, err)
					continue
				}

				expected := naiveShift(x, y)
				if math.Abs(actual-expected) > tolerance {
					t.Errorf("fastShift(%d, %d) = %v, want %v", m, n, actual, expected)
				}
			}
		}
	}
}

func TestFastShiftMediumArrays(t *testing.T) {
	const tolerance = 1e-9
	rng := rand.New(rand.NewSource(42))

	for size := 20; size <= 100; size += 10 {
		for iteration := 0; iteration < 3; iteration++ {
			x := make([]float64, size)
			y := make([]float64, size/2)
			for i := range x {
				x[i] = rng.NormFloat64()
			}
			for i := range y {
				y[i] = rng.NormFloat64()
			}

			actual, err := fastShift(x, y)
			if err != nil {
				t.Errorf("fastShift(%d, %d) error: %v", size, size/2, err)
				continue
			}

			expected := naiveShift(x, y)
			if math.Abs(actual-expected) > tolerance {
				t.Errorf("fastShift(%d, %d) = %v, want %v", size, size/2, actual, expected)
			}
		}
	}
}

func TestFastShiftDifferentDistributions(t *testing.T) {
	const tolerance = 1e-9
	seed := int64(2024)

	type distribution struct {
		name  string
		mean  float64
		scale float64
	}

	distributions := []distribution{
		{"standard", 0.0, 1.0},
		{"shifted", 5.0, 2.0},
		{"negative", -10.0, 1.0},
		{"small", 0.0, 0.1},
	}

	for _, dist := range distributions {
		t.Run(dist.name, func(t *testing.T) {
			rng := rand.New(rand.NewSource(seed))
			seed++

			for trial := 0; trial < 10; trial++ {
				x := make([]float64, 15)
				y := make([]float64, 10)
				for i := range x {
					x[i] = dist.mean + dist.scale*rng.NormFloat64()
				}
				for i := range y {
					y[i] = dist.mean + dist.scale*rng.NormFloat64()
				}

				actual, err := fastShift(x, y)
				if err != nil {
					t.Errorf("fastShift error: %v", err)
					continue
				}

				expected := naiveShift(x, y)
				if math.Abs(actual-expected) > tolerance {
					t.Errorf("fastShift = %v, want %v", actual, expected)
				}
			}
		})
	}
}

func TestFastShiftSingleElement(t *testing.T) {
	const tolerance = 1e-9
	rng := rand.New(rand.NewSource(123))

	for trial := 0; trial < 20; trial++ {
		x := []float64{rng.NormFloat64()}
		y := []float64{rng.NormFloat64()}

		result, err := fastShift(x, y)
		if err != nil {
			t.Errorf("fastShift single element error: %v", err)
			continue
		}

		expected := x[0] - y[0]
		if math.Abs(result-expected) > tolerance {
			t.Errorf("fastShift([%v], [%v]) = %v, want %v", x[0], y[0], result, expected)
		}
	}
}

func TestFastShiftIdenticalArrays(t *testing.T) {
	const tolerance = 1e-9
	rng := rand.New(rand.NewSource(456))

	for size := 1; size <= 30; size++ {
		for trial := 0; trial < 3; trial++ {
			x := make([]float64, size)
			for i := range x {
				x[i] = rng.NormFloat64()
			}

			result, err := fastShift(x, x)
			if err != nil {
				t.Errorf("fastShift identical arrays error: %v", err)
				continue
			}

			if math.Abs(result) > tolerance {
				t.Errorf("fastShift(x, x) = %v, want 0.0", result)
			}
		}
	}
}

func TestFastShiftAsymmetricSizes(t *testing.T) {
	const tolerance = 1e-9
	rng := rand.New(rand.NewSource(789))

	configs := []struct{ m, n int }{
		{1, 100},
		{100, 1},
		{10, 50},
		{50, 10},
		{5, 200},
	}

	for _, cfg := range configs {
		t.Run("", func(t *testing.T) {
			x := make([]float64, cfg.m)
			y := make([]float64, cfg.n)
			for i := range x {
				x[i] = rng.NormFloat64()
			}
			for i := range y {
				y[i] = rng.NormFloat64()
			}

			actual, err := fastShift(x, y)
			if err != nil {
				t.Errorf("fastShift(%d, %d) error: %v", cfg.m, cfg.n, err)
				return
			}

			expected := naiveShift(x, y)
			if math.Abs(actual-expected) > tolerance {
				t.Errorf("fastShift(%d, %d) = %v, want %v", cfg.m, cfg.n, actual, expected)
			}
		})
	}
}

func TestFastShiftExtremeValues(t *testing.T) {
	const tolerance = 1e-9

	tests := []struct {
		name string
		x    []float64
		y    []float64
	}{
		{"large values", []float64{1e6, 2e6, 3e6}, []float64{1e5, 2e5}},
		{"small values", []float64{1e-8, 2e-8, 3e-8}, []float64{1e-9, 2e-9}},
		{"negative values", []float64{-100, -50, -10}, []float64{-80, -40}},
		{"mixed values", []float64{-100, 0, 100}, []float64{-50, 50}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			actual, err := fastShift(tt.x, tt.y)
			if err != nil {
				t.Errorf("fastShift error: %v", err)
				return
			}

			expected := naiveShift(tt.x, tt.y)
			if math.Abs(actual-expected) > tolerance {
				t.Errorf("fastShift = %v, want %v", actual, expected)
			}

			if math.IsNaN(actual) {
				t.Errorf("fastShift returned NaN")
			}
			if math.IsInf(actual, 0) {
				t.Errorf("fastShift returned Inf")
			}
		})
	}
}

func TestFastShiftDuplicateValues(t *testing.T) {
	const tolerance = 1e-9
	rng := rand.New(rand.NewSource(222))

	for trial := 0; trial < 10; trial++ {
		x := make([]float64, 12)
		y := make([]float64, 10)
		for i := range x {
			x[i] = math.Round(rng.NormFloat64()*5) / 5.0
		}
		for i := range y {
			y[i] = math.Round(rng.NormFloat64()*5) / 5.0
		}

		actual, err := fastShift(x, y)
		if err != nil {
			t.Errorf("fastShift with duplicates error: %v", err)
			continue
		}

		expected := naiveShift(x, y)
		if math.Abs(actual-expected) > tolerance {
			t.Errorf("fastShift = %v, want %v", actual, expected)
		}
	}
}

func TestFastShiftZeroSpread(t *testing.T) {
	const tolerance = 1e-9

	x := []float64{5.0, 5.0, 5.0, 5.0, 5.0}
	y := []float64{2.0, 2.0, 2.0, 2.0}

	result, err := fastShift(x, y)
	if err != nil {
		t.Errorf("fastShift zero spread error: %v", err)
		return
	}

	expected := 3.0
	if math.Abs(result-expected) > tolerance {
		t.Errorf("fastShift = %v, want %v", result, expected)
	}
}

func TestFastShiftShiftInvariance(t *testing.T) {
	const tolerance = 1e-6
	rng := rand.New(rand.NewSource(555))

	for trial := 0; trial < 10; trial++ {
		x := make([]float64, 15)
		y := make([]float64, 12)
		for i := range x {
			x[i] = rng.NormFloat64()
		}
		for i := range y {
			y[i] = rng.NormFloat64()
		}
		shift := rng.NormFloat64() * 10

		result1, _ := fastShift(x, y)

		xShifted := make([]float64, len(x))
		for i := range x {
			xShifted[i] = x[i] + shift
		}
		result2, _ := fastShift(xShifted, y)

		if math.Abs(result2-(result1+shift)) > tolerance {
			t.Errorf("X shift invariance failed: got %v, want %v", result2, result1+shift)
		}

		yShifted := make([]float64, len(y))
		for i := range y {
			yShifted[i] = y[i] + shift
		}
		result3, _ := fastShift(x, yShifted)

		if math.Abs(result3-(result1-shift)) > tolerance {
			t.Errorf("Y shift invariance failed: got %v, want %v", result3, result1-shift)
		}
	}
}

func TestFastShiftScaleInvariance(t *testing.T) {
	const tolerance = 1e-6
	rng := rand.New(rand.NewSource(777))

	for trial := 0; trial < 10; trial++ {
		x := make([]float64, 15)
		y := make([]float64, 12)
		for i := range x {
			x[i] = rng.NormFloat64()
		}
		for i := range y {
			y[i] = rng.NormFloat64()
		}

		result1, _ := fastShift(x, y)

		scale := 2.0
		xScaled := make([]float64, len(x))
		yScaled := make([]float64, len(y))
		for i := range x {
			xScaled[i] = x[i] * scale
		}
		for i := range y {
			yScaled[i] = y[i] * scale
		}

		result2, _ := fastShift(xScaled, yScaled)

		if math.Abs(result2-result1*scale) > tolerance {
			t.Errorf("Scale invariance failed: got %v, want %v", result2, result1*scale)
		}
	}
}

func TestFastShiftErrorHandling(t *testing.T) {
	tests := []struct {
		name string
		x    []float64
		y    []float64
	}{
		{"empty x", []float64{}, []float64{1.0}},
		{"empty y", []float64{1.0}, []float64{}},
		{"both empty", []float64{}, []float64{}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := fastShift(tt.x, tt.y)
			if err == nil {
				t.Errorf("fastShift(%v, %v) expected error, got nil", tt.x, tt.y)
			}
		})
	}
}

func TestFastShiftIntegerInputs(t *testing.T) {
	const tolerance = 1e-9

	tests := []struct {
		name string
		x    []int
		y    []int
	}{
		{"simple", []int{3, 5, 7}, []int{1, 2}},
		{"negative", []int{-5, -3, -1}, []int{1, 2}},
		{"single", []int{10}, []int{3}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := fastShift(tt.x, tt.y)
			if err != nil {
				t.Errorf("fastShift error: %v", err)
				return
			}

			expected := naiveShift(tt.x, tt.y)
			if math.Abs(result-expected) > tolerance {
				t.Errorf("fastShift(%v, %v) = %v, want %v", tt.x, tt.y, result, expected)
			}
		})
	}
}

func BenchmarkFastShift(b *testing.B) {
	rng := rand.New(rand.NewSource(1729))

	sizes := []struct{ m, n int }{
		{100, 100},
		{200, 200},
		{500, 500},
		{1000, 1000},
	}

	for _, size := range sizes {
		x := make([]float64, size.m)
		y := make([]float64, size.n)
		for i := range x {
			x[i] = rng.NormFloat64()
		}
		for i := range y {
			y[i] = rng.NormFloat64()
		}

		b.Run("", func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				_, _ = fastShift(x, y)
			}
		})
	}
}

func BenchmarkNaiveShift(b *testing.B) {
	rng := rand.New(rand.NewSource(1729))

	sizes := []struct{ m, n int }{
		{100, 100},
		{200, 200},
	}

	for _, size := range sizes {
		x := make([]float64, size.m)
		y := make([]float64, size.n)
		for i := range x {
			x[i] = rng.NormFloat64()
		}
		for i := range y {
			y[i] = rng.NormFloat64()
		}

		b.Run("", func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				_ = naiveShift(x, y)
			}
		})
	}
}
