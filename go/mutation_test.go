package pragmastat

import (
	"testing"
)

// TestRawAPIDoesNotMutateInput is a regression guard: the public raw
// (native-slice) API must never mutate the caller's slice. The kernels sort a
// copy, so the caller's slice must remain byte-for-byte unchanged after every
// estimator call, which is what assertBitsUnchanged checks.
func TestRawAPIDoesNotMutateInput(t *testing.T) {
	const misrate = 0.3

	// One-sample estimators.
	t.Run("OneSample", func(t *testing.T) {
		x := []float64{3, 1, 2, 5, 4, 8, 6, 7}
		snapshot := append([]float64(nil), x...)

		if _, err := Center(x, false); err != nil {
			t.Fatalf("Center: %v", err)
		}
		if _, err := Spread(x, false); err != nil {
			t.Fatalf("Spread: %v", err)
		}
		if _, err := CenterBounds(x, misrate, false); err != nil {
			t.Fatalf("CenterBounds: %v", err)
		}
		if _, err := SpreadBounds(x, misrate, false); err != nil {
			t.Fatalf("SpreadBounds: %v", err)
		}

		assertBitsUnchanged(t, "x", x, snapshot)
	})

	// Two-sample estimators.
	t.Run("TwoSample", func(t *testing.T) {
		x := []float64{3, 1, 2, 5, 4, 8, 6, 7}
		y := []float64{9, 11, 10, 13, 12, 16, 14, 15}
		snapshotX := append([]float64(nil), x...)
		snapshotY := append([]float64(nil), y...)

		if _, err := Shift(x, y, false); err != nil {
			t.Fatalf("Shift: %v", err)
		}
		if _, err := Ratio(x, y, false); err != nil {
			t.Fatalf("Ratio: %v", err)
		}
		if _, err := Disparity(x, y, false); err != nil {
			t.Fatalf("Disparity: %v", err)
		}
		if _, err := ShiftBounds(x, y, misrate, false); err != nil {
			t.Fatalf("ShiftBounds: %v", err)
		}
		if _, err := RatioBounds(x, y, misrate, false); err != nil {
			t.Fatalf("RatioBounds: %v", err)
		}
		if _, err := DisparityBounds(x, y, misrate, false); err != nil {
			t.Fatalf("DisparityBounds: %v", err)
		}

		assertBitsUnchanged(t, "x", x, snapshotX)
		assertBitsUnchanged(t, "y", y, snapshotY)
	})
}

// assertBitsUnchanged fails if any element of got differs from snapshot at the
// bit level. It goes through sameFloatBits rather than == (or reflect.DeepEqual,
// which is == for a float64 element), because == cannot see a 0.0 -> -0.0
// rewrite, and a kernel that rewrote the caller's buffer that way has mutated it
// just as surely as one that changed the magnitude.
func assertBitsUnchanged(t *testing.T, label string, got, snapshot []float64) {
	t.Helper()
	if len(got) != len(snapshot) {
		t.Fatalf("%s length changed: got %d, want %d", label, len(got), len(snapshot))
	}
	for i := range got {
		if !sameFloatBits(got[i], snapshot[i]) {
			t.Errorf("%s[%d] was mutated: got %s, want %s",
				label, i, formatFloatBits(got[i]), formatFloatBits(snapshot[i]))
		}
	}
}

// TestRawAPIDoesNotMutateSortedInput covers the assumeSorted=true branch. With
// the flag set, the sorted-only kernels read the caller's buffer directly (no
// defensive copy is taken), so any in-place write inside a kernel — a
// partition swap, a shuffle, a re-sort side effect — would corrupt the
// caller's slice. Pass an already-sorted slice, snapshot it, and assert every
// element stays bit-identical after each estimator call.
func TestRawAPIDoesNotMutateSortedInput(t *testing.T) {
	const misrate = 0.3

	// One-sample estimators.
	t.Run("OneSample", func(t *testing.T) {
		x := []float64{1, 2, 3, 4, 5, 6, 7, 8}
		snapshot := append([]float64(nil), x...)

		if _, err := Center(x, true); err != nil {
			t.Fatalf("Center: %v", err)
		}
		if _, err := Spread(x, true); err != nil {
			t.Fatalf("Spread: %v", err)
		}
		if _, err := CenterBounds(x, misrate, true); err != nil {
			t.Fatalf("CenterBounds: %v", err)
		}
		if _, err := SpreadBounds(x, misrate, true); err != nil {
			t.Fatalf("SpreadBounds: %v", err)
		}

		assertBitsUnchanged(t, "x", x, snapshot)
	})

	// Two-sample estimators.
	t.Run("TwoSample", func(t *testing.T) {
		x := []float64{1, 2, 3, 4, 5, 6, 7, 8}
		y := []float64{9, 10, 11, 12, 13, 14, 15, 16}
		snapshotX := append([]float64(nil), x...)
		snapshotY := append([]float64(nil), y...)

		if _, err := Shift(x, y, true); err != nil {
			t.Fatalf("Shift: %v", err)
		}
		if _, err := Ratio(x, y, true); err != nil {
			t.Fatalf("Ratio: %v", err)
		}
		if _, err := Disparity(x, y, true); err != nil {
			t.Fatalf("Disparity: %v", err)
		}
		if _, err := ShiftBounds(x, y, misrate, true); err != nil {
			t.Fatalf("ShiftBounds: %v", err)
		}
		if _, err := RatioBounds(x, y, misrate, true); err != nil {
			t.Fatalf("RatioBounds: %v", err)
		}
		if _, err := DisparityBounds(x, y, misrate, true); err != nil {
			t.Fatalf("DisparityBounds: %v", err)
		}

		assertBitsUnchanged(t, "x", x, snapshotX)
		assertBitsUnchanged(t, "y", y, snapshotY)
	})
}
