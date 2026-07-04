// Package pragmastat provides robust statistical estimators for real-world data analysis.
package pragmastat

import (
	"fmt"
	"math"
	"sort"
)

// Number is a constraint that permits signed/unsigned integer or floating-point type.
type Number interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 |
		~float32 | ~float64
}

var errEmptyInput = fmt.Errorf("input slice cannot be empty")

// Bounds represents an interval [Lower, Upper] with an associated measurement unit.
type Bounds struct {
	Lower float64
	Upper float64
	Unit  *MeasurementUnit
}

// Contains returns true if value is within [Lower, Upper].
func (b Bounds) Contains(value float64) bool {
	return b.Lower <= value && value <= b.Upper
}

func (b Bounds) String() string {
	if b.Unit != nil && len(b.Unit.Abbreviation) > 0 {
		return fmt.Sprintf("[%v;%v] %s", b.Lower, b.Upper, b.Unit.Abbreviation)
	}
	return fmt.Sprintf("[%v;%v]", b.Lower, b.Upper)
}

// =============================================================================
// Raw (slice-based) public API
//
// The package-level estimator functions form the raw API: they accept the
// native []float64 slice directly and return plain, UNITLESS results (float64
// or Bounds with NumberUnit). They contain the single, canonical implementation
// of each estimator; the Sample methods below are thin adapters that handle
// units and reuse the cached sorted view.
//
// The assumeSorted flag lets callers with already-sorted data skip the internal
// sort:
//
//   - Order-INDEPENDENT estimators (Center, Spread, Shift, Ratio, Disparity,
//     CenterBounds, ShiftBounds, RatioBounds): assumeSorted=true means "the
//     input is already sorted ascending — skip the internal sort." This changes
//     the computation path. The caller is responsible: passing true on UNSORTED
//     input gives a wrong result (undefined behavior).
//
//   - SHUFFLE-based bounds (SpreadBounds, DisparityBounds): the disjoint-pair
//     shuffle ALWAYS runs on the passed slice's order. For SpreadBounds the flag
//     therefore never changes the result (it only skips the sparity (spread > 0)
//     re-sort). For DisparityBounds the embedded shift-bounds sub-computation is
//     order-independent only given sorted input, so assumeSorted=true on UNSORTED
//     input is undefined behavior and CAN change the result. Same misuse contract.
// =============================================================================

// checkValidity returns a validity error if the slice is empty or contains any
// NaN or infinite value.
func checkValidity(x []float64, subject Subject) error {
	if len(x) == 0 {
		return NewValidityError(subject)
	}
	for _, v := range x {
		if math.IsNaN(v) || math.IsInf(v, 0) {
			return NewValidityError(subject)
		}
	}
	return nil
}

// Center estimates the central value of the data.
// Calculates the median of all pairwise averages (x[i] + x[j])/2.
//
// If assumeSorted is true, x is assumed already sorted ascending and the
// internal sort is skipped (undefined behavior on unsorted input).
func Center(x []float64, assumeSorted bool) (float64, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	return centerImpl(x, assumeSorted)
}

// Spread estimates data dispersion (variability or scatter).
// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
//
// Assumptions:
//   - sparity(x) - sample must be non tie-dominant (Spread > 0)
//
// If assumeSorted is true, x is assumed already sorted ascending and the
// internal sort is skipped (undefined behavior on unsorted input).
func Spread(x []float64, assumeSorted bool) (float64, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	spreadVal, err := spreadImpl(x, assumeSorted)
	if err != nil {
		return 0, err
	}
	if spreadVal <= 0 {
		return 0, NewSparityError(SubjectX)
	}
	return spreadVal, nil
}

// Shift measures the typical difference between elements of x and y.
// Calculates the median of all pairwise differences (x[i] - y[j]).
//
// If assumeSorted is true, both x and y are assumed already sorted ascending
// and the internal sort is skipped (undefined behavior on unsorted input).
func Shift(x, y []float64, assumeSorted bool) (float64, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}
	result, err := shiftQuantilesImpl(x, y, []float64{0.5}, assumeSorted)
	if err != nil {
		return 0, err
	}
	return result[0], nil
}

// Ratio measures how many times larger x is compared to y.
// Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
//
// Assumptions:
//   - positivity(x) - all values in x must be strictly positive
//   - positivity(y) - all values in y must be strictly positive
//
// If assumeSorted is true, both x and y are assumed already sorted ascending
// and the internal sort is skipped (undefined behavior on unsorted input).
func Ratio(x, y []float64, assumeSorted bool) (float64, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}
	for _, v := range x {
		if v <= 0 {
			return 0, NewPositivityError(SubjectX)
		}
	}
	for _, v := range y {
		if v <= 0 {
			return 0, NewPositivityError(SubjectY)
		}
	}
	result, err := ratioQuantilesImpl(x, y, []float64{0.5}, assumeSorted)
	if err != nil {
		return 0, err
	}
	return result[0], nil
}

// avgSpread measures the typical variability when considering both samples together.
// Internal estimator backing the Sample-based avgSpread method. Operates on raw
// slices. Disparity does not call this; it inlines the equivalent computation.
func avgSpread(x, y []float64, assumeSorted bool) (float64, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}

	n := float64(len(x))
	m := float64(len(y))

	spreadX, err := spreadImpl(x, assumeSorted)
	if err != nil {
		return 0, err
	}
	if spreadX <= 0 {
		return 0, NewSparityError(SubjectX)
	}
	spreadY, err := spreadImpl(y, assumeSorted)
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
//
// If assumeSorted is true, both x and y are assumed already sorted ascending
// and the internal sort is skipped (undefined behavior on unsorted input).
func Disparity(x, y []float64, assumeSorted bool) (float64, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return 0, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return 0, err
	}

	n := float64(len(x))
	m := float64(len(y))

	spreadX, err := spreadImpl(x, assumeSorted)
	if err != nil {
		return 0, err
	}
	if spreadX <= 0 {
		return 0, NewSparityError(SubjectX)
	}
	spreadY, err := spreadImpl(y, assumeSorted)
	if err != nil {
		return 0, err
	}
	if spreadY <= 0 {
		return 0, NewSparityError(SubjectY)
	}

	shiftVal, err := shiftQuantilesImpl(x, y, []float64{0.5}, assumeSorted)
	if err != nil {
		return 0, err
	}
	avgSpreadVal := (n*spreadX + m*spreadY) / (n + m)

	return shiftVal[0] / avgSpreadVal, nil
}

// ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
//
// If assumeSorted is true, both x and y are assumed already sorted ascending
// and the internal sort is skipped (undefined behavior on unsorted input).
func ShiftBounds(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	m := len(y)

	minMisrate, err := minAchievableMisrateTwoSample(n, m)
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	xSorted := sortedOne(x, assumeSorted)
	ySorted := sortedOne(y, assumeSorted)

	total := int64(n) * int64(m)

	if total == 1 {
		value := xSorted[0] - ySorted[0]
		return Bounds{Lower: value, Upper: value, Unit: NumberUnit}, nil
	}

	margin, err := pairwiseMargin(n, m, misrate)
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

	// total >= 2 here (the total == 1 case returned early above), so total-1 >= 1.
	denominator := float64(total - 1)
	p := []float64{float64(kLeft) / denominator, float64(kRight) / denominator}
	bounds, err := shiftQuantilesImpl(xSorted, ySorted, p, true)
	if err != nil {
		return Bounds{}, err
	}

	lower := bounds[0]
	upper := bounds[1]
	if lower > upper {
		lower, upper = upper, lower
	}

	return Bounds{Lower: lower, Upper: upper, Unit: NumberUnit}, nil
}

// RatioBounds provides bounds on the Ratio estimator with specified misclassification rate.
//
// Assumptions:
//   - positivity(x) - all values in x must be strictly positive
//   - positivity(y) - all values in y must be strictly positive
//
// If assumeSorted is true, both x and y are assumed already sorted ascending
// and the internal sort is skipped (undefined behavior on unsorted input).
func RatioBounds(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	minMisrate, err := minAchievableMisrateTwoSample(len(x), len(y))
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	logX, err := Log(x, SubjectX)
	if err != nil {
		return Bounds{}, err
	}
	logY, err := Log(y, SubjectY)
	if err != nil {
		return Bounds{}, err
	}

	// log is monotonic: sorted positive input -> sorted log output, so the
	// assumeSorted flag carries through unchanged.
	logBounds, err := ShiftBounds(logX, logY, misrate, assumeSorted)
	if err != nil {
		return Bounds{}, err
	}

	return Bounds{
		Lower: math.Exp(logBounds.Lower),
		Upper: math.Exp(logBounds.Upper),
		Unit:  NumberUnit,
	}, nil
}

// CenterBounds provides exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
// Requires weak symmetry assumption: distribution symmetric around unknown center.
//
// If assumeSorted is true, x is assumed already sorted ascending and the
// internal sort is skipped (undefined behavior on unsorted input).
func CenterBounds(x []float64, misrate float64, assumeSorted bool) (Bounds, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
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
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	margin, err := signedRankMargin(n, misrate)
	if err != nil {
		return Bounds{}, err
	}

	totalPairs := int64(n) * int64(n+1) / 2

	halfMargin := int64(margin / 2)
	maxHalfMargin := (totalPairs - 1) / 2
	if halfMargin > maxHalfMargin {
		halfMargin = maxHalfMargin
	}

	kLeft := halfMargin + 1
	kRight := totalPairs - halfMargin

	lo, hi := centerQuantileBoundsImpl(sortedOne(x, assumeSorted), kLeft, kRight)
	return Bounds{Lower: lo, Upper: hi, Unit: NumberUnit}, nil
}

// SpreadBounds provides distribution-free bounds for Spread using disjoint pairs.
//
// The disjoint-pair shuffle always runs on x's current order, so assumeSorted
// never changes the result; it only skips the internal sort of the sparity
// check. Passing assumeSorted=true on unsorted input is undefined behavior for
// that check.
func SpreadBounds(x []float64, misrate float64, assumeSorted bool) (Bounds, error) {
	return spreadBoundsImpl(x, sortedView(x, assumeSorted), misrate, NewRng())
}

// SpreadBoundsWithSeed provides distribution-free bounds for Spread with deterministic randomization.
func SpreadBoundsWithSeed(x []float64, misrate float64, seed string, assumeSorted bool) (Bounds, error) {
	return spreadBoundsImpl(x, sortedView(x, assumeSorted), misrate, NewRngFromString(seed))
}

// DisparityBounds provides distribution-free bounds for the Disparity estimator.
//
// The disjoint-pair shuffle always runs on the original order of x and y.
// When the input is genuinely sorted, assumeSorted only skips internal sorts
// of the sub-computations and does not change the result. On UNSORTED input,
// however, assumeSorted=true is undefined behavior and CAN change the result:
// the embedded order-independent shift bounds consumes the passed slice as a
// sorted view, so an unsorted slice yields different (incorrect) bounds.
func DisparityBounds(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error) {
	return disparityBoundsImpl(x, sortedView(x, assumeSorted), y, sortedView(y, assumeSorted), misrate, NewRng(), NewRng())
}

// DisparityBoundsWithSeed provides distribution-free bounds for Disparity with deterministic randomization.
func DisparityBoundsWithSeed(x, y []float64, misrate float64, seed string, assumeSorted bool) (Bounds, error) {
	return disparityBoundsImpl(x, sortedView(x, assumeSorted), y, sortedView(y, assumeSorted), misrate, NewRngFromString(seed), NewRngFromString(seed))
}

// =============================================================================
// Shuffle-based bounds: single implementations
//
// x/y are ALWAYS in their original order (the disjoint-pair shuffle is
// order-dependent). sortedX/sortedY, when non-nil, are pre-sorted views used
// only to speed up the order-independent sparity check. NEVER pass the sorted
// view as the shuffle slice.
// =============================================================================

// sortedView maps the public assumeSorted flag to the internal optional
// pre-sorted view: when the caller's slice is already sorted, it doubles as the
// sorted view for the order-independent sparity check (skipping a re-sort). The
// shuffle always runs on the original slice regardless, so the flag never
// changes the result.
func sortedView(x []float64, assumeSorted bool) []float64 {
	if assumeSorted {
		return x
	}
	return nil
}

// spreadForSparity computes the spread value for the sparity check. The result
// is order-independent, so a pre-sorted view (when available) is used to skip
// re-sorting; otherwise the original slice is sorted internally.
func spreadForSparity(orig, sorted []float64) (float64, error) {
	if sorted != nil {
		return spreadImpl(sorted, true)
	}
	return spreadImpl(orig, false)
}

func spreadBoundsImpl(x, sortedX []float64, misrate float64, rng *Rng) (Bounds, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := len(x)
	if n < 2 {
		return Bounds{}, NewSparityError(SubjectX)
	}
	m := n / 2

	minMisrate, err := minAchievableMisrateOneSample(m)
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}
	spreadVal, err := spreadForSparity(x, sortedX)
	if err != nil {
		return Bounds{}, err
	}
	if spreadVal <= 0 {
		return Bounds{}, NewSparityError(SubjectX)
	}

	return spreadBoundsInner(x, misrate, rng)
}

// spreadBoundsInner shuffles the original order into disjoint pairs and returns
// order-statistic bounds. The caller is responsible for validity, domain and
// sparity checks (so avgSpreadBounds can reuse it without re-checking).
func spreadBoundsInner(x []float64, misrate float64, rng *Rng) (Bounds, error) {
	n := len(x)
	m := n / 2

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

	indices := make([]int, n)
	for i := range indices {
		indices[i] = i
	}
	shuffled := RngShuffle(rng, indices)

	diffs := make([]float64, m)
	for i := 0; i < m; i++ {
		diffs[i] = math.Abs(x[shuffled[2*i]] - x[shuffled[2*i+1]])
	}
	sort.Float64s(diffs)

	return Bounds{Lower: diffs[kLeft-1], Upper: diffs[kRight-1], Unit: NumberUnit}, nil
}

// avgSpreadBoundsImpl computes weighted-average spread bounds. x/y are always in
// original order; sortedX/sortedY are sparity-only pre-sorted views.
func avgSpreadBoundsImpl(x, sortedX, y, sortedY []float64, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

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

	spreadX, err := spreadForSparity(x, sortedX)
	if err != nil {
		return Bounds{}, err
	}
	if spreadX <= 0 {
		return Bounds{}, NewSparityError(SubjectX)
	}
	spreadY, err := spreadForSparity(y, sortedY)
	if err != nil {
		return Bounds{}, err
	}
	if spreadY <= 0 {
		return Bounds{}, NewSparityError(SubjectY)
	}

	// Validity/domain/sparity already checked above; use the inner shuffle
	// directly to avoid re-running spreadImpl per sample. The shuffle operates
	// on the ORIGINAL order; sorted views are sparity-only.
	boundsX, err := spreadBoundsInner(x, alpha, rngX)
	if err != nil {
		return Bounds{}, err
	}
	boundsY, err := spreadBoundsInner(y, alpha, rngY)
	if err != nil {
		return Bounds{}, err
	}

	wx := float64(n) / float64(n+m)
	wy := float64(m) / float64(n+m)

	return Bounds{
		Lower: wx*boundsX.Lower + wy*boundsY.Lower,
		Upper: wx*boundsX.Upper + wy*boundsY.Upper,
		Unit:  NumberUnit,
	}, nil
}

func disparityBoundsImpl(x, sortedX, y, sortedY []float64, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	if err := checkValidity(x, SubjectX); err != nil {
		return Bounds{}, err
	}
	if err := checkValidity(y, SubjectY); err != nil {
		return Bounds{}, err
	}

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

	// The spread>0 sparity check is performed by avgSpreadBoundsImpl below
	// (identical predicate, same x/y subject order). ShiftBounds runs first but
	// cannot error for these inputs (alphaShift >= the two-sample minimum), so
	// it cannot mask that sparity error. ShiftBounds is order-independent given
	// sorted input; use sorted views when present.
	var sb Bounds
	if sortedX != nil && sortedY != nil {
		sb, err = ShiftBounds(sortedX, sortedY, alphaShift, true)
	} else {
		sb, err = ShiftBounds(x, y, alphaShift, false)
	}
	if err != nil {
		return Bounds{}, err
	}
	ab, err := avgSpreadBoundsImpl(x, sortedX, y, sortedY, alphaAvg, rngX, rngY)
	if err != nil {
		return Bounds{}, err
	}

	la := ab.Lower
	ua := ab.Upper
	ls := sb.Lower
	us := sb.Upper

	unit := NumberUnit

	if la > 0.0 {
		r1 := ls / la
		r2 := ls / ua
		r3 := us / la
		r4 := us / ua
		lower := math.Min(math.Min(r1, r2), math.Min(r3, r4))
		upper := math.Max(math.Max(r1, r2), math.Max(r3, r4))
		return Bounds{Lower: lower, Upper: upper, Unit: unit}, nil
	}

	if ua <= 0.0 {
		if ls == 0.0 && us == 0.0 {
			return Bounds{Lower: 0.0, Upper: 0.0, Unit: unit}, nil
		}
		if ls >= 0.0 {
			return Bounds{Lower: 0.0, Upper: math.Inf(1), Unit: unit}, nil
		}
		if us <= 0.0 {
			return Bounds{Lower: math.Inf(-1), Upper: 0.0, Unit: unit}, nil
		}
		return Bounds{Lower: math.Inf(-1), Upper: math.Inf(1), Unit: unit}, nil
	}

	// Default: ua > 0 && la <= 0
	if ls > 0.0 {
		return Bounds{Lower: ls / ua, Upper: math.Inf(1), Unit: unit}, nil
	}
	if us < 0.0 {
		return Bounds{Lower: math.Inf(-1), Upper: us / ua, Unit: unit}, nil
	}
	if ls == 0.0 && us == 0.0 {
		return Bounds{Lower: 0.0, Upper: 0.0, Unit: unit}, nil
	}
	if ls == 0.0 && us > 0.0 {
		return Bounds{Lower: 0.0, Upper: math.Inf(1), Unit: unit}, nil
	}
	if ls < 0.0 && us == 0.0 {
		return Bounds{Lower: math.Inf(-1), Upper: 0.0, Unit: unit}, nil
	}

	return Bounds{Lower: math.Inf(-1), Upper: math.Inf(1), Unit: unit}, nil
}

// sortedOne returns a sorted view of x: returns x unchanged if assumeSorted,
// otherwise returns a sorted copy (x is never mutated).
func sortedOne(x []float64, assumeSorted bool) []float64 {
	if assumeSorted {
		return x
	}
	sorted := make([]float64, len(x))
	copy(sorted, x)
	sort.Float64s(sorted)
	return sorted
}

// =============================================================================
// Sample methods — thin unit-aware adapters over the raw API
//
// Each method validates the sample is non-weighted, handles unit propagation,
// and delegates to the single raw implementation, passing the cached sorted
// view so order-independent estimators skip a re-sort.
// =============================================================================

// Center estimates the central value of the sample.
func (s *Sample) Center() (Measurement, error) {
	if err := checkNonWeighted("x", s); err != nil {
		return Measurement{}, err
	}
	result, err := Center(s.cachedSortedValues(), true)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, s.unit), nil
}

// Spread estimates data dispersion of the sample.
func (s *Sample) Spread() (Measurement, error) {
	if err := checkNonWeighted("x", s); err != nil {
		return Measurement{}, err
	}
	result, err := Spread(s.cachedSortedValues(), true)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, s.unit), nil
}

// Shift measures the typical difference between this sample and other.
func (s *Sample) Shift(other *Sample) (Measurement, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Measurement{}, err
	}
	result, err := Shift(x.cachedSortedValues(), y.cachedSortedValues(), true)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, x.unit), nil
}

// Ratio measures how many times larger this sample is compared to other.
func (s *Sample) Ratio(other *Sample) (Measurement, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Measurement{}, err
	}
	result, err := Ratio(x.cachedSortedValues(), y.cachedSortedValues(), true)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, RatioUnit), nil
}

// Disparity measures effect size between this sample and other.
func (s *Sample) Disparity(other *Sample) (Measurement, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Measurement{}, err
	}
	result, err := Disparity(x.cachedSortedValues(), y.cachedSortedValues(), true)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, DisparityUnit), nil
}

// avgSpread is the internal Sample-based weighted-average spread estimator.
func (s *Sample) avgSpread(other *Sample) (Measurement, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Measurement{}, err
	}
	result, err := avgSpread(x.cachedSortedValues(), y.cachedSortedValues(), true)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, x.unit), nil
}

// CenterBounds provides distribution-free bounds for Center.
func (s *Sample) CenterBounds(misrate float64) (Bounds, error) {
	if err := checkNonWeighted("x", s); err != nil {
		return Bounds{}, err
	}
	rb, err := CenterBounds(s.cachedSortedValues(), misrate, true)
	if err != nil {
		return Bounds{}, err
	}
	return Bounds{Lower: rb.Lower, Upper: rb.Upper, Unit: s.unit}, nil
}

// SpreadBounds provides distribution-free bounds for Spread.
func (s *Sample) SpreadBounds(misrate float64) (Bounds, error) {
	return s.spreadBoundsWithRng(misrate, NewRng())
}

// SpreadBoundsWithSeed provides distribution-free bounds for Spread with deterministic randomization.
func (s *Sample) SpreadBoundsWithSeed(misrate float64, seed string) (Bounds, error) {
	return s.spreadBoundsWithRng(misrate, NewRngFromString(seed))
}

func (s *Sample) spreadBoundsWithRng(misrate float64, rng *Rng) (Bounds, error) {
	if err := checkNonWeighted("x", s); err != nil {
		return Bounds{}, err
	}
	// Shuffle runs on the original order; the cached sorted view is sparity-only.
	rb, err := spreadBoundsImpl(s.values, s.cachedSortedValues(), misrate, rng)
	if err != nil {
		return Bounds{}, err
	}
	return Bounds{Lower: rb.Lower, Upper: rb.Upper, Unit: s.unit}, nil
}

// ShiftBounds provides bounds on Shift relative to other.
func (s *Sample) ShiftBounds(other *Sample, misrate float64) (Bounds, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Bounds{}, err
	}
	rb, err := ShiftBounds(x.cachedSortedValues(), y.cachedSortedValues(), misrate, true)
	if err != nil {
		return Bounds{}, err
	}
	return Bounds{Lower: rb.Lower, Upper: rb.Upper, Unit: x.unit}, nil
}

// RatioBounds provides bounds on Ratio relative to other.
func (s *Sample) RatioBounds(other *Sample, misrate float64) (Bounds, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Bounds{}, err
	}
	rb, err := RatioBounds(x.cachedSortedValues(), y.cachedSortedValues(), misrate, true)
	if err != nil {
		return Bounds{}, err
	}
	return Bounds{Lower: rb.Lower, Upper: rb.Upper, Unit: RatioUnit}, nil
}

// DisparityBounds provides bounds on Disparity relative to other.
func (s *Sample) DisparityBounds(other *Sample, misrate float64) (Bounds, error) {
	return s.disparityBoundsWithRngs(other, misrate, NewRng(), NewRng())
}

// DisparityBoundsWithSeed provides bounds on Disparity with deterministic randomization.
func (s *Sample) DisparityBoundsWithSeed(other *Sample, misrate float64, seed string) (Bounds, error) {
	return s.disparityBoundsWithRngs(other, misrate, NewRngFromString(seed), NewRngFromString(seed))
}

func (s *Sample) disparityBoundsWithRngs(other *Sample, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Bounds{}, err
	}
	// Shuffle runs on the original order; cached sorted views are sparity/shift-only.
	rb, err := disparityBoundsImpl(
		x.values, x.cachedSortedValues(),
		y.values, y.cachedSortedValues(),
		misrate, rngX, rngY,
	)
	if err != nil {
		return Bounds{}, err
	}
	return Bounds{Lower: rb.Lower, Upper: rb.Upper, Unit: DisparityUnit}, nil
}

// avgSpreadBoundsWithRngs is the internal Sample-based weighted-average spread bounds.
func (s *Sample) avgSpreadBoundsWithRngs(other *Sample, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	x, y, err := s.preparePair(other)
	if err != nil {
		return Bounds{}, err
	}
	rb, err := avgSpreadBoundsImpl(
		x.values, x.cachedSortedValues(),
		y.values, y.cachedSortedValues(),
		misrate, rngX, rngY,
	)
	if err != nil {
		return Bounds{}, err
	}
	return Bounds{Lower: rb.Lower, Upper: rb.Upper, Unit: x.unit}, nil
}

// preparePair validates both samples are non-weighted, checks unit
// compatibility, and converts both to the finer unit. It does NOT relabel the
// samples: the error "subject" (x = arg1, y = arg2) is supplied positionally by
// the raw two-sample impls the callers delegate to. The original samples' cached
// sorted views are reused (no clone), so a warm cache survives repeated calls.
func (s *Sample) preparePair(other *Sample) (*Sample, *Sample, error) {
	if err := checkNonWeighted("x", s); err != nil {
		return nil, nil, err
	}
	if err := checkNonWeighted("y", other); err != nil {
		return nil, nil, err
	}
	if err := checkCompatibleUnits(s, other); err != nil {
		return nil, nil, err
	}
	return convertToFiner(s, other)
}
