// Package pragmastat provides a deterministic RNG for cross-language reproducibility.
package pragmastat

import "time"

// Rng is a deterministic random number generator.
//
// Rng uses xoshiro256++ internally and guarantees identical output sequences
// across all Pragmastat language implementations when initialized with the same seed.
type Rng struct {
	inner *xoshiro256PlusPlus
}

// NewRng creates a new Rng with system entropy (non-deterministic).
func NewRng() *Rng {
	return NewRngFromSeed(time.Now().UnixNano())
}

// NewRngFromSeed creates a new Rng from an integer seed.
// The same seed always produces the same sequence of random numbers.
func NewRngFromSeed(seed int64) *Rng {
	return &Rng{
		inner: newXoshiro256PlusPlus(uint64(seed)),
	}
}

// NewRngFromString creates a new Rng from a string seed.
// The string is hashed using FNV-1a to produce a numeric seed.
func NewRngFromString(seed string) *Rng {
	return &Rng{
		inner: newXoshiro256PlusPlus(fnv1aHash(seed)),
	}
}

// Uniform generates a uniform random float in [0, 1).
// Uses 53 bits of precision for the mantissa.
func (r *Rng) Uniform() float64 {
	return r.inner.uniform()
}

// UniformInt generates a uniform random integer in [min, max).
// Returns min if min >= max.
//
// Uses modulo reduction which introduces slight bias for ranges that don't
// evenly divide 2^64. This bias is negligible for statistical simulations
// but not suitable for cryptographic applications.
func (r *Rng) UniformInt(min, max int64) int64 {
	return r.inner.uniformInt(min, max)
}

// Shuffle returns a shuffled copy of the input slice.
// Uses the Fisher-Yates shuffle algorithm for uniform distribution.
// The original slice is not modified.
func Shuffle[T any](rng *Rng, x []T) []T {
	result := make([]T, len(x))
	copy(result, x)
	n := len(result)

	// Fisher-Yates shuffle (backwards)
	for i := n - 1; i > 0; i-- {
		j := int(rng.UniformInt(0, int64(i+1)))
		result[i], result[j] = result[j], result[i]
	}

	return result
}

// Sample returns k elements from the input slice without replacement.
// Uses selection sampling to maintain order of first appearance.
// Returns all elements if k >= len(x).
// Panics if k is negative.
func Sample[T any](rng *Rng, x []T, k int) []T {
	if k < 0 {
		panic("sample: k must be non-negative")
	}
	n := len(x)
	if k >= n {
		result := make([]T, n)
		copy(result, x)
		return result
	}

	result := make([]T, 0, k)
	remaining := k

	for i := 0; i < n && remaining > 0; i++ {
		available := n - i
		// Probability of selecting this item: remaining / available
		if rng.Uniform()*float64(available) < float64(remaining) {
			result = append(result, x[i])
			remaining--
		}
	}

	return result
}

// ShuffleFloat64 returns a shuffled copy of the float64 slice.
// This is a convenience method; Shuffle[T] is also available for other types.
func (r *Rng) ShuffleFloat64(x []float64) []float64 {
	return Shuffle(r, x)
}

// SampleFloat64 returns k float64 elements from the slice without replacement.
// This is a convenience method; Sample[T] is also available for other types.
func (r *Rng) SampleFloat64(x []float64, k int) []float64 {
	return Sample(r, x, k)
}
