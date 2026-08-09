package pragmastat

// smallestPositiveSubnormal is the smallest positive subnormal (denormalized)
// IEEE 754 double-precision value.
// Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
//
// This is the smallest positive value representable in IEEE 754 binary64 format,
// the absolute smallest positive number before underflow to zero.
const smallestPositiveSubnormal = 5e-324

// samplingCumulance draws the cumulance Additive and Multiplic consume, the two families whose
// QuantileAt(0) is not a sample: Additive returns negative infinity there, Multiplic returns the
// zero its own support excludes. The other three families' QuantileAt(0) is the finite value
// their support already includes, so they draw UniformFloat64 directly and never call this.
//
// A drawn 0 is lifted to the smallest positive subnormal. UniformFloat64 never returns 1, so the
// other endpoint needs no guard.
func samplingCumulance(rng *Rng) float64 {
	u := rng.UniformFloat64()
	if u == 0.0 {
		return smallestPositiveSubnormal
	}
	return u
}

// Distribution is the interface for distributions that can generate samples.
type Distribution interface {
	// Sample generates a single sample from this distribution.
	Sample(rng *Rng) float64

	// DensityAt returns the probability density at x. Zero outside the support.
	DensityAt(x float64) float64

	// CumulativeAt returns the probability of not exceeding x. Zero below the
	// support, one above it.
	CumulativeAt(x float64) float64

	// QuantileAt returns the value below which a fraction p of the distribution
	// lies. A p outside (0, 1) returns the endpoints of the support, which are
	// infinities for the unbounded families.
	QuantileAt(p float64) float64

	// Samples generates multiple samples from this distribution.
	Samples(rng *Rng, count int) []float64
}

// sqrt2Pi is sqrt(2 * pi), the normalizing constant of the normal density.
//
// A literal rather than math.Sqrt(2 * math.Pi): the seven ports must divide by the
// same double, and a runtime square root of a runtime product is one more place for
// them to differ.
const sqrt2Pi = 2.506628274631000241612355239340
