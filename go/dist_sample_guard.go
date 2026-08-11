package pragmastat

import (
	"math"
	"sort"
)

// prepareDistSample runs the two assumptions that follow validity in the canonical order:
// domain(x) (at least two observations and a finite representable range) and distinctness(x) (no
// repeated value; -0.0 and +0.0 count as repeated).
//
// The caller's slice is never retained: the model always owns a private copy.
func prepareDistSample(values []float64, assumeSorted bool) ([]float64, error) {
	x := make([]float64, len(values))
	copy(x, values)
	if !assumeSorted {
		sort.Float64s(x)
	}

	n := len(x)
	if n < 2 {
		return nil, NewDomainError(SubjectX)
	}

	span := x[n-1] - x[0]
	if math.IsNaN(span) || math.IsInf(span, 0) {
		return nil, NewDomainError(SubjectX)
	}

	for i := 1; i < n; i++ {
		if x[i-1] == x[i] {
			return nil, NewDistinctnessError(SubjectX)
		}
	}

	return x, nil
}

// distSampleValues runs the sample-level guards shared by the two measured model constructors
// and returns the ascending values to build from.
//
// A Sample already stores its values in ascending order and enforced validity at construction,
// so the model neither sorts nor re-validates. Weighted samples are refused: neither
// reconstruction has a weighted form, and silently ignoring the weights would answer a different
// question than the caller asked.
func distSampleValues(x *Sample) ([]float64, error) {
	if err := checkNonWeighted("x", x); err != nil {
		return nil, err
	}
	return x.cachedSortedValues(), nil
}

// toMeasurements attaches a unit to every element of a query result.
func toMeasurements(values []float64, unit *MeasurementUnit) []Measurement {
	result := make([]Measurement, len(values))
	for i, v := range values {
		result[i] = NewMeasurement(v, unit)
	}
	return result
}

// toRawValues converts a measured query vector into the model's own unit.
//
// A query in a compatible but different unit is converted rather than rejected, matching the
// two-sample estimators, which reconcile their inputs instead of demanding an exact match. An
// incompatible unit is a caller error and is reported as one.
func toRawValues(values []Measurement, unit *MeasurementUnit) ([]float64, error) {
	raw := make([]float64, len(values))
	for i, m := range values {
		source := m.Unit
		if source == nil {
			source = NumberUnit
		}
		if !source.IsCompatible(unit) {
			return nil, &UnitMismatchError{Unit1: source, Unit2: unit}
		}
		raw[i] = m.Value * ConversionFactor(source, unit)
	}
	return raw, nil
}
