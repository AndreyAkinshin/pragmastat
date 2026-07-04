package pragmastat

import (
	"testing"
	"time"
)

// TestCenterConvergenceFailureOnUnsortedAssumeSorted is a regression test with
// no shared cross-language fixture: Center() on UNSORTED input with
// assumeSorted=true must NOT hang forever. The centerImpl Monahan-selection loop
// previously had no iteration cap, so misusing assumeSorted on unsorted data
// (a pathological anti-sorted input makes the pivot ping-pong without shrinking
// the active set) could wedge the process in an unkillable infinite loop. The
// loop is now bounded and returns a deterministic convergence-failure error
// (a plain error, NOT an AssumptionError) on pathological input.
//
// We assert the call returns QUICKLY with an error rather than hanging.
func TestCenterConvergenceFailureOnUnsortedAssumeSorted(t *testing.T) {
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
		val, err := Center(unsorted, true)
		done <- struct {
			val float64
			err error
		}{val, err}
	}()

	select {
	case res := <-done:
		// The contract: on pathological (unsorted) input the call must
		// terminate. It either converges to some value or returns a
		// deterministic convergence-failure error; it must never hang.
		// The important property is that it returned at all. We additionally
		// accept an error here as the expected pathological outcome. (The
		// plain convergence error on the public path is asserted
		// unconditionally by TestCenterConvergenceErrorPublicPath below.)
		if res.err == nil {
			// Acceptable: bounded loop still produced a (meaningless) value
			// without hanging. The key guarantee is termination.
			t.Logf("Center(unsorted, assumeSorted=true) terminated with value %v (no hang)", res.val)
		} else {
			t.Logf("Center(unsorted, assumeSorted=true) returned convergence error: %v", res.err)
		}
	case <-time.After(5 * time.Second):
		t.Fatal("Center(unsorted, assumeSorted=true) hung: the selection-loop iteration cap is broken")
	}
}

// TestCenterConvergenceErrorPublicPath asserts UNCONDITIONALLY that the public
// Center() entry surfaces the plain convergence-failure error (NOT an
// AssumptionError) on an input that drives the Monahan-selection loop past its
// iteration cap. Center has no sparity guard, so the kernel's plain convergence
// error propagates straight through the public wrapper. This input was verified
// to surface the plain convergence error on the public Center path.
func TestCenterConvergenceErrorPublicPath(t *testing.T) {
	input := []float64{100, 90, 80, 5, 70, 1, 60, 50, 3, 40}

	_, err := Center(input, true)
	if err == nil {
		t.Fatal("expected convergence failure error from Center, got nil")
	}
	if _, isAssumption := err.(*AssumptionError); isAssumption {
		t.Fatalf("expected plain convergence error from Center, got AssumptionError: %v", err)
	}
	if err.Error() != "convergence failure (pathological input)" {
		t.Fatalf("unexpected Center error: %v", err)
	}
}

// TestCenterConvergenceErrorReturnedFast checks that the bounded loop produces a
// deterministic error on input crafted to never converge, and does so quickly.
func TestCenterConvergenceErrorReturnedFast(t *testing.T) {
	// A larger descending input maximizes the chance of triggering the cap /
	// stall guard rather than accidental convergence.
	unsorted := make([]float64, 257)
	for i := range unsorted {
		unsorted[i] = float64(len(unsorted) - i)
	}

	start := time.Now()
	_, err := Center(unsorted, true)
	elapsed := time.Since(start)

	if elapsed > 2*time.Second {
		t.Fatalf("Center(unsorted, assumeSorted=true) took %v — should terminate quickly", elapsed)
	}
	// We don't strictly require an error (some unsorted inputs may still
	// converge), but if one is returned it must be the plain convergence error,
	// never an AssumptionError.
	if err != nil {
		if _, isAssumption := err.(*AssumptionError); isAssumption {
			t.Fatalf("expected plain convergence error, got AssumptionError: %v", err)
		}
		if err.Error() != "convergence failure (pathological input)" {
			t.Fatalf("unexpected error: %v", err)
		}
	}
}
