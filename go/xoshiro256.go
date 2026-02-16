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

// ========================================================================
// Floating Point Methods
// ========================================================================

func (x *xoshiro256PlusPlus) uniformFloat64() float64 {
	// Use upper 53 bits for maximum precision
	return float64(x.nextU64()>>11) * (1.0 / float64(uint64(1)<<53))
}

// Note: FP rounding in min + (max-min)*u can theoretically yield max
// for extreme values of (max-min). Acceptable for statistical use.
func (x *xoshiro256PlusPlus) uniformFloat64Range(min, max float64) float64 {
	if min >= max {
		return min
	}
	return min + (max-min)*x.uniformFloat64()
}

func (x *xoshiro256PlusPlus) uniformFloat32() float32 {
	// Use 24 bits for float32 mantissa precision
	return float32(x.nextU64()>>40) * (1.0 / float32(uint64(1)<<24))
}

func (x *xoshiro256PlusPlus) uniformFloat32Range(min, max float32) float32 {
	if min >= max {
		return min
	}
	return min + (max-min)*x.uniformFloat32()
}

// ========================================================================
// Signed Integer Methods
// ========================================================================

func (x *xoshiro256PlusPlus) uniformInt64(min, max int64) int64 {
	if min >= max {
		return min
	}
	// uint64 subtraction gives correct unsigned distance for all int64 pairs
	rangeSize := uint64(max) - uint64(min)
	return min + int64(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformInt32(min, max int32) int32 {
	if min >= max {
		return min
	}
	rangeSize := uint64(int64(max) - int64(min))
	return min + int32(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformInt16(min, max int16) int16 {
	if min >= max {
		return min
	}
	rangeSize := uint64(int32(max) - int32(min))
	return min + int16(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformInt8(min, max int8) int8 {
	if min >= max {
		return min
	}
	rangeSize := uint64(int16(max) - int16(min))
	return min + int8(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformInt(min, max int) int {
	if min >= max {
		return min
	}
	rangeSize := uint64(int64(max) - int64(min))
	return min + int(x.nextU64()%rangeSize)
}

// ========================================================================
// Unsigned Integer Methods
// ========================================================================

func (x *xoshiro256PlusPlus) uniformUint64(min, max uint64) uint64 {
	if min >= max {
		return min
	}
	rangeSize := max - min
	return min + x.nextU64()%rangeSize
}

func (x *xoshiro256PlusPlus) uniformUint32(min, max uint32) uint32 {
	if min >= max {
		return min
	}
	rangeSize := uint64(max - min)
	return min + uint32(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformUint16(min, max uint16) uint16 {
	if min >= max {
		return min
	}
	rangeSize := uint64(max - min)
	return min + uint16(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformUint8(min, max uint8) uint8 {
	if min >= max {
		return min
	}
	rangeSize := uint64(max - min)
	return min + uint8(x.nextU64()%rangeSize)
}

func (x *xoshiro256PlusPlus) uniformUint(min, max uint) uint {
	if min >= max {
		return min
	}
	rangeSize := uint64(max - min)
	return min + uint(x.nextU64()%rangeSize)
}

// ========================================================================
// Boolean Methods
// ========================================================================

func (x *xoshiro256PlusPlus) uniformBool() bool {
	return x.uniformFloat64() < 0.5
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
