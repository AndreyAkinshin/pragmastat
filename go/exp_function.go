package pragmastat

import "math"

// The float64 conversions in this file pin intermediate roundings against FMA contraction,
// for the reason given at the top of signed_rank_margin.go. The mise task named
// "go:check:fma" is the guard.

// Constants of the range reduction, emitted by tests/oracles/fit_exp.py.
//
// ln 2 is split so that k*ln2Hi is exact: ln2Hi carries 32 significant bits and |k| needs at
// most 11, which leaves the product inside the 53 available. Without the split the reduction
// would lose the low bits of r, and r is where the accuracy lives.
const (
	invLn2 = 1.4426950408889634e+00
	ln2Hi  = 6.9314718036912382e-01
	ln2Lo  = 1.9082149292705877e-10
)

// expFunction is the exponential every port evaluates, in place of the platform's.
//
// IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
// nothing about the exponential. Conforming libraries disagree in the last bit and do:
// measured on one Edgeworth crossover, Go's software exp and glibc's return neighboring
// values, which moved the reported margin by two. Since a margin selects an order statistic,
// that is a different confidence interval from the same inputs.
//
// So the exponential cannot be the platform's. This one is built from operations the standard
// does fix:
//
//	k = floor(y/ln2 + 1/2),  r = (y - k*ln2Hi) - k*ln2Lo,  exp(y) = 2^k * exp(r)
//
// with exp(r) on |r| <= ln2/2 from the polynomial in tests/oracles/fit_exp.py. Fitting
// (exp(r) - 1 - r)/r^2 rather than exp(r) keeps the two leading terms exact and leaves the
// polynomial supplying a correction below 0.07, so the assembled result stays within one ulp
// of the correctly rounded exponential while being reproducible everywhere.
//
// The shape is fdlibm's without its division: fdlibm needs the rational form 1 + r + r*R/(2-R)
// because its polynomial is degree 5 in r^2, and measured here that form buys nothing on top of
// a degree-11 chain. A table like the one glibc has gained in 2018 would reduce r by a further
// factor of 128 and does reach a better result, at 263 constants against 15 and seven
// independent chances to mistype one; two prototypes of it here came out an order of magnitude
// worse than this, which is the argument for the simpler construction rather than against the
// table.
//
// floor(x + 1/2) rather than a rounding function: Go rounds halves away from zero and R
// rounds them to even, so naming a rounding is naming a disagreement.
func expFunction(y float64) float64 {
	if math.IsNaN(y) {
		return y
	}
	// Past these the answer is not in doubt, and stating the cutoffs keeps the reduction
	// from having to produce a k it cannot scale by.
	if y > 709.79 {
		return math.Inf(1)
	}
	if y < -745.2 {
		return 0
	}

	// The two halves of the reduction are kept apart until the assembly below, which is where
	// the accuracy is: see the comment there.
	k := math.Floor(float64(y*invLn2) + 0.5)
	hi := y - float64(k*ln2Hi)
	lo := float64(k * ln2Lo)
	r := hi - lo

	q := 1.6086622436215554e-10
	q = float64(q*r) + 2.0918129454967065e-09
	q = float64(q*r) + 2.5052071109214447e-08
	q = float64(q*r) + 2.7557263301474753e-07
	q = float64(q*r) + 2.7557319247204789e-06
	q = float64(q*r) + 2.4801587336421322e-05
	q = float64(q*r) + 1.9841269841263304e-04
	q = float64(q*r) + 1.3888888888879082e-03
	q = float64(q*r) + 8.3333333333333332e-03
	q = float64(q*r) + 4.1666666666666678e-02
	q = float64(q*r) + 1.6666666666666666e-01
	q = float64(q*r) + 5.0000000000000000e-01
	// Assembled from hi and lo rather than from r. Writing 1 + r + r*r*q discards the low bits of
	// r: r reaches 0.35, adding it to 1 shifts the mantissa two places, and the tail falls off
	// the end. Reconstructing the same sum from the two halves of the reduction keeps them, and
	// costs nothing, being the same operations in a different order. Measured against a 60-digit
	// reference over the band these estimators reach, it halves the worst relative error, from
	// 2.18e-16 to 1.27e-16, and raises the share of correctly rounded results from 73.6% to
	// 90.2%. That is a shade better than fdlibm, which reaches 1.30e-16 with a division this does not need.
	p := 1.0 - ((lo - float64(float64(r*r)*q)) - hi)

	// Two scalings rather than one. Splitting k in half keeps the first factor inside the
	// normal range whatever k is, so only the second can denormalize or overflow, and it
	// does so in a single rounding.
	half := int(k) / 2
	return float64(p*math.Ldexp(1, half)) * math.Ldexp(1, int(k)-half)
}
