package pragmastat

import (
	"math"
	"math/rand"
	"testing"
	"time"
)

// centerSimple is the simple O(n^2) implementation for comparison
func centerSimple(x []float64) float64 {
	n := len(x)
	var pairwiseAverages []float64
	for i := 0; i < n; i++ {
		for j := i; j < n; j++ {
			pairwiseAverages = append(pairwiseAverages, (x[i]+x[j])/2.0)
		}
	}
	result, _ := median(pairwiseAverages)
	return result
}

// spreadSimple is the simple O(n^2) implementation for comparison
func spreadSimple(x []float64) float64 {
	n := len(x)
	if n == 1 {
		return 0.0
	}
	var pairwiseDiffs []float64
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			pairwiseDiffs = append(pairwiseDiffs, math.Abs(x[i]-x[j]))
		}
	}
	result, _ := median(pairwiseDiffs)
	return result
}

func TestCenterCorrectness(t *testing.T) {
	rand.Seed(1729)
	for n := 1; n <= 100; n++ {
		for iter := 0; iter < n; iter++ {
			x := make([]float64, n)
			for i := 0; i < n; i++ {
				x[i] = rand.NormFloat64()
			}

			expected := centerSimple(x)
			actual, err := fastCenter(x)
			if err != nil {
				t.Fatalf("fastCenter failed for n=%d: %v", n, err)
			}

			if math.Abs(expected-actual) > 1e-9 {
				t.Errorf("Mismatch for n=%d: expected=%.10f, actual=%.10f", n, expected, actual)
			}
		}
	}
}

func TestSpreadCorrectness(t *testing.T) {
	rand.Seed(1729)
	for n := 1; n <= 100; n++ {
		for iter := 0; iter < n; iter++ {
			x := make([]float64, n)
			for i := 0; i < n; i++ {
				x[i] = rand.NormFloat64()
			}

			expected := spreadSimple(x)
			actual, err := fastSpread(x)
			if err != nil {
				t.Fatalf("fastSpread failed for n=%d: %v", n, err)
			}

			if math.Abs(expected-actual) > 1e-9 {
				t.Errorf("Mismatch for n=%d: expected=%.10f, actual=%.10f", n, expected, actual)
			}
		}
	}
}

func TestCenterPerformance(t *testing.T) {
	rand.Seed(1729)
	n := 100000
	x := make([]float64, n)
	for i := 0; i < n; i++ {
		x[i] = rand.NormFloat64()
	}

	start := time.Now()
	result, err := fastCenter(x)
	elapsed := time.Since(start)

	if err != nil {
		t.Fatalf("fastCenter failed: %v", err)
	}

	t.Logf("Center for n=%d: %.6f", n, result)
	t.Logf("Elapsed time: %v", elapsed)

	if elapsed > 5*time.Second {
		t.Errorf("Performance too slow: %v", elapsed)
	}
}

func TestSpreadPerformance(t *testing.T) {
	rand.Seed(1729)
	n := 100000
	x := make([]float64, n)
	for i := 0; i < n; i++ {
		x[i] = rand.NormFloat64()
	}

	start := time.Now()
	result, err := fastSpread(x)
	elapsed := time.Since(start)

	if err != nil {
		t.Fatalf("fastSpread failed: %v", err)
	}

	t.Logf("Spread for n=%d: %.6f", n, result)
	t.Logf("Elapsed time: %v", elapsed)

	if elapsed > 5*time.Second {
		t.Errorf("Performance too slow: %v", elapsed)
	}
}

func BenchmarkCenterN100(b *testing.B) {
	rand.Seed(1729)
	x := make([]float64, 100)
	for i := 0; i < 100; i++ {
		x[i] = rand.NormFloat64()
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fastCenter(x)
	}
}

func BenchmarkCenterN1000(b *testing.B) {
	rand.Seed(1729)
	x := make([]float64, 1000)
	for i := 0; i < 1000; i++ {
		x[i] = rand.NormFloat64()
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fastCenter(x)
	}
}

func BenchmarkCenterN10000(b *testing.B) {
	rand.Seed(1729)
	x := make([]float64, 10000)
	for i := 0; i < 10000; i++ {
		x[i] = rand.NormFloat64()
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fastCenter(x)
	}
}

func BenchmarkSpreadN100(b *testing.B) {
	rand.Seed(1729)
	x := make([]float64, 100)
	for i := 0; i < 100; i++ {
		x[i] = rand.NormFloat64()
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fastSpread(x)
	}
}

func BenchmarkSpreadN1000(b *testing.B) {
	rand.Seed(1729)
	x := make([]float64, 1000)
	for i := 0; i < 1000; i++ {
		x[i] = rand.NormFloat64()
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fastSpread(x)
	}
}

func BenchmarkSpreadN10000(b *testing.B) {
	rand.Seed(1729)
	x := make([]float64, 10000)
	for i := 0; i < 10000; i++ {
		x[i] = rand.NormFloat64()
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fastSpread(x)
	}
}
