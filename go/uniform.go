package pragmastat

// Uniform represents a uniform distribution on [min, max).
type Uniform struct {
	Min float64
	Max float64
}

// NewUniform creates a new uniform distribution on [min, max).
// Panics if min >= max.
func NewUniform(min, max float64) *Uniform {
	if min >= max {
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
