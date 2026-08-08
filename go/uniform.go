package pragmastat

import "math"

// Uniform represents a uniform distribution on [min, max).
type Uniform struct {
	Min float64
	Max float64
}

// NewUniform creates a new uniform distribution on [min, max).
// Panics if min or max is not finite, or if min is not less than max.
func NewUniform(min, max float64) *Uniform {
	if !isFinite(min) {
		panic("min must be finite")
	}
	if !isFinite(max) {
		panic("max must be finite")
	}
	if !(min < max) {
		panic("min must be less than max")
	}
	return &Uniform{Min: min, Max: max}
}

// Sample generates a single sample from the uniform distribution.
//
// The float64 conversion pins the intermediate rounding against FMA
// contraction; see uniformFloat64Range in xoshiro256.go for why sampling
// cannot afford a fused multiply-add.
func (u *Uniform) Sample(rng *Rng) float64 {
	return u.Min + float64(rng.UniformFloat64()*(u.Max-u.Min))
}

// Samples generates multiple samples from the uniform distribution.
func (u *Uniform) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := range count {
		result[i] = u.Sample(rng)
	}
	return result
}

// DensityAt returns the probability density at x.
//
// The support is half-open, so the density is zero at Max and positive at Min.
// This is the one family assembled from arithmetic alone: no primitive of the
// toolkit appears on any of its three paths.
func (u *Uniform) DensityAt(x float64) float64 {
	// NaN is refused before the bounds are compared. This is the only one of the
	// fifteen functions whose result does not depend on its argument, so a NaN
	// reaching the comparison satisfies neither half of it and falls through to an
	// ordinary density.
	if math.IsNaN(x) {
		return x
	}
	if x < u.Min || x >= u.Max {
		return 0.0
	}
	return 1.0 / (u.Max - u.Min)
}

// CumulativeAt returns the probability of not exceeding x.
func (u *Uniform) CumulativeAt(x float64) float64 {
	if x < u.Min {
		return 0.0
	}
	if x >= u.Max {
		return 1.0
	}
	return (x - u.Min) / (u.Max - u.Min)
}

// QuantileAt returns the value below which a fraction p of the distribution lies.
//
// The float64 conversion pins the intermediate rounding against FMA contraction,
// for the reason given on Sample, which the go:check:fma task guards.
func (u *Uniform) QuantileAt(p float64) float64 {
	if p <= 0.0 {
		return u.Min
	}
	if p >= 1.0 {
		return u.Max
	}
	return u.Min + float64(p*(u.Max-u.Min))
}
