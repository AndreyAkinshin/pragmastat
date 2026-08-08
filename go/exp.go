package pragmastat

import "math"

// Exp represents an exponential distribution with given rate parameter.
// The mean of this distribution is 1/rate.
type Exp struct {
	Rate float64
}

// NewExp creates a new exponential distribution with given rate.
// Panics if rate is not finite and positive.
func NewExp(rate float64) *Exp {
	if !(isFinite(rate) && rate > 0) {
		panic("rate must be finite and positive")
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
	for i := range count {
		result[i] = e.Sample(rng)
	}
	return result
}

// DensityAt returns the probability density at x.
func (e *Exp) DensityAt(x float64) float64 {
	if x < 0.0 {
		return 0.0
	}
	return e.Rate * expFunction(-e.Rate*x)
}

// CumulativeAt returns the probability of not exceeding x.
func (e *Exp) CumulativeAt(x float64) float64 {
	if x < 0.0 {
		return 0.0
	}
	return 1.0 - expFunction(-e.Rate*x)
}

// QuantileAt returns the value below which a fraction p of the distribution lies.
func (e *Exp) QuantileAt(p float64) float64 {
	if p <= 0.0 {
		return 0.0
	}
	if p >= 1.0 {
		return math.Inf(1)
	}
	return -logFunction(1.0-p) / e.Rate
}
