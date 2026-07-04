package pragmastat

import (
	"fmt"
	"math"
	"sort"
	"sync"
)

// Sample wraps values with optional weights and a measurement unit.
//
// A Sample carries no error "subject": the subject (x/y/misrate) is purely a
// property of argument POSITION and is supplied by the validating function (the
// raw slice-based impls), never stored here.
type Sample struct {
	values       []float64
	weights      []float64 // nil for unweighted
	unit         *MeasurementUnit
	isWeighted   bool
	totalWeight  float64
	weightedSize float64

	sortCache *sortedCache
}

// sortedCache holds the lazily-computed sorted values/weights. It lives behind a
// pointer so the Sample struct stays copy-safe (the sync.Once is not embedded in
// Sample) and so a unit-converted view can keep its own cache.
type sortedCache struct {
	once    sync.Once
	values  []float64
	weights []float64
}

// Values returns a defensive copy of the sample values.
func (s *Sample) Values() []float64 {
	result := make([]float64, len(s.values))
	copy(result, s.values)
	return result
}

// Weights returns a defensive copy of the sample weights, or nil if unweighted.
func (s *Sample) Weights() []float64 {
	if s.weights == nil {
		return nil
	}
	result := make([]float64, len(s.weights))
	copy(result, s.weights)
	return result
}

// Unit returns the measurement unit of the sample.
func (s *Sample) Unit() *MeasurementUnit { return s.unit }

// IsWeighted returns true if the sample has weights.
func (s *Sample) IsWeighted() bool { return s.isWeighted }

// TotalWeight returns the total weight of the sample.
func (s *Sample) TotalWeight() float64 { return s.totalWeight }

// WeightedSize returns the effective sample size accounting for weights.
func (s *Sample) WeightedSize() float64 { return s.weightedSize }

// NewSample creates an unweighted sample from numeric values.
func NewSample[T Number](values []T) (*Sample, error) {
	return NewSampleWithUnit(values, nil)
}

// NewSampleWithUnit creates an unweighted sample with a specified unit.
func NewSampleWithUnit[T Number](values []T, unit *MeasurementUnit) (*Sample, error) {
	return newSample(values, nil, unit)
}

// NewWeightedSample creates a weighted sample.
func NewWeightedSample[T Number](values []T, weights []float64, unit *MeasurementUnit) (*Sample, error) {
	return newSample(values, weights, unit)
}

// newSample constructs a Sample, validating the values. Construction validity
// errors (empty / NaN / Inf) are always reported with subject "x": construction
// cannot know which argument position the sample will occupy.
func newSample[T Number](values []T, weights []float64, unit *MeasurementUnit) (*Sample, error) {
	if unit == nil {
		unit = NumberUnit
	}
	if len(values) == 0 {
		return nil, NewValidityError(SubjectX)
	}

	fValues := make([]float64, len(values))
	for i, v := range values {
		fv := float64(v)
		if math.IsNaN(fv) || math.IsInf(fv, 0) {
			return nil, NewValidityError(SubjectX)
		}
		fValues[i] = fv
	}

	s := &Sample{
		values:    fValues,
		unit:      unit,
		sortCache: &sortedCache{},
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
		wCopy := make([]float64, len(weights))
		copy(wCopy, weights)
		s.weights = wCopy
		s.isWeighted = true
		s.totalWeight = totalWeight
		s.weightedSize = (totalWeight * totalWeight) / totalWeightSq
	} else {
		s.isWeighted = false
		s.totalWeight = 1.0
		s.weightedSize = float64(len(fValues))
	}

	return s, nil
}

// Size returns the number of values.
func (s *Sample) Size() int {
	return len(s.values)
}

// SortedValues returns a sorted copy of the values (lazily computed).
func (s *Sample) SortedValues() []float64 {
	sorted := s.cachedSortedValues()
	result := make([]float64, len(sorted))
	copy(result, sorted)
	return result
}

// cachedSortedValues returns the internal sorted values cache without copying.
// Package-internal use only — callers must not mutate the returned slice.
func (s *Sample) cachedSortedValues() []float64 {
	s.ensureSorted()
	return s.sortCache.values
}

// ensureSorted lazily computes the sorted cache exactly once, even when several
// goroutines share this *Sample. sync.Once also establishes the happens-before
// so every caller observes the written cache.
func (s *Sample) ensureSorted() {
	s.sortCache.once.Do(s.computeSorted)
}

func (s *Sample) computeSorted() {
	n := len(s.values)
	sortedValues := make([]float64, n)
	copy(sortedValues, s.values)

	if s.isWeighted && s.weights != nil {
		sortedWeights := make([]float64, n)
		copy(sortedWeights, s.weights)
		// Co-sort values and weights
		indices := make([]int, n)
		for i := range indices {
			indices[i] = i
		}
		sort.Slice(indices, func(i, j int) bool {
			return sortedValues[indices[i]] < sortedValues[indices[j]]
		})
		tmpV := make([]float64, n)
		tmpW := make([]float64, n)
		for i, idx := range indices {
			tmpV[i] = sortedValues[idx]
			tmpW[i] = sortedWeights[idx]
		}
		copy(sortedValues, tmpV)
		copy(sortedWeights, tmpW)
		s.sortCache.weights = sortedWeights
	} else {
		sort.Float64s(sortedValues)
	}
	s.sortCache.values = sortedValues
}

// ConvertTo converts the sample to a different (compatible) unit.
func (s *Sample) ConvertTo(target *MeasurementUnit) (*Sample, error) {
	if !s.unit.IsCompatible(target) {
		return nil, &UnitMismatchError{Unit1: s.unit, Unit2: target}
	}
	if s.unit == target {
		return s, nil
	}
	factor := ConversionFactor(s.unit, target)
	converted := make([]float64, len(s.values))
	for i, v := range s.values {
		converted[i] = v * factor
	}
	result := &Sample{
		values:       converted,
		unit:         target,
		isWeighted:   s.isWeighted,
		totalWeight:  s.totalWeight,
		weightedSize: s.weightedSize,
		sortCache:    &sortedCache{},
	}
	if s.isWeighted {
		result.weights = make([]float64, len(s.weights))
		copy(result.weights, s.weights)
	}
	return result, nil
}

// checkNonWeighted returns an error if the sample is weighted.
func checkNonWeighted(name string, s *Sample) error {
	if s == nil {
		return fmt.Errorf("%s cannot be nil", name)
	}
	if s.isWeighted {
		return fmt.Errorf("weighted samples are not supported for %s", name)
	}
	return nil
}

// checkCompatibleUnits returns an error if two samples have incompatible units.
func checkCompatibleUnits(a, b *Sample) error {
	if !a.unit.IsCompatible(b.unit) {
		return &UnitMismatchError{Unit1: a.unit, Unit2: b.unit}
	}
	return nil
}

// convertToFiner converts both samples to the finer unit.
func convertToFiner(a, b *Sample) (*Sample, *Sample, error) {
	if a.unit == b.unit {
		return a, b, nil
	}
	target := Finer(a.unit, b.unit)
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
