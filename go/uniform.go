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
func (u *Uniform) Sample(rng *Rng) float64 {
	return u.Min + rng.UniformFloat64()*(u.Max-u.Min)
}

// Samples generates multiple samples from the uniform distribution.
func (u *Uniform) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := 0; i < count; i++ {
		result[i] = u.Sample(rng)
	}
	return result
}
