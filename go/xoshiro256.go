// Package pragmastat provides xoshiro256++ PRNG for cross-language reproducibility.
package pragmastat

import (
	"math/bits"
)

// SplitMix64 PRNG for seed expansion
type splitMix64 struct {
	state uint64
}

func newSplitMix64(seed uint64) *splitMix64 {
	return &splitMix64{state: seed}
}

func (s *splitMix64) next() uint64 {
	s.state += 0x9e3779b97f4a7c15
	z := s.state
	z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9
	z = (z ^ (z >> 27)) * 0x94d049bb133111eb
	return z ^ (z >> 31)
}

// Xoshiro256PlusPlus is a PRNG that passes BigCrush
// Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
type xoshiro256PlusPlus struct {
	state [4]uint64
}

func newXoshiro256PlusPlus(seed uint64) *xoshiro256PlusPlus {
	sm := newSplitMix64(seed)
	return &xoshiro256PlusPlus{
		state: [4]uint64{sm.next(), sm.next(), sm.next(), sm.next()},
	}
}

func (x *xoshiro256PlusPlus) nextU64() uint64 {
	result := bits.RotateLeft64(x.state[0]+x.state[3], 23) + x.state[0]

	t := x.state[1] << 17

	x.state[2] ^= x.state[0]
	x.state[3] ^= x.state[1]
	x.state[1] ^= x.state[2]
	x.state[0] ^= x.state[3]

	x.state[2] ^= t
	x.state[3] = bits.RotateLeft64(x.state[3], 45)

	return result
}

func (x *xoshiro256PlusPlus) uniform() float64 {
	// Use upper 53 bits for maximum precision
	return float64(x.nextU64()>>11) * (1.0 / float64(uint64(1)<<53))
}

func (x *xoshiro256PlusPlus) uniformInt(min, max int64) int64 {
	if min >= max {
		return min
	}
	// Safe range computation avoiding signed overflow
	var rangeSize uint64
	if min >= 0 {
		rangeSize = uint64(max) - uint64(min)
	} else if max <= 0 {
		rangeSize = uint64(-min) - uint64(-max)
	} else {
		// min < 0 < max: check for overflow
		rangeSize = uint64(max) + uint64(-min)
		if rangeSize < uint64(max) {
			panic("uniform_int: range overflow")
		}
	}
	return min + int64(x.nextU64()%rangeSize)
}

// FNV-1a hash constants
const (
	fnvOffsetBasis = 0xcbf29ce484222325
	fnvPrime       = 0x00000100000001b3
)

// fnv1aHash computes FNV-1a 64-bit hash of a string
func fnv1aHash(s string) uint64 {
	hash := uint64(fnvOffsetBasis)
	for i := 0; i < len(s); i++ {
		hash ^= uint64(s[i])
		hash *= fnvPrime
	}
	return hash
}
