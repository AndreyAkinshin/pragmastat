package pragmastat

import "math"

// Exp represents an exponential distribution with given rate parameter.
// The mean of this distribution is 1/rate.
type Exp struct {
	Rate float64
}

// NewExp creates a new exponential distribution with given rate.
// Panics if rate <= 0.
func NewExp(rate float64) *Exp {
	if rate <= 0 {
		panic("rate must be positive")
	}
	return &Exp{Rate: rate}
}

// Sample generates a single sample from the exponential distribution.
func (e *Exp) Sample(rng *Rng) float64 {
	// Inverse CDF method: -ln(1 - U) / rate
	u := rng.UniformFloat64()
	// Avoid log(0)
	if u == 1.0 {
		u = 1.0 - machineEpsilon
	}
	return -math.Log(1.0-u) / e.Rate
}

// Samples generates multiple samples from the exponential distribution.
func (e *Exp) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := 0; i < count; i++ {
		result[i] = e.Sample(rng)
	}
	return result
}
