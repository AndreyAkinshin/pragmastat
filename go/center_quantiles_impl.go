package pragmastat

import (
	"math"
	"sort"
)

// Every multiply-add in this file is written as float64(a*b) + float64(c*d)
// rather than a*b + c*d. The conversions are not redundant. The Go
// specification lets an implementation fuse a multiply into an add as a single
// rounding, and gc does it on arm64, ppc64le, s390x, riscv64 and loong64,
// never on amd64. None of the other six languages fuses, so a fused result
// would differ in the last bit from every one of them. An explicit conversion
// is the only way the language offers to pin the intermediate rounding.
//
// Here the multiplier is always 0.5, so the scaling is exact and the fused and
// unfused forms agree bit for bit: none of these sites was ever wrong. They are
// pinned for uniformity, so this file needs no entry on the go:check:fma
// exemption list and no future reader has to re-derive that proof to know the
// file is safe.

// relativeEpsilon is the tolerance for floating-point comparisons in binary search convergence.
const relativeEpsilon = 1e-14

// centerQuantileBoundsImpl computes both lower and upper bounds from pairwise averages.
// Uses binary search with counting function to avoid materializing all N(N+1)/2 pairs.
func centerQuantileBoundsImpl(sorted []float64, marginLo, marginHi int64) (lo, hi float64) {
	n := len(sorted)
	totalPairs := int64(n) * int64(n+1) / 2

	if marginLo < 1 {
		marginLo = 1
	}
	if marginLo > totalPairs {
		marginLo = totalPairs
	}
	if marginHi < 1 {
		marginHi = 1
	}
	if marginHi > totalPairs {
		marginHi = totalPairs
	}

	lo = centerFindExactQuantileImpl(sorted, marginLo)
	hi = centerFindExactQuantileImpl(sorted, marginHi)

	if lo > hi {
		lo, hi = hi, lo
	}
	return lo, hi
}

// centerCountPairsLessOrEqualImpl counts pairwise averages <= target value.
// Uses O(n) two-pointer algorithm.
func centerCountPairsLessOrEqualImpl(sorted []float64, target float64) int64 {
	n := len(sorted)
	var count int64
	// j is not reset: as i increases, threshold decreases monotonically
	j := n - 1

	for i := range n {
		threshold := 2*target - sorted[i]

		for j >= 0 && sorted[j] > threshold {
			j--
		}

		if j >= i {
			count += int64(j - i + 1)
		}
	}

	return count
}

// centerFindExactQuantileImpl finds the exact k-th pairwise average using selection algorithm.
func centerFindExactQuantileImpl(sorted []float64, k int64) float64 {
	n := len(sorted)
	totalPairs := int64(n) * int64(n+1) / 2

	if n == 1 {
		return sorted[0]
	}

	if k == 1 {
		return sorted[0]
	}

	if k == totalPairs {
		return sorted[n-1]
	}

	lo := sorted[0]
	hi := sorted[n-1]
	const eps = relativeEpsilon

	for hi-lo > eps*math.Max(1.0, math.Max(math.Abs(lo), math.Abs(hi))) {
		// Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b (halve before
		// summing; never overflows; operand order is irrelevant).
		mid := float64(0.5*lo) + float64(0.5*hi)
		countLessOrEqual := centerCountPairsLessOrEqualImpl(sorted, mid)

		if countLessOrEqual >= k {
			hi = mid
		} else {
			lo = mid
		}
	}

	// Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b (halve before
	// summing; never overflows; operand order is irrelevant).
	target := float64(0.5*lo) + float64(0.5*hi)
	var candidates []float64

	for i := range n {
		threshold := 2*target - sorted[i]

		left := i
		right := n

		for left < right {
			m := (left + right) / 2
			if sorted[m] < threshold-eps {
				left = m + 1
			} else {
				right = m
			}
		}

		if left < n && left >= i && math.Abs(sorted[left]-threshold) < eps*math.Max(1.0, math.Abs(threshold)) {
			candidates = append(candidates, float64(0.5*sorted[i])+float64(0.5*sorted[left]))
		}

		if left > i {
			avgBefore := float64(0.5*sorted[i]) + float64(0.5*sorted[left-1])
			if avgBefore <= target+eps {
				candidates = append(candidates, avgBefore)
			}
		}
	}

	if len(candidates) == 0 {
		return target
	}

	// Sort candidates
	sort.Float64s(candidates)

	for _, candidate := range candidates {
		countAtCandidate := centerCountPairsLessOrEqualImpl(sorted, candidate)
		if countAtCandidate >= k {
			return candidate
		}
	}

	return target
}
