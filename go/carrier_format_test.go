package pragmastat

import (
	"fmt"
	"testing"
)

// Measurement.String used strconv.FormatFloat with 'G' while Bounds.String used %v, which is 'g'
// for a float64. So one double had two spellings inside one port, and a Bounds disagreed with the
// Measurements it is built from: 1e-7 printed as 1E-07 from one and 1e-07 from the other.
//
// Only the exponent form diverged, which is why nothing caught it: every value in the demos, the
// README and the fixtures is positional. This test compares the two carriers against each other
// rather than against a fixed string, so the two cannot drift apart again whichever spelling a
// later change picks.
func TestCarriersSpellOneDoubleOneWay(t *testing.T) {
	values := []float64{
		0,
		86,
		100.5,
		0.125,
		1e-7, // below the positional band: exponent form
		1e21, // above it
		1e-5, // the first value Go writes as an exponent
		123456789012345680000,
	}

	for _, v := range values {
		measurement := NewNumberMeasurement(v).String()
		bounds := newBounds(v, v, NumberUnit).String()
		want := fmt.Sprintf("[%s;%s]", measurement, measurement)

		if bounds != want {
			t.Errorf("Measurement(%v) prints %q, but Bounds(%v, %v) prints %q, want %q",
				v, measurement, v, v, bounds, want)
		}
	}
}
