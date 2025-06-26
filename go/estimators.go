// Package pragmastat provides robust statistical estimators for real-world data analysis.
package pragmastat

import (
	"errors"
	"math"
	"sort"
)

// median calculates the median of a slice of float64 values.
func median(values []float64) (float64, error) {
	n := len(values)
	if n == 0 {
		return 0, errors.New("input slice cannot be empty")
	}

	// Create a copy to avoid modifying the original slice
	sorted := make([]float64, n)
	copy(sorted, values)
	sort.Float64s(sorted)

	if n%2 == 0 {
		return (sorted[n/2-1] + sorted[n/2]) / 2.0, nil
	}
	return sorted[n/2], nil
}

// Center estimates the central value of the data.
// Calculates the median of all pairwise averages (x[i] + x[j])/2.
// More robust than the mean and more efficient than the median.
func Center(x []float64) (float64, error) {
	n := len(x)
	if n == 0 {
		return 0, errors.New("input slice cannot be empty")
	}

	var pairwiseAverages []float64
	for i := 0; i < n; i++ {
		for j := i; j < n; j++ {
			pairwiseAverages = append(pairwiseAverages, (x[i]+x[j])/2.0)
		}
	}

	return median(pairwiseAverages)
}

// Spread estimates data dispersion (variability or scatter).
// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
// More robust than standard deviation and more efficient than MAD.
func Spread(x []float64) (float64, error) {
	n := len(x)
	if n == 0 {
		return 0, errors.New("input slice cannot be empty")
	}
	if n == 1 {
		return 0.0, nil
	}

	var pairwiseDiffs []float64
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			pairwiseDiffs = append(pairwiseDiffs, math.Abs(x[i]-x[j]))
		}
	}

	return median(pairwiseDiffs)
}

// Volatility measures the relative dispersion of a sample.
// Calculates the ratio of Spread to absolute Center.
// Robust alternative to the coefficient of variation.
func Volatility(x []float64) (float64, error) {
	centerVal, err := Center(x)
	if err != nil {
		return 0, err
	}
	if centerVal == 0.0 {
		return 0, errors.New("volatility is undefined when Center equals zero")
	}
	spreadVal, err := Spread(x)
	if err != nil {
		return 0, err
	}
	return spreadVal / math.Abs(centerVal), nil
}

// Precision measures the distance between two estimations of independent random samples.
// Calculated as 2 * Spread / sqrt(n). The interval center ± precision forms a range
// that probably contains the true center value.
func Precision(x []float64) (float64, error) {
	n := len(x)
	if n == 0 {
		return 0, errors.New("input slice cannot be empty")
	}
	spreadVal, err := Spread(x)
	if err != nil {
		return 0, err
	}
	return 2.0 * spreadVal / math.Sqrt(float64(n)), nil
}

// MedShift measures the typical difference between elements of x and y.
// Calculates the median of all pairwise differences (x[i] - y[j]).
// Positive values mean x is typically larger, negative means y is typically larger.
func MedShift(x, y []float64) (float64, error) {
	if len(x) == 0 || len(y) == 0 {
		return 0, errors.New("input slices cannot be empty")
	}

	var pairwiseShifts []float64
	for _, xi := range x {
		for _, yj := range y {
			pairwiseShifts = append(pairwiseShifts, xi-yj)
		}
	}

	return median(pairwiseShifts)
}

// MedRatio measures how many times larger x is compared to y.
// Calculates the median of all pairwise ratios (x[i] / y[j]).
// For example, MedRatio = 1.2 means x is typically 20% larger than y.
func MedRatio(x, y []float64) (float64, error) {
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
			pairwiseRatios = append(pairwiseRatios, xi/yj)
		}
	}

	return median(pairwiseRatios)
}

// MedSpread measures the typical variability when considering both samples together.
// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
func MedSpread(x, y []float64) (float64, error) {
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

// MedDisparity measures effect size: a normalized absolute difference between x and y.
// Calculated as MedShift / MedSpread. Robust alternative to Cohen's d.
// Returns infinity if MedSpread is zero.
func MedDisparity(x, y []float64) (float64, error) {
	medShiftVal, err := MedShift(x, y)
	if err != nil {
		return 0, err
	}
	medSpreadVal, err := MedSpread(x, y)
	if err != nil {
		return 0, err
	}
	if medSpreadVal == 0.0 {
		return math.Inf(1), nil
	}
	return medShiftVal / medSpreadVal, nil
}
