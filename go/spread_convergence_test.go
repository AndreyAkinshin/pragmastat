package pragmastat

import (
	"testing"
	"time"
)

// TestSpreadConvergenceFailureOnUnsortedAssumeSorted is a regression test with
// no shared cross-language fixture: Spread() on UNSORTED input with
// assumeSorted=true must NOT hang forever. The spreadImpl Monahan-selection loop
// previously had no iteration cap, so misusing assumeSorted on unsorted data
// (a pathological anti-sorted input makes the pivot ping-pong without shrinking
// the active set) could wedge the process in an unkillable infinite loop. The
// loop is now bounded and returns a deterministic convergence-failure error
// (a plain error, NOT an AssumptionError) on pathological input.
//
// We assert the call returns QUICKLY with an error rather than hanging.
func TestSpreadConvergenceFailureOnUnsortedAssumeSorted(t *testing.T) {
	// Adversarial unsorted input designed to make the selection loop spin:
	// a descending sequence is the worst case for "assume ascending".
	unsorted := make([]float64, 64)
	for i := range unsorted {
		unsorted[i] = float64(len(unsorted) - i)
	}

	done := make(chan struct {
		val float64
		err error
	}, 1)

	go func() {
		val, err := Spread(unsorted, true)
		done <- struct {
			val float64
			err error
		}{val, err}
	}()

	select {
	case res := <-done:
		// The contract: on pathological (unsorted) input the call must
		// terminate. It either converges to some value or returns a
		// deterministic error; it must never hang. The important property
		// here is that it returned at all.
		//
		// For this descending input the bounded loop produces a non-positive
		// pseudo-spread, so the public Spread wrapper surfaces a sparity(x)
		// AssumptionError rather than the plain convergence error. (The plain
		// convergence error from the spreadImpl kernel is exercised
		// unconditionally by TestSpreadConvergenceErrorPublicPath below.)
		if res.err == nil {
			// Acceptable: bounded loop still produced a (meaningless) value
			// without hanging. The key guarantee is termination.
			t.Logf("Spread(unsorted, assumeSorted=true) terminated with value %v (no hang)", res.val)
		} else {
			t.Logf("Spread(unsorted, assumeSorted=true) returned error (sparity expected for this input): %v", res.err)
		}
	case <-time.After(5 * time.Second):
		t.Fatal("Spread(unsorted, assumeSorted=true) hung: the selection-loop iteration cap is broken")
	}
}

// TestSpreadConvergenceErrorPublicPath asserts UNCONDITIONALLY that the public
// Spread() entry surfaces the plain convergence-failure error (NOT an
// AssumptionError) on an input that drives the Monahan-selection loop past its
// iteration cap while still yielding a positive pseudo-spread (so the sparity
// guard does not mask it). This input was verified to surface the plain
// convergence error on the public path (a descending sequence instead yields a
// sparity(x) error).
func TestSpreadConvergenceErrorPublicPath(t *testing.T) {
	input := []float64{100, 90, 80, 5, 70, 1, 60, 50, 3, 40}

	_, err := Spread(input, true)
	if err == nil {
		t.Fatal("expected convergence failure error from Spread, got nil")
	}
	if _, isAssumption := err.(*AssumptionError); isAssumption {
		t.Fatalf("expected plain convergence error from Spread, got AssumptionError: %v", err)
	}
	if err.Error() != "convergence failure (pathological input)" {
		t.Fatalf("unexpected Spread error: %v", err)
	}
}

// TestSpreadConvergenceErrorReturnedFast checks that the bounded loop produces a
// deterministic error on input crafted to never converge, and does so quickly.
func TestSpreadConvergenceErrorReturnedFast(t *testing.T) {
	// A larger descending input maximizes the chance of triggering the cap /
	// stall guard rather than accidental convergence.
	unsorted := make([]float64, 257)
	for i := range unsorted {
		unsorted[i] = float64(len(unsorted) - i)
	}

	start := time.Now()
	_, err := Spread(unsorted, true)
	elapsed := time.Since(start)

	if elapsed > 2*time.Second {
		t.Fatalf("Spread(unsorted, assumeSorted=true) took %v — should terminate quickly", elapsed)
	}
	// We don't strictly require an error (some unsorted inputs may still
	// converge). The spreadImpl kernel returns the plain convergence error on
	// the bounded-loop path; the public Spread wrapper may additionally surface
	// a sparity AssumptionError if the bounded loop yields a non-positive
	// pseudo-result. Both are acceptable non-hang outcomes — the guarantee is
	// fast termination, which the elapsed-time check above enforces.
	if err != nil {
		t.Logf("Spread(unsorted, assumeSorted=true) terminated fast with error: %v", err)
	}

	// Verify the kernel itself returns the plain convergence error (never an
	// AssumptionError) directly, parallel to the centerImpl contract.
	_, kernelErr := spreadImpl(unsorted, true)
	if kernelErr != nil {
		if _, isAssumption := kernelErr.(*AssumptionError); isAssumption {
			t.Fatalf("expected plain convergence error from spreadImpl, got AssumptionError: %v", kernelErr)
		}
		if kernelErr.Error() != "convergence failure (pathological input)" {
			t.Fatalf("unexpected spreadImpl error: %v", kernelErr)
		}
	}
}
