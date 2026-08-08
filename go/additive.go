package pragmastat

import "math"

// Additive represents an additive (normal/Gaussian) distribution.
// Uses the Box-Muller transform to generate samples.
type Additive struct {
	Mean   float64
	StdDev float64
}

// NewAdditive creates a new additive (normal) distribution.
// Panics if mean is not finite, or if stdDev is not finite and positive.
func NewAdditive(mean, stdDev float64) *Additive {
	if !isFinite(mean) {
		panic("mean must be finite")
	}
	if !(isFinite(stdDev) && stdDev > 0) {
		panic("stdDev must be finite and positive")
	}
	return &Additive{Mean: mean, StdDev: stdDev}
}

// Sample generates a single sample from the additive distribution.
func (a *Additive) Sample(rng *Rng) float64 {
	// Box-Muller transform
	u1 := rng.UniformFloat64()
	u2 := rng.UniformFloat64()

	// Avoid log(0) - use smallest positive subnormal for cross-language consistency
	if u1 == 0 {
		u1 = smallestPositiveSubnormal
	}

	r := math.Sqrt(-2.0 * math.Log(u1))
	theta := 2.0 * math.Pi * u2

	// Use the first of the two Box-Muller outputs
	z := r * math.Cos(theta)

	// float64 conversion pins the intermediate rounding; see uniformFloat64Range
	// in xoshiro256.go. It does not make this draw portable on its own: math.Log
	// and math.Cos are themselves polynomial evaluations in Go, and gc fuses them
	// on FMA-capable targets, so a residual divergence remains that no conversion
	// in this file can remove.
	return a.Mean + float64(z*a.StdDev)
}

// Samples generates multiple samples from the additive distribution.
func (a *Additive) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := range count {
		result[i] = a.Sample(rng)
	}
	return result
}

// DensityAt returns the probability density at x.
func (a *Additive) DensityAt(x float64) float64 {
	z := (x - a.Mean) / a.StdDev
	return expFunction(-z*z/2) / (a.StdDev * sqrt2Pi)
}

// CumulativeAt returns the probability of not exceeding x.
func (a *Additive) CumulativeAt(x float64) float64 {
	return additiveCumulative((x - a.Mean) / a.StdDev)
}

// QuantileAt returns the value below which a fraction p of the distribution lies.
//
// The inverse goes through additiveQuantile, which is what that function was built
// for. It replaces two Newton steps taken against an error function accurate to
// 1.5e-07, which had left this quantile eight orders of magnitude coarser than the
// cumulative beside it. The infinities at the ends are the ones additiveQuantile
// already returns.
//
// The float64 conversion pins the intermediate rounding against FMA contraction,
// which the go:check:fma task guards.
func (a *Additive) QuantileAt(p float64) float64 {
	return a.Mean + float64(a.StdDev*additiveQuantile(p))
}
