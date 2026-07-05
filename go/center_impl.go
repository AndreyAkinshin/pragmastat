package pragmastat

import (
	"errors"
	"math"
	"sort"
)

// deriveSeed computes a deterministic seed from input values using FNV-1a hash.
func deriveSeed[T Number](values []T) int64 {
	const fnvOffsetBasis = uint64(0xcbf29ce484222325)
	const fnvPrime = uint64(0x00000100000001b3)

	hash := fnvOffsetBasis
	for _, v := range values {
		bits := math.Float64bits(float64(v))
		for i := 0; i < 8; i++ {
			hash ^= (bits >> (i * 8)) & 0xff
			hash *= fnvPrime
		}
	}
	return int64(hash)
}

// centerImpl computes the median of all pairwise averages efficiently.
// Time complexity: O(n log n) expected
// Space complexity: O(n)
func centerImpl[T Number](values []T, assumeSorted bool) (float64, error) {
	n := len(values)
	if n == 0 {
		return 0, errEmptyInput
	}
	if n == 1 {
		return float64(values[0]), nil
	}
	if n == 2 {
		a := float64(values[0])
		b := float64(values[1])
		// Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b (halve before
		// summing; never overflows; operand order is irrelevant).
		return 0.5*a + 0.5*b, nil
	}

	// Create deterministic RNG from input values
	rng := NewRngFromSeed(deriveSeed(values))

	// Sort the values
	var sortedValues []T
	if assumeSorted {
		sortedValues = values
	} else {
		sortedValues = make([]T, n)
		copy(sortedValues, values)
		sort.Slice(sortedValues, func(i, j int) bool { return sortedValues[i] < sortedValues[j] })
	}

	// Calculate target median rank(s) among all pairwise sums
	totalPairs := int64(n) * int64(n+1) / 2
	medianRankLow := (totalPairs + 1) / 2 // 1-based rank
	medianRankHigh := (totalPairs + 2) / 2

	// Initialize search bounds for each row (1-based indexing)
	leftBounds := make([]int64, n)
	rightBounds := make([]int64, n)
	for i := 0; i < n; i++ {
		leftBounds[i] = int64(i + 1) // Row i pairs with columns [i+1..n]
		rightBounds[i] = int64(n)
	}

	// Start with a good pivot: sum of middle elements
	pivot := float64(sortedValues[(n-1)/2]) + float64(sortedValues[n/2])
	activeSetSize := totalPairs
	previousCount := int64(0)

	// Bound the selection loop. On valid sorted input the Monahan selection
	// converges in O(log n) iterations; this cap is far higher than ever
	// needed for sorted input but guarantees termination on misuse (e.g.,
	// assumeSorted=true on UNSORTED input, which is undefined behavior and
	// would otherwise spin forever). The cap scales with n so large valid
	// inputs are never starved. We also track no-progress (stall) on the
	// active set to bail out deterministically.
	const baseIterations = 256
	maxIterations := baseIterations + 4*n
	prevActiveSetSize := int64(-1)
	stallCount := 0
	const maxStall = 8

	for iter := 0; ; iter++ {
		if iter >= maxIterations {
			return 0, errors.New("convergence failure (pathological input)")
		}

		// === PARTITION STEP ===
		countBelowPivot := int64(0)
		currentColumn := int64(n)
		partitionCounts := make([]int64, n)

		for row := 1; row <= n; row++ {
			// Move left from current column until we find sums < pivot
			for currentColumn >= int64(row) && float64(sortedValues[row-1])+float64(sortedValues[currentColumn-1]) >= pivot {
				currentColumn--
			}

			// Count elements in this row that are < pivot
			elementsBelow := int64(0)
			if currentColumn >= int64(row) {
				elementsBelow = currentColumn - int64(row) + 1
			}
			partitionCounts[row-1] = elementsBelow
			countBelowPivot += elementsBelow
		}

		// === CONVERGENCE CHECK ===
		if countBelowPivot == previousCount {
			minActiveSum := math.Inf(1)
			maxActiveSum := math.Inf(-1)

			for i := 0; i < n; i++ {
				if leftBounds[i] > rightBounds[i] {
					continue
				}

				smallestInRow := float64(sortedValues[leftBounds[i]-1]) + float64(sortedValues[i])
				largestInRow := float64(sortedValues[rightBounds[i]-1]) + float64(sortedValues[i])

				minActiveSum = math.Min(minActiveSum, smallestInRow)
				maxActiveSum = math.Max(maxActiveSum, largestInRow)
			}

			pivot = 0.5*minActiveSum + 0.5*maxActiveSum
			if pivot <= minActiveSum || pivot > maxActiveSum {
				pivot = maxActiveSum
			}

			if minActiveSum == maxActiveSum || activeSetSize <= 2 {
				return pivot / 2, nil
			}

			continue
		}

		// === TARGET CHECK ===
		atTargetRank := countBelowPivot == medianRankLow || countBelowPivot == medianRankHigh-1

		if atTargetRank {
			largestBelowPivot := math.Inf(-1)
			smallestAtOrAbovePivot := math.Inf(1)

			for i := 0; i < n; i++ {
				countInRow := partitionCounts[i]
				rowValue := sortedValues[i]
				totalInRow := int64(n - i)

				// Find largest sum in this row that's < pivot
				if countInRow > 0 {
					lastBelowIndex := int64(i) + countInRow
					lastBelowValue := float64(rowValue) + float64(sortedValues[lastBelowIndex-1])
					largestBelowPivot = math.Max(largestBelowPivot, lastBelowValue)
				}

				// Find smallest sum in this row that's >= pivot
				if countInRow < totalInRow {
					firstAtOrAboveIndex := int64(i) + countInRow + 1
					firstAtOrAboveValue := float64(rowValue) + float64(sortedValues[firstAtOrAboveIndex-1])
					smallestAtOrAbovePivot = math.Min(smallestAtOrAbovePivot, firstAtOrAboveValue)
				}
			}

			// Calculate final result
			if medianRankLow < medianRankHigh {
				// Even total: average the two middle values. Overflow-safe: quarter each
				// pair-sum before summing (both operands can be near the double max).
				return 0.25*smallestAtOrAbovePivot + 0.25*largestBelowPivot, nil
			}
			// Odd total: return the single middle value
			needLargest := countBelowPivot == medianRankLow
			if needLargest {
				return largestBelowPivot / 2, nil
			}
			return smallestAtOrAbovePivot / 2, nil
		}

		// === UPDATE BOUNDS ===
		if countBelowPivot < medianRankLow {
			// Too few values below pivot - search higher
			for i := 0; i < n; i++ {
				leftBounds[i] = int64(i) + partitionCounts[i] + 1
			}
		} else {
			// Too many values below pivot - search lower
			for i := 0; i < n; i++ {
				rightBounds[i] = int64(i) + partitionCounts[i]
			}
		}

		// === PREPARE NEXT ITERATION ===
		previousCount = countBelowPivot

		// Recalculate active set size
		activeSetSize = 0
		for i := 0; i < n; i++ {
			rowSize := rightBounds[i] - leftBounds[i] + 1
			if rowSize > 0 {
				activeSetSize += rowSize
			}
		}

		// Stall detection: on valid sorted input the active set strictly
		// shrinks toward the target. If it fails to shrink for several
		// consecutive iterations, the input is pathological (e.g.,
		// assumeSorted=true on unsorted data) and we bail deterministically.
		if activeSetSize >= prevActiveSetSize && prevActiveSetSize >= 0 {
			stallCount++
			if stallCount >= maxStall {
				return 0, errors.New("convergence failure (pathological input)")
			}
		} else {
			stallCount = 0
		}
		prevActiveSetSize = activeSetSize

		// Choose next pivot
		if activeSetSize > 2 {
			// Use randomized row median strategy
			targetIndex := rng.UniformInt64(0, activeSetSize)
			cumulativeSize := int64(0)
			selectedRow := 0

			for i := 0; i < n; i++ {
				rowSize := rightBounds[i] - leftBounds[i] + 1
				if rowSize > 0 {
					if targetIndex < cumulativeSize+rowSize {
						selectedRow = i
						break
					}
					cumulativeSize += rowSize
				}
			}

			// Use median element of the selected row as pivot
			medianColumnInRow := (leftBounds[selectedRow] + rightBounds[selectedRow]) / 2
			pivot = float64(sortedValues[selectedRow]) + float64(sortedValues[medianColumnInRow-1])
		} else {
			// Few elements remain - use midrange strategy
			minRemainingSum := math.Inf(1)
			maxRemainingSum := math.Inf(-1)

			for i := 0; i < n; i++ {
				if leftBounds[i] > rightBounds[i] {
					continue
				}

				minInRow := float64(sortedValues[leftBounds[i]-1]) + float64(sortedValues[i])
				maxInRow := float64(sortedValues[rightBounds[i]-1]) + float64(sortedValues[i])

				minRemainingSum = math.Min(minRemainingSum, minInRow)
				maxRemainingSum = math.Max(maxRemainingSum, maxInRow)
			}

			pivot = 0.5*minRemainingSum + 0.5*maxRemainingSum
			if pivot <= minRemainingSum || pivot > maxRemainingSum {
				pivot = maxRemainingSum
			}

			if minRemainingSum == maxRemainingSum {
				return pivot / 2, nil
			}
		}
	}
}
