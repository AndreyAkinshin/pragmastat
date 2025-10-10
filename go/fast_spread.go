package pragmastat

import (
	"math"
	"math/rand"
	"sort"
)

// fastSpread computes the median of all pairwise absolute differences efficiently.
// Time complexity: O(n log n) expected
// Space complexity: O(n)
func fastSpread[T Number](values []T) (float64, error) {
	n := len(values)
	if n == 0 {
		return 0.0, errEmptyInput
	}
	if n == 1 {
		return 0.0, nil
	}
	if n == 2 {
		return math.Abs(float64(values[1] - values[0])), nil
	}

	// Sort the values
	a := make([]T, n)
	copy(a, values)
	sort.Slice(a, func(i, j int) bool { return a[i] < a[j] })

	// Total number of pairwise differences with i < j
	N := int64(n) * int64(n-1) / 2
	kLow := (N + 1) / 2  // 1-based rank of lower middle
	kHigh := (N + 2) / 2 // 1-based rank of upper middle

	// Per-row active bounds over columns j (0-based indices)
	// Row i allows j in [i+1, n-1] initially
	L := make([]int, n)
	R := make([]int, n)
	for i := 0; i < n; i++ {
		L[i] = i + 1
		if L[i] >= n {
			L[i] = n
		}
		R[i] = n - 1
		if L[i] > R[i] {
			L[i] = 1
			R[i] = 0 // mark empty
		}
	}

	rowCounts := make([]int64, n)

	// Initial pivot: a central gap
	pivot := float64(a[n/2] - a[(n-1)/2])
	prevCountBelow := int64(-1)

	for {
		// === PARTITION: count how many differences are < pivot ===
		countBelow := int64(0)
		largestBelow := math.Inf(-1)
		smallestAtOrAbove := math.Inf(1)

		j := 1 // global two-pointer
		for i := 0; i < n-1; i++ {
			if j < i+1 {
				j = i + 1
			}
			for j < n && float64(a[j]-a[i]) < pivot {
				j++
			}

			cntRow := int64(j - (i + 1))
			if cntRow < 0 {
				cntRow = 0
			}
			rowCounts[i] = cntRow
			countBelow += cntRow

			// boundary elements for this row
			if cntRow > 0 {
				candBelow := float64(a[j-1] - a[i])
				largestBelow = math.Max(largestBelow, candBelow)
			}

			if j < n {
				candAtOrAbove := float64(a[j] - a[i])
				smallestAtOrAbove = math.Min(smallestAtOrAbove, candAtOrAbove)
			}
		}

		// === TARGET CHECK ===
		atTarget := countBelow == kLow || countBelow == kHigh-1

		if atTarget {
			if kLow < kHigh {
				// Even N: average the two central order stats
				return 0.5 * (largestBelow + smallestAtOrAbove), nil
			}
			// Odd N: pick the single middle
			needLargest := countBelow == kLow
			if needLargest {
				return largestBelow, nil
			}
			return smallestAtOrAbove, nil
		}

		// === STALL HANDLING ===
		if countBelow == prevCountBelow {
			minActive := math.Inf(1)
			maxActive := math.Inf(-1)
			active := int64(0)

			for i := 0; i < n-1; i++ {
				Li, Ri := L[i], R[i]
				if Li > Ri {
					continue
				}

				rowMin := float64(a[Li] - a[i])
				rowMax := float64(a[Ri] - a[i])
				minActive = math.Min(minActive, rowMin)
				maxActive = math.Max(maxActive, rowMax)
				active += int64(Ri - Li + 1)
			}

			if active <= 0 {
				if kLow < kHigh {
					return 0.5 * (largestBelow + smallestAtOrAbove), nil
				}
				if countBelow >= kLow {
					return largestBelow, nil
				}
				return smallestAtOrAbove, nil
			}

			if maxActive <= minActive {
				return minActive, nil
			}

			mid := 0.5 * (minActive + maxActive)
			if mid > minActive && mid <= maxActive {
				pivot = mid
			} else {
				pivot = maxActive
			}
			prevCountBelow = countBelow
			continue
		}

		// === SHRINK ACTIVE WINDOW ===
		if countBelow < kLow {
			// Need larger differences: discard all strictly below pivot
			for i := 0; i < n-1; i++ {
				newL := i + 1 + int(rowCounts[i])
				if newL > L[i] {
					L[i] = newL
				}
				if L[i] > R[i] {
					L[i] = 1
					R[i] = 0
				}
			}
		} else {
			// Too many below: keep only those strictly below pivot
			for i := 0; i < n-1; i++ {
				newR := i + int(rowCounts[i])
				if newR < R[i] {
					R[i] = newR
				}
				if R[i] < i+1 {
					L[i] = 1
					R[i] = 0
				}
			}
		}

		prevCountBelow = countBelow

		// === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
		activeSize := int64(0)
		for i := 0; i < n-1; i++ {
			if L[i] <= R[i] {
				activeSize += int64(R[i] - L[i] + 1)
			}
		}

		if activeSize <= 2 {
			// Few candidates left: return midrange of remaining
			minRem := math.Inf(1)
			maxRem := math.Inf(-1)
			for i := 0; i < n-1; i++ {
				if L[i] > R[i] {
					continue
				}
				lo := float64(a[L[i]] - a[i])
				hi := float64(a[R[i]] - a[i])
				minRem = math.Min(minRem, lo)
				maxRem = math.Max(maxRem, hi)
			}

			if activeSize <= 0 {
				if kLow < kHigh {
					return 0.5 * (largestBelow + smallestAtOrAbove), nil
				}
				if countBelow >= kLow {
					return largestBelow, nil
				}
				return smallestAtOrAbove, nil
			}

			if kLow < kHigh {
				return 0.5 * (minRem + maxRem), nil
			}
			if math.Abs(float64(kLow-1)-float64(countBelow)) <= math.Abs(float64(countBelow)-float64(kLow)) {
				return minRem, nil
			}
			return maxRem, nil
		}

		// Weighted random row selection
		t := rand.Int63n(activeSize)
		acc := int64(0)
		row := 0
		for row = 0; row < n-1; row++ {
			if L[row] > R[row] {
				continue
			}
			size := int64(R[row] - L[row] + 1)
			if t < acc+size {
				break
			}
			acc += size
		}

		// Median column of the selected row
		col := (L[row] + R[row]) / 2
		pivot = float64(a[col] - a[row])
	}
}
