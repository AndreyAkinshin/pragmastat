package pragmastat

import (
	"math"
	"testing"
)

// The values the definition fixes, none of which may reach an approximant.
func TestAdditiveQuantileSpecialValues(t *testing.T) {
	if got := additiveQuantile(0.5); got != 0.0 || math.Signbit(got) {
		t.Errorf("Phi^-1(0.5) = %v, want +0", got)
	}
	if got := additiveQuantile(0.0); !math.IsInf(got, -1) {
		t.Errorf("Phi^-1(0) = %v, want -Inf", got)
	}
	if got := additiveQuantile(1.0); !math.IsInf(got, 1) {
		t.Errorf("Phi^-1(1) = %v, want +Inf", got)
	}
	if got := additiveQuantile(-0.1); !math.IsInf(got, -1) {
		t.Errorf("Phi^-1(-0.1) = %v, want -Inf", got)
	}
	if got := additiveQuantile(1.1); !math.IsInf(got, 1) {
		t.Errorf("Phi^-1(1.1) = %v, want +Inf", got)
	}
	if got := additiveQuantile(math.NaN()); !math.IsNaN(got) {
		t.Errorf("Phi^-1(NaN) = %v, want NaN", got)
	}
}

// Textbook quantiles, the numbers a reader is most likely to check by hand.
func TestAdditiveQuantileTextbookValues(t *testing.T) {
	cases := []struct {
		p    float64
		want float64
	}{
		{0.75, 0.674489750196082},
		{0.9, 1.2815515655446004},
		{0.95, 1.6448536269514722},
		{0.975, 1.959963984540054},
		{0.99, 2.3263478740408408},
		{0.995, 2.5758293035489004},
		{0.999, 3.090232306167813},
	}
	for _, c := range cases {
		got := additiveQuantile(c.p)
		tol := 8.0 * 2.220446049250313e-16 * math.Abs(c.want)
		if math.Abs(got-c.want) > tol {
			t.Errorf("Phi^-1(%v) = %v, want about %v", c.p, got, c.want)
		}
	}
}

// The central branch is exactly odd about 1/2, because it consumes q = p - 1/2 directly and the
// approximant multiplies it back in.
//
// The tails cannot be, and that is a property of binary64 rather than of this function: they work
// from min(p, 1-p), and 1.0 - 0.95 is 0.050000000000000044 rather than 0.05, so the two sides are
// not handed the same number. The band edge is excluded for a related reason, since
// 0.5 + 0.425 - 0.5 returns 0.42500000000000004 and lands its point in the other branch.
func TestAdditiveQuantileCentralBranchIsOdd(t *testing.T) {
	for _, q := range []float64{0.25, 0.125, 0.0625, 0.4} {
		if 0.5+q-0.5 != q {
			t.Fatalf("the offset itself does not round-trip: %v", q)
		}
		hi := additiveQuantile(0.5 + q)
		lo := additiveQuantile(0.5 - q)
		if hi != -lo {
			t.Errorf("asymmetry at q = %v: %v vs %v", q, hi, -lo)
		}
	}
}

// Monotone, and reaching the far tail rather than saturating. The smallest positive double is the
// last argument a caller can pass, and its quantile is finite.
func TestAdditiveQuantileMonotoneIntoTheFarTail(t *testing.T) {
	previous := math.Inf(-1)
	for p := 5e-324; p < 0.5; p *= 8.0 {
		got := additiveQuantile(p)
		if got <= previous {
			t.Fatalf("not monotone at p = %v: %v after %v", p, got, previous)
		}
		if math.IsInf(got, 0) || math.IsNaN(got) {
			t.Fatalf("not finite at p = %v: %v", p, got)
		}
		previous = got
	}
	if math.Abs(additiveQuantile(5e-324)-(-38.467405617144)) > 1e-9 {
		t.Errorf("far tail endpoint drifted: %v", additiveQuantile(5e-324))
	}
}
