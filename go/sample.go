package pragmastat

import (
	"fmt"
	"math"
	"sort"
)

// Sample wraps values with optional weights and a measurement unit.
type Sample struct {
	Values       []float64
	Weights      []float64 // nil for unweighted
	Unit         *MeasurementUnit
	IsWeighted   bool
	TotalWeight  float64
	WeightedSize float64
	subject      Subject

	sortedValues  []float64
	sortedWeights []float64
	sorted        bool
}

// NewSample creates an unweighted sample from numeric values.
func NewSample[T Number](values []T) (*Sample, error) {
	return NewSampleWithUnit(values, nil)
}

// NewSampleWithUnit creates an unweighted sample with a specified unit.
func NewSampleWithUnit[T Number](values []T, unit *MeasurementUnit) (*Sample, error) {
	return newSample(values, nil, unit, SubjectX)
}

// NewWeightedSample creates a weighted sample.
func NewWeightedSample[T Number](values []T, weights []float64, unit *MeasurementUnit) (*Sample, error) {
	return newSample(values, weights, unit, SubjectX)
}

func newSample[T Number](values []T, weights []float64, unit *MeasurementUnit, subject Subject) (*Sample, error) {
	if unit == nil {
		unit = NumberUnit
	}
	if len(values) == 0 {
		return nil, NewValidityError(subject)
	}

	fValues := make([]float64, len(values))
	for i, v := range values {
		fv := float64(v)
		if math.IsNaN(fv) || math.IsInf(fv, 0) {
			return nil, NewValidityError(subject)
		}
		fValues[i] = fv
	}

	s := &Sample{
		Values:  fValues,
		Unit:    unit,
		subject: subject,
	}

	if weights != nil {
		if len(weights) != len(values) {
			return nil, fmt.Errorf("weights length (%d) must match values length (%d)", len(weights), len(values))
		}
		var totalWeight, totalWeightSq float64
		var minW = math.MaxFloat64
		for _, w := range weights {
			totalWeight += w
			totalWeightSq += w * w
			if w < minW {
				minW = w
			}
		}
		if minW < 0 {
			return nil, fmt.Errorf("all weights must be non-negative")
		}
		if totalWeight < 1e-9 {
			return nil, fmt.Errorf("total weight must be positive")
		}
		s.Weights = weights
		s.IsWeighted = true
		s.TotalWeight = totalWeight
		s.WeightedSize = (totalWeight * totalWeight) / totalWeightSq
	} else {
		s.IsWeighted = false
		s.TotalWeight = 1.0
		s.WeightedSize = float64(len(fValues))
	}

	return s, nil
}

// Size returns the number of values.
func (s *Sample) Size() int {
	return len(s.Values)
}

// SortedValues returns a sorted copy of the values (lazily computed).
func (s *Sample) SortedValues() []float64 {
	if !s.sorted {
		s.computeSorted()
	}
	return s.sortedValues
}

func (s *Sample) computeSorted() {
	n := len(s.Values)
	s.sortedValues = make([]float64, n)
	copy(s.sortedValues, s.Values)

	if s.IsWeighted && s.Weights != nil {
		s.sortedWeights = make([]float64, n)
		copy(s.sortedWeights, s.Weights)
		// Co-sort values and weights
		indices := make([]int, n)
		for i := range indices {
			indices[i] = i
		}
		sort.Slice(indices, func(i, j int) bool {
			return s.sortedValues[indices[i]] < s.sortedValues[indices[j]]
		})
		tmpV := make([]float64, n)
		tmpW := make([]float64, n)
		for i, idx := range indices {
			tmpV[i] = s.sortedValues[idx]
			tmpW[i] = s.sortedWeights[idx]
		}
		copy(s.sortedValues, tmpV)
		copy(s.sortedWeights, tmpW)
	} else {
		sort.Float64s(s.sortedValues)
	}
	s.sorted = true
}

// ConvertTo converts the sample to a different (compatible) unit.
func (s *Sample) ConvertTo(target *MeasurementUnit) (*Sample, error) {
	if !s.Unit.IsCompatible(target) {
		return nil, &UnitMismatchError{Unit1: s.Unit, Unit2: target}
	}
	if s.Unit == target {
		return s, nil
	}
	factor := ConversionFactor(s.Unit, target)
	converted := make([]float64, len(s.Values))
	for i, v := range s.Values {
		converted[i] = v * factor
	}
	result := &Sample{
		Values:       converted,
		Unit:         target,
		IsWeighted:   s.IsWeighted,
		TotalWeight:  s.TotalWeight,
		WeightedSize: s.WeightedSize,
		subject:      s.subject,
	}
	if s.IsWeighted {
		result.Weights = make([]float64, len(s.Weights))
		copy(result.Weights, s.Weights)
	}
	return result, nil
}

// logTransform returns a new sample with log-transformed values and NumberUnit.
func (s *Sample) logTransform() (*Sample, error) {
	logValues := make([]float64, len(s.Values))
	for i, v := range s.Values {
		if v <= 0 {
			return nil, NewPositivityError(s.subject)
		}
		logValues[i] = math.Log(v)
	}
	result := &Sample{
		Values:       logValues,
		Unit:         NumberUnit,
		IsWeighted:   s.IsWeighted,
		TotalWeight:  s.TotalWeight,
		WeightedSize: s.WeightedSize,
		subject:      s.subject,
	}
	if s.IsWeighted {
		result.Weights = make([]float64, len(s.Weights))
		copy(result.Weights, s.Weights)
	}
	return result, nil
}

// checkNonWeighted returns an error if the sample is weighted.
func checkNonWeighted(name string, s *Sample) error {
	if s == nil {
		return fmt.Errorf("%s cannot be nil", name)
	}
	if s.IsWeighted {
		return fmt.Errorf("weighted samples are not supported for %s", name)
	}
	return nil
}

// checkCompatibleUnits returns an error if two samples have incompatible units.
func checkCompatibleUnits(a, b *Sample) error {
	if !a.Unit.IsCompatible(b.Unit) {
		return &UnitMismatchError{Unit1: a.Unit, Unit2: b.Unit}
	}
	return nil
}

// convertToFiner converts both samples to the finer unit.
func convertToFiner(a, b *Sample) (*Sample, *Sample, error) {
	if a.Unit == b.Unit {
		return a, b, nil
	}
	target := Finer(a.Unit, b.Unit)
	newA, err := a.ConvertTo(target)
	if err != nil {
		return nil, nil, err
	}
	newB, err := b.ConvertTo(target)
	if err != nil {
		return nil, nil, err
	}
	return newA, newB, nil
}
