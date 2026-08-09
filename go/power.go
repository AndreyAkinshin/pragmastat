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

// Sample draws by inverting QuantileAt at a drawn cumulance.
func (p *Power) Sample(rng *Rng) float64 {
	return p.QuantileAt(rng.UniformFloat64())
}

// Samples generates multiple samples from the power distribution.
func (p *Power) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := range count {
		result[i] = p.Sample(rng)
	}
	return result
}

// DensityAt returns the probability density at x.
//
// shape * min^shape / x^(shape+1), rearranged as
// (shape / x) * exp(shape * (log min - log x)). The toolkit owns an exponential and
// a logarithm and owns no general power, so a real exponent is spelled as the
// exponential of a logarithm. This family is the reason the logarithm had to exist
// before the trio could.
//
// The support is [min, infinity), closed at the left, so the density at min is
// shape / min rather than zero, while the cumulative there is zero. Uniform reads
// the same way at its own lower endpoint.
func (p *Power) DensityAt(x float64) float64 {
	if x < p.Min {
		return 0.0
	}
	return (p.Shape / x) * expFunction(p.Shape*(logFunction(p.Min)-logFunction(x)))
}

// CumulativeAt returns the probability of not exceeding x.
func (p *Power) CumulativeAt(x float64) float64 {
	if x <= p.Min {
		return 0.0
	}
	return 1.0 - expFunction(p.Shape*(logFunction(p.Min)-logFunction(x)))
}

// QuantileAt returns the value below which a fraction prob of the distribution lies.
func (p *Power) QuantileAt(prob float64) float64 {
	if prob <= 0.0 {
		return p.Min
	}
	if prob >= 1.0 {
		return math.Inf(1)
	}
	return p.Min * expFunction(-logFunction(1.0-prob)/p.Shape)
}
