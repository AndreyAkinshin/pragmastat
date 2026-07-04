package pragmastat

import (
	"math"
	"testing"
)

// Property tests that have no shared cross-language fixture:
// bounds unit re-attachment, the raw-path misrate domain branch, and the n==2
// center midpoint order-symmetry guard.

// testSecond is a custom (non-Number) unit used to prove that center/spread/
// shift bounds re-attach the *sample's* unit rather than the raw NumberUnit.
var testSecond = &MeasurementUnit{
	ID:           "second",
	Family:       "Time",
	Abbreviation: "s",
	FullName:     "Second",
	BaseUnits:    1,
}

// --- unit re-attachment ---------------------------------------------------
//
// The Sample-path ratio/disparity bounds re-attach Ratio/Disparity units, while
// the raw (native-slice) bounds API returns a unitless Number unit. center/
// spread/shift Sample bounds propagate the sample's unit (x / finer(x,y)).

func TestBoundsUnitReattachment(t *testing.T) {
	// 0.2 clears the disparity-bounds minimum (minShift+minAvg) for n=m=10
	// while remaining valid for center/spread/shift/ratio bounds.
	const misrate = 0.2
	x := []float64{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	y := []float64{2, 4, 6, 8, 10, 12, 14, 16, 18, 20}

	sx, err := NewSampleWithUnit(x, testSecond)
	if err != nil {
		t.Fatalf("NewSampleWithUnit x: %v", err)
	}
	sy, err := NewSampleWithUnit(y, testSecond)
	if err != nil {
		t.Fatalf("NewSampleWithUnit y: %v", err)
	}

	// Sample path: ratio/disparity bounds carry the dedicated units.
	rb, err := sx.RatioBounds(sy, misrate)
	if err != nil {
		t.Fatalf("Sample.RatioBounds: %v", err)
	}
	if rb.Unit != RatioUnit {
		t.Errorf("Sample RatioBounds unit = %v, want RatioUnit", rb.Unit)
	}

	db, err := sx.DisparityBounds(sy, misrate)
	if err != nil {
		t.Fatalf("Sample.DisparityBounds: %v", err)
	}
	if db.Unit != DisparityUnit {
		t.Errorf("Sample DisparityBounds unit = %v, want DisparityUnit", db.Unit)
	}

	// Raw path: ratio/disparity bounds are UNITLESS (NumberUnit).
	rawRb, err := RatioBounds(x, y, misrate, false)
	if err != nil {
		t.Fatalf("RatioBounds raw: %v", err)
	}
	if rawRb.Unit != NumberUnit {
		t.Errorf("raw RatioBounds unit = %v, want NumberUnit", rawRb.Unit)
	}

	rawDb, err := DisparityBounds(x, y, misrate, false)
	if err != nil {
		t.Fatalf("DisparityBounds raw: %v", err)
	}
	if rawDb.Unit != NumberUnit {
		t.Errorf("raw DisparityBounds unit = %v, want NumberUnit", rawDb.Unit)
	}

	// center/spread Sample bounds propagate the sample's own unit.
	cb, err := sx.CenterBounds(misrate)
	if err != nil {
		t.Fatalf("Sample.CenterBounds: %v", err)
	}
	if cb.Unit != testSecond {
		t.Errorf("Sample CenterBounds unit = %v, want testSecond", cb.Unit)
	}

	spb, err := sx.SpreadBounds(misrate)
	if err != nil {
		t.Fatalf("Sample.SpreadBounds: %v", err)
	}
	if spb.Unit != testSecond {
		t.Errorf("Sample SpreadBounds unit = %v, want testSecond", spb.Unit)
	}

	// shift Sample bounds propagate finer(x, y); identical units -> that unit.
	shb, err := sx.ShiftBounds(sy, misrate)
	if err != nil {
		t.Fatalf("Sample.ShiftBounds: %v", err)
	}
	if shb.Unit != testSecond {
		t.Errorf("Sample ShiftBounds unit = %v, want testSecond", shb.Unit)
	}
}

// --- raw-bounds misrate domain --------------------------------------------
//
// The raw bounds API (native slice, double misrate) must reject an out-of-[0,1]
// or NaN misrate with the domain/misrate AssumptionError. This covers a
// one-sample (CenterBounds) and a two-sample (ShiftBounds) entry. Go has no
// typed Probability wrapper: the Sample methods take the same plain float64
// misrate and delegate to this raw validation, so the raw path is the single
// domain gate.
func TestRawBoundsRejectMisrateDomain(t *testing.T) {
	x := []float64{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	y := []float64{2, 4, 6, 8, 10, 12, 14, 16, 18, 20}

	badMisrates := []float64{2.0, -0.1, math.NaN()}

	assertDomainMisrate := func(t *testing.T, err error, label string) {
		t.Helper()
		if err == nil {
			t.Fatalf("%s: expected domain/misrate error, got nil", label)
			return
		}
		ae, ok := err.(*AssumptionError)
		if !ok {
			t.Fatalf("%s: expected *AssumptionError, got %T: %v", label, err, err)
			return
		}
		if ae.Violation.ID != Domain {
			t.Errorf("%s: id = %q, want %q", label, ae.Violation.ID, Domain)
		}
		if ae.Violation.Subject != SubjectMisrate {
			t.Errorf("%s: subject = %q, want %q", label, ae.Violation.Subject, SubjectMisrate)
		}
	}

	for _, m := range badMisrates {
		m := m
		t.Run("CenterBounds", func(t *testing.T) {
			_, err := CenterBounds(x, m, false)
			assertDomainMisrate(t, err, "CenterBounds")
		})
		t.Run("ShiftBounds", func(t *testing.T) {
			_, err := ShiftBounds(x, y, m, false)
			assertDomainMisrate(t, err, "ShiftBounds")
		})
	}
}

// --- n==2 center midpoint order-symmetry -----------------------------------
//
// Forward guard: pins EXACT order-symmetry of the n==2 center midpoint
// (0.5*a + 0.5*b). assumeSorted=true is REQUIRED so the midpoint sees the raw
// element order: the normalizing sort would otherwise canonicalize the order
// and hide an order-dependent formula. An order-dependent midpoint such as
// a+(b-a)*0.5 rounds differently per order (-3.4 forward but
// -3.4000000000000004 reversed for this input); the exact bit-equality
// assertions below fail on any such replacement.
func TestCenterMidpointOrderSymmetry(t *testing.T) {
	forward, err := Center([]float64{-5.0, -1.8}, true)
	if err != nil {
		t.Fatalf("Center forward: %v", err)
	}
	reverse, err := Center([]float64{-1.8, -5.0}, true)
	if err != nil {
		t.Fatalf("Center reverse: %v", err)
	}
	if forward != reverse {
		t.Errorf("center order asymmetry: forward=%v (bits %#x) reverse=%v (bits %#x)",
			forward, math.Float64bits(forward), reverse, math.Float64bits(reverse))
	}
	if forward != -3.4 {
		t.Errorf("center([-5.0,-1.8]) = %v, want exactly -3.4", forward)
	}
}
