package pragmastat

import (
	"math"
	"sort"
)

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

	for i := 0; i < n; i++ {
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
		mid := 0.5*lo + 0.5*hi
		countLessOrEqual := centerCountPairsLessOrEqualImpl(sorted, mid)

		if countLessOrEqual >= k {
			hi = mid
		} else {
			lo = mid
		}
	}

	// Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b (halve before
	// summing; never overflows; operand order is irrelevant).
	target := 0.5*lo + 0.5*hi
	var candidates []float64

	for i := 0; i < n; i++ {
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
			candidates = append(candidates, 0.5*sorted[i]+0.5*sorted[left])
		}

		if left > i {
			avgBefore := 0.5*sorted[i] + 0.5*sorted[left-1]
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
