package pragmastat

import "math"

// Additive represents an additive (normal/Gaussian) distribution.
// Uses the Box-Muller transform to generate samples.
type Additive struct {
	Mean   float64
	StdDev float64
}

// NewAdditive creates a new additive (normal) distribution.
// Panics if stdDev <= 0.
func NewAdditive(mean, stdDev float64) *Additive {
	if stdDev <= 0 {
		panic("stdDev must be positive")
	}
	return &Additive{Mean: mean, StdDev: stdDev}
}

// Sample generates a single sample from the additive distribution.
func (a *Additive) Sample(rng *Rng) float64 {
	// Box-Muller transform
	u1 := rng.UniformFloat64()
	u2 := rng.UniformFloat64()

	// Avoid log(0) - use smallest positive subnormal for cross-language consistency
	if u1 == 0 {
		u1 = smallestPositiveSubnormal
	}

	r := math.Sqrt(-2.0 * math.Log(u1))
	theta := 2.0 * math.Pi * u2

	// Use the first of the two Box-Muller outputs
	z := r * math.Cos(theta)

	return a.Mean + z*a.StdDev
}

// Samples generates multiple samples from the additive distribution.
func (a *Additive) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := 0; i < count; i++ {
		result[i] = a.Sample(rng)
	}
	return result
}
