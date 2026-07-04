package pragmastat

import "testing"

// TestErrorSubjectIsPositional verifies that the error "subject" is determined
// by ARGUMENT POSITION, not stored on the Sample. The same constant sample
// reports subject "x" when validated as a one-sample input, but surfaces as
// subject "y" when it is the second argument of a two-sample estimator.
func TestErrorSubjectIsPositional(t *testing.T) {
	constant, err := NewSample([]float64{5, 5, 5}) // constant -> spread 0
	if err != nil {
		t.Fatalf("NewSample: %v", err)
	}
	x, err := NewSample([]float64{1.0, 2.0, 3.0})
	if err != nil {
		t.Fatalf("NewSample: %v", err)
	}

	// One-sample path: the constant sample reports sparity on subject x.
	_, err = constant.Spread()
	if ae, ok := err.(*AssumptionError); !ok || ae.Violation.ID != Sparity || ae.Violation.Subject != SubjectX {
		t.Fatalf("Spread(constant): expected sparity(x), got %v", err)
	}

	// Two-sample path with the constant sample as the SECOND argument: the
	// sparity error is reported positionally on subject y.
	_, err = x.Disparity(constant)
	ae, ok := err.(*AssumptionError)
	if !ok {
		t.Fatalf("Disparity: expected AssumptionError, got %v", err)
	}
	if ae.Violation.ID != Sparity || ae.Violation.Subject != SubjectY {
		t.Errorf("Disparity(x, constant): expected sparity(y), got %s", ae.Violation)
	}

	// The caller's sample is not mutated: it still reports subject x on its own.
	_, err = constant.Spread()
	if ae, ok := err.(*AssumptionError); !ok || ae.Violation.Subject != SubjectX {
		t.Errorf("constant sample was mutated: Spread reports %v, want sparity(x)", err)
	}
}
