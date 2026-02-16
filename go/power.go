package pragmastat

import "math"

// Power represents a power (Pareto) distribution with minimum value and shape parameter.
// Follows a power-law distribution where large values are rare but possible.
type Power struct {
	Min   float64
	Shape float64
}

// NewPower creates a new power (Pareto) distribution.
// Panics if min <= 0 or shape <= 0.
func NewPower(min, shape float64) *Power {
	if min <= 0 {
		panic("min must be positive")
	}
	if shape <= 0 {
		panic("shape must be positive")
	}
	return &Power{Min: min, Shape: shape}
}

// Sample generates a single sample from the power distribution.
func (p *Power) Sample(rng *Rng) float64 {
	// Inverse CDF method: min / (1 - U)^(1/shape)
	u := rng.UniformFloat64()
	// Avoid division by zero
	if u == 1.0 {
		u = 1.0 - machineEpsilon
	}
	return p.Min / math.Pow(1.0-u, 1.0/p.Shape)
}

// Samples generates multiple samples from the power distribution.
func (p *Power) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := 0; i < count; i++ {
		result[i] = p.Sample(rng)
	}
	return result
}
