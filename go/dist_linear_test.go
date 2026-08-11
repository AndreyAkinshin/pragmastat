package pragmastat

import (
	"math"
	"testing"
)

// Every expected value here is computed by hand against the model's own definition: piecewise
// linear interpolation of the sorted sample at anchors (i-1)/(n-1), so the checks double as a
// derivation, not just a pin.
var linearTestX = []float64{1.0, 2.0, 4.0, 8.0}

func newTestLinearModel(t *testing.T) *LinearDistModel {
	t.Helper()
	model, err := NewLinearDistModel(linearTestX, false)
	if err != nil {
		t.Fatalf("NewLinearDistModel: %v", err)
	}
	return model
}

func TestLinearDistModel_QuantileAtLandsOnEachObservationAtItsAnchor(t *testing.T) {
	model := newTestLinearModel(t)
	cases := []struct {
		c        float64
		expected float64
	}{
		{0.0, 1.0},
		{1.0 / 3.0, 2.0},
		{2.0 / 3.0, 4.0},
		{1.0, 8.0},
	}
	for _, c := range cases {
		if got := model.QuantileAt(c.c); got != c.expected {
			t.Errorf("QuantileAt(%v) = %v, want %v", c.c, got, c.expected)
		}
	}
}

func TestLinearDistModel_QuantileAtInterpolatesBetweenTwoObservations(t *testing.T) {
	model := newTestLinearModel(t)
	if got := model.QuantileAt(0.5); got != 3.0 {
		t.Errorf("QuantileAt(0.5) = %v, want 3.0", got)
	}
}

func TestLinearDistModel_QuantileAtClampsOutsideTheUnitInterval(t *testing.T) {
	model := newTestLinearModel(t)
	if got := model.QuantileAt(-0.1); got != 1.0 {
		t.Errorf("QuantileAt(-0.1) = %v, want 1.0", got)
	}
	if got := model.QuantileAt(1.1); got != 8.0 {
		t.Errorf("QuantileAt(1.1) = %v, want 8.0", got)
	}
}

func TestLinearDistModel_CumulativeAtInvertsQuantileAt(t *testing.T) {
	model := newTestLinearModel(t)
	cases := []struct {
		v        float64
		expected float64
	}{
		{3.0, 0.5},
		{2.0, 1.0 / 3.0},
		{4.0, 2.0 / 3.0},
	}
	for _, c := range cases {
		if got := model.CumulativeAt(c.v); got != c.expected {
			t.Errorf("CumulativeAt(%v) = %v, want %v", c.v, got, c.expected)
		}
	}
}

func TestLinearDistModel_CumulativeAtIsZeroBelowAndOneAboveTheSupport(t *testing.T) {
	model := newTestLinearModel(t)
	if got := model.CumulativeAt(1.0); got != 0.0 {
		t.Errorf("CumulativeAt(1.0) = %v, want 0.0", got)
	}
	if got := model.CumulativeAt(-5.0); got != 0.0 {
		t.Errorf("CumulativeAt(-5.0) = %v, want 0.0", got)
	}
	if got := model.CumulativeAt(8.0); got != 1.0 {
		t.Errorf("CumulativeAt(8.0) = %v, want 1.0", got)
	}
	if got := model.CumulativeAt(20.0); got != 1.0 {
		t.Errorf("CumulativeAt(20.0) = %v, want 1.0", got)
	}
}

func TestLinearDistModel_DensityAtIsMassOverWidthInsideEachCell(t *testing.T) {
	model := newTestLinearModel(t)
	cases := []struct {
		v        float64
		expected float64
	}{
		{1.5, 1.0 / 3.0},
		{3.0, 1.0 / 6.0},
		{6.0, 1.0 / 12.0},
		{8.0, 1.0 / 12.0}, // The right edge belongs to the last cell.
	}
	for _, c := range cases {
		if got := model.DensityAt(c.v); got != c.expected {
			t.Errorf("DensityAt(%v) = %v, want %v", c.v, got, c.expected)
		}
	}
}

func TestLinearDistModel_DensityAtIsZeroOutsideTheSupport(t *testing.T) {
	model := newTestLinearModel(t)
	if got := model.DensityAt(0.5); got != 0.0 {
		t.Errorf("DensityAt(0.5) = %v, want 0.0", got)
	}
	if got := model.DensityAt(9.0); got != 0.0 {
		t.Errorf("DensityAt(9.0) = %v, want 0.0", got)
	}
}

func TestLinearDistModel_ScalarQueriesPropagateNaN(t *testing.T) {
	model := newTestLinearModel(t)
	if !math.IsNaN(model.QuantileAt(math.NaN())) {
		t.Error("QuantileAt(NaN) is not NaN")
	}
	if !math.IsNaN(model.CumulativeAt(math.NaN())) {
		t.Error("CumulativeAt(NaN) is not NaN")
	}
	if !math.IsNaN(model.DensityAt(math.NaN())) {
		t.Error("DensityAt(NaN) is not NaN")
	}
}

func TestLinearDistModel_GetLandscapeAgreesWithTheScalarQueriesBitForBit(t *testing.T) {
	model := newTestLinearModel(t)
	landscape := model.GetLandscape()

	if landscape.CellCount() != 3 {
		t.Fatalf("CellCount() = %d, want 3", landscape.CellCount())
	}
	boundaries := landscape.Boundaries()
	cumulative := landscape.Cumulative()
	for i, b := range boundaries {
		if got := model.CumulativeAt(b); got != cumulative[i] {
			t.Errorf("CumulativeAt(%v) = %v, landscape has %v", b, got, cumulative[i])
		}
	}
	densities := landscape.Densities()
	cellCount := landscape.CellCount()
	for j := range cellCount {
		mid := (boundaries[j] + boundaries[j+1]) / 2.0
		if got := model.DensityAt(mid); got != densities[j] {
			t.Errorf("DensityAt(%v) = %v, landscape has %v", mid, got, densities[j])
		}
	}
}

func TestLinearDistModel_GetLandscapeHandsOutACopy(t *testing.T) {
	model := newTestLinearModel(t)
	landscape := model.GetLandscape()
	boundaries := landscape.Boundaries()
	boundaries[0] = 999.0
	if model.SupportMin() != linearTestX[0] {
		t.Error("mutating the returned landscape boundaries mutated the model")
	}
}

func TestLinearDistModel_SampleIsQuantileAtOfARawDrawnCumulance(t *testing.T) {
	model := newTestLinearModel(t)
	drawing := NewRngFromString("identity-seed")
	inverting := NewRngFromString("identity-seed")
	for i := range 100 {
		drawn := model.Sample(drawing)
		u := inverting.UniformFloat64()
		if math.Float64bits(drawn) != math.Float64bits(model.QuantileAt(u)) {
			t.Fatalf("draw %d: Sample gave %v, QuantileAt(%v) gave %v", i, drawn, u, model.QuantileAt(u))
		}
	}
}

func TestLinearDistModel_ConstructorRejectsFewerThanTwoObservations(t *testing.T) {
	_, err := NewLinearDistModel([]float64{1.0}, false)
	assertErr, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("error = %v, want *AssumptionError", err)
	}
	if assertErr.Violation.ID != AssumptionDomain {
		t.Errorf("Violation.ID = %v, want %v", assertErr.Violation.ID, AssumptionDomain)
	}
}

func TestLinearDistModel_ConstructorRejectsARepeatedValue(t *testing.T) {
	_, err := NewLinearDistModel([]float64{1.0, 1.0, 2.0}, false)
	assertErr, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("error = %v, want *AssumptionError", err)
	}
	if assertErr.Violation.ID != AssumptionDistinctness {
		t.Errorf("Violation.ID = %v, want %v", assertErr.Violation.ID, AssumptionDistinctness)
	}
}

func TestLinearDistModel_QuantileRejectsACumulanceOutsideTheUnitInterval(t *testing.T) {
	model := newTestLinearModel(t)
	_, err := model.Quantile([]float64{1.5})
	assertErr, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("error = %v, want *AssumptionError", err)
	}
	if assertErr.Violation.ID != AssumptionCumulance {
		t.Errorf("Violation.ID = %v, want %v", assertErr.Violation.ID, AssumptionCumulance)
	}
}

func TestLinearDistModel_AnEmptyQueryVectorIsNotAViolation(t *testing.T) {
	model := newTestLinearModel(t)
	if _, err := model.Quantile([]float64{}); err != nil {
		t.Errorf("Quantile([]): %v", err)
	}
	if _, err := model.Cumulative([]float64{}); err != nil {
		t.Errorf("Cumulative([]): %v", err)
	}
	if _, err := model.Density([]float64{}); err != nil {
		t.Errorf("Density([]): %v", err)
	}
}

func TestLinearDistModel_SampleConstructorTakesTheSamplesUnit(t *testing.T) {
	meter := NewMeasurementUnit("meter", "Length", "m", "Meter", 1)
	sample, err := NewSampleWithUnit(linearTestX, meter)
	if err != nil {
		t.Fatalf("NewSampleWithUnit: %v", err)
	}
	model, err := sample.LinearDistModel()
	if err != nil {
		t.Fatalf("LinearDistModel: %v", err)
	}
	if model.Unit() != meter {
		t.Errorf("Unit() = %v, want %v", model.Unit(), meter)
	}
}

func TestLinearDistModel_MeasuredQuantileReturnsAMeasurementInTheModelsUnit(t *testing.T) {
	meter := NewMeasurementUnit("meter", "Length", "m", "Meter", 1)
	sample, err := NewSampleWithUnit(linearTestX, meter)
	if err != nil {
		t.Fatalf("NewSampleWithUnit: %v", err)
	}
	model, err := sample.LinearDistModel()
	if err != nil {
		t.Fatalf("LinearDistModel: %v", err)
	}
	result, err := model.Measured().Quantile([]float64{0.5})
	if err != nil {
		t.Fatalf("Quantile: %v", err)
	}
	if result[0].Value != 3.0 || result[0].Unit != meter {
		t.Errorf("result = %+v, want {3.0 %v}", result[0], meter)
	}
}

func TestLinearDistModel_MeasuredDensityReturnsAPlainNumber(t *testing.T) {
	meter := NewMeasurementUnit("meter", "Length", "m", "Meter", 1)
	sample, err := NewSampleWithUnit(linearTestX, meter)
	if err != nil {
		t.Fatalf("NewSampleWithUnit: %v", err)
	}
	model, err := sample.LinearDistModel()
	if err != nil {
		t.Fatalf("LinearDistModel: %v", err)
	}
	result, err := model.Measured().Density([]Measurement{NewMeasurement(3.0, meter)})
	if err != nil {
		t.Fatalf("Density: %v", err)
	}
	if result[0] != 1.0/6.0 {
		t.Errorf("Density = %v, want %v", result[0], 1.0/6.0)
	}
}
