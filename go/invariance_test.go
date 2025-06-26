package pragmastat

import (
	"testing"
)

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

func TestVolatilityInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0, 4.0, 5.0}

	// Location invariance: Volatility(x + c) depends on c
	t.Run("location effect", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(x))
		for i, v := range x {
			shifted[i] = v + c
		}

		original, err := Volatility(x)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := Volatility(shifted)
		if err != nil {
			t.Fatal(err)
		}

		// Volatility should decrease when shifting positive values further from zero
		if shiftedResult >= original {
			t.Errorf("Volatility should decrease with positive shift: original=%v, shifted=%v", original, shiftedResult)
		}
	})

	// Scale invariance: Volatility(c * x) = Volatility(x) for c > 0
	t.Run("scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := Volatility(x)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Volatility(scaled)
		if err != nil {
			t.Fatal(err)
		}

		if !floatEquals(scaledResult, original, 1e-10) {
			t.Errorf("Scale invariance failed: Volatility(%v*x) = %v, expected %v", c, scaledResult, original)
		}
	})
}

func TestPrecisionInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0, 4.0, 5.0}

	// Scale invariance: Precision(c * x) = |c| * Precision(x)
	t.Run("scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := Precision(x)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := Precision(scaled)
		if err != nil {
			t.Fatal(err)
		}
		expected := c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Scale invariance failed: Precision(%v*x) = %v, expected %v", c, scaledResult, expected)
		}
	})
}

func TestMedShiftInvariance(t *testing.T) {
	x := []float64{1.0, 2.0, 3.0}
	y := []float64{4.0, 5.0, 6.0}

	// Location invariance: MedShift(x + c, y) = MedShift(x, y) + c
	t.Run("x location invariance", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(x))
		for i, v := range x {
			shifted[i] = v + c
		}

		original, err := MedShift(x, y)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := MedShift(shifted, y)
		if err != nil {
			t.Fatal(err)
		}
		expected := original + c

		if !floatEquals(shiftedResult, expected, 1e-10) {
			t.Errorf("X location invariance failed: MedShift(x+%v, y) = %v, expected %v", c, shiftedResult, expected)
		}
	})

	// Location invariance: MedShift(x, y + c) = MedShift(x, y) - c
	t.Run("y location invariance", func(t *testing.T) {
		c := 10.0
		shifted := make([]float64, len(y))
		for i, v := range y {
			shifted[i] = v + c
		}

		original, err := MedShift(x, y)
		if err != nil {
			t.Fatal(err)
		}
		shiftedResult, err := MedShift(x, shifted)
		if err != nil {
			t.Fatal(err)
		}
		expected := original - c

		if !floatEquals(shiftedResult, expected, 1e-10) {
			t.Errorf("Y location invariance failed: MedShift(x, y+%v) = %v, expected %v", c, shiftedResult, expected)
		}
	})

	// Scale invariance: MedShift(c * x, c * y) = c * MedShift(x, y)
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

		original, err := MedShift(x, y)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := MedShift(scaledX, scaledY)
		if err != nil {
			t.Fatal(err)
		}
		expected := c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Scale invariance failed: MedShift(%v*x, %v*y) = %v, expected %v", c, c, scaledResult, expected)
		}
	})
}

func TestMedRatioInvariance(t *testing.T) {
	x := []float64{2.0, 4.0, 6.0}
	y := []float64{1.0, 2.0, 3.0}

	// Scale invariance: MedRatio(c * x, y) = c * MedRatio(x, y)
	t.Run("x scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(x))
		for i, v := range x {
			scaled[i] = v * c
		}

		original, err := MedRatio(x, y)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := MedRatio(scaled, y)
		if err != nil {
			t.Fatal(err)
		}
		expected := c * original

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("X scale invariance failed: MedRatio(%v*x, y) = %v, expected %v", c, scaledResult, expected)
		}
	})

	// Scale invariance: MedRatio(x, c * y) = MedRatio(x, y) / c
	t.Run("y scale invariance", func(t *testing.T) {
		c := 2.5
		scaled := make([]float64, len(y))
		for i, v := range y {
			scaled[i] = v * c
		}

		original, err := MedRatio(x, y)
		if err != nil {
			t.Fatal(err)
		}
		scaledResult, err := MedRatio(x, scaled)
		if err != nil {
			t.Fatal(err)
		}
		expected := original / c

		if !floatEquals(scaledResult, expected, 1e-10) {
			t.Errorf("Y scale invariance failed: MedRatio(x, %v*y) = %v, expected %v", c, scaledResult, expected)
		}
	})
}
