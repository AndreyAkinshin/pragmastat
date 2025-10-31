package pragmastat

import (
	"math"
	"testing"
	"time"
)

// TestCenterPerformance validates the fast O(n log n) algorithm as specified in tests.md
func TestCenterPerformance(t *testing.T) {
	n := 100000
	x := make([]float64, n)
	for i := 0; i < n; i++ {
		x[i] = float64(i + 1)
	}

	start := time.Now()
	result, err := fastCenter(x)
	elapsed := time.Since(start)

	if err != nil {
		t.Fatalf("fastCenter failed: %v", err)
	}

	expected := 50000.5
	if math.Abs(result-expected) > 1e-9 {
		t.Errorf("Center for n=%d: expected %.1f, got %.6f", n, expected, result)
	}

	t.Logf("Center for n=%d: %.6f", n, result)
	t.Logf("Elapsed time: %v", elapsed)

	if elapsed > 5*time.Second {
		t.Errorf("Performance too slow: %v", elapsed)
	}
}

// TestSpreadPerformance validates the fast O(n log n) algorithm as specified in tests.md
func TestSpreadPerformance(t *testing.T) {
	n := 100000
	x := make([]float64, n)
	for i := 0; i < n; i++ {
		x[i] = float64(i + 1)
	}

	start := time.Now()
	result, err := fastSpread(x)
	elapsed := time.Since(start)

	if err != nil {
		t.Fatalf("fastSpread failed: %v", err)
	}

	expected := 29290.0
	if math.Abs(result-expected) > 1e-9 {
		t.Errorf("Spread for n=%d: expected %.0f, got %.6f", n, expected, result)
	}

	t.Logf("Spread for n=%d: %.6f", n, result)
	t.Logf("Elapsed time: %v", elapsed)

	if elapsed > 5*time.Second {
		t.Errorf("Performance too slow: %v", elapsed)
	}
}

// TestShiftPerformance validates the fast O((m+n) log L) binary search algorithm as specified in tests.md
func TestShiftPerformance(t *testing.T) {
	n := 100000
	x := make([]float64, n)
	y := make([]float64, n)
	for i := 0; i < n; i++ {
		x[i] = float64(i + 1)
		y[i] = float64(i + 1)
	}

	start := time.Now()
	result, err := fastShift(x, y)
	elapsed := time.Since(start)

	if err != nil {
		t.Fatalf("fastShift failed: %v", err)
	}

	expected := 0.0
	if math.Abs(result-expected) > 1e-9 {
		t.Errorf("Shift for n=m=%d: expected %.0f, got %.6f", n, expected, result)
	}

	t.Logf("Shift for n=m=%d: %.6f", n, result)
	t.Logf("Elapsed time: %v", elapsed)

	if elapsed > 5*time.Second {
		t.Errorf("Performance too slow: %v", elapsed)
	}
}
