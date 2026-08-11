package pragmastat

import "math"

// Binary64 primitives shared by the distribution estimation models.
//
// Every operation is written out explicitly so that all seven language implementations evaluate
// the same graph and produce identical bits. None of these helpers may be replaced by a fused,
// vectorized or library variant without a new algorithm identifier.

// lerp interpolates between a and b, reproducing both endpoints exactly.
func lerp(a, b, t float64) float64 {
	if t == 0.0 {
		return a
	}
	if t == 1.0 {
		return b
	}
	if a < 0.0 && 0.0 < b {
		return float64((1.0-t)*a) + float64(t*b)
	}
	return a + float64(t*(b-a))
}

// unitRatio returns num/den clamped to the unit interval. Callers must have proved
// 0 <= num <= den and den > 0; a NaN ratio means that proof was violated, which is a defect
// rather than a data problem.
func unitRatio(num, den float64) float64 {
	t := num / den
	if math.IsNaN(t) {
		panic("numerical conformance failure: unitRatio produced NaN")
	}
	return min(1.0, max(0.0, t))
}

// upperBound returns the first index i with values[i] > key, over an ascending slice.
func upperBound(values []float64, key float64) int {
	lo, hi := 0, len(values)
	for lo < hi {
		mid := lo + (hi-lo)/2
		if values[mid] <= key {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}
