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

// Center estimates the central value of the data.
// Calculates the median of all pairwise averages (x[i] + x[j])/2.
func Center(x *Sample) (Measurement, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Measurement{}, err
	}
	result, err := fastCenter(x.Values)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, x.Unit), nil
}

// Spread estimates data dispersion (variability or scatter).
// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
//
// Assumptions:
//   - sparity(x) - sample must be non tie-dominant (Spread > 0)
func Spread(x *Sample) (Measurement, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Measurement{}, err
	}
	spreadVal, err := fastSpread(x.Values)
	if err != nil {
		return Measurement{}, err
	}
	if spreadVal <= 0 {
		return Measurement{}, NewSparityError(x.subject)
	}
	return NewMeasurement(spreadVal, x.Unit), nil
}

// Shift measures the typical difference between elements of x and y.
// Calculates the median of all pairwise differences (x[i] - y[j]).
func Shift(x, y *Sample) (Measurement, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Measurement{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Measurement{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Measurement{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Measurement{}, err
	}
	result, err := fastShift(x.Values, y.Values)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result, x.Unit), nil
}

// Ratio measures how many times larger x is compared to y.
// Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
//
// Assumptions:
//   - positivity(x) - all values in x must be strictly positive
//   - positivity(y) - all values in y must be strictly positive
func Ratio(x, y *Sample) (Measurement, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Measurement{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Measurement{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Measurement{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Measurement{}, err
	}
	for _, v := range x.Values {
		if v <= 0 {
			return Measurement{}, NewPositivityError(x.subject)
		}
	}
	for _, v := range y.Values {
		if v <= 0 {
			return Measurement{}, NewPositivityError(y.subject)
		}
	}
	result, err := fastRatioQuantiles(x.Values, y.Values, []float64{0.5}, false)
	if err != nil {
		return Measurement{}, err
	}
	return NewMeasurement(result[0], RatioUnit), nil
}

// avgSpread measures the typical variability when considering both samples together.
// Internal estimator used by Disparity.
func avgSpread(x, y *Sample) (Measurement, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Measurement{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Measurement{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Measurement{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Measurement{}, err
	}

	n := float64(x.Size())
	m := float64(y.Size())

	spreadX, err := fastSpread(x.Values)
	if err != nil {
		return Measurement{}, err
	}
	if spreadX <= 0 {
		return Measurement{}, NewSparityError(x.subject)
	}
	spreadY, err := fastSpread(y.Values)
	if err != nil {
		return Measurement{}, err
	}
	if spreadY <= 0 {
		return Measurement{}, NewSparityError(y.subject)
	}

	return NewMeasurement((n*spreadX+m*spreadY)/(n+m), x.Unit), nil
}

// Disparity measures effect size: a normalized difference between x and y.
// Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
//
// Assumptions:
//   - sparity(x) - first sample must be non tie-dominant (Spread > 0)
//   - sparity(y) - second sample must be non tie-dominant (Spread > 0)
func Disparity(x, y *Sample) (Measurement, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Measurement{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Measurement{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Measurement{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Measurement{}, err
	}

	n := float64(x.Size())
	m := float64(y.Size())

	spreadX, err := fastSpread(x.Values)
	if err != nil {
		return Measurement{}, err
	}
	if spreadX <= 0 {
		return Measurement{}, NewSparityError(x.subject)
	}
	spreadY, err := fastSpread(y.Values)
	if err != nil {
		return Measurement{}, err
	}
	if spreadY <= 0 {
		return Measurement{}, NewSparityError(y.subject)
	}

	shiftVal, err := fastShift(x.Values, y.Values)
	if err != nil {
		return Measurement{}, err
	}
	avgSpreadVal := (n*spreadX + m*spreadY) / (n + m)

	return NewMeasurement(shiftVal/avgSpreadVal, DisparityUnit), nil
}

// ShiftBounds provides bounds on the Shift estimator with specified misclassification rate.
func ShiftBounds(x, y *Sample, misrate float64) (Bounds, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Bounds{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Bounds{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Bounds{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := x.Size()
	m := y.Size()

	minMisrate, err := minAchievableMisrateTwoSample(n, m)
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	xSorted := x.SortedValues()
	ySorted := y.SortedValues()

	total := int64(n) * int64(m)

	if total == 1 {
		value := xSorted[0] - ySorted[0]
		return Bounds{Lower: value, Upper: value, Unit: x.Unit}, nil
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

	denominator := float64(total - 1)
	if denominator <= 0 {
		denominator = 1
	}

	p := []float64{float64(kLeft) / denominator, float64(kRight) / denominator}
	bounds, err := fastShiftQuantiles(xSorted, ySorted, p, true)
	if err != nil {
		return Bounds{}, err
	}

	lower := bounds[0]
	upper := bounds[1]
	if lower > upper {
		lower, upper = upper, lower
	}

	return Bounds{Lower: lower, Upper: upper, Unit: x.Unit}, nil
}

// RatioBounds provides bounds on the Ratio estimator with specified misclassification rate.
//
// Assumptions:
//   - positivity(x) - all values in x must be strictly positive
//   - positivity(y) - all values in y must be strictly positive
func RatioBounds(x, y *Sample, misrate float64) (Bounds, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Bounds{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Bounds{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Bounds{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	minMisrate, err := minAchievableMisrateTwoSample(x.Size(), y.Size())
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	logX, err := x.logTransform()
	if err != nil {
		return Bounds{}, err
	}
	logY, err := y.logTransform()
	if err != nil {
		return Bounds{}, err
	}

	logBounds, err := ShiftBounds(logX, logY, misrate)
	if err != nil {
		return Bounds{}, err
	}

	return Bounds{
		Lower: math.Exp(logBounds.Lower),
		Upper: math.Exp(logBounds.Upper),
		Unit:  RatioUnit,
	}, nil
}

// CenterBounds provides exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
// Requires weak symmetry assumption: distribution symmetric around unknown center.
func CenterBounds(x *Sample, misrate float64) (Bounds, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := x.Size()
	if n < 2 {
		return Bounds{}, NewDomainError(x.subject)
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

	lo, hi := fastCenterQuantileBounds(x.SortedValues(), kLeft, kRight)
	return Bounds{Lower: lo, Upper: hi, Unit: x.Unit}, nil
}

// SpreadBounds provides distribution-free bounds for Spread using disjoint pairs.
func SpreadBounds(x *Sample, misrate float64) (Bounds, error) {
	return spreadBoundsWithRng(x, misrate, NewRng())
}

// SpreadBoundsWithSeed provides distribution-free bounds for Spread with deterministic randomization.
func SpreadBoundsWithSeed(x *Sample, misrate float64, seed string) (Bounds, error) {
	return spreadBoundsWithRng(x, misrate, NewRngFromString(seed))
}

func spreadBoundsWithRng(x *Sample, misrate float64, rng *Rng) (Bounds, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := x.Size()
	m := n / 2

	minMisrate, err := minAchievableMisrateOneSample(m)
	if err != nil {
		return Bounds{}, err
	}
	if misrate < minMisrate {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	if n < 2 {
		return Bounds{}, NewSparityError(x.subject)
	}
	spreadVal, err := fastSpread(x.Values)
	if err != nil {
		return Bounds{}, err
	}
	if spreadVal <= 0 {
		return Bounds{}, NewSparityError(x.subject)
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

	indices := make([]int, n)
	for i := range indices {
		indices[i] = i
	}
	shuffled := RngShuffle(rng, indices)

	diffs := make([]float64, m)
	for i := 0; i < m; i++ {
		diffs[i] = math.Abs(x.Values[shuffled[2*i]] - x.Values[shuffled[2*i+1]])
	}
	sort.Float64s(diffs)

	return Bounds{Lower: diffs[kLeft-1], Upper: diffs[kRight-1], Unit: x.Unit}, nil
}

func avgSpreadBoundsWithRngs(x, y *Sample, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Bounds{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Bounds{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Bounds{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := x.Size()
	m := y.Size()
	if n < 2 {
		return Bounds{}, NewDomainError(x.subject)
	}
	if m < 2 {
		return Bounds{}, NewDomainError(y.subject)
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

	spreadX, err := fastSpread(x.Values)
	if err != nil {
		return Bounds{}, err
	}
	if spreadX <= 0 {
		return Bounds{}, NewSparityError(x.subject)
	}
	spreadY, err := fastSpread(y.Values)
	if err != nil {
		return Bounds{}, err
	}
	if spreadY <= 0 {
		return Bounds{}, NewSparityError(y.subject)
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
		Unit:  x.Unit,
	}, nil
}

// DisparityBounds provides distribution-free bounds for the Disparity estimator.
func DisparityBounds(x, y *Sample, misrate float64) (Bounds, error) {
	return disparityBoundsWithRngs(x, y, misrate, NewRng(), NewRng())
}

// DisparityBoundsWithSeed provides distribution-free bounds for Disparity with deterministic randomization.
func DisparityBoundsWithSeed(x, y *Sample, misrate float64, seed string) (Bounds, error) {
	return disparityBoundsWithRngs(x, y, misrate, NewRngFromString(seed), NewRngFromString(seed))
}

func disparityBoundsWithRngs(x, y *Sample, misrate float64, rngX, rngY *Rng) (Bounds, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return Bounds{}, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return Bounds{}, err
	}
	x.subject = SubjectX
	y.subject = SubjectY
	if err := checkCompatibleUnits(x, y); err != nil {
		return Bounds{}, err
	}
	x, y, err := convertToFiner(x, y)
	if err != nil {
		return Bounds{}, err
	}

	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return Bounds{}, NewDomainError(SubjectMisrate)
	}

	n := x.Size()
	m := y.Size()
	if n < 2 {
		return Bounds{}, NewDomainError(x.subject)
	}
	if m < 2 {
		return Bounds{}, NewDomainError(y.subject)
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

	spreadXVal, err := fastSpread(x.Values)
	if err != nil {
		return Bounds{}, err
	}
	if spreadXVal <= 0 {
		return Bounds{}, NewSparityError(x.subject)
	}
	spreadYVal, err := fastSpread(y.Values)
	if err != nil {
		return Bounds{}, err
	}
	if spreadYVal <= 0 {
		return Bounds{}, NewSparityError(y.subject)
	}

	sb, err := ShiftBounds(x, y, alphaShift)
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

	unit := DisparityUnit

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

// Fluent methods on *Sample

// Center estimates the central value of the sample.
func (s *Sample) Center() (Measurement, error) { return Center(s) }

// Spread estimates data dispersion of the sample.
func (s *Sample) Spread() (Measurement, error) { return Spread(s) }

// Shift measures the typical difference between this sample and other.
func (s *Sample) Shift(other *Sample) (Measurement, error) { return Shift(s, other) }

// Ratio measures how many times larger this sample is compared to other.
func (s *Sample) Ratio(other *Sample) (Measurement, error) { return Ratio(s, other) }

// Disparity measures effect size between this sample and other.
func (s *Sample) Disparity(other *Sample) (Measurement, error) { return Disparity(s, other) }

// CenterBounds provides distribution-free bounds for Center.
func (s *Sample) CenterBounds(misrate float64) (Bounds, error) { return CenterBounds(s, misrate) }

// SpreadBounds provides distribution-free bounds for Spread.
func (s *Sample) SpreadBounds(misrate float64) (Bounds, error) { return SpreadBounds(s, misrate) }

// ShiftBounds provides bounds on Shift relative to other.
func (s *Sample) ShiftBounds(other *Sample, misrate float64) (Bounds, error) {
	return ShiftBounds(s, other, misrate)
}

// RatioBounds provides bounds on Ratio relative to other.
func (s *Sample) RatioBounds(other *Sample, misrate float64) (Bounds, error) {
	return RatioBounds(s, other, misrate)
}

// DisparityBounds provides bounds on Disparity relative to other.
func (s *Sample) DisparityBounds(other *Sample, misrate float64) (Bounds, error) {
	return DisparityBounds(s, other, misrate)
}
