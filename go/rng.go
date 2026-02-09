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

// ========================================================================
// Floating Point Methods
// ========================================================================

// Uniform generates a uniform random float in [0, 1).
// Uses 53 bits of precision for the mantissa.
func (r *Rng) Uniform() float64 {
	return r.inner.uniform()
}

// UniformRange generates a uniform random float in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformRange(min, max float64) float64 {
	return r.inner.uniformRange(min, max)
}

// UniformFloat32 generates a uniform random float32 in [0, 1).
// Uses 24 bits for float32 mantissa precision.
func (r *Rng) UniformFloat32() float32 {
	return r.inner.uniformFloat32()
}

// UniformFloat32Range generates a uniform random float32 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformFloat32Range(min, max float32) float32 {
	return r.inner.uniformFloat32Range(min, max)
}

// ========================================================================
// Signed Integer Methods
// ========================================================================

// UniformInt64 generates a uniform random int64 in [min, max).
// Returns min if min >= max.
//
// Uses modulo reduction which introduces slight bias for ranges that don't
// evenly divide 2^64. This bias is negligible for statistical simulations
// but not suitable for cryptographic applications.
func (r *Rng) UniformInt64(min, max int64) int64 {
	return r.inner.uniformInt64(min, max)
}

// UniformInt32 generates a uniform random int32 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformInt32(min, max int32) int32 {
	return r.inner.uniformInt32(min, max)
}

// UniformInt16 generates a uniform random int16 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformInt16(min, max int16) int16 {
	return r.inner.uniformInt16(min, max)
}

// UniformInt8 generates a uniform random int8 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformInt8(min, max int8) int8 {
	return r.inner.uniformInt8(min, max)
}

// UniformIntN generates a uniform random int in [min, max).
// Returns min if min >= max. This uses the platform-specific int type.
func (r *Rng) UniformIntN(min, max int) int {
	return r.inner.uniformInt(min, max)
}

// ========================================================================
// Unsigned Integer Methods
// ========================================================================

// UniformUint64 generates a uniform random uint64 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformUint64(min, max uint64) uint64 {
	return r.inner.uniformUint64(min, max)
}

// UniformUint32 generates a uniform random uint32 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformUint32(min, max uint32) uint32 {
	return r.inner.uniformUint32(min, max)
}

// UniformUint16 generates a uniform random uint16 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformUint16(min, max uint16) uint16 {
	return r.inner.uniformUint16(min, max)
}

// UniformUint8 generates a uniform random uint8 in [min, max).
// Returns min if min >= max.
func (r *Rng) UniformUint8(min, max uint8) uint8 {
	return r.inner.uniformUint8(min, max)
}

// UniformUintN generates a uniform random uint in [min, max).
// Returns min if min >= max. This uses the platform-specific uint type.
func (r *Rng) UniformUintN(min, max uint) uint {
	return r.inner.uniformUint(min, max)
}

// ========================================================================
// Boolean Methods
// ========================================================================

// UniformBool generates a uniform random boolean with P(true) = 0.5.
func (r *Rng) UniformBool() bool {
	return r.inner.uniformBool()
}

// ========================================================================
// Collection Methods
// ========================================================================

// Shuffle returns a shuffled copy of the input slice.
// Uses the Fisher-Yates shuffle algorithm for uniform distribution.
// The original slice is not modified.
func Shuffle[T any](rng *Rng, x []T) []T {
	result := make([]T, len(x))
	copy(result, x)
	n := len(result)

	// Fisher-Yates shuffle (backwards)
	for i := n - 1; i > 0; i-- {
		j := int(rng.UniformInt64(0, int64(i+1)))
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

// Resample returns k elements from the input slice with replacement (bootstrap sampling).
// Each element is independently selected with equal probability.
// The original slice is not modified.
// Panics if k is negative or if x is empty.
func Resample[T any](rng *Rng, x []T, k int) []T {
	if k < 0 {
		panic("resample: k must be non-negative")
	}
	if len(x) == 0 {
		panic("resample: cannot resample from empty slice")
	}

	result := make([]T, k)
	n := len(x)
	for i := 0; i < k; i++ {
		result[i] = x[rng.UniformIntN(0, n)]
	}
	return result
}

// ResampleFloat64 returns k float64 elements from the slice with replacement.
func (r *Rng) ResampleFloat64(x []float64, k int) []float64 {
	return Resample(r, x, k)
}
