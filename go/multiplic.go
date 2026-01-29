package pragmastat

import "math"

// Multiplic represents a multiplicative (log-normal) distribution.
// The logarithm of samples follows an Additive (Normal) distribution.
type Multiplic struct {
	LogMean   float64
	LogStdDev float64
	additive  *Additive
}

// NewMultiplic creates a new multiplicative (log-normal) distribution.
// Panics if logStdDev <= 0.
func NewMultiplic(logMean, logStdDev float64) *Multiplic {
	if logStdDev <= 0 {
		panic("logStdDev must be positive")
	}
	return &Multiplic{
		LogMean:   logMean,
		LogStdDev: logStdDev,
		additive:  NewAdditive(logMean, logStdDev),
	}
}

// Sample generates a single sample from the multiplicative distribution.
func (m *Multiplic) Sample(rng *Rng) float64 {
	return math.Exp(m.additive.Sample(rng))
}

// Samples generates multiple samples from the multiplicative distribution.
func (m *Multiplic) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := 0; i < count; i++ {
		result[i] = m.Sample(rng)
	}
	return result
}
