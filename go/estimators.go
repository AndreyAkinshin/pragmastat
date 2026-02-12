// Package pragmastat provides robust statistical estimators for real-world data analysis.
package pragmastat

import (
	"errors"
	"math"
	"sort"
)

// Number is a constraint that permits signed/unsigned integer or floating-point type.
type Number interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 |
		~float32 | ~float64
}

var errEmptyInput = errors.New("input slice cannot be empty")

// Cross-language consistency: all implementations keep median as internal utility
func median[T Number](values []T) float64 { //nolint:unused
	sorted := make([]T, len(values))
	copy(sorted, values)
	sort.Slice(sorted, func(i, j int) bool { return sorted[i] < sorted[j] })
	mid := len(sorted) / 2
	if len(sorted)%2 == 0 {
		return (float64(sorted[mid-1]) + float64(sorted[mid])) / 2
	}
	return float64(sorted[mid])
}

// Center estimates the central value of the data.
// Calculates the median of all pairwise averages (x[i] + x[j])/2.
// More robust than the mean and more efficient than the median.
// Uses fast O(n log n) algorithm.
func Center[T Number](x []T) (float64, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	return fastCenter(x)
}

// Spread estimates data dispersion (variability or scatter).
// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
// More robust than standard deviation and more efficient than MAD.
// Uses fast O(n log n) algorithm.
//
// Assumptions:
//   - sparity(x) - sample must be non tie-dominant (Spread > 0)
func Spread[T Number](x []T) (float64, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check sparity (priority 2)
	if err := checkSparity(x, SubjectX); err != nil {
		return 0, err
	}
	return fastSpread(x)
}

// RelSpread measures the relative dispersion of a sample.
// Calculates the ratio of Spread to absolute Center.
// Robust alternative to the coefficient of variation.
//
// Assumptions:
//   - positivity(x) - all values must be strictly positive (ensures Center > 0)
func RelSpread[T Number](x []T) (float64, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check positivity (priority 1)
	if err := checkPositivity(x, SubjectX); err != nil {
		return 0, err
	}
	// Calculate center (we know x is valid)
	centerVal, err := fastCenter(x)
	if err != nil {
		return 0, err
	}
	// Calculate spread (using internal implementation since we already validated)
	spreadVal, err := fastSpread(x)
	if err != nil {
		return 0, err
	}
	// center is guaranteed positive because all values are positive
	return spreadVal / math.Abs(centerVal), nil
}

// Shift measures the typical difference between elements of x and y.
// Calculates the median of all pairwise differences (x[i] - y[j]).
// Positive values mean x is typically larger, negative means y is typically larger.
// Uses fast O((m + n) * log(precision)) algorithm.
func Shift[T Number](x, y []T) (float64, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}
	return fastShift(x, y)
}

// Ratio measures how many times larger x is compared to y.
// Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
// Equivalent to: exp(Shift(log(x), log(y)))
// For example, Ratio = 1.2 means x is typically 20% larger than y.
// Uses fast O((m + n) * log(precision)) algorithm.
//
// Assumptions:
//   - positivity(x) - all values in x must be strictly positive
//   - positivity(y) - all values in y must be strictly positive
func Ratio[T Number](x, y []T) (float64, error) {
	// Check validity for x (priority 0, subject x)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check validity for y (priority 0, subject y)
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}
	// Check positivity for x (priority 1, subject x)
	if err := checkPositivity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check positivity for y (priority 1, subject y)
	if err := checkPositivity(y, SubjectY); err != nil {
		return 0, err
	}

	result, err := fastRatioQuantiles(x, y, []float64{0.5}, false)
	if err != nil {
		return 0, err
	}
	return result[0], nil
}

// AvgSpread measures the typical variability when considering both samples together.
// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
//
// Assumptions:
//   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
//   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
func AvgSpread[T Number](x, y []T) (float64, error) {
	// Check validity for x (priority 0, subject x)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check validity for y (priority 0, subject y)
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}
	// Check sparity for x (priority 2, subject x)
	if err := checkSparity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check sparity for y (priority 2, subject y)
	if err := checkSparity(y, SubjectY); err != nil {
		return 0, err
	}

	n := float64(len(x))
	m := float64(len(y))

	// Calculate spreads (using internal implementation since we already validated)
	spreadX, err := fastSpread(x)
	if err != nil {
		return 0, err
	}
	spreadY, err := fastSpread(y)
	if err != nil {
		return 0, err
	}

	return (n*spreadX + m*spreadY) / (n + m), nil
}

// Disparity measures effect size: a normalized difference between x and y.
// Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
//
// Assumptions:
//   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
//   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
func Disparity[T Number](x, y []T) (float64, error) {
	// Check validity for x (priority 0, subject x)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check validity for y (priority 0, subject y)
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}
	// Check sparity for x (priority 2, subject x)
	if err := checkSparity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check sparity for y (priority 2, subject y)
	if err := checkSparity(y, SubjectY); err != nil {
		return 0, err
	}

	n := float64(len(x))
	m := float64(len(y))

	// Calculate shift (we know inputs are valid)
	shiftVal, err := fastShift(x, y)
	if err != nil {
		return 0, err
	}
	// Calculate avg_spread (using internal implementation since we already validated)
	spreadX, err := fastSpread(x)
	if err != nil {
		return 0, err
	}
	spreadY, err := fastSpread(y)
	if err != nil {
		return 0, err
	}
	avgSpreadVal := (n*spreadX + m*spreadY) / (n + m)

	return shiftVal / avgSpreadVal, nil
}

// Bounds represents an interval [Lower, Upper].
type Bounds struct {
	Lower float64
	Upper float64
}

// ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
// The misrate represents the probability that the true shift falls outside the computed bounds.
// This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
func ShiftBounds[T Number](x, y []T, misrate ...float64) (Bounds, error) {
	mr := DefaultMisrate
	if len(misrate) > 0 {
		mr = misrate[0]
	}

	// Check validity for x
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	// Check validity for y
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(mr) || mr < 0 || mr > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	m := len(y)

	minMisrate, err := minAchievableMisrateTwoSample(n, m)
	if err != nil {
		return Bounds{}, err
	}
	if mr < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	// Sort both arrays
	xs := make([]T, n)
	ys := make([]T, m)
	copy(xs, x)
	copy(ys, y)
	sort.Slice(xs, func(i, j int) bool { return xs[i] < xs[j] })
	sort.Slice(ys, func(i, j int) bool { return ys[i] < ys[j] })

	total := int64(n) * int64(m)

	// Special case: when there's only one pairwise difference, bounds collapse to a single value
	if total == 1 {
		value := float64(xs[0] - ys[0])
		return Bounds{Lower: value, Upper: value}, nil
	}

	margin, err := pairwiseMargin(n, m, mr)
	if err != nil {
		return Bounds{}, err
	}
	halfMargin := int64(margin / 2)
	maxHalfMargin := (total - 1) / 2
	if halfMargin > maxHalfMargin {
		halfMargin = maxHalfMargin
	}
	kLeft := halfMargin
	kRight := (total - 1) - halfMargin

	// Compute quantile positions
	denominator := float64(total - 1)
	if denominator <= 0 {
		denominator = 1
	}

	p := []float64{float64(kLeft) / denominator, float64(kRight) / denominator}
	bounds, err := fastShiftQuantiles(xs, ys, p, true)
	if err != nil {
		return Bounds{}, err
	}

	lower := bounds[0]
	upper := bounds[1]
	if lower > upper {
		lower, upper = upper, lower
	}

	return Bounds{Lower: lower, Upper: upper}, nil
}

// RatioBounds provides bounds on the Ratio estimator with specified misclassification rate.
//
// Computes bounds via log-transformation and ShiftBounds delegation:
// RatioBounds(x, y, misrate) = exp(ShiftBounds(log(x), log(y), misrate))
//
// Assumptions:
//   - positivity(x) - all values in x must be strictly positive
//   - positivity(y) - all values in y must be strictly positive
func RatioBounds[T Number](x, y []T, misrate ...float64) (Bounds, error) {
	mr := DefaultMisrate
	if len(misrate) > 0 {
		mr = misrate[0]
	}

	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(mr) || mr < 0 || mr > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	minMisrate, err := minAchievableMisrateTwoSample(len(x), len(y))
	if err != nil {
		return Bounds{}, err
	}
	if mr < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	// Log-transform samples (includes positivity check)
	logX, err := Log(x, SubjectX)
	if err != nil {
		return Bounds{}, err
	}
	logY, err := Log(y, SubjectY)
	if err != nil {
		return Bounds{}, err
	}

	// Delegate to ShiftBounds in log-space
	logBounds, err := ShiftBounds(logX, logY, mr)
	if err != nil {
		return Bounds{}, err
	}

	// Exp-transform back to ratio-space
	return Bounds{
		Lower: math.Exp(logBounds.Lower),
		Upper: math.Exp(logBounds.Upper),
	}, nil
}

// CenterBounds provides exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
// Requires weak symmetry assumption: distribution symmetric around unknown center.
func CenterBounds[T Number](x []T, misrate ...float64) (Bounds, error) {
	mr := DefaultMisrate
	if len(misrate) > 0 {
		mr = misrate[0]
	}

	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(mr) || mr < 0 || mr > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	if n < 2 {
		return Bounds{}, NewDomainError(SubjectX)
	}

	minMisrate, err := minAchievableMisrateOneSample(n)
	if err != nil {
		return Bounds{}, err
	}
	if mr < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	margin, err := signedRankMargin(n, mr)
	if err != nil {
		return Bounds{}, err
	}

	totalPairs := int64(n) * int64(n+1) / 2

	halfMargin := int64(margin / 2)
	maxHalfMargin := (totalPairs - 1) / 2
	if halfMargin > maxHalfMargin {
		halfMargin = maxHalfMargin
	}

	// kLeft and kRight are 1-based ranks (fastCenterQuantileBounds uses 1-based rank semantics)
	kLeft := halfMargin + 1
	kRight := totalPairs - halfMargin

	// Sort the input
	sorted := make([]float64, n)
	for i, v := range x {
		sorted[i] = float64(v)
	}
	sort.Float64s(sorted)

	lo, hi := fastCenterQuantileBounds(sorted, kLeft, kRight)
	return Bounds{Lower: lo, Upper: hi}, nil
}
