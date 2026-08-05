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
		1e-7,
		1e21,
		1e-5,
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

// Go used to be the one port that wrote a large number as an exponent: 'g' switches over from
// 1e+06 upward, so a Bounds read [1e+06;1e+14] here and [1000000;100000000000000] in the six
// other ports. Nothing caught it because every value in the demos, the README and the fixtures is
// small enough to sit below the switch.
//
// The values below are exactly the ones where 'f' and 'g' disagree, so this fails if the verb
// goes back. It states the strings outright rather than deriving them, since deriving them from
// the same call the code makes would assert nothing.
func TestNumbersArePositionalAtEveryMagnitude(t *testing.T) {
	cases := []struct {
		value float64
		want  string
	}{
		{1e6, "1000000"},
		{1234567.25, "1234567.25"},
		{1e14, "100000000000000"},
		{1e20, "100000000000000000000"},
		{1e-5, "0.00001"},
		{1e-7, "0.0000001"},
		{86, "86"},
		{100.5, "100.5"},
		{0.5008354224706334, "0.5008354224706334"},
	}

	for _, c := range cases {
		if got := formatValue(c.value); got != c.want {
			t.Errorf("formatValue(%v) = %q, want %q", c.value, got, c.want)
		}
	}
}
