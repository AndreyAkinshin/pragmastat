package pragmastat

import (
	"math"
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

func TestMedian(t *testing.T) {
	tests := []struct {
		name     string
		input    []float64
		expected float64
	}{
		{"empty", []float64{}, 0.0},
		{"single", []float64{1.0}, 1.0},
		{"two elements", []float64{1.0, 2.0}, 1.5},
		{"three elements", []float64{1.0, 2.0, 3.0}, 2.0},
		{"four elements", []float64{1.0, 2.0, 3.0, 4.0}, 2.5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "empty" {
				_, err := median(tt.input)
				if err == nil {
					t.Errorf("median(%v) expected error, got nil", tt.input)
				}
			} else {
				result, err := median(tt.input)
				if err != nil {
					t.Errorf("median(%v) unexpected error: %v", tt.input, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("median(%v) = %v, want %v", tt.input, result, tt.expected)
				}
			}
		})
	}
}

func TestCenter(t *testing.T) {
	tests := []struct {
		name     string
		input    []float64
		expected float64
	}{
		{"empty", []float64{}, 0.0},
		{"single", []float64{1.0}, 1.0},
		{"two elements", []float64{1.0, 3.0}, 2.0},
		{"three elements", []float64{1.0, 2.0, 3.0}, 2.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "empty" {
				_, err := Center(tt.input)
				if err == nil {
					t.Errorf("Center(%v) expected error, got nil", tt.input)
				}
			} else {
				result, err := Center(tt.input)
				if err != nil {
					t.Errorf("Center(%v) unexpected error: %v", tt.input, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("Center(%v) = %v, want %v", tt.input, result, tt.expected)
				}
			}
		})
	}
}

func TestSpread(t *testing.T) {
	tests := []struct {
		name     string
		input    []float64
		expected float64
	}{
		{"empty", []float64{}, 0.0},
		{"single", []float64{1.0}, 0.0},
		{"two elements", []float64{1.0, 3.0}, 2.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "empty" {
				_, err := Spread(tt.input)
				if err == nil {
					t.Errorf("Spread(%v) expected error, got nil", tt.input)
				}
			} else {
				result, err := Spread(tt.input)
				if err != nil {
					t.Errorf("Spread(%v) unexpected error: %v", tt.input, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("Spread(%v) = %v, want %v", tt.input, result, tt.expected)
				}
			}
		})
	}
}

func TestRelSpread(t *testing.T) {
	tests := []struct {
		name     string
		input    []float64
		expected float64
	}{
		{"single non-zero", []float64{2.0}, 0.0},
		{"zeros", []float64{0.0, 0.0}, math.Inf(1)},
		{"normal case", []float64{10.0, 20.0}, 10.0 / 15.0}, // Spread([10,20]) = 10, Center([10,20]) = 15, RelSpread = 10/15
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "zeros" {
				_, err := RelSpread(tt.input)
				if err == nil {
					t.Errorf("RelSpread(%v) expected error, got nil", tt.input)
				}
			} else {
				result, err := RelSpread(tt.input)
				if err != nil {
					t.Errorf("RelSpread(%v) unexpected error: %v", tt.input, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("RelSpread(%v) = %v, want %v", tt.input, result, tt.expected)
				}
			}
		})
	}
}

func TestShift(t *testing.T) {
	tests := []struct {
		name     string
		x        []float64
		y        []float64
		expected float64
	}{
		{"empty x", []float64{}, []float64{1.0}, 0.0},
		{"empty y", []float64{1.0}, []float64{}, 0.0},
		{"single elements", []float64{3.0}, []float64{1.0}, 2.0},
		{"opposite", []float64{1.0}, []float64{3.0}, -2.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "empty x" || tt.name == "empty y" {
				_, err := Shift(tt.x, tt.y)
				if err == nil {
					t.Errorf("Shift(%v, %v) expected error, got nil", tt.x, tt.y)
				}
			} else {
				result, err := Shift(tt.x, tt.y)
				if err != nil {
					t.Errorf("Shift(%v, %v) unexpected error: %v", tt.x, tt.y, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("Shift(%v, %v) = %v, want %v", tt.x, tt.y, result, tt.expected)
				}
			}
		})
	}
}

func TestRatio(t *testing.T) {
	tests := []struct {
		name     string
		x        []float64
		y        []float64
		expected float64
	}{
		{"empty x", []float64{}, []float64{1.0}, 1.0},
		{"empty y", []float64{1.0}, []float64{}, 1.0},
		{"single elements", []float64{4.0}, []float64{2.0}, 2.0},
		{"opposite", []float64{2.0}, []float64{4.0}, 0.5},
		{"zero divisor", []float64{2.0}, []float64{0.0}, 1.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "empty x" || tt.name == "empty y" || tt.name == "zero divisor" {
				_, err := Ratio(tt.x, tt.y)
				if err == nil {
					t.Errorf("Ratio(%v, %v) expected error, got nil", tt.x, tt.y)
				}
			} else {
				result, err := Ratio(tt.x, tt.y)
				if err != nil {
					t.Errorf("Ratio(%v, %v) unexpected error: %v", tt.x, tt.y, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("Ratio(%v, %v) = %v, want %v", tt.x, tt.y, result, tt.expected)
				}
			}
		})
	}
}

func TestAvgSpread(t *testing.T) {
	tests := []struct {
		name     string
		x        []float64
		y        []float64
		expected float64
	}{
		{"both empty", []float64{}, []float64{}, 0.0},
		{"x empty", []float64{}, []float64{1.0, 3.0}, 2.0},
		{"y empty", []float64{1.0, 3.0}, []float64{}, 2.0},
		{"equal sizes", []float64{1.0, 3.0}, []float64{5.0, 9.0}, 3.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "both empty" || tt.name == "x empty" || tt.name == "y empty" {
				_, err := AvgSpread(tt.x, tt.y)
				if err == nil {
					t.Errorf("AvgSpread(%v, %v) expected error, got nil", tt.x, tt.y)
				}
			} else {
				result, err := AvgSpread(tt.x, tt.y)
				if err != nil {
					t.Errorf("AvgSpread(%v, %v) unexpected error: %v", tt.x, tt.y, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("AvgSpread(%v, %v) = %v, want %v", tt.x, tt.y, result, tt.expected)
				}
			}
		})
	}
}

func TestDisparity(t *testing.T) {
	tests := []struct {
		name     string
		x        []float64
		y        []float64
		expected float64
	}{
		{"both empty", []float64{}, []float64{}, math.Inf(1)},
		{"no difference", []float64{1.0, 2.0}, []float64{1.0, 2.0}, 0.0},
		{"normal case", []float64{3.0, 5.0}, []float64{1.0, 3.0}, 1.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "both empty" {
				_, err := Disparity(tt.x, tt.y)
				if err == nil {
					t.Errorf("Disparity(%v, %v) expected error, got nil", tt.x, tt.y)
				}
			} else {
				result, err := Disparity(tt.x, tt.y)
				if err != nil {
					t.Errorf("Disparity(%v, %v) unexpected error: %v", tt.x, tt.y, err)
				}
				if !floatEquals(result, tt.expected, 1e-10) {
					t.Errorf("Disparity(%v, %v) = %v, want %v", tt.x, tt.y, result, tt.expected)
				}
			}
		})
	}
}
