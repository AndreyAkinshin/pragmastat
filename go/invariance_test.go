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

func TestCenterInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0, 4.0, 5.0}

	// Location invariance: Center(x + c) = Center(x) + c
	t.Run("location invariance", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(x))
		for i, v := range x {
			shifted[i] = v + c
		}

		original, err := Center(x)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := Center(shifted)
		if err != nil {
			t.Fatal(err)
		}
		expected := original + c

		if !floatEquals(shiftedResult, expected, 1e-10) {
			t.Errorf("Location invariance failed: Center(x+%v) = %v, expected %v", c, shiftedResult, expected)
		}
	})

	// Scale invariance: Center(c * x) = c * Center(x)
	t.Run("scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := Center(x)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Center(scaled)
		if err != nil {
			t.Fatal(err)
		}
		expected := original * c

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Scale invariance failed: Center(%v*x) = %v, expected %v", c, scaledResult, expected)
		}
	})
}

func TestSpreadInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0, 4.0, 5.0}

	// Location invariance: Spread(x + c) = Spread(x)
	t.Run("location invariance", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(x))
		for i, v := range x {
			shifted[i] = v + c
		}

		original, err := Spread(x)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := Spread(shifted)
		if err != nil {
			t.Fatal(err)
		}

		if !floatEquals(shiftedResult, original, 1e-10) {
			t.Errorf("Location invariance failed: Spread(x+%v) = %v, expected %v", c, shiftedResult, original)
		}
	})

	// Scale invariance: Spread(c * x) = |c| * Spread(x)
	t.Run("scale invariance positive", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := Spread(x)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Spread(scaled)
		if err != nil {
			t.Fatal(err)
		}
		expected := c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Scale invariance failed: Spread(%v*x) = %v, expected %v", c, scaledResult, expected)
		}
	})

	t.Run("scale invariance negative", func(t *testing.T) {
		c := -2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := Spread(x)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Spread(scaled)
		if err != nil {
			t.Fatal(err)
		}
		expected := -c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Scale invariance failed: Spread(%v*x) = %v, expected %v", c, scaledResult, expected)
		}
	})
}

func TestRelSpreadInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0, 4.0, 5.0}

	// Location invariance: RelSpread(x + c) depends on c
	t.Run("location effect", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(x))
		for i, v := range x {
			shifted[i] = v + c
		}

		original, err := RelSpread(x)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := RelSpread(shifted)
		if err != nil {
			t.Fatal(err)
		}

		// RelSpread should decrease when shifting positive values further from zero
		if shiftedResult >= original {
			t.Errorf("RelSpread should decrease with positive shift: original=%v, shifted=%v", original, shiftedResult)
		}
	})

	// Scale invariance: RelSpread(c * x) = RelSpread(x) for c > 0
	t.Run("scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := RelSpread(x)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := RelSpread(scaled)
		if err != nil {
			t.Fatal(err)
		}

		if !floatEquals(scaledResult, original, 1e-10) {
			t.Errorf("Scale invariance failed: RelSpread(%v*x) = %v, expected %v", c, scaledResult, original)
		}
	})
}

func TestShiftInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0}
	y := []float64{4.0, 5.0, 6.0}

	// Location invariance: Shift(x + c, y) = Shift(x, y) + c
	t.Run("x location invariance", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(x))
		for i, v := range x {
			shifted[i] = v + c
		}

		original, err := Shift(x, y)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := Shift(shifted, y)
		if err != nil {
			t.Fatal(err)
		}
		expected := original + c

		if !floatEquals(shiftedResult, expected, 1e-10) {
			t.Errorf("X location invariance failed: Shift(x+%v, y) = %v, expected %v", c, shiftedResult, expected)
		}
	})

	// Location invariance: Shift(x, y + c) = Shift(x, y) - c
	t.Run("y location invariance", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(y))
		for i, v := range y {
			shifted[i] = v + c
		}

		original, err := Shift(x, y)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := Shift(x, shifted)
		if err != nil {
			t.Fatal(err)
		}
		expected := original - c

		if !floatEquals(shiftedResult, expected, 1e-10) {
			t.Errorf("Y location invariance failed: Shift(x, y+%v) = %v, expected %v", c, shiftedResult, expected)
		}
	})

	// Scale invariance: Shift(c * x, c * y) = c * Shift(x, y)
	t.Run("scale invariance", func(t *testing.T) {
		c := 2.5
		scaledX := make([]float64, len(x))
		scaledY := make([]float64, len(y))
		for i, v := range x {
			scaledX[i] = v * c
		}
		for i, v := range y {
			scaledY[i] = v * c
		}

		original, err := Shift(x, y)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Shift(scaledX, scaledY)
		if err != nil {
			t.Fatal(err)
		}
		expected := c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Scale invariance failed: Shift(%v*x, %v*y) = %v, expected %v", c, c, scaledResult, expected)
		}
	})
}

func TestRatioInvariance(t *testing.T) {
	x := []float64{2.0, 4.0, 6.0}
	y := []float64{1.0, 2.0, 3.0}

	// Scale invariance: Ratio(c * x, y) = c * Ratio(x, y)
	t.Run("x scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := Ratio(x, y)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Ratio(scaled, y)
		if err != nil {
			t.Fatal(err)
		}
		expected := c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("X scale invariance failed: Ratio(%v*x, y) = %v, expected %v", c, scaledResult, expected)
		}
	})

	// Scale invariance: Ratio(x, c * y) = Ratio(x, y) / c
	t.Run("y scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(y))
		for i, v := range y {
			scaled[i] = v * c
		}

		original, err := Ratio(x, y)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Ratio(x, scaled)
		if err != nil {
			t.Fatal(err)
		}
		expected := original / c

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Y scale invariance failed: Ratio(x, %v*y) = %v, expected %v", c, scaledResult, expected)
		}
	})
}
