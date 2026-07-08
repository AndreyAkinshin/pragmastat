package pragmastat

import (
	"math"
	"testing"
)

// Regression: search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to +-Inf on
// finite extreme input, turning the midpoint into NaN and returning +-Inf instead
// of the true finite shift.
func TestShift_Overflow(t *testing.T) {
	max := math.MaxFloat64
	tests := []struct {
		name string
		x, y []float64
		want float64
	}{
		{"symmetric extremes", []float64{-max, max}, []float64{-max, max}, 0},
		{"one-sided extremes", []float64{0, max}, []float64{-max, 0}, max},
	}
	for _, tt := range tests {
		got, err := Shift(tt.x, tt.y, true)
		if err != nil {
			t.Fatalf("%s: unexpected error: %v", tt.name, err)
		}
		if got != tt.want {
			t.Errorf("%s: Shift = %v, want %v", tt.name, got, tt.want)
		}
	}
}
