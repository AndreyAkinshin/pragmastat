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
	spreadVal, err := fastSpread(x)
	if err != nil {
		return 0, err
	}
	if spreadVal <= 0 {
		return 0, NewSparityError(SubjectX)
	}
	return spreadVal, nil
}

// RelSpread measures the relative dispersion of a sample.
// Calculates the ratio of Spread to absolute Center.
// Robust alternative to the coefficient of variation.
//
// Deprecated: Use Spread(x) / math.Abs(Center(x)) instead.
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

// avgSpread measures the typical variability when considering both samples together.
// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
//
// Assumptions:
//   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
//   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
func avgSpread[T Number](x, y []T) (float64, error) {
	// Check validity for x (priority 0, subject x)
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	// Check validity for y (priority 0, subject y)
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}

	n := float64(len(x))
	m := float64(len(y))

	spreadX, err := fastSpread(x)
	if err != nil {
		return 0, err
	}
	if spreadX <= 0 {
		return 0, NewSparityError(SubjectX)
	}
	spreadY, err := fastSpread(y)
	if err != nil {
		return 0, err
	}
	if spreadY <= 0 {
		return 0, NewSparityError(SubjectY)
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

	n := float64(len(x))
	m := float64(len(y))

	spreadX, err := fastSpread(x)
	if err != nil {
		return 0, err
	}
	if spreadX <= 0 {
		return 0, NewSparityError(SubjectX)
	}
	spreadY, err := fastSpread(y)
	if err != nil {
		return 0, err
	}
	if spreadY <= 0 {
		return 0, NewSparityError(SubjectY)
	}

	// Calculate shift (we know inputs are valid)
	shiftVal, err := fastShift(x, y)
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

// BoundsConfig holds optional parameters for bounds estimators.
type BoundsConfig struct {
	Misrate float64 // Misclassification rate; 0 means DefaultMisrate
	Seed    string  // Random seed; empty means non-deterministic
}

func parseBoundsConfig(config []BoundsConfig) (float64, string) {
	if len(config) > 1 {
		panic("pragmastat: at most one BoundsConfig allowed")
	}
	mr := DefaultMisrate
	seed := ""
	if len(config) > 0 {
		if config[0].Misrate != 0 {
			mr = config[0].Misrate
		}
		seed = config[0].Seed
	}
	return mr, seed
}

// ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
// The misrate represents the probability that the true shift falls outside the computed bounds.
// This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
func ShiftBounds[T Number](x, y []T, config ...BoundsConfig) (Bounds, error) {
	mr, _ := parseBoundsConfig(config)

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
func RatioBounds[T Number](x, y []T, config ...BoundsConfig) (Bounds, error) {
	mr, _ := parseBoundsConfig(config)

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
	logBounds, err := ShiftBounds(logX, logY, BoundsConfig{Misrate: mr})
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
func CenterBounds[T Number](x []T, config ...BoundsConfig) (Bounds, error) {
	mr, _ := parseBoundsConfig(config)

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

// SpreadBounds provides distribution-free bounds for Spread using disjoint pairs.
func SpreadBounds[T Number](x []T, config ...BoundsConfig) (Bounds, error) {
	mr, seed := parseBoundsConfig(config)
	var rng *Rng
	if seed != "" {
		rng = NewRngFromString(seed)
	} else {
		rng = NewRng()
	}
	return spreadBoundsWithRng(x, mr, rng)
}

func avgSpreadBoundsWithRngs[T Number](x, y []T, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

	// Check misrate domain
	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	m := len(y)
	if n < 2 {
		return Bounds{}, NewDomainError(SubjectX)
	}
	if m < 2 {
		return Bounds{}, NewDomainError(SubjectY)
	}

	alpha := misrate / 2
	minX, err := minAchievableMisrateOneSample(n / 2)
	if err != nil {
		return Bounds{}, err
	}
	minY, err := minAchievableMisrateOneSample(m / 2)
	if err != nil {
		return Bounds{}, err
	}
	if alpha < minX || alpha < minY {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	spreadX, err := fastSpread(x)
	if err != nil {
		return Bounds{}, err
	}
	if spreadX <= 0 {
		return Bounds{}, NewSparityError(SubjectX)
	}
	spreadY, err := fastSpread(y)
	if err != nil {
		return Bounds{}, err
	}
	if spreadY <= 0 {
		return Bounds{}, NewSparityError(SubjectY)
	}

	boundsX, err := spreadBoundsWithRng(x, alpha, rngX)
	if err != nil {
		return Bounds{}, err
	}
	boundsY, err := spreadBoundsWithRng(y, alpha, rngY)
	if err != nil {
		return Bounds{}, err
	}

	wx := float64(n) / float64(n+m)
	wy := float64(m) / float64(n+m)

	return Bounds{
		Lower: wx*boundsX.Lower + wy*boundsY.Lower,
		Upper: wx*boundsX.Upper + wy*boundsY.Upper,
	}, nil
}

// DisparityBounds provides distribution-free bounds for the Disparity estimator (Shift / AvgSpread)
// using Bonferroni combination of ShiftBounds and AvgSpreadBounds.
func DisparityBounds[T Number](x, y []T, config ...BoundsConfig) (Bounds, error) {
	mr, seed := parseBoundsConfig(config)
	var rngX, rngY *Rng
	if seed != "" {
		rngX = NewRngFromString(seed)
		rngY = NewRngFromString(seed)
	} else {
		rngX = NewRng()
		rngY = NewRng()
	}
	return disparityBoundsWithRngs(x, y, mr, rngX, rngY)
}

func disparityBoundsWithRngs[T Number](x, y []T, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

	// Check misrate domain
	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	m := len(y)
	if n < 2 {
		return Bounds{}, NewDomainError(SubjectX)
	}
	if m < 2 {
		return Bounds{}, NewDomainError(SubjectY)
	}

	minShift, err := minAchievableMisrateTwoSample(n, m)
	if err != nil {
		return Bounds{}, err
	}
	minX, err := minAchievableMisrateOneSample(n / 2)
	if err != nil {
		return Bounds{}, err
	}
	minY, err := minAchievableMisrateOneSample(m / 2)
	if err != nil {
		return Bounds{}, err
	}
	minAvg := 2.0 * math.Max(minX, minY)

	if misrate < minShift+minAvg {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	extra := misrate - (minShift + minAvg)
	alphaShift := minShift + extra/2.0
	alphaAvg := minAvg + extra/2.0

	spreadXVal, err := fastSpread(x)
	if err != nil {
		return Bounds{}, err
	}
	if spreadXVal <= 0 {
		return Bounds{}, NewSparityError(SubjectX)
	}
	spreadYVal, err := fastSpread(y)
	if err != nil {
		return Bounds{}, err
	}
	if spreadYVal <= 0 {
		return Bounds{}, NewSparityError(SubjectY)
	}

	sb, err := ShiftBounds(x, y, BoundsConfig{Misrate: alphaShift})
	if err != nil {
		return Bounds{}, err
	}
	ab, err := avgSpreadBoundsWithRngs(x, y, alphaAvg, rngX, rngY)
	if err != nil {
		return Bounds{}, err
	}

	la := ab.Lower
	ua := ab.Upper
	ls := sb.Lower
	us := sb.Upper

	if la > 0.0 {
		r1 := ls / la
		r2 := ls / ua
		r3 := us / la
		r4 := us / ua
		lower := math.Min(math.Min(r1, r2), math.Min(r3, r4))
		upper := math.Max(math.Max(r1, r2), math.Max(r3, r4))
		return Bounds{Lower: lower, Upper: upper}, nil
	}

	if ua <= 0.0 {
		if ls == 0.0 && us == 0.0 {
			return Bounds{Lower: 0.0, Upper: 0.0}, nil
		}
		if ls >= 0.0 {
			return Bounds{Lower: 0.0, Upper: math.Inf(1)}, nil
		}
		if us <= 0.0 {
			return Bounds{Lower: math.Inf(-1), Upper: 0.0}, nil
		}
		return Bounds{Lower: math.Inf(-1), Upper: math.Inf(1)}, nil
	}

	// Default: ua > 0 && la <= 0
	if ls > 0.0 {
		return Bounds{Lower: ls / ua, Upper: math.Inf(1)}, nil
	}
	if us < 0.0 {
		return Bounds{Lower: math.Inf(-1), Upper: us / ua}, nil
	}
	if ls == 0.0 && us == 0.0 {
		return Bounds{Lower: 0.0, Upper: 0.0}, nil
	}
	if ls == 0.0 && us > 0.0 {
		return Bounds{Lower: 0.0, Upper: math.Inf(1)}, nil
	}
	if ls < 0.0 && us == 0.0 {
		return Bounds{Lower: math.Inf(-1), Upper: 0.0}, nil
	}

	return Bounds{Lower: math.Inf(-1), Upper: math.Inf(1)}, nil
}

func spreadBoundsWithRng[T Number](x []T, misrate float64, rng *Rng) (Bounds, error) {
	// Check validity (priority 0)
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}

	// Check misrate domain
	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	m := n / 2

	minMisrate, err := minAchievableMisrateOneSample(m)
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	if len(x) < 2 {
		return Bounds{}, NewSparityError(SubjectX)
	}
	spreadVal, err := fastSpread(x)
	if err != nil {
		return Bounds{}, err
	}
	if spreadVal <= 0 {
		return Bounds{}, NewSparityError(SubjectX)
	}

	margin, err := signMarginRandomized(m, misrate, rng)
	if err != nil {
		return Bounds{}, err
	}

	halfMargin := margin / 2
	maxHalfMargin := (m - 1) / 2
	if halfMargin > maxHalfMargin {
		halfMargin = maxHalfMargin
	}

	kLeft := halfMargin + 1
	kRight := m - halfMargin

	// Create index array and shuffle
	indices := make([]int, n)
	for i := range indices {
		indices[i] = i
	}
	shuffled := Shuffle(rng, indices)

	// Compute pairwise absolute differences
	diffs := make([]float64, m)
	for i := 0; i < m; i++ {
		diffs[i] = math.Abs(float64(x[shuffled[2*i]]) - float64(x[shuffled[2*i+1]]))
	}
	sort.Float64s(diffs)

	return Bounds{Lower: diffs[kLeft-1], Upper: diffs[kRight-1]}, nil
}
