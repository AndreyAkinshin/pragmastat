package pragmastat

import (
	"fmt"
)

// Metric represents the type of statistical metric being compared.
type Metric int

const (
	// MetricCenter represents the central value of a sample.
	MetricCenter Metric = iota
	// MetricSpread represents data dispersion (variability or scatter).
	MetricSpread
	// MetricShift represents the typical difference between two samples.
	MetricShift
	// MetricRatio represents the multiplicative factor between two samples.
	MetricRatio
	// MetricDisparity represents the normalized difference (effect size) between two samples.
	MetricDisparity
)

// String returns the string representation of the metric.
func (m Metric) String() string {
	switch m {
	case MetricCenter:
		return "center"
	case MetricSpread:
		return "spread"
	case MetricShift:
		return "shift"
	case MetricRatio:
		return "ratio"
	case MetricDisparity:
		return "disparity"
	default:
		return "unknown"
	}
}

// ComparisonVerdict represents the result of comparing an estimate against a threshold.
type ComparisonVerdict int

const (
	// VerdictInconclusive means not enough evidence to conclude (interval contains threshold).
	VerdictInconclusive ComparisonVerdict = iota
	// VerdictLess means the estimate is statistically less than the threshold.
	VerdictLess
	// VerdictGreater means the estimate is statistically greater than the threshold.
	VerdictGreater
)

// String returns the string representation of the verdict.
func (v ComparisonVerdict) String() string {
	switch v {
	case VerdictLess:
		return "less"
	case VerdictGreater:
		return "greater"
	case VerdictInconclusive:
		return "inconclusive"
	default:
		return "unknown"
	}
}

// Threshold represents a threshold value with a metric type and misrate for comparison.
type Threshold struct {
	Metric  Metric
	Value   Measurement
	Misrate float64
}

// NewThreshold creates a new Threshold with validation.
// The misrate must be in (0, 1] and the value must be finite.
func NewThreshold(metric Metric, value Measurement, misrate float64) (*Threshold, error) {
	if !misrateIsValid(misrate) {
		return nil, NewDomainError(SubjectMisrate)
	}
	if !isFinite(value.Value) {
		return nil, fmt.Errorf("threshold value must be finite")
	}
	return &Threshold{Metric: metric, Value: value, Misrate: misrate}, nil
}

// Projection represents the result of comparing an estimate against a threshold.
type Projection struct {
	Threshold Threshold
	Estimate  Measurement
	Bounds    Bounds
	Verdict   ComparisonVerdict
}

// metricSpec defines the specification for a metric's validation, estimation, and bounds computation.
type metricSpec struct {
	metric               Metric
	validateAndNormalize func(threshold *Threshold, x, y *Sample) (Measurement, error)
	estimate             func(x, y *Sample) (Measurement, error)
	bounds               func(x, y *Sample, misrate float64) (Bounds, error)
	seededBounds         func(x, y *Sample, misrate float64, seed string) (Bounds, error)
}

// compare1Specs defines the metric specifications for Compare1.
var compare1Specs = []metricSpec{
	{
		metric: MetricCenter,
		validateAndNormalize: func(threshold *Threshold, x, y *Sample) (Measurement, error) {
			return validateCenterOrSpread(threshold, x)
		},
		estimate: func(x, y *Sample) (Measurement, error) {
			return Center(x)
		},
		bounds: func(x, y *Sample, misrate float64) (Bounds, error) {
			return CenterBounds(x, misrate)
		},
	},
	{
		metric: MetricSpread,
		validateAndNormalize: func(threshold *Threshold, x, y *Sample) (Measurement, error) {
			return validateCenterOrSpread(threshold, x)
		},
		estimate: func(x, y *Sample) (Measurement, error) {
			return Spread(x)
		},
		bounds: func(x, y *Sample, misrate float64) (Bounds, error) {
			return SpreadBounds(x, misrate)
		},
		seededBounds: func(x, y *Sample, misrate float64, seed string) (Bounds, error) {
			return SpreadBoundsWithSeed(x, misrate, seed)
		},
	},
}

// compare2Specs defines the metric specifications for Compare2.
var compare2Specs = []metricSpec{
	{
		metric:               MetricShift,
		validateAndNormalize: validateShift,
		estimate:             Shift,
		bounds:               ShiftBounds,
	},
	{
		metric: MetricRatio,
		validateAndNormalize: func(threshold *Threshold, x, y *Sample) (Measurement, error) {
			return validateRatio(threshold)
		},
		estimate: Ratio,
		bounds:   RatioBounds,
	},
	{
		metric: MetricDisparity,
		validateAndNormalize: func(threshold *Threshold, x, y *Sample) (Measurement, error) {
			return validateDisparity(threshold)
		},
		estimate:     Disparity,
		bounds:       DisparityBounds,
		seededBounds: DisparityBoundsWithSeed,
	},
}

// validateCenterOrSpread validates and normalizes a Center or Spread threshold.
func validateCenterOrSpread(threshold *Threshold, x *Sample) (Measurement, error) {
	if !threshold.Value.Unit.IsCompatible(x.Unit) {
		return Measurement{}, &UnitMismatchError{Unit1: threshold.Value.Unit, Unit2: x.Unit}
	}
	factor := ConversionFactor(threshold.Value.Unit, x.Unit)
	return NewMeasurement(threshold.Value.Value*factor, x.Unit), nil
}

// validateShift validates and normalizes a Shift threshold.
func validateShift(threshold *Threshold, x, y *Sample) (Measurement, error) {
	if !threshold.Value.Unit.IsCompatible(x.Unit) {
		return Measurement{}, &UnitMismatchError{Unit1: threshold.Value.Unit, Unit2: x.Unit}
	}
	target := Finer(x.Unit, y.Unit)
	factor := ConversionFactor(threshold.Value.Unit, target)
	return NewMeasurement(threshold.Value.Value*factor, target), nil
}

// validateRatio validates and normalizes a Ratio threshold.
func validateRatio(threshold *Threshold) (Measurement, error) {
	unit := threshold.Value.Unit
	if unit != RatioUnit && unit != NumberUnit {
		return Measurement{}, &UnitMismatchError{Unit1: unit, Unit2: RatioUnit}
	}
	value := threshold.Value.Value
	if value <= 0 || !isFinite(value) {
		return Measurement{}, fmt.Errorf("ratio threshold value must be finite and positive")
	}
	return NewMeasurement(value, RatioUnit), nil
}

// validateDisparity validates and normalizes a Disparity threshold.
func validateDisparity(threshold *Threshold) (Measurement, error) {
	unit := threshold.Value.Unit
	if unit != DisparityUnit && unit != NumberUnit {
		return Measurement{}, &UnitMismatchError{Unit1: unit, Unit2: DisparityUnit}
	}
	value := threshold.Value.Value
	if !isFinite(value) {
		return Measurement{}, fmt.Errorf("disparity threshold value must be finite")
	}
	return NewMeasurement(value, DisparityUnit), nil
}

// misrateIsValid checks if the misrate is valid (in (0, 1]).
func misrateIsValid(misrate float64) bool {
	return isFinite(misrate) && misrate > 0 && misrate <= 1
}

// computeVerdict computes the verdict by comparing bounds against a threshold value.
func computeVerdict(bounds Bounds, thresholdValue float64) ComparisonVerdict {
	if bounds.Lower > thresholdValue {
		return VerdictGreater
	}
	if bounds.Upper < thresholdValue {
		return VerdictLess
	}
	return VerdictInconclusive
}

// getSpec returns the metric spec for the given metric from the provided specs.
func getSpec(specs []metricSpec, metric Metric) (*metricSpec, error) {
	for i := range specs {
		if specs[i].metric == metric {
			return &specs[i], nil
		}
	}
	return nil, fmt.Errorf("no spec found for metric %v", metric)
}

// Compare1 performs one-sample confirmatory analysis: compares Center/Spread against practical thresholds.
//
// Parameters:
//   - x: The sample to analyze
//   - thresholds: List of thresholds to compare against (must not be empty)
//
// Returns a slice of Projections in the same order as the input thresholds.
func Compare1(x *Sample, thresholds []*Threshold) ([]Projection, error) {
	return Compare1WithSeed(x, thresholds, "")
}

// Compare1WithSeed performs one-sample confirmatory analysis with a seed for reproducibility.
// The seed is used for randomized bounds (Spread bounds only).
func Compare1WithSeed(x *Sample, thresholds []*Threshold, seed string) ([]Projection, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return nil, err
	}

	if len(thresholds) == 0 {
		return nil, fmt.Errorf("thresholds list cannot be empty")
	}

	// Validate all thresholds are supported by Compare1
	for i, threshold := range thresholds {
		if threshold == nil {
			return nil, fmt.Errorf("thresholds[%d] cannot be nil", i)
		}
		if threshold.Metric != MetricCenter && threshold.Metric != MetricSpread {
			return nil, fmt.Errorf("metric %v is not supported by Compare1. Use Compare2 instead", threshold.Metric)
		}
	}

	// Normalize all threshold values
	normalizedValues := make([]Measurement, len(thresholds))
	for i, threshold := range thresholds {
		spec, err := getSpec(compare1Specs, threshold.Metric)
		if err != nil {
			return nil, err
		}
		normalized, err := spec.validateAndNormalize(threshold, x, nil)
		if err != nil {
			return nil, err
		}
		normalizedValues[i] = normalized
	}

	return executeCompare(compare1Specs, x, nil, thresholds, normalizedValues, seed)
}

// Compare2 performs two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.
//
// Parameters:
//   - x: The first sample
//   - y: The second sample
//   - thresholds: List of thresholds to compare against (must not be empty)
//
// Returns a slice of Projections in the same order as the input thresholds.
func Compare2(x, y *Sample, thresholds []*Threshold) ([]Projection, error) {
	return Compare2WithSeed(x, y, thresholds, "")
}

// Compare2WithSeed performs two-sample confirmatory analysis with a seed for reproducibility.
// The seed is used for randomized bounds (Disparity bounds only).
func Compare2WithSeed(x, y *Sample, thresholds []*Threshold, seed string) ([]Projection, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return nil, err
	}
	if err := checkNonWeighted("y", y); err != nil {
		return nil, err
	}
	if err := checkCompatibleUnits(x, y); err != nil {
		return nil, err
	}

	if len(thresholds) == 0 {
		return nil, fmt.Errorf("thresholds list cannot be empty")
	}

	// Validate all thresholds are supported by Compare2
	for i, threshold := range thresholds {
		if threshold == nil {
			return nil, fmt.Errorf("thresholds[%d] cannot be nil", i)
		}
		if threshold.Metric != MetricShift && threshold.Metric != MetricRatio && threshold.Metric != MetricDisparity {
			return nil, fmt.Errorf("metric %v is not supported by Compare2. Use Compare1 instead", threshold.Metric)
		}
	}

	// Convert both samples to the finer unit before any estimation
	target := Finer(x.Unit, y.Unit)
	xConv, err := x.ConvertTo(target)
	if err != nil {
		return nil, err
	}
	yConv, err := y.ConvertTo(target)
	if err != nil {
		return nil, err
	}

	// Normalize all threshold values
	normalizedValues := make([]Measurement, len(thresholds))
	for i, threshold := range thresholds {
		spec, err := getSpec(compare2Specs, threshold.Metric)
		if err != nil {
			return nil, err
		}
		normalized, err := spec.validateAndNormalize(threshold, xConv, yConv)
		if err != nil {
			return nil, err
		}
		normalizedValues[i] = normalized
	}

	return executeCompare(compare2Specs, xConv, yConv, thresholds, normalizedValues, seed)
}

// executeCompare executes the comparison for the given specs and returns projections.
func executeCompare(
	specs []metricSpec,
	x, y *Sample,
	thresholds []*Threshold,
	normalizedValues []Measurement,
	seed string,
) ([]Projection, error) {
	results := make([]Projection, len(thresholds))
	computed := make([]bool, len(thresholds))

	// Group thresholds by metric
	type entry struct {
		index           int
		threshold       *Threshold
		normalizedValue Measurement
	}
	byMetric := make(map[Metric][]entry)

	for i, threshold := range thresholds {
		byMetric[threshold.Metric] = append(byMetric[threshold.Metric], entry{
			index:           i,
			threshold:       threshold,
			normalizedValue: normalizedValues[i],
		})
	}

	// Process each metric spec
	for i := range specs {
		spec := &specs[i]
		entries, ok := byMetric[spec.metric]
		if !ok {
			continue
		}

		// Compute estimate once per metric
		estimate, err := spec.estimate(x, y)
		if err != nil {
			return nil, err
		}

		// Compute bounds and verdict for each threshold of this metric
		for _, e := range entries {
			var bounds Bounds
			if seed != "" && spec.seededBounds != nil {
				bounds, err = spec.seededBounds(x, y, e.threshold.Misrate, seed)
			} else {
				bounds, err = spec.bounds(x, y, e.threshold.Misrate)
			}
			if err != nil {
				return nil, err
			}

			verdict := computeVerdict(bounds, e.normalizedValue.Value)
			results[e.index] = Projection{
				Threshold: *e.threshold,
				Estimate:  estimate,
				Bounds:    bounds,
				Verdict:   verdict,
			}
			computed[e.index] = true
		}
	}

	// Verify all projections were computed
	for i, ok := range computed {
		if !ok {
			return nil, fmt.Errorf("projection %d was not computed", i)
		}
	}

	return results, nil
}
