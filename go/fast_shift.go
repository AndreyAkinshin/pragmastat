package pragmastat

import (
	"math"
	"sort"
)

// fastShift computes the median of all pairwise differences {x[i] - y[j]}.
// Time complexity: O((m + n) * log(precision)) per quantile
// Space complexity: O(1) - avoids materializing all m*n differences
func fastShift[T Number](x, y []T) (float64, error) {
	m := len(x)
	n := len(y)
	if m == 0 || n == 0 {
		return 0, errEmptyInput
	}

	// Sort both arrays
	xs := make([]T, m)
	ys := make([]T, n)
	copy(xs, x)
	copy(ys, y)
	sort.Slice(xs, func(i, j int) bool { return xs[i] < xs[j] })
	sort.Slice(ys, func(i, j int) bool { return ys[i] < ys[j] })

	total := int64(m) * int64(n)

	// Type-7 quantile: h = 1 + (n-1)*p, then interpolate between floor(h) and ceil(h)
	// For median, p = 0.5
	h := 1.0 + float64(total-1)*0.5
	lowerRank := int64(math.Floor(h))
	upperRank := int64(math.Ceil(h))
	weight := h - float64(lowerRank)

	if lowerRank < 1 {
		lowerRank = 1
	}
	if upperRank > total {
		upperRank = total
	}

	lower := selectKthPairwiseDiff(xs, ys, lowerRank)
	if lowerRank == upperRank {
		return lower, nil
	}

	upper := selectKthPairwiseDiff(xs, ys, upperRank)
	if weight == 0.0 {
		return lower, nil
	}
	return (1.0-weight)*lower + weight*upper, nil
}

// selectKthPairwiseDiff finds the k-th smallest pairwise difference (1-based indexing).
// Uses binary search combined with two-pointer counting to avoid materializing all differences.
func selectKthPairwiseDiff[T Number](x, y []T, k int64) float64 {
	m := len(x)
	n := len(y)
	total := int64(m) * int64(n)

	if k < 1 || k > total {
		panic("k out of range")
	}

	searchMin := float64(x[0] - y[n-1])
	searchMax := float64(x[m-1] - y[0])

	if math.IsNaN(searchMin) || math.IsNaN(searchMax) {
		panic("NaN in input values")
	}

	const maxIterations = 128 // Sufficient for double precision convergence
	prevMin := math.Inf(-1)
	prevMax := math.Inf(1)

	for iter := 0; iter < maxIterations && searchMin != searchMax; iter++ {
		mid := searchMin + (searchMax-searchMin)*0.5
		countLessOrEqual, closestBelow, closestAbove := countAndNeighbors(x, y, mid)

		if closestBelow == closestAbove {
			return closestBelow
		}

		// No progress means we're stuck between two discrete values
		if searchMin == prevMin && searchMax == prevMax {
			if countLessOrEqual >= k {
				return closestBelow
			}
			return closestAbove
		}

		prevMin = searchMin
		prevMax = searchMax

		if countLessOrEqual >= k {
			searchMax = closestBelow
		} else {
			searchMin = closestAbove
		}
	}

	if searchMin != searchMax {
		panic("convergence failure (pathological input)")
	}

	return searchMin
}

// countAndNeighbors counts pairs where x[i] - y[j] <= threshold using two-pointer algorithm.
// Also tracks the closest actual differences on either side of threshold.
func countAndNeighbors[T Number](x, y []T, threshold float64) (int64, float64, float64) {
	m := len(x)
	n := len(y)
	var count int64
	maxBelow := math.Inf(-1)
	minAbove := math.Inf(1)

	j := 0
	for i := 0; i < m; i++ {
		// Move j forward while x[i] - y[j] > threshold
		for j < n && float64(x[i]-y[j]) > threshold {
			j++
		}

		// Count pairs where x[i] - y[j] <= threshold
		count += int64(n - j)

		// Track closest difference <= threshold
		if j < n {
			diff := float64(x[i] - y[j])
			if diff > maxBelow {
				maxBelow = diff
			}
		}

		// Track closest difference > threshold
		if j > 0 {
			diff := float64(x[i] - y[j-1])
			if diff < minAbove {
				minAbove = diff
			}
		}
	}

	// Fallback to actual min/max if no boundaries found (shouldn't happen in normal operation)
	if math.IsInf(maxBelow, -1) {
		maxBelow = float64(x[0] - y[n-1])
	}
	if math.IsInf(minAbove, 1) {
		minAbove = float64(x[m-1] - y[0])
	}

	return count, maxBelow, minAbove
}
