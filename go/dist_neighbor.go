package pragmastat

import (
	"math"
	"slices"
)

// NeighborDistModel is a local step-density distribution reconstructed from a sample
// (algorithm dist-neighbor-v1).
//
// Each observation contributes a cell whose height reflects how tightly its nearest
// neighbors are packed, so the reconstruction follows local structure instead of
// imposing a single global shape. The window is indexed by k0 = ceil(sqrt(n)) with
// radius r = floor(k0/2); windows near the ends are truncated and the numerator is
// always the actual neighbor count.
//
// The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry estimator:
// support is bounded by the observed range and the cell masses are explicitly
// normalized, so the density integrates to one.
//
// Only Quantile carries a unit -- the sample's own, attached through Measured. Cumulative
// answers in probabilities and Density is a terminal value: both the numeric and the measured
// path return plain numbers.
//
// A built model is immutable, so concurrent Quantile/Cumulative/Density reads are safe.
type NeighborDistModel struct {
	boundaries []float64
	cumulative []float64
	masses     []float64
	densities  []float64
	sources    []int
	scaleIndex int
	radius     int
	unit       *MeasurementUnit
}

// NeighborDistModelAlgorithm is the normative algorithm identifier. Changing the window
// rule, the boundaries or the normalization requires a new identifier.
const NeighborDistModelAlgorithm = "dist-neighbor-v1"

// NewNeighborDistModel builds the model from a native numeric vector.
//
// values must be non-empty, finite, of size at least two, with a finite
// representable range and pairwise distinct elements.
//
// If assumeSorted is true, values is assumed already sorted ascending and the
// internal sort is skipped. Passing an unsorted slice under this flag is a contract
// violation with an unspecified but terminating result.
//
// The model built this way is dimensionless. A caller holding a measured *Sample
// uses (*Sample).NeighborDistModel instead.
func NewNeighborDistModel(values []float64, assumeSorted bool) (*NeighborDistModel, error) {
	if err := checkValidity(values, SubjectX); err != nil {
		return nil, err
	}
	return newNeighborDistModelTrusted(values, assumeSorted)
}

// NeighborDistModel builds the model from the sample and remembers its unit, so the measured
// queries reached through Measured carry a dimension.
//
// The sample already stores its values in ascending order, so the model does not
// sort again. Weighted samples are refused: this reconstruction has no weighted
// form, and silently ignoring the weights would answer a different question than
// the caller asked.
func (s *Sample) NeighborDistModel() (*NeighborDistModel, error) {
	values, err := distSampleValues(s)
	if err != nil {
		return nil, err
	}
	model, err := newNeighborDistModelTrusted(values, true)
	if err != nil {
		return nil, err
	}
	model.unit = s.unit
	return model, nil
}

// newNeighborDistModelTrusted builds the model from a vector whose validity the caller
// has already checked.
func newNeighborDistModelTrusted(values []float64, assumeSorted bool) (*NeighborDistModel, error) {
	x, err := prepareDistSample(values, assumeSorted)
	if err != nil {
		return nil, err
	}
	return buildNeighborDistModel(x), nil
}

func buildNeighborDistModel(x []float64) *NeighborDistModel {
	n := len(x)
	scaleIndex := ceilSqrt(n)
	radius := scaleIndex / 2

	counts := make([]int, n)
	spans := make([]float64, n)
	for i := range n {
		left := max(0, i-radius)
		right := min(n-1, i+radius)
		counts[i] = right - left + 1
		spans[i] = x[right] - x[left]
	}

	e := make([]float64, n+1)
	e[0] = x[0]
	for i := 1; i < n; i++ {
		e[i] = midpoint(x[i-1], x[i])
	}
	e[n] = x[n-1]

	// Rounding can collapse the midpoint of two adjacent binary64 observations onto a
	// neighboring boundary. Such a cell carries no mass and is dropped; the surviving
	// cells keep the index of the observation they came from.
	boundaries := make([]float64, 0, n+1)
	boundaries = append(boundaries, e[0])
	surviving := make([]int, 0, n)
	for i := range n {
		if e[i+1] == e[i] {
			continue
		}
		surviving = append(surviving, i)
		boundaries = append(boundaries, e[i+1])
	}

	s := len(surviving)
	a := make([]float64, s)
	for j, i := range surviving {
		width := e[i+1] - e[i]
		a[j] = float64(counts[i]) / float64(n) * (width / spans[i])
	}

	total := 0.0
	for _, mass := range a {
		total += mass
	}

	// A broken assumeSorted promise can make a cell wider than the window it normalizes
	// against, so its unnormalized mass overflows, or it can collapse every cell and leave a
	// zero total. Either way the divisor is unusable and NaN would reach the query kernels.
	// The sample itself satisfies domain(x) and distinctness(x), so this is not an assumption
	// violation: it is a numerical-conformance failure, reported through the same channel
	// unitRatio uses. For a genuinely ascending sample the total is finite and strictly positive.
	// It is not bounded by 1: the total is a normalizer, not a probability. It sits above 1 and
	// decays toward it as n grows.
	if math.IsNaN(total) || math.IsInf(total, 0) || total <= 0.0 {
		panic("numerical conformance failure: the NeighborDistModel normalizer is not usable; " +
			"this indicates an assumeSorted promise that the input does not satisfy")
	}

	// Reserve one representable step for every cell that has not been placed yet, so a
	// mass too small to register at this point in the prefix still advances the
	// cumulative value instead of flattening it into a plateau.
	ceiling := make([]float64, s+1)
	ceiling[s] = 1.0
	for j := s - 1; j >= 0; j-- {
		ceiling[j] = nextDown(ceiling[j+1])
	}

	cumulative := make([]float64, s+1)
	cumulative[0] = 0.0
	for j := range s - 1 {
		raw := cumulative[j] + a[j]/total
		cumulative[j+1] = min(ceiling[j+1], max(nextUp(cumulative[j]), raw))
	}
	cumulative[s] = 1.0

	masses := make([]float64, s)
	densities := make([]float64, s)
	for j := range s {
		masses[j] = cumulative[j+1] - cumulative[j]
		densities[j] = masses[j] / (boundaries[j+1] - boundaries[j])
	}

	return &NeighborDistModel{
		boundaries: boundaries,
		cumulative: cumulative,
		masses:     masses,
		densities:  densities,
		sources:    surviving,
		scaleIndex: scaleIndex,
		radius:     radius,
		unit:       NumberUnit,
	}
}

// CellCount returns the number of surviving cells in the reconstruction.
func (d *NeighborDistModel) CellCount() int {
	return len(d.masses)
}

// SupportMin returns the lower end of the support.
func (d *NeighborDistModel) SupportMin() float64 {
	return d.boundaries[0]
}

// SupportMax returns the upper end of the support.
func (d *NeighborDistModel) SupportMax() float64 {
	return d.boundaries[len(d.boundaries)-1]
}

// Unit returns the unit of the observations the model was built from. A model built
// from a native numeric vector is dimensionless.
func (d *NeighborDistModel) Unit() *MeasurementUnit {
	return d.unit
}

// Measured returns the measured view of this model: the same three queries, each
// carrying the dimension its result actually has.
func (d *NeighborDistModel) Measured() MeasuredDist {
	return MeasuredDist{model: d}
}

// GetLandscape returns the staircase this model is: cell boundaries, cumulative values, masses
// and heights.
//
// Point evaluation answers what the density is at a value a caller picked; this answers what the
// density is, on the boundaries the model itself uses. Every height is the expression DensityAt
// evaluates inside the same cell, so the two agree bit for bit rather than approximately.
func (d *NeighborDistModel) GetLandscape() *NeighborLandscape {
	return &NeighborLandscape{
		scaleIndex: d.scaleIndex,
		radius:     d.radius,
		boundaries: slices.Clone(d.boundaries),
		cumulative: slices.Clone(d.cumulative),
		masses:     slices.Clone(d.masses),
		densities:  slices.Clone(d.densities),
		sources:    slices.Clone(d.sources),
	}
}

// Quantile returns the value at each requested cumulance, in the order the
// cumulances were given.
func (d *NeighborDistModel) Quantile(cumulances []float64) ([]float64, error) {
	if err := checkQueryValidity(cumulances); err != nil {
		return nil, err
	}
	if err := checkCumulance(cumulances); err != nil {
		return nil, err
	}
	return d.quantileTrusted(cumulances), nil
}

// Cumulative returns the probability of not exceeding each requested value, in the
// order the values were given.
func (d *NeighborDistModel) Cumulative(values []float64) ([]float64, error) {
	if err := checkQueryValidity(values); err != nil {
		return nil, err
	}
	return d.cumulativeTrusted(values), nil
}

// Density returns the density at each requested value, in the order the values
// were given.
func (d *NeighborDistModel) Density(values []float64) ([]float64, error) {
	if err := checkQueryValidity(values); err != nil {
		return nil, err
	}
	return d.densityTrusted(values), nil
}

// Sample generates a single sample from the distribution.
func (d *NeighborDistModel) Sample(rng *Rng) float64 {
	return d.QuantileAt(rng.UniformFloat64())
}

// Samples generates multiple samples from the distribution.
func (d *NeighborDistModel) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := range count {
		result[i] = d.Sample(rng)
	}
	return result
}

func (d *NeighborDistModel) quantileTrusted(cumulances []float64) []float64 {
	result := make([]float64, len(cumulances))
	for i, c := range cumulances {
		result[i] = d.QuantileAt(c)
	}
	return result
}

func (d *NeighborDistModel) cumulativeTrusted(values []float64) []float64 {
	result := make([]float64, len(values))
	for i, v := range values {
		result[i] = d.CumulativeAt(v)
	}
	return result
}

func (d *NeighborDistModel) densityTrusted(values []float64) []float64 {
	result := make([]float64, len(values))
	for i, v := range values {
		result[i] = d.DensityAt(v)
	}
	return result
}

// QuantileAt returns the value below which a fraction p of the reconstruction lies.
//
// NaN first, for the same reason LinearDistModel.QuantileAt checks it: every branch below
// compares false against it, and the fallback would otherwise depend on how the platform
// converts a NaN cumulance to an index.
func (d *NeighborDistModel) QuantileAt(p float64) float64 {
	if math.IsNaN(p) {
		return math.NaN()
	}

	s := len(d.masses)
	var value float64
	if p <= 0.0 {
		value = d.boundaries[0]
	} else if p >= 1.0 {
		value = d.boundaries[s]
	} else {
		j := lowerBound(d.cumulative, 1, s+1, p) - 1
		t := unitRatio(p-d.cumulative[j], d.masses[j])
		value = lerp(d.boundaries[j], d.boundaries[j+1], t)
	}

	// Every branch hands back a value built from the sample's own boundaries -- the clamped
	// ones return one verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among
	// the observations would otherwise reach the caller unnormalized.
	return normalizeZero(value)
}

// CumulativeAt returns the probability of not exceeding x.
func (d *NeighborDistModel) CumulativeAt(x float64) float64 {
	if math.IsNaN(x) {
		return math.NaN()
	}

	s := len(d.masses)
	if x <= d.boundaries[0] {
		return 0.0
	}
	if x >= d.boundaries[s] {
		return 1.0
	}

	j := upperBound(d.boundaries, x) - 1
	t := unitRatio(x-d.boundaries[j], d.boundaries[j+1]-d.boundaries[j])
	return lerp(d.cumulative[j], d.cumulative[j+1], t)
}

// DensityAt returns the density at x.
func (d *NeighborDistModel) DensityAt(x float64) float64 {
	if math.IsNaN(x) {
		return math.NaN()
	}

	s := len(d.masses)
	if x < d.boundaries[0] || x > d.boundaries[s] {
		return 0.0
	}

	j := s - 1
	if x != d.boundaries[s] {
		j = upperBound(d.boundaries, x) - 1
	}
	return d.masses[j] / (d.boundaries[j+1] - d.boundaries[j])
}
