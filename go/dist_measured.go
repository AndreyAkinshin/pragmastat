package pragmastat

// The measured facade over the distribution estimation models.
//
// The metrology path, alongside the numeric one, exactly as every estimator offers both. Only
// Quantile carries a unit -- the sample's own. Cumulative answers in probabilities, and Density
// is a terminal value read, plotted, or compared against another density of the same model, never
// converted or combined, so both come back as plain numbers.
//
// Go has no overloading, so the measured queries cannot share the numeric names on the model.
// They live behind their own view instead of under prefixed names, so that a caller who entered
// the metrology world stays in it across all three operations and reads the same three verbs in
// both worlds.
//
// Probabilities are plain float64 here, the way misrate is plain float64 on the Sample
// estimators: this package has no typed probability wrapper.

// measuredModel is the numeric surface a measured view sits on. Both reconstructions satisfy it,
// so the view is written once instead of once per model.
type measuredModel interface {
	Quantile(cumulances []float64) ([]float64, error)
	Cumulative(values []float64) ([]float64, error)
	Density(values []float64) ([]float64, error)
	Unit() *MeasurementUnit
}

// MeasuredDist is the measured view of a built reconstruction, obtained from
// LinearDistModel.Measured or NeighborDistModel.Measured.
type MeasuredDist struct {
	model measuredModel
}

// Quantile returns the value at each requested cumulance, in the unit of the sample the model
// was built from.
func (m MeasuredDist) Quantile(cumulances []float64) ([]Measurement, error) {
	values, err := m.model.Quantile(cumulances)
	if err != nil {
		return nil, err
	}
	return toMeasurements(values, m.model.Unit()), nil
}

// Cumulative returns the probability of not exceeding each requested measurement. Probabilities
// are dimensionless, so they come back as plain numbers.
//
// A query in a compatible but different unit is converted; an incompatible one is rejected with
// *UnitMismatchError.
func (m MeasuredDist) Cumulative(values []Measurement) ([]float64, error) {
	raw, err := toRawValues(values, m.model.Unit())
	if err != nil {
		return nil, err
	}
	return m.model.Cumulative(raw)
}

// Density returns the density at each requested measurement, as a plain number.
//
// A query in a compatible but different unit is converted; an incompatible one is rejected with
// *UnitMismatchError.
func (m MeasuredDist) Density(values []Measurement) ([]float64, error) {
	raw, err := toRawValues(values, m.model.Unit())
	if err != nil {
		return nil, err
	}
	return m.model.Density(raw)
}
