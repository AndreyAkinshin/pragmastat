package pragmastat

import "math"

// Binary64 primitives shared by the distribution estimation models.
//
// Every operation is written out explicitly so that all seven language implementations evaluate
// the same graph and produce identical bits. None of these helpers may be replaced by a fused,
// vectorized or library variant without a new algorithm identifier.

// nextUp returns the next representable binary64 value toward positive infinity.
func nextUp(value float64) float64 {
	if math.IsNaN(value) || math.IsInf(value, 1) {
		return value
	}
	if value == 0.0 {
		return math.SmallestNonzeroFloat64
	}

	bits := math.Float64bits(value)
	if value > 0.0 {
		bits++
	} else {
		bits--
	}
	return math.Float64frombits(bits)
}

// nextDown returns the next representable binary64 value toward negative infinity.
func nextDown(value float64) float64 {
	if math.IsNaN(value) || math.IsInf(value, -1) {
		return value
	}
	if value == 0.0 {
		return -math.SmallestNonzeroFloat64
	}

	bits := math.Float64bits(value)
	if value > 0.0 {
		bits--
	} else {
		bits++
	}
	return math.Float64frombits(bits)
}

// midpoint returns the overflow-safe midpoint of two finite values.
//
// The float64 conversions are not redundant. The Go specification lets an implementation fuse a
// multiply and an add into a single rounding, and on arm64 the compiler does exactly that here;
// the specification of these models forbids it, because a fused result differs in the last bit
// from the other six languages. An explicit conversion is the only way the language offers to
// pin the intermediate rounding.
func midpoint(a, b float64) float64 {
	if a < 0.0 && 0.0 < b {
		return float64(a/2.0) + float64(b/2.0)
	}
	return a + float64((b-a)/2.0)
}

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

// ceilSqrt returns the smallest integer k with k >= ceil(n/k), computed without forming k*k so
// that the result never depends on floating-point rounding.
func ceilSqrt(n int) int {
	k := 1
	for k < (n+k-1)/k {
		k++
	}
	return k
}

// lowerBound returns the first index i in [from, to) with values[i] >= key, over an ascending
// slice.
func lowerBound(values []float64, from, to int, key float64) int {
	lo, hi := from, to
	for lo < hi {
		mid := lo + (hi-lo)/2
		if values[mid] < key {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
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
