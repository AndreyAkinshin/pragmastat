package pragmastat

import "math"

// The float64 conversions in this file pin intermediate roundings against FMA contraction,
// for the reason given at the top of signed_rank_margin.go. The mise task named
// "go:check:fma" is the guard.

// Constants of the range reduction, emitted by tests/oracles/fit_log.py.
//
// ln 2 is split so that k*logLn2Hi is exact: it carries 32 significant bits and |k| needs at
// most 11, which leaves the product inside the 53 available.
const (
	logLn2Hi   = 0.6931471803691238
	logLn2Lo   = 1.9082149292705877e-10
	sqrt2Over2 = 0.7071067811865476
)

// logPolyP is the Chebyshev fit of (log((1+s)/(1-s)) - 2*s) / s^3 over the reduced range,
// emitted by tests/oracles/fit_log.py. The leading terms are the recognizable 2/3, 2/5, 2/7 of
// the odd series doubled, which is where the assembly's missing factor of two lives; the tail
// departs from the series because the fit minimizes the worst error rather than matching Taylor.
var logPolyP = [7]float64{
	0.666666666666667,
	0.39999999999899505,
	0.28571428625975487,
	0.2222221113479508,
	0.18182889125261723,
	0.15331721600556042,
	0.14616449685043406,
}

// logFunction is the natural logarithm every port evaluates, in place of the platform's.
//
// IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
// nothing about the logarithm. The reason this one has to be ours is stronger than for most
// transcendentals: Ratio is exp(median(log x - log y)), so a last-bit difference in log moves
// the median's input and changes which pairwise element is selected. That is a different
// answer, not a small error in the same one.
//
// The reduction is the classical one:
//
//	x = 2^k * m with m in [sqrt(2)/2, sqrt(2)),  f = m - 1,  s = f/(2+f),
//	log(m) = f - (hfsq - s*(hfsq + R)),  R = s*s * P(s*s)
//
// The special cases follow IEEE 754 and match what every platform returns for them, since they
// are the cases a platform has no freedom about.
func logFunction(x float64) float64 {
	if math.IsNaN(x) {
		return x
	}
	if x < 0 {
		return math.NaN()
	}
	if x == 0 {
		return math.Inf(-1)
	}
	if math.IsInf(x, 1) {
		return math.Inf(1)
	}
	if x == 1 {
		return 0
	}

	m, k := math.Frexp(x)
	// Frexp returns m in [0.5, 1); recenter on [sqrt(2)/2, sqrt(2)) so |f| stays small at both
	// ends. Doubling is exact.
	if m < sqrt2Over2 {
		m *= 2
		k--
	}

	f := m - 1
	s := float64(f / (2 + f))
	z := float64(s * s)

	acc := logPolyP[len(logPolyP)-1]
	for i := len(logPolyP) - 2; i >= 0; i-- {
		acc = float64(acc*z) + logPolyP[i]
	}
	r := float64(z * acc)
	hfsq := float64(0.5 * f * f)
	kf := float64(k)

	if k == 0 {
		return f - (hfsq - float64(s*(hfsq+r)))
	}
	return float64(kf*logLn2Hi) - ((hfsq - (float64(s*(hfsq+r)) + float64(kf*logLn2Lo))) - f)
}
