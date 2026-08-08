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
// Panics if logMean is not finite, or if logStdDev is not finite and positive.
func NewMultiplic(logMean, logStdDev float64) *Multiplic {
	if !isFinite(logMean) {
		panic("logMean must be finite")
	}
	if !(isFinite(logStdDev) && logStdDev > 0) {
		panic("logStdDev must be finite and positive")
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
	for i := range count {
		result[i] = m.Sample(rng)
	}
	return result
}

// DensityAt returns the probability density at x.
//
// The support is x > 0, and that is the test. It used to be x < 1e-9, which is not
// the edge of the support but an arbitrary point inside it: a Multiplic(-30, 1) has
// its median at 9.4e-14, so that threshold reported zero density and zero
// probability across essentially the whole distribution.
func (m *Multiplic) DensityAt(x float64) float64 {
	if x <= 0.0 {
		return 0.0
	}
	z := (logFunction(x) - m.LogMean) / m.LogStdDev
	return expFunction(-z*z/2) / (x * m.LogStdDev * sqrt2Pi)
}

// CumulativeAt returns the probability of not exceeding x.
//
// A lognormal variable's distribution function is the normal one applied to log x,
// so this goes through additiveCumulative rather than spelling the same thing out
// through erf.
func (m *Multiplic) CumulativeAt(x float64) float64 {
	if x <= 0.0 {
		return 0.0
	}
	return additiveCumulative((logFunction(x) - m.LogMean) / m.LogStdDev)
}

// QuantileAt returns the value below which a fraction p of the distribution lies.
//
// The float64 conversion pins the intermediate rounding against FMA contraction,
// which the go:check:fma task guards.
func (m *Multiplic) QuantileAt(p float64) float64 {
	if p <= 0.0 {
		return 0.0
	}
	if p >= 1.0 {
		return math.Inf(1)
	}
	return expFunction(m.LogMean + float64(m.LogStdDev*additiveQuantile(p)))
}
