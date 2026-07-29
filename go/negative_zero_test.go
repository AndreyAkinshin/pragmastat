package pragmastat

import (
	"math"
	"slices"
	"testing"
)

// No public estimator reports a negative zero.
//
// A sample holding both +0.0 and -0.0 used to make the reported value depend on which of the two
// the sort happened to leave in the selected position: comparison cannot separate them, so the
// position was settled by the sorting algorithm rather than by the data, and the seven ports each
// bring their own sort. Python returned -0.0 for center([0.0, -0.0, 0.0, -0.0, 1.0]) where this
// port returned +0.0, against an exact conformance class that promises identical bits.
//
// Every estimator now sheds the sign on the way out, so these tests compare PAYLOADS. == reads
// -0.0 and +0.0 as equal, so an equality assertion would pass on exactly the results this file
// exists to reject; sameFloatBits is the predicate the rest of the suite uses for the same reason.
//
// The samples are the ones that reached a negative zero before the fix, plus a sweep that holds
// the invariant for the estimators where it is currently unreachable. Those are unreachable by
// proof, not by construction (spread is an absolute difference, ratio is an exponential), and a
// proof nobody re-checks is how the next divergence gets in. Disparity is here because it was the
// one exit this port left unwrapped after the other six had been done.

const negativeZeroPayload = 0x8000000000000000

// mixedZeroSamples each estimate to zero, and each selected a -0.0 before the fix.
var mixedZeroSamples = [][]float64{
	{0.0, math.Copysign(0, -1), 0.0, math.Copysign(0, -1), 1.0},
	{math.Copysign(0, -1), math.Copysign(0, -1)},
	{math.Copysign(0, -1), 0.0},
	{0.0, math.Copysign(0, -1)},
	{math.Copysign(0, -1), math.Copysign(0, -1), math.Copysign(0, -1)},
	{-1.0, 1.0},
	{-2.0, math.Copysign(0, -1), 2.0},
}

var (
	mixedSample    = []float64{0.0, math.Copysign(0, -1), 0.0, math.Copysign(0, -1), 1.0, -1.0}
	positiveSample = []float64{1.0, 2.0, 3.0, 4.0, 5.0, 6.0}
)

func assertPositiveZero(t *testing.T, value float64, what string) {
	t.Helper()
	if !sameFloatBits(value, 0.0) {
		t.Errorf("%s = %s, want %s", what, formatFloatBits(value), formatFloatBits(0.0))
	}
}

func assertNotNegativeZero(t *testing.T, value float64, what string) {
	t.Helper()
	if math.Float64bits(value) == negativeZeroPayload {
		t.Errorf("%s reported a negative zero: %s", what, formatFloatBits(value))
	}
}

func TestCenterReportsPositiveZero(t *testing.T) {
	for _, x := range mixedZeroSamples {
		value, err := Center(x, false)
		if err != nil {
			t.Fatalf("Center(%v): %v", x, err)
		}
		assertPositiveZero(t, value, "Center")

		sample, err := NewSample(x)
		if err != nil {
			t.Fatalf("NewSample(%v): %v", x, err)
		}
		measurement, err := sample.Center()
		if err != nil {
			t.Fatalf("Sample.Center(%v): %v", x, err)
		}
		assertPositiveZero(t, measurement.Value, "Sample.Center")
	}
}

func TestShiftReportsPositiveZero(t *testing.T) {
	pairs := [][2][]float64{
		{{math.Copysign(0, -1)}, {0.0}},
		{{math.Copysign(0, -1), 1.0}, {0.0, 1.0}},
		{{math.Copysign(0, -1), math.Copysign(0, -1)}, {0.0, 0.0}},
	}
	for _, pair := range pairs {
		value, err := Shift(pair[0], pair[1], false)
		if err != nil {
			t.Fatalf("Shift(%v, %v): %v", pair[0], pair[1], err)
		}
		assertPositiveZero(t, value, "Shift")
	}
}

func TestDisparityReportsPositiveZero(t *testing.T) {
	// Shift selects the -0.0 difference; the average spread is positive, so the sign survived.
	x := []float64{math.Copysign(0, -1), 1.0}
	y := []float64{0.0, 1.0}
	value, err := Disparity(x, y, false)
	if err != nil {
		t.Fatalf("Disparity: %v", err)
	}
	assertPositiveZero(t, value, "Disparity")

	sx, err := NewSample(x)
	if err != nil {
		t.Fatalf("NewSample(x): %v", err)
	}
	sy, err := NewSample(y)
	if err != nil {
		t.Fatalf("NewSample(y): %v", err)
	}
	measurement, err := sx.Disparity(sy)
	if err != nil {
		t.Fatalf("Sample.Disparity: %v", err)
	}
	assertPositiveZero(t, measurement.Value, "Sample.Disparity")
}

func TestBoundsReportPositiveZeros(t *testing.T) {
	// A single pair takes the degenerate x[0] - y[0] path, where -0.0 - +0.0 is -0.0.
	bounds, err := ShiftBounds([]float64{math.Copysign(0, -1)}, []float64{0.0}, 1.0, false)
	if err != nil {
		t.Fatalf("ShiftBounds: %v", err)
	}
	assertPositiveZero(t, bounds.Lower, "ShiftBounds lower")
	assertPositiveZero(t, bounds.Upper, "ShiftBounds upper")

	centerBounds, err := CenterBounds(mixedSample, 0.3, false)
	if err != nil {
		t.Fatalf("CenterBounds: %v", err)
	}
	assertPositiveZero(t, centerBounds.Lower, "CenterBounds lower")
	assertPositiveZero(t, centerBounds.Upper, "CenterBounds upper")
}

// TestNoEstimatorReportsNegativeZero sweeps every public exit, including the ones where a negative
// zero is currently unreachable. Reachability is what changes when the arithmetic behind an exit
// changes, so the sweep is the part that keeps holding after such a change.
func TestNoEstimatorReportsNegativeZero(t *testing.T) {
	scalar := []struct {
		name  string
		value func() (float64, error)
	}{
		{"Center", func() (float64, error) { return Center(mixedSample, false) }},
		{"Spread", func() (float64, error) { return Spread(mixedSample, false) }},
		{"Shift", func() (float64, error) { return Shift(mixedSample, mixedSample, false) }},
		{"Ratio", func() (float64, error) { return Ratio(positiveSample, positiveSample, false) }},
		{"Disparity", func() (float64, error) { return Disparity(mixedSample, mixedSample, false) }},
		{"avgSpread", func() (float64, error) { return avgSpread(mixedSample, mixedSample, false) }},
	}
	for _, c := range scalar {
		value, err := c.value()
		if err != nil {
			t.Fatalf("%s: %v", c.name, err)
		}
		assertNotNegativeZero(t, value, c.name)
	}

	bounded := []struct {
		name   string
		bounds func() (Bounds, error)
	}{
		{"CenterBounds", func() (Bounds, error) { return CenterBounds(mixedSample, 0.3, false) }},
		{"SpreadBounds", func() (Bounds, error) { return SpreadBoundsWithSeed(mixedSample, 0.5, "seed", false) }},
		{"ShiftBounds", func() (Bounds, error) { return ShiftBounds(mixedSample, mixedSample, 0.5, false) }},
		{"RatioBounds", func() (Bounds, error) { return RatioBounds(positiveSample, positiveSample, 0.5, false) }},
		{"DisparityBounds", func() (Bounds, error) {
			return DisparityBoundsWithSeed(mixedSample, mixedSample, 0.9, "seed", false)
		}},
		{"avgSpreadBounds", func() (Bounds, error) {
			sorted := append([]float64(nil), mixedSample...)
			slices.Sort(sorted)
			return avgSpreadBoundsImpl(mixedSample, sorted, mixedSample, sorted, 0.9,
				NewRngFromString("seed"), NewRngFromString("seed"))
		}},
	}
	for _, c := range bounded {
		bounds, err := c.bounds()
		if err != nil {
			t.Fatalf("%s: %v", c.name, err)
		}
		assertNotNegativeZero(t, bounds.Lower, c.name+" lower")
		assertNotNegativeZero(t, bounds.Upper, c.name+" upper")
	}
}

func TestInputsKeepTheirNegativeZeros(t *testing.T) {
	// Only outputs are normalized: a sample must still be able to CONTAIN a -0.0.
	x := []float64{0.0, math.Copysign(0, -1), 0.0, math.Copysign(0, -1), 1.0}
	if _, err := Center(x, false); err != nil {
		t.Fatalf("Center: %v", err)
	}
	if math.Float64bits(x[1]) != negativeZeroPayload {
		t.Errorf("input was rewritten: %s", formatFloatBits(x[1]))
	}

	sample, err := NewSample(x)
	if err != nil {
		t.Fatalf("NewSample: %v", err)
	}
	if _, err := sample.Center(); err != nil {
		t.Fatalf("Sample.Center: %v", err)
	}
	for _, v := range sample.cachedSortedValues() {
		if math.Float64bits(v) == negativeZeroPayload {
			return
		}
	}
	t.Error("sorted values were rewritten: no negative zero left in the sample")
}

func TestNormalizeZeroLeavesEveryNonZeroAlone(t *testing.T) {
	values := []float64{1.0, -1.0, 1e-320, -1e-320, math.Inf(1), math.Inf(-1), math.NaN(), 5e-324, -5e-324}
	for _, v := range values {
		if !sameFloatBits(normalizeZero(v), v) {
			t.Errorf("normalizeZero(%s) = %s", formatFloatBits(v), formatFloatBits(normalizeZero(v)))
		}
	}
}

func TestNormalizeZeroMapsBothZerosToThePositiveOne(t *testing.T) {
	for _, v := range []float64{0.0, math.Copysign(0, -1)} {
		assertPositiveZero(t, normalizeZero(v), "normalizeZero")
	}
}

func TestNewBoundsNormalizesBothEndpoints(t *testing.T) {
	bounds := newBounds(math.Copysign(0, -1), math.Copysign(0, -1), NumberUnit)
	assertPositiveZero(t, bounds.Lower, "lower")
	assertPositiveZero(t, bounds.Upper, "upper")

	infinite := newBounds(math.Inf(-1), math.Inf(1), NumberUnit)
	if !sameFloatBits(infinite.Lower, math.Inf(-1)) || !sameFloatBits(infinite.Upper, math.Inf(1)) {
		t.Error("newBounds rewrote an infinite endpoint")
	}
}

// TestAdditiveCumulativePropagatesNaN pins the one input for which the function has no answer. Both of
// its range comparisons are false for a NaN, so without the guard it leaves the tail branch as a
// finite 0 or 1: an undefined input answered rather than reported.
func TestAdditiveCumulativePropagatesNaN(t *testing.T) {
	if !math.IsNaN(additiveCumulative(math.NaN())) {
		t.Errorf("additiveCumulative(NaN) = %s, want NaN", formatFloatBits(additiveCumulative(math.NaN())))
	}
	if !sameFloatBits(additiveCumulative(math.Inf(1)), 1.0) {
		t.Errorf("additiveCumulative(+Inf) = %s, want 1", formatFloatBits(additiveCumulative(math.Inf(1))))
	}
	if !sameFloatBits(additiveCumulative(math.Inf(-1)), 0.0) {
		t.Errorf("additiveCumulative(-Inf) = %s, want 0", formatFloatBits(additiveCumulative(math.Inf(-1))))
	}
	if !sameFloatBits(additiveCumulative(0.0), 0.5) || !sameFloatBits(additiveCumulative(math.Copysign(0, -1)), 0.5) {
		t.Error("additiveCumulative disagreed with itself on the two zeros")
	}
}

// TestExpFunctionCutoffs pins the values outside the reduction band, where a language can name
// its infinity and JSON cannot: the shared fixture stops at 709.78 for that reason, so these
// arguments are covered here or nowhere.
func TestExpFunctionCutoffs(t *testing.T) {
	if !math.IsNaN(expFunction(math.NaN())) {
		t.Error("expFunction(NaN) should be NaN")
	}
	if !math.IsInf(expFunction(709.8), 1) || !math.IsInf(expFunction(math.Inf(1)), 1) {
		t.Error("expFunction should overflow to +Inf above the cutoff")
	}
	if !sameFloatBits(expFunction(-745.3), 0.0) || !sameFloatBits(expFunction(math.Inf(-1)), 0.0) {
		t.Error("expFunction should underflow to +0 below the cutoff")
	}
	if !sameFloatBits(expFunction(0.0), 1.0) {
		t.Error("expFunction(0) should be exactly 1")
	}
}
