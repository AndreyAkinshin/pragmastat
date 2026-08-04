package pragmastat

import "math"

// Power represents a power (Pareto) distribution with minimum value and shape parameter.
// Follows a power-law distribution where large values are rare but possible.
type Power struct {
	Min   float64
	Shape float64
}

// NewPower creates a new power (Pareto) distribution.
// Panics if min or shape is not finite and positive.
func NewPower(min, shape float64) *Power {
	if !(isFinite(min) && min > 0) {
		panic("min must be finite and positive")
	}
	if !(isFinite(shape) && shape > 0) {
		panic("shape must be finite and positive")
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
	for i := range count {
		result[i] = p.Sample(rng)
	}
	return result
}
