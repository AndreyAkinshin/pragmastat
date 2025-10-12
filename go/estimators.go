// Package pragmastat provides robust statistical estimators for real-world data analysis.
package pragmastat

import (
	"errors"
	"math"
	"sort"
)

// Number is a constraint that permits signed integer or floating-point type.
type Number interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 | ~float32 | ~float64
}

var errEmptyInput = errors.New("input slice cannot be empty")

// median calculates the median of a slice of numeric values.
func median[T Number](values []T) (float64, error) {
	n := len(values)
	if n == 0 {
		return 0, errors.New("input slice cannot be empty")
	}

	// Create a copy to avoid modifying the original slice
	sorted := make([]T, n)
	copy(sorted, values)
	sort.Slice(sorted, func(i, j int) bool { return sorted[i] < sorted[j] })

	if n%2 == 0 {
		return (float64(sorted[n/2-1] + sorted[n/2])) / 2.0, nil
	}
	return float64(sorted[n/2]), nil
}

// Center estimates the central value of the data.
// Calculates the median of all pairwise averages (x[i] + x[j])/2.
// More robust than the mean and more efficient than the median.
// Uses fast O(n log n) algorithm.
func Center[T Number](x []T) (float64, error) {
	return fastCenter(x)
}

// Spread estimates data dispersion (variability or scatter).
// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
// More robust than standard deviation and more efficient than MAD.
// Uses fast O(n log n) algorithm.
func Spread[T Number](x []T) (float64, error) {
	return fastSpread(x)
}

// RelSpread measures the relative dispersion of a sample.
// Calculates the ratio of Spread to absolute Center.
// Robust alternative to the coefficient of variation.
func RelSpread[T Number](x []T) (float64, error) {
	centerVal, err := Center(x)
	if err != nil {
		return 0, err
	}
	if centerVal == 0.0 {
		return 0, errors.New("RelSpread is undefined when Center equals zero")
	}
	spreadVal, err := Spread(x)
	if err != nil {
		return 0, err
	}
	return spreadVal / math.Abs(centerVal), nil
}

// Shift measures the typical difference between elements of x and y.
// Calculates the median of all pairwise differences (x[i] - y[j]).
// Positive values mean x is typically larger, negative means y is typically larger.
// Uses fast O((m + n) * log(precision)) algorithm.
func Shift[T Number](x, y []T) (float64, error) {
	return fastShift(x, y)
}

// Ratio measures how many times larger x is compared to y.
// Calculates the median of all pairwise ratios (x[i] / y[j]).
// For example, Ratio = 1.2 means x is typically 20% larger than y.
func Ratio[T Number](x, y []T) (float64, error) {
	if len(x) == 0 || len(y) == 0 {
		return 0, errors.New("input slices cannot be empty")
	}

	// Check that all y values are strictly positive
	for _, yj := range y {
		if yj <= 0 {
			return 0, errors.New("all values in y must be strictly positive")
		}
	}

	var pairwiseRatios []float64
	for _, xi := range x {
		for _, yj := range y {
			pairwiseRatios = append(pairwiseRatios, float64(xi)/float64(yj))
		}
	}

	return median(pairwiseRatios)
}

// AvgSpread measures the typical variability when considering both samples together.
// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
func AvgSpread[T Number](x, y []T) (float64, error) {
	if len(x) == 0 || len(y) == 0 {
		return 0, errors.New("input slices cannot be empty")
	}

	n := float64(len(x))
	m := float64(len(y))

	spreadX, err := Spread(x)
	if err != nil {
		return 0, err
	}
	spreadY, err := Spread(y)
	if err != nil {
		return 0, err
	}

	return (n*spreadX + m*spreadY) / (n + m), nil
}

// Disparity measures effect size: a normalized difference between x and y.
// Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
// Returns infinity if AvgSpread is zero.
func Disparity[T Number](x, y []T) (float64, error) {
	shiftVal, err := Shift(x, y)
	if err != nil {
		return 0, err
	}
	avgSpreadVal, err := AvgSpread(x, y)
	if err != nil {
		return 0, err
	}
	if avgSpreadVal == 0.0 {
		return math.Inf(1), nil
	}
	return shiftVal / avgSpreadVal, nil
}
