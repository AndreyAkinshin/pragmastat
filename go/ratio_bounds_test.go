package pragmastat

import "testing"

// Error-priority contract for RatioBounds: the domain(misrate) check runs
// before the positivity check, and a valid misrate lets positivity(x) surface.
// Mirrors the Rust ratio_bounds tests.

func TestRatioBoundsDomainBeforePositivity(t *testing.T) {
	// misrate=-0.1 is invalid (domain), x=-1 is non-positive (positivity);
	// domain(misrate) must take priority over positivity(x).
	x := []float64{-1}
	y := []float64{1}

	_, err := RatioBounds(x, y, -0.1, false)
	ae, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("expected *AssumptionError, got %T: %v", err, err)
	}
	if ae.Violation.ID != Domain || ae.Violation.Subject != SubjectMisrate {
		t.Errorf("expected domain(misrate), got %s", ae.Violation)
	}
}

func TestRatioBoundsPositivityWhenMisrateValid(t *testing.T) {
	// Valid misrate but non-positive x -> positivity(x).
	x := []float64{-1, -2, -3}
	y := []float64{1, 2, 3}

	_, err := RatioBounds(x, y, 0.5, false)
	ae, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("expected *AssumptionError, got %T: %v", err, err)
	}
	if ae.Violation.ID != Positivity || ae.Violation.Subject != SubjectX {
		t.Errorf("expected positivity(x), got %s", ae.Violation)
	}
}
