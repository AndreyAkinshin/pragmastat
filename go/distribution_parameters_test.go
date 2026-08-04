package pragmastat

import (
	"math"
	"testing"
)

// The five parametric families refuse malformed parameters at construction. Every parameter has to
// be finite, and every scale or shape parameter has to be positive.
//
// Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
// location constructed a family whose every draw is NaN. And the predicate was written negatively
// (stdDev <= 0), which is not the negation of stdDev > 0 under IEEE 754: NaN <= 0 is false, so NaN
// walked past the guard. The predicates are positive now, which refuses NaN without a branch of
// its own.

func expectDistributionPanic(t *testing.T, want string, build func()) {
	t.Helper()
	defer func() {
		r := recover()
		if r == nil {
			t.Fatalf("construction should panic with %q", want)
		}
		got, ok := r.(string)
		if !ok || got != want {
			t.Fatalf("panic = %v, want %q", r, want)
		}
	}()
	build()
}

func TestNewUniformRefusesNonFiniteBounds(t *testing.T) {
	expectDistributionPanic(t, "min must be finite", func() { NewUniform(math.NaN(), 1) })
	expectDistributionPanic(t, "min must be finite", func() { NewUniform(math.Inf(-1), 1) })
	expectDistributionPanic(t, "max must be finite", func() { NewUniform(0, math.NaN()) })
	expectDistributionPanic(t, "max must be finite", func() { NewUniform(0, math.Inf(1)) })
}

func TestNewUniformRefusesEmptyInterval(t *testing.T) {
	expectDistributionPanic(t, "min must be less than max", func() { NewUniform(1, 1) })
	expectDistributionPanic(t, "min must be less than max", func() { NewUniform(2, 1) })
}

func TestNewAdditiveRefusesMalformedParameters(t *testing.T) {
	expectDistributionPanic(t, "mean must be finite", func() { NewAdditive(math.NaN(), 1) })
	expectDistributionPanic(t, "mean must be finite", func() { NewAdditive(math.Inf(1), 1) })
	want := "stdDev must be finite and positive"
	expectDistributionPanic(t, want, func() { NewAdditive(0, math.NaN()) })
	expectDistributionPanic(t, want, func() { NewAdditive(0, math.Inf(1)) })
	expectDistributionPanic(t, want, func() { NewAdditive(0, 0) })
	expectDistributionPanic(t, want, func() { NewAdditive(0, -1) })
}

func TestNewMultiplicRefusesMalformedParameters(t *testing.T) {
	expectDistributionPanic(t, "logMean must be finite", func() { NewMultiplic(math.NaN(), 1) })
	expectDistributionPanic(t, "logMean must be finite", func() { NewMultiplic(math.Inf(1), 1) })
	want := "logStdDev must be finite and positive"
	expectDistributionPanic(t, want, func() { NewMultiplic(0, math.NaN()) })
	expectDistributionPanic(t, want, func() { NewMultiplic(0, math.Inf(1)) })
	expectDistributionPanic(t, want, func() { NewMultiplic(0, 0) })
	expectDistributionPanic(t, want, func() { NewMultiplic(0, -1) })
}

func TestNewExpRefusesMalformedParameters(t *testing.T) {
	want := "rate must be finite and positive"
	expectDistributionPanic(t, want, func() { NewExp(math.NaN()) })
	expectDistributionPanic(t, want, func() { NewExp(math.Inf(1)) })
	expectDistributionPanic(t, want, func() { NewExp(0) })
	expectDistributionPanic(t, want, func() { NewExp(-1) })
}

func TestNewPowerRefusesMalformedParameters(t *testing.T) {
	wantMin := "min must be finite and positive"
	expectDistributionPanic(t, wantMin, func() { NewPower(math.NaN(), 3) })
	expectDistributionPanic(t, wantMin, func() { NewPower(math.Inf(1), 3) })
	expectDistributionPanic(t, wantMin, func() { NewPower(0, 3) })
	expectDistributionPanic(t, wantMin, func() { NewPower(-1, 3) })
	wantShape := "shape must be finite and positive"
	expectDistributionPanic(t, wantShape, func() { NewPower(1, math.NaN()) })
	expectDistributionPanic(t, wantShape, func() { NewPower(1, math.Inf(1)) })
	expectDistributionPanic(t, wantShape, func() { NewPower(1, 0) })
	expectDistributionPanic(t, wantShape, func() { NewPower(1, -1) })
}

func TestWellFormedDistributionsStillConstructAndDraw(t *testing.T) {
	rng := NewRngFromString("distribution-parameters")
	families := []Distribution{
		NewUniform(-2.5, 7.5),
		NewAdditive(10, 2),
		NewMultiplic(1, 0.5),
		NewExp(3),
		NewPower(1, 2),
	}
	for i, family := range families {
		if v := family.Sample(rng); !isFinite(v) {
			t.Errorf("family %d drew %v, want a finite value", i, v)
		}
	}
}
