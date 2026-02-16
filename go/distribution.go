package pragmastat

// machineEpsilon is the machine epsilon for IEEE 754 double-precision (binary64).
// Value: 2^(-52) ≈ 2.220446049250313e-16
//
// This is the smallest ε such that 1.0 + ε ≠ 1.0 in float64 arithmetic.
// Represents the distance between 1.0 and the next representable number.
//
// Used to avoid log(0) or division by zero when uniformFloat64() returns exactly 1.0.
// All language implementations use this same value to ensure cross-language
// determinism in distribution sampling.
const machineEpsilon = 2.220446049250313e-16

// smallestPositiveSubnormal is the smallest positive subnormal (denormalized)
// IEEE 754 double-precision value.
// Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
//
// This is the smallest positive value representable in IEEE 754 binary64 format.
// Unlike machine epsilon (which is the smallest ε where 1+ε ≠ 1), this is the
// absolute smallest positive number before underflow to zero.
//
// Used to avoid log(0) in Box-Muller transform when uniformFloat64() returns exactly 0.
// All language implementations use this same value to ensure cross-language
// determinism in distribution sampling.
const smallestPositiveSubnormal = 5e-324

// Distribution is the interface for distributions that can generate samples.
type Distribution interface {
	// Sample generates a single sample from this distribution.
	Sample(rng *Rng) float64

	// Samples generates multiple samples from this distribution.
	Samples(rng *Rng, count int) []float64
}
