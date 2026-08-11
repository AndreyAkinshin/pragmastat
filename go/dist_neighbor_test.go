package pragmastat

import (
	"math"
	"testing"
)

var neighborTestX = []float64{0.0, 1.0, 2.0, 4.0, 8.0}

func newTestNeighborModel(t *testing.T) *NeighborDistModel {
	t.Helper()
	model, err := NewNeighborDistModel(neighborTestX, false)
	if err != nil {
		t.Fatalf("NewNeighborDistModel: %v", err)
	}
	return model
}

func TestNeighborDistModel_SupportMatchesTheMinAndMaxObservation(t *testing.T) {
	model := newTestNeighborModel(t)
	if model.SupportMin() != 0.0 {
		t.Errorf("SupportMin() = %v, want 0.0", model.SupportMin())
	}
	if model.SupportMax() != 8.0 {
		t.Errorf("SupportMax() = %v, want 8.0", model.SupportMax())
	}
}

func TestNeighborDistModel_CellCountIsAtMostOneFewerThanTheObservationCount(t *testing.T) {
	model := newTestNeighborModel(t)
	if model.CellCount() != len(neighborTestX) {
		t.Errorf("CellCount() = %d, want %d (no collapses on this sample)", model.CellCount(), len(neighborTestX))
	}
}

func TestNeighborDistModel_CumulativeAtTheSupportIsZeroAndOneExactly(t *testing.T) {
	model := newTestNeighborModel(t)
	if got := model.CumulativeAt(model.SupportMin()); got != 0.0 {
		t.Errorf("CumulativeAt(SupportMin) = %v, want 0.0", got)
	}
	if got := model.CumulativeAt(model.SupportMax()); got != 1.0 {
		t.Errorf("CumulativeAt(SupportMax) = %v, want 1.0", got)
	}
}

func TestNeighborDistModel_CellMassesSumToExactlyOne(t *testing.T) {
	// cumulative[0] = 0.0 and cumulative[CellCount] = 1.0 are fixed by construction, so the
	// masses telescope to exactly 1.0 regardless of how the intermediate values rounded.
	model := newTestNeighborModel(t)
	landscape := model.GetLandscape()
	sum := 0.0
	for _, mass := range landscape.Masses() {
		sum += mass
	}
	if sum != 1.0 {
		t.Errorf("sum of masses = %v, want 1.0", sum)
	}
}

func TestNeighborDistModel_CumulativeAtIsNonDecreasingAcrossTheBoundaries(t *testing.T) {
	model := newTestNeighborModel(t)
	cumulative := model.GetLandscape().Cumulative()
	for i := 1; i < len(cumulative); i++ {
		if !(cumulative[i] > cumulative[i-1]) {
			t.Errorf("cumulative[%d]=%v is not greater than cumulative[%d]=%v", i, cumulative[i], i-1, cumulative[i-1])
		}
	}
}

func TestNeighborDistModel_QuantileAtInvertsCumulativeAtAtEachBoundary(t *testing.T) {
	model := newTestNeighborModel(t)
	landscape := model.GetLandscape()
	boundaries := landscape.Boundaries()
	cumulative := landscape.Cumulative()
	for i, b := range boundaries {
		if got := model.QuantileAt(cumulative[i]); got != b {
			t.Errorf("QuantileAt(%v) = %v, want %v", cumulative[i], got, b)
		}
	}
}

func TestNeighborDistModel_QuantileAtClampsOutsideTheUnitInterval(t *testing.T) {
	model := newTestNeighborModel(t)
	if got := model.QuantileAt(-0.1); got != model.SupportMin() {
		t.Errorf("QuantileAt(-0.1) = %v, want %v", got, model.SupportMin())
	}
	if got := model.QuantileAt(1.1); got != model.SupportMax() {
		t.Errorf("QuantileAt(1.1) = %v, want %v", got, model.SupportMax())
	}
}

func TestNeighborDistModel_DensityAtIsZeroOutsideTheSupport(t *testing.T) {
	model := newTestNeighborModel(t)
	if got := model.DensityAt(model.SupportMin() - 1.0); got != 0.0 {
		t.Errorf("DensityAt(below) = %v, want 0.0", got)
	}
	if got := model.DensityAt(model.SupportMax() + 1.0); got != 0.0 {
		t.Errorf("DensityAt(above) = %v, want 0.0", got)
	}
}

func TestNeighborDistModel_ScalarQueriesPropagateNaN(t *testing.T) {
	model := newTestNeighborModel(t)
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

func TestNeighborDistModel_GetLandscapeAgreesWithTheScalarQueriesBitForBit(t *testing.T) {
	model := newTestNeighborModel(t)
	landscape := model.GetLandscape()
	boundaries := landscape.Boundaries()
	cumulative := landscape.Cumulative()
	densities := landscape.Densities()

	for i, b := range boundaries {
		if got := model.CumulativeAt(b); got != cumulative[i] {
			t.Errorf("CumulativeAt(%v) = %v, landscape has %v", b, got, cumulative[i])
		}
	}
	cellCount := landscape.CellCount()
	for j := range cellCount {
		mid := (boundaries[j] + boundaries[j+1]) / 2.0
		if got := model.DensityAt(mid); got != densities[j] {
			t.Errorf("DensityAt(%v) = %v, landscape has %v", mid, got, densities[j])
		}
	}
}

func TestNeighborDistModel_GetLandscapeExposesScaleIndexAndRadius(t *testing.T) {
	model := newTestNeighborModel(t)
	landscape := model.GetLandscape()
	// n = 5: CeilSqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = 3 / 2 = 1.
	if landscape.ScaleIndex() != 3 {
		t.Errorf("ScaleIndex() = %d, want 3", landscape.ScaleIndex())
	}
	if landscape.Radius() != 1 {
		t.Errorf("Radius() = %d, want 1", landscape.Radius())
	}
}

func TestNeighborDistModel_GetLandscapeSourcesMapEachCellToItsObservation(t *testing.T) {
	model := newTestNeighborModel(t)
	sources := model.GetLandscape().Sources()
	want := []int{0, 1, 2, 3, 4}
	if len(sources) != len(want) {
		t.Fatalf("Sources() = %v, want %v", sources, want)
	}
	for i := range want {
		if sources[i] != want[i] {
			t.Errorf("Sources()[%d] = %d, want %d", i, sources[i], want[i])
		}
	}
}

func TestNeighborDistModel_AZeroWidthCellFromACollapsedMidpointIsDropped(t *testing.T) {
	// The center observation sits exactly one ULP from each neighbor on either side. Both
	// adjacent midpoints round to 1.0 itself under round-to-nearest-even (each is exactly
	// halfway between two representable doubles, and 1.0 has the even mantissa), so the center
	// cell's two boundaries collapse onto the same point and the cell is dropped.
	center := 1.0
	below := math.Float64frombits(math.Float64bits(center) - 1)
	above := math.Float64frombits(math.Float64bits(center) + 1)
	values := []float64{0.0, below, center, above, 10.0}

	model, err := NewNeighborDistModel(values, false)
	if err != nil {
		t.Fatalf("NewNeighborDistModel: %v", err)
	}
	landscape := model.GetLandscape()

	if landscape.CellCount() != len(values)-1 {
		t.Errorf("CellCount() = %d, want %d", landscape.CellCount(), len(values)-1)
	}
	for _, s := range landscape.Sources() {
		if s == 2 {
			t.Error("Sources() still contains the collapsed observation's index")
		}
	}
}

func TestNeighborDistModel_SampleIsQuantileAtOfARawDrawnCumulance(t *testing.T) {
	model := newTestNeighborModel(t)
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

func TestNeighborDistModel_ConstructorRejectsFewerThanTwoObservations(t *testing.T) {
	_, err := NewNeighborDistModel([]float64{1.0}, false)
	assertErr, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("error = %v, want *AssumptionError", err)
	}
	if assertErr.Violation.ID != AssumptionDomain {
		t.Errorf("Violation.ID = %v, want %v", assertErr.Violation.ID, AssumptionDomain)
	}
}

func TestNeighborDistModel_ConstructorRejectsARepeatedValue(t *testing.T) {
	_, err := NewNeighborDistModel([]float64{1.0, 1.0, 2.0}, false)
	assertErr, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("error = %v, want *AssumptionError", err)
	}
	if assertErr.Violation.ID != AssumptionDistinctness {
		t.Errorf("Violation.ID = %v, want %v", assertErr.Violation.ID, AssumptionDistinctness)
	}
}

func TestNeighborDistModel_QuantileRejectsACumulanceOutsideTheUnitInterval(t *testing.T) {
	model := newTestNeighborModel(t)
	_, err := model.Quantile([]float64{-0.1})
	assertErr, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("error = %v, want *AssumptionError", err)
	}
	if assertErr.Violation.ID != AssumptionCumulance {
		t.Errorf("Violation.ID = %v, want %v", assertErr.Violation.ID, AssumptionCumulance)
	}
}

func TestNeighborDistModel_AnEmptyQueryVectorIsNotAViolation(t *testing.T) {
	model := newTestNeighborModel(t)
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

func TestNeighborDistModel_ABrokenAssumeSortedPromiseIsANumericalConformanceFailure(t *testing.T) {
	// prepareDistSample's distinctness check only looks at ADJACENT elements of the (unsorted,
	// since assumeSorted is a promise it trusts) slice, so a non-adjacent repeat -- x[2] and
	// x[6] both 5.0 here -- slips through it. That repeat then makes one window's span exactly
	// zero while its cell keeps a nonzero width, so the cell's unnormalized mass divides by
	// zero and the total the whole reconstruction divides by is not a usable number.
	defer func() {
		if recover() == nil {
			t.Error("expected a panic for a broken assumeSorted promise")
		}
	}()
	scrambled := []float64{0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0}
	_, _ = NewNeighborDistModel(scrambled, true)
}

func TestNeighborDistModel_SampleConstructorTakesTheSamplesUnit(t *testing.T) {
	meter := NewMeasurementUnit("meter", "Length", "m", "Meter", 1)
	sample, err := NewSampleWithUnit(neighborTestX, meter)
	if err != nil {
		t.Fatalf("NewSampleWithUnit: %v", err)
	}
	model, err := sample.NeighborDistModel()
	if err != nil {
		t.Fatalf("NeighborDistModel: %v", err)
	}
	if model.Unit() != meter {
		t.Errorf("Unit() = %v, want %v", model.Unit(), meter)
	}
}

func TestNeighborDistModel_MeasuredDensityReturnsAPlainNumber(t *testing.T) {
	meter := NewMeasurementUnit("meter", "Length", "m", "Meter", 1)
	sample, err := NewSampleWithUnit(neighborTestX, meter)
	if err != nil {
		t.Fatalf("NewSampleWithUnit: %v", err)
	}
	model, err := sample.NeighborDistModel()
	if err != nil {
		t.Fatalf("NeighborDistModel: %v", err)
	}
	result, err := model.Measured().Density([]Measurement{NewMeasurement(3.0, meter)})
	if err != nil {
		t.Fatalf("Density: %v", err)
	}
	if result[0] != model.DensityAt(3.0) {
		t.Errorf("Density = %v, want %v", result[0], model.DensityAt(3.0))
	}
}
