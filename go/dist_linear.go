package pragmastat

import "math"

// LinearDistModel is a piecewise-linear distribution reconstructed from a sample
// (algorithm dist-linear-v1).
//
// The model interpolates the order statistics at the anchors (i-1)/(n-1), which is
// the Hyndman-Fan type 7 rule. Support is [min(x), max(x)].
//
// Only Quantile carries a unit -- the sample's own, attached through Measured. Cumulative
// answers in probabilities and Density is a terminal value: both the numeric and the measured
// path return plain numbers.
//
// A built model is immutable, so concurrent Quantile/Cumulative/Density reads are safe.
type LinearDistModel struct {
	x    []float64
	unit *MeasurementUnit
}

// LinearDistModelAlgorithm is the normative algorithm identifier. Changing the anchors
// or the interpolation rule requires a new identifier, not a silent edit.
const LinearDistModelAlgorithm = "dist-linear-v1"

// NewLinearDistModel builds the model from a native numeric vector.
//
// values must be non-empty, finite, of size at least two, with a finite
// representable range and pairwise distinct elements.
//
// If assumeSorted is true, values is assumed already sorted ascending and the
// internal sort is skipped. Passing an unsorted slice under this flag is a contract
// violation with an unspecified but terminating result.
//
// The model built this way is dimensionless. A caller holding a measured *Sample
// uses (*Sample).LinearDistModel instead.
func NewLinearDistModel(values []float64, assumeSorted bool) (*LinearDistModel, error) {
	if err := checkValidity(values, SubjectX); err != nil {
		return nil, err
	}
	return newLinearDistModelTrusted(values, assumeSorted)
}

// LinearDistModel builds the model from the sample and remembers its unit, so the measured
// queries reached through Measured carry a dimension.
//
// The sample already stores its values in ascending order, so the model does not
// sort again. Weighted samples are refused: this reconstruction has no weighted
// form, and silently ignoring the weights would answer a different question than
// the caller asked.
func (s *Sample) LinearDistModel() (*LinearDistModel, error) {
	values, err := distSampleValues(s)
	if err != nil {
		return nil, err
	}
	model, err := newLinearDistModelTrusted(values, true)
	if err != nil {
		return nil, err
	}
	model.unit = s.unit
	return model, nil
}

// newLinearDistModelTrusted builds the model from a vector whose validity the caller
// has already checked.
func newLinearDistModelTrusted(values []float64, assumeSorted bool) (*LinearDistModel, error) {
	x, err := prepareDistSample(values, assumeSorted)
	if err != nil {
		return nil, err
	}
	return &LinearDistModel{x: x, unit: NumberUnit}, nil
}

// Size returns the number of observations backing the model.
func (d *LinearDistModel) Size() int {
	return len(d.x)
}

// SupportMin returns the lower end of the support.
func (d *LinearDistModel) SupportMin() float64 {
	return d.x[0]
}

// SupportMax returns the upper end of the support.
func (d *LinearDistModel) SupportMax() float64 {
	return d.x[len(d.x)-1]
}

// Unit returns the unit of the observations the model was built from. A model built
// from a native numeric vector is dimensionless.
func (d *LinearDistModel) Unit() *MeasurementUnit {
	return d.unit
}

// Measured returns the measured view of this model: the same three queries, each
// carrying the dimension its result actually has.
func (d *LinearDistModel) Measured() MeasuredDist {
	return MeasuredDist{model: d}
}

// Quantile returns the value at each requested cumulance, in the order the
// cumulances were given.
func (d *LinearDistModel) Quantile(cumulances []float64) ([]float64, error) {
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
func (d *LinearDistModel) Cumulative(values []float64) ([]float64, error) {
	if err := checkQueryValidity(values); err != nil {
		return nil, err
	}
	return d.cumulativeTrusted(values), nil
}

// Density returns the density at each requested value, in the order the values
// were given.
func (d *LinearDistModel) Density(values []float64) ([]float64, error) {
	if err := checkQueryValidity(values); err != nil {
		return nil, err
	}
	return d.densityTrusted(values), nil
}

// Sample generates a single sample from the distribution.
func (d *LinearDistModel) Sample(rng *Rng) float64 {
	return d.QuantileAt(rng.UniformFloat64())
}

// Samples generates multiple samples from the distribution.
func (d *LinearDistModel) Samples(rng *Rng, count int) []float64 {
	result := make([]float64, count)
	for i := range count {
		result[i] = d.Sample(rng)
	}
	return result
}

func (d *LinearDistModel) quantileTrusted(cumulances []float64) []float64 {
	result := make([]float64, len(cumulances))
	for i, c := range cumulances {
		result[i] = d.QuantileAt(c)
	}
	return result
}

func (d *LinearDistModel) cumulativeTrusted(values []float64) []float64 {
	result := make([]float64, len(values))
	for i, v := range values {
		result[i] = d.CumulativeAt(v)
	}
	return result
}

func (d *LinearDistModel) densityTrusted(values []float64) []float64 {
	result := make([]float64, len(values))
	for i, v := range values {
		result[i] = d.DensityAt(v)
	}
	return result
}

// QuantileAt returns the value below which a fraction p of the reconstruction lies.
//
// NaN first, for the same reason CumulativeAt and DensityAt check it: every branch below
// compares false against it, and the fallback would otherwise depend on how the platform
// converts a NaN cumulance to an index.
func (d *LinearDistModel) QuantileAt(p float64) float64 {
	if math.IsNaN(p) {
		return math.NaN()
	}

	n := len(d.x)
	var value float64
	if p <= 0.0 {
		value = d.x[0]
	} else if p >= 1.0 {
		value = d.x[n-1]
	} else {
		// The conversion is not redundant: without it the compiler is free to fuse the product
		// into the subtraction below, and on every FMA-capable target it does. A fused g comes
		// off the unrounded product while j comes off the rounded one, which breaks the
		// bit-for-bit contract against the other six languages.
		r := float64(float64(n-1) * p)
		j := int(math.Floor(r))
		g := r - float64(j)
		value = lerp(d.x[j], d.x[j+1], g)
	}

	// Every branch hands back a value the sample supplied -- the clamped ones return an order
	// statistic verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among the
	// observations would otherwise reach the caller unnormalized.
	return normalizeZero(value)
}

// CumulativeAt returns the probability of not exceeding x.
func (d *LinearDistModel) CumulativeAt(x float64) float64 {
	if math.IsNaN(x) {
		return math.NaN()
	}

	n := len(d.x)
	if x <= d.x[0] {
		return 0.0
	}
	if x >= d.x[n-1] {
		return 1.0
	}

	j := upperBound(d.x, x) - 1
	t := unitRatio(x-d.x[j], d.x[j+1]-d.x[j])
	return (float64(j) + t) / float64(n-1)
}

// DensityAt returns the density at x.
func (d *LinearDistModel) DensityAt(x float64) float64 {
	if math.IsNaN(x) {
		return math.NaN()
	}

	n := len(d.x)
	if x < d.x[0] || x > d.x[n-1] {
		return 0.0
	}

	j := n - 2
	if x != d.x[n-1] {
		j = upperBound(d.x, x) - 1
	}
	return 1.0 / float64(n-1) / (d.x[j+1] - d.x[j])
}

// GetLandscape returns the staircase this model is: cell boundaries, cumulative values and
// heights.
//
// Point evaluation answers what the density is at a value a caller picked; this answers what the
// density is, on the boundaries the model itself uses. Every height is the expression DensityAt
// evaluates inside the same cell, so the two agree bit for bit rather than approximately.
func (d *LinearDistModel) GetLandscape() *LinearLandscape {
	n := len(d.x)
	boundaries := make([]float64, n)
	copy(boundaries, d.x)
	cumulative := make([]float64, n)
	// The general CumulativeAt path for an exact order statistic d.x[i] always resolves j = i and
	// t = 0.0, so it reduces to exactly float64(i) / float64(n-1) -- computing that directly skips
	// the O(log n) binary search per boundary without changing a single bit of the result.
	for i := range n {
		cumulative[i] = float64(i) / float64(n-1)
	}
	densities := make([]float64, n-1)
	for j := range n - 1 {
		densities[j] = 1.0 / float64(n-1) / (d.x[j+1] - d.x[j])
	}
	return &LinearLandscape{boundaries: boundaries, cumulative: cumulative, densities: densities}
}
