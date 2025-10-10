package pragmastat

import (
	"math"
	"math/rand"
	"sort"
)

// fastCenter computes the median of all pairwise averages efficiently.
// Time complexity: O(n log n) expected
// Space complexity: O(n)
func fastCenter[T Number](values []T) (float64, error) {
	n := len(values)
	if n == 0 {
		return 0, errEmptyInput
	}
	if n == 1 {
		return float64(values[0]), nil
	}
	if n == 2 {
		return (float64(values[0] + values[1])) / 2, nil
	}

	// Sort the values
	sortedValues := make([]T, n)
	copy(sortedValues, values)
	sort.Slice(sortedValues, func(i, j int) bool { return sortedValues[i] < sortedValues[j] })

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
	pivot := float64(sortedValues[(n-1)/2] + sortedValues[n/2])
	activeSetSize := totalPairs
	previousCount := int64(0)

	for {
		// === PARTITION STEP ===
		countBelowPivot := int64(0)
		currentColumn := int64(n)
		partitionCounts := make([]int64, n)

		for row := 1; row <= n; row++ {
			// Move left from current column until we find sums < pivot
			for currentColumn >= int64(row) && float64(sortedValues[row-1]+sortedValues[currentColumn-1]) >= pivot {
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

				smallestInRow := float64(sortedValues[leftBounds[i]-1] + sortedValues[i])
				largestInRow := float64(sortedValues[rightBounds[i]-1] + sortedValues[i])

				minActiveSum = math.Min(minActiveSum, smallestInRow)
				maxActiveSum = math.Max(maxActiveSum, largestInRow)
			}

			pivot = (minActiveSum + maxActiveSum) / 2
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
					lastBelowValue := float64(rowValue + sortedValues[lastBelowIndex-1])
					largestBelowPivot = math.Max(largestBelowPivot, lastBelowValue)
				}

				// Find smallest sum in this row that's >= pivot
				if countInRow < totalInRow {
					firstAtOrAboveIndex := int64(i) + countInRow + 1
					firstAtOrAboveValue := float64(rowValue + sortedValues[firstAtOrAboveIndex-1])
					smallestAtOrAbovePivot = math.Min(smallestAtOrAbovePivot, firstAtOrAboveValue)
				}
			}

			// Calculate final result
			if medianRankLow < medianRankHigh {
				// Even total: average the two middle values
				return (smallestAtOrAbovePivot + largestBelowPivot) / 4, nil
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

		// Choose next pivot
		if activeSetSize > 2 {
			// Use randomized row median strategy
			targetIndex := rand.Int63n(activeSetSize)
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
			pivot = float64(sortedValues[selectedRow] + sortedValues[medianColumnInRow-1])
		} else {
			// Few elements remain - use midrange strategy
			minRemainingSum := math.Inf(1)
			maxRemainingSum := math.Inf(-1)

			for i := 0; i < n; i++ {
				if leftBounds[i] > rightBounds[i] {
					continue
				}

				minInRow := float64(sortedValues[leftBounds[i]-1] + sortedValues[i])
				maxInRow := float64(sortedValues[rightBounds[i]-1] + sortedValues[i])

				minRemainingSum = math.Min(minRemainingSum, minInRow)
				maxRemainingSum = math.Max(maxRemainingSum, maxInRow)
			}

			pivot = (minRemainingSum + maxRemainingSum) / 2
			if pivot <= minRemainingSum || pivot > maxRemainingSum {
				pivot = maxRemainingSum
			}

			if minRemainingSum == maxRemainingSum {
				return pivot / 2, nil
			}
		}
	}
}
